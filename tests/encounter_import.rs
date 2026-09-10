use std::collections::BTreeMap;
use std::fs;

use eso_weave::catalog::Channel;
use eso_weave::encounter::{
    backup_store, canonical_bytes, delete_all, delete_encounter, import_encounter, list_encounters,
    load_encounter, parse_capture, EncounterEvent, ImportOutcome, ImportRequest, PayloadValue,
    MAX_CAPTURE_BYTES, MAX_EVENTS,
};

const COMPLETE: &str = include_str!("fixtures/encounter/valid-complete.lua");
const PARTIAL_JSON: &str =
    include_str!("../specs/075-encounter-capture/fixtures/representative-capture.json");

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

fn partial_lua() -> String {
    let value: serde_json::Value = serde_json::from_str(PARTIAL_JSON).unwrap();
    format!("EsoWeaveEncounterSaved = {}", json_to_lua(&value))
}

fn recovered_partial_lua() -> String {
    let mut value: serde_json::Value = serde_json::from_str(PARTIAL_JSON).unwrap();
    value["warnings"]["recovered_interruption"] = serde_json::json!(1);
    format!("EsoWeaveEncounterSaved = {}", json_to_lua(&value))
}

#[test]
fn complete_and_truthful_partial_captures_canonicalize() {
    let complete = parse_capture(COMPLETE.as_bytes(), Channel::Live).unwrap();
    assert_eq!(complete.events.len(), 2);
    let first = canonical_bytes(&complete).unwrap();
    let second = canonical_bytes(&complete).unwrap();
    assert_eq!(first, second);

    let partial = parse_capture(partial_lua().as_bytes(), Channel::Live).unwrap();
    assert_eq!(partial.events.len(), 14);
    assert_eq!(partial.omitted_event_count, 1);
    assert_eq!(
        partial.events[12].payload["missing_sequence_from"],
        PayloadValue::Integer(13)
    );

    let recovered = parse_capture(recovered_partial_lua().as_bytes(), Channel::Live).unwrap();
    assert_eq!(recovered.warnings["recovered_interruption"], 1);
}

#[test]
fn parser_and_validator_reject_hostile_or_inconsistent_inputs() {
    for invalid in [
        "EsoWeaveEncounterSaved = function() return {} end".to_string(),
        "Other = {}".to_string(),
        format!("{COMPLETE}; os.execute(\"x\")"),
        COMPLETE.replace("[\"channel\"] = \"live\"", "[\"channel\"] = \"pts\""),
        COMPLETE.replace("[\"last_sequence\"] = 2", "[\"last_sequence\"] = 3"),
        COMPLETE.replace("[\"monotonic_ms\"] = 1000", "[\"monotonic_ms\"] = 999"),
        COMPLETE.replace(
            "[\"reason\"] = \"combat-ended\"",
            "[\"reason\"] = \"player name\"",
        ),
        COMPLETE.replace(
            "[\"game_version\"] = \"12.0.7\"",
            "[\"game_version\"] = \"@account\"",
        ),
        COMPLETE.replace(
            "[\"payload\"] = { [\"reason\"] = \"combat-started\" }",
            "[\"payload\"] = { [\"reason\"] = \"combat-started\", [\"name\"] = \"@account\" }",
        ),
    ] {
        assert!(parse_capture(invalid.as_bytes(), Channel::Live).is_err());
    }
    assert!(parse_capture(COMPLETE.as_bytes(), Channel::Pts).is_err());
}

#[test]
fn import_is_atomic_immutable_idempotent_and_collision_safe() {
    let sandbox = tempfile::tempdir().unwrap();
    let input = sandbox.path().join("capture.lua");
    let store = sandbox.path().join("encounters.sqlite");
    fs::write(&input, COMPLETE).unwrap();

    let first = import_encounter(&ImportRequest::new(&input, &store, Channel::Live)).unwrap();
    assert_eq!(first.outcome, ImportOutcome::Imported);
    let second = import_encounter(&ImportRequest::new(&input, &store, Channel::Live)).unwrap();
    assert_eq!(second.outcome, ImportOutcome::AlreadyPresent);
    assert_eq!(list_encounters(&store).unwrap().len(), 1);
    assert_eq!(
        load_encounter(&store, &first.session_id, &first.encounter_id)
            .unwrap()
            .unwrap()
            .events
            .len(),
        2
    );

    let connection = rusqlite::Connection::open(&store).unwrap();
    assert!(connection
        .execute("UPDATE raw_encounters SET status = 'partial'", [])
        .is_err());
    drop(connection);

    fs::write(
        &input,
        COMPLETE.replace(
            "[\"finished_at\"] = \"1788912001\"",
            "[\"finished_at\"] = \"1788912002\"",
        ),
    )
    .unwrap();
    assert!(import_encounter(&ImportRequest::new(&input, &store, Channel::Live)).is_err());
    assert_eq!(list_encounters(&store).unwrap().len(), 1);

    fs::write(&input, "EsoWeaveEncounterSaved = broken").unwrap();
    assert!(import_encounter(&ImportRequest::new(&input, &store, Channel::Live)).is_err());
    assert_eq!(list_encounters(&store).unwrap().len(), 1);
}

#[test]
fn backup_and_explicit_deletion_preserve_user_ownership() {
    let sandbox = tempfile::tempdir().unwrap();
    let input = sandbox.path().join("capture.lua");
    let store = sandbox.path().join("encounters.sqlite");
    let backup = sandbox.path().join("backup.sqlite");
    fs::write(&input, COMPLETE).unwrap();
    let imported = import_encounter(&ImportRequest::new(&input, &store, Channel::Live)).unwrap();

    let receipt = backup_store(&store, &backup).unwrap();
    assert!(receipt.byte_length > 0);
    assert_eq!(receipt.sha256.len(), 64);
    assert_eq!(list_encounters(&backup).unwrap().len(), 1);

    let tampered = sandbox.path().join("tampered.sqlite");
    fs::copy(&backup, &tampered).unwrap();
    let connection = rusqlite::Connection::open(&tampered).unwrap();
    connection
        .execute_batch(
            "DROP TRIGGER raw_encounters_no_update;
             UPDATE raw_encounters SET canonical_json = X'00';
             CREATE TRIGGER raw_encounters_no_update
             BEFORE UPDATE ON raw_encounters
             BEGIN
                 SELECT RAISE(ABORT, 'raw encounter records are immutable');
             END;",
        )
        .unwrap();
    drop(connection);
    assert!(backup_store(&tampered, sandbox.path().join("rejected.sqlite")).is_err());
    assert!(delete_all(&tampered).is_err());
    assert!(import_encounter(&ImportRequest::new(&input, &tampered, Channel::Live)).is_err());
    let connection = rusqlite::Connection::open(&tampered).unwrap();
    let remaining: i64 = connection
        .query_row("SELECT count(*) FROM raw_encounters", [], |row| row.get(0))
        .unwrap();
    assert_eq!(remaining, 1);
    drop(connection);

    let deleted = delete_encounter(&store, &imported.session_id, &imported.encounter_id).unwrap();
    assert_eq!(deleted.deleted_records, 1);
    assert!(list_encounters(&store).unwrap().is_empty());
    assert_eq!(list_encounters(&backup).unwrap().len(), 1);

    fs::write(&input, partial_lua()).unwrap();
    import_encounter(&ImportRequest::new(&input, &store, Channel::Live)).unwrap();
    assert_eq!(delete_all(&store).unwrap().deleted_records, 1);
    let replacement = backup_store(&store, &backup).unwrap();
    assert_ne!(replacement.sha256, receipt.sha256);
    assert!(list_encounters(&backup).unwrap().is_empty());
}

#[test]
fn corrupt_unrecognized_and_future_stores_are_not_replaced() {
    let sandbox = tempfile::tempdir().unwrap();
    let input = sandbox.path().join("capture.lua");
    fs::write(&input, COMPLETE).unwrap();
    let corrupt = sandbox.path().join("corrupt.sqlite");
    fs::write(&corrupt, b"not sqlite").unwrap();
    let before = fs::read(&corrupt).unwrap();
    assert!(import_encounter(&ImportRequest::new(&input, &corrupt, Channel::Live)).is_err());
    assert_eq!(fs::read(&corrupt).unwrap(), before);

    let unrelated = sandbox.path().join("unrelated.sqlite");
    let connection = rusqlite::Connection::open(&unrelated).unwrap();
    connection
        .execute("CREATE TABLE other(value INTEGER)", [])
        .unwrap();
    drop(connection);
    assert!(import_encounter(&ImportRequest::new(&input, &unrelated, Channel::Live)).is_err());

    let future = sandbox.path().join("future.sqlite");
    let connection = rusqlite::Connection::open(&future).unwrap();
    connection.pragma_update(None, "user_version", 2).unwrap();
    drop(connection);
    assert!(import_encounter(&ImportRequest::new(&input, &future, Channel::Live)).is_err());
}

#[test]
fn altered_schema_objects_are_rejected() {
    let sandbox = tempfile::tempdir().unwrap();
    let input = sandbox.path().join("capture.lua");
    fs::write(&input, COMPLETE).unwrap();

    let altered_trigger = sandbox.path().join("altered-trigger.sqlite");
    import_encounter(&ImportRequest::new(&input, &altered_trigger, Channel::Live)).unwrap();
    let connection = rusqlite::Connection::open(&altered_trigger).unwrap();
    connection
        .execute_batch(
            "DROP TRIGGER raw_encounters_no_update;
             CREATE TRIGGER raw_encounters_no_update
             AFTER INSERT ON raw_encounters
             BEGIN
                 SELECT 1;
             END;",
        )
        .unwrap();
    drop(connection);
    assert!(list_encounters(&altered_trigger).is_err());
    assert!(delete_all(&altered_trigger).is_err());

    let altered_table = sandbox.path().join("altered-table.sqlite");
    import_encounter(&ImportRequest::new(&input, &altered_table, Channel::Live)).unwrap();
    let connection = rusqlite::Connection::open(&altered_table).unwrap();
    connection
        .execute_batch("ALTER TABLE raw_encounters RENAME COLUMN source_sha256 TO source_digest;")
        .unwrap();
    drop(connection);
    assert!(list_encounters(&altered_table).is_err());
    assert!(delete_all(&altered_table).is_err());
}

#[test]
fn production_event_ceiling_is_canonicalizable() {
    let mut capture = parse_capture(COMPLETE.as_bytes(), Channel::Live).unwrap();
    let mut terminal = capture.events.pop().unwrap();
    capture.events.reserve(MAX_EVENTS - 2);
    for sequence in 2..MAX_EVENTS as u64 {
        capture.events.push(EncounterEvent {
            session_id: capture.session_id.clone(),
            encounter_id: capture.encounter_id.clone(),
            sequence,
            monotonic_ms: sequence,
            kind: "performance".into(),
            payload: BTreeMap::from([
                ("frames_per_second".into(), PayloadValue::Integer(60)),
                ("latency_ms".into(), PayloadValue::Integer(45)),
            ]),
        });
    }
    terminal.sequence = MAX_EVENTS as u64;
    terminal.monotonic_ms = MAX_EVENTS as u64;
    capture.events.push(terminal);
    capture.ended_monotonic_ms = MAX_EVENTS as u64;
    capture.last_sequence = MAX_EVENTS as u64;
    capture.stored_event_count = MAX_EVENTS;
    let canonical = canonical_bytes(&capture).expect("the S075 event ceiling remains valid");
    assert!(canonical.len() > 10_000_000);
}

#[test]
fn file_boundaries_reject_aliases_oversize_and_link_like_inputs() {
    let sandbox = tempfile::tempdir().unwrap();
    let input = sandbox.path().join("capture.lua");
    fs::write(&input, COMPLETE).unwrap();
    assert!(import_encounter(&ImportRequest::new(&input, &input, Channel::Live)).is_err());

    let oversized = sandbox.path().join("oversized.lua");
    let file = fs::File::create(&oversized).unwrap();
    file.set_len(MAX_CAPTURE_BYTES + 1).unwrap();
    drop(file);
    let store = sandbox.path().join("encounters.sqlite");
    assert!(import_encounter(&ImportRequest::new(&oversized, &store, Channel::Live)).is_err());
    assert!(!store.exists());

    let linked = sandbox.path().join("linked.lua");
    if create_file_symlink(&input, &linked).is_ok() {
        assert!(import_encounter(&ImportRequest::new(&linked, &store, Channel::Live)).is_err());
        assert!(!store.exists());
    }

    import_encounter(&ImportRequest::new(&input, &store, Channel::Live)).unwrap();
    let linked_store = sandbox.path().join("linked-store.sqlite");
    if create_file_symlink(&store, &linked_store).is_ok() {
        assert!(list_encounters(&linked_store).is_err());
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
