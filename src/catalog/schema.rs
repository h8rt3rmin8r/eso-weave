//! SQLite schema, canonical semantic hashing, and integrity diagnostics.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use rusqlite::config::DbConfig;
use rusqlite::types::ValueRef;
use rusqlite::{Connection, Transaction};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::CatalogError;

pub const SCHEMA_VERSION: u32 = 1;
pub const APPLICATION_ID: u32 = 0x4553_4F57;

pub const SCHEMA_SQL: &str = r#"
CREATE TABLE schema_migration (
    version INTEGER PRIMARY KEY CHECK(version > 0),
    name TEXT NOT NULL UNIQUE,
    applied_by TEXT NOT NULL
) WITHOUT ROWID;

CREATE TABLE catalog_release (
    singleton INTEGER PRIMARY KEY CHECK(singleton = 1),
    schema_version INTEGER NOT NULL CHECK(schema_version = 1),
    catalog_version TEXT NOT NULL CHECK(length(catalog_version) > 0),
    channel TEXT NOT NULL CHECK(channel IN ('live', 'pts')),
    game_version TEXT NOT NULL CHECK(length(game_version) > 0),
    api_version INTEGER NOT NULL CHECK(api_version > 0),
    created_at TEXT NOT NULL CHECK(length(created_at) > 0),
    locales_json TEXT NOT NULL CHECK(json_valid(locales_json)),
    tool_version TEXT NOT NULL CHECK(length(tool_version) > 0),
    input_sha256 TEXT NOT NULL CHECK(length(input_sha256) = 64),
    source_set_sha256 TEXT NOT NULL CHECK(length(source_set_sha256) = 64),
    semantic_sha256 TEXT NOT NULL CHECK(length(semantic_sha256) IN (0, 64))
) WITHOUT ROWID;

CREATE TABLE source_snapshot (
    snapshot_id TEXT PRIMARY KEY CHECK(length(snapshot_id) > 0),
    family TEXT NOT NULL CHECK(length(family) > 0),
    channel TEXT NOT NULL CHECK(channel IN ('live', 'pts', 'not-applicable')),
    game_version TEXT NOT NULL,
    api_version INTEGER NOT NULL CHECK(api_version >= 0),
    locale TEXT NOT NULL CHECK(length(locale) > 0),
    revision TEXT NOT NULL CHECK(length(revision) > 0),
    raw_sha256 TEXT NOT NULL CHECK(length(raw_sha256) = 64),
    uri TEXT NOT NULL CHECK(length(uri) > 0),
    acquired_at TEXT NOT NULL CHECK(length(acquired_at) > 0),
    license_scope TEXT NOT NULL CHECK(length(license_scope) > 0),
    acquisition_method TEXT NOT NULL CHECK(length(acquisition_method) > 0),
    redistribution TEXT NOT NULL CHECK(redistribution IN (
        'allowed', 'attribution-required', 'user-generated-only', 'prohibited', 'unresolved'
    ))
) WITHOUT ROWID;

CREATE TABLE source_record (
    record_id TEXT PRIMARY KEY CHECK(length(record_id) > 0),
    snapshot_id TEXT NOT NULL REFERENCES source_snapshot(snapshot_id),
    category TEXT NOT NULL CHECK(length(category) > 0),
    source_key TEXT NOT NULL CHECK(length(source_key) > 0),
    content_sha256 TEXT NOT NULL CHECK(length(content_sha256) = 64),
    acquisition_method TEXT NOT NULL CHECK(length(acquisition_method) > 0),
    import_result TEXT NOT NULL CHECK(import_result IN ('accepted', 'observed-only')),
    UNIQUE(snapshot_id, category, source_key)
) WITHOUT ROWID;

CREATE TABLE coverage (
    category TEXT NOT NULL,
    snapshot_id TEXT NOT NULL REFERENCES source_snapshot(snapshot_id),
    locale TEXT NOT NULL,
    scope TEXT NOT NULL,
    completeness TEXT NOT NULL CHECK(completeness IN (
        'exhaustive', 'bounded', 'opportunistic', 'unknown'
    )),
    limits_text TEXT NOT NULL,
    PRIMARY KEY(category, snapshot_id, locale, scope),
    CHECK(completeness = 'exhaustive' OR length(limits_text) > 0)
) WITHOUT ROWID;

CREATE TABLE entity (
    kind TEXT NOT NULL CHECK(kind IN (
        'skill-type', 'skill-line', 'ability', 'ability-progression', 'ability-rank',
        'ability-morph', 'crafted-ability', 'script', 'effect', 'item', 'item-variant',
        'equipment-type', 'armor-type', 'weapon-type', 'quality', 'trait', 'enchantment',
        'item-set', 'item-set-piece', 'item-set-bonus', 'combat-stat', 'champion-skill',
        'mundus', 'food-drink', 'potion', 'poison', 'class', 'race', 'companion',
        'collection-category', 'constant'
    )),
    stable_id INTEGER NOT NULL CHECK(stable_id > 0),
    channel TEXT NOT NULL CHECK(channel IN ('live', 'pts')),
    api_version INTEGER NOT NULL CHECK(api_version > 0),
    observed_only INTEGER NOT NULL CHECK(observed_only IN (0, 1)),
    PRIMARY KEY(kind, stable_id, channel, api_version)
) WITHOUT ROWID;

CREATE TABLE entity_source (
    kind TEXT NOT NULL,
    stable_id INTEGER NOT NULL,
    channel TEXT NOT NULL,
    api_version INTEGER NOT NULL,
    record_id TEXT NOT NULL REFERENCES source_record(record_id),
    PRIMARY KEY(kind, stable_id, channel, api_version, record_id),
    FOREIGN KEY(kind, stable_id, channel, api_version)
        REFERENCES entity(kind, stable_id, channel, api_version)
) WITHOUT ROWID;

CREATE TABLE entity_attribute (
    kind TEXT NOT NULL,
    stable_id INTEGER NOT NULL,
    channel TEXT NOT NULL,
    api_version INTEGER NOT NULL,
    name TEXT NOT NULL CHECK(length(name) > 0),
    value_type TEXT NOT NULL CHECK(value_type IN ('integer', 'real', 'text', 'boolean', 'json')),
    integer_value INTEGER,
    real_value REAL,
    text_value TEXT,
    source_record_id TEXT NOT NULL REFERENCES source_record(record_id),
    PRIMARY KEY(kind, stable_id, channel, api_version, name),
    FOREIGN KEY(kind, stable_id, channel, api_version)
        REFERENCES entity(kind, stable_id, channel, api_version),
    CHECK(
        (value_type IN ('integer', 'boolean') AND integer_value IS NOT NULL AND real_value IS NULL AND text_value IS NULL)
        OR (value_type = 'real' AND integer_value IS NULL AND real_value IS NOT NULL AND text_value IS NULL)
        OR (value_type IN ('text', 'json') AND integer_value IS NULL AND real_value IS NULL AND text_value IS NOT NULL)
    ),
    CHECK(value_type != 'boolean' OR integer_value IN (0, 1))
) WITHOUT ROWID;

CREATE TABLE entity_relation (
    relation_kind TEXT NOT NULL CHECK(length(relation_kind) > 0),
    from_kind TEXT NOT NULL,
    from_stable_id INTEGER NOT NULL,
    to_kind TEXT NOT NULL,
    to_stable_id INTEGER NOT NULL,
    channel TEXT NOT NULL,
    api_version INTEGER NOT NULL,
    source_record_id TEXT NOT NULL REFERENCES source_record(record_id),
    PRIMARY KEY(relation_kind, from_kind, from_stable_id, to_kind, to_stable_id, channel, api_version),
    FOREIGN KEY(from_kind, from_stable_id, channel, api_version)
        REFERENCES entity(kind, stable_id, channel, api_version),
    FOREIGN KEY(to_kind, to_stable_id, channel, api_version)
        REFERENCES entity(kind, stable_id, channel, api_version)
) WITHOUT ROWID;

CREATE TABLE entity_alias (
    alias_kind TEXT NOT NULL CHECK(alias_kind IN ('renamed', 'morph', 'perfected', 'retired', 'superseded')),
    from_kind TEXT NOT NULL,
    from_stable_id INTEGER NOT NULL,
    to_kind TEXT NOT NULL,
    to_stable_id INTEGER NOT NULL,
    channel TEXT NOT NULL,
    api_version INTEGER NOT NULL,
    source_record_id TEXT NOT NULL REFERENCES source_record(record_id),
    PRIMARY KEY(alias_kind, from_kind, from_stable_id, to_kind, to_stable_id, channel, api_version),
    FOREIGN KEY(from_kind, from_stable_id, channel, api_version)
        REFERENCES entity(kind, stable_id, channel, api_version),
    FOREIGN KEY(to_kind, to_stable_id, channel, api_version)
        REFERENCES entity(kind, stable_id, channel, api_version)
) WITHOUT ROWID;

CREATE TABLE localized_text (
    kind TEXT NOT NULL,
    stable_id INTEGER NOT NULL,
    channel TEXT NOT NULL,
    api_version INTEGER NOT NULL,
    locale TEXT NOT NULL CHECK(length(locale) > 0),
    text_kind TEXT NOT NULL CHECK(length(text_kind) > 0),
    value TEXT NOT NULL,
    normalized_search TEXT,
    source_version TEXT NOT NULL CHECK(length(source_version) > 0),
    redistribution TEXT NOT NULL CHECK(redistribution IN (
        'allowed', 'attribution-required', 'user-generated-only'
    )),
    source_record_id TEXT NOT NULL REFERENCES source_record(record_id),
    PRIMARY KEY(kind, stable_id, channel, api_version, locale, text_kind),
    FOREIGN KEY(kind, stable_id, channel, api_version)
        REFERENCES entity(kind, stable_id, channel, api_version)
) WITHOUT ROWID;

CREATE TABLE icon_reference (
    kind TEXT NOT NULL,
    stable_id INTEGER NOT NULL,
    channel TEXT NOT NULL,
    api_version INTEGER NOT NULL,
    virtual_path TEXT NOT NULL CHECK(length(virtual_path) > 0),
    availability TEXT NOT NULL CHECK(availability IN ('reference-only', 'local', 'missing', 'placeholder')),
    source_record_id TEXT NOT NULL REFERENCES source_record(record_id),
    PRIMARY KEY(kind, stable_id, channel, api_version, virtual_path),
    FOREIGN KEY(kind, stable_id, channel, api_version)
        REFERENCES entity(kind, stable_id, channel, api_version)
) WITHOUT ROWID;

CREATE TABLE icon_asset (
    content_sha256 TEXT NOT NULL CHECK(length(content_sha256) = 64),
    transformation_id TEXT NOT NULL CHECK(length(transformation_id) > 0),
    media_type TEXT NOT NULL CHECK(length(media_type) > 0),
    width INTEGER NOT NULL CHECK(width > 0),
    height INTEGER NOT NULL CHECK(height > 0),
    origin TEXT NOT NULL CHECK(length(origin) > 0),
    attribution TEXT NOT NULL,
    availability TEXT NOT NULL CHECK(availability IN ('local', 'missing', 'placeholder')),
    redistribution TEXT NOT NULL CHECK(redistribution IN (
        'allowed', 'attribution-required', 'user-generated-only', 'prohibited', 'unresolved'
    )),
    PRIMARY KEY(content_sha256, transformation_id)
) WITHOUT ROWID;
"#;

const CANONICAL_QUERIES: [(&str, &str); 12] = [
    ("schema_migration", "SELECT version, name, applied_by FROM schema_migration ORDER BY version"),
    ("catalog_release", "SELECT singleton, schema_version, catalog_version, channel, game_version, api_version, created_at, locales_json, tool_version, input_sha256, source_set_sha256 FROM catalog_release ORDER BY singleton"),
    ("source_snapshot", "SELECT snapshot_id, family, channel, game_version, api_version, locale, revision, raw_sha256, uri, acquired_at, license_scope, acquisition_method, redistribution FROM source_snapshot ORDER BY snapshot_id"),
    ("source_record", "SELECT record_id, snapshot_id, category, source_key, content_sha256, acquisition_method, import_result FROM source_record ORDER BY record_id"),
    ("coverage", "SELECT category, snapshot_id, locale, scope, completeness, limits_text FROM coverage ORDER BY category, snapshot_id, locale, scope"),
    ("entity", "SELECT kind, stable_id, channel, api_version, observed_only FROM entity ORDER BY kind, stable_id, channel, api_version"),
    ("entity_source", "SELECT kind, stable_id, channel, api_version, record_id FROM entity_source ORDER BY kind, stable_id, channel, api_version, record_id"),
    ("entity_attribute", "SELECT kind, stable_id, channel, api_version, name, value_type, integer_value, real_value, text_value, source_record_id FROM entity_attribute ORDER BY kind, stable_id, channel, api_version, name"),
    ("entity_relation", "SELECT relation_kind, from_kind, from_stable_id, to_kind, to_stable_id, channel, api_version, source_record_id FROM entity_relation ORDER BY relation_kind, from_kind, from_stable_id, to_kind, to_stable_id, channel, api_version"),
    ("entity_alias", "SELECT alias_kind, from_kind, from_stable_id, to_kind, to_stable_id, channel, api_version, source_record_id FROM entity_alias ORDER BY alias_kind, from_kind, from_stable_id, to_kind, to_stable_id, channel, api_version"),
    ("localized_text", "SELECT kind, stable_id, channel, api_version, locale, text_kind, value, normalized_search, source_version, redistribution, source_record_id FROM localized_text ORDER BY kind, stable_id, channel, api_version, locale, text_kind"),
    ("icon_reference", "SELECT kind, stable_id, channel, api_version, virtual_path, availability, source_record_id FROM icon_reference ORDER BY kind, stable_id, channel, api_version, virtual_path"),
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdRange {
    pub min: i64,
    pub max: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatabaseDiagnostics {
    pub schema_version: u32,
    pub integrity_check: String,
    pub foreign_key_violations: usize,
    pub semantic_sha256: String,
    pub entity_counts: BTreeMap<String, usize>,
    pub coverage_counts: BTreeMap<String, usize>,
    pub id_ranges: BTreeMap<String, IdRange>,
}

pub fn configure_new_database(connection: &Connection) -> Result<(), CatalogError> {
    connection.execute_batch(&format!(
        "PRAGMA page_size=4096;\
         PRAGMA auto_vacuum=NONE;\
         PRAGMA encoding='UTF-8';\
         PRAGMA journal_mode=DELETE;\
         PRAGMA synchronous=FULL;\
         PRAGMA foreign_keys=ON;\
         PRAGMA application_id={APPLICATION_ID};"
    ))?;
    Ok(())
}

pub fn create_schema(transaction: &Transaction<'_>) -> Result<(), CatalogError> {
    transaction.execute_batch(SCHEMA_SQL)?;
    transaction.pragma_update(None, "user_version", SCHEMA_VERSION)?;
    transaction.execute(
        "INSERT INTO schema_migration(version, name, applied_by) VALUES (?1, ?2, ?3)",
        (SCHEMA_VERSION, "initial-catalog", "s070-v1"),
    )?;
    Ok(())
}

pub fn verify_connection(connection: &Connection) -> Result<DatabaseDiagnostics, CatalogError> {
    let schema_version: u32 =
        connection.pragma_query_value(None, "user_version", |row| row.get(0))?;
    if schema_version != SCHEMA_VERSION {
        return Err(CatalogError::IncompatibleSchema {
            found: schema_version,
            supported: SCHEMA_VERSION,
        });
    }
    let application_id: u32 =
        connection.pragma_query_value(None, "application_id", |row| row.get(0))?;
    if application_id != APPLICATION_ID {
        return Err(CatalogError::Validation(format!(
            "unexpected SQLite application id {application_id}"
        )));
    }
    let integrity_check: String =
        connection.pragma_query_value(None, "integrity_check", |row| row.get(0))?;
    if integrity_check != "ok" {
        return Err(CatalogError::Validation(format!(
            "integrity_check failed: {integrity_check}"
        )));
    }
    let foreign_key_violations: i64 =
        connection.query_row("SELECT count(*) FROM pragma_foreign_key_check", [], |row| {
            row.get(0)
        })?;
    if foreign_key_violations != 0 {
        return Err(CatalogError::Validation(format!(
            "foreign_key_check found {foreign_key_violations} violations"
        )));
    }
    let expected: String = connection.query_row(
        "SELECT semantic_sha256 FROM catalog_release WHERE singleton = 1",
        [],
        |row| row.get(0),
    )?;
    let semantic_sha256 = semantic_sha256(connection)?;
    if expected != semantic_sha256 {
        return Err(CatalogError::Checksum {
            expected,
            actual: semantic_sha256,
        });
    }
    let foreign_key_violations = usize::try_from(foreign_key_violations)
        .map_err(|_| CatalogError::Validation("invalid foreign-key count".to_string()))?;
    Ok(DatabaseDiagnostics {
        schema_version,
        integrity_check,
        foreign_key_violations,
        semantic_sha256,
        entity_counts: grouped_counts(connection, "entity", "kind")?,
        coverage_counts: grouped_counts(connection, "coverage", "completeness")?,
        id_ranges: id_ranges(connection)?,
    })
}

pub fn semantic_sha256(connection: &Connection) -> Result<String, CatalogError> {
    let mut hasher = Sha256::new();
    for (table, query) in CANONICAL_QUERIES {
        hash_bytes(&mut hasher, table.as_bytes());
        let mut statement = connection.prepare(query)?;
        let columns = statement.column_count();
        let mut rows = statement.query([])?;
        while let Some(row) = rows.next()? {
            hasher.update([0x52]);
            for index in 0..columns {
                hash_value(&mut hasher, row.get_ref(index)?);
            }
        }
    }
    hash_table(
        connection,
        &mut hasher,
        "icon_asset",
        "SELECT content_sha256, transformation_id, media_type, width, height, origin, attribution, availability, redistribution FROM icon_asset ORDER BY content_sha256, transformation_id",
    )?;
    Ok(hex_digest(hasher.finalize()))
}

pub fn artifact_sha256(path: &Path) -> Result<String, CatalogError> {
    let mut reader = BufReader::new(File::open(path)?);
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(hex_digest(hasher.finalize()))
}

fn hash_table(
    connection: &Connection,
    hasher: &mut Sha256,
    table: &str,
    query: &str,
) -> Result<(), CatalogError> {
    hash_bytes(hasher, table.as_bytes());
    let mut statement = connection.prepare(query)?;
    let columns = statement.column_count();
    let mut rows = statement.query([])?;
    while let Some(row) = rows.next()? {
        hasher.update([0x52]);
        for index in 0..columns {
            hash_value(hasher, row.get_ref(index)?);
        }
    }
    Ok(())
}

fn hash_value(hasher: &mut Sha256, value: ValueRef<'_>) {
    match value {
        ValueRef::Null => hasher.update([0x4E]),
        ValueRef::Integer(value) => {
            hasher.update([0x49]);
            hasher.update(value.to_be_bytes());
        }
        ValueRef::Real(value) => {
            hasher.update([0x52]);
            hasher.update(value.to_bits().to_be_bytes());
        }
        ValueRef::Text(value) => {
            hasher.update([0x54]);
            hash_bytes(hasher, value);
        }
        ValueRef::Blob(value) => {
            hasher.update([0x42]);
            hash_bytes(hasher, value);
        }
    }
}

fn hash_bytes(hasher: &mut Sha256, value: &[u8]) {
    hasher.update((value.len() as u64).to_be_bytes());
    hasher.update(value);
}

fn hex_digest(bytes: impl AsRef<[u8]>) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let bytes = bytes.as_ref();
    let mut result = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        result.push(HEX[(byte >> 4) as usize] as char);
        result.push(HEX[(byte & 0x0f) as usize] as char);
    }
    result
}

fn grouped_counts(
    connection: &Connection,
    table: &str,
    column: &str,
) -> Result<BTreeMap<String, usize>, CatalogError> {
    let sql = format!("SELECT {column}, count(*) FROM {table} GROUP BY {column} ORDER BY {column}");
    let mut statement = connection.prepare(&sql)?;
    let values = statement.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
    })?;
    let mut counts = BTreeMap::new();
    for value in values {
        let (key, count) = value?;
        let count = usize::try_from(count)
            .map_err(|_| CatalogError::Validation("invalid grouped count".to_string()))?;
        counts.insert(key, count);
    }
    Ok(counts)
}

fn id_ranges(connection: &Connection) -> Result<BTreeMap<String, IdRange>, CatalogError> {
    let mut statement = connection.prepare(
        "SELECT kind, min(stable_id), max(stable_id) FROM entity GROUP BY kind ORDER BY kind",
    )?;
    let values = statement.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            IdRange {
                min: row.get(1)?,
                max: row.get(2)?,
            },
        ))
    })?;
    values
        .map(|value| value.map_err(CatalogError::from))
        .collect()
}

pub fn open_read_only(path: &Path) -> Result<Connection, CatalogError> {
    let flags = rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY
        | rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX
        | rusqlite::OpenFlags::SQLITE_OPEN_URI;
    let connection = Connection::open_with_flags(path, flags)?;
    connection.set_db_config(DbConfig::SQLITE_DBCONFIG_DEFENSIVE, true)?;
    connection.set_db_config(DbConfig::SQLITE_DBCONFIG_TRUSTED_SCHEMA, false)?;
    connection.pragma_update(None, "query_only", true)?;
    connection.pragma_update(None, "foreign_keys", true)?;
    Ok(connection)
}

pub fn canonical_row(value: ValueRef<'_>) -> String {
    match value {
        ValueRef::Null => "null".to_string(),
        ValueRef::Integer(value) => format!("i:{value}"),
        ValueRef::Real(value) => format!("r:{:016x}", value.to_bits()),
        ValueRef::Text(value) => format!("t:{}", String::from_utf8_lossy(value)),
        ValueRef::Blob(value) => format!("b:{}", hex_digest(value)),
    }
}
