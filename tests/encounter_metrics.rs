use std::fs;

use eso_weave::catalog::compiler::{build_catalog, BuildRequest};
use eso_weave::catalog::{CatalogAccess, Channel};
use eso_weave::encounter::{
    calculate_projection, canonical_projection_bytes, import_encounter, project_encounter,
    EncounterCapture, ImportRequest, MetricQuality, PayloadValue, ProjectionRequest,
};

const CAPTURE: &str =
    include_str!("../specs/077-encounter-metrics/fixtures/encounter-metrics-capture.json");
const LIVE_CATALOG: &str = "specs/070-catalog-compiler/fixtures/minimal-live.json";
const PTS_CATALOG: &str = "specs/070-catalog-compiler/fixtures/minimal-pts.json";
const SESSION: &str = "session-1788998400-500000";
const ENCOUNTER: &str = "encounter-1788998400-1";

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

fn capture_lua() -> String {
    let value: serde_json::Value = serde_json::from_str(CAPTURE).unwrap();
    format!("EsoWeaveEncounterSaved = {}", json_to_lua(&value))
}

fn imported_store(root: &std::path::Path) -> std::path::PathBuf {
    let input = root.join("capture.lua");
    let store = root.join("encounters.sqlite");
    fs::write(&input, capture_lua()).unwrap();
    import_encounter(&ImportRequest::new(&input, &store, Channel::Live)).unwrap();
    store
}

fn catalog(root: &std::path::Path, fixture: &str, name: &str) -> std::path::PathBuf {
    let path = root.join(name);
    build_catalog(&BuildRequest::new(
        fixture,
        &path,
        if fixture == PTS_CATALOG {
            Channel::Pts
        } else {
            Channel::Live
        },
    ))
    .unwrap();
    path
}

#[test]
fn baseline_projection_is_deterministic_loss_aware_and_actor_safe() {
    let sandbox = tempfile::tempdir().unwrap();
    let store = imported_store(sandbox.path());
    let catalog = catalog(sandbox.path(), LIVE_CATALOG, "catalog.sqlite");
    let output = sandbox.path().join("projection.json");
    let raw_before = fs::read(&store).unwrap();
    let catalog_before = fs::read(&catalog).unwrap();
    let request = ProjectionRequest::new(&store, &catalog, &output, SESSION, ENCOUNTER);

    let first = project_encounter(&request).unwrap();
    let first_bytes = fs::read(&output).unwrap();
    let second = project_encounter(&request).unwrap();
    assert_eq!(first, second);
    assert_eq!(first_bytes, fs::read(&output).unwrap());
    assert_eq!(first_bytes, canonical_projection_bytes(&first).unwrap());
    assert_eq!(fs::read(&store).unwrap(), raw_before);
    assert_eq!(fs::read(&catalog).unwrap(), catalog_before);

    assert_eq!(first.duration_ms, 10_000);
    assert_eq!(first.observed_dps.value, Some(300.0));
    assert_eq!(first.effective_hps.value, Some(80.0));
    assert_eq!(first.observed_dps.quality, MetricQuality::Degraded);
    assert_eq!(first.observed_dps.loss_ranges[0].missing_sequence_from, 10);
    assert_eq!(first.observed_dps.loss_ranges[0].missing_sequence_to, 11);
    assert_eq!(
        first
            .ability_damage_share
            .iter()
            .map(|value| (value.ability_id, value.result.value))
            .collect::<Vec<_>>(),
        vec![(100, Some(0.5)), (999999, Some(0.5))]
    );
    assert_eq!(first.effect_uptime.len(), 1);
    assert_eq!(first.effect_uptime[0].ability_id, 200);
    assert_eq!(first.effect_uptime[0].result.value, Some(0.6));
    assert_eq!(
        first.ordered_cast_sequence.ability_ids,
        vec![100, 999999, 100]
    );
    assert_eq!(first.catalog_join.known_ids, vec![100, 200]);
    assert_eq!(first.catalog_join.unknown_ids, vec![101, 999999]);
    assert_eq!(first.catalog_join.known_ability_ids, vec![100]);
    assert_eq!(first.catalog_join.unknown_ability_ids, vec![101, 999999]);
    assert_eq!(first.catalog_join.known_effect_ids, vec![200]);
    assert!(first.catalog_join.unknown_effect_ids.is_empty());
}

#[test]
fn catalog_join_keeps_resolution_scoped_to_entity_kind() {
    let sandbox = tempfile::tempdir().unwrap();
    let catalog_path = catalog(sandbox.path(), LIVE_CATALOG, "catalog.sqlite");
    let catalog = CatalogAccess::open_or_empty(&catalog_path);
    let mut capture: EncounterCapture = serde_json::from_str(CAPTURE).unwrap();
    capture
        .events
        .iter_mut()
        .find(|event| event.kind == "effect")
        .unwrap()
        .payload
        .insert("ability_id".into(), PayloadValue::Integer(100));

    let projection = calculate_projection(&capture, &catalog).unwrap();
    assert_eq!(projection.catalog_join.known_ability_ids, vec![100]);
    assert_eq!(projection.catalog_join.unknown_effect_ids, vec![100]);
    assert_eq!(projection.catalog_join.known_effect_ids, vec![200]);
    assert!(projection.catalog_join.known_ids.contains(&100));
    assert!(projection.catalog_join.unknown_ids.contains(&100));
}

#[test]
fn zero_duration_and_non_player_damage_have_explicit_results() {
    let sandbox = tempfile::tempdir().unwrap();
    let catalog_path = catalog(sandbox.path(), LIVE_CATALOG, "catalog.sqlite");
    let catalog = CatalogAccess::open_or_empty(&catalog_path);
    let mut capture: EncounterCapture = serde_json::from_str(CAPTURE).unwrap();
    capture.ended_monotonic_ms = 0;
    for event in &mut capture.events {
        event.monotonic_ms = 0;
        if event.kind == "damage" {
            event
                .payload
                .insert("source_type".into(), PayloadValue::Integer(5));
        }
    }
    let projection = calculate_projection(&capture, &catalog).unwrap();
    assert_eq!(projection.observed_dps.value, None);
    assert_eq!(projection.effective_hps.value, None);
    assert!(projection.ability_damage_share.is_empty());
    assert_eq!(projection.effect_uptime[0].result.value, None);
    assert_eq!(
        projection.ordered_cast_sequence.ability_ids,
        vec![100, 999999, 100]
    );
}

#[test]
fn aggregate_overflow_is_rejected_instead_of_wrapping() {
    let sandbox = tempfile::tempdir().unwrap();
    let catalog_path = catalog(sandbox.path(), LIVE_CATALOG, "catalog.sqlite");
    let catalog = CatalogAccess::open_or_empty(&catalog_path);
    let mut capture: EncounterCapture = serde_json::from_str(CAPTURE).unwrap();
    for event in &mut capture.events {
        if matches!(event.kind.as_str(), "damage" | "healing") {
            event.kind = "damage".into();
            event
                .payload
                .insert("amount".into(), PayloadValue::Integer(i64::MAX));
        }
    }
    let error = calculate_projection(&capture, &catalog).unwrap_err();
    assert!(error.to_string().contains("damage total overflow"));
}

#[test]
fn a_later_catalog_resolves_unknown_ids_without_changing_metrics_or_raw_identity() {
    let sandbox = tempfile::tempdir().unwrap();
    let store = imported_store(sandbox.path());
    let first_catalog = catalog(sandbox.path(), LIVE_CATALOG, "first.sqlite");
    let first = project_encounter(&ProjectionRequest::new(
        &store,
        &first_catalog,
        sandbox.path().join("first.json"),
        SESSION,
        ENCOUNTER,
    ))
    .unwrap();

    let mut bundle: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(LIVE_CATALOG).unwrap()).unwrap();
    bundle["release"]["catalog_version"] = serde_json::json!("s077-live-2");
    bundle["source_records"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({
            "record_id": "live-ability-999999",
            "snapshot_id": "synthetic-live",
            "category": "player-skills",
            "source_key": "ability:999999",
            "content_sha256": "9999999999999999999999999999999999999999999999999999999999999999",
            "acquisition_method": "project-fixture",
            "import_result": "accepted"
        }));
    bundle["entities"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({
            "entity": {"kind": "ability", "stable_id": 999999},
            "observed_only": true,
            "source_records": ["live-ability-999999"]
        }));
    let input = sandbox.path().join("later.json");
    fs::write(&input, serde_json::to_vec(&bundle).unwrap()).unwrap();
    let later_catalog = sandbox.path().join("later.sqlite");
    build_catalog(&BuildRequest::new(&input, &later_catalog, Channel::Live)).unwrap();
    let later = project_encounter(&ProjectionRequest::new(
        &store,
        &later_catalog,
        sandbox.path().join("later-projection.json"),
        SESSION,
        ENCOUNTER,
    ))
    .unwrap();

    assert!(first.catalog_join.unknown_ids.contains(&999999));
    assert!(later.catalog_join.known_ids.contains(&999999));
    assert!(!later.catalog_join.unknown_ids.contains(&999999));
    assert_eq!(
        first.catalog_join.raw_content_sha256,
        later.catalog_join.raw_content_sha256
    );
    assert_eq!(first.observed_dps, later.observed_dps);
    assert_eq!(first.effective_hps, later.effective_hps);
    assert_eq!(first.ability_damage_share, later.ability_damage_share);
    assert_eq!(first.effect_uptime, later.effect_uptime);
    assert_eq!(first.ordered_cast_sequence, later.ordered_cast_sequence);
}

#[test]
fn mismatch_alias_and_link_failures_preserve_existing_output() {
    let sandbox = tempfile::tempdir().unwrap();
    let store = imported_store(sandbox.path());
    let pts = catalog(sandbox.path(), PTS_CATALOG, "pts.sqlite");
    let output = sandbox.path().join("projection.json");
    fs::write(&output, b"prior").unwrap();
    assert!(project_encounter(&ProjectionRequest::new(
        &store, &pts, &output, SESSION, ENCOUNTER,
    ))
    .is_err());
    assert_eq!(fs::read(&output).unwrap(), b"prior");

    let live = catalog(sandbox.path(), LIVE_CATALOG, "live.sqlite");
    assert!(project_encounter(&ProjectionRequest::new(
        &store, &live, &store, SESSION, ENCOUNTER,
    ))
    .is_err());

    let linked = sandbox.path().join("linked.json");
    if create_file_symlink(&output, &linked).is_ok() {
        assert!(project_encounter(&ProjectionRequest::new(
            &store, &live, &linked, SESSION, ENCOUNTER,
        ))
        .is_err());
        assert_eq!(fs::read(&output).unwrap(), b"prior");
    }
}

#[cfg(unix)]
fn create_file_symlink(target: &std::path::Path, link: &std::path::Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn create_file_symlink(target: &std::path::Path, link: &std::path::Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_file(target, link)
}
