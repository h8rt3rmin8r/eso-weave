use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

use rusqlite::{params, Connection, OpenFlags, OptionalExtension, TransactionBehavior};

use crate::bounded_file::is_link_like;
use crate::catalog::Channel;

use super::model::{
    BackupReceipt, CaptureStatus, DeleteReceipt, EncounterCapture, EncounterSummary, ImportOutcome,
    ImportReceipt,
};
use super::{
    ensure_distinct_paths, invalid, sha256, EncounterError, CANONICAL_FORMAT_VERSION,
    STORE_SCHEMA_VERSION,
};

const META_TABLE_SQL: &str = r#"CREATE TABLE encounter_store_meta (
    singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
    schema_version INTEGER NOT NULL CHECK (schema_version = 1),
    canonical_format_version INTEGER NOT NULL CHECK (canonical_format_version = 1)
)"#;
const META_INSERT_SQL: &str = r#"INSERT INTO encounter_store_meta(singleton, schema_version, canonical_format_version)
VALUES (1, 1, 1)"#;
const RAW_TABLE_SQL: &str = r#"CREATE TABLE raw_encounters (
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
const IMMUTABILITY_TRIGGER_SQL: &str = r#"CREATE TRIGGER raw_encounters_no_update
BEFORE UPDATE ON raw_encounters
BEGIN
    SELECT RAISE(ABORT, 'raw encounter records are immutable');
END"#;

pub(crate) fn append(
    store_path: &Path,
    capture: &EncounterCapture,
    canonical: &[u8],
    source_sha256: String,
    content_sha256: String,
) -> Result<ImportReceipt, EncounterError> {
    let mut connection = open_store(store_path, true, true)?;
    let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
    let existing = transaction
        .query_row(
            "SELECT content_sha256 FROM raw_encounters WHERE session_id = ?1 AND encounter_id = ?2",
            params![capture.session_id, capture.encounter_id],
            |row| row.get::<_, String>(0),
        )
        .optional()?;
    let outcome = match existing {
        Some(existing) if existing == content_sha256 => ImportOutcome::AlreadyPresent,
        Some(_) => return invalid("encounter identity already exists with different content"),
        None => {
            transaction.execute(
                "INSERT INTO raw_encounters (
                    content_sha256, source_sha256, session_id, encounter_id, channel,
                    capture_schema_version, addon_version, status, started_at, finished_at,
                    first_sequence, last_sequence, stored_event_count, omitted_event_count,
                    canonical_json
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
                params![
                    content_sha256,
                    source_sha256,
                    capture.session_id,
                    capture.encounter_id,
                    capture.channel.as_str(),
                    capture.schema_version,
                    capture.addon_version,
                    capture.status.as_str(),
                    capture.started_at,
                    capture.finished_at,
                    i64::try_from(capture.first_sequence).map_err(|_| {
                        EncounterError::Validation("first sequence is out of range".into())
                    })?,
                    i64::try_from(capture.last_sequence).map_err(
                        |_| EncounterError::Validation("last sequence is out of range".into())
                    )?,
                    i64::try_from(capture.stored_event_count).map_err(|_| {
                        EncounterError::Validation("stored count is out of range".into())
                    })?,
                    i64::try_from(capture.omitted_event_count).map_err(|_| {
                        EncounterError::Validation("omitted count is out of range".into())
                    })?,
                    canonical,
                ],
            )?;
            ImportOutcome::Imported
        }
    };
    transaction.commit()?;
    Ok(ImportReceipt {
        outcome,
        source_sha256,
        content_sha256,
        channel: capture.channel,
        status: capture.status,
        session_id: capture.session_id.clone(),
        encounter_id: capture.encounter_id.clone(),
        stored_event_count: capture.stored_event_count,
        omitted_event_count: capture.omitted_event_count,
    })
}

pub fn list_encounters(
    store_path: impl AsRef<Path>,
) -> Result<Vec<EncounterSummary>, EncounterError> {
    let connection = open_store(store_path.as_ref(), false, false)?;
    let mut statement = connection.prepare(
        "SELECT content_sha256, session_id, encounter_id, channel, status, started_at,
                finished_at, first_sequence, last_sequence, stored_event_count,
                omitted_event_count
         FROM raw_encounters
         ORDER BY length(started_at), started_at, session_id, encounter_id",
    )?;
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
        ))
    })?;
    rows.map(|row| {
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
            .map_err(|_| EncounterError::Validation("stored event count is out of range".into()))?,
            omitted_event_count: nonnegative_db(omitted_event_count, "omitted event count")?,
        })
    })
    .collect()
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
    let deleted = transaction.execute(
        "DELETE FROM raw_encounters WHERE session_id = ?1 AND encounter_id = ?2",
        params![session_id, encounter_id],
    )?;
    transaction.commit()?;
    Ok(DeleteReceipt {
        deleted_records: deleted,
    })
}

pub fn delete_all(store_path: impl AsRef<Path>) -> Result<DeleteReceipt, EncounterError> {
    let mut connection = open_store(store_path.as_ref(), false, true)?;
    let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
    let deleted = transaction.execute("DELETE FROM raw_encounters", [])?;
    transaction.commit()?;
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
    validate_record_hashes(&connection)?;
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
    validate_record_hashes(&candidate)?;
    drop(candidate);
    File::options().write(true).open(&temporary)?.sync_all()?;
    let (byte_length, digest) = hash_file(&temporary)?;
    crate::atomic_file::persist(temporary, destination)?;
    Ok(BackupReceipt {
        schema_version: STORE_SCHEMA_VERSION,
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
        let schema = format!(
            "{META_TABLE_SQL};\n{META_INSERT_SQL};\n{RAW_TABLE_SQL};\n{IMMUTABILITY_TRIGGER_SQL};\nPRAGMA user_version = 1;"
        );
        connection.execute_batch("BEGIN IMMEDIATE;")?;
        if let Err(error) = connection
            .execute_batch(&schema)
            .and_then(|_| connection.execute_batch("COMMIT;"))
        {
            let _ = connection.execute_batch("ROLLBACK;");
            return Err(error.into());
        }
    } else if version != STORE_SCHEMA_VERSION {
        return invalid(format!("unsupported encounter store schema {version}"));
    }
    validate_schema(&connection)?;
    validate_record_hashes(&connection)?;
    Ok(connection)
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

fn validate_schema(connection: &Connection) -> Result<(), EncounterError> {
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
    let expected = [
        (
            "table",
            "encounter_store_meta",
            "encounter_store_meta",
            META_TABLE_SQL,
        ),
        ("table", "raw_encounters", "raw_encounters", RAW_TABLE_SQL),
        (
            "trigger",
            "raw_encounters_no_update",
            "raw_encounters",
            IMMUTABILITY_TRIGGER_SQL,
        ),
    ];
    let schema_matches = actual.len() == expected.len()
        && actual.iter().zip(expected).all(|(actual, expected)| {
            actual.0 == expected.0
                && actual.1 == expected.1
                && actual.2 == expected.2
                && normalize_sql(&actual.3) == normalize_sql(expected.3)
        });
    if !schema_matches {
        return invalid("encounter store schema objects do not match schema version 1");
    }

    let meta: Option<(u32, u32)> = connection
        .query_row(
            "SELECT schema_version, canonical_format_version FROM encounter_store_meta WHERE singleton = 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()?;
    if meta != Some((STORE_SCHEMA_VERSION, CANONICAL_FORMAT_VERSION)) {
        return invalid("encounter store metadata does not match its schema");
    }
    Ok(())
}

fn normalize_sql(sql: &str) -> String {
    sql.split_ascii_whitespace().collect::<Vec<_>>().join(" ")
}

fn validate_record_hashes(connection: &Connection) -> Result<(), EncounterError> {
    let mut statement =
        connection.prepare("SELECT content_sha256, canonical_json FROM raw_encounters")?;
    let rows = statement.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, Vec<u8>>(1)?))
    })?;
    for row in rows {
        let (expected, canonical) = row?;
        if sha256(&canonical) != expected {
            return invalid("encounter store contains a raw content hash mismatch");
        }
    }
    Ok(())
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

fn nonnegative_db(value: i64, label: &str) -> Result<u64, EncounterError> {
    u64::try_from(value)
        .map_err(|_| EncounterError::Validation(format!("stored {label} is negative")))
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
    Ok((length, format!("{:x}", hasher.finalize())))
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
