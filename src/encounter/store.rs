use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

use rusqlite::{params, Connection, OpenFlags, OptionalExtension, TransactionBehavior};

use crate::bounded_file::is_link_like;
use crate::catalog::Channel;

use super::model::{
    BackupReceipt, CaptureImportReport, CaptureMode, CaptureStatus, DeleteReceipt,
    EncounterCapture, EncounterSummary, ImportOutcome, ImportReceipt, OrderedEncounterCapture,
    ParsedCaptureSet, SessionEncounterReference, SessionSnapshot,
};
use super::{
    ensure_distinct_paths, invalid, sha256, EncounterError, CANONICAL_FORMAT_VERSION,
    STORE_SCHEMA_VERSION,
};

mod ingest;
pub use ingest::{
    NativeCommit, NativeIngestBatch, NativeIngestCheckpoint, NativeIngestReceipt,
    NativeIngestWriter,
};
pub(crate) use ingest::{PendingCursor, PreparedDerivation};

const MAX_SESSION_SNAPSHOT_BYTES: usize = 1024 * 1024;

const V1_META_TABLE_SQL: &str = r#"CREATE TABLE encounter_store_meta (
    singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
    schema_version INTEGER NOT NULL CHECK (schema_version = 1),
    canonical_format_version INTEGER NOT NULL CHECK (canonical_format_version = 1)
)"#;
const V1_RAW_TABLE_SQL: &str = r#"CREATE TABLE raw_encounters (
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
)"#;
const V2_META_TABLE_SQL: &str = r#"CREATE TABLE encounter_store_meta (
    singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
    schema_version INTEGER NOT NULL CHECK (schema_version = 2),
    canonical_format_version INTEGER NOT NULL CHECK (canonical_format_version = 2)
)"#;
const V2_RAW_TABLE_SQL: &str = r#"CREATE TABLE raw_encounters (
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
)"#;
const V3_META_TABLE_SQL: &str = r#"CREATE TABLE encounter_store_meta (
    singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
    schema_version INTEGER NOT NULL CHECK (schema_version = 3),
    canonical_format_version INTEGER NOT NULL CHECK (canonical_format_version = 2)
)"#;
const V3_RAW_TABLE_SQL: &str = r#"CREATE TABLE raw_encounters (
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
)"#;
const V4_META_TABLE_SQL: &str = r#"CREATE TABLE encounter_store_meta (
    singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
    schema_version INTEGER NOT NULL CHECK (schema_version = 4),
    canonical_format_version INTEGER NOT NULL CHECK (canonical_format_version = 2)
)"#;
const V5_META_TABLE_SQL: &str = r#"CREATE TABLE encounter_store_meta (
    singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
    schema_version INTEGER NOT NULL CHECK (schema_version = 5),
    canonical_format_version INTEGER NOT NULL CHECK (canonical_format_version = 2)
)"#;
const V6_META_TABLE_SQL: &str = r#"CREATE TABLE encounter_store_meta (
    singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
    schema_version INTEGER NOT NULL CHECK (schema_version = 6),
    canonical_format_version INTEGER NOT NULL CHECK (canonical_format_version = 2)
)"#;
const META_TABLE_SQL: &str = r#"CREATE TABLE encounter_store_meta (
    singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
    schema_version INTEGER NOT NULL CHECK (schema_version = 7),
    canonical_format_version INTEGER NOT NULL CHECK (canonical_format_version = 2)
)"#;
const ORDER_TABLE_SQL: &str = r#"CREATE TABLE encounter_session_order (
    session_id TEXT PRIMARY KEY,
    import_ordinal INTEGER NOT NULL UNIQUE CHECK (import_ordinal > 0)
)"#;
const ORDER_IMMUTABILITY_TRIGGER_SQL: &str = r#"CREATE TRIGGER encounter_session_order_no_update
BEFORE UPDATE ON encounter_session_order
BEGIN
    SELECT RAISE(ABORT, 'encounter session order is immutable');
END"#;
const ORDER_BACKFILL_SQL: &str = r#"INSERT INTO encounter_session_order(session_id, import_ordinal)
SELECT session_id, ROW_NUMBER() OVER (ORDER BY session_id)
FROM (SELECT session_id FROM raw_encounters UNION SELECT session_id FROM encounter_session_snapshots)"#;
const META_INSERT_SQL: &str = r#"INSERT INTO encounter_store_meta(singleton, schema_version, canonical_format_version)
VALUES (1, 7, 2)"#;
const RAW_TABLE_SQL: &str = r#"CREATE TABLE raw_encounters (
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
    capture_mode TEXT NOT NULL CHECK (capture_mode IN ('single', 'continuous')),
    encounter_ordinal INTEGER NOT NULL CHECK (encounter_ordinal > 0),
    canonical_json BLOB NOT NULL,
    UNIQUE (session_id, encounter_id),
    UNIQUE (session_id, encounter_ordinal),
    CHECK ((capture_schema_version = 1 AND canonical_format_version = 1) OR
           (capture_schema_version = 2 AND canonical_format_version = 2))
)"#;
const SESSION_TABLE_SQL: &str = r#"CREATE TABLE encounter_session_snapshots (
    session_id TEXT NOT NULL,
    revision INTEGER NOT NULL CHECK (revision > 0),
    content_sha256 TEXT NOT NULL CHECK (length(content_sha256) = 64),
    channel TEXT NOT NULL CHECK (channel IN ('live', 'pts')),
    mode TEXT NOT NULL CHECK (mode IN ('single', 'continuous')),
    disposition TEXT NOT NULL CHECK (disposition IN ('active', 'stopped', 'failed')),
    started_at TEXT NOT NULL,
    finished_at TEXT,
    interruption_count INTEGER NOT NULL CHECK (interruption_count >= 0),
    canonical_json BLOB NOT NULL CHECK (length(canonical_json) <= 1048576),
    PRIMARY KEY (session_id, revision),
    UNIQUE (content_sha256)
)"#;
const IMMUTABILITY_TRIGGER_SQL: &str = r#"CREATE TRIGGER raw_encounters_no_update
BEFORE UPDATE ON raw_encounters
BEGIN
    SELECT RAISE(ABORT, 'raw encounter records are immutable');
END"#;
const SESSION_IMMUTABILITY_TRIGGER_SQL: &str = r#"CREATE TRIGGER encounter_session_snapshots_no_update
BEFORE UPDATE ON encounter_session_snapshots
BEGIN
    SELECT RAISE(ABORT, 'encounter session snapshots are immutable');
END"#;

const V6_NATIVE_RUN_TABLE_SQL: &str = r#"CREATE TABLE native_runs (
    session_id TEXT PRIMARY KEY,
    content_sha256 TEXT NOT NULL CHECK (length(content_sha256) = 64),
    channel TEXT NOT NULL CHECK (channel IN ('live', 'pts')),
    native_schema_version INTEGER NOT NULL CHECK (native_schema_version = 1),
    run_ordinal INTEGER NOT NULL CHECK (run_ordinal > 0),
    canonical_json BLOB NOT NULL
)"#;
const V6_NATIVE_TABLE_SQL: &str = r#"CREATE TABLE native_encounters (
    content_sha256 TEXT PRIMARY KEY CHECK (length(content_sha256) = 64),
    source_sha256 TEXT NOT NULL CHECK (length(source_sha256) = 64),
    session_id TEXT NOT NULL,
    encounter_id TEXT NOT NULL,
    channel TEXT NOT NULL CHECK (channel IN ('live', 'pts')),
    native_schema_version INTEGER NOT NULL CHECK (native_schema_version = 1),
    encounter_ordinal INTEGER NOT NULL CHECK (encounter_ordinal > 0),
    started_at TEXT NOT NULL,
    finished_at TEXT NOT NULL,
    first_sequence INTEGER NOT NULL,
    last_sequence INTEGER NOT NULL,
    record_count INTEGER NOT NULL,
    unknown_count INTEGER NOT NULL,
    canonical_json BLOB NOT NULL,
    UNIQUE (session_id, encounter_id),
    FOREIGN KEY (session_id) REFERENCES native_runs(session_id)
)"#;
const NATIVE_RUN_TABLE_SQL: &str = r#"CREATE TABLE native_runs (
    session_id TEXT PRIMARY KEY,
    content_sha256 TEXT NOT NULL CHECK (length(content_sha256) = 64),
    channel TEXT NOT NULL CHECK (channel IN ('live', 'pts')),
    native_schema_version INTEGER NOT NULL CHECK (native_schema_version IN (1, 2)),
    run_ordinal INTEGER NOT NULL CHECK (run_ordinal > 0),
    canonical_json BLOB NOT NULL
)"#;
const NATIVE_TABLE_SQL: &str = r#"CREATE TABLE native_encounters (
    content_sha256 TEXT PRIMARY KEY CHECK (length(content_sha256) = 64),
    source_sha256 TEXT NOT NULL CHECK (length(source_sha256) = 64),
    session_id TEXT NOT NULL,
    encounter_id TEXT NOT NULL,
    channel TEXT NOT NULL CHECK (channel IN ('live', 'pts')),
    native_schema_version INTEGER NOT NULL CHECK (native_schema_version IN (1, 2)),
    encounter_ordinal INTEGER NOT NULL CHECK (encounter_ordinal > 0),
    started_at TEXT NOT NULL,
    finished_at TEXT NOT NULL,
    first_sequence INTEGER NOT NULL,
    last_sequence INTEGER NOT NULL,
    record_count INTEGER NOT NULL,
    unknown_count INTEGER NOT NULL,
    canonical_json BLOB NOT NULL,
    UNIQUE (session_id, encounter_id),
    FOREIGN KEY (session_id) REFERENCES native_runs(session_id)
)"#;
const NATIVE_RUN_TRIGGER_SQL: &str = r#"CREATE TRIGGER native_runs_no_update
BEFORE UPDATE ON native_runs
BEGIN
    SELECT RAISE(ABORT, 'native recording runs are immutable');
END"#;
const NATIVE_TRIGGER_SQL: &str = r#"CREATE TRIGGER native_encounters_no_update
BEFORE UPDATE ON native_encounters
BEGIN
    SELECT RAISE(ABORT, 'native encounters are immutable');
END"#;

fn native_schema_sql() -> String {
    format!("{NATIVE_RUN_TABLE_SQL};\n{NATIVE_TABLE_SQL};\n{NATIVE_RUN_TRIGGER_SQL};\n{NATIVE_TRIGGER_SQL};")
}

struct StoredRecord {
    content_sha256: String,
    source_sha256: String,
    session_id: String,
    encounter_id: String,
    channel: String,
    capture_schema_version: i64,
    addon_version: i64,
    canonical_format_version: i64,
    status: String,
    started_at: String,
    finished_at: String,
    first_sequence: i64,
    last_sequence: i64,
    stored_event_count: i64,
    omitted_event_count: i64,
    capture_mode: String,
    encounter_ordinal: i64,
    canonical_json: Vec<u8>,
}

struct StoredSessionMember {
    content_sha256: String,
    session_id: String,
    encounter_id: String,
    channel: String,
    capture_mode: String,
    encounter_ordinal: i64,
}

pub(crate) fn append_set(
    store_path: &Path,
    parsed: &ParsedCaptureSet,
    prepared: &[(Vec<u8>, String)],
    source_sha256: String,
) -> Result<CaptureImportReport, EncounterError> {
    if parsed.records.len() != prepared.len() {
        return invalid("prepared encounter count does not match parsed records");
    }
    let mut connection = open_store(store_path, true, true)?;
    let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
    migrate_for_write(&transaction)?;
    let outcomes = parsed
        .records
        .iter()
        .zip(prepared)
        .map(|(record, (_, hash))| preflight_record(&transaction, record, hash))
        .collect::<Result<Vec<_>, EncounterError>>()?;
    let snapshots = parsed
        .sessions
        .iter()
        .map(|state| build_snapshot(state, parsed, prepared))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    let snapshot_advances = snapshots
        .iter()
        .map(|(snapshot, canonical, hash)| {
            preflight_snapshot(&transaction, snapshot, canonical, hash)
        })
        .collect::<Result<Vec<_>, _>>()?;
    for (snapshot, _, _) in &snapshots {
        ensure_session_order(&transaction, &snapshot.session_id)?;
    }
    for ((record, (canonical, hash)), outcome) in parsed.records.iter().zip(prepared).zip(&outcomes)
    {
        ensure_session_order(&transaction, &record.capture.session_id)?;
        if *outcome == ImportOutcome::Imported {
            insert_record(&transaction, record, canonical, &source_sha256, hash)?;
        }
    }
    for ((snapshot, canonical, hash), advances) in snapshots.iter().zip(snapshot_advances) {
        if advances {
            validate_snapshot_members(&transaction, snapshot)?;
        } else if let Some(latest) = latest_session_snapshot(&transaction, &snapshot.session_id)? {
            validate_snapshot_members(&transaction, &latest)?;
        }
        insert_snapshot_if_absent(&transaction, snapshot, canonical, hash)?;
    }
    if snapshots.is_empty() {
        for record in &parsed.records {
            if let Some(snapshot) =
                latest_session_snapshot(&transaction, &record.capture.session_id)?
            {
                validate_snapshot_members(&transaction, &snapshot)?;
            }
        }
    }
    transaction.commit()?;
    let receipts = parsed
        .records
        .iter()
        .zip(prepared)
        .zip(&outcomes)
        .map(|((record, (_, hash)), outcome)| {
            receipt(record, *outcome, source_sha256.clone(), hash.clone())
        })
        .collect::<Vec<_>>();
    Ok(CaptureImportReport {
        source_sha256,
        imported_count: outcomes
            .iter()
            .filter(|outcome| **outcome == ImportOutcome::Imported)
            .count(),
        already_present_count: outcomes
            .iter()
            .filter(|outcome| **outcome == ImportOutcome::AlreadyPresent)
            .count(),
        receipts,
        last_saved_state: parsed.last_saved_summary(),
    })
}

fn latest_session_snapshot(
    transaction: &rusqlite::Transaction<'_>,
    session_id: &str,
) -> Result<Option<SessionSnapshot>, EncounterError> {
    let bytes = transaction
        .query_row(
            "SELECT canonical_json FROM encounter_session_snapshots
             WHERE session_id = ?1 ORDER BY revision DESC LIMIT 1",
            params![session_id],
            |row| row.get::<_, Vec<u8>>(0),
        )
        .optional()?;
    bytes
        .map(|bytes| {
            serde_json::from_slice(&bytes).map_err(|_| {
                EncounterError::Validation("stored session snapshot is invalid".into())
            })
        })
        .transpose()
}

fn validate_snapshot_members(
    transaction: &rusqlite::Transaction<'_>,
    snapshot: &SessionSnapshot,
) -> Result<(), EncounterError> {
    let mut statement = transaction.prepare(
        "SELECT content_sha256, encounter_id, channel, capture_mode, encounter_ordinal
         FROM raw_encounters WHERE session_id = ?1",
    )?;
    let rows = statement.query_map(params![snapshot.session_id], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, i64>(4)?,
        ))
    })?;
    for row in rows {
        let (content_sha256, encounter_id, channel, mode, ordinal) = row?;
        let reference = snapshot
            .encounters
            .iter()
            .find(|reference| reference.encounter_id == encounter_id);
        if reference.is_none_or(|reference| {
            reference.content_sha256 != content_sha256
                || i64::try_from(reference.ordinal) != Ok(ordinal)
        }) || snapshot.channel.as_str() != channel
            || snapshot.mode.as_str() != mode
        {
            return invalid("encounter store member does not match its session snapshot");
        }
    }
    Ok(())
}

fn preflight_record(
    transaction: &rusqlite::Transaction<'_>,
    record: &OrderedEncounterCapture,
    content_sha256: &str,
) -> Result<ImportOutcome, EncounterError> {
    let capture = &record.capture;
    let existing = transaction
        .query_row(
            "SELECT content_sha256, capture_mode, encounter_ordinal FROM raw_encounters
             WHERE session_id = ?1 AND encounter_id = ?2",
            params![capture.session_id, capture.encounter_id],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, i64>(2)?,
                ))
            },
        )
        .optional()?;
    match existing {
        Some((hash, mode, ordinal))
            if hash == content_sha256
                && mode == record.mode.as_str()
                && u64::try_from(ordinal) == Ok(record.ordinal) =>
        {
            Ok(ImportOutcome::AlreadyPresent)
        }
        Some(_) => invalid("encounter identity already exists with different content"),
        None => {
            let ordinal_collision = transaction
                .query_row(
                    "SELECT content_sha256, encounter_id FROM raw_encounters
                     WHERE session_id = ?1 AND encounter_ordinal = ?2",
                    params![
                        capture.session_id,
                        to_i64(record.ordinal, "encounter ordinal")?
                    ],
                    |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
                )
                .optional()?;
            if ordinal_collision.is_some() {
                return invalid("encounter session ordinal already exists with different content");
            }
            Ok(ImportOutcome::Imported)
        }
    }
}

fn insert_record(
    transaction: &rusqlite::Transaction<'_>,
    record: &OrderedEncounterCapture,
    canonical: &[u8],
    source_sha256: &str,
    content_sha256: &str,
) -> Result<(), EncounterError> {
    let capture = &record.capture;
    transaction.execute(
        "INSERT INTO raw_encounters (
            content_sha256, source_sha256, session_id, encounter_id, channel,
            capture_schema_version, addon_version, canonical_format_version,
            status, started_at, finished_at, first_sequence, last_sequence,
            stored_event_count, omitted_event_count, capture_mode, encounter_ordinal,
            canonical_json
         ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13,
                   ?14, ?15, ?16, ?17, ?18)",
        params![
            content_sha256,
            source_sha256,
            capture.session_id,
            capture.encounter_id,
            capture.channel.as_str(),
            capture.schema_version,
            capture.addon_version,
            canonical_format_for(capture.schema_version)?,
            capture.status.as_str(),
            capture.started_at,
            capture.finished_at,
            to_i64(capture.first_sequence, "first sequence")?,
            to_i64(capture.last_sequence, "last sequence")?,
            i64::try_from(capture.stored_event_count)
                .map_err(|_| EncounterError::Validation("stored count is out of range".into()))?,
            to_i64(capture.omitted_event_count, "omitted count")?,
            record.mode.as_str(),
            to_i64(record.ordinal, "encounter ordinal")?,
            canonical,
        ],
    )?;
    Ok(())
}

fn receipt(
    record: &OrderedEncounterCapture,
    outcome: ImportOutcome,
    source_sha256: String,
    content_sha256: String,
) -> ImportReceipt {
    let capture = &record.capture;
    ImportReceipt {
        outcome,
        source_sha256,
        content_sha256,
        channel: capture.channel,
        status: capture.status,
        session_id: capture.session_id.clone(),
        encounter_id: capture.encounter_id.clone(),
        stored_event_count: capture.stored_event_count,
        omitted_event_count: capture.omitted_event_count,
        capture_mode: record.mode,
        encounter_ordinal: record.ordinal,
    }
}

fn build_snapshot(
    state: &super::model::EncounterModuleState,
    parsed: &ParsedCaptureSet,
    prepared: &[(Vec<u8>, String)],
) -> Result<Option<(SessionSnapshot, Vec<u8>, String)>, EncounterError> {
    let Some(session) = state.session.as_ref() else {
        return Ok(None);
    };
    let encounters = parsed
        .records
        .iter()
        .zip(prepared)
        .filter(|(record, _)| record.capture.session_id == session.session_id)
        .map(|(record, (_, content_sha256))| SessionEncounterReference {
            ordinal: record.ordinal,
            encounter_id: record.capture.encounter_id.clone(),
            content_sha256: content_sha256.clone(),
        })
        .collect();
    let snapshot = SessionSnapshot {
        session_id: session.session_id.clone(),
        revision: state.revision,
        channel: session.channel,
        mode: session.mode,
        disposition: session.status,
        started_at: session.started_at.clone(),
        finished_at: session.finished_at.clone(),
        interruptions: state.interruptions.clone(),
        failure: state.failure.clone(),
        encounters,
    };
    let canonical = serde_json::to_vec(&snapshot)
        .map_err(|_| EncounterError::Validation("session snapshot is not serializable".into()))?;
    if canonical.len() > MAX_SESSION_SNAPSHOT_BYTES {
        return invalid("session snapshot exceeds its byte limit");
    }
    let hash = sha256(&canonical);
    Ok(Some((snapshot, canonical, hash)))
}

fn preflight_snapshot(
    transaction: &rusqlite::Transaction<'_>,
    snapshot: &SessionSnapshot,
    canonical: &[u8],
    content_sha256: &str,
) -> Result<bool, EncounterError> {
    validate_snapshot_facts(snapshot)?;
    let same_revision = transaction
        .query_row(
            "SELECT content_sha256, canonical_json FROM encounter_session_snapshots
             WHERE session_id = ?1 AND revision = ?2",
            params![
                snapshot.session_id,
                to_i64(snapshot.revision, "session revision")?
            ],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, Vec<u8>>(1)?)),
        )
        .optional()?;
    if let Some((stored_hash, stored_bytes)) = same_revision {
        return if stored_hash == content_sha256 && stored_bytes == canonical {
            Ok(false)
        } else {
            invalid("encounter session revision already exists with different content")
        };
    }

    let latest = transaction
        .query_row(
            "SELECT revision, canonical_json FROM encounter_session_snapshots
             WHERE session_id = ?1 ORDER BY revision DESC LIMIT 1",
            params![snapshot.session_id],
            |row| Ok((row.get::<_, i64>(0)?, row.get::<_, Vec<u8>>(1)?)),
        )
        .optional()?;
    let Some((latest_revision, latest_bytes)) = latest else {
        return Ok(true);
    };
    let latest_revision = u64::try_from(latest_revision)
        .map_err(|_| EncounterError::Validation("stored session revision is invalid".into()))?;
    if snapshot.revision <= latest_revision {
        return invalid("encounter session revision is not monotonic");
    }
    let previous: SessionSnapshot = serde_json::from_slice(&latest_bytes)
        .map_err(|_| EncounterError::Validation("stored session snapshot is invalid".into()))?;
    validate_snapshot_extension(&previous, snapshot)?;
    Ok(true)
}

fn validate_snapshot_extension(
    previous: &SessionSnapshot,
    current: &SessionSnapshot,
) -> Result<(), EncounterError> {
    let encounter_prefix = current.encounters.starts_with(&previous.encounters);
    let interruption_prefix = current.interruptions.starts_with(&previous.interruptions);
    let failure_monotonic = previous
        .failure
        .as_ref()
        .is_none_or(|failure| current.failure.as_ref() == Some(failure));
    let resumed = previous.mode == CaptureMode::Continuous
        && previous.disposition == super::model::CaptureSessionStatus::Stopped
        && current
            .interruptions
            .iter()
            .skip(previous.interruptions.len())
            .any(|marker| {
                marker.reason == super::model::CaptureInterruptionReason::UserResumed
                    && marker.prior_finished_at == previous.finished_at
                    && marker.after_encounter_ordinal == previous.encounters.len() as u64
            });
    let finished_monotonic = previous
        .finished_at
        .as_ref()
        .is_none_or(|finished| current.finished_at.as_ref() == Some(finished) || resumed);
    let disposition_monotonic = match previous.disposition {
        super::model::CaptureSessionStatus::Active => true,
        super::model::CaptureSessionStatus::Stopped => {
            resumed || current.disposition == super::model::CaptureSessionStatus::Stopped
        }
        disposition => current.disposition == disposition,
    };
    if previous.session_id != current.session_id
        || previous.channel != current.channel
        || previous.mode != current.mode
        || previous.started_at != current.started_at
        || !encounter_prefix
        || !interruption_prefix
        || !failure_monotonic
        || !finished_monotonic
        || !disposition_monotonic
    {
        return invalid("encounter session snapshot is not an append-only extension");
    }
    Ok(())
}

fn insert_snapshot_if_absent(
    transaction: &rusqlite::Transaction<'_>,
    snapshot: &SessionSnapshot,
    canonical: &[u8],
    content_sha256: &str,
) -> Result<(), EncounterError> {
    transaction.execute(
        "INSERT OR IGNORE INTO encounter_session_snapshots (
            session_id, revision, content_sha256, channel, mode, disposition,
            started_at, finished_at, interruption_count, canonical_json
         ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            snapshot.session_id,
            to_i64(snapshot.revision, "session revision")?,
            content_sha256,
            snapshot.channel.as_str(),
            snapshot.mode.as_str(),
            snapshot.disposition.as_str(),
            snapshot.started_at,
            snapshot.finished_at,
            i64::try_from(snapshot.interruptions.len()).map_err(|_| {
                EncounterError::Validation("session interruption count is out of range".into())
            })?,
            canonical,
        ],
    )?;
    Ok(())
}

pub fn list_encounters(
    store_path: impl AsRef<Path>,
) -> Result<Vec<EncounterSummary>, EncounterError> {
    let connection = open_store(store_path.as_ref(), false, false)?;
    let version: u32 = connection.pragma_query_value(None, "user_version", |row| row.get(0))?;
    let query = if version >= 5 {
        "SELECT content_sha256, session_id, encounter_id, channel, status, started_at,
                finished_at, first_sequence, last_sequence, stored_event_count,
                omitted_event_count, capture_mode, encounter_ordinal
         FROM raw_encounters JOIN encounter_session_order USING (session_id)
         ORDER BY import_ordinal, encounter_ordinal"
    } else if version == 4 {
        "SELECT content_sha256, session_id, encounter_id, channel, status, started_at,
                finished_at, first_sequence, last_sequence, stored_event_count,
                omitted_event_count, capture_mode, encounter_ordinal
         FROM raw_encounters
         ORDER BY session_id, encounter_ordinal"
    } else {
        "SELECT content_sha256, session_id, encounter_id, channel, status, started_at,
                finished_at, first_sequence, last_sequence, stored_event_count,
                omitted_event_count, 'single',
                ROW_NUMBER() OVER (
                    PARTITION BY session_id
                    ORDER BY length(started_at), started_at, encounter_id, content_sha256)
         FROM raw_encounters
         ORDER BY session_id, encounter_ordinal"
    };
    let mut statement = connection.prepare(query)?;
    let rows = statement.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
            row.get::<_, String>(5)?,
            row.get::<_, String>(6)?,
            row.get::<_, i64>(7)?,
            row.get::<_, i64>(8)?,
            row.get::<_, i64>(9)?,
            row.get::<_, i64>(10)?,
            row.get::<_, String>(11)?,
            row.get::<_, i64>(12)?,
        ))
    })?;
    let mut summaries = rows
        .map(|row| {
            let (
                content_sha256,
                session_id,
                encounter_id,
                channel,
                status,
                started_at,
                finished_at,
                first_sequence,
                last_sequence,
                stored_event_count,
                omitted_event_count,
                capture_mode,
                encounter_ordinal,
            ) = row?;
            Ok(EncounterSummary {
                content_sha256,
                session_id,
                encounter_id,
                channel: parse_channel(&channel)?,
                status: parse_status(&status)?,
                started_at,
                finished_at,
                first_sequence: nonnegative_db(first_sequence, "first sequence")?,
                last_sequence: nonnegative_db(last_sequence, "last sequence")?,
                stored_event_count: usize::try_from(nonnegative_db(
                    stored_event_count,
                    "stored event count",
                )?)
                .map_err(|_| {
                    EncounterError::Validation("stored event count is out of range".into())
                })?,
                omitted_event_count: nonnegative_db(omitted_event_count, "omitted event count")?,
                capture_mode: Some(parse_mode(&capture_mode)?),
                source_kind: super::model::EncounterSourceKind::SavedAddon,
                unknown_record_count: 0,
                encounter_ordinal: nonnegative_db(encounter_ordinal, "encounter ordinal")?,
            })
        })
        .collect::<Result<Vec<_>, EncounterError>>()?;
    if version >= 6 {
        summaries.extend(native_summaries(&connection)?);
        let mut statement =
            connection.prepare("SELECT session_id,import_ordinal FROM encounter_session_order")?;
        let order = statement
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
            })?
            .collect::<Result<std::collections::BTreeMap<_, _>, _>>()?;
        summaries.sort_by_key(|summary| {
            (
                order.get(&summary.session_id).copied().unwrap_or(i64::MAX),
                summary.encounter_ordinal,
            )
        });
    }
    Ok(summaries)
}

pub fn load_encounter(
    store_path: impl AsRef<Path>,
    session_id: &str,
    encounter_id: &str,
) -> Result<Option<EncounterCapture>, EncounterError> {
    let connection = open_store(store_path.as_ref(), false, false)?;
    let row = connection
        .query_row(
            "SELECT content_sha256, canonical_json FROM raw_encounters
             WHERE session_id = ?1 AND encounter_id = ?2",
            params![session_id, encounter_id],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, Vec<u8>>(1)?)),
        )
        .optional()?;
    let Some((expected_hash, canonical)) = row else {
        return Ok(None);
    };
    if sha256(&canonical) != expected_hash {
        return invalid("stored encounter content hash does not match its bytes");
    }
    let capture: EncounterCapture = serde_json::from_slice(&canonical).map_err(|_| {
        EncounterError::Validation("stored encounter canonical bytes are invalid".into())
    })?;
    super::validate::validate(&capture)?;
    if capture.session_id != session_id || capture.encounter_id != encounter_id {
        return invalid("stored encounter identity does not match its index");
    }
    Ok(Some(capture))
}

pub fn delete_encounter(
    store_path: impl AsRef<Path>,
    session_id: &str,
    encounter_id: &str,
) -> Result<DeleteReceipt, EncounterError> {
    let mut connection = open_store(store_path.as_ref(), false, true)?;
    let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
    migrate_for_write(&transaction)?;
    ingest::suppress_deleted(&transaction, Some((session_id, encounter_id)))?;
    let mut deleted = transaction.execute(
        "DELETE FROM raw_encounters WHERE session_id = ?1 AND encounter_id = ?2",
        params![session_id, encounter_id],
    )?;
    deleted += transaction.execute(
        "DELETE FROM native_encounters WHERE session_id=?1 AND encounter_id=?2",
        params![session_id, encounter_id],
    )?;
    if deleted > 0 {
        transaction.execute("DELETE FROM native_runs WHERE NOT EXISTS(SELECT 1 FROM native_encounters WHERE native_encounters.session_id=native_runs.session_id)",[])?;
        transaction.commit()?;
    } else {
        transaction.rollback()?;
    }
    Ok(DeleteReceipt {
        deleted_records: deleted,
    })
}

pub fn delete_all(store_path: impl AsRef<Path>) -> Result<DeleteReceipt, EncounterError> {
    let mut connection = open_store(store_path.as_ref(), false, true)?;
    let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
    migrate_for_write(&transaction)?;
    ingest::suppress_deleted(&transaction, None)?;
    let deleted = transaction.execute("DELETE FROM raw_encounters", [])?
        + transaction.execute("DELETE FROM native_encounters", [])?;
    if deleted > 0 {
        transaction.execute("DELETE FROM native_runs", [])?;
    }
    if deleted > 0 {
        transaction.commit()?;
    } else {
        transaction.rollback()?;
    }
    Ok(DeleteReceipt {
        deleted_records: deleted,
    })
}

pub fn backup_store(
    store_path: impl AsRef<Path>,
    destination: impl AsRef<Path>,
) -> Result<BackupReceipt, EncounterError> {
    let store_path = store_path.as_ref();
    let destination = destination.as_ref();
    ensure_distinct_paths(store_path, destination)?;
    reject_link_like_file(destination)?;
    let connection = open_store(store_path, false, false)?;
    let schema_version: u32 =
        connection.pragma_query_value(None, "user_version", |row| row.get(0))?;
    let parent = destination
        .parent()
        .filter(|value| !value.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;
    let temporary = tempfile::Builder::new()
        .prefix(".encounter-backup-")
        .tempfile_in(parent)?
        .into_temp_path();
    connection.backup("main", &temporary, None)?;
    let candidate = open_store(&temporary, false, false)?;
    validate_store_contents(&candidate)?;
    drop(candidate);
    File::options().write(true).open(&temporary)?.sync_all()?;
    let (byte_length, digest) = hash_file(&temporary)?;
    crate::atomic_file::persist(temporary, destination)?;
    Ok(BackupReceipt {
        schema_version,
        byte_length,
        sha256: digest,
    })
}

fn open_store(path: &Path, allow_create: bool, write: bool) -> Result<Connection, EncounterError> {
    let exists = match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.is_file() && !is_link_like(&metadata) => true,
        Ok(_) => return invalid("encounter store must be a regular non-link file"),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
        Err(error) => return Err(error.into()),
    };
    if !exists && !allow_create {
        return invalid("encounter store does not exist");
    }
    if allow_create {
        if let Some(parent) = path.parent().filter(|value| !value.as_os_str().is_empty()) {
            fs::create_dir_all(parent)?;
        }
    }
    let mut flags = if write {
        OpenFlags::SQLITE_OPEN_READ_WRITE
    } else {
        OpenFlags::SQLITE_OPEN_READ_ONLY
    };
    if allow_create {
        flags |= OpenFlags::SQLITE_OPEN_CREATE;
    }
    let connection = Connection::open_with_flags(path, flags)?;
    connection.busy_timeout(std::time::Duration::from_secs(5))?;
    validate_integrity(&connection)?;
    let version: u32 = connection.pragma_query_value(None, "user_version", |row| row.get(0))?;
    if version == 0 {
        if !allow_create || !schema_is_empty(&connection)? {
            return invalid("encounter store has an unrecognized schema");
        }
        let native_schema = native_schema_sql();
        let ingest_schema = ingest::schema_sql();
        let schema = format!(
            "{META_TABLE_SQL};\n{META_INSERT_SQL};\n{RAW_TABLE_SQL};\n{SESSION_TABLE_SQL};\n{IMMUTABILITY_TRIGGER_SQL};\n{SESSION_IMMUTABILITY_TRIGGER_SQL};\n{ORDER_TABLE_SQL};\n{ORDER_IMMUTABILITY_TRIGGER_SQL};\n{native_schema}\n{ingest_schema}\nPRAGMA user_version = 7;"
        );
        connection.execute_batch("BEGIN IMMEDIATE;")?;
        if let Err(error) = connection
            .execute_batch(&schema)
            .and_then(|_| connection.execute_batch("COMMIT;"))
        {
            let _ = connection.execute_batch("ROLLBACK;");
            return Err(error.into());
        }
        validate_schema(&connection, STORE_SCHEMA_VERSION)?;
        validate_records(&connection, STORE_SCHEMA_VERSION)?;
    } else if version == 1 {
        validate_schema(&connection, 1)?;
        validate_records(&connection, 1)?;
    } else if version == 2 {
        validate_schema(&connection, 2)?;
        validate_records(&connection, 2)?;
    } else if version == 3 {
        validate_schema(&connection, 3)?;
        validate_records(&connection, 3)?;
    } else if version == 4 {
        validate_schema(&connection, 4)?;
        validate_records(&connection, 4)?;
    } else if version == 5 {
        validate_schema(&connection, 5)?;
        validate_records(&connection, 5)?;
    } else if version == 6 {
        validate_schema(&connection, 6)?;
        validate_records(&connection, 6)?;
    } else if version == STORE_SCHEMA_VERSION {
        validate_schema(&connection, STORE_SCHEMA_VERSION)?;
        validate_records(&connection, STORE_SCHEMA_VERSION)?;
    } else {
        return invalid(format!("unsupported encounter store schema {version}"));
    }
    Ok(connection)
}

fn validate_store_contents(connection: &Connection) -> Result<(), EncounterError> {
    let version: u32 = connection.pragma_query_value(None, "user_version", |row| row.get(0))?;
    if !matches!(version, 1 | 2 | 3 | 4 | 5 | 6 | STORE_SCHEMA_VERSION) {
        return invalid(format!("unsupported encounter store schema {version}"));
    }
    validate_schema(connection, version)?;
    validate_records(connection, version)
}

fn migrate_for_write(transaction: &rusqlite::Transaction<'_>) -> Result<(), EncounterError> {
    let version: u32 = transaction.pragma_query_value(None, "user_version", |row| row.get(0))?;
    match version {
        1 => migrate_v1_to_current(transaction)?,
        2 => migrate_v2_to_current(transaction)?,
        3 => migrate_v3_to_current(transaction)?,
        4 => migrate_v4_to_current(transaction)?,
        5 => transaction.execute_batch(&format!("DROP TABLE encounter_store_meta; {META_TABLE_SQL}; {META_INSERT_SQL}; PRAGMA user_version=7;"))?,
        6 => {
            transaction.execute_batch(&format!(
                "DROP TRIGGER native_encounters_no_update;
                 DROP TRIGGER native_runs_no_update;
                 ALTER TABLE native_encounters RENAME TO native_encounters_v6;
                 ALTER TABLE native_runs RENAME TO native_runs_v6;
                 DROP TABLE encounter_store_meta;
                 {META_TABLE_SQL}; {META_INSERT_SQL};
                 {NATIVE_RUN_TABLE_SQL}; {NATIVE_TABLE_SQL};
                 INSERT INTO native_runs SELECT * FROM native_runs_v6;
                 INSERT INTO native_encounters SELECT * FROM native_encounters_v6;
                 DROP TABLE native_encounters_v6; DROP TABLE native_runs_v6;
                 {NATIVE_RUN_TRIGGER_SQL}; {NATIVE_TRIGGER_SQL};
                 PRAGMA user_version=7;"
            ))?;
        }
        STORE_SCHEMA_VERSION => return Ok(()),
        _ => return invalid("unsupported encounter store schema"),
    }
    if version < 6 {
        transaction.execute_batch(&native_schema_sql())?;
    }
    transaction.execute_batch(&ingest::schema_sql())?;
    ingest::backfill_aliases(transaction)?;
    validate_schema(transaction, STORE_SCHEMA_VERSION)?;
    validate_records(transaction, STORE_SCHEMA_VERSION)?;
    Ok(())
}

fn migrate_v1_to_current(transaction: &rusqlite::Transaction<'_>) -> Result<(), EncounterError> {
    let migration = format!(
        "DROP TRIGGER raw_encounters_no_update;
         ALTER TABLE raw_encounters RENAME TO raw_encounters_v1;
         DROP TABLE encounter_store_meta;
         {META_TABLE_SQL};
         {META_INSERT_SQL};
         {RAW_TABLE_SQL};
         {SESSION_TABLE_SQL};
         INSERT INTO raw_encounters (
             content_sha256, source_sha256, session_id, encounter_id, channel,
             capture_schema_version, addon_version, canonical_format_version,
             status, started_at, finished_at, first_sequence, last_sequence,
             stored_event_count, omitted_event_count, capture_mode,
             encounter_ordinal, canonical_json)
         SELECT content_sha256, source_sha256, session_id, encounter_id, channel,
             capture_schema_version, addon_version, 1, status, started_at,
             finished_at, first_sequence, last_sequence, stored_event_count,
             omitted_event_count, 'single',
             ROW_NUMBER() OVER (
                 PARTITION BY session_id
                 ORDER BY length(started_at), started_at, encounter_id, content_sha256),
             canonical_json
         FROM raw_encounters_v1;
         DROP TABLE raw_encounters_v1;
         {IMMUTABILITY_TRIGGER_SQL};
         {SESSION_IMMUTABILITY_TRIGGER_SQL};
         {ORDER_TABLE_SQL};
         {ORDER_BACKFILL_SQL};
         {ORDER_IMMUTABILITY_TRIGGER_SQL};
         PRAGMA user_version = 7;
"
    );
    transaction.execute_batch(&migration)?;
    Ok(())
}

fn migrate_v2_to_current(transaction: &rusqlite::Transaction<'_>) -> Result<(), EncounterError> {
    let migration = format!(
        "DROP TRIGGER raw_encounters_no_update;
         ALTER TABLE raw_encounters RENAME TO raw_encounters_v2;
         DROP TABLE encounter_store_meta;
         {META_TABLE_SQL};
         {META_INSERT_SQL};
         {RAW_TABLE_SQL};
         {SESSION_TABLE_SQL};
         INSERT INTO raw_encounters (
             content_sha256, source_sha256, session_id, encounter_id, channel,
             capture_schema_version, addon_version, canonical_format_version,
             status, started_at, finished_at, first_sequence, last_sequence,
             stored_event_count, omitted_event_count, capture_mode,
             encounter_ordinal, canonical_json)
         SELECT content_sha256, source_sha256, session_id, encounter_id, channel,
             capture_schema_version, addon_version, canonical_format_version,
             status, started_at, finished_at, first_sequence, last_sequence,
             stored_event_count, omitted_event_count, 'single',
             ROW_NUMBER() OVER (
                 PARTITION BY session_id
                 ORDER BY length(started_at), started_at, encounter_id, content_sha256),
             canonical_json
         FROM raw_encounters_v2;
         DROP TABLE raw_encounters_v2;
         {IMMUTABILITY_TRIGGER_SQL};
         {SESSION_IMMUTABILITY_TRIGGER_SQL};
         {ORDER_TABLE_SQL};
         {ORDER_BACKFILL_SQL};
         {ORDER_IMMUTABILITY_TRIGGER_SQL};
         PRAGMA user_version = 7;
"
    );
    transaction.execute_batch(&migration)?;
    Ok(())
}

fn migrate_v3_to_current(transaction: &rusqlite::Transaction<'_>) -> Result<(), EncounterError> {
    let migration = format!(
        "DROP TRIGGER raw_encounters_no_update;
         ALTER TABLE raw_encounters RENAME TO raw_encounters_v3;
         DROP TABLE encounter_store_meta;
         {META_TABLE_SQL};
         {META_INSERT_SQL};
         {RAW_TABLE_SQL};
         {SESSION_TABLE_SQL};
         INSERT INTO raw_encounters (
             content_sha256, source_sha256, session_id, encounter_id, channel,
             capture_schema_version, addon_version, canonical_format_version,
             status, started_at, finished_at, first_sequence, last_sequence,
             stored_event_count, omitted_event_count, capture_mode,
             encounter_ordinal, canonical_json)
         SELECT content_sha256, source_sha256, session_id, encounter_id, channel,
             capture_schema_version, addon_version, canonical_format_version,
             status, started_at, finished_at, first_sequence, last_sequence,
             stored_event_count, omitted_event_count, 'single',
             ROW_NUMBER() OVER (
                 PARTITION BY session_id
                 ORDER BY length(started_at), started_at, encounter_id, content_sha256),
             canonical_json
         FROM raw_encounters_v3;
         DROP TABLE raw_encounters_v3;
         {IMMUTABILITY_TRIGGER_SQL};
         {SESSION_IMMUTABILITY_TRIGGER_SQL};
         {ORDER_TABLE_SQL};
         {ORDER_BACKFILL_SQL};
         {ORDER_IMMUTABILITY_TRIGGER_SQL};
         PRAGMA user_version = 7;
"
    );
    transaction.execute_batch(&migration)?;
    Ok(())
}

fn migrate_v4_to_current(transaction: &rusqlite::Transaction<'_>) -> Result<(), EncounterError> {
    let migration = format!(
        "DROP TABLE encounter_store_meta;
         {META_TABLE_SQL};
         {META_INSERT_SQL};
         {ORDER_TABLE_SQL};
         {ORDER_BACKFILL_SQL};
         {ORDER_IMMUTABILITY_TRIGGER_SQL};
         PRAGMA user_version = 7;
"
    );
    transaction.execute_batch(&migration)?;
    Ok(())
}

fn ensure_session_order(
    transaction: &rusqlite::Transaction<'_>,
    id: &str,
) -> Result<(), EncounterError> {
    let exists: bool = transaction.query_row(
        "SELECT EXISTS(SELECT 1 FROM encounter_session_order WHERE session_id = ?1)",
        [id],
        |row| row.get(0),
    )?;
    if !exists {
        let last: i64 = transaction.query_row(
            "SELECT COALESCE(MAX(import_ordinal), 0) FROM encounter_session_order",
            [],
            |row| row.get(0),
        )?;
        let next = last
            .checked_add(1)
            .ok_or_else(|| EncounterError::Validation("session order exhausted".into()))?;
        transaction.execute(
            "INSERT INTO encounter_session_order(session_id, import_ordinal) VALUES (?1, ?2)",
            params![id, next],
        )?;
    }
    Ok(())
}

fn validate_session_order(connection: &Connection) -> Result<(), EncounterError> {
    let version: u32 = connection.pragma_query_value(None, "user_version", |row| row.get(0))?;
    let order_query = if version >= 6 {
        "SELECT EXISTS(SELECT 1 FROM (SELECT session_id FROM raw_encounters UNION SELECT session_id FROM encounter_session_snapshots UNION SELECT session_id FROM native_runs) AS sessions WHERE NOT EXISTS(SELECT 1 FROM encounter_session_order AS ordering WHERE ordering.session_id=sessions.session_id))"
    } else {
        "SELECT EXISTS(SELECT 1 FROM
          (SELECT session_id FROM raw_encounters UNION SELECT session_id FROM encounter_session_snapshots) AS sessions
          WHERE NOT EXISTS(SELECT 1 FROM encounter_session_order AS ordering WHERE ordering.session_id = sessions.session_id))"
    };
    let missing: bool = connection.query_row(order_query, [], |row| row.get(0))?;
    if missing {
        return invalid("encounter session ordering is incomplete");
    }
    let mut statement =
        connection.prepare("SELECT session_id, import_ordinal FROM encounter_session_order")?;
    for row in statement.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
    })? {
        let (id, ordinal) = row?;
        if ordinal <= 0
            || id.is_empty()
            || id.len() > 128
            || !id
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"._:-".contains(&b))
        {
            return invalid("encounter session ordering is invalid");
        }
    }
    Ok(())
}
fn validate_integrity(connection: &Connection) -> Result<(), EncounterError> {
    let result: String = connection.query_row("PRAGMA quick_check(1)", [], |row| row.get(0))?;
    if result != "ok" {
        return invalid("encounter store failed SQLite integrity checking");
    }
    Ok(())
}

fn schema_is_empty(connection: &Connection) -> Result<bool, EncounterError> {
    let count: i64 = connection.query_row(
        "SELECT count(*) FROM sqlite_schema WHERE name NOT LIKE 'sqlite_%'",
        [],
        |row| row.get(0),
    )?;
    Ok(count == 0)
}

fn validate_schema(connection: &Connection, version: u32) -> Result<(), EncounterError> {
    let mut statement = connection.prepare(
        "SELECT type, name, tbl_name, sql
         FROM sqlite_schema
         WHERE name NOT LIKE 'sqlite_%'
         ORDER BY type, name",
    )?;
    let actual = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    let (meta_sql, raw_sql) = match version {
        1 => (V1_META_TABLE_SQL, V1_RAW_TABLE_SQL),
        2 => (V2_META_TABLE_SQL, V2_RAW_TABLE_SQL),
        3 => (V3_META_TABLE_SQL, V3_RAW_TABLE_SQL),
        4 => (V4_META_TABLE_SQL, RAW_TABLE_SQL),
        5 => (V5_META_TABLE_SQL, RAW_TABLE_SQL),
        6 => (V6_META_TABLE_SQL, RAW_TABLE_SQL),
        7 => (META_TABLE_SQL, RAW_TABLE_SQL),
        _ => return invalid("unsupported encounter store schema"),
    };
    let mut expected = vec![
        (
            "table",
            "encounter_store_meta",
            "encounter_store_meta",
            meta_sql,
        ),
        ("table", "raw_encounters", "raw_encounters", raw_sql),
        (
            "trigger",
            "raw_encounters_no_update",
            "raw_encounters",
            IMMUTABILITY_TRIGGER_SQL,
        ),
    ];
    if version >= 4 {
        expected.insert(
            1,
            (
                "table",
                "encounter_session_snapshots",
                "encounter_session_snapshots",
                SESSION_TABLE_SQL,
            ),
        );
        expected.push((
            "trigger",
            "encounter_session_snapshots_no_update",
            "encounter_session_snapshots",
            SESSION_IMMUTABILITY_TRIGGER_SQL,
        ));
        expected.sort_by_key(|entry| (entry.0, entry.1));
    }
    if version >= 5 {
        expected.push((
            "table",
            "encounter_session_order",
            "encounter_session_order",
            ORDER_TABLE_SQL,
        ));
        expected.push((
            "trigger",
            "encounter_session_order_no_update",
            "encounter_session_order",
            ORDER_IMMUTABILITY_TRIGGER_SQL,
        ));
        expected.sort_by_key(|entry| (entry.0, entry.1));
    }
    if version >= 6 {
        let (run_sql, encounter_sql) = if version == 6 {
            (V6_NATIVE_RUN_TABLE_SQL, V6_NATIVE_TABLE_SQL)
        } else {
            (NATIVE_RUN_TABLE_SQL, NATIVE_TABLE_SQL)
        };
        expected.extend([
            ("table", "native_runs", "native_runs", run_sql),
            (
                "table",
                "native_encounters",
                "native_encounters",
                encounter_sql,
            ),
            (
                "trigger",
                "native_runs_no_update",
                "native_runs",
                NATIVE_RUN_TRIGGER_SQL,
            ),
            (
                "trigger",
                "native_encounters_no_update",
                "native_encounters",
                NATIVE_TRIGGER_SQL,
            ),
        ]);
        expected.sort_by_key(|entry| (entry.0, entry.1));
    }
    if version >= 7 {
        expected.extend(ingest::schema_objects());
        expected.sort_by_key(|entry| (entry.0, entry.1));
    }
    let schema_matches = actual.len() == expected.len()
        && actual.iter().zip(&expected).all(|(actual, expected)| {
            actual.0 == expected.0
                && actual.1 == expected.1
                && actual.2 == expected.2
                && normalize_sql(&actual.3) == normalize_sql(expected.3)
        });
    if !schema_matches {
        return invalid(format!(
            "encounter store schema objects do not match schema version {version}"
        ));
    }

    let meta: Option<(u32, u32)> = connection
        .query_row(
            "SELECT schema_version, canonical_format_version FROM encounter_store_meta WHERE singleton = 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()?;
    let expected_meta = match version {
        1 => (1, 1),
        2 => (2, 2),
        3 => (3, CANONICAL_FORMAT_VERSION),
        4 => (4, CANONICAL_FORMAT_VERSION),
        5 => (5, CANONICAL_FORMAT_VERSION),
        6 => (6, CANONICAL_FORMAT_VERSION),
        7 => (STORE_SCHEMA_VERSION, CANONICAL_FORMAT_VERSION),
        _ => unreachable!(),
    };
    if meta != Some(expected_meta) {
        return invalid("encounter store metadata does not match its schema");
    }
    Ok(())
}

fn normalize_sql(sql: &str) -> String {
    sql.split_ascii_whitespace().collect::<Vec<_>>().join(" ")
}

fn validate_records(connection: &Connection, version: u32) -> Result<(), EncounterError> {
    let query = if version == 1 {
        "SELECT content_sha256, source_sha256, session_id, encounter_id, channel,
                capture_schema_version, addon_version, 1, status, started_at, finished_at,
                first_sequence, last_sequence, stored_event_count, omitted_event_count,
                'single', 1, canonical_json
         FROM raw_encounters"
    } else if version < 4 {
        "SELECT content_sha256, source_sha256, session_id, encounter_id, channel,
                capture_schema_version, addon_version, canonical_format_version,
                status, started_at, finished_at, first_sequence, last_sequence,
                stored_event_count, omitted_event_count, 'single', 1, canonical_json
         FROM raw_encounters"
    } else {
        "SELECT content_sha256, source_sha256, session_id, encounter_id, channel,
                capture_schema_version, addon_version, canonical_format_version,
                status, started_at, finished_at, first_sequence, last_sequence,
                stored_event_count, omitted_event_count, capture_mode,
                encounter_ordinal, canonical_json
         FROM raw_encounters"
    };
    let mut statement = connection.prepare(query)?;
    let rows = statement.query_map([], |row| {
        Ok(StoredRecord {
            content_sha256: row.get(0)?,
            source_sha256: row.get(1)?,
            session_id: row.get(2)?,
            encounter_id: row.get(3)?,
            channel: row.get(4)?,
            capture_schema_version: row.get(5)?,
            addon_version: row.get(6)?,
            canonical_format_version: row.get(7)?,
            status: row.get(8)?,
            started_at: row.get(9)?,
            finished_at: row.get(10)?,
            first_sequence: row.get(11)?,
            last_sequence: row.get(12)?,
            stored_event_count: row.get(13)?,
            omitted_event_count: row.get(14)?,
            capture_mode: row.get(15)?,
            encounter_ordinal: row.get(16)?,
            canonical_json: row.get(17)?,
        })
    })?;
    let mut session_members = Vec::new();
    for row in rows {
        let stored = row?;
        if sha256(&stored.canonical_json) != stored.content_sha256 {
            return invalid("encounter store contains a raw content hash mismatch");
        }
        if !is_sha256(&stored.source_sha256) {
            return invalid("encounter store contains an invalid source hash");
        }
        let capture: EncounterCapture =
            serde_json::from_slice(&stored.canonical_json).map_err(|_| {
                EncounterError::Validation("stored canonical encounter is invalid".into())
            })?;
        super::validate::validate(&capture)?;
        if super::canonical_bytes(&capture)? != stored.canonical_json {
            return invalid("encounter store contains noncanonical encounter bytes");
        }
        let indexed_fields_match = stored.session_id == capture.session_id
            && stored.encounter_id == capture.encounter_id
            && stored.channel == capture.channel.as_str()
            && stored.capture_schema_version == i64::from(capture.schema_version)
            && stored.addon_version == i64::from(capture.addon_version)
            && stored.canonical_format_version
                == i64::from(canonical_format_for(capture.schema_version)?)
            && stored.status == capture.status.as_str()
            && stored.started_at == capture.started_at
            && stored.finished_at == capture.finished_at
            && u64::try_from(stored.first_sequence) == Ok(capture.first_sequence)
            && u64::try_from(stored.last_sequence) == Ok(capture.last_sequence)
            && usize::try_from(stored.stored_event_count) == Ok(capture.stored_event_count)
            && u64::try_from(stored.omitted_event_count) == Ok(capture.omitted_event_count)
            && parse_mode(&stored.capture_mode).is_ok()
            && stored.encounter_ordinal > 0;
        if !indexed_fields_match {
            return invalid("encounter store index fields do not match canonical content");
        }
        if version >= 4 {
            session_members.push(StoredSessionMember {
                content_sha256: stored.content_sha256,
                session_id: stored.session_id,
                encounter_id: stored.encounter_id,
                channel: stored.channel,
                capture_mode: stored.capture_mode,
                encounter_ordinal: stored.encounter_ordinal,
            });
        }
    }
    if version >= 4 {
        validate_session_snapshots(connection, &session_members)?;
    }
    if version >= 5 {
        if version >= 6 {
            validate_native_records(connection)?;
        }
        if version >= 7 {
            ingest::validate_contents(connection)?;
        }
        validate_session_order(connection)?;
    }
    Ok(())
}

fn validate_session_snapshots(
    connection: &Connection,
    stored_members: &[StoredSessionMember],
) -> Result<(), EncounterError> {
    let mut statement = connection.prepare(
        "SELECT session_id, revision, content_sha256, channel, mode, disposition,
                started_at, finished_at, interruption_count, length(canonical_json),
                CASE WHEN length(canonical_json) <= 1048576 THEN canonical_json END
         FROM encounter_session_snapshots ORDER BY session_id, revision",
    )?;
    let rows = statement.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, i64>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
            row.get::<_, String>(5)?,
            row.get::<_, String>(6)?,
            row.get::<_, Option<String>>(7)?,
            row.get::<_, i64>(8)?,
            row.get::<_, i64>(9)?,
            row.get::<_, Option<Vec<u8>>>(10)?,
        ))
    })?;
    let mut previous: Option<SessionSnapshot> = None;
    let mut latest_snapshots = std::collections::HashMap::new();
    for row in rows {
        let (
            session_id,
            revision,
            content_sha256,
            channel,
            mode,
            disposition,
            started_at,
            finished_at,
            interruption_count,
            canonical_length,
            canonical,
        ) = row?;
        let canonical_length = usize::try_from(canonical_length).map_err(|_| {
            EncounterError::Validation("stored session snapshot byte length is invalid".into())
        })?;
        if canonical_length > MAX_SESSION_SNAPSHOT_BYTES {
            return invalid("stored session snapshot exceeds its byte limit");
        }
        let canonical = canonical.ok_or_else(|| {
            EncounterError::Validation("stored session snapshot exceeds its byte limit".into())
        })?;
        if sha256(&canonical) != content_sha256 {
            return invalid("encounter store contains a session snapshot hash mismatch");
        }
        let snapshot: SessionSnapshot = serde_json::from_slice(&canonical)
            .map_err(|_| EncounterError::Validation("stored session snapshot is invalid".into()))?;
        validate_snapshot_facts(&snapshot)?;
        if serde_json::to_vec(&snapshot)
            .map_err(|_| EncounterError::Validation("stored session snapshot is invalid".into()))?
            != canonical
            || snapshot.session_id != session_id
            || u64::try_from(revision) != Ok(snapshot.revision)
            || snapshot.channel.as_str() != channel
            || snapshot.mode.as_str() != mode
            || snapshot.disposition.as_str() != disposition
            || snapshot.started_at != started_at
            || snapshot.finished_at != finished_at
            || usize::try_from(interruption_count) != Ok(snapshot.interruptions.len())
        {
            return invalid("encounter store session snapshot index does not match its content");
        }
        if let Some(prior) = previous
            .as_ref()
            .filter(|prior| prior.session_id == session_id)
        {
            if snapshot.revision <= prior.revision {
                return invalid("encounter store session revisions are not monotonic");
            }
            validate_snapshot_extension(prior, &snapshot)?;
        }
        latest_snapshots.insert(snapshot.session_id.clone(), snapshot.clone());
        previous = Some(snapshot);
    }
    for member in stored_members {
        let Some(snapshot) = latest_snapshots.get(&member.session_id) else {
            continue;
        };
        let reference = snapshot
            .encounters
            .iter()
            .find(|reference| reference.encounter_id == member.encounter_id);
        if reference.is_none_or(|reference| {
            reference.content_sha256 != member.content_sha256
                || i64::try_from(reference.ordinal) != Ok(member.encounter_ordinal)
        }) || snapshot.channel.as_str() != member.channel
            || snapshot.mode.as_str() != member.capture_mode
        {
            return invalid("encounter store member does not match its session snapshot");
        }
    }
    Ok(())
}

fn validate_snapshot_facts(snapshot: &SessionSnapshot) -> Result<(), EncounterError> {
    super::validate::validate_opaque_id(&snapshot.session_id, "session")?;
    let started_at = super::validate::validate_decimal_time(&snapshot.started_at)?;
    let finished_at = snapshot
        .finished_at
        .as_deref()
        .map(super::validate::validate_decimal_time)
        .transpose()?;
    if snapshot.revision == 0
        || snapshot.encounters.len() > super::MAX_SESSION_ENCOUNTERS
        || snapshot.interruptions.len() > super::MAX_SESSION_INTERRUPTION_MARKERS
        || finished_at.is_some_and(|finished| finished < started_at)
        || (snapshot.disposition == super::model::CaptureSessionStatus::Active)
            != snapshot.finished_at.is_none()
        || (snapshot.disposition == super::model::CaptureSessionStatus::Failed)
            != snapshot.failure.is_some()
    {
        return invalid("stored session snapshot facts are invalid");
    }
    for (index, encounter) in snapshot.encounters.iter().enumerate() {
        if encounter.ordinal != (index + 1) as u64 || !is_sha256(&encounter.content_sha256) {
            return invalid("stored session encounter references are invalid");
        }
        super::validate::validate_opaque_id(&encounter.encounter_id, "encounter")?;
    }
    for (index, interruption) in snapshot.interruptions.iter().enumerate() {
        if interruption.sequence != (index + 1) as u64
            || interruption.after_encounter_ordinal > snapshot.encounters.len() as u64
        {
            return invalid("stored session interruption references are invalid");
        }
        super::validate::validate_interruption_marker(interruption, snapshot.mode)?;
    }
    if let Some(failure) = snapshot.failure.as_ref() {
        super::validate::validate_decimal_time(&failure.occurred_at)?;
        if failure
            .encounter_ordinal
            .is_some_and(|ordinal| ordinal == 0 || ordinal > snapshot.encounters.len() as u64 + 1)
        {
            return invalid("stored session failure reference is invalid");
        }
    }
    Ok(())
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn parse_channel(value: &str) -> Result<Channel, EncounterError> {
    match value {
        "live" => Ok(Channel::Live),
        "pts" => Ok(Channel::Pts),
        _ => invalid("encounter store contains an invalid channel"),
    }
}

fn parse_status(value: &str) -> Result<CaptureStatus, EncounterError> {
    match value {
        "complete" => Ok(CaptureStatus::Complete),
        "partial" => Ok(CaptureStatus::Partial),
        _ => invalid("encounter store contains an invalid status"),
    }
}

fn parse_mode(value: &str) -> Result<CaptureMode, EncounterError> {
    match value {
        "single" => Ok(CaptureMode::Single),
        "continuous" => Ok(CaptureMode::Continuous),
        _ => invalid("encounter store contains an invalid capture mode"),
    }
}

fn canonical_format_for(capture_schema_version: u32) -> Result<u32, EncounterError> {
    match capture_schema_version {
        1 => Ok(1),
        2 => Ok(CANONICAL_FORMAT_VERSION),
        _ => invalid("unsupported encounter capture version in store"),
    }
}

fn nonnegative_db(value: i64, label: &str) -> Result<u64, EncounterError> {
    u64::try_from(value)
        .map_err(|_| EncounterError::Validation(format!("stored {label} is negative")))
}

fn to_i64(value: u64, label: &str) -> Result<i64, EncounterError> {
    i64::try_from(value).map_err(|_| EncounterError::Validation(format!("{label} is out of range")))
}

fn hash_file(path: &Path) -> Result<(u64, String), EncounterError> {
    use sha2::{Digest, Sha256};

    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut length = 0_u64;
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        length = length
            .checked_add(read as u64)
            .ok_or_else(|| EncounterError::Validation("backup length overflow".into()))?;
        hasher.update(&buffer[..read]);
    }
    Ok((length, crate::hash::hex_lower(hasher.finalize())))
}

fn reject_link_like_file(path: &Path) -> Result<(), EncounterError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if !metadata.is_file() || is_link_like(&metadata) => {
            invalid("encounter backup destination must be a regular non-link file")
        }
        Ok(_) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.into()),
    }
}

pub(crate) fn append_native_set(
    store_path: &Path,
    parsed: &super::native::ParsedNativeLog,
    prepared: &[(Vec<u8>, String)],
    source_sha256: String,
) -> Result<super::native::NativeImportReport, EncounterError> {
    use super::native::{canonical_native_run_bytes, NativeImportReceipt, NativeImportReport};
    if parsed.encounters.len() != prepared.len() || !is_sha256(&source_sha256) {
        return invalid("native prepared import is invalid");
    }
    let mut connection = open_store(store_path, true, true)?;
    let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
    migrate_for_write(&transaction)?;
    let mut receipts = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    let mut logical_seen =
        std::collections::BTreeMap::<String, (String, NativeImportReceipt)>::new();
    let mut runs = std::collections::BTreeMap::new();
    for (capture, (canonical, hash)) in parsed.encounters.iter().zip(prepared) {
        if !seen.insert((&capture.run.session_id, &capture.encounter_id))
            || super::native::canonical_native_bytes(capture)? != *canonical
            || sha256(canonical) != *hash
        {
            return invalid("native prepared encounter is invalid");
        }
        let logical = super::native::logical_native_identity(capture)?;
        let receipt = if let Some((fingerprint, prior)) = logical_seen.get(&logical.fight_key) {
            if *fingerprint != logical.evidence_sha256 {
                return invalid("native logical encounter identity collision");
            }
            NativeImportReceipt {
                outcome: ImportOutcome::AlreadyPresent,
                ..prior.clone()
            }
        } else {
            match ingest::logical_disposition(&transaction, &logical)? {
                ingest::LogicalDisposition::Suppressed => return invalid(
                    "native encounter was deliberately deleted; compatibility import suppressed",
                ),
                ingest::LogicalDisposition::Present {
                    session_id,
                    encounter_id,
                    content_sha256,
                } => NativeImportReceipt {
                    outcome: ImportOutcome::AlreadyPresent,
                    session_id,
                    encounter_id,
                    content_sha256,
                },
                ingest::LogicalDisposition::Missing => {
                    let existing: Option<(String,Vec<u8>)> = transaction.query_row(
            "SELECT content_sha256,canonical_json FROM native_encounters WHERE session_id=?1 AND encounter_id=?2",
            params![capture.run.session_id,capture.encounter_id],
            |row|Ok((row.get(0)?,row.get(1)?))
        ).optional()?;
                    let outcome = match existing {
                        Some((existing_hash, existing_bytes))
                            if existing_hash == *hash && existing_bytes == *canonical =>
                        {
                            ImportOutcome::AlreadyPresent
                        }
                        Some(_) => return invalid("native encounter identity collision"),
                        None => ImportOutcome::Imported,
                    };
                    NativeImportReceipt {
                        outcome,
                        session_id: capture.run.session_id.clone(),
                        encounter_id: capture.encounter_id.clone(),
                        content_sha256: hash.clone(),
                    }
                }
            }
        };
        if receipt.outcome == ImportOutcome::Imported {
            let run_bytes = canonical_native_run_bytes(&capture.run)?;
            if let Some(existing) = runs.insert(
                capture.run.session_id.clone(),
                (capture.run.clone(), run_bytes.clone()),
            ) {
                if existing.1 != run_bytes {
                    return invalid("native run identity collision");
                }
            }
        }
        logical_seen.insert(
            logical.fight_key,
            (logical.evidence_sha256, receipt.clone()),
        );
        receipts.push(receipt);
    }
    for (id, (_, canonical)) in &runs {
        let existing: Option<Vec<u8>> = transaction
            .query_row(
                "SELECT canonical_json FROM native_runs WHERE session_id=?1",
                [id],
                |row| row.get(0),
            )
            .optional()?;
        if existing.is_some_and(|bytes| bytes != *canonical) {
            return invalid("native run identity collision");
        }
    }
    // Allocation follows file order, not hash-key order.
    for (capture, receipt) in parsed.encounters.iter().zip(&receipts) {
        if receipt.outcome == ImportOutcome::Imported {
            ensure_session_order(&transaction, &capture.run.session_id)?;
        }
    }
    for (run, canonical) in runs.values() {
        transaction.execute(
            "INSERT OR IGNORE INTO native_runs VALUES (?1,?2,?3,1,?4,?5)",
            params![
                run.session_id,
                sha256(canonical),
                run.channel.as_str(),
                to_db(run.run_ordinal)?,
                canonical
            ],
        )?;
    }
    for ((capture, (canonical, hash)), receipt) in
        parsed.encounters.iter().zip(prepared).zip(&receipts)
    {
        if receipt.outcome == ImportOutcome::Imported {
            let (started, finished) = native_times(capture)?;
            transaction.execute(
                "INSERT INTO native_encounters VALUES (?1,?2,?3,?4,?5,1,?6,?7,?8,?9,?10,?11,?12,?13)",
                params![hash,source_sha256,capture.run.session_id,capture.encounter_id,
                    capture.run.channel.as_str(),to_db(capture.encounter_ordinal)?,started,finished,
                    to_db(capture.first_ordinal)?,to_db(capture.last_ordinal)?,
                    to_db(capture.records.len() as u64)?,
                    to_db((capture.unknown_record_count+capture.unknown_context_count) as u64)?,canonical]
            )?;
        }
        if receipt.session_id == capture.run.session_id
            && receipt.encounter_id == capture.encounter_id
            && receipt.content_sha256 == *hash
        {
            ingest::remember_alias(&transaction, capture, hash)?;
        }
    }
    transaction.commit()?;
    Ok(NativeImportReport {
        source_sha256,
        imported_count: receipts
            .iter()
            .filter(|receipt| receipt.outcome == ImportOutcome::Imported)
            .count(),
        already_present_count: receipts
            .iter()
            .filter(|receipt| receipt.outcome == ImportOutcome::AlreadyPresent)
            .count(),
        source: parsed.report.clone(),
        receipts,
    })
}

fn to_db(value: u64) -> Result<i64, EncounterError> {
    i64::try_from(value)
        .map_err(|_| EncounterError::Validation("native index value exceeds its range".into()))
}
fn native_times(
    capture: &super::native::NativeEncounter,
) -> Result<(String, String), EncounterError> {
    let epoch = capture.run.header.epoch_ms;
    let started = epoch
        .checked_add(capture.started_ms)
        .ok_or_else(|| EncounterError::Validation("native timestamp overflowed".into()))?;
    let finished = epoch
        .checked_add(capture.finished_ms)
        .ok_or_else(|| EncounterError::Validation("native timestamp overflowed".into()))?;
    Ok(((started / 1000).to_string(), (finished / 1000).to_string()))
}

pub fn load_native_encounter(
    store_path: impl AsRef<Path>,
    session_id: &str,
    encounter_id: &str,
) -> Result<Option<super::native::NativeEncounter>, EncounterError> {
    let connection = open_store(store_path.as_ref(), false, false)?;
    let version: u32 = connection.pragma_query_value(None, "user_version", |row| row.get(0))?;
    if version < 6 {
        return Ok(None);
    }
    let canonical: Option<Vec<u8>> = connection
        .query_row(
            "SELECT canonical_json FROM native_encounters WHERE session_id=?1 AND encounter_id=?2",
            params![session_id, encounter_id],
            |row| row.get(0),
        )
        .optional()?;
    canonical
        .map(|bytes| {
            serde_json::from_slice(&bytes).map_err(|_| {
                EncounterError::Validation("stored native encounter is invalid".into())
            })
        })
        .transpose()
}

fn native_summaries(connection: &Connection) -> Result<Vec<EncounterSummary>, EncounterError> {
    let mut statement = connection.prepare(
        "SELECT content_sha256,session_id,encounter_id,channel,started_at,finished_at,
         first_sequence,last_sequence,record_count,unknown_count,encounter_ordinal FROM native_encounters"
    )?;
    let rows = statement.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
            row.get::<_, String>(5)?,
            row.get::<_, i64>(6)?,
            row.get::<_, i64>(7)?,
            row.get::<_, i64>(8)?,
            row.get::<_, i64>(9)?,
            row.get::<_, i64>(10)?,
        ))
    })?;
    rows.map(|row| {
        let (
            hash,
            session_id,
            encounter_id,
            channel,
            started,
            finished,
            first,
            last,
            count,
            unknown,
            ordinal,
        ) = row?;
        Ok(EncounterSummary {
            content_sha256: hash,
            session_id,
            encounter_id,
            channel: parse_channel(&channel)?,
            status: CaptureStatus::Complete,
            started_at: started,
            finished_at: finished,
            first_sequence: nonnegative_db(first, "native first sequence")?,
            last_sequence: nonnegative_db(last, "native last sequence")?,
            stored_event_count: usize::try_from(count)
                .map_err(|_| EncounterError::Validation("native record count is invalid".into()))?,
            omitted_event_count: 0,
            capture_mode: None,
            encounter_ordinal: nonnegative_db(ordinal, "native ordinal")?,
            source_kind: super::model::EncounterSourceKind::NativeLog,
            unknown_record_count: usize::try_from(unknown).map_err(|_| {
                EncounterError::Validation("native unknown count is invalid".into())
            })?,
        })
    })
    .collect()
}

fn validate_native_records(connection: &Connection) -> Result<(), EncounterError> {
    use super::native::{
        canonical_native_bytes, canonical_native_run_bytes, NativeEncounter, NativeRun,
        MAX_NATIVE_CANONICAL_BYTES,
    };
    let mut runs = std::collections::BTreeMap::new();
    let mut statement = connection.prepare(
        "SELECT session_id,content_sha256,channel,native_schema_version,run_ordinal,length(canonical_json),
         CASE WHEN length(canonical_json)<=262144 THEN canonical_json END FROM native_runs"
    )?;
    let rows = statement.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, i64>(3)?,
            row.get::<_, i64>(4)?,
            row.get::<_, i64>(5)?,
            row.get::<_, Option<Vec<u8>>>(6)?,
        ))
    })?;
    for row in rows {
        let (id, hash, channel, schema, ordinal, length, canonical) = row?;
        if !(0..=262144).contains(&length) {
            return invalid("native run exceeds its byte limit");
        }
        let canonical = canonical.ok_or_else(|| {
            EncounterError::Validation("native run exceeds its byte limit".into())
        })?;
        let run: NativeRun = serde_json::from_slice(&canonical)
            .map_err(|_| EncounterError::Validation("stored native run is invalid".into()))?;
        if canonical_native_run_bytes(&run)? != canonical
            || sha256(&canonical) != hash
            || run.session_id != id
            || run.channel.as_str() != channel
            || u32::try_from(schema) != Ok(run.schema_version)
            || u64::try_from(ordinal) != Ok(run.run_ordinal)
        {
            return invalid("native run index does not match canonical evidence");
        }
        runs.insert(id, run);
    }
    let mut statement=connection.prepare(
        "SELECT content_sha256,source_sha256,session_id,encounter_id,channel,native_schema_version,
         encounter_ordinal,started_at,finished_at,first_sequence,last_sequence,record_count,unknown_count,
         length(canonical_json),CASE WHEN length(canonical_json)<=67108864 THEN canonical_json END FROM native_encounters"
    )?;
    let rows = statement.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
            row.get::<_, i64>(5)?,
            row.get::<_, i64>(6)?,
            row.get::<_, String>(7)?,
            row.get::<_, String>(8)?,
            row.get::<_, i64>(9)?,
            row.get::<_, i64>(10)?,
            row.get::<_, i64>(11)?,
            row.get::<_, i64>(12)?,
            row.get::<_, i64>(13)?,
            row.get::<_, Option<Vec<u8>>>(14)?,
        ))
    })?;
    let mut referenced = std::collections::BTreeSet::new();
    for row in rows {
        let (
            hash,
            source,
            id,
            encounter,
            channel,
            schema,
            ordinal,
            started,
            finished,
            first,
            last,
            count,
            unknown,
            length,
            canonical,
        ) = row?;
        if length < 0 || length as usize > MAX_NATIVE_CANONICAL_BYTES {
            return invalid("native encounter exceeds its byte limit");
        }
        let canonical = canonical.ok_or_else(|| {
            EncounterError::Validation("native encounter exceeds its byte limit".into())
        })?;
        let capture: NativeEncounter = serde_json::from_slice(&canonical)
            .map_err(|_| EncounterError::Validation("stored native encounter is invalid".into()))?;
        let times = native_times(&capture)?;
        if canonical_native_bytes(&capture)? != canonical
            || sha256(&canonical) != hash
            || !is_sha256(&source)
            || capture.run.session_id != id
            || capture.encounter_id != encounter
            || capture.run.channel.as_str() != channel
            || u32::try_from(schema) != Ok(capture.schema_version)
            || u64::try_from(ordinal) != Ok(capture.encounter_ordinal)
            || started != times.0
            || finished != times.1
            || u64::try_from(first) != Ok(capture.first_ordinal)
            || u64::try_from(last) != Ok(capture.last_ordinal)
            || usize::try_from(count) != Ok(capture.records.len())
            || usize::try_from(unknown)
                != Ok(capture.unknown_record_count + capture.unknown_context_count)
            || !runs
                .get(&id)
                .is_some_and(|run| same_native_run(run, &capture.run))
        {
            return invalid("native encounter index does not match canonical evidence");
        }
        referenced.insert(id);
    }
    if runs.keys().any(|id| !referenced.contains(id)) {
        return invalid("native run has no imported encounter");
    }
    Ok(())
}

fn same_native_run(left: &super::native::NativeRun, right: &super::native::NativeRun) -> bool {
    if left.schema_version == 1 || right.schema_version == 1 {
        return left == right;
    }
    left.schema_version == right.schema_version
        && left.session_id == right.session_id
        && left.channel == right.channel
        && left.header.epoch_ms == right.header.epoch_ms
        && left.header.log_version == right.header.log_version
        && left.header.realm == right.header.realm
        && left.header.language == right.header.language
        && left.header.game_version == right.header.game_version
}
