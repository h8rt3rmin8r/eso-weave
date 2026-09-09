//! Bounded ESO catalog collection, hostile SavedVariables parsing, and staging.

pub mod import;
pub mod lifecycle;
mod parser;

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::catalog::{Channel, EntityKind};

pub use import::{import_capture, ImportReceipt};
pub use lifecycle::embedded_checksum;

pub const CAPTURE_SCHEMA_VERSION: u32 = 1;
pub const COLLECTOR_VERSION: u32 = 1;
pub const MAX_CAPTURE_BYTES: u64 = 64 * 1024 * 1024;
pub const MAX_CAPTURE_RECORDS: usize = 500_000;
pub const MAX_CAPTURE_STRING_BYTES: usize = 64 * 1024;
pub const MAX_CHUNK_BYTES: usize = 64 * 1024;
pub const MAX_CHUNKS: usize = 1024;
pub const MAX_PARSE_DEPTH: usize = 16;
pub const MAX_PARSE_TOKENS: usize = 1_000_000;
pub const MAX_TABLE_ENTRIES: usize = 600_000;

pub const CATEGORIES: [&str; 5] = [
    "player-skills",
    "crafted-abilities",
    "item-sets",
    "champion-skills",
    "companions-races-classes",
];

#[derive(thiserror::Error, Debug)]
pub enum CollectorError {
    #[error("collector I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("collector JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("collector capture rejected: {0}")]
    Validation(String),
    #[error("collector staging failed S070 validation: {0}")]
    Catalog(#[from] crate::catalog::CatalogError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CaptureStatus {
    Complete,
    Paused,
    Cancelled,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoverageDeclaration {
    pub category: String,
    pub completeness: String,
    pub scope: String,
    pub record_count: usize,
    pub limitations: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Checkpoint {
    pub adapter: usize,
    pub cursor: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CollectorChunk {
    pub sequence: usize,
    pub record_count: usize,
    pub byte_count: usize,
    pub checksum: String,
    pub payload: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordParent {
    pub relation: String,
    pub kind: EntityKind,
    pub stable_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CollectorRecord {
    pub category: String,
    pub kind: EntityKind,
    pub stable_id: i64,
    pub source_key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<RecordParent>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub attributes: BTreeMap<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CollectorEnvelope {
    pub schema_version: u32,
    pub collector_version: u32,
    pub collector_checksum: String,
    pub status: CaptureStatus,
    pub channel: Channel,
    pub game_version: String,
    pub api_version: u32,
    pub locale: String,
    pub platform: String,
    pub megaserver: String,
    pub scope_key: String,
    pub started_at: String,
    pub finished_at: String,
    pub selected_categories: Vec<String>,
    pub coverage: Vec<CoverageDeclaration>,
    pub warnings: Vec<String>,
    pub cancellation_reason: Option<String>,
    pub checkpoint: Checkpoint,
    pub chunks: Vec<CollectorChunk>,
    #[serde(skip)]
    pub records: Vec<CollectorRecord>,
}

pub struct ImportRequest {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub expected_channel: Channel,
    pub catalog_version: String,
}

impl ImportRequest {
    pub fn new(
        input_path: impl Into<PathBuf>,
        output_path: impl Into<PathBuf>,
        expected_channel: Channel,
        catalog_version: impl Into<String>,
    ) -> Self {
        Self {
            input_path: input_path.into(),
            output_path: output_path.into(),
            expected_channel,
            catalog_version: catalog_version.into(),
        }
    }
}

pub fn parse_capture(bytes: &[u8]) -> Result<CollectorEnvelope, CollectorError> {
    if bytes.len() as u64 > MAX_CAPTURE_BYTES {
        return invalid(format!(
            "capture exceeds the {MAX_CAPTURE_BYTES} byte limit"
        ));
    }
    let source = std::str::from_utf8(bytes)
        .map_err(|_| CollectorError::Validation("capture is not valid UTF-8".to_string()))?;
    let value = parser::parse_saved_variables(source)?;
    let mut envelope: CollectorEnvelope = serde_json::from_value(value)?;
    validate_envelope(&mut envelope)?;
    Ok(envelope)
}

fn validate_envelope(envelope: &mut CollectorEnvelope) -> Result<(), CollectorError> {
    if envelope.schema_version != CAPTURE_SCHEMA_VERSION {
        return invalid(format!(
            "unsupported capture schema {}, expected {CAPTURE_SCHEMA_VERSION}",
            envelope.schema_version
        ));
    }
    if envelope.collector_version != COLLECTOR_VERSION {
        return invalid(format!(
            "unsupported collector version {}, expected {COLLECTOR_VERSION}",
            envelope.collector_version
        ));
    }
    if envelope.collector_checksum != embedded_checksum() {
        return invalid("collector checksum does not match the embedded managed version");
    }
    if envelope.status != CaptureStatus::Complete {
        return invalid(format!(
            "capture status {:?} is incomplete and cannot enter staging",
            envelope.status
        ));
    }
    if envelope.api_version == 0
        || envelope.game_version.is_empty()
        || envelope.locale.len() < 2
        || envelope.scope_key.is_empty()
        || envelope.started_at.is_empty()
        || envelope.finished_at.is_empty()
    {
        return invalid("capture provenance is incomplete");
    }
    validate_all_strings(envelope)?;

    if envelope.selected_categories.is_empty()
        || envelope.selected_categories.len() > CATEGORIES.len()
    {
        return invalid("capture must select between one and five categories");
    }
    let selected: BTreeSet<_> = envelope.selected_categories.iter().cloned().collect();
    if selected.len() != envelope.selected_categories.len()
        || selected
            .iter()
            .any(|value| !CATEGORIES.contains(&value.as_str()))
    {
        return invalid("capture contains duplicate or unsupported selected categories");
    }
    envelope.selected_categories.sort();

    if envelope.chunks.len() > MAX_CHUNKS {
        return invalid("capture exceeds the chunk limit");
    }
    envelope.chunks.sort_by_key(|chunk| chunk.sequence);
    let mut records = Vec::new();
    for (index, chunk) in envelope.chunks.iter().enumerate() {
        if chunk.sequence != index + 1 {
            return invalid("chunk sequences must be unique and contiguous from one");
        }
        if chunk.payload.is_empty()
            || chunk.payload.ends_with('\n')
            || chunk.payload.len() > MAX_CHUNK_BYTES
            || chunk.byte_count != chunk.payload.len()
        {
            return invalid(format!(
                "chunk {} has invalid byte metadata",
                chunk.sequence
            ));
        }
        if chunk.checksum != adler32_hex(chunk.payload.as_bytes()) {
            return invalid(format!("chunk {} checksum mismatch", chunk.sequence));
        }
        let lines: Vec<_> = chunk.payload.split('\n').collect();
        if lines.len() != chunk.record_count || lines.iter().any(|line| line.is_empty()) {
            return invalid(format!("chunk {} record count mismatch", chunk.sequence));
        }
        for line in lines {
            let record: CollectorRecord = serde_json::from_str(line)?;
            validate_record(&record, &selected)?;
            records.push(record);
            if records.len() > MAX_CAPTURE_RECORDS {
                return invalid("capture exceeds the record limit");
            }
        }
    }
    records.sort_by(|left, right| {
        (&left.category, &left.kind, left.stable_id, &left.source_key).cmp(&(
            &right.category,
            &right.kind,
            right.stable_id,
            &right.source_key,
        ))
    });
    let mut source_keys = BTreeSet::new();
    let mut identities = BTreeSet::new();
    for record in &records {
        if !source_keys.insert(record.source_key.clone()) {
            return invalid(format!("duplicate source key {}", record.source_key));
        }
        if !identities.insert((record.kind, record.stable_id)) {
            return invalid(format!(
                "duplicate typed entity {} {}",
                record.kind.as_str(),
                record.stable_id
            ));
        }
    }
    for record in &records {
        if let Some(parent) = &record.parent {
            if !identities.contains(&(parent.kind, parent.stable_id)) {
                return invalid(format!(
                    "record {} references a missing parent",
                    record.source_key
                ));
            }
        }
    }

    if envelope.coverage.len() != selected.len() {
        return invalid("coverage must contain exactly one declaration per selection");
    }
    let mut covered = BTreeSet::new();
    for coverage in &envelope.coverage {
        if coverage.completeness != "bounded"
            || !selected.contains(&coverage.category)
            || !covered.insert(coverage.category.clone())
            || coverage.scope.is_empty()
            || coverage.limitations.is_empty()
        {
            return invalid("coverage is duplicate, unsupported, or not bounded");
        }
        let actual = records
            .iter()
            .filter(|record| record.category == coverage.category)
            .count();
        if coverage.record_count != actual {
            return invalid(format!("coverage count mismatch for {}", coverage.category));
        }
    }
    envelope
        .coverage
        .sort_by(|a, b| a.category.cmp(&b.category));
    envelope.records = records;
    Ok(())
}

fn validate_record(
    record: &CollectorRecord,
    selected: &BTreeSet<String>,
) -> Result<(), CollectorError> {
    if record.stable_id <= 0 || !selected.contains(&record.category) {
        return invalid("record has an invalid identity or unselected category");
    }
    let expected = format!(
        "{}/{}/{}",
        record.category,
        record.kind.as_str(),
        record.stable_id
    );
    if record.source_key != expected || !kind_allowed(&record.category, record.kind) {
        return invalid(format!(
            "record {} violates its category/type contract",
            record.source_key
        ));
    }
    if let Some(parent) = &record.parent {
        if parent.stable_id <= 0 || parent.relation.is_empty() {
            return invalid(format!(
                "record {} has an invalid parent",
                record.source_key
            ));
        }
    }
    for (name, value) in &record.attributes {
        let valid_value = match value {
            serde_json::Value::Bool(_) | serde_json::Value::String(_) => true,
            serde_json::Value::Number(number) => {
                number.as_i64().is_some() || number.as_u64().is_some()
            }
            _ => false,
        };
        if name.is_empty()
            || matches!(name.as_str(), "index" | "array-index" | "lua-index")
            || !valid_value
        {
            return invalid(format!(
                "record {} has an invalid attribute",
                record.source_key
            ));
        }
    }
    validate_record_strings(record)
}

fn validate_record_strings(record: &CollectorRecord) -> Result<(), CollectorError> {
    validate_capture_string(&record.category)?;
    validate_capture_string(&record.source_key)?;
    if let Some(parent) = &record.parent {
        validate_capture_string(&parent.relation)?;
    }
    for (name, value) in &record.attributes {
        validate_capture_string(name)?;
        if let serde_json::Value::String(value) = value {
            validate_capture_string(value)?;
        }
    }
    for value in [
        record.name.as_deref(),
        record.description.as_deref(),
        record.icon_path.as_deref(),
    ]
    .into_iter()
    .flatten()
    {
        validate_capture_string(value)?;
    }
    Ok(())
}

fn validate_capture_string(value: &str) -> Result<(), CollectorError> {
    if value.len() > MAX_CAPTURE_STRING_BYTES {
        return invalid(format!(
            "string exceeds the {MAX_CAPTURE_STRING_BYTES} byte limit"
        ));
    }
    Ok(())
}

fn kind_allowed(category: &str, kind: EntityKind) -> bool {
    use EntityKind::*;
    match category {
        "player-skills" => matches!(
            kind,
            SkillType | SkillLine | Ability | AbilityProgression | AbilityRank | AbilityMorph
        ),
        "crafted-abilities" => matches!(kind, CraftedAbility | Script | Ability),
        "item-sets" => matches!(
            kind,
            ItemSet | ItemSetPiece | ItemSetBonus | CollectionCategory
        ),
        "champion-skills" => matches!(kind, ChampionSkill | CollectionCategory),
        "companions-races-classes" => {
            matches!(kind, Class | Race | Companion | CollectionCategory)
        }
        _ => false,
    }
}

fn validate_all_strings(envelope: &CollectorEnvelope) -> Result<(), CollectorError> {
    validate_json_strings(&serde_json::to_value(envelope)?)
}

fn validate_json_strings(value: &serde_json::Value) -> Result<(), CollectorError> {
    let mut pending = vec![value];
    while let Some(value) = pending.pop() {
        match value {
            serde_json::Value::String(value) => validate_capture_string(value)?,
            serde_json::Value::Array(values) => pending.extend(values),
            serde_json::Value::Object(values) => pending.extend(values.values()),
            _ => {}
        }
    }
    Ok(())
}

pub fn adler32_hex(bytes: &[u8]) -> String {
    const MODULUS: u32 = 65_521;
    let mut a = 1_u32;
    let mut b = 0_u32;
    for byte in bytes {
        a = (a + u32::from(*byte)) % MODULUS;
        b = (b + a) % MODULUS;
    }
    format!("{:08x}", (b << 16) | a)
}

fn invalid<T>(message: impl Into<String>) -> Result<T, CollectorError> {
    Err(CollectorError::Validation(message.into()))
}
