use std::fs;

use eso_weave::catalog::compiler::{build_catalog, BuildRequest};
use eso_weave::catalog::Channel;
use eso_weave::encounter::{
    EncounterHistoryService, EncounterIdentity, HistoryDiagnosticKind, ImportOutcome, MetricQuality,
};

const CAPTURE: &str =
    include_str!("../specs/077-encounter-metrics/fixtures/encounter-metrics-capture.json");
const LIVE_CATALOG: &str = "specs/070-catalog-compiler/fixtures/minimal-live.json";
const PTS_CATALOG: &str = "specs/070-catalog-compiler/fixtures/minimal-pts.json";

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

fn catalog(root: &std::path::Path, fixture: &str, channel: Channel) -> std::path::PathBuf {
    let output = root.join(format!("{}-catalog.sqlite", channel.as_str()));
    build_catalog(&BuildRequest::new(fixture, &output, channel)).unwrap();
    output
}

#[test]
fn absent_store_is_empty_and_read_only_until_explicit_import() {
    let root = tempfile::tempdir().unwrap();
    let store = root.path().join("data/encounters/encounters.sqlite");
    let service = EncounterHistoryService::from_paths(&store, root.path().join("catalog.sqlite"));

    assert!(service.snapshot().unwrap().is_empty());
    assert!(!store.exists());
}

#[test]
fn invalid_store_ancestor_is_not_reported_as_empty_history() {
    let root = tempfile::tempdir().unwrap();
    let invalid_parent = root.path().join("not-a-directory");
    fs::write(&invalid_parent, b"ordinary file").unwrap();
    let service = EncounterHistoryService::from_paths(
        invalid_parent.join("encounters.sqlite"),
        root.path().join("catalog.sqlite"),
    );

    assert_eq!(
        service.snapshot().unwrap_err().kind,
        HistoryDiagnosticKind::StoreInvalid
    );
}

#[cfg(unix)]
#[test]
fn dangling_store_symlink_is_not_reported_as_empty_history() {
    let root = tempfile::tempdir().unwrap();
    let store = root.path().join("encounters.sqlite");
    std::os::unix::fs::symlink(root.path().join("missing.sqlite"), &store).unwrap();
    let service = EncounterHistoryService::from_paths(store, root.path().join("catalog.sqlite"));

    assert_eq!(
        service.snapshot().unwrap_err().kind,
        HistoryDiagnosticKind::StoreInvalid
    );
}

#[test]
fn explicit_import_lists_and_projects_truthful_quality_and_catalog_coverage() {
    let root = tempfile::tempdir().unwrap();
    let input = root.path().join("EsoWeaveEncounter.lua");
    fs::write(&input, capture_lua()).unwrap();
    let catalog = catalog(root.path(), LIVE_CATALOG, Channel::Live);
    let service = EncounterHistoryService::new(root.path(), &catalog);

    let first = service.import_current(&input, Channel::Live).unwrap();
    assert_eq!(first.outcome, ImportOutcome::Imported);
    let repeated = service.import_current(&input, Channel::Live).unwrap();
    assert_eq!(repeated.outcome, ImportOutcome::AlreadyPresent);

    let snapshot = service.snapshot().unwrap();
    assert_eq!(snapshot.len(), 1);
    assert_eq!(snapshot[0].omitted_event_count, 2);
    let identity = EncounterIdentity::from(&snapshot[0]);
    let projection = service.detail(&identity).unwrap();

    assert_eq!(projection.algorithm_version, "s069-v1");
    assert_eq!(projection.catalog_join.catalog_version, "s070-live-1");
    assert_eq!(projection.observed_dps.value, Some(300.0));
    assert_eq!(projection.observed_dps.quality, MetricQuality::Degraded);
    assert_eq!(projection.observed_dps.loss_ranges.len(), 1);
    assert_eq!(
        projection.observed_dps.loss_ranges[0].missing_sequence_from,
        10
    );
    assert_eq!(
        projection.observed_dps.loss_ranges[0].missing_sequence_to,
        11
    );
    assert_eq!(projection.catalog_join.known_ids, vec![100, 200]);
    assert_eq!(projection.catalog_join.unknown_ids, vec![101, 999999]);
    assert_eq!(projection.catalog_join.known_ability_ids, vec![100]);
    assert_eq!(
        projection.catalog_join.unknown_ability_ids,
        vec![101, 999999]
    );
    assert_eq!(projection.catalog_join.known_effect_ids, vec![200]);
    assert!(projection.catalog_join.unknown_effect_ids.is_empty());
}

#[test]
fn store_and_catalog_failures_have_distinct_safe_diagnostics() {
    let root = tempfile::tempdir().unwrap();
    let corrupt_store = root.path().join("corrupt.sqlite");
    fs::write(&corrupt_store, b"not sqlite").unwrap();
    let service = EncounterHistoryService::from_paths(
        &corrupt_store,
        root.path().join("missing-catalog.sqlite"),
    );
    let store_error = service.snapshot().unwrap_err();
    assert_eq!(store_error.kind, HistoryDiagnosticKind::StoreInvalid);
    assert!(!store_error
        .message
        .contains(root.path().to_string_lossy().as_ref()));

    let input = root.path().join("capture.lua");
    fs::write(&input, capture_lua()).unwrap();
    let valid_store = root.path().join("valid.sqlite");
    let service = EncounterHistoryService::from_paths(
        &valid_store,
        root.path().join("missing-catalog.sqlite"),
    );
    service.import_current(&input, Channel::Live).unwrap();
    let identity = EncounterIdentity::from(&service.snapshot().unwrap()[0]);
    assert_eq!(
        service.detail(&identity).unwrap_err().kind,
        HistoryDiagnosticKind::CatalogUnavailable
    );

    let pts = catalog(root.path(), PTS_CATALOG, Channel::Pts);
    let mismatch = EncounterHistoryService::from_paths(&valid_store, pts);
    assert_eq!(
        mismatch.detail(&identity).unwrap_err().kind,
        HistoryDiagnosticKind::VersionMismatch
    );
}

#[test]
fn explicit_deletion_removes_exactly_the_requested_records() {
    let root = tempfile::tempdir().unwrap();
    let input = root.path().join("capture.lua");
    fs::write(&input, capture_lua()).unwrap();
    let service = EncounterHistoryService::from_paths(
        root.path().join("encounters.sqlite"),
        root.path().join("catalog.sqlite"),
    );
    service.import_current(&input, Channel::Live).unwrap();
    let identity = EncounterIdentity::from(&service.snapshot().unwrap()[0]);

    assert_eq!(service.delete_one(&identity).unwrap().deleted_records, 1);
    assert!(service.snapshot().unwrap().is_empty());
    service.import_current(&input, Channel::Live).unwrap();
    assert_eq!(service.delete_all().unwrap().deleted_records, 1);
    assert!(service.snapshot().unwrap().is_empty());
}

#[test]
fn missing_identity_does_not_mutate_the_store() {
    let root = tempfile::tempdir().unwrap();
    let input = root.path().join("capture.lua");
    fs::write(&input, capture_lua()).unwrap();
    let service = EncounterHistoryService::from_paths(
        root.path().join("encounters.sqlite"),
        root.path().join("catalog.sqlite"),
    );
    service.import_current(&input, Channel::Live).unwrap();

    let missing = EncounterIdentity::new("missing-session", "missing-encounter");
    assert_eq!(
        service.detail(&missing).unwrap_err().kind,
        HistoryDiagnosticKind::EncounterMissing
    );
    assert_eq!(service.delete_one(&missing).unwrap().deleted_records, 0);
    assert_eq!(service.snapshot().unwrap().len(), 1);
}

#[test]
fn detail_projection_follows_active_catalog_path_changes() {
    let root = tempfile::tempdir().unwrap();
    let input = root.path().join("capture.lua");
    fs::write(&input, capture_lua()).unwrap();
    let live = catalog(root.path(), LIVE_CATALOG, Channel::Live);
    let pts = catalog(root.path(), PTS_CATALOG, Channel::Pts);
    let service = EncounterHistoryService::new(root.path(), &pts);
    service.import_current(&input, Channel::Live).unwrap();
    let identity = EncounterIdentity::from(&service.snapshot().unwrap()[0]);

    assert_eq!(
        service.detail(&identity).unwrap_err().kind,
        HistoryDiagnosticKind::VersionMismatch
    );
    service.set_catalog_path(live);
    assert_eq!(
        service
            .detail(&identity)
            .unwrap()
            .catalog_join
            .catalog_version,
        "s070-live-1"
    );
}
