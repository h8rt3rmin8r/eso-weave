//! Retained history domain access after removal of the desktop viewer.
use eso_weave::catalog::compiler::{build_catalog, BuildRequest};
use eso_weave::catalog::Channel;
use eso_weave::encounter::{
    EncounterHistoryService, EncounterIdentity, HistoryDiagnosticKind, ImportOutcome,
};
use eso_weave::recommendation::generate_recommendations;
use std::fs;
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

#[test]
fn historical_metrics_and_recommendations_remain_readable_without_a_viewer() {
    let root = tempfile::tempdir().unwrap();
    let service = seeded_service(root.path());
    let identity = EncounterIdentity::from(&service.snapshot().unwrap()[0]);
    let projection = service.detail(&identity).unwrap();
    assert_eq!(projection.observed_dps.value, Some(300.0));
    let report = generate_recommendations(&projection);
    assert_eq!(
        report.evidence.citation.raw_content_sha256,
        projection.raw_content_sha256
    );
    assert_eq!(
        report.evidence.citation.catalog_version,
        projection.catalog_join.catalog_version
    );
}

#[test]
fn catalog_replacement_requalifies_retained_detail_without_changing_source() {
    let root = tempfile::tempdir().unwrap();
    let service = seeded_service(root.path());
    let source = root.path().join("capture.lua");
    let before = fs::read(&source).unwrap();
    let identity = EncounterIdentity::from(&service.snapshot().unwrap()[0]);
    let pts = root.path().join("pts.sqlite");
    build_catalog(&BuildRequest::new(
        "specs/070-catalog-compiler/fixtures/minimal-pts.json",
        &pts,
        Channel::Pts,
    ))
    .unwrap();
    service.set_catalog_path(&pts);
    assert_eq!(
        service.detail(&identity).unwrap_err().kind,
        HistoryDiagnosticKind::VersionMismatch
    );
    service.set_catalog_path(root.path().join("catalog.sqlite"));
    assert_eq!(
        service
            .detail(&identity)
            .unwrap()
            .catalog_join
            .catalog_version,
        "s070-live-1"
    );
    assert_eq!(fs::read(source).unwrap(), before);
}

#[test]
fn missing_catalog_keeps_historical_rows_and_source_bytes() {
    let root = tempfile::tempdir().unwrap();
    let service = seeded_service(root.path());
    let before = fs::read(root.path().join("capture.lua")).unwrap();
    let identity = EncounterIdentity::from(&service.snapshot().unwrap()[0]);
    service.set_catalog_path(root.path().join("missing.sqlite"));
    assert_eq!(
        service.detail(&identity).unwrap_err().kind,
        HistoryDiagnosticKind::CatalogUnavailable
    );
    assert_eq!(service.snapshot().unwrap().len(), 1);
    assert_eq!(fs::read(root.path().join("capture.lua")).unwrap(), before);
}
