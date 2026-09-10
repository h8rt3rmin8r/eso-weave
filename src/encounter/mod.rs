//! Hostile encounter SavedVariables import and user-owned raw storage.

mod metrics;
mod model;
mod store;
mod validate;

use std::path::{Component, Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::bounded_file::{read_bounded_stable, StableReadError};
use crate::catalog::Channel;
use crate::saved_variables::{self, EmptyTable, ParseLimits};

pub use metrics::{
    calculate_projection, canonical_projection_bytes, project_encounter, AbilityDamageShare,
    CatalogJoinReceipt, EffectUptime, EncounterProjection, LossRange, MetricQuality, MetricResult,
    OrderedCastSequence, ProjectionRequest, ALGORITHM_VERSION, PROJECTION_SCHEMA_VERSION,
};
pub use model::{
    BackupReceipt, CaptureStatus, DeleteReceipt, EncounterCapture, EncounterEvent,
    EncounterSummary, ImportOutcome, ImportReceipt, PartialReason, PayloadValue, SourceProvenance,
};
pub use store::{backup_store, delete_all, delete_encounter, list_encounters, load_encounter};

pub const CAPTURE_SCHEMA_VERSION: u32 = 1;
pub const STORE_SCHEMA_VERSION: u32 = 1;
pub const CANONICAL_FORMAT_VERSION: u32 = 1;
pub const MAX_CAPTURE_BYTES: u64 = 64 * 1024 * 1024;
pub const MAX_EVENTS: usize = 100_000;
pub const MAX_ESTIMATED_BYTES: u64 = 32 * 1024 * 1024;
pub const MAX_STRING_BYTES: usize = 64 * 1024;
pub const MAX_PARSE_DEPTH: usize = 16;
pub const MAX_PARSE_TOKENS: usize = 3_000_000;
pub const MAX_TABLE_ENTRIES: usize = 2_000_000;
pub const MAX_ACTORS: u64 = 4_096;

const ROOT: &str = "EsoWeaveEncounterSaved";

#[derive(thiserror::Error, Debug)]
pub enum EncounterError {
    #[error("encounter I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("encounter database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("encounter input rejected: {0}")]
    Validation(String),
}

#[derive(Debug, Clone)]
pub struct ImportRequest {
    pub input_path: PathBuf,
    pub store_path: PathBuf,
    pub expected_channel: Channel,
}

impl ImportRequest {
    pub fn new(
        input_path: impl Into<PathBuf>,
        store_path: impl Into<PathBuf>,
        expected_channel: Channel,
    ) -> Self {
        Self {
            input_path: input_path.into(),
            store_path: store_path.into(),
            expected_channel,
        }
    }
}

pub fn import_encounter(request: &ImportRequest) -> Result<ImportReceipt, EncounterError> {
    ensure_distinct_paths(&request.input_path, &request.store_path)?;
    let absolute_input = absolute_path(&request.input_path)?;
    let parent = absolute_input
        .parent()
        .ok_or_else(|| EncounterError::Validation("encounter input has no parent".into()))?;
    let canonical_parent = std::fs::canonicalize(parent)?;
    let bytes = read_bounded_stable(&canonical_parent, &absolute_input, MAX_CAPTURE_BYTES)
        .map_err(map_stable_read)?;
    let source_sha256 = sha256(&bytes);
    let capture = parse_capture(&bytes, request.expected_channel)?;
    let canonical = canonical_bytes(&capture)?;
    let content_sha256 = sha256(&canonical);
    store::append(
        &request.store_path,
        &capture,
        &canonical,
        source_sha256,
        content_sha256,
    )
}

pub fn parse_capture(
    bytes: &[u8],
    expected_channel: Channel,
) -> Result<EncounterCapture, EncounterError> {
    if bytes.len() as u64 > MAX_CAPTURE_BYTES {
        return invalid(format!(
            "capture exceeds the {MAX_CAPTURE_BYTES} byte limit"
        ));
    }
    let source = std::str::from_utf8(bytes)
        .map_err(|_| EncounterError::Validation("capture is not valid UTF-8".into()))?;
    let value = saved_variables::parse_assignment(
        source,
        ROOT,
        ParseLimits {
            max_depth: MAX_PARSE_DEPTH,
            max_tokens: MAX_PARSE_TOKENS,
            max_entries: MAX_TABLE_ENTRIES,
            max_string_bytes: MAX_STRING_BYTES,
        },
        EmptyTable::Object,
    )
    .map_err(|error| EncounterError::Validation(error.to_string()))?;
    let capture: EncounterCapture = serde_json::from_value(value).map_err(|error| {
        EncounterError::Validation(format!("invalid encounter schema: {error}"))
    })?;
    if capture.channel != expected_channel {
        return invalid(format!(
            "capture channel {} does not match expected {}",
            capture.channel, expected_channel
        ));
    }
    validate::validate(&capture)?;
    Ok(capture)
}

pub fn canonical_bytes(capture: &EncounterCapture) -> Result<Vec<u8>, EncounterError> {
    validate::validate(capture)?;
    serde_json::to_vec(capture)
        .map_err(|error| EncounterError::Validation(format!("canonicalization failed: {error}")))
}

pub(crate) fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub(crate) fn invalid<T>(message: impl Into<String>) -> Result<T, EncounterError> {
    Err(EncounterError::Validation(message.into()))
}

pub(crate) fn ensure_distinct_paths(left: &Path, right: &Path) -> Result<(), EncounterError> {
    let left = comparable_path(left)?;
    let right = comparable_path(right)?;
    if same_path(&left, &right) {
        invalid("encounter input, store, and backup paths must be distinct")
    } else {
        Ok(())
    }
}

fn absolute_path(path: &Path) -> Result<PathBuf, EncounterError> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}

fn comparable_path(path: &Path) -> Result<PathBuf, EncounterError> {
    let absolute = absolute_path(path)?;
    if absolute.exists() {
        return Ok(std::fs::canonicalize(absolute)?);
    }
    let mut ancestor = absolute.as_path();
    let mut suffix = Vec::new();
    while !ancestor.exists() {
        suffix.push(
            ancestor
                .file_name()
                .ok_or_else(|| EncounterError::Validation("path has no existing ancestor".into()))?
                .to_os_string(),
        );
        ancestor = ancestor
            .parent()
            .ok_or_else(|| EncounterError::Validation("path has no existing ancestor".into()))?;
    }
    let mut resolved = std::fs::canonicalize(ancestor)?;
    for component in suffix.into_iter().rev() {
        resolved.push(component);
    }
    Ok(normalize_lexically(resolved))
}

fn normalize_lexically(path: PathBuf) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            Component::Prefix(_) | Component::RootDir | Component::Normal(_) => {
                normalized.push(component.as_os_str());
            }
        }
    }
    normalized
}

fn same_path(left: &Path, right: &Path) -> bool {
    #[cfg(windows)]
    {
        left.to_string_lossy().to_lowercase() == right.to_string_lossy().to_lowercase()
    }
    #[cfg(not(windows))]
    {
        left == right
    }
}

fn map_stable_read(error: StableReadError) -> EncounterError {
    match error {
        StableReadError::Io(error) => EncounterError::Io(error),
        StableReadError::Invalid(message) => EncounterError::Validation(message.into()),
    }
}
