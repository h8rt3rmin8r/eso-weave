use eso_weave::catalog::Channel;
use eso_weave::encounter::{parse_native_log, NativeEncounter};

const REPRESENTATIVE: &str =
    include_str!("../specs/127-native-completed-import/fixtures/representative-v15.log");

fn header() -> &'static str {
    "0,BEGIN_LOG,1700000000000,15,\"FixtureRealm\",\"zz\",\"fixture.game\"\n"
}

fn tiny() -> String {
    format!(
        "{}0,UNIT_ADDED,1,PLAYER,T,0,1,F,1,1,\"NativeCanary, Ω\",\"@ActorCanary\",1,1,1,0,PLAYER_ALLY,F\n\
         10,BEGIN_COMBAT\n\
         20,COMBAT_EVENT,DAMAGE,PHYSICAL,STAMINA,300,0,10,100,1,100/100,100/100,100/100,0/100,0/0,0,0.25,0.25,0,*\n\
         30,END_COMBAT\n40,END_LOG\n",
        header()
    )
}

fn first(source: &str) -> NativeEncounter {
    parse_native_log(source.as_bytes(), Channel::Live)
        .unwrap()
        .encounters
        .remove(0)
}

#[test]
fn native_representative_fixture_groups_two_completed_combats_with_native_provenance() {
    let parsed = parse_native_log(REPRESENTATIVE.as_bytes(), Channel::Live).unwrap();
    assert_eq!(parsed.encounters.len(), 2);
    assert_eq!(parsed.report.completed_encounter_count, 2);
    assert_eq!(parsed.report.run_count, 1);
    assert_eq!(parsed.report.unknown_record_count, 0);
    for encounter in &parsed.encounters {
        assert_eq!(encounter.run.header.log_version, 15);
        assert!(encounter.run.session_id.starts_with("native-session-"));
        assert!(encounter.encounter_id.starts_with("native-encounter-"));
        let run = serde_json::to_value(&encounter.run).unwrap();
        assert!(run.get("api_version").is_none());
        assert!(run.get("addon_version").is_none());
    }
    assert!(parsed.encounters[0].first_ordinal < parsed.encounters[1].first_ordinal);
}

#[test]
fn native_identical_content_and_later_run_preserve_prior_ids() {
    let source = tiny();
    let baseline = first(&source);
    let appended = format!(
        "{source}{}",
        tiny().replace("1700000000000", "1700000100000")
    );
    let parsed = parse_native_log(appended.as_bytes(), Channel::Live).unwrap();
    assert_eq!(parsed.encounters.len(), 2);
    assert_eq!(parsed.encounters[0], baseline);
    assert_ne!(
        parsed.encounters[0].run.session_id,
        parsed.encounters[1].run.session_id
    );
    assert_eq!(first(&source), baseline);
    assert_ne!(
        parse_native_log(source.as_bytes(), Channel::Pts)
            .unwrap()
            .encounters[0]
            .run
            .session_id,
        baseline.run.session_id
    );
}

#[test]
fn native_quoted_fields_remain_exact_and_debug_omits_payloads() {
    let capture = first(&tiny());
    assert!(capture
        .context
        .iter()
        .any(|record| record.line.contains("\"NativeCanary, Ω\"")));
    assert!(capture
        .records
        .iter()
        .any(|record| record.line.contains("300,0,10,100")));
    let debug = format!("{capture:?}");
    assert!(!debug.contains("NativeCanary"));
    assert!(!debug.contains("ActorCanary"));
}

#[test]
fn native_unknown_records_are_retained_and_counted() {
    let source = tiny().replace(
        "30,END_COMBAT",
        "25,FUTURE_NATIVE_TYPE,\"UnknownCanary\"\n30,END_COMBAT",
    );
    let parsed = parse_native_log(source.as_bytes(), Channel::Live).unwrap();
    assert_eq!(parsed.report.unknown_record_count, 1);
    assert_eq!(parsed.encounters[0].unknown_record_count, 1);
    assert!(parsed.encounters[0]
        .records
        .iter()
        .any(|r| r.line.contains("UnknownCanary")));
}

#[test]
fn native_unterminated_tail_and_unfinished_combat_are_explicit() {
    let source = format!("{}10,BEGIN_COMBAT\n20,END_COMBAT", header());
    let parsed = parse_native_log(source.as_bytes(), Channel::Live).unwrap();
    assert!(parsed.encounters.is_empty());
    assert_eq!(parsed.report.incomplete_combat_count, 1);
    assert_eq!(parsed.report.incomplete_run_count, 1);
    assert_eq!(parsed.report.unterminated_line_count, 1);
}

#[test]
fn native_empty_completed_run_has_no_fabricated_encounter() {
    let source = format!("{}40,END_LOG\n", header());
    let parsed = parse_native_log(source.as_bytes(), Channel::Live).unwrap();
    assert!(parsed.encounters.is_empty());
    assert_eq!(parsed.report.record_count, 2);
}

#[test]
fn native_malformed_and_unsupported_records_reject_without_raw_diagnostics() {
    for source in [
        tiny().replace(",15,", ",99,"),
        tiny().replace("30,END_COMBAT", "21,BEGIN_COMBAT\n30,END_COMBAT"),
        tiny().replace("10,BEGIN_COMBAT\n", ""),
        tiny().replace("30,END_COMBAT", "9,END_COMBAT"),
        tiny().replace("300,0,10,100", "\"ErrorCanary\",0,10,100"),
        tiny().replace("30,END_COMBAT", "30,END_COMBAT,\"ErrorCanary\""),
        format!("{}10,PLAYER_INFO,1,[1,[2],[],[],[]\n", header()),
    ] {
        let error = parse_native_log(source.as_bytes(), Channel::Live).unwrap_err();
        assert!(!format!("{error}").contains("ErrorCanary"));
        assert!(!format!("{error:?}").contains("ErrorCanary"));
    }
}

#[test]
fn native_atomic_import_copy_reimport_mixed_history_delete_and_backup() {
    use eso_weave::encounter::{
        backup_store, delete_all, delete_encounter, import_encounter, import_native_log,
        list_encounters, ImportRequest,
    };
    let sandbox = tempfile::tempdir().unwrap();
    let native = sandbox.path().join("native.log");
    let copied = sandbox.path().join("copied.log");
    let addon = sandbox.path().join("addon.lua");
    let store = sandbox.path().join("history.sqlite");
    std::fs::write(&native, tiny()).unwrap();
    std::fs::write(&copied, tiny()).unwrap();
    std::fs::write(
        &addon,
        include_str!("fixtures/encounter/valid-complete.lua"),
    )
    .unwrap();
    let addon_receipt =
        import_encounter(&ImportRequest::new(&addon, &store, Channel::Live)).unwrap();
    let original = std::fs::read(&native).unwrap();
    let receipt = import_native_log(&ImportRequest::new(&native, &store, Channel::Live)).unwrap();
    assert_eq!(receipt.imported_count, 1);
    let repeated = import_native_log(&ImportRequest::new(&copied, &store, Channel::Live)).unwrap();
    assert_eq!(repeated.imported_count, 0);
    assert_eq!(repeated.already_present_count, 1);
    assert_eq!(
        receipt.receipts[0].encounter_id,
        repeated.receipts[0].encounter_id
    );
    assert_eq!(list_encounters(&store).unwrap().len(), 2);
    let backup = sandbox.path().join("backup.sqlite");
    backup_store(&store, &backup).unwrap();
    assert_eq!(list_encounters(&backup).unwrap().len(), 2);
    let identity = &receipt.receipts[0];
    assert_eq!(
        delete_encounter(&store, &identity.session_id, &identity.encounter_id)
            .unwrap()
            .deleted_records,
        1
    );
    assert_eq!(
        list_encounters(&store).unwrap()[0].encounter_id,
        addon_receipt.encounter_id
    );
    import_native_log(&ImportRequest::new(&native, &store, Channel::Live)).unwrap();
    assert_eq!(
        list_encounters(&store).unwrap()[1].encounter_id,
        identity.encounter_id
    );
    assert_eq!(delete_all(&store).unwrap().deleted_records, 2);
    assert!(list_encounters(&store).unwrap().is_empty());
    assert_eq!(std::fs::read(&native).unwrap(), original);
}

#[test]
fn native_late_invalid_source_creates_no_store_and_changes_no_source() {
    use eso_weave::encounter::{import_native_log, ImportRequest};
    let sandbox = tempfile::tempdir().unwrap();
    let input = sandbox.path().join("invalid.log");
    let store = sandbox.path().join("absent.sqlite");
    let source = format!(
        "{}{}",
        tiny(),
        tiny().replace("300,0,10,100", "\"LateCanary\",0,10,100")
    );
    std::fs::write(&input, &source).unwrap();
    assert!(import_native_log(&ImportRequest::new(&input, &store, Channel::Live)).is_err());
    assert!(!store.exists());
    assert_eq!(std::fs::read_to_string(input).unwrap(), source);
}

#[test]
fn native_identity_collision_preserves_existing_store_bytes() {
    use eso_weave::encounter::{import_native_log, ImportRequest};
    let sandbox = tempfile::tempdir().unwrap();
    let input = sandbox.path().join("recording.log");
    let store = sandbox.path().join("history.sqlite");
    std::fs::write(&input, tiny()).unwrap();
    let request = ImportRequest::new(&input, &store, Channel::Live);
    import_native_log(&request).unwrap();
    let before = std::fs::read(&store).unwrap();
    std::fs::write(&input, tiny().replace("300,0,10,100", "301,0,10,100")).unwrap();
    assert!(import_native_log(&request).is_err());
    assert!(
        std::fs::read(&store).unwrap() == before,
        "rejected native collision changed database bytes"
    );
}

#[test]
fn native_rejected_write_rolls_back_schema5_migration_when_order_is_exhausted() {
    use eso_weave::encounter::{import_encounter, import_native_log, ImportRequest};
    let sandbox = tempfile::tempdir().unwrap();
    let addon = sandbox.path().join("addon.lua");
    let native = sandbox.path().join("native.log");
    let store = sandbox.path().join("legacy.sqlite");
    std::fs::write(
        &addon,
        include_str!("fixtures/encounter/valid-complete.lua"),
    )
    .unwrap();
    std::fs::write(&native, tiny()).unwrap();
    import_encounter(&ImportRequest::new(&addon, &store, Channel::Live)).unwrap();
    let connection = rusqlite::Connection::open(&store).unwrap();
    connection
        .execute_batch(
            "BEGIN IMMEDIATE;
         DROP TRIGGER IF EXISTS native_encounters_no_update;
         DROP TRIGGER IF EXISTS native_runs_no_update;
         DROP TABLE IF EXISTS native_encounters;
         DROP TABLE IF EXISTS native_runs;
         DROP TABLE encounter_store_meta;
         CREATE TABLE encounter_store_meta (
             singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
             schema_version INTEGER NOT NULL CHECK (schema_version = 5),
             canonical_format_version INTEGER NOT NULL CHECK (canonical_format_version = 2)
         );
         INSERT INTO encounter_store_meta VALUES (1,5,2);
         DROP TRIGGER encounter_session_order_no_update;
         UPDATE encounter_session_order SET import_ordinal = 9223372036854775807;
         CREATE TRIGGER encounter_session_order_no_update
         BEFORE UPDATE ON encounter_session_order
         BEGIN
             SELECT RAISE(ABORT, 'encounter session order is immutable');
         END;
         PRAGMA user_version=5;
         COMMIT;",
        )
        .unwrap();
    drop(connection);
    let before = std::fs::read(&store).unwrap();
    assert!(import_native_log(&ImportRequest::new(&native, &store, Channel::Live)).is_err());
    assert!(
        std::fs::read(&store).unwrap() == before,
        "rejected schema5 native import changed database bytes"
    );
    let connection = rusqlite::Connection::open(&store).unwrap();
    assert_eq!(
        connection
            .pragma_query_value::<u32, _>(None, "user_version", |row| row.get(0))
            .unwrap(),
        5
    );
}

#[test]
fn native_service_inspection_is_read_only_and_import_keeps_catalog_independent() {
    use eso_weave::encounter::{EncounterHistoryService, EncounterSourceKind};
    let sandbox = tempfile::tempdir().unwrap();
    let input = sandbox.path().join("native.log");
    std::fs::write(&input, tiny()).unwrap();
    let store = sandbox.path().join("history.sqlite");
    let service =
        EncounterHistoryService::from_paths(&store, sandbox.path().join("missing-catalog.sqlite"));
    let inspected = service.inspect_native_source(&input, Channel::Pts).unwrap();
    assert_eq!(inspected.channel, Channel::Pts);
    assert_eq!(inspected.report.completed_encounter_count, 1);
    assert!(!store.exists());
    let report = service.import_native_current(&input, Channel::Pts).unwrap();
    assert_eq!(report.imported_count, 1);
    let snapshot = service.snapshot().unwrap();
    assert_eq!(snapshot[0].source_kind, EncounterSourceKind::NativeLog);
    assert_eq!(snapshot[0].capture_mode, None);
    assert_eq!(snapshot[0].channel, Channel::Pts);
}

#[test]
fn native_service_errors_never_reproduce_payload_values() {
    use eso_weave::encounter::{EncounterHistoryService, HistoryDiagnosticKind};
    let sandbox = tempfile::tempdir().unwrap();
    let input = sandbox.path().join("native.log");
    let store = sandbox.path().join("history.sqlite");
    let service =
        EncounterHistoryService::from_paths(&store, sandbox.path().join("catalog.sqlite"));
    std::fs::write(
        &input,
        tiny().replace("300,0,10,100", "\"PrivateInputCanary\",0,10,100"),
    )
    .unwrap();
    let error = service
        .inspect_native_source(&input, Channel::Live)
        .unwrap_err();
    assert_eq!(error.kind, HistoryDiagnosticKind::SourceInvalid);
    assert!(!error.message.contains("PrivateInputCanary"));
    assert!(!store.exists());
    std::fs::write(&input, tiny().replace(",15,", ",99,")).unwrap();
    assert_eq!(
        service
            .inspect_native_source(&input, Channel::Live)
            .unwrap_err()
            .kind,
        HistoryDiagnosticKind::SourceVersionUnsupported
    );
}

#[test]
fn native_projection_separates_hit_overflow_and_requires_observed_local_actor() {
    use eso_weave::encounter::project_native_encounter;
    let source = tiny().replace("300,0,10", "300,75,10");
    let capture = first(&source);
    let before = eso_weave::encounter::canonical_native_bytes(&capture).unwrap();
    let projection = project_native_encounter(&capture).unwrap();
    assert_eq!(projection.duration_ms, 20);
    assert_eq!(projection.combat_event_count, 1);
    assert_eq!(projection.damage_hit, 300);
    assert_eq!(projection.damage_overflow, 75);
    assert_eq!(projection.local_damage_hit, Some(300));
    assert_eq!(projection.local_damage_per_second, Some(15000.0));
    assert!(projection
        .unavailable
        .iter()
        .any(|reason| reason.contains("uptime")));
    assert!(projection
        .unavailable
        .iter()
        .any(|reason| reason.contains("recommendations")));
    let no_actor_source = source
        .lines()
        .filter(|line| !line.contains(",UNIT_ADDED,"))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    let no_actor = first(&no_actor_source);
    assert_eq!(
        project_native_encounter(&no_actor)
            .unwrap()
            .local_damage_hit,
        None
    );
    assert_eq!(
        eso_weave::encounter::canonical_native_bytes(&capture).unwrap(),
        before
    );
    assert!(!format!("{projection:?}").contains("ActorCanary"));
}

#[test]
fn native_unknown_or_reversed_observations_reduce_rate_claims() {
    use eso_weave::encounter::project_native_encounter;
    let unknown = first(&tiny().replace("30,END_COMBAT", "25,FUTURE_EVENT,1\n30,END_COMBAT"));
    let projection = project_native_encounter(&unknown).unwrap();
    assert_eq!(projection.damage_hit, 300);
    assert_eq!(projection.local_damage_per_second, None);
    assert_eq!(projection.unknown_record_count, 1);
    assert!(!projection.quality_reasons.is_empty());
}

#[test]
fn native_hostile_line_depth_token_utf8_and_boundaries_reject_without_payloads() {
    let mut cases = vec![
        format!("{}10,BEGIN_COMBAT\n20,BEGIN_COMBAT\n", header()),
        format!("{}10,END_COMBAT\n", header()),
        tiny().replace("30,END_COMBAT", "5,END_COMBAT"),
        tiny().replace("PHYSICAL,STAMINA", "PHYSICAL,STAMINA,EXTRA"),
        format!(
            "{}10,FUTURE_EVENT,{}\n",
            header(),
            "[".repeat(9) + "1" + &"]".repeat(9)
        ),
        format!("{}10,FUTURE_EVENT,{}\n", header(), "1,".repeat(4097)),
        format!(
            "{}10,FUTURE_EVENT,\"{}\"\n",
            header(),
            "PrivateCanary".repeat(6000)
        ),
        tiny().replace("15,", "16,"),
    ]
    .into_iter()
    .map(String::into_bytes)
    .collect::<Vec<_>>();
    let mut invalid_tail = tiny().into_bytes();
    invalid_tail.push(0xff);
    cases.push(invalid_tail);
    for source in cases {
        let error = parse_native_log(&source, Channel::Live).unwrap_err();
        assert!(!format!("{error:?}").contains("PrivateCanary"));
    }
}

#[test]
fn native_utf8_bom_crlf_escapes_and_unknown_context_stay_local_and_exact() {
    let source = format!(
        "\u{feff}{}",
        tiny()
            .replace(
                "10,BEGIN_COMBAT",
                "5,FUTURE_CONTEXT,[1,\"Value, Ω\"]\n10,BEGIN_COMBAT"
            )
            .replace("NativeCanary, Ω", "Fixture\\\"Quoted, Ω")
            .replace('\n', "\r\n")
    );
    let capture = first(&source);
    assert_eq!(capture.run.header.record.byte_offset, 3);
    assert_eq!(capture.unknown_context_count, 1);
    assert!(capture
        .context
        .iter()
        .any(|record| record.line.contains("Fixture\\\"Quoted")));
    assert_eq!(
        eso_weave::encounter::project_native_encounter(&capture)
            .unwrap()
            .local_damage_per_second,
        None
    );
}

#[test]
fn native_oversize_source_and_empty_completed_set_preserve_existing_store() {
    use eso_weave::encounter::{import_native_log, ImportRequest};
    let root = tempfile::tempdir().unwrap();
    let source = root.path().join("native.log");
    let store = root.path().join("history.sqlite");
    std::fs::write(&source, tiny()).unwrap();
    import_native_log(&ImportRequest::new(&source, &store, Channel::Live)).unwrap();
    let before = std::fs::read(&store).unwrap();
    std::fs::write(&source, format!("{}10,BEGIN_COMBAT\n", header())).unwrap();
    let report = import_native_log(&ImportRequest::new(&source, &store, Channel::Live)).unwrap();
    assert_eq!(report.imported_count, 0);
    assert_eq!(std::fs::read(&store).unwrap(), before);
    std::fs::File::create(&source)
        .unwrap()
        .set_len(eso_weave::encounter::MAX_NATIVE_FILE_BYTES + 1)
        .unwrap();
    assert!(import_native_log(&ImportRequest::new(&source, &store, Channel::Live)).is_err());
    assert_eq!(std::fs::read(&store).unwrap(), before);
}

#[test]
fn native_persisted_index_and_canonical_tampering_are_rejected() {
    use eso_weave::encounter::{import_native_log, list_encounters, ImportRequest};
    for field in ["record_count", "canonical_json"] {
        let root = tempfile::tempdir().unwrap();
        let source = root.path().join("native.log");
        let store = root.path().join("history.sqlite");
        std::fs::write(&source, tiny()).unwrap();
        import_native_log(&ImportRequest::new(&source, &store, Channel::Live)).unwrap();
        let connection = rusqlite::Connection::open(&store).unwrap();
        let trigger: String = connection
            .query_row(
                "SELECT sql FROM sqlite_master WHERE name='native_encounters_no_update'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        connection
            .execute_batch("DROP TRIGGER native_encounters_no_update")
            .unwrap();
        if field == "record_count" {
            connection
                .execute_batch("UPDATE native_encounters SET record_count=record_count+1")
                .unwrap();
        } else {
            connection
                .execute_batch("UPDATE native_encounters SET canonical_json=x'7b7d'")
                .unwrap();
        }
        // Restore the exact trigger so validation reaches the row integrity check.
        connection.execute_batch(&trigger).unwrap();
        drop(connection);
        assert!(list_encounters(&store).is_err());
    }
}

#[test]
fn native_import_migrates_schema4_and5_without_rewriting_addon_evidence() {
    use eso_weave::encounter::{
        import_encounter, import_native_log, list_encounters, EncounterSourceKind, ImportRequest,
    };
    for version in [4, 5] {
        let root = tempfile::tempdir().unwrap();
        let store = root.path().join("legacy.sqlite");
        let addon = root.path().join("addon.lua");
        let native = root.path().join("native.log");
        std::fs::write(
            &addon,
            include_str!("fixtures/encounter/valid-complete.lua"),
        )
        .unwrap();
        std::fs::write(&native, tiny()).unwrap();
        import_encounter(&ImportRequest::new(&addon, &store, Channel::Live)).unwrap();
        let connection = rusqlite::Connection::open(&store).unwrap();
        let before: (String, Vec<u8>) = connection
            .query_row(
                "SELECT content_sha256,canonical_json FROM raw_encounters",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        connection
            .execute_batch(&format!(
                "
            BEGIN IMMEDIATE;
            DROP TRIGGER native_encounters_no_update;
            DROP TRIGGER native_runs_no_update;
            DROP TABLE native_encounters;
            DROP TABLE native_runs;
            DROP TABLE encounter_store_meta;
            CREATE TABLE encounter_store_meta (
                singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
                schema_version INTEGER NOT NULL CHECK (schema_version = {version}),
                canonical_format_version INTEGER NOT NULL CHECK (canonical_format_version = 2)
            );
            INSERT INTO encounter_store_meta VALUES (1,{version},2);
            PRAGMA user_version={version};
            COMMIT;
        "
            ))
            .unwrap();
        if version == 4 {
            connection.execute_batch("DROP TRIGGER encounter_session_order_no_update; DROP TABLE encounter_session_order;").unwrap();
        }
        drop(connection);
        let old_bytes = std::fs::read(&store).unwrap();
        assert_eq!(list_encounters(&store).unwrap().len(), 1);
        assert_eq!(std::fs::read(&store).unwrap(), old_bytes);
        assert_eq!(
            import_native_log(&ImportRequest::new(&native, &store, Channel::Live))
                .unwrap()
                .imported_count,
            1
        );
        let summaries = list_encounters(&store).unwrap();
        assert_eq!(summaries.len(), 2);
        assert_eq!(summaries[0].source_kind, EncounterSourceKind::SavedAddon);
        assert_eq!(summaries[1].source_kind, EncounterSourceKind::NativeLog);
        let connection = rusqlite::Connection::open(&store).unwrap();
        let after: (String, Vec<u8>) = connection
            .query_row(
                "SELECT content_sha256,canonical_json FROM raw_encounters",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(before, after);
    }
}

#[test]
fn native_observed_totals_keep_valid_large_quantities_exact() {
    let source = tiny().replace("300,0,10", "18446744073709551615,0,10");
    let extra = tiny()
        .lines()
        .find(|line| line.contains(",COMBAT_EVENT,"))
        .unwrap()
        .replace("300,0,10", "1,0,10");
    let capture = first(&source.replace("30,END_COMBAT", &format!("{extra}\n30,END_COMBAT")));
    let projection = eso_weave::encounter::project_native_encounter(&capture).unwrap();
    assert_eq!(projection.damage_hit.to_string(), "18446744073709551616");
    assert_eq!(
        projection.local_damage_hit.unwrap().to_string(),
        "18446744073709551616"
    );
    assert_eq!(projection.combat_event_count, 2);
}

#[test]
fn native_repeated_irrelevant_context_hits_a_cumulative_work_limit_atomically() {
    use eso_weave::encounter::{import_native_log, ImportRequest};
    let mut source = header().to_owned();
    // A small physical file can amplify a long irrelevant name across tiny fights.
    source.push_str(&format!(
        "1,ABILITY_INFO,99,\"{}\",\"\",F,F\n",
        "Synthetic".repeat(7000)
    ));
    for _ in 0..10_000 {
        source.push_str("10,BEGIN_COMBAT\n20,END_COMBAT\n");
    }
    source.push_str("30,END_LOG\n");
    assert!(source.len() < 1024 * 1024);
    let root = tempfile::tempdir().unwrap();
    let input = root.path().join("native.log");
    let store = root.path().join("history.sqlite");
    std::fs::write(&input, &source).unwrap();
    let result = import_native_log(&ImportRequest::new(&input, &store, Channel::Live));
    assert!(
        result.is_err(),
        "cumulative context amplification was accepted"
    );
    let error = result.unwrap_err();
    assert!(error.to_string().contains("context work"));
    assert!(!error.to_string().contains("Synthetic"));
    assert!(!store.exists());
    assert_eq!(std::fs::read(&input).unwrap(), source.as_bytes());
}
