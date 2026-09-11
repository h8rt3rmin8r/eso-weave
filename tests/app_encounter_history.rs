use std::fs;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use eframe::egui;
use egui_kittest::{kittest::Queryable, Harness};

use eso_weave::app::encounter_history::{
    metric_presentation, EncounterHistoryWorker, HistoryEvent, HistoryOperation,
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

fn seeded_service(root: &std::path::Path) -> EncounterHistoryService {
    let input = root.join("capture.lua");
    let value: serde_json::Value = serde_json::from_str(CAPTURE).unwrap();
    fs::write(
        &input,
        format!("EsoWeaveEncounterSaved = {}", json_to_lua(&value)),
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
    assert_eq!(
        service
            .import_current(&input, Channel::Live)
            .unwrap()
            .outcome,
        ImportOutcome::Imported
    );
    service
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
    let mut installed = false;
    Harness::builder()
        .with_size(egui::vec2(760.0, 1000.0))
        .build_ui_state(
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
    assert_eq!(view.quality, "Degraded");
    assert_eq!(view.loss, vec!["Sequences 10-11 (capture-overflow)"]);
}

#[test]
fn worker_serializes_refresh_detail_and_deletion_results() {
    let root = tempfile::tempdir().unwrap();
    let service = seeded_service(root.path());
    let worker = EncounterHistoryWorker::spawn(service);
    worker.refresh().unwrap();
    let summaries = match worker.receive_timeout(Duration::from_secs(2)).unwrap() {
        HistoryEvent::Snapshot { encounters, .. } => encounters,
        event => panic!("unexpected event: {event:?}"),
    };
    let identity: EncounterIdentity = (&summaries[0]).into();
    worker.load_detail(identity.clone()).unwrap();
    match worker.receive_timeout(Duration::from_secs(2)).unwrap() {
        HistoryEvent::Detail { result, .. } => {
            assert_eq!(result.unwrap().observed_dps.value, Some(300.0));
        }
        event => panic!("unexpected event: {event:?}"),
    }
    worker.delete_one(identity).unwrap();
    match worker.receive_timeout(Duration::from_secs(2)).unwrap() {
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
    match worker.receive_timeout(Duration::from_secs(2)).unwrap() {
        HistoryEvent::Detail { result, .. } => assert_eq!(
            result.unwrap_err().kind,
            HistoryDiagnosticKind::VersionMismatch
        ),
        event => panic!("unexpected event: {event:?}"),
    }

    worker.set_catalog_path(live);
    worker.load_detail(identity).unwrap();
    match worker.receive_timeout(Duration::from_secs(2)).unwrap() {
        HistoryEvent::Detail { result, .. } => {
            assert_eq!(result.unwrap().catalog_join.catalog_version, "s070-live-1");
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
    harness.get_by_label("Degraded");
    harness.get_by_label("Sequences 10-11 (capture-overflow)");
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
    harness.get_by_label("No local encounters. Import a terminal capture to begin.");
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
        saved_variables.join("EsoWeaveEncounter.lua"),
        format!("EsoWeaveEncounterSaved = {}", json_to_lua(&value)),
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
        .get_by_role_and_label(egui::accesskit::Role::Button, "Import Current Capture")
        .click_accesskit();
    settle(&mut harness);
    assert_eq!(harness.state().encounter_history_count(), 1);
    harness.get_by_label("Encounter imported.");
}
