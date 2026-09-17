use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use eso_weave::database_query::{
    MAX_CONCURRENT_QUERIES, MAX_DURATION, MAX_PARAMETERS, MAX_RESULT_BYTES, MAX_ROWS, MAX_SQL_BYTES,
};
use eso_weave::local_service::{DEFAULT_PORT, EXTERNAL_SCHEMA_VERSION, MAX_REQUEST_BODY_BYTES};
use eso_weave::player_state::{PUBLIC_PATHS, SCHEMA_VERSION};
use serde::Deserialize;

const GUIDE: &str = "docs/src/reference/local-api-and-mcp.md";
const INVENTORY: &str = "docs/src/reference/local-extension-state-fields.json";
const WARNING: &str = "Connected clients can read live player state and query ESO Weave application data. Enable only for local clients you trust.";

#[derive(Debug, Deserialize)]
struct DocumentedInventory {
    schema_version: String,
    fields: Vec<DocumentedField>,
}

#[derive(Debug, Deserialize)]
struct DocumentedField {
    path: String,
    #[serde(rename = "type")]
    value_type: String,
    meaning: String,
    source: String,
    availability: String,
}

#[test]
fn canonical_guide_names_the_shipped_surface_and_safe_examples() {
    let guide = read(GUIDE);
    let summary = read("docs/src/SUMMARY.md");
    let reference_index = read("docs/src/reference/README.md");
    let ui_strings = read("src/app/strings.rs");

    assert!(summary.contains("reference/local-api-and-mcp.md"));
    assert!(reference_index.contains("local-api-and-mcp.md"));
    assert!(guide.contains("Local API and MCP Server"));
    assert!(guide.contains(WARNING));
    assert!(ui_strings.contains(WARNING));

    for required in [
        "GET /api/v1/capabilities",
        "GET /api/v1/player-state",
        "GET /api/v1/databases",
        "POST /api/v1/databases/{database_id}/query",
        "esoweave://capabilities",
        "esoweave://player-state",
        "esoweave://databases",
        "query_database",
        "ESOWEAVE_TOKEN",
        "Authorization",
        "Bearer",
        "snapshot_revision",
        "service_generation",
        "unknown",
        "unavailable",
        "dormant",
        "fresh",
        "stale",
        "i64::to_string()",
        "positive_infinity",
        "negative_infinity",
        "nan",
    ] {
        assert!(guide.contains(required), "guide is missing {required}");
    }

    for value in [
        DEFAULT_PORT.to_string(),
        MAX_REQUEST_BODY_BYTES.to_string(),
        MAX_SQL_BYTES.to_string(),
        MAX_PARAMETERS.to_string(),
        MAX_CONCURRENT_QUERIES.to_string(),
        MAX_DURATION.as_millis().to_string(),
        MAX_ROWS.to_string(),
        MAX_RESULT_BYTES.to_string(),
    ] {
        assert!(guide.contains(&value), "guide is missing bound {value}");
    }

    assert!(guide.contains(INVENTORY.rsplit('/').next().unwrap()));
    assert!(!guide.contains(TEST_CREDENTIAL_SHAPE));
    assert!(!guide.contains("0.0.0.0"));
    assert!(!guide.contains("DELETE FROM"));
}

#[test]
fn documented_field_inventory_matches_production_exactly() {
    let bytes = fs::read(root().join(INVENTORY)).expect("read documented field inventory");
    assert!(!bytes.starts_with(&[0xef, 0xbb, 0xbf]));
    let inventory: DocumentedInventory =
        serde_json::from_slice(&bytes).expect("parse documented field inventory");

    assert_eq!(inventory.schema_version, SCHEMA_VERSION);
    assert_eq!(inventory.schema_version, EXTERNAL_SCHEMA_VERSION);
    let documented = inventory
        .fields
        .iter()
        .map(|field| field.path.as_str())
        .collect::<Vec<_>>();
    let documented_set = documented.iter().copied().collect::<BTreeSet<_>>();
    let production_set = PUBLIC_PATHS.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(
        documented.len(),
        documented_set.len(),
        "duplicate documented path"
    );
    assert_eq!(
        documented_set, production_set,
        "documented field inventory drift"
    );
    assert_eq!(
        documented, PUBLIC_PATHS,
        "inventory order must follow production"
    );

    for field in &inventory.fields {
        assert!(!field.value_type.trim().is_empty(), "{} type", field.path);
        assert!(!field.meaning.trim().is_empty(), "{} meaning", field.path);
        assert!(!field.source.trim().is_empty(), "{} source", field.path);
        assert!(
            !field.availability.trim().is_empty(),
            "{} availability",
            field.path
        );
    }

    let field = |path: &str| {
        inventory
            .fields
            .iter()
            .find(|field| field.path == path)
            .unwrap_or_else(|| panic!("missing documented field {path}"))
    };
    assert!(field("game.context").meaning.contains("dormant"));
    assert!(field("game.context").meaning.contains("active"));
    assert!(field("interpretation.weave.front_timing")
        .meaning
        .contains("effective_delays_ms"));
    assert!(field("interpretation.weave.back_timing")
        .meaning
        .contains("effective_delays_ms"));
    assert!(field("interpretation.latency")
        .meaning
        .contains("current observed latency"));
}

const TEST_CREDENTIAL_SHAPE: &str =
    "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(relative: &str) -> String {
    fs::read_to_string(root().join(relative))
        .unwrap_or_else(|error| panic!("read {relative}: {error}"))
}
