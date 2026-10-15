//! Atomic incremental ingest, immutable logical aliases and rebuildable results.

use std::path::Path;
use std::time::Duration;

use rusqlite::{
    params, Connection, OpenFlags, OptionalExtension, Transaction, TransactionBehavior,
};

use super::{ensure_session_order, invalid, is_sha256, sha256, to_db, EncounterError};
use crate::catalog_update::CancellationToken;
use crate::encounter::{
    canonical_native_bytes, logical_native_identity, project_native_encounter, NativeCheckpoint,
    NativeCompletedFight, NativeEncounter, NativeLogicalIdentity, MAX_NATIVE_CHECKPOINT_BYTES,
};

const ALIASES: &str = r#"CREATE TABLE native_logical_aliases (
    fight_key TEXT PRIMARY KEY CHECK (length(fight_key) = 64),
    run_key TEXT NOT NULL CHECK (length(run_key) = 64),
    evidence_sha256 TEXT NOT NULL CHECK (length(evidence_sha256) = 64),
    session_id TEXT NOT NULL,
    encounter_id TEXT NOT NULL,
    content_sha256 TEXT NOT NULL CHECK (length(content_sha256) = 64)
)"#;
const CHECKPOINTS: &str = r#"CREATE TABLE native_ingest_checkpoints (
    source_key TEXT PRIMARY KEY CHECK (length(source_key) = 64),
    generation TEXT NOT NULL CHECK (length(generation) = 64),
    revision INTEGER NOT NULL CHECK (revision > 0),
    consumed_bytes INTEGER NOT NULL CHECK (consumed_bytes >= 0),
    channel TEXT NOT NULL CHECK (channel IN ('live', 'pts')),
    checkpoint_sha256 TEXT NOT NULL CHECK (length(checkpoint_sha256) = 64),
    checkpoint_bytes BLOB NOT NULL CHECK (length(checkpoint_bytes) <= 134217728)
)"#;
const DELETED: &str = r#"CREATE TABLE native_deleted_encounters (
    fight_key TEXT PRIMARY KEY CHECK (length(fight_key) = 64),
    evidence_sha256 TEXT NOT NULL CHECK (length(evidence_sha256) = 64),
    session_id TEXT NOT NULL,
    encounter_id TEXT NOT NULL,
    content_sha256 TEXT NOT NULL CHECK (length(content_sha256) = 64)
)"#;
const DERIVATIONS: &str = r#"CREATE TABLE encounter_derivations (
    content_sha256 TEXT NOT NULL CHECK (length(content_sha256) = 64),
    source_kind TEXT NOT NULL CHECK (source_kind IN ('native-log', 'saved-addon')),
    algorithm_version TEXT NOT NULL,
    catalog_sha256 TEXT NOT NULL,
    processing_context_sha256 TEXT NOT NULL,
    processing_generation INTEGER NOT NULL CHECK (processing_generation >= 0),
    result_sha256 TEXT NOT NULL CHECK (length(result_sha256) = 64),
    result_json BLOB NOT NULL CHECK (length(result_json) <= 67108864),
    PRIMARY KEY (content_sha256, source_kind, algorithm_version, catalog_sha256)
)"#;
const DERIVATION_CONTEXT_INDEX:&str="CREATE INDEX encounter_derivations_context ON encounter_derivations(content_sha256, source_kind, algorithm_version, processing_context_sha256)";
const ALIAS_RETAINED_INDEX:&str="CREATE INDEX native_logical_aliases_retained ON native_logical_aliases(session_id, encounter_id)";

pub(super) fn schema_objects() -> Vec<(&'static str, &'static str, &'static str, &'static str)> {
    vec![
        (
            "index",
            "encounter_derivations_context",
            "encounter_derivations",
            DERIVATION_CONTEXT_INDEX,
        ),
        (
            "index",
            "native_logical_aliases_retained",
            "native_logical_aliases",
            ALIAS_RETAINED_INDEX,
        ),
        (
            "table",
            "native_logical_aliases",
            "native_logical_aliases",
            ALIASES,
        ),
        (
            "table",
            "native_ingest_checkpoints",
            "native_ingest_checkpoints",
            CHECKPOINTS,
        ),
        (
            "table",
            "native_deleted_encounters",
            "native_deleted_encounters",
            DELETED,
        ),
        (
            "table",
            "encounter_derivations",
            "encounter_derivations",
            DERIVATIONS,
        ),
    ]
}

pub(super) fn schema_sql() -> String {
    format!("{ALIASES};\n{CHECKPOINTS};\n{DELETED};\n{DERIVATIONS};\n{DERIVATION_CONTEXT_INDEX};\n{ALIAS_RETAINED_INDEX};")
}

#[derive(Clone, PartialEq, Eq)]
pub struct NativeIngestCheckpoint {
    pub source_key: String,
    pub generation: String,
    pub revision: u64,
    pub consumed_bytes: u64,
    pub channel: crate::catalog::Channel,
    pub checkpoint_bytes: Vec<u8>,
    pub checkpoint_sha256: String,
}

impl std::fmt::Debug for NativeIngestCheckpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeIngestCheckpoint")
            .field("revision", &self.revision)
            .field("consumed_bytes", &self.consumed_bytes)
            .field("checkpoint_length", &self.checkpoint_bytes.len())
            .finish()
    }
}

pub struct NativeIngestBatch {
    pub source_key: String,
    pub generation: String,
    pub expected_revision: Option<u64>,
    pub consumed_bytes: u64,
    pub checkpoint_bytes: Vec<u8>,
    pub completed: Option<NativeCompletedFight>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeIngestReceipt {
    pub revision: u64,
    pub consumed_bytes: u64,
    pub imported_count: usize,
    pub already_present_count: usize,
    pub suppressed_count: usize,
}

pub struct NativeIngestWriter {
    connection: Connection,
}

/// Final commit is admitted while the application's cancellation gate is held.
pub struct NativeCommit<'connection> {
    transaction: Option<Transaction<'connection>>,
    committed: bool,
}

impl NativeCommit<'_> {
    pub fn commit(&mut self) -> Result<(), EncounterError> {
        let transaction = self.transaction.take().ok_or_else(|| {
            EncounterError::Validation("native transaction already finalized".into())
        })?;
        transaction.commit()?;
        self.committed = true;
        Ok(())
    }
}

impl NativeIngestWriter {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, EncounterError> {
        Self::open_with_cancellation(path, CancellationToken::new())
    }

    pub fn open_with_cancellation(
        path: impl AsRef<Path>,
        cancellation: CancellationToken,
    ) -> Result<Self, EncounterError> {
        let path = path.as_ref();
        if cancellation.is_cancelled() {
            return invalid("native store opening cancelled");
        }
        match std::fs::symlink_metadata(path) {
            Ok(meta) if !meta.is_file() || crate::bounded_file::is_link_like(&meta) => {
                return invalid("encounter store must be a regular non-link file");
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
            }
            Err(error) => return Err(error.into()),
        }
        let connection = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        )?;
        connection.busy_timeout(Duration::from_millis(100))?;
        let token = cancellation.clone();
        connection.progress_handler(1000, Some(move || token.is_cancelled()))?;
        let version: u32 = connection.pragma_query_value(None, "user_version", |r| r.get(0))?;
        if version == 0 {
            if !super::schema_is_empty(&connection)? {
                return invalid("encounter store has an unrecognized schema");
            }
        } else {
            super::validate_integrity(&connection)?;
            super::validate_store_contents(&connection)?;
        }
        if cancellation.is_cancelled() {
            return invalid("native store opening cancelled");
        }
        // The per-root token belongs to one job. A later eligible root may reuse
        // this connection, so job cancellation is checked by commit admission.
        connection.progress_handler(0, None::<fn() -> bool>)?;
        Ok(Self { connection })
    }

    pub fn checkpoint(
        &self,
        source_key: &str,
    ) -> Result<Option<NativeIngestCheckpoint>, EncounterError> {
        if !is_sha256(source_key) {
            return invalid("native source key is invalid");
        }
        let version: u32 = self
            .connection
            .pragma_query_value(None, "user_version", |r| r.get(0))?;
        if version < 7 {
            return Ok(None);
        }
        checkpoint(&self.connection, source_key)
    }

    pub fn commit_batch<F>(
        &mut self,
        batch: NativeIngestBatch,
        finalize: F,
    ) -> Result<NativeIngestReceipt, EncounterError>
    where
        F: FnOnce(&mut NativeCommit<'_>) -> Result<(), EncounterError>,
    {
        if !is_sha256(&batch.source_key)
            || !is_sha256(&batch.generation)
            || batch.checkpoint_bytes.len() > MAX_NATIVE_CHECKPOINT_BYTES
        {
            return invalid("native ingest batch metadata is invalid");
        }
        let parser_checkpoint = NativeCheckpoint::from_bytes(&batch.checkpoint_bytes)
            .map_err(|_| EncounterError::Validation("native checkpoint is invalid".into()))?;
        if parser_checkpoint.position().accepted_bytes != batch.consumed_bytes {
            return invalid("native checkpoint position does not match batch");
        }
        if batch
            .completed
            .as_ref()
            .is_some_and(|fight| fight.encounter.run.channel != parser_checkpoint.channel())
        {
            return invalid("native checkpoint channel does not match fight");
        }
        if batch.completed.as_ref().is_some_and(|fight| {
            let capture = &fight.encounter;
            capture.schema_version != 2
                || capture.last_ordinal > parser_checkpoint.position().record_count
                || capture
                    .records
                    .iter()
                    .chain(&capture.context)
                    .chain(std::iter::once(&capture.run.header.record))
                    .any(|record| {
                        record
                            .byte_offset
                            .checked_add(record.line.len() as u64)
                            .is_none_or(|end| end > parser_checkpoint.position().complete_bytes)
                    })
        }) {
            return invalid("native completed fight exceeds checkpoint progress");
        }
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let version: u32 = transaction.pragma_query_value(None, "user_version", |r| r.get(0))?;
        if version == 0 {
            create_store(&transaction)?;
        } else {
            super::migrate_for_write(&transaction)?;
        }
        let previous = checkpoint_head(&transaction, &batch.source_key)?;
        if previous.as_ref().map(|c| c.revision) != batch.expected_revision {
            return invalid("native checkpoint revision conflict");
        }
        if previous.as_ref().is_some_and(|old| {
            old.generation == batch.generation && old.consumed_bytes > batch.consumed_bytes
        }) {
            return invalid("native checkpoint cannot move backward in a generation");
        }
        if let Some(old) = &previous {
            if old.generation == batch.generation && old.channel != parser_checkpoint.channel() {
                return invalid("native checkpoint cannot change channel in a generation");
            }
        }
        let revision = batch
            .expected_revision
            .unwrap_or(0)
            .checked_add(1)
            .ok_or_else(|| {
                EncounterError::Validation("native checkpoint revision exhausted".into())
            })?;
        let mut receipt = NativeIngestReceipt {
            revision,
            consumed_bytes: batch.consumed_bytes,
            imported_count: 0,
            already_present_count: 0,
            suppressed_count: 0,
        };
        if let Some(fight) = batch.completed {
            insert_completed(&transaction, &fight, &mut receipt)?;
        }
        transaction.execute(
            "INSERT INTO native_ingest_checkpoints VALUES (?1,?2,?3,?4,?5,?6,?7)
             ON CONFLICT(source_key) DO UPDATE SET generation=excluded.generation,
             revision=excluded.revision,consumed_bytes=excluded.consumed_bytes,
             channel=excluded.channel,checkpoint_sha256=excluded.checkpoint_sha256,
             checkpoint_bytes=excluded.checkpoint_bytes",
            params![
                batch.source_key,
                batch.generation,
                to_db(revision)?,
                to_db(batch.consumed_bytes)?,
                parser_checkpoint.channel().as_str(),
                sha256(&batch.checkpoint_bytes),
                batch.checkpoint_bytes
            ],
        )?;
        let mut commit = NativeCommit {
            transaction: Some(transaction),
            committed: false,
        };
        finalize(&mut commit)?;
        if !commit.committed {
            return invalid("native transaction was not admitted");
        }
        Ok(receipt)
    }
}

fn create_store(transaction: &Transaction<'_>) -> Result<(), EncounterError> {
    if !super::schema_is_empty(transaction)? {
        return invalid("encounter store has an unrecognized schema");
    }
    transaction.execute_batch(&format!(
        "{}; {}; {}; {}; {}; {}; {}; {}; {} {} PRAGMA user_version=7;",
        super::META_TABLE_SQL,
        super::META_INSERT_SQL,
        super::RAW_TABLE_SQL,
        super::SESSION_TABLE_SQL,
        super::IMMUTABILITY_TRIGGER_SQL,
        super::SESSION_IMMUTABILITY_TRIGGER_SQL,
        super::ORDER_TABLE_SQL,
        super::ORDER_IMMUTABILITY_TRIGGER_SQL,
        super::native_schema_sql(),
        schema_sql()
    ))?;
    Ok(())
}

fn checkpoint(
    connection: &Connection,
    key: &str,
) -> Result<Option<NativeIngestCheckpoint>, EncounterError> {
    let row = connection.query_row(
        "SELECT generation,revision,consumed_bytes,channel,checkpoint_sha256,length(checkpoint_bytes),
         CASE WHEN length(checkpoint_bytes)<=134217728 THEN checkpoint_bytes END
         FROM native_ingest_checkpoints WHERE source_key=?1", [key],
        |r| Ok((r.get::<_,String>(0)?,r.get::<_,i64>(1)?,r.get::<_,i64>(2)?,r.get::<_,String>(3)?,r.get::<_,String>(4)?,r.get::<_,i64>(5)?,r.get::<_,Option<Vec<u8>>>(6)?)),
    ).optional()?;
    row.map(
        |(generation, revision, consumed, channel, hash, length, bytes)| {
            if !is_sha256(&generation)
                || !is_sha256(&hash)
                || revision <= 0
                || consumed < 0
                || length < 0
                || length as u64 > MAX_NATIVE_CHECKPOINT_BYTES as u64
            {
                return invalid("stored native checkpoint metadata is invalid");
            }
            let bytes = bytes.ok_or_else(|| {
                EncounterError::Validation("native checkpoint exceeds byte limit".into())
            })?;
            if sha256(&bytes) != hash {
                return invalid("stored native checkpoint hash is invalid");
            }
            let encoded = NativeCheckpoint::from_bytes(&bytes).map_err(|_| {
                EncounterError::Validation("stored native checkpoint is invalid".into())
            })?;
            let expected_channel = super::parse_channel(&channel)?;
            if encoded.channel() != expected_channel
                || encoded.position().accepted_bytes != consumed as u64
            {
                return invalid("stored native checkpoint position is invalid");
            }
            Ok(NativeIngestCheckpoint {
                source_key: key.into(),
                generation,
                revision: revision as u64,
                consumed_bytes: consumed as u64,
                channel: expected_channel,
                checkpoint_bytes: bytes,
                checkpoint_sha256: hash,
            })
        },
    )
    .transpose()
}

struct CheckpointHead {
    generation: String,
    revision: u64,
    consumed_bytes: u64,
    channel: crate::catalog::Channel,
}

fn checkpoint_head(
    connection: &Connection,
    key: &str,
) -> Result<Option<CheckpointHead>, EncounterError> {
    let row=connection.query_row("SELECT generation,revision,consumed_bytes,channel FROM native_ingest_checkpoints WHERE source_key=?1",[key],|r|Ok((r.get::<_,String>(0)?,r.get::<_,i64>(1)?,r.get::<_,i64>(2)?,r.get::<_,String>(3)?))).optional()?;
    row.map(|(generation, revision, bytes, channel)| {
        if !is_sha256(&generation) || revision <= 0 || bytes < 0 {
            return invalid("stored native checkpoint metadata is invalid");
        }
        Ok(CheckpointHead {
            generation,
            revision: revision as u64,
            consumed_bytes: bytes as u64,
            channel: super::parse_channel(&channel)?,
        })
    })
    .transpose()
}

pub(super) fn backfill_aliases(connection: &Connection) -> Result<(), EncounterError> {
    let mut statement = connection.prepare(
        "SELECT e.canonical_json,e.content_sha256 FROM native_encounters e
         WHERE NOT EXISTS(SELECT 1 FROM native_logical_aliases a WHERE a.session_id=e.session_id AND a.encounter_id=e.encounter_id)",
    )?;
    let mut rows = statement.query([])?;
    while let Some(row) = rows.next()? {
        let bytes: Vec<u8> = row.get(0)?;
        let hash: String = row.get(1)?;
        let capture: NativeEncounter = serde_json::from_slice(&bytes)
            .map_err(|_| EncounterError::Validation("stored native evidence is invalid".into()))?;
        remember_alias(connection, &capture, &hash)?;
    }
    Ok(())
}

pub(super) fn remember_alias(
    connection: &Connection,
    capture: &NativeEncounter,
    hash: &str,
) -> Result<(), EncounterError> {
    let identity = logical_native_identity(capture)?;
    let prior: Option<String> = connection
        .query_row(
            "SELECT evidence_sha256 FROM native_logical_aliases WHERE fight_key=?1",
            [&identity.fight_key],
            |r| r.get(0),
        )
        .optional()?;
    if prior
        .as_ref()
        .is_some_and(|old| old != &identity.evidence_sha256)
    {
        return invalid("native logical encounter identity collision");
    }
    connection.execute(
        "INSERT OR IGNORE INTO native_logical_aliases VALUES (?1,?2,?3,?4,?5,?6)",
        params![
            identity.fight_key,
            identity.run_key,
            identity.evidence_sha256,
            capture.run.session_id,
            capture.encounter_id,
            hash
        ],
    )?;
    Ok(())
}

pub(super) enum LogicalDisposition {
    Missing,
    Present {
        session_id: String,
        encounter_id: String,
        content_sha256: String,
    },
    Suppressed,
}

/// All native intake shares deletion and logical identity authority before any
/// raw run, encounter or first-import order is inserted.
pub(super) fn logical_disposition(
    connection: &Connection,
    logical: &NativeLogicalIdentity,
) -> Result<LogicalDisposition, EncounterError> {
    let deleted: Option<String> = connection
        .query_row(
            "SELECT evidence_sha256 FROM native_deleted_encounters WHERE fight_key=?1",
            [&logical.fight_key],
            |r| r.get(0),
        )
        .optional()?;
    if let Some(deleted) = deleted {
        if deleted != logical.evidence_sha256 {
            return invalid("deleted native encounter identity collision");
        }
        return Ok(LogicalDisposition::Suppressed);
    }
    let existing: Option<(String,String,String,String)> = connection.query_row("SELECT evidence_sha256,session_id,encounter_id,content_sha256 FROM native_logical_aliases WHERE fight_key=?1",[&logical.fight_key],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?))).optional()?;
    if let Some((fingerprint, session, encounter, hash)) = existing {
        if fingerprint != logical.evidence_sha256 {
            return invalid("native logical encounter identity collision");
        }
        let present: bool = connection.query_row("SELECT EXISTS(SELECT 1 FROM native_encounters WHERE session_id=?1 AND encounter_id=?2 AND content_sha256=?3)",params![session,encounter,hash],|r|r.get(0))?;
        if !present {
            return invalid("native alias references unavailable evidence");
        }
        return Ok(LogicalDisposition::Present {
            session_id: session,
            encounter_id: encounter,
            content_sha256: hash,
        });
    }
    Ok(LogicalDisposition::Missing)
}

fn insert_completed(
    transaction: &Transaction<'_>,
    fight: &NativeCompletedFight,
    receipt: &mut NativeIngestReceipt,
) -> Result<(), EncounterError> {
    let capture = &fight.encounter;
    let logical = logical_native_identity(capture)?;
    if logical != fight.logical_identity {
        return invalid("native completed fight identity is invalid");
    }
    let canonical = canonical_native_bytes(capture)?;
    let hash = sha256(&canonical);
    match logical_disposition(transaction, &logical)? {
        LogicalDisposition::Suppressed => {
            receipt.suppressed_count = 1;
            return Ok(());
        }
        LogicalDisposition::Present { .. } => {
            receipt.already_present_count = 1;
            return Ok(());
        }
        LogicalDisposition::Missing => {}
    }
    let run_bytes = crate::encounter::native::canonical_native_run_bytes(&capture.run)?;
    let existing_run: Option<Vec<u8>> = transaction
        .query_row(
            "SELECT canonical_json FROM native_runs WHERE session_id=?1",
            [&capture.run.session_id],
            |r| r.get(0),
        )
        .optional()?;
    if let Some(prior) = existing_run {
        let prior = serde_json::from_slice(&prior)
            .map_err(|_| EncounterError::Validation("stored native run is invalid".into()))?;
        if !super::same_native_run(&prior, &capture.run) {
            return invalid("native recording run identity collision");
        }
    }
    ensure_session_order(transaction, &capture.run.session_id)?;
    transaction.execute(
        "INSERT OR IGNORE INTO native_runs VALUES (?1,?2,?3,?4,?5,?6)",
        params![
            capture.run.session_id,
            sha256(&run_bytes),
            capture.run.channel.as_str(),
            capture.run.schema_version,
            to_db(capture.run.run_ordinal)?,
            run_bytes
        ],
    )?;
    let (started, finished) = super::native_times(capture)?;
    // This hash covers retained literal evidence with normalized LF separators,
    // not the unfinished physical file. Original whole-file hashes stay intact.
    let mut source = sha2::Sha256::new();
    use sha2::Digest;
    for record in std::iter::once(&capture.run.header.record)
        .chain(&capture.context)
        .chain(&capture.records)
    {
        source.update(record.line.as_bytes());
        source.update(b"\n");
    }
    let source_hash = crate::hash::hex_lower(source.finalize());
    transaction.execute(
        "INSERT INTO native_encounters VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14)",
        params![
            hash,
            source_hash,
            capture.run.session_id,
            capture.encounter_id,
            capture.run.channel.as_str(),
            capture.schema_version,
            to_db(capture.encounter_ordinal)?,
            started,
            finished,
            to_db(capture.first_ordinal)?,
            to_db(capture.last_ordinal)?,
            to_db(capture.records.len() as u64)?,
            to_db((capture.unknown_record_count + capture.unknown_context_count) as u64)?,
            canonical
        ],
    )?;
    remember_alias(transaction, capture, &hash)?;
    persist_native_projection(transaction, capture, &hash, 0)?;
    receipt.imported_count = 1;
    Ok(())
}

pub(super) fn persist_native_projection(
    connection: &Connection,
    capture: &NativeEncounter,
    hash: &str,
    generation: u64,
) -> Result<(), EncounterError> {
    let projection = project_native_encounter(capture)?;
    let result = serde_json::to_vec(&projection)
        .map_err(|_| EncounterError::Validation("native projection serialization failed".into()))?;
    persist_derivation(
        connection,
        hash,
        "native-log",
        &format!("native-v{}", projection.algorithm_version),
        "",
        generation,
        &result,
    )
}

pub(super) fn persist_derivation(
    connection: &Connection,
    hash: &str,
    kind: &str,
    algorithm: &str,
    catalog: &str,
    generation: u64,
    result: &[u8],
) -> Result<(), EncounterError> {
    persist_result(
        connection,
        &PreparedDerivation {
            content_sha256: hash.into(),
            source_kind: kind.into(),
            algorithm_version: algorithm.into(),
            catalog_sha256: catalog.into(),
            context_sha256: String::new(),
            generation,
            result: result.to_vec(),
        },
    )
}

pub(crate) struct PreparedDerivation {
    pub content_sha256: String,
    pub source_kind: String,
    pub algorithm_version: String,
    pub catalog_sha256: String,
    pub context_sha256: String,
    pub generation: u64,
    pub result: Vec<u8>,
}

pub(crate) struct PendingEvidence {
    pub hash: String,
    pub source_kind: String,
    pub canonical: Vec<u8>,
}

#[derive(Default, Clone)]
pub(crate) struct PendingCursor {
    addon: i64,
    native: i64,
    next_native: bool,
}

impl NativeIngestWriter {
    pub(crate) fn pending(
        &self,
        limit: usize,
        context: &str,
        cursor: &mut PendingCursor,
    ) -> Result<Vec<PendingEvidence>, EncounterError> {
        let version: u32 = self
            .connection
            .pragma_query_value(None, "user_version", |r| r.get(0))?;
        if version == 0 {
            return Ok(Vec::new());
        }
        let mut pending = Vec::new();
        let mut bytes = 0usize;
        // Bound examined keys as well as returned work. A LIMIT applied after
        // NOT EXISTS would still walk the entire history on every idle turn.
        let start_native = cursor.next_native;
        for native in [start_native, !start_native] {
            if native && version < 6 {
                continue;
            }
            let (table, kind, algorithm) = if native {
                ("native_encounters", "native-log", "native-v1")
            } else {
                ("raw_encounters", "saved-addon", "s069-v1+s090-v1")
            };
            let after = if native {
                &mut cursor.native
            } else {
                &mut cursor.addon
            };
            let missing = if version >= 7 {
                format!("NOT EXISTS(SELECT 1 FROM encounter_derivations d WHERE d.content_sha256=e.content_sha256 AND d.source_kind='{kind}' AND d.algorithm_version='{algorithm}'{})",if native {""}else{" AND d.processing_context_sha256=?2"})
            } else {
                "1".into()
            };
            // Bind ?2 even when it is not required by the native predicate.
            let sql=format!("SELECT rowid,content_sha256,length(canonical_json),CASE WHEN {missing} THEN canonical_json END FROM {table} e WHERE rowid>?1 AND ?2 IS NOT NULL ORDER BY rowid LIMIT 32");
            let mut statement = self.connection.prepare(&sql)?;
            let mut rows = statement.query(params![*after, context])?;
            let mut examined = false;
            while let Some(row) = rows.next()? {
                examined = true;
                let rowid: i64 = row.get(0)?;
                let size: i64 = row.get(2)?;
                if !(0..=67108864).contains(&size) {
                    return invalid("stored encounter exceeds derivation byte budget");
                }
                if bytes.saturating_add(size as usize) > 64 * 1024 * 1024 && !pending.is_empty() {
                    return Ok(pending);
                }
                let canonical: Option<Vec<u8>> = row.get(3)?;
                *after = rowid;
                if let Some(canonical) = canonical {
                    let hash: String = row.get(1)?;
                    if sha256(&canonical) != hash {
                        return invalid("stored encounter hash is invalid");
                    }
                    bytes += canonical.len();
                    pending.push(PendingEvidence {
                        hash,
                        source_kind: kind.into(),
                        canonical,
                    });
                    if pending.len() >= limit.clamp(1, 16) {
                        cursor.next_native = !native;
                        return Ok(pending);
                    }
                }
            }
            if !examined {
                *after = 0;
            }
        }
        cursor.next_native = !start_native;
        Ok(pending)
    }

    pub(crate) fn commit_derivations<F>(
        &mut self,
        results: Vec<PreparedDerivation>,
        finalize: F,
    ) -> Result<usize, EncounterError>
    where
        F: FnOnce(&mut NativeCommit<'_>) -> Result<(), EncounterError>,
    {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        super::migrate_for_write(&transaction)?;
        let mut stored = 0;
        for result in results {
            let sql = if result.source_kind == "native-log" {
                "SELECT EXISTS(SELECT 1 FROM native_encounters WHERE content_sha256=?1)"
            } else {
                "SELECT EXISTS(SELECT 1 FROM raw_encounters WHERE content_sha256=?1)"
            };
            let present: bool =
                transaction.query_row(sql, [&result.content_sha256], |r| r.get(0))?;
            if present {
                persist_result(&transaction, &result)?;
                stored += 1;
            }
        }
        let mut commit = NativeCommit {
            transaction: Some(transaction),
            committed: false,
        };
        finalize(&mut commit)?;
        if !commit.committed {
            return invalid("encounter processing transaction was not admitted");
        }
        Ok(stored)
    }
}

fn persist_result(
    connection: &Connection,
    result: &PreparedDerivation,
) -> Result<(), EncounterError> {
    let PreparedDerivation {
        content_sha256: hash,
        source_kind: kind,
        algorithm_version: algorithm,
        catalog_sha256: catalog,
        context_sha256: context,
        generation,
        result,
    } = result;
    if result.len() > 64 * 1024 * 1024
        || !is_sha256(hash)
        || (!catalog.is_empty() && !is_sha256(catalog))
    {
        return invalid("encounter derivation is invalid");
    }
    connection.execute("INSERT INTO encounter_derivations VALUES (?1,?2,?3,?4,?5,?6,?7,?8) ON CONFLICT(content_sha256,source_kind,algorithm_version,catalog_sha256) DO UPDATE SET processing_context_sha256=excluded.processing_context_sha256,processing_generation=excluded.processing_generation,result_sha256=excluded.result_sha256,result_json=excluded.result_json",params![hash,kind,algorithm,catalog,context,to_db(*generation)?,sha256(result),result])?;
    Ok(())
}

pub(super) fn suppress_deleted(
    connection: &Connection,
    session: Option<(&str, &str)>,
) -> Result<(), EncounterError> {
    if let Some((session, encounter)) = session {
        connection.execute("INSERT OR IGNORE INTO native_deleted_encounters SELECT fight_key,evidence_sha256,session_id,encounter_id,content_sha256 FROM native_logical_aliases WHERE session_id=?1 AND encounter_id=?2",params![session,encounter])?;
        connection.execute("DELETE FROM encounter_derivations WHERE content_sha256 IN (SELECT content_sha256 FROM native_encounters WHERE session_id=?1 AND encounter_id=?2 UNION SELECT content_sha256 FROM raw_encounters WHERE session_id=?1 AND encounter_id=?2)",params![session,encounter])?;
    } else {
        connection.execute("INSERT OR IGNORE INTO native_deleted_encounters SELECT fight_key,evidence_sha256,session_id,encounter_id,content_sha256 FROM native_logical_aliases",[])?;
        connection.execute("DELETE FROM encounter_derivations", [])?;
    }
    Ok(())
}

pub(super) fn validate_contents(connection: &Connection) -> Result<(), EncounterError> {
    let mut aliases=connection.prepare("SELECT fight_key,run_key,evidence_sha256,session_id,encounter_id,content_sha256 FROM native_logical_aliases")?;
    let mut alias_rows = aliases.query([])?;
    while let Some(row) = alias_rows.next()? {
        let fight: String = row.get(0)?;
        let run: String = row.get(1)?;
        let evidence: String = row.get(2)?;
        let session: String = row.get(3)?;
        let encounter: String = row.get(4)?;
        let hash: String = row.get(5)?;
        if [&fight, &run, &evidence, &hash]
            .iter()
            .any(|value| !is_sha256(value))
        {
            return invalid("native logical alias metadata is invalid");
        }
        let canonical:Option<Vec<u8>>=connection.query_row("SELECT canonical_json FROM native_encounters WHERE session_id=?1 AND encounter_id=?2 AND content_sha256=?3",params![session,encounter,hash],|r|r.get(0)).optional()?;
        if let Some(bytes) = canonical {
            let capture: NativeEncounter = serde_json::from_slice(&bytes).map_err(|_| {
                EncounterError::Validation("native alias evidence is invalid".into())
            })?;
            let actual = logical_native_identity(&capture)?;
            if actual.fight_key != fight
                || actual.run_key != run
                || actual.evidence_sha256 != evidence
                || sha256(&bytes) != hash
            {
                return invalid("native logical alias does not match evidence");
            }
        } else {
            let suppressed:bool=connection.query_row("SELECT EXISTS(SELECT 1 FROM native_deleted_encounters WHERE fight_key=?1 AND evidence_sha256=?2 AND session_id=?3 AND encounter_id=?4 AND content_sha256=?5)",params![fight,evidence,session,encounter,hash],|r|r.get(0))?;
            if !suppressed {
                return invalid("native logical alias has no retained evidence or suppression");
            }
        }
    }
    let mut deleted = connection.prepare(
        "SELECT fight_key,evidence_sha256,content_sha256 FROM native_deleted_encounters",
    )?;
    let mut deleted_rows = deleted.query([])?;
    while let Some(row) = deleted_rows.next()? {
        let fight: String = row.get(0)?;
        let evidence: String = row.get(1)?;
        let hash: String = row.get(2)?;
        if [&fight, &evidence, &hash]
            .iter()
            .any(|value| !is_sha256(value))
        {
            return invalid("native suppression metadata is invalid");
        }
        let alias:bool=connection.query_row("SELECT EXISTS(SELECT 1 FROM native_logical_aliases WHERE fight_key=?1 AND evidence_sha256=?2 AND content_sha256=?3)",params![fight,evidence,hash],|r|r.get(0))?;
        if !alias {
            return invalid("native suppression has no matching identity alias");
        }
    }
    let mut statement = connection.prepare("SELECT source_key FROM native_ingest_checkpoints")?;
    let mut rows = statement.query([])?;
    while let Some(row) = rows.next()? {
        let key: String = row.get(0)?;
        if !is_sha256(&key) {
            return invalid("native checkpoint source is invalid");
        }
        checkpoint(connection, &key)?;
    }
    let mut statement=connection.prepare("SELECT content_sha256,result_sha256,length(result_json),CASE WHEN length(result_json)<=67108864 THEN result_json END FROM encounter_derivations")?;
    let mut rows = statement.query([])?;
    while let Some(row) = rows.next()? {
        let raw: String = row.get(0)?;
        let hash: String = row.get(1)?;
        let size: i64 = row.get(2)?;
        let bytes: Option<Vec<u8>> = row.get(3)?;
        if !is_sha256(&raw) || !is_sha256(&hash) || !(0..=67108864).contains(&size) {
            return invalid("encounter derivation metadata is invalid");
        }
        let bytes = bytes.ok_or_else(|| {
            EncounterError::Validation("encounter derivation exceeds byte limit".into())
        })?;
        if sha256(&bytes) != hash || serde_json::from_slice::<serde_json::Value>(&bytes).is_err() {
            return invalid("encounter derivation content is invalid");
        }
    }
    Ok(())
}
