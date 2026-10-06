use std::fs;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use eframe::egui;
use egui_kittest::{kittest::Queryable, Harness};

use eso_weave::app::encounter_history::{
    metric_presentation, recommendation_presentation, EncounterHistoryWorker, HistoryEvent,
    HistoryOperation,
};
use eso_weave::app::ui::EsoWeaveApp;
use eso_weave::app::AppModel;
use eso_weave::beacon::{BeaconPrefs, Environment};
use eso_weave::catalog::compiler::{build_catalog, BuildRequest};
use eso_weave::catalog::Channel;
use eso_weave::config::{LoggingPrefs, Settings};
use eso_weave::encounter::{
    EncounterHistoryService, EncounterIdentity, HistoryDiagnosticKind, ImportOutcome, LossRange,
    MetricQuality, MetricResult,
};
use eso_weave::fishing::{FishingConfig, FishingController, MockFishingSink};
use eso_weave::input::bindings::BindingTable;
use eso_weave::input::InputEngine;
use eso_weave::logging;
use eso_weave::weave::{WeaveConfig, WeaveEngine};

const CAPTURE: &str =
    include_str!("../specs/077-encounter-metrics/fixtures/encounter-metrics-capture.json");
const LOSSLESS_CAPTURE: &str = include_str!("fixtures/encounter/valid-v2-lossless.json");
const WORKER_TIMEOUT: Duration = Duration::from_secs(5);
const DESKTOP_CONTROL_SOURCES: [&str; 9] = [
    include_str!("../src/app/routing.rs"),
    include_str!("../src/app/mod.rs"),
    include_str!("../src/config/mod.rs"),
    include_str!("../src/config/state.rs"),
    include_str!("../src/input/action.rs"),
    include_str!("../src/input/bindings.rs"),
    include_str!("../src/input/mod.rs"),
    include_str!("../src/app/settings_form.rs"),
    include_str!("../src/main.rs"),
];
const DESKTOP_UI_SOURCE: &str = include_str!("../src/app/ui.rs");

#[test]
fn s121_metric_loss_presentation_names_missing_observations_and_explains_reason() {
    let loss = eso_weave::app::encounter_history::loss_labels(&[LossRange {
        missing_sequence_from: 10,
        missing_sequence_to: 11,
        reason: "record-limit".into(),
    }]);
    assert!(loss[0].contains("Missing observations 10-11"));
    assert!(loss[0].contains("recording limit"));
    let quality = eso_weave::app::encounter_history::quality_label(MetricQuality::Degraded);
    assert_eq!(quality, "Incomplete observations");
}

fn json_to_lua(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => "nil".into(),
        serde_json::Value::Bool(value) => value.to_string(),
        serde_json::Value::Number(value) => value.to_string(),
        serde_json::Value::String(value) => serde_json::to_string(value).unwrap(),
        serde_json::Value::Array(values) => format!(
            "{{{}}}",
            values
                .iter()
                .enumerate()
                .map(|(index, value)| format!("[{}]={}", index + 1, json_to_lua(value)))
                .collect::<Vec<_>>()
                .join(",")
        ),
        serde_json::Value::Object(values) => format!(
            "{{{}}}",
            values
                .iter()
                .map(|(key, value)| format!(
                    "[{}]={}",
                    serde_json::to_string(key).unwrap(),
                    json_to_lua(value)
                ))
                .collect::<Vec<_>>()
                .join(",")
        ),
    }
}

fn seeded_service_with_capture(
    root: &std::path::Path,
    value: &serde_json::Value,
) -> EncounterHistoryService {
    let input = root.join("capture.lua");
    fs::write(
        &input,
        format!(
            "EsoWeaveDataSaved = {{ [\"schema_version\"] = 1, [\"addon_version\"] = 1, [\"encounter\"] = {} }}",
            json_to_lua(value)
        ),
    )
    .unwrap();
    let catalog = root.join("catalog.sqlite");
    build_catalog(&BuildRequest::new(
        "specs/070-catalog-compiler/fixtures/minimal-live.json",
        &catalog,
        Channel::Live,
    ))
    .unwrap();
    let service = EncounterHistoryService::new(root, catalog);
    let report = service.import_current(&input, Channel::Live).unwrap();
    assert_eq!(report.imported_count, 1);
    assert_eq!(report.receipts[0].outcome, ImportOutcome::Imported);
    service
}

fn seeded_service(root: &std::path::Path) -> EncounterHistoryService {
    seeded_service_with_capture(root, &serde_json::from_str(CAPTURE).unwrap())
}

fn complete_capture() -> serde_json::Value {
    let mut value: serde_json::Value = serde_json::from_str(CAPTURE).unwrap();
    let object = value.as_object_mut().unwrap();
    object.insert("status".into(), "complete".into());
    object.remove("partial_reason");
    object.insert("omitted_event_count".into(), 0.into());
    let events = object.get_mut("events").unwrap().as_array_mut().unwrap();
    events.retain(|event| event["kind"] != "discontinuity");
    for (index, event) in events.iter_mut().enumerate() {
        event["sequence"] = serde_json::json!(index + 1);
        if matches!(event["payload"]["ability_id"].as_i64(), Some(101 | 999999)) {
            event["payload"]["ability_id"] = 100.into();
        }
        if event["kind"] == "effect" && event["sequence"] == 5 {
            event["payload"]["end_ms"] = 505_000.into();
        }
        if event["kind"] == "encounter-end" {
            event["payload"]["complete"] = true.into();
            event["payload"]["reason"] = "combat-ended".into();
        }
    }
    let event_count = events.len();
    object.insert("last_sequence".into(), event_count.into());
    object.insert("stored_event_count".into(), event_count.into());
    value
}

fn continuous_terminal(session_id: &str, encounter_id: &str) -> serde_json::Value {
    let mut capture: serde_json::Value = serde_json::from_str(LOSSLESS_CAPTURE).unwrap();
    capture["addon_version"] = serde_json::json!(3);
    capture["status"] = serde_json::json!("partial");
    capture["partial_reason"] = serde_json::json!("user-stopped");
    capture["normalization_profile"] = serde_json::json!({
        "version": 1,
        "api_version": 101050,
        "player_combat_unit_type": 4,
        "health_power_type": 1,
        "quickslot_category": 9,
        "damage_results": [10, 11, 12, 13, 14, 15],
        "healing_results": [20, 21, 22, 23],
        "death_results": [30, 31],
        "resurrect_result": 32
    });
    capture["session_id"] = serde_json::json!(session_id);
    capture["encounter_id"] = serde_json::json!(encounter_id);
    for observation in capture["raw_observations"].as_array_mut().unwrap() {
        observation["session_id"] = serde_json::json!(session_id);
        observation["encounter_id"] = serde_json::json!(encounter_id);
    }
    for event in capture["events"].as_array_mut().unwrap() {
        event["session_id"] = serde_json::json!(session_id);
        event["encounter_id"] = serde_json::json!(encounter_id);
    }
    capture["events"][1]["payload"] =
        serde_json::json!({"complete": false, "reason": "user-stopped"});
    capture["raw_observations"][3]["values"][0]["string"] = serde_json::json!("user-stopped");
    capture["raw_observations"][3]["values"][1]["boolean"] = serde_json::json!(false);
    capture
}

fn continuous_state_lua() -> String {
    let session_id = "session-1788912002-2000";
    let first = continuous_terminal(session_id, "encounter-1788912002-1");
    let second = continuous_terminal(session_id, "encounter-1788912002-2");
    let state = serde_json::json!({
        "state_schema_version": 1,
        "addon_version": 4,
        "selected_mode": "continuous",
        "selected_channel": "live",
        "requested_mode": "continuous",
        "active_mode": "continuous",
        "state": "waiting",
        "session": {
            "session_id": session_id,
            "mode": "continuous",
            "channel": "live",
            "status": "active",
            "started_at": "1788912002",
            "next_encounter_ordinal": 3,
            "completed_encounter_count": 2,
            "degraded_encounter_count": 2,
            "aggregate_estimated_bytes": 8192,
            "aggregate_event_count": 4,
            "aggregate_raw_observation_count": 8,
            "interruption_count": 0
        },
        "records": {
            "0000000001": {"ordinal": 1, "capture": first},
            "0000000002": {"ordinal": 2, "capture": second}
        },
        "interruptions": [],
        "revision": 7
    });
    format!(
        "EsoWeaveDataSaved = {{ [\"schema_version\"] = 1, [\"addon_version\"] = 1, [\"encounter\"] = {} }}",
        json_to_lua(&state)
    )
}

fn qualified_capture() -> serde_json::Value {
    let mut value = complete_capture();
    let object = value.as_object_mut().unwrap();
    object.insert("status".into(), "partial".into());
    object.insert("partial_reason".into(), "capture-overflow".into());
    object.insert("stored_event_count".into(), 11.into());
    object.insert("omitted_event_count".into(), 1.into());
    let events = object.get_mut("events").unwrap().as_array_mut().unwrap();
    events.retain(|event| event["sequence"] != 10);
    let marker = events
        .iter_mut()
        .find(|event| event["sequence"] == 11)
        .unwrap();
    marker["kind"] = "discontinuity".into();
    marker["payload"] = serde_json::json!({
        "missing_sequence_from": 10,
        "missing_sequence_to": 10,
        "reason": "capture-overflow"
    });
    let end = events
        .iter_mut()
        .find(|event| event["kind"] == "encounter-end")
        .unwrap();
    end["payload"]["complete"] = false.into();
    end["payload"]["reason"] = "capture-overflow".into();
    value
}

fn test_app(service: EncounterHistoryService, settings: Settings) -> EsoWeaveApp {
    let (engine, _rx) = InputEngine::new(BindingTable::default(), 16);
    let weave = Arc::new(Mutex::new(WeaveEngine::new(WeaveConfig::default())));
    let fishing = Arc::new(Mutex::new(FishingController::new(FishingConfig::default())));
    let (_dispatch, log) = logging::build(&LoggingPrefs::default(), PathBuf::from("."));
    let (reader_update_tx, reader_updates) = mpsc::channel();
    let model = AppModel::new(
        Arc::new(engine),
        weave,
        fishing,
        Box::new(MockFishingSink::new()),
        Arc::new(Mutex::new(eso_weave::potion::AutoPotionController::new(
            eso_weave::potion::AutoPotionConfig::default(),
        ))),
        log,
        reader_update_tx,
        settings,
        None,
        std::time::Instant::now(),
    );
    let (toggle_tx, toggle_rx) = mpsc::channel();
    let (api_tx, api_rx) = mpsc::channel();
    std::mem::forget(toggle_tx);
    std::mem::forget(api_tx);
    std::mem::forget(reader_updates);
    EsoWeaveApp::new(model, toggle_rx, api_rx, None).with_encounter_history(service)
}

fn harness_with_settings(
    service: EncounterHistoryService,
    settings: Settings,
) -> Harness<'static, EsoWeaveApp> {
    harness_with_size(service, settings, egui::vec2(760.0, 1000.0))
}

fn harness_with_size(
    service: EncounterHistoryService,
    settings: Settings,
    size: egui::Vec2,
) -> Harness<'static, EsoWeaveApp> {
    let mut installed = false;
    Harness::builder().with_size(size).build_ui_state(
        move |ui, app: &mut EsoWeaveApp| {
            if !installed {
                eso_weave::app::theme::install_fonts(ui.ctx());
                installed = true;
                return;
            }
            app.frame_ui(ui);
        },
        test_app(service, settings),
    )
}

fn harness(service: EncounterHistoryService) -> Harness<'static, EsoWeaveApp> {
    harness_with_settings(service, Settings::default())
}

fn settle(harness: &mut Harness<'static, EsoWeaveApp>) {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        harness.step();
        if !harness.state().encounter_history_busy() {
            harness.step();
            return;
        }
        assert!(
            Instant::now() < deadline,
            "encounter history worker did not settle"
        );
        std::thread::sleep(Duration::from_millis(5));
    }
}

#[test]
fn metric_presentation_names_unavailable_and_exact_degraded_loss() {
    let result = MetricResult {
        metric_id: "observed-dps".into(),
        value: None,
        unit: "damage-per-second".into(),
        algorithm_version: "s069-v1".into(),
        first_sequence: 1,
        last_sequence: 12,
        quality: MetricQuality::Degraded,
        loss_ranges: vec![LossRange {
            missing_sequence_from: 10,
            missing_sequence_to: 11,
            reason: "capture-overflow".into(),
        }],
    };
    let view = metric_presentation("Observed DPS", &result);
    assert_eq!(view.label, "Observed DPS");
    assert_eq!(view.value, "Unavailable");
    assert_eq!(view.quality, "Incomplete observations");
    assert_eq!(
        view.loss,
        vec!["Missing observations 10-11 (recording limit reached; code: capture-overflow)"]
    );
}

#[test]
fn worker_serializes_refresh_detail_and_deletion_results() {
    let root = tempfile::tempdir().unwrap();
    let service = seeded_service(root.path());
    let worker = EncounterHistoryWorker::spawn(service);
    worker.refresh().unwrap();
    let summaries = match worker.receive_timeout(WORKER_TIMEOUT).unwrap() {
        HistoryEvent::Snapshot { encounters, .. } => encounters,
        event => panic!("unexpected event: {event:?}"),
    };
    let identity: EncounterIdentity = (&summaries[0]).into();
    worker.load_detail(identity.clone()).unwrap();
    match worker.receive_timeout(WORKER_TIMEOUT).unwrap() {
        HistoryEvent::Detail { result, .. } => {
            let detail = result.unwrap();
            assert_eq!(detail.projection.observed_dps.value, Some(300.0));
            assert_eq!(
                detail.recommendations.evidence.citation.raw_content_sha256,
                detail.projection.raw_content_sha256
            );
        }
        event => panic!("unexpected event: {event:?}"),
    }
    worker.delete_one(identity).unwrap();
    match worker.receive_timeout(WORKER_TIMEOUT).unwrap() {
        HistoryEvent::Snapshot {
            encounters,
            operation,
            ..
        } => {
            assert_eq!(operation, HistoryOperation::DeleteOne);
            assert!(encounters.is_empty());
        }
        event => panic!("unexpected event: {event:?}"),
    }
}

#[test]
fn worker_imports_an_ordered_batch_and_reports_last_saved_state() {
    let root = tempfile::tempdir().unwrap();
    let input = root.path().join("continuous.lua");
    fs::write(&input, continuous_state_lua()).unwrap();
    let catalog = root.path().join("catalog.sqlite");
    build_catalog(&BuildRequest::new(
        "specs/070-catalog-compiler/fixtures/minimal-live.json",
        &catalog,
        Channel::Live,
    ))
    .unwrap();
    let worker = EncounterHistoryWorker::spawn(EncounterHistoryService::new(root.path(), catalog));

    worker.import_current(input, Channel::Live).unwrap();
    match worker.receive_timeout(WORKER_TIMEOUT).unwrap() {
        HistoryEvent::Snapshot {
            encounters,
            message,
            last_saved_state,
            ..
        } => {
            assert_eq!(encounters.len(), 2);
            assert_eq!(encounters[0].encounter_ordinal, 1);
            assert_eq!(encounters[1].encounter_ordinal, 2);
            assert_eq!(message.as_deref(), Some("2 encounters imported."));
            let saved = last_saved_state.unwrap();
            assert_eq!(
                saved.state,
                eso_weave::encounter::CaptureControllerState::Waiting
            );
        }
        event => panic!("unexpected event: {event:?}"),
    }
}

#[test]
fn worker_uses_catalog_path_replacements_for_subsequent_details() {
    let root = tempfile::tempdir().unwrap();
    let service = seeded_service(root.path());
    let identity = EncounterIdentity::from(&service.snapshot().unwrap()[0]);
    let live = root.path().join("catalog.sqlite");
    let pts = root.path().join("pts-catalog.sqlite");
    build_catalog(&BuildRequest::new(
        "specs/070-catalog-compiler/fixtures/minimal-pts.json",
        &pts,
        Channel::Pts,
    ))
    .unwrap();
    service.set_catalog_path(&pts);
    let worker = EncounterHistoryWorker::spawn(service);

    worker.load_detail(identity.clone()).unwrap();
    match worker.receive_timeout(WORKER_TIMEOUT).unwrap() {
        HistoryEvent::Detail { result, .. } => assert_eq!(
            result.unwrap_err().kind,
            HistoryDiagnosticKind::VersionMismatch
        ),
        event => panic!("unexpected event: {event:?}"),
    }

    worker.set_catalog_path(live);
    worker.load_detail(identity).unwrap();
    match worker.receive_timeout(WORKER_TIMEOUT).unwrap() {
        HistoryEvent::Detail { result, .. } => {
            let detail = result.unwrap();
            assert_eq!(
                detail.projection.catalog_join.catalog_version,
                "s070-live-1"
            );
            assert_eq!(
                detail.recommendations.evidence.citation.catalog_version,
                detail.projection.catalog_join.catalog_version
            );
        }
        event => panic!("unexpected event: {event:?}"),
    }
}

#[test]
fn rendered_history_exposes_quality_and_requires_delete_confirmation() {
    let root = tempfile::tempdir().unwrap();
    let mut harness = harness(seeded_service(root.path()));
    for _ in 0..6 {
        harness.step();
    }
    harness
        .get_by_role_and_label(egui::accesskit::Role::Button, "File")
        .click_accesskit();
    harness.step();
    harness
        .get_by_role_and_label(
            egui::accesskit::Role::Button,
            eso_weave::app::strings::MENU_ENCOUNTER_HISTORY,
        )
        .click_accesskit();
    settle(&mut harness);
    harness.get_by_label("Encounter History");
    harness
        .get_by_role_and_label(egui::accesskit::Role::Button, "Delete All")
        .click_accesskit();
    harness.step();
    harness.get_by_label("Delete All Encounters?");
    harness
        .get_by_role_and_label(egui::accesskit::Role::Button, "Cancel")
        .click_accesskit();
    harness.step();
    assert_eq!(harness.state().encounter_history_count(), 1);
    harness
        .get_by_role_and_label(egui::accesskit::Role::Button, "encounter-1788998400-1")
        .click_accesskit();
    settle(&mut harness);
    harness.get_by_label("Observed DPS");
    harness.get_by_label("Incomplete observations");
    harness.get_by_label(
        "Missing observations 10-11 (recording limit reached; code: capture-overflow)",
    );
    harness.get_by_label("Provisional Recommendations");
    harness.get_by_label("Review prompts unavailable");
    harness
        .get_by_label("No review prompts are shown because this recording does not meet the analysis requirements. The reasons below explain what is missing; observed metrics remain separate.");
    assert!(harness
        .query_by_role_and_label(egui::accesskit::Role::Button, "Apply Recommendation")
        .is_none());
    harness
        .get_by_role_and_label(egui::accesskit::Role::Button, "Delete Encounter")
        .click_accesskit();
    harness.step();
    harness.get_by_label("Delete Encounter?");
    harness
        .get_by_role_and_label(egui::accesskit::Role::Button, "Cancel")
        .click_accesskit();
    harness.step();
    assert_eq!(harness.state().encounter_history_count(), 1);

    harness
        .get_by_role_and_label(egui::accesskit::Role::Button, "Delete Encounter")
        .click_accesskit();
    harness.step();
    harness
        .get_by_role_and_label(egui::accesskit::Role::Button, "Confirm Delete Encounter")
        .click_accesskit();
    settle(&mut harness);
    assert_eq!(harness.state().encounter_history_count(), 0);
    harness.get_by_label("No imported encounters yet. Record inside ESO, save addon data with /reloadui, logout, or exit, then choose Import Saved Capture.");
}

#[test]
fn rendered_import_uses_the_explicitly_selected_environment() {
    let root = tempfile::tempdir().unwrap();
    let environment = root.path().join("live");
    let addons = environment.join("AddOns");
    let saved_variables = environment.join("SavedVariables");
    fs::create_dir_all(&addons).unwrap();
    fs::create_dir_all(&saved_variables).unwrap();
    let value: serde_json::Value = serde_json::from_str(CAPTURE).unwrap();
    fs::write(
        saved_variables.join("EsoWeaveData.lua"),
        format!(
            "EsoWeaveDataSaved = {{ [\"schema_version\"] = 1, [\"addon_version\"] = 1, [\"encounter\"] = {} }}",
            json_to_lua(&value)
        ),
    )
    .unwrap();
    let catalog = root.path().join("catalog.sqlite");
    build_catalog(&BuildRequest::new(
        "specs/070-catalog-compiler/fixtures/minimal-live.json",
        &catalog,
        Channel::Live,
    ))
    .unwrap();
    let service = EncounterHistoryService::new(root.path().join("app-data"), catalog);
    let settings = Settings {
        beacon: eso_weave::beacon::prefs_to_value(&BeaconPrefs {
            path_override: Some(addons),
            environment: Environment::Live,
        }),
        ..Settings::default()
    };
    let mut harness = harness_with_settings(service, settings);
    for _ in 0..6 {
        harness.step();
    }
    harness
        .get_by_role_and_label(egui::accesskit::Role::Button, "File")
        .click_accesskit();
    harness.step();
    harness
        .get_by_role_and_label(
            egui::accesskit::Role::Button,
            eso_weave::app::strings::MENU_ENCOUNTER_HISTORY,
        )
        .click_accesskit();
    settle(&mut harness);
    assert_eq!(harness.state().encounter_history_count(), 0);
    harness
        .get_by_role_and_label(egui::accesskit::Role::Button, "Import Saved Capture")
        .click_accesskit();
    settle(&mut harness);
    assert_eq!(harness.state().encounter_history_count(), 1);
    harness.get_by_label("1 encounter imported.");
}

#[test]
fn rendered_batch_import_groups_ordinals_and_labels_last_saved_state() {
    let root = tempfile::tempdir().unwrap();
    let environment = root.path().join("live");
    let addons = environment.join("AddOns");
    let saved_variables = environment.join("SavedVariables");
    fs::create_dir_all(&addons).unwrap();
    fs::create_dir_all(&saved_variables).unwrap();
    fs::write(
        saved_variables.join("EsoWeaveData.lua"),
        continuous_state_lua(),
    )
    .unwrap();
    let catalog = root.path().join("catalog.sqlite");
    build_catalog(&BuildRequest::new(
        "specs/070-catalog-compiler/fixtures/minimal-live.json",
        &catalog,
        Channel::Live,
    ))
    .unwrap();
    let service = EncounterHistoryService::new(root.path().join("app-data"), catalog);
    let settings = Settings {
        beacon: eso_weave::beacon::prefs_to_value(&BeaconPrefs {
            path_override: Some(addons),
            environment: Environment::Live,
        }),
        ..Settings::default()
    };
    let mut harness = harness_with_settings(service, settings);
    for _ in 0..6 {
        harness.step();
    }
    harness
        .get_by_role_and_label(egui::accesskit::Role::Button, "File")
        .click_accesskit();
    harness.step();
    harness
        .get_by_role_and_label(
            egui::accesskit::Role::Button,
            eso_weave::app::strings::MENU_ENCOUNTER_HISTORY,
        )
        .click_accesskit();
    settle(&mut harness);
    harness
        .get_by_role_and_label(egui::accesskit::Role::Button, "Import Saved Capture")
        .click_accesskit();
    settle(&mut harness);

    assert_eq!(harness.state().encounter_history_count(), 2);
    harness.get_by_label("2 encounters imported.");
    harness.get_by_label("Last saved recording state");
    harness.get_by_label("Continuous fights mode | Waiting for combat | revision 7");
    harness.get_by_label(
        "This is the addon's state from the last successfully imported saved file, not current activity. Refresh and failed imports keep this older summary; check Saved channel below. Enter /ewencounter status inside ESO for current recording status.",
    );
    harness.get_by_label("Encounter 1");
    harness.get_by_label("Encounter 2");
    assert!(harness
        .query_by_role_and_label(egui::accesskit::Role::Button, "Start Capture")
        .is_none());

    let legacy: serde_json::Value = serde_json::from_str(CAPTURE).unwrap();
    fs::write(
        saved_variables.join("EsoWeaveData.lua"),
        format!(
            "EsoWeaveDataSaved = {{ [\"schema_version\"] = 1, [\"addon_version\"] = 1, [\"encounter\"] = {} }}",
            json_to_lua(&legacy)
        ),
    )
    .unwrap();
    harness
        .get_by_role_and_label(egui::accesskit::Role::Button, "Import Saved Capture")
        .click_accesskit();
    settle(&mut harness);
    assert_eq!(harness.state().encounter_history_count(), 3);
    harness.get_by_label("1 encounter imported.");
    assert!(harness
        .query_by_label("Last saved recording state")
        .is_none());
}

#[test]
fn desktop_action_and_configuration_surfaces_have_zero_encounter_command_ingress() {
    for source in DESKTOP_CONTROL_SOURCES
        .into_iter()
        .chain(std::iter::once(DESKTOP_UI_SOURCE))
    {
        let source = source.to_ascii_lowercase();
        for forbidden in [
            "toggleencounter",
            "startencounter",
            "stopencounter",
            "encounter_mode",
            "requested_mode",
        ] {
            assert!(
                !source.contains(forbidden),
                "desktop control source contains forbidden ingress token {forbidden}"
            );
        }
    }
    // AppModel remediation and UI may contain instructions, but configuration,
    // routing, input actions and bindings expose no recording command.
    for (index, source) in DESKTOP_CONTROL_SOURCES.into_iter().enumerate() {
        if index != 1 {
            assert!(!source.contains("/ewencounter"));
        }
    }
    assert!(DESKTOP_UI_SOURCE.contains(
        "This is the addon's state from the last successfully imported saved file, not current activity. Refresh and failed imports keep this older summary; check Saved channel below. Enter /ewencounter status inside ESO for current recording status."
    ));
    // Static instructions tell the player what to type inside ESO. The worker
    // exposes disk/history operations only, never a game-command sender.
    for instruction in [
        "/ewencounter toggle",
        "/ewencounter mode single",
        "/ewencounter channel live",
    ] {
        assert!(DESKTOP_UI_SOURCE.contains(instruction));
    }
    let worker_source = include_str!("../src/app/encounter_history.rs");
    let command_surface = worker_source
        .split("enum HistoryCommand {")
        .nth(1)
        .unwrap()
        .split("#[derive")
        .next()
        .unwrap();
    for operation in [
        "Refresh",
        "Import",
        "LoadDetail",
        "DeleteOne",
        "DeleteAll",
        "Stop",
    ] {
        assert!(command_surface.contains(operation));
    }
    for forbidden in [
        "Toggle",
        "StartCapture",
        "StopCapture",
        "SetMode",
        "SendCommand",
    ] {
        assert!(!command_surface.contains(forbidden));
    }
}

#[test]
fn recommendation_presentation_keeps_provisional_advice_and_citations_explicit() {
    let root = tempfile::tempdir().unwrap();
    let service = seeded_service_with_capture(root.path(), &complete_capture());
    let worker = EncounterHistoryWorker::spawn(service.clone());
    let identity = EncounterIdentity::from(&service.snapshot().unwrap()[0]);
    worker.load_detail(identity).unwrap();
    let detail = match worker.receive_timeout(WORKER_TIMEOUT).unwrap() {
        HistoryEvent::Detail { result, .. } => result.unwrap(),
        event => panic!("unexpected event: {event:?}"),
    };

    let view = recommendation_presentation(&detail.recommendations);
    assert_eq!(view.heading, "Provisional Recommendations");
    assert_eq!(view.status, "Review prompts available");
    assert_eq!(view.items.len(), 2);
    assert_eq!(view.items[0].title, "Review dominant observed damage share");
    assert!(view.items[0].body.contains("Ability 100"));
    assert!(view.items[0].body.contains("Compare"));
    assert_eq!(view.items[0].qualification, "Provisional review prompt");
    assert!(view.items[0].qualification_reasons.is_empty());
    assert!(view.items[0].citation.contains("encounter-1788998400-1"));
    assert!(view.items[0].citation.contains("s069-v1"));
    assert!(view.items[0].citation.contains("s070-live-1"));
    assert!(view.items[0].citation.contains("s090-v1"));
}

#[test]
fn s121_empty_prompts_and_suppressed_loss_do_not_claim_thresholds_or_visible_advice() {
    let root = tempfile::tempdir().unwrap();
    let service = seeded_service_with_capture(root.path(), &qualified_capture());
    let identity = EncounterIdentity::from(&service.snapshot().unwrap()[0]);
    let mut projection = service.detail(&identity).unwrap();
    projection.duration_ms = 9_000;
    let report = eso_weave::recommendation::generate_recommendations(&projection);
    assert_eq!(
        report.availability,
        eso_weave::recommendation::RecommendationAvailability::Suppressed
    );
    let loss = report
        .reasons
        .iter()
        .find(|reason| {
            reason.kind == eso_weave::recommendation::RecommendationReasonKind::DeclaredCaptureLoss
        })
        .unwrap();
    assert!(!loss.message.contains("are shown"));
    assert!(loss.message.contains("Any available prompts"));

    projection.duration_ms = 10_000;
    projection.catalog_join.known_ability_ids.clear();
    projection.catalog_join.known_effect_ids.clear();
    projection.catalog_join.unknown_ability_ids = vec![100];
    projection.catalog_join.unknown_effect_ids = vec![200];
    projection.catalog_join.unknown_ids = vec![100, 200];
    let report = eso_weave::recommendation::generate_recommendations(&projection);
    assert!(report.advice.is_empty());
    assert!(report.reasons.iter().any(|reason| reason.kind
        == eso_weave::recommendation::RecommendationReasonKind::UnknownTargetOmitted));
    let view = recommendation_presentation(&report);
    assert!(!view
        .summary
        .contains("No ability damage share or effect uptime met"));
    assert!(view.summary.contains("omitted prompts or unmet thresholds"));
    assert_eq!(view.status, "No review prompts for this recording");
    let mut ready_empty = report.clone();
    ready_empty.availability = eso_weave::recommendation::RecommendationAvailability::Ready;
    assert_eq!(
        recommendation_presentation(&ready_empty).status,
        "No review prompts for this recording"
    );
}

fn open_history_and_select(harness: &mut Harness<'static, EsoWeaveApp>) {
    for _ in 0..6 {
        harness.step();
    }
    harness
        .get_by_role_and_label(egui::accesskit::Role::Button, "File")
        .click_accesskit();
    harness.step();
    harness
        .get_by_role_and_label(
            egui::accesskit::Role::Button,
            eso_weave::app::strings::MENU_ENCOUNTER_HISTORY,
        )
        .click_accesskit();
    settle(harness);
    harness
        .get_by_role_and_label(egui::accesskit::Role::Button, "encounter-1788998400-1")
        .click_accesskit();
    settle(harness);
}

#[test]
fn rendered_ready_and_qualified_recommendations_remain_separate_and_actionless() {
    let ready_root = tempfile::tempdir().unwrap();
    let ready_service = seeded_service_with_capture(ready_root.path(), &complete_capture());
    let mut ready = harness(ready_service);
    open_history_and_select(&mut ready);
    ready.get_by_label("Observed Metrics");
    ready.get_by_label("Provisional Recommendations");
    ready.get_by_label("Review prompts available");
    ready.get_by_label("Review dominant observed damage share");
    ready.get_by_label("Review low observed effect uptime");
    assert!(ready
        .query_by_role_and_label(egui::accesskit::Role::Button, "Execute Recommendation")
        .is_none());

    let qualified_root = tempfile::tempdir().unwrap();
    let qualified_service =
        seeded_service_with_capture(qualified_root.path(), &qualified_capture());
    let mut qualified = harness_with_size(
        qualified_service,
        Settings::default(),
        egui::vec2(400.0, 760.0),
    );
    open_history_and_select(&mut qualified);
    qualified.get_by_label("Observed Metrics");
    qualified.get_by_label("Provisional Recommendations");
    qualified.get_by_label("Review prompts have limitations");
    assert_eq!(
        qualified
            .query_all_by_label("Provisional review prompt with limitations")
            .count(),
        2
    );
    assert_eq!(
        qualified
            .query_all_by_label(
                "The recording is missing 1 of 12 observations. Any available prompts carry this limitation."
            )
            .count(),
        3
    );
    assert_eq!(
        qualified
            .query_all_by_label(
                "Missing observations 10-10 (recording limit reached; code: capture-overflow)"
            )
            .count(),
        3
    );
}

#[test]
fn s121_wrapped_loss_diagnostics_are_fully_painted_at_minimum_width() {
    let root = tempfile::tempdir().unwrap();
    let mut capture: serde_json::Value = serde_json::from_str(CAPTURE).unwrap();
    capture["last_sequence"] = 30.into();
    capture["stored_event_count"] = 20.into();
    capture["omitted_event_count"] = 10.into();
    let events = capture["events"].as_array_mut().unwrap();
    let mut end = events.last().unwrap().clone();
    events.truncate(9);
    for ordinal in 0..10 {
        let sequence = 11 + ordinal * 2;
        events.push(serde_json::json!({
            "session_id": "session-1788998400-500000",
            "encounter_id": "encounter-1788998400-1",
            "sequence": sequence,
            "monotonic_ms": 6000 + ordinal * 250,
            "kind": "discontinuity",
            "payload": {
                "missing_sequence_from": sequence - 1,
                "missing_sequence_to": sequence - 1,
                "reason": "capture-overflow"
            }
        }));
    }
    end["sequence"] = 30.into();
    events.push(end);
    let service = seeded_service_with_capture(root.path(), &capture);
    let mut harness = harness_with_size(service, Settings::default(), egui::vec2(360.0, 3000.0));
    open_history_and_select(&mut harness);
    harness
        .get_by_label("How to record and import")
        .click_accesskit();
    for _ in 0..6 {
        harness.step();
    }
    let labels = [
        "Missing observations 10-10 (recording limit reached; code: capture-overflow)",
        "Missing observations 12-12 (recording limit reached; code: capture-overflow)",
    ];
    harness.get_by_label(labels[0]).scroll_to_me();
    for _ in 0..12 {
        harness.step();
    }
    let first = harness.get_by_label(labels[0]).rect();
    let second = harness.get_by_label(labels[1]).rect();
    assert!(
        first.height() > 20.0,
        "fixture must exercise wrapping: {first:?}"
    );
    assert!(
        second.top() >= first.bottom(),
        "diagnostics overlap: {first:?} {second:?}"
    );
    fn inspect(shape: &egui::epaint::Shape, clip: egui::Rect, label: &str) -> Option<bool> {
        match shape {
            egui::epaint::Shape::Text(text) if text.galley.text() == label => {
                let bounds = text.visual_bounding_rect();
                Some(clip.contains_rect(bounds))
            }
            egui::epaint::Shape::Vec(shapes) => {
                shapes.iter().find_map(|shape| inspect(shape, clip, label))
            }
            _ => None,
        }
    }
    for label in labels {
        let fully_visible = harness
            .output()
            .shapes
            .iter()
            .find_map(|clipped| inspect(&clipped.shape, clipped.clip_rect, label));
        assert_eq!(
            fully_visible,
            Some(true),
            "wrapped loss text is clipped: {label}"
        );
    }
    harness.event(egui::Event::PointerMoved(first.center()));
    harness.event(egui::Event::MouseWheel {
        unit: egui::MouseWheelUnit::Point,
        phase: egui::TouchPhase::Move,
        delta: egui::vec2(0.0, -250.0),
        modifiers: egui::Modifiers::NONE,
    });
    for _ in 0..30 {
        harness.step();
    }
    let last_label = "Missing observations 28-28 (recording limit reached; code: capture-overflow)";
    let fully_visible = harness
        .output()
        .shapes
        .iter()
        .find_map(|clipped| inspect(&clipped.shape, clipped.clip_rect, last_label));
    assert_eq!(
        fully_visible,
        Some(true),
        "last wrapped loss cannot be read after scrolling"
    );
}
