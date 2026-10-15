use eso_weave::catalog::Channel;
use eso_weave::encounter::{
    backup_store, delete_all, delete_encounter, import_encounter, import_native_log,
    list_encounters, ImportRequest, IncrementalNativeParser, NativeIngestBatch, NativeIngestWriter,
};

const SOURCE: &str = "0,BEGIN_LOG,1700000000000,15,\"FixtureRealm\",\"zz\",\"fixture.game\"\n10,BEGIN_COMBAT\n30,END_COMBAT\n40,END_LOG\n";

fn batch(source: &str, source_key: char, revision: Option<u64>) -> NativeIngestBatch {
    let mut parser = IncrementalNativeParser::new(Channel::Live);
    let feed = parser.feed(source.as_bytes()).unwrap();
    NativeIngestBatch {
        source_key: source_key.to_string().repeat(64),
        generation: "b".repeat(64),
        expected_revision: revision,
        consumed_bytes: parser.position().accepted_bytes,
        checkpoint_bytes: parser.checkpoint().unwrap().as_bytes().to_vec(),
        completed: feed.completed,
    }
}

#[test]
fn ingest_commit_and_progress_are_one_atomic_unit() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("history.sqlite");
    let mut writer = NativeIngestWriter::open(&path).unwrap();
    assert!(writer.checkpoint(&"a".repeat(64)).unwrap().is_none());
    assert!(writer
        .commit_batch(batch(SOURCE, 'a', None), |_| Ok(()))
        .is_err());
    assert!(writer.checkpoint(&"a".repeat(64)).unwrap().is_none());
    let receipt = writer
        .commit_batch(batch(SOURCE, 'a', None), |commit| commit.commit())
        .unwrap();
    assert_eq!(receipt.imported_count, 1);
    assert_eq!(receipt.revision, 1);
    let checkpoint = writer.checkpoint(&"a".repeat(64)).unwrap().unwrap();
    assert_eq!(checkpoint.revision, 1);
    assert_eq!(checkpoint.consumed_bytes, receipt.consumed_bytes);
    assert_eq!(list_encounters(&path).unwrap().len(), 1);
    assert!(writer
        .commit_batch(batch(SOURCE, 'a', None), |commit| commit.commit())
        .is_err());
    assert_eq!(
        writer
            .checkpoint(&"a".repeat(64))
            .unwrap()
            .unwrap()
            .revision,
        1
    );
    let connection = rusqlite::Connection::open(&path).unwrap();
    assert_eq!(
        connection
            .query_row("SELECT count(*) FROM encounter_derivations", [], |r| r
                .get::<_, i64>(0))
            .unwrap(),
        1
    );
}

#[test]
fn ingest_copy_alias_preserves_legacy_bytes_identity_and_order() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("native.log");
    let addon = temp.path().join("addon.lua");
    let path = temp.path().join("history.sqlite");
    std::fs::write(&source, SOURCE).unwrap();
    std::fs::write(
        &addon,
        include_str!("fixtures/encounter/valid-complete.lua"),
    )
    .unwrap();
    import_encounter(&ImportRequest::new(&addon, &path, Channel::Live)).unwrap();
    let original = import_native_log(&ImportRequest::new(&source, &path, Channel::Live)).unwrap();
    let before = list_encounters(&path).unwrap();
    let connection = rusqlite::Connection::open(&path).unwrap();
    let canonical: Vec<u8> = connection
        .query_row("SELECT canonical_json FROM native_encounters", [], |r| {
            r.get(0)
        })
        .unwrap();
    drop(connection);
    let mut writer = NativeIngestWriter::open(&path).unwrap();
    let receipt = writer
        .commit_batch(batch(SOURCE, 'a', None), |commit| commit.commit())
        .unwrap();
    assert_eq!(receipt.imported_count, 0);
    assert_eq!(receipt.already_present_count, 1);
    assert_eq!(list_encounters(&path).unwrap(), before);
    let connection = rusqlite::Connection::open(&path).unwrap();
    assert_eq!(
        connection
            .query_row("SELECT canonical_json FROM native_encounters", [], |r| r
                .get::<_, Vec<u8>>(0))
            .unwrap(),
        canonical
    );
    assert_eq!(
        list_encounters(&path).unwrap()[1].encounter_id,
        original.receipts[0].encounter_id
    );
}

#[test]
fn compatibility_copy_reuses_automatic_identity_bytes_and_first_import_order() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("history.sqlite");
    let addon = temp.path().join("addon.lua");
    let source = temp.path().join("completed.log");
    std::fs::write(
        &addon,
        include_str!("fixtures/encounter/valid-complete.lua"),
    )
    .unwrap();
    std::fs::write(&source, SOURCE).unwrap();
    import_encounter(&ImportRequest::new(&addon, &path, Channel::Live)).unwrap();
    let mut writer = NativeIngestWriter::open(&path).unwrap();
    writer
        .commit_batch(batch(SOURCE, 'a', None), |c| c.commit())
        .unwrap();
    let before = list_encounters(&path).unwrap();
    let connection = rusqlite::Connection::open(&path).unwrap();
    let raw: Vec<u8> = connection
        .query_row("SELECT canonical_json FROM native_encounters", [], |r| {
            r.get(0)
        })
        .unwrap();
    drop(connection);
    let copied = import_native_log(&ImportRequest::new(&source, &path, Channel::Live)).unwrap();
    assert_eq!(copied.imported_count, 0);
    assert_eq!(copied.already_present_count, 1);
    assert_eq!(copied.receipts[0].session_id, before[1].session_id);
    assert_eq!(copied.receipts[0].encounter_id, before[1].encounter_id);
    assert_eq!(copied.receipts[0].content_sha256, before[1].content_sha256);
    assert_eq!(list_encounters(&path).unwrap(), before);
    let connection = rusqlite::Connection::open(&path).unwrap();
    assert_eq!(
        connection
            .query_row("SELECT count(*) FROM native_runs", [], |r| r
                .get::<_, i64>(0))
            .unwrap(),
        1
    );
    assert_eq!(
        connection
            .query_row("SELECT count(*) FROM encounter_session_order", [], |r| r
                .get::<_, i64>(0))
            .unwrap(),
        2
    );
    assert_eq!(
        connection
            .query_row("SELECT canonical_json FROM native_encounters", [], |r| r
                .get::<_, Vec<u8>>(0))
            .unwrap(),
        raw
    );
    assert_eq!(std::fs::read(&source).unwrap(), SOURCE.as_bytes());
}

#[test]
fn compatibility_copy_cannot_resurrect_automatic_deletion() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("history.sqlite");
    let source = temp.path().join("completed.log");
    std::fs::write(&source, SOURCE).unwrap();
    let mut writer = NativeIngestWriter::open(&path).unwrap();
    writer
        .commit_batch(batch(SOURCE, 'a', None), |c| c.commit())
        .unwrap();
    let retained = list_encounters(&path).unwrap().remove(0);
    delete_encounter(&path, &retained.session_id, &retained.encounter_id).unwrap();
    let before = std::fs::read(&path).unwrap();
    assert!(import_native_log(&ImportRequest::new(&source, &path, Channel::Live)).is_err());
    assert!(list_encounters(&path).unwrap().is_empty());
    assert_eq!(std::fs::read(&path).unwrap(), before);
    assert_eq!(std::fs::read(&source).unwrap(), SOURCE.as_bytes());
    let mixed = format!(
        "{}{SOURCE}",
        SOURCE.replace("1700000000000", "1700000100000")
    );
    std::fs::write(&source, &mixed).unwrap();
    assert!(import_native_log(&ImportRequest::new(&source, &path, Channel::Live)).is_err());
    assert!(list_encounters(&path).unwrap().is_empty());
    assert_eq!(std::fs::read(&path).unwrap(), before);
    assert_eq!(std::fs::read(&source).unwrap(), mixed.as_bytes());
    assert_eq!(
        writer
            .commit_batch(batch(SOURCE, 'c', None), |c| c.commit())
            .unwrap()
            .suppressed_count,
        1
    );
}

#[test]
fn compatibility_duplicate_runs_share_one_preflight_identity_and_divergence_rolls_back() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("completed.log");
    let path = temp.path().join("history.sqlite");
    let duplicated = format!("{SOURCE}{SOURCE}");
    std::fs::write(&source, &duplicated).unwrap();
    let report = import_native_log(&ImportRequest::new(&source, &path, Channel::Live)).unwrap();
    assert_eq!(report.imported_count, 1);
    assert_eq!(report.already_present_count, 1);
    assert_eq!(report.receipts[0].session_id, report.receipts[1].session_id);
    assert_eq!(
        report.receipts[0].encounter_id,
        report.receipts[1].encounter_id
    );
    assert_eq!(
        report.receipts[0].content_sha256,
        report.receipts[1].content_sha256
    );
    assert_eq!(list_encounters(&path).unwrap().len(), 1);
    let before = std::fs::read(&path).unwrap();
    let conflict = format!(
        "{SOURCE}{}",
        SOURCE.replace("30,END_COMBAT", "31,END_COMBAT")
    );
    std::fs::write(&source, &conflict).unwrap();
    assert!(import_native_log(&ImportRequest::new(&source, &path, Channel::Live)).is_err());
    assert_eq!(std::fs::read(&path).unwrap(), before);
    assert_eq!(std::fs::read(&source).unwrap(), conflict.as_bytes());
}

#[test]
fn ingest_deleted_native_history_does_not_return_from_a_copy_or_backup() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("history.sqlite");
    let mut writer = NativeIngestWriter::open(&path).unwrap();
    writer
        .commit_batch(batch(SOURCE, 'a', None), |commit| commit.commit())
        .unwrap();
    let first = list_encounters(&path).unwrap().remove(0);
    delete_encounter(&path, &first.session_id, &first.encounter_id).unwrap();
    let receipt = writer
        .commit_batch(batch(SOURCE, 'c', None), |commit| commit.commit())
        .unwrap();
    assert_eq!(receipt.suppressed_count, 1);
    assert!(list_encounters(&path).unwrap().is_empty());
    let later = SOURCE.replace("1700000000000", "1700000100000");
    assert_eq!(
        writer
            .commit_batch(batch(&later, 'd', None), |commit| commit.commit())
            .unwrap()
            .imported_count,
        1
    );
    delete_all(&path).unwrap();
    let backup = temp.path().join("backup.sqlite");
    backup_store(&path, &backup).unwrap();
    drop(writer);
    let mut restored = NativeIngestWriter::open(&backup).unwrap();
    assert_eq!(
        restored
            .commit_batch(batch(&later, 'e', None), |commit| commit.commit())
            .unwrap()
            .suppressed_count,
        1
    );
    assert!(list_encounters(&backup).unwrap().is_empty());
}

#[test]
fn ingest_sqlite_failure_preserves_checkpoint_and_source() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("history.sqlite");
    let mut writer = NativeIngestWriter::open(&path).unwrap();
    writer
        .commit_batch(batch(SOURCE, 'a', None), |commit| commit.commit())
        .unwrap();
    let previous = writer.checkpoint(&"a".repeat(64)).unwrap().unwrap();
    let other = rusqlite::Connection::open(&path).unwrap();
    other.execute_batch("BEGIN IMMEDIATE;").unwrap();
    assert!(writer
        .commit_batch(batch(SOURCE, 'a', Some(1)), |commit| commit.commit())
        .is_err());
    other.execute_batch("ROLLBACK;").unwrap();
    assert_eq!(
        writer
            .checkpoint(&"a".repeat(64))
            .unwrap()
            .unwrap()
            .checkpoint_bytes,
        previous.checkpoint_bytes
    );
    assert_eq!(
        writer
            .checkpoint(&"a".repeat(64))
            .unwrap()
            .unwrap()
            .revision,
        1
    );
}

#[test]
fn ingest_schema6_migration_rolls_back_then_preserves_raw_bytes() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("legacy.sqlite");
    let source = temp.path().join("native.log");
    std::fs::write(&source, SOURCE).unwrap();
    import_native_log(&ImportRequest::new(&source, &path, Channel::Live)).unwrap();
    let connection = rusqlite::Connection::open(&path).unwrap();
    let mut runs: String = connection
        .query_row(
            "SELECT sql FROM sqlite_schema WHERE name='native_runs'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    let mut fights: String = connection
        .query_row(
            "SELECT sql FROM sqlite_schema WHERE name='native_encounters'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    runs = runs.replace(
        "native_schema_version IN (1, 2)",
        "native_schema_version = 1",
    );
    fights = fights.replace(
        "native_schema_version IN (1, 2)",
        "native_schema_version = 1",
    );
    connection.execute_batch(&format!("BEGIN IMMEDIATE;
        DROP TABLE native_ingest_checkpoints; DROP TABLE native_deleted_encounters; DROP TABLE native_logical_aliases; DROP TABLE encounter_derivations;
        DROP TRIGGER native_runs_no_update; DROP TRIGGER native_encounters_no_update;
        ALTER TABLE native_runs RENAME TO old_runs; ALTER TABLE native_encounters RENAME TO old_fights;
        {runs}; {fights}; INSERT INTO native_runs SELECT * FROM old_runs; INSERT INTO native_encounters SELECT * FROM old_fights;
        DROP TABLE old_fights; DROP TABLE old_runs;
        CREATE TRIGGER native_runs_no_update BEFORE UPDATE ON native_runs BEGIN SELECT RAISE(ABORT, 'native recording runs are immutable'); END;
        CREATE TRIGGER native_encounters_no_update BEFORE UPDATE ON native_encounters BEGIN SELECT RAISE(ABORT, 'native encounters are immutable'); END;
        DROP TABLE encounter_store_meta;
        CREATE TABLE encounter_store_meta (
            singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
            schema_version INTEGER NOT NULL CHECK (schema_version = 6),
            canonical_format_version INTEGER NOT NULL CHECK (canonical_format_version = 2)
        );
        INSERT INTO encounter_store_meta VALUES (1,6,2); PRAGMA user_version=6; COMMIT;")).unwrap();
    let raw: Vec<u8> = connection
        .query_row("SELECT canonical_json FROM native_encounters", [], |r| {
            r.get(0)
        })
        .unwrap();
    drop(connection);
    let mut writer = NativeIngestWriter::open(&path).unwrap();
    assert!(writer
        .commit_batch(batch(SOURCE, 'a', None), |_| Ok(()))
        .is_err());
    let connection = rusqlite::Connection::open(&path).unwrap();
    assert_eq!(
        connection
            .pragma_query_value::<u32, _>(None, "user_version", |r| r.get(0))
            .unwrap(),
        6
    );
    assert_eq!(
        connection
            .query_row("SELECT canonical_json FROM native_encounters", [], |r| r
                .get::<_, Vec<u8>>(0))
            .unwrap(),
        raw
    );
    drop(connection);
    assert_eq!(
        writer
            .commit_batch(batch(SOURCE, 'a', None), |c| c.commit())
            .unwrap()
            .already_present_count,
        1
    );
    let connection = rusqlite::Connection::open(&path).unwrap();
    assert_eq!(
        connection
            .pragma_query_value::<u32, _>(None, "user_version", |r| r.get(0))
            .unwrap(),
        7
    );
    assert_eq!(
        connection
            .query_row("SELECT canonical_json FROM native_encounters", [], |r| r
                .get::<_, Vec<u8>>(0))
            .unwrap(),
        raw
    );
}

#[test]
fn retained_domain_processing_survives_missing_catalog_restart_and_activation() {
    use eso_weave::catalog::compiler::{build_catalog, BuildRequest};
    use eso_weave::encounter::EncounterHistoryService;
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("history.sqlite");
    let addon = temp.path().join("addon.lua");
    std::fs::write(
        &addon,
        include_str!("fixtures/encounter/valid-complete.lua"),
    )
    .unwrap();
    import_encounter(&ImportRequest::new(&addon, &path, Channel::Live)).unwrap();
    let missing = temp.path().join("missing.sqlite");
    let history = EncounterHistoryService::from_paths(&path, &missing);
    let mut processor = history.processor().unwrap();
    let first = processor.process_pending(1, |c| c.commit()).unwrap();
    assert_eq!(first.processed_count, 1);
    assert_eq!(first.deferred_count, 1);
    assert_eq!(
        processor
            .process_pending(1, |c| c.commit())
            .unwrap()
            .processed_count,
        0
    );
    drop(processor);
    let mut restarted = EncounterHistoryService::from_paths(&path, &missing)
        .processor()
        .unwrap();
    assert_eq!(
        restarted
            .process_pending(1, |c| c.commit())
            .unwrap()
            .processed_count,
        0
    );
    let catalog = temp.path().join("catalog.sqlite");
    build_catalog(&BuildRequest::new(
        "specs/070-catalog-compiler/fixtures/minimal-live.json",
        &catalog,
        Channel::Live,
    ))
    .unwrap();
    history.set_catalog_path(&catalog);
    let mut processor = history.processor().unwrap();
    assert_eq!(
        processor
            .process_pending(1, |c| c.commit())
            .unwrap()
            .processed_count,
        1
    );
    assert_eq!(
        processor
            .process_pending(1, |c| c.commit())
            .unwrap()
            .processed_count,
        0
    );
    let connection = rusqlite::Connection::open(&path).unwrap();
    let rows: Vec<Vec<u8>> = connection
        .prepare("SELECT result_json FROM encounter_derivations WHERE catalog_sha256<>''")
        .unwrap()
        .query_map([], |r| r.get(0))
        .unwrap()
        .map(Result::unwrap)
        .collect();
    assert_eq!(rows.len(), 1);
    let result: serde_json::Value = serde_json::from_slice(&rows[0]).unwrap();
    assert_eq!(result["availability"], "available");
    assert!(result["projection"]["catalog_join"]["catalog_semantic_sha256"].is_string());
    assert!(result["recommendations"]["citation"].is_null());
    assert_eq!(
        result["recommendations"]["evidence"]["citation"]["raw_content_sha256"],
        result["projection"]["raw_content_sha256"]
    );
}

#[test]
fn shifted_copy_keeps_first_evidence_and_accepts_new_fights_in_same_logical_run() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("history.sqlite");
    let mut writer = NativeIngestWriter::open(&path).unwrap();
    writer
        .commit_batch(batch(SOURCE, 'a', None), |c| c.commit())
        .unwrap();
    let original = list_encounters(&path).unwrap().remove(0);
    let shifted = format!(
        "0,BEGIN_LOG,1600000000000,15,\"OtherRealm\",\"zz\",\"fixture.game\"\n1,END_LOG\n{}",
        SOURCE.replace("40,END_LOG", "50,BEGIN_COMBAT\n60,END_COMBAT\n70,END_LOG")
    );
    let mut parser = IncrementalNativeParser::new(Channel::Live);
    let first = parser.feed(shifted.as_bytes()).unwrap();
    let first_accepted = first.accepted;
    let receipt = writer
        .commit_batch(
            NativeIngestBatch {
                source_key: "c".repeat(64),
                generation: "d".repeat(64),
                expected_revision: None,
                consumed_bytes: parser.position().accepted_bytes,
                checkpoint_bytes: parser.checkpoint().unwrap().as_bytes().to_vec(),
                completed: first.completed,
            },
            |c| c.commit(),
        )
        .unwrap();
    assert_eq!(receipt.already_present_count, 1);
    let second = parser.feed(&shifted.as_bytes()[first_accepted..]).unwrap();
    assert_eq!(
        writer
            .commit_batch(
                NativeIngestBatch {
                    source_key: "c".repeat(64),
                    generation: "d".repeat(64),
                    expected_revision: Some(1),
                    consumed_bytes: parser.position().accepted_bytes,
                    checkpoint_bytes: parser.checkpoint().unwrap().as_bytes().to_vec(),
                    completed: second.completed
                },
                |c| c.commit()
            )
            .unwrap()
            .imported_count,
        1
    );
    let history = list_encounters(&path).unwrap();
    assert_eq!(history.len(), 2);
    assert_eq!(history[0], original);
    assert_eq!(history[1].session_id, original.session_id);
    assert!(history[1].first_sequence > history[0].last_sequence);
    backup_store(&path, temp.path().join("backup.sqlite")).unwrap();
}

#[test]
fn failed_checkpoint_write_rolls_back_new_fight_and_projection() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("history.sqlite");
    let mut writer = NativeIngestWriter::open(&path).unwrap();
    writer
        .commit_batch(batch(SOURCE, 'a', None), |c| c.commit())
        .unwrap();
    let connection = rusqlite::Connection::open(&path).unwrap();
    connection.execute_batch("CREATE TEMP TABLE unused(value); CREATE TRIGGER simulate_checkpoint_failure BEFORE INSERT ON native_ingest_checkpoints BEGIN SELECT RAISE(ABORT, 'injected checkpoint failure'); END;").unwrap();
    let later = SOURCE.replace("1700000000000", "1700000100000");
    assert!(writer
        .commit_batch(batch(&later, 'c', None), |c| c.commit())
        .is_err());
    connection
        .execute_batch("DROP TRIGGER simulate_checkpoint_failure;")
        .unwrap();
    assert!(writer.checkpoint(&"c".repeat(64)).unwrap().is_none());
    assert_eq!(list_encounters(&path).unwrap().len(), 1);
    assert_eq!(
        connection
            .query_row("SELECT count(*) FROM encounter_derivations", [], |r| r
                .get::<_, i64>(0))
            .unwrap(),
        1
    );
    assert_eq!(
        writer
            .commit_batch(batch(&later, 'c', None), |c| c.commit())
            .unwrap()
            .imported_count,
        1
    );
}

#[test]
fn divergent_logical_fight_and_mismatched_checkpoint_do_not_commit() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("history.sqlite");
    let mut writer = NativeIngestWriter::open(&path).unwrap();
    writer
        .commit_batch(batch(SOURCE, 'a', None), |c| c.commit())
        .unwrap();
    let conflicting = SOURCE.replace("30,END_COMBAT", "31,END_COMBAT");
    assert!(writer
        .commit_batch(batch(&conflicting, 'c', None), |c| c.commit())
        .is_err());
    assert!(writer.checkpoint(&"c".repeat(64)).unwrap().is_none());
    let mut malformed = batch(SOURCE, 'd', None);
    malformed.consumed_bytes += 1;
    assert!(writer.commit_batch(malformed, |c| c.commit()).is_err());
    assert_eq!(list_encounters(&path).unwrap().len(), 1);
}

#[test]
fn corrupt_checkpoint_store_is_preserved_instead_of_reset() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("history.sqlite");
    let mut writer = NativeIngestWriter::open(&path).unwrap();
    writer
        .commit_batch(batch(SOURCE, 'a', None), |c| c.commit())
        .unwrap();
    drop(writer);
    let connection = rusqlite::Connection::open(&path).unwrap();
    connection
        .execute(
            "UPDATE native_ingest_checkpoints SET checkpoint_bytes=x'7b7d'",
            [],
        )
        .unwrap();
    drop(connection);
    let original = std::fs::read(&path).unwrap();
    assert!(NativeIngestWriter::open(&path).is_err());
    assert_eq!(std::fs::read(&path).unwrap(), original);
}

#[test]
fn terminal_checkpoint_archive_and_shorter_replacement_share_durable_revision_rules() {
    use eso_weave::encounter::NativeTerminationReason;
    for reason in [
        NativeTerminationReason::Replaced,
        NativeTerminationReason::Truncated,
    ] {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("history.sqlite");
        let mut writer = NativeIngestWriter::open(&path).unwrap();
        let unfinished = SOURCE.replace("40,END_LOG", "50,BEGIN_COMBAT\n60,FUTURE_RECORD");
        let mut parser = IncrementalNativeParser::new(Channel::Live);
        let completed = parser.feed(unfinished.as_bytes()).unwrap();
        let consumed = completed.accepted;
        writer
            .commit_batch(
                NativeIngestBatch {
                    source_key: "a".repeat(64),
                    generation: "b".repeat(64),
                    expected_revision: None,
                    consumed_bytes: parser.position().accepted_bytes,
                    checkpoint_bytes: parser.checkpoint().unwrap().as_bytes().to_vec(),
                    completed: completed.completed,
                },
                |c| c.commit(),
            )
            .unwrap();
        parser.feed(&unfinished.as_bytes()[consumed..]).unwrap();
        let suspension = parser.checkpoint().unwrap();
        let position = parser.position().accepted_bytes;
        writer
            .commit_batch(
                NativeIngestBatch {
                    source_key: "a".repeat(64),
                    generation: "b".repeat(64),
                    expected_revision: Some(1),
                    consumed_bytes: position,
                    checkpoint_bytes: suspension.as_bytes().to_vec(),
                    completed: None,
                },
                |c| c.commit(),
            )
            .unwrap();
        parser.terminate_generation(reason).unwrap();
        writer
            .commit_batch(
                NativeIngestBatch {
                    source_key: "c".repeat(64),
                    generation: "b".repeat(64),
                    expected_revision: None,
                    consumed_bytes: position,
                    checkpoint_bytes: parser.checkpoint().unwrap().as_bytes().to_vec(),
                    completed: None,
                },
                |c| c.commit(),
            )
            .unwrap();
        let replacement = SOURCE.replace("1700000000000", "1700000100000");
        let mut next = batch(&replacement, 'a', Some(2));
        next.generation = "d".repeat(64);
        assert!(next.consumed_bytes < position);
        assert_eq!(
            writer.commit_batch(next, |c| c.commit()).unwrap().revision,
            3
        );
        assert_eq!(
            writer
                .checkpoint(&"c".repeat(64))
                .unwrap()
                .unwrap()
                .consumed_bytes,
            position
        );
        assert_eq!(list_encounters(&path).unwrap().len(), 2);
        drop(writer);
        NativeIngestWriter::open(&path).unwrap();
    }
}
