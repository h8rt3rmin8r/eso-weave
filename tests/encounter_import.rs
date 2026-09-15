use std::collections::BTreeMap;
use std::fs;

use eso_weave::catalog::Channel;
use eso_weave::encounter::{
    assess_replay, backup_store, canonical_bytes, delete_all, delete_encounter, import_capture_set,
    import_encounter, list_encounters, load_encounter, parse_capture, parse_capture_set,
    CaptureMode, EncounterEvent, ImportOutcome, ImportRequest, PayloadValue, RawObservation,
    RawSourceKind, RawValue, RawValueType, ReplayAssessment, MAX_CAPTURE_BYTES,
    MAX_ESTIMATED_BYTES, MAX_EVENTS, STORE_SCHEMA_VERSION,
};
use sha2::{Digest, Sha256};

const COMPLETE: &str = include_str!("fixtures/encounter/valid-complete.lua");
const LOSSLESS_V2_JSON: &str = include_str!("fixtures/encounter/valid-v2-lossless.json");
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
    format!(
        "EsoWeaveDataSaved = {{ [\"schema_version\"] = 1, [\"addon_version\"] = 1, [\"encounter\"] = {} }}",
        json_to_lua(&value)
    )
}

fn recovered_partial_lua() -> String {
    let mut value: serde_json::Value = serde_json::from_str(PARTIAL_JSON).unwrap();
    value["warnings"]["recovered_interruption"] = serde_json::json!(1);
    format!(
        "EsoWeaveDataSaved = {{ [\"schema_version\"] = 1, [\"addon_version\"] = 1, [\"encounter\"] = {} }}",
        json_to_lua(&value)
    )
}

fn lossless_v2() -> serde_json::Value {
    serde_json::from_str(LOSSLESS_V2_JSON).expect("valid synthetic capture v2 fixture")
}

fn capture_lua(value: &serde_json::Value) -> String {
    format!(
        "EsoWeaveDataSaved = {{ [\"schema_version\"] = 1, [\"addon_version\"] = 1, [\"encounter\"] = {} }}",
        json_to_lua(value)
    )
}

fn lossless_v2_lua() -> String {
    capture_lua(&lossless_v2())
}

fn current_terminal(session_id: &str, encounter_id: &str) -> serde_json::Value {
    let mut capture = lossless_v2();
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

fn capture_state(records: Vec<serde_json::Value>, revision: u64) -> serde_json::Value {
    capture_state_for("session-1788912002-2000", records, revision)
}

fn capture_state_for(
    session_id: &str,
    records: Vec<serde_json::Value>,
    revision: u64,
) -> serde_json::Value {
    let record_map = records
        .into_iter()
        .enumerate()
        .map(|(index, capture)| {
            (
                format!("{:010}", index + 1),
                serde_json::json!({"ordinal": index + 1, "capture": capture}),
            )
        })
        .collect::<serde_json::Map<_, _>>();
    let completed = record_map.len();
    serde_json::json!({
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
            "next_encounter_ordinal": completed + 1,
            "completed_encounter_count": completed,
            "degraded_encounter_count": completed,
            "aggregate_estimated_bytes": completed * 4096,
            "aggregate_event_count": completed * 2,
            "aggregate_raw_observation_count": completed * 4,
            "interruption_count": 0
        },
        "records": record_map,
        "interruptions": [],
        "revision": revision
    })
}

fn state_lua(state: &serde_json::Value) -> String {
    format!(
        "EsoWeaveDataSaved = {{ [\"schema_version\"] = 1, [\"addon_version\"] = 1, [\"encounter\"] = {} }}",
        json_to_lua(state)
    )
}

fn partial_v2_with_raw_loss() -> serde_json::Value {
    let mut capture = lossless_v2();
    capture["status"] = serde_json::json!("partial");
    capture["partial_reason"] = serde_json::json!("record-limit");
    capture["raw_last_sequence"] = serde_json::json!(5);
    capture["raw_observation_count"] = serde_json::json!(3);
    capture["raw_omitted_observation_count"] = serde_json::json!(2);
    capture["raw_loss"] = serde_json::json!({
        "missing_sequence_from": 3,
        "missing_sequence_to": 4,
        "reason": "record-limit"
    });
    capture["raw_observations"]
        .as_array_mut()
        .unwrap()
        .remove(2);
    capture["raw_observations"][2]["sequence"] = serde_json::json!(5);
    capture["events"][1]["payload"] = serde_json::json!({
        "complete": false,
        "reason": "record-limit"
    });
    capture["events"][1]["source_sequence"] = serde_json::json!(5);
    capture
}

#[test]
fn state_schema_dispatch_imports_only_terminal_records_in_authoritative_order() {
    let session = "session-1788912002-2000";
    let state = capture_state(
        vec![
            current_terminal(session, "encounter-1788912002-1"),
            current_terminal(session, "encounter-1788912002-2"),
        ],
        7,
    );
    let source = state_lua(&state);
    let parsed = parse_capture_set(source.as_bytes(), Channel::Live).unwrap();
    assert_eq!(parsed.records.len(), 2);
    assert_eq!(parsed.records[0].mode, CaptureMode::Continuous);
    assert_eq!(parsed.records[0].ordinal, 1);
    assert_eq!(parsed.records[1].ordinal, 2);

    let sandbox = tempfile::tempdir().unwrap();
    let input = sandbox.path().join("continuous.lua");
    let store = sandbox.path().join("encounters.sqlite");
    fs::write(&input, source).unwrap();
    let report = import_capture_set(&ImportRequest::new(&input, &store, Channel::Live)).unwrap();
    assert_eq!(report.imported_count, 2);
    assert_eq!(report.already_present_count, 0);
    assert_eq!(report.receipts.len(), 2);
    let public_report = serde_json::to_string(&report).unwrap();
    assert!(!public_report.contains("raw_observations"));
    assert!(!public_report.contains("events"));
    assert!(!public_report.contains("records"));
    assert!(!public_report.contains("normalization_profile"));
    assert_eq!(list_encounters(&store).unwrap().len(), 2);
}

#[test]
fn outer_controller_keeps_a_wrapped_legacy_terminal_importable() {
    let capture: serde_json::Value = serde_json::from_str(PARTIAL_JSON).unwrap();
    let session_id = capture["session_id"].as_str().unwrap();
    let state = serde_json::json!({
        "state_schema_version": 1,
        "addon_version": 4,
        "selected_mode": "single",
        "selected_channel": "live",
        "state": "stopped",
        "session": {
            "session_id": session_id,
            "mode": "single",
            "channel": "live",
            "status": "stopped",
            "started_at": capture["started_at"],
            "finished_at": capture["finished_at"],
            "next_encounter_ordinal": 2,
            "completed_encounter_count": 1,
            "degraded_encounter_count": 1,
            "aggregate_estimated_bytes": capture["estimated_bytes"],
            "aggregate_event_count": capture["stored_event_count"],
            "aggregate_raw_observation_count": 0,
            "interruption_count": 0
        },
        "records": {
            "0000000001": {"ordinal": 1, "capture": capture}
        },
        "interruptions": [],
        "stop_reason": "single-partial",
        "revision": 1
    });
    let parsed = parse_capture_set(state_lua(&state).as_bytes(), Channel::Live).unwrap();
    assert_eq!(parsed.records.len(), 1);
    assert_eq!(parsed.records[0].mode, CaptureMode::Single);
    assert_eq!(parsed.records[0].ordinal, 1);
    assert_eq!(parsed.records[0].capture.schema_version, 1);

    let capture = lossless_v2();
    let session_id = capture["session_id"].as_str().unwrap();
    let state = serde_json::json!({
        "state_schema_version": 1,
        "addon_version": 4,
        "selected_mode": "single",
        "selected_channel": "live",
        "state": "stopped",
        "session": {
            "session_id": session_id,
            "mode": "single",
            "channel": "live",
            "status": "stopped",
            "started_at": capture["started_at"],
            "finished_at": capture["finished_at"],
            "next_encounter_ordinal": 2,
            "completed_encounter_count": 1,
            "degraded_encounter_count": 0,
            "aggregate_estimated_bytes": capture["estimated_bytes"],
            "aggregate_event_count": capture["stored_event_count"],
            "aggregate_raw_observation_count": capture["raw_observation_count"],
            "interruption_count": 0
        },
        "records": {
            "0000000001": {"ordinal": 1, "capture": capture}
        },
        "interruptions": [],
        "stop_reason": "single-complete",
        "revision": 1
    });
    let parsed = parse_capture_set(state_lua(&state).as_bytes(), Channel::Live).unwrap();
    assert_eq!(parsed.records.len(), 1);
    assert_eq!(parsed.records[0].capture.schema_version, 2);
    assert_eq!(parsed.records[0].capture.addon_version, 2);
}

#[test]
fn state_schema_validation_rejects_unknown_fields_sparse_ordinals_and_unbounded_ids() {
    let session = "session-1788912002-2000";
    let valid = capture_state(vec![current_terminal(session, "encounter-1788912002-1")], 2);
    let mut unknown = valid.clone();
    unknown["hostile_canary"] = serde_json::json!("never echo this");
    let diagnostic = parse_capture_set(state_lua(&unknown).as_bytes(), Channel::Live)
        .unwrap_err()
        .to_string();
    assert!(!diagnostic.contains("never echo this"));

    let mut sparse = valid.clone();
    let first = sparse["records"]
        .as_object_mut()
        .unwrap()
        .remove("0000000001")
        .unwrap();
    sparse["records"]["0000000002"] = first;
    assert!(parse_capture_set(state_lua(&sparse).as_bytes(), Channel::Live).is_err());

    let oversized = format!("session-{}-1", "1".repeat(129));
    let mut unbounded = capture_state(
        vec![current_terminal(&oversized, "encounter-1788912002-1")],
        2,
    );
    unbounded["session"]["session_id"] = serde_json::json!(oversized);
    assert!(parse_capture_set(state_lua(&unbounded).as_bytes(), Channel::Live).is_err());

    let terminal = current_terminal(session, "encounter-1788912002-1");
    let records = (1..=1024)
        .map(|ordinal| {
            (
                format!("{ordinal:010}"),
                serde_json::json!({"ordinal": ordinal, "capture": terminal}),
            )
        })
        .collect::<serde_json::Map<_, _>>();
    let mut terminal_plus_active = capture_state(Vec::new(), 2);
    terminal_plus_active["records"] = serde_json::Value::Object(records);
    terminal_plus_active["current"] = serde_json::json!({});
    assert!(parse_capture_set(state_lua(&terminal_plus_active).as_bytes(), Channel::Live).is_err());
}

#[test]
fn batch_import_is_all_or_nothing_and_growing_reimports_are_idempotent() {
    let session = "session-1788912002-2000";
    let sandbox = tempfile::tempdir().unwrap();
    let input = sandbox.path().join("continuous.lua");
    let store = sandbox.path().join("encounters.sqlite");
    let first = capture_state(vec![current_terminal(session, "encounter-1788912002-1")], 2);
    fs::write(&input, state_lua(&first)).unwrap();
    let initial = import_capture_set(&ImportRequest::new(&input, &store, Channel::Live)).unwrap();
    assert_eq!(
        (initial.imported_count, initial.already_present_count),
        (1, 0)
    );

    let growing = capture_state(
        vec![
            current_terminal(session, "encounter-1788912002-1"),
            current_terminal(session, "encounter-1788912002-2"),
        ],
        4,
    );
    fs::write(&input, state_lua(&growing)).unwrap();
    let expanded = import_capture_set(&ImportRequest::new(&input, &store, Channel::Live)).unwrap();
    assert_eq!(
        (expanded.imported_count, expanded.already_present_count),
        (1, 1)
    );
    let repeated = import_capture_set(&ImportRequest::new(&input, &store, Channel::Live)).unwrap();
    assert_eq!(
        (repeated.imported_count, repeated.already_present_count),
        (0, 2)
    );

    let mut colliding = growing;
    colliding["records"]["0000000002"]["capture"]["warnings"]["actor_limit"] = serde_json::json!(1);
    fs::write(&input, state_lua(&colliding)).unwrap();
    assert!(import_capture_set(&ImportRequest::new(&input, &store, Channel::Live)).is_err());
    assert_eq!(list_encounters(&store).unwrap().len(), 2);
}

#[test]
fn encounter_listing_keeps_overlapping_sessions_contiguous_and_ordinal() {
    let first_session = "session-1788912002-2000";
    let second_session = "session-1788912003-3000";
    let mut first = current_terminal(first_session, "encounter-1788912002-1");
    first["started_at"] = serde_json::json!("1788912002");
    first["finished_at"] = serde_json::json!("1788912003");
    let mut second = current_terminal(first_session, "encounter-1788912002-2");
    second["started_at"] = serde_json::json!("1788912006");
    second["finished_at"] = serde_json::json!("1788912007");
    let first_state = capture_state_for(first_session, vec![first, second], 3);

    let mut overlapping = current_terminal(second_session, "encounter-1788912003-1");
    overlapping["started_at"] = serde_json::json!("1788912004");
    overlapping["finished_at"] = serde_json::json!("1788912005");
    let second_state = capture_state_for(second_session, vec![overlapping], 2);

    let sandbox = tempfile::tempdir().unwrap();
    let input = sandbox.path().join("continuous.lua");
    let store = sandbox.path().join("encounters.sqlite");
    fs::write(&input, state_lua(&first_state)).unwrap();
    import_capture_set(&ImportRequest::new(&input, &store, Channel::Live)).unwrap();
    fs::write(&input, state_lua(&second_state)).unwrap();
    import_capture_set(&ImportRequest::new(&input, &store, Channel::Live)).unwrap();

    let listed = list_encounters(&store).unwrap();
    assert_eq!(listed.len(), 3);
    assert_eq!(listed[0].session_id, first_session);
    assert_eq!(listed[0].encounter_ordinal, 1);
    assert_eq!(listed[1].session_id, first_session);
    assert_eq!(listed[1].encounter_ordinal, 2);
    assert_eq!(listed[2].session_id, second_session);
    assert_eq!(listed[2].encounter_ordinal, 1);
}

#[test]
fn session_snapshots_reject_revision_reuse_and_member_prefix_regression() {
    let session = "session-1788912002-2000";
    let sandbox = tempfile::tempdir().unwrap();
    let input = sandbox.path().join("continuous.lua");
    let store = sandbox.path().join("encounters.sqlite");
    let two = capture_state(
        vec![
            current_terminal(session, "encounter-1788912002-1"),
            current_terminal(session, "encounter-1788912002-2"),
        ],
        4,
    );
    fs::write(&input, state_lua(&two)).unwrap();
    import_capture_set(&ImportRequest::new(&input, &store, Channel::Live)).unwrap();

    let mut reused = two.clone();
    reused["state"] = serde_json::json!("interrupted");
    reused["active_mode"] = serde_json::Value::Null;
    reused["interruptions"] = serde_json::json!([{
        "sequence": 1, "occurred_at": "1788912003",
        "after_encounter_ordinal": 2, "reason": "runtime-interrupted"
    }]);
    reused["session"]["interruption_count"] = serde_json::json!(1);
    assert!(parse_capture_set(state_lua(&reused).as_bytes(), Channel::Live).is_ok());
    fs::write(&input, state_lua(&reused)).unwrap();
    assert!(import_capture_set(&ImportRequest::new(&input, &store, Channel::Live)).is_err());

    let regressed = capture_state(vec![current_terminal(session, "encounter-1788912002-1")], 5);
    fs::write(&input, state_lua(&regressed)).unwrap();
    assert!(import_capture_set(&ImportRequest::new(&input, &store, Channel::Live)).is_err());
    assert_eq!(list_encounters(&store).unwrap().len(), 2);
}

#[test]
fn session_snapshots_authenticate_present_member_mode_and_ordinal_indexes() {
    let session = "session-1788912002-2000";
    let sandbox = tempfile::tempdir().unwrap();
    let input = sandbox.path().join("continuous.lua");
    let store = sandbox.path().join("encounters.sqlite");
    let state = capture_state(vec![current_terminal(session, "encounter-1788912002-1")], 2);
    fs::write(&input, state_lua(&state)).unwrap();
    import_capture_set(&ImportRequest::new(&input, &store, Channel::Live)).unwrap();

    let connection = rusqlite::Connection::open(&store).unwrap();
    connection
        .execute_batch(
            "DROP TRIGGER raw_encounters_no_update;
             UPDATE raw_encounters
             SET capture_mode = 'single', encounter_ordinal = 7;
             CREATE TRIGGER raw_encounters_no_update
             BEFORE UPDATE ON raw_encounters
             BEGIN
                 SELECT RAISE(ABORT, 'raw encounter records are immutable');
             END;",
        )
        .unwrap();
    drop(connection);

    let diagnostic = list_encounters(&store).unwrap_err().to_string();
    assert!(diagnostic.contains("member does not match its session snapshot"));
}

#[test]
fn exact_one_state_import_advances_snapshot_and_omitted_members_fail_atomically() {
    let session = "session-1788912002-2000";
    let sandbox = tempfile::tempdir().unwrap();
    let input = sandbox.path().join("continuous.lua");
    let store = sandbox.path().join("encounters.sqlite");

    fs::write(&input, state_lua(&capture_state(Vec::new(), 1))).unwrap();
    let empty = import_capture_set(&ImportRequest::new(&input, &store, Channel::Live)).unwrap();
    assert!(empty.receipts.is_empty());

    let one = capture_state(vec![current_terminal(session, "encounter-1788912002-1")], 2);
    fs::write(&input, state_lua(&one)).unwrap();
    let receipt = import_encounter(&ImportRequest::new(&input, &store, Channel::Live)).unwrap();
    assert_eq!(receipt.encounter_ordinal, 1);
    assert_eq!(list_encounters(&store).unwrap().len(), 1);

    let two = capture_state(
        vec![
            current_terminal(session, "encounter-1788912002-1"),
            current_terminal(session, "encounter-1788912002-2"),
        ],
        4,
    );
    fs::write(&input, state_lua(&two)).unwrap();
    import_capture_set(&ImportRequest::new(&input, &store, Channel::Live)).unwrap();
    fs::write(&input, state_lua(&one)).unwrap();
    let older_retry =
        import_capture_set(&ImportRequest::new(&input, &store, Channel::Live)).unwrap();
    assert_eq!(older_retry.already_present_count, 1);
    assert_eq!(list_encounters(&store).unwrap().len(), 2);

    let separate_store = sandbox.path().join("legacy.sqlite");
    let standalone = current_terminal(session, "encounter-1788912002-1");
    fs::write(&input, capture_lua(&standalone)).unwrap();
    import_encounter(&ImportRequest::new(&input, &separate_store, Channel::Live)).unwrap();
    fs::write(&input, state_lua(&capture_state(Vec::new(), 3))).unwrap();
    assert!(
        import_capture_set(&ImportRequest::new(&input, &separate_store, Channel::Live)).is_err()
    );
    assert_eq!(list_encounters(&separate_store).unwrap().len(), 1);
}

#[test]
fn session_snapshot_blob_is_rejected_at_the_query_boundary() {
    let session = "session-1788912002-2000";
    let sandbox = tempfile::tempdir().unwrap();
    let input = sandbox.path().join("continuous.lua");
    let store = sandbox.path().join("encounters.sqlite");
    let state = capture_state(vec![current_terminal(session, "encounter-1788912002-1")], 2);
    fs::write(&input, state_lua(&state)).unwrap();
    import_capture_set(&ImportRequest::new(&input, &store, Channel::Live)).unwrap();

    let connection = rusqlite::Connection::open(&store).unwrap();
    connection
        .execute_batch("PRAGMA ignore_check_constraints = ON;")
        .unwrap();
    connection
        .execute(
            "INSERT INTO encounter_session_snapshots (
                session_id, revision, content_sha256, channel, mode, disposition,
                started_at, finished_at, interruption_count, canonical_json
             ) VALUES ('session-1788912003-3000', 1, ?1, 'live', 'continuous',
                       'active', '1788912003', NULL, 0, ?2)",
            rusqlite::params!["0".repeat(64), vec![b'x'; 1024 * 1024 + 1]],
        )
        .unwrap();
    drop(connection);

    let diagnostic = list_encounters(&store).unwrap_err().to_string();
    assert!(diagnostic.contains("snapshot exceeds its byte limit"));
}

#[test]
fn mid_combat_partial_binds_exact_api_start_to_its_label_and_terminal_authority() {
    let mut capture = current_terminal("session-1788912002-2000", "encounter-1788912002-1");
    capture["partial_reason"] = serde_json::json!("started-mid-combat");
    capture["events"][0]["payload"]["reason"] = serde_json::json!("started-mid-combat");
    capture["events"][1]["payload"] =
        serde_json::json!({"complete": false, "reason": "started-mid-combat"});
    capture["events"][1]["source_sequence"] = serde_json::json!(5);
    capture["raw_observations"][0]["source_kind"] = serde_json::json!("api-sample");
    capture["raw_observations"][0]["source_id"] = serde_json::json!("IsUnitInCombat");
    capture["raw_observations"][0]
        .as_object_mut()
        .unwrap()
        .remove("source_code");
    capture["raw_observations"][0]["argument_count"] = serde_json::json!(1);
    capture["raw_observations"][0]["return_count"] = serde_json::json!(1);
    capture["raw_observations"][0]["values"] = serde_json::json!([
        {"position": 1, "value_type": "string", "string": "player"},
        {"position": 2, "value_type": "boolean", "boolean": true}
    ]);
    let mut exit = capture["raw_observations"][0].clone();
    exit["sequence"] = serde_json::json!(4);
    exit["monotonic_ms"] = serde_json::json!(1000);
    exit["source_kind"] = serde_json::json!("callback");
    exit["source_id"] = serde_json::json!("EVENT_PLAYER_COMBAT_STATE");
    exit["source_code"] = serde_json::json!(2);
    exit["argument_count"] = serde_json::json!(2);
    exit["return_count"] = serde_json::json!(0);
    exit["values"] = serde_json::json!([
        {"position": 1, "value_type": "number", "sign": 1, "significand": "2", "exponent": 0},
        {"position": 2, "value_type": "boolean", "boolean": false}
    ]);
    capture["raw_observations"]
        .as_array_mut()
        .unwrap()
        .insert(3, exit);
    capture["raw_observations"][4]["sequence"] = serde_json::json!(5);
    capture["raw_observations"][4]["values"][0]["string"] = serde_json::json!("started-mid-combat");
    capture["raw_observation_count"] = serde_json::json!(5);
    capture["raw_last_sequence"] = serde_json::json!(5);

    let parsed = parse_capture(capture_lua(&capture).as_bytes(), Channel::Live).unwrap();
    assert_eq!(
        assess_replay(&parsed).unwrap(),
        ReplayAssessment::Indeterminate
    );
    let mut mislabeled = capture.clone();
    mislabeled["events"][0]["payload"]["reason"] = serde_json::json!("combat-started");
    assert!(parse_capture(capture_lua(&mislabeled).as_bytes(), Channel::Live).is_err());

    let mut mislinked = capture.clone();
    mislinked["events"][0]["source_sequence"] = serde_json::json!(2);
    assert!(parse_capture(capture_lua(&mislinked).as_bytes(), Channel::Live).is_err());

    let mut fabricated = capture;
    fabricated["raw_observations"][0]["source_kind"] = serde_json::json!("callback");
    fabricated["raw_observations"][0]["source_id"] = serde_json::json!("EVENT_PLAYER_COMBAT_STATE");
    fabricated["raw_observations"][0]["source_code"] = serde_json::json!(2);
    assert!(parse_capture(capture_lua(&fabricated).as_bytes(), Channel::Live).is_err());
}

#[test]
fn lossless_v2_raw_values_canonicalize_import_and_reload_exactly() {
    let fixture = lossless_v2();
    let source = capture_lua(&fixture);
    let parsed = parse_capture(source.as_bytes(), Channel::Live).unwrap();
    assert_eq!(
        assess_replay(&parsed).unwrap(),
        ReplayAssessment::Unavailable
    );
    let parsed_json = serde_json::to_value(&parsed).unwrap();

    assert_eq!(parsed_json["schema_version"], 2);
    assert_eq!(parsed_json["raw_observations"], fixture["raw_observations"]);
    assert_eq!(
        parsed_json["raw_observations"][1]["values"],
        serde_json::json!([
            {"position": 1, "value_type": "number", "sign": 1, "significand": "3", "exponent": 0},
            {"position": 2, "value_type": "number", "sign": 1, "significand": "987654321", "exponent": 0},
            {"position": 3, "value_type": "nil"},
            {"position": 4, "value_type": "boolean", "boolean": false},
            {"position": 5, "value_type": "string", "string": "Future @Account Ω"},
            {"position": 6, "value_type": "number", "sign": 1, "significand": "3", "exponent": -1},
            {"position": 7, "value_type": "number", "sign": -1, "significand": "0", "exponent": 0},
            {"position": 8, "value_type": "number", "sign": 1, "significand": "1", "exponent": -1074},
            {"position": 9, "value_type": "nil"}
        ])
    );
    assert!(parsed_json["events"]
        .as_array()
        .unwrap()
        .iter()
        .all(|event| event["source_sequence"] != 2));

    let canonical = canonical_bytes(&parsed).unwrap();
    assert_eq!(canonical, canonical_bytes(&parsed).unwrap());

    let sandbox = tempfile::tempdir().unwrap();
    let input = sandbox.path().join("capture-v2.lua");
    let store = sandbox.path().join("encounters.sqlite");
    fs::write(&input, source).unwrap();
    let receipt = import_encounter(&ImportRequest::new(&input, &store, Channel::Live)).unwrap();
    let loaded = load_encounter(&store, &receipt.session_id, &receipt.encounter_id)
        .unwrap()
        .unwrap();
    assert_eq!(serde_json::to_value(&loaded).unwrap(), parsed_json);
    assert_eq!(canonical_bytes(&loaded).unwrap(), canonical);

    let connection = rusqlite::Connection::open(&store).unwrap();
    assert_eq!(
        connection
            .pragma_query_value::<u32, _>(None, "user_version", |row| row.get(0))
            .unwrap(),
        STORE_SCHEMA_VERSION
    );
    let stored: (i64, i64, Vec<u8>) = connection
        .query_row(
            "SELECT capture_schema_version, canonical_format_version, canonical_json
             FROM raw_encounters WHERE content_sha256 = ?1",
            [&receipt.content_sha256],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .unwrap();
    assert_eq!(stored.0, 2);
    assert_eq!(stored.1, 2);
    assert_eq!(stored.2, canonical);
}

#[test]
fn raw_loss_is_exact_and_structurally_validated() {
    let valid = partial_v2_with_raw_loss();
    let parsed = parse_capture(capture_lua(&valid).as_bytes(), Channel::Live).unwrap();
    let parsed_json = serde_json::to_value(parsed).unwrap();
    assert_eq!(parsed_json["raw_omitted_observation_count"], 2);
    assert_eq!(
        parsed_json["raw_loss"],
        serde_json::json!({
            "missing_sequence_from": 3,
            "missing_sequence_to": 4,
            "reason": "record-limit"
        })
    );

    let mut invalid = Vec::new();

    let mut missing_loss = valid.clone();
    missing_loss.as_object_mut().unwrap().remove("raw_loss");
    invalid.push(missing_loss);

    let mut wrong_count = valid.clone();
    wrong_count["raw_omitted_observation_count"] = serde_json::json!(1);
    invalid.push(wrong_count);

    let mut wrong_range = valid.clone();
    wrong_range["raw_loss"]["missing_sequence_from"] = serde_json::json!(4);
    invalid.push(wrong_range);

    let mut undeclared_gap = valid.clone();
    undeclared_gap["raw_loss"] = serde_json::Value::Null;
    undeclared_gap["raw_omitted_observation_count"] = serde_json::json!(0);
    invalid.push(undeclared_gap);

    let mut invalid_reason = valid.clone();
    invalid_reason["raw_loss"]["reason"] = serde_json::json!("silently-dropped");
    invalid.push(invalid_reason);

    let mut complete_with_loss = valid.clone();
    complete_with_loss["status"] = serde_json::json!("complete");
    complete_with_loss
        .as_object_mut()
        .unwrap()
        .remove("partial_reason");
    complete_with_loss["events"][1]["payload"] = serde_json::json!({
        "complete": true,
        "reason": "combat-ended"
    });
    invalid.push(complete_with_loss);

    let mut unsupported_partial_reason = valid.clone();
    unsupported_partial_reason["partial_reason"] = serde_json::json!("capture-overflow");
    unsupported_partial_reason["events"][1]["payload"]["reason"] =
        serde_json::json!("capture-overflow");
    invalid.push(unsupported_partial_reason);

    let mut unknown_source = lossless_v2();
    unknown_source["raw_observations"][1]["source_id"] = serde_json::json!("EVENT_FUTURE_UNKNOWN");
    invalid.push(unknown_source);

    let mut mismatched_callback_code = lossless_v2();
    mismatched_callback_code["raw_observations"][0]["source_code"] = serde_json::json!(3);
    invalid.push(mismatched_callback_code);

    let mut wrong_projection_time = lossless_v2();
    wrong_projection_time["raw_observations"][3]["monotonic_ms"] = serde_json::json!(999);
    invalid.push(wrong_projection_time);

    let mut wrong_projection_source = lossless_v2();
    wrong_projection_source["raw_observations"][2]["monotonic_ms"] = serde_json::json!(1000);
    wrong_projection_source["events"][1]["source_sequence"] = serde_json::json!(3);
    invalid.push(wrong_projection_source);

    let mut overflowing_finite_descriptor = lossless_v2();
    overflowing_finite_descriptor["raw_observations"][1]["values"][0] = serde_json::json!({
        "position": 1,
        "value_type": "number",
        "sign": 1,
        "significand": "2",
        "exponent": 1023
    });
    invalid.push(overflowing_finite_descriptor);

    let mut noncontiguous_values = lossless_v2();
    noncontiguous_values["raw_observations"][1]["values"][8]["position"] = serde_json::json!(10);
    invalid.push(noncontiguous_values);

    let mut mismatched_value_count = lossless_v2();
    mismatched_value_count["raw_observations"][1]["argument_count"] = serde_json::json!(7);
    invalid.push(mismatched_value_count);

    for capture in invalid {
        assert!(parse_capture(capture_lua(&capture).as_bytes(), Channel::Live).is_err());
    }
}

#[test]
fn populated_v1_store_migrates_without_rewriting_legacy_evidence() {
    let sandbox = tempfile::tempdir().unwrap();
    let v1_input = sandbox.path().join("capture-v1.lua");
    let v2_input = sandbox.path().join("capture-v2.lua");
    let store = sandbox.path().join("encounters.sqlite");
    fs::write(&v1_input, COMPLETE).unwrap();
    let v1_receipt =
        import_encounter(&ImportRequest::new(&v1_input, &store, Channel::Live)).unwrap();

    let connection = rusqlite::Connection::open(&store).unwrap();
    connection
        .execute_batch(
            "BEGIN IMMEDIATE;
             DROP TRIGGER raw_encounters_no_update;
             DROP TRIGGER encounter_session_snapshots_no_update;
             DROP TABLE encounter_session_snapshots;
             ALTER TABLE raw_encounters RENAME TO raw_encounters_v2;
             DROP TABLE encounter_store_meta;
             CREATE TABLE encounter_store_meta (
                 singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
                 schema_version INTEGER NOT NULL CHECK (schema_version = 1),
                 canonical_format_version INTEGER NOT NULL CHECK (canonical_format_version = 1)
             );
             INSERT INTO encounter_store_meta(singleton, schema_version, canonical_format_version)
             VALUES (1, 1, 1);
             CREATE TABLE raw_encounters (
                 content_sha256 TEXT PRIMARY KEY CHECK (length(content_sha256) = 64),
                 source_sha256 TEXT NOT NULL CHECK (length(source_sha256) = 64),
                 session_id TEXT NOT NULL,
                 encounter_id TEXT NOT NULL,
                 channel TEXT NOT NULL CHECK (channel IN ('live', 'pts')),
                 capture_schema_version INTEGER NOT NULL CHECK (capture_schema_version = 1),
                 addon_version INTEGER NOT NULL CHECK (addon_version = 1),
                 status TEXT NOT NULL CHECK (status IN ('complete', 'partial')),
                 started_at TEXT NOT NULL,
                 finished_at TEXT NOT NULL,
                 first_sequence INTEGER NOT NULL,
                 last_sequence INTEGER NOT NULL,
                 stored_event_count INTEGER NOT NULL,
                 omitted_event_count INTEGER NOT NULL,
                 canonical_json BLOB NOT NULL,
                 UNIQUE (session_id, encounter_id)
             );
             INSERT INTO raw_encounters (
                 content_sha256, source_sha256, session_id, encounter_id, channel,
                 capture_schema_version, addon_version, status, started_at, finished_at,
                 first_sequence, last_sequence, stored_event_count, omitted_event_count,
                 canonical_json)
             SELECT content_sha256, source_sha256, session_id, encounter_id, channel,
                 capture_schema_version, addon_version, status, started_at, finished_at,
                 first_sequence, last_sequence, stored_event_count, omitted_event_count,
                 canonical_json
             FROM raw_encounters_v2;
             DROP TABLE raw_encounters_v2;
             CREATE TRIGGER raw_encounters_no_update
             BEFORE UPDATE ON raw_encounters
             BEGIN
                 SELECT RAISE(ABORT, 'raw encounter records are immutable');
             END;
             PRAGMA user_version = 1;
             COMMIT;",
        )
        .unwrap();
    assert_eq!(
        connection
            .pragma_query_value::<u32, _>(None, "user_version", |row| row.get(0))
            .unwrap(),
        1
    );
    let legacy_before: (String, Vec<u8>) = connection
        .query_row(
            "SELECT content_sha256, canonical_json FROM raw_encounters
             WHERE session_id = ?1 AND encounter_id = ?2",
            [&v1_receipt.session_id, &v1_receipt.encounter_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .unwrap();
    drop(connection);

    fs::write(&v2_input, lossless_v2_lua()).unwrap();
    let v2_receipt =
        import_encounter(&ImportRequest::new(&v2_input, &store, Channel::Live)).unwrap();

    let connection = rusqlite::Connection::open(&store).unwrap();
    assert_eq!(
        connection
            .pragma_query_value::<u32, _>(None, "user_version", |row| row.get(0))
            .unwrap(),
        STORE_SCHEMA_VERSION
    );
    let legacy_after: (String, Vec<u8>, i64) = connection
        .query_row(
            "SELECT content_sha256, canonical_json, canonical_format_version
             FROM raw_encounters WHERE session_id = ?1 AND encounter_id = ?2",
            [&v1_receipt.session_id, &v1_receipt.encounter_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .unwrap();
    assert_eq!(legacy_after.0, legacy_before.0);
    assert_eq!(legacy_after.1, legacy_before.1);
    assert_eq!(legacy_after.2, 1);
    let v2_format: i64 = connection
        .query_row(
            "SELECT canonical_format_version FROM raw_encounters
             WHERE session_id = ?1 AND encounter_id = ?2",
            [&v2_receipt.session_id, &v2_receipt.encounter_id],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(v2_format, 2);
    assert!(connection
        .execute(
            "UPDATE raw_encounters SET canonical_json = canonical_json
             WHERE session_id = ?1 AND encounter_id = ?2",
            [&v1_receipt.session_id, &v1_receipt.encounter_id],
        )
        .is_err());
    drop(connection);

    assert_eq!(list_encounters(&store).unwrap().len(), 2);
    let legacy = load_encounter(&store, &v1_receipt.session_id, &v1_receipt.encounter_id)
        .unwrap()
        .unwrap();
    assert_eq!(canonical_bytes(&legacy).unwrap(), legacy_before.1);
    let current = load_encounter(&store, &v2_receipt.session_id, &v2_receipt.encounter_id)
        .unwrap()
        .unwrap();
    assert_eq!(serde_json::to_value(current).unwrap()["schema_version"], 2);
    assert_eq!(
        import_encounter(&ImportRequest::new(&v1_input, &store, Channel::Live))
            .unwrap()
            .outcome,
        ImportOutcome::AlreadyPresent
    );
    assert_eq!(
        import_encounter(&ImportRequest::new(&v2_input, &store, Channel::Live))
            .unwrap()
            .outcome,
        ImportOutcome::AlreadyPresent
    );

    let backup = sandbox.path().join("mixed-backup.sqlite");
    backup_store(&store, &backup).unwrap();
    assert_eq!(list_encounters(&backup).unwrap().len(), 2);
}

#[test]
fn populated_v2_store_migrates_to_v4_without_rewriting_canonical_evidence() {
    let sandbox = tempfile::tempdir().unwrap();
    let input = sandbox.path().join("capture-v2.lua");
    let store = sandbox.path().join("encounters-v2.sqlite");
    let source = lossless_v2_lua();
    let capture = parse_capture(source.as_bytes(), Channel::Live).unwrap();
    let canonical = canonical_bytes(&capture).unwrap();
    let content_sha256 = format!("{:x}", Sha256::digest(&canonical));
    let connection = rusqlite::Connection::open(&store).unwrap();
    connection
        .execute_batch(
            r#"
            CREATE TABLE encounter_store_meta (
                singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
                schema_version INTEGER NOT NULL CHECK (schema_version = 2),
                canonical_format_version INTEGER NOT NULL CHECK (canonical_format_version = 2)
            );
            INSERT INTO encounter_store_meta VALUES (1, 2, 2);
            CREATE TABLE raw_encounters (
                content_sha256 TEXT PRIMARY KEY CHECK (length(content_sha256) = 64),
                source_sha256 TEXT NOT NULL CHECK (length(source_sha256) = 64),
                session_id TEXT NOT NULL,
                encounter_id TEXT NOT NULL,
                channel TEXT NOT NULL CHECK (channel IN ('live', 'pts')),
                capture_schema_version INTEGER NOT NULL CHECK (capture_schema_version IN (1, 2)),
                addon_version INTEGER NOT NULL CHECK (addon_version IN (1, 2)),
                canonical_format_version INTEGER NOT NULL CHECK (canonical_format_version IN (1, 2)),
                status TEXT NOT NULL CHECK (status IN ('complete', 'partial')),
                started_at TEXT NOT NULL,
                finished_at TEXT NOT NULL,
                first_sequence INTEGER NOT NULL,
                last_sequence INTEGER NOT NULL,
                stored_event_count INTEGER NOT NULL,
                omitted_event_count INTEGER NOT NULL,
                canonical_json BLOB NOT NULL,
                UNIQUE (session_id, encounter_id),
                CHECK ((capture_schema_version = 1 AND canonical_format_version = 1) OR
                       (capture_schema_version = 2 AND canonical_format_version = 2))
            );
            CREATE TRIGGER raw_encounters_no_update
            BEFORE UPDATE ON raw_encounters
            BEGIN
                SELECT RAISE(ABORT, 'raw encounter records are immutable');
            END;
            PRAGMA user_version = 2;
            "#,
        )
        .unwrap();
    connection
        .execute(
            "INSERT INTO raw_encounters VALUES (?1, ?2, ?3, ?4, 'live', 2, 2, 2,
             'complete', ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            rusqlite::params![
                content_sha256,
                "0".repeat(64),
                capture.session_id,
                capture.encounter_id,
                capture.started_at,
                capture.finished_at,
                i64::try_from(capture.first_sequence).unwrap(),
                i64::try_from(capture.last_sequence).unwrap(),
                i64::try_from(capture.stored_event_count).unwrap(),
                i64::try_from(capture.omitted_event_count).unwrap(),
                canonical,
            ],
        )
        .unwrap();
    drop(connection);

    fs::write(&input, source).unwrap();
    let receipt = import_encounter(&ImportRequest::new(&input, &store, Channel::Live)).unwrap();
    assert_eq!(receipt.outcome, ImportOutcome::AlreadyPresent);
    let connection = rusqlite::Connection::open(&store).unwrap();
    assert_eq!(
        connection
            .pragma_query_value::<u32, _>(None, "user_version", |row| row.get(0))
            .unwrap(),
        STORE_SCHEMA_VERSION
    );
    let preserved: Vec<u8> = connection
        .query_row(
            "SELECT canonical_json FROM raw_encounters WHERE content_sha256 = ?1",
            [&receipt.content_sha256],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(preserved, canonical_bytes(&capture).unwrap());
}

#[test]
fn populated_v3_store_migrates_to_v4_without_rewriting_canonical_evidence() {
    let sandbox = tempfile::tempdir().unwrap();
    let input = sandbox.path().join("capture-v3.lua");
    let store = sandbox.path().join("encounters.sqlite");
    let capture_value = current_terminal("session-1788912002-2000", "encounter-1788912002-1");
    let source = capture_lua(&capture_value);
    fs::write(&input, &source).unwrap();
    let capture = parse_capture(source.as_bytes(), Channel::Live).unwrap();
    let canonical = canonical_bytes(&capture).unwrap();
    let content_sha256 = format!("{:x}", Sha256::digest(&canonical));
    let second_value = current_terminal("session-1788912002-2000", "encounter-1788912002-2");
    let second = parse_capture(capture_lua(&second_value).as_bytes(), Channel::Live).unwrap();
    let second_canonical = canonical_bytes(&second).unwrap();
    let second_content_sha256 = format!("{:x}", Sha256::digest(&second_canonical));
    let original_source_sha256 = "ab".repeat(32);

    let connection = rusqlite::Connection::open(&store).unwrap();
    connection
        .execute_batch(
            "CREATE TABLE encounter_store_meta (
                 singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
                 schema_version INTEGER NOT NULL CHECK (schema_version = 3),
                 canonical_format_version INTEGER NOT NULL CHECK (canonical_format_version = 2)
             );
             INSERT INTO encounter_store_meta(singleton, schema_version, canonical_format_version)
             VALUES (1, 3, 2);
             CREATE TABLE raw_encounters (
                 content_sha256 TEXT PRIMARY KEY CHECK (length(content_sha256) = 64),
                 source_sha256 TEXT NOT NULL CHECK (length(source_sha256) = 64),
                 session_id TEXT NOT NULL,
                 encounter_id TEXT NOT NULL,
                 channel TEXT NOT NULL CHECK (channel IN ('live', 'pts')),
                 capture_schema_version INTEGER NOT NULL CHECK (capture_schema_version IN (1, 2)),
                 addon_version INTEGER NOT NULL CHECK (addon_version IN (1, 2, 3)),
                 canonical_format_version INTEGER NOT NULL CHECK (canonical_format_version IN (1, 2)),
                 status TEXT NOT NULL CHECK (status IN ('complete', 'partial')),
                 started_at TEXT NOT NULL,
                 finished_at TEXT NOT NULL,
                 first_sequence INTEGER NOT NULL,
                 last_sequence INTEGER NOT NULL,
                 stored_event_count INTEGER NOT NULL,
                 omitted_event_count INTEGER NOT NULL,
                 canonical_json BLOB NOT NULL,
                 UNIQUE (session_id, encounter_id),
                 CHECK ((capture_schema_version = 1 AND canonical_format_version = 1) OR
                        (capture_schema_version = 2 AND canonical_format_version = 2))
             );
             CREATE TRIGGER raw_encounters_no_update
             BEFORE UPDATE ON raw_encounters
             BEGIN
                 SELECT RAISE(ABORT, 'raw encounter records are immutable');
             END;
             PRAGMA user_version = 3;",
        )
        .unwrap();
    connection
        .execute(
            "INSERT INTO raw_encounters (
                content_sha256, source_sha256, session_id, encounter_id, channel,
                capture_schema_version, addon_version, canonical_format_version,
                status, started_at, finished_at, first_sequence, last_sequence,
                stored_event_count, omitted_event_count, canonical_json
             ) VALUES (?1, ?2, ?3, ?4, 'live', 2, 3, 2, 'partial', ?5, ?6, ?7,
                       ?8, ?9, ?10, ?11)",
            rusqlite::params![
                content_sha256,
                original_source_sha256,
                capture.session_id,
                capture.encounter_id,
                capture.started_at,
                capture.finished_at,
                capture.first_sequence as i64,
                capture.last_sequence as i64,
                capture.stored_event_count as i64,
                capture.omitted_event_count as i64,
                canonical,
            ],
        )
        .unwrap();
    connection
        .execute(
            "INSERT INTO raw_encounters (
                content_sha256, source_sha256, session_id, encounter_id, channel,
                capture_schema_version, addon_version, canonical_format_version,
                status, started_at, finished_at, first_sequence, last_sequence,
                stored_event_count, omitted_event_count, canonical_json
             ) VALUES (?1, ?2, ?3, ?4, 'live', 2, 3, 2, 'partial', ?5, ?6, ?7,
                       ?8, ?9, ?10, ?11)",
            rusqlite::params![
                second_content_sha256,
                original_source_sha256,
                second.session_id,
                second.encounter_id,
                second.started_at,
                second.finished_at,
                second.first_sequence as i64,
                second.last_sequence as i64,
                second.stored_event_count as i64,
                second.omitted_event_count as i64,
                second_canonical,
            ],
        )
        .unwrap();
    let before: (String, String, Vec<u8>) = connection
        .query_row(
            "SELECT source_sha256, content_sha256, canonical_json FROM raw_encounters
             WHERE content_sha256 = ?1",
            [&content_sha256],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .unwrap();
    drop(connection);

    let receipt = import_encounter(&ImportRequest::new(&input, &store, Channel::Live)).unwrap();
    assert_eq!(receipt.outcome, ImportOutcome::AlreadyPresent);
    let connection = rusqlite::Connection::open(&store).unwrap();
    assert_eq!(
        connection
            .pragma_query_value::<u32, _>(None, "user_version", |row| row.get(0))
            .unwrap(),
        STORE_SCHEMA_VERSION
    );
    let after: (String, String, Vec<u8>, String, i64) = connection
        .query_row(
            "SELECT source_sha256, content_sha256, canonical_json, capture_mode,
                    encounter_ordinal FROM raw_encounters WHERE content_sha256 = ?1",
            [&content_sha256],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                ))
            },
        )
        .unwrap();
    assert_eq!(
        (&after.0, &after.1, &after.2),
        (&before.0, &before.1, &before.2)
    );
    assert_eq!((after.3.as_str(), after.4), ("single", 1));
    let summaries = list_encounters(&store).unwrap();
    assert_eq!(summaries.len(), 2);
    assert_eq!(summaries[0].encounter_ordinal, 1);
    assert_eq!(summaries[1].encounter_ordinal, 2);
}

#[test]
fn capture_version_dispatch_accepts_only_v1_and_v2() {
    assert!(parse_capture(COMPLETE.as_bytes(), Channel::Live).is_ok());
    assert!(parse_capture(lossless_v2_lua().as_bytes(), Channel::Live).is_ok());

    for version in [0, 3, u32::MAX] {
        let mut capture = lossless_v2();
        capture["schema_version"] = serde_json::json!(version);
        assert!(parse_capture(capture_lua(&capture).as_bytes(), Channel::Live).is_err());
    }

    let mut v1_shape_claiming_v2 = lossless_v2();
    for field in [
        "raw_first_sequence",
        "raw_last_sequence",
        "raw_observation_count",
        "raw_omitted_observation_count",
        "raw_observations",
    ] {
        v1_shape_claiming_v2.as_object_mut().unwrap().remove(field);
    }
    assert!(parse_capture(capture_lua(&v1_shape_claiming_v2).as_bytes(), Channel::Live).is_err());

    let v1_with_v2_reason = partial_lua().replace("clock-reset", "record-limit");
    assert!(parse_capture(v1_with_v2_reason.as_bytes(), Channel::Live).is_err());
}

#[test]
fn schema_errors_never_echo_raw_value_canaries() {
    let mut capture = lossless_v2();
    capture["raw_observations"][1]["values"][4]["value_type"] =
        serde_json::json!("private-canary-never-log");
    let error = parse_capture(capture_lua(&capture).as_bytes(), Channel::Live).unwrap_err();
    let diagnostic = error.to_string();
    assert!(diagnostic.contains("invalid encounter schema"));
    assert!(!diagnostic.contains("private-canary-never-log"));
    assert!(!diagnostic.contains("Future @Account"));
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
        "EsoWeaveDataSaved = { [\"schema_version\"] = 1, [\"addon_version\"] = 1, [\"encounter\"] = function() return {} end }".to_string(),
        "Other = {}".to_string(),
        format!("{COMPLETE}; os.execute(\"x\")"),
        COMPLETE.replace("[\"channel\"] = \"live\"", "[\"channel\"] = \"pts\""),
        COMPLETE.replace("[\"last_sequence\"] = 2", "[\"last_sequence\"] = 3"),
        COMPLETE.replacen("[\"schema_version\"] = 1", "[\"schema_version\"] = 2", 1),
        COMPLETE.replacen("[\"addon_version\"] = 1", "[\"addon_version\"] = 2", 1),
        COMPLETE.replace(
            "\n    [\"schema_version\"] = 1",
            "\n    [\"schema_version\"] = 2",
        ),
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

    fs::write(
        &input,
        "EsoWeaveDataSaved = { [\"schema_version\"] = 1, [\"addon_version\"] = 1, [\"encounter\"] = broken }",
    )
    .unwrap();
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

    let indexed_drift = sandbox.path().join("indexed-drift.sqlite");
    fs::copy(&backup, &indexed_drift).unwrap();
    let connection = rusqlite::Connection::open(&indexed_drift).unwrap();
    connection
        .execute_batch(
            "DROP TRIGGER raw_encounters_no_update;
             UPDATE raw_encounters SET started_at = '1';
             CREATE TRIGGER raw_encounters_no_update
             BEFORE UPDATE ON raw_encounters
             BEGIN
                 SELECT RAISE(ABORT, 'raw encounter records are immutable');
             END;",
        )
        .unwrap();
    drop(connection);
    assert!(list_encounters(&indexed_drift).is_err());
    assert!(delete_all(&indexed_drift).is_err());

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
            source_sequence: None,
            projection_ordinal: None,
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
fn production_raw_ceiling_parses_imports_and_reloads() {
    let mut capture = parse_capture(lossless_v2_lua().as_bytes(), Channel::Live).unwrap();
    let mut terminal = capture.raw_observations.pop().unwrap();
    capture.raw_observations.truncate(1);
    capture.raw_observations.reserve(MAX_EVENTS - 2);
    for sequence in 2..MAX_EVENTS as u64 {
        capture.raw_observations.push(RawObservation {
            session_id: capture.session_id.clone(),
            encounter_id: capture.encounter_id.clone(),
            sequence,
            monotonic_ms: 0,
            api_version: capture.source.api_version,
            source_kind: RawSourceKind::Callback,
            source_id: "EVENT_PLAYER_DEAD".into(),
            source_code: Some(8),
            source_version: 1,
            argument_count: 1,
            return_count: 0,
            values: vec![RawValue {
                position: 1,
                value_type: RawValueType::Number,
                boolean: None,
                string: None,
                sign: Some(1),
                significand: Some("1".into()),
                exponent: Some(3),
            }],
        });
    }
    terminal.sequence = MAX_EVENTS as u64;
    capture.raw_observations.push(terminal);
    capture.raw_last_sequence = Some(MAX_EVENTS as u64);
    capture.raw_observation_count = Some(MAX_EVENTS);
    capture.estimated_bytes = MAX_ESTIMATED_BYTES;
    capture.events[1].source_sequence = Some(MAX_EVENTS as u64);

    let value = serde_json::to_value(&capture).unwrap();
    let source = capture_lua(&value);
    let parsed = parse_capture(source.as_bytes(), Channel::Live).unwrap();
    assert_eq!(parsed.raw_observations.len(), MAX_EVENTS);
    let canonical = canonical_bytes(&parsed).unwrap();
    assert!(canonical.len() > 20_000_000);

    let sandbox = tempfile::tempdir().unwrap();
    let input = sandbox.path().join("capture-v2-ceiling.lua");
    let store = sandbox.path().join("encounters.sqlite");
    fs::write(&input, source).unwrap();
    let receipt = import_encounter(&ImportRequest::new(&input, &store, Channel::Live)).unwrap();
    let loaded = load_encounter(&store, &receipt.session_id, &receipt.encounter_id)
        .unwrap()
        .unwrap();
    assert_eq!(loaded.raw_observations.len(), MAX_EVENTS);
    assert_eq!(canonical_bytes(&loaded).unwrap(), canonical);
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
