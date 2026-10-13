//! Hostile encounter SavedVariables import and user-owned raw storage.

mod history;
mod metrics;
mod model;
mod native;
mod replay;
mod store;
mod validate;

use std::path::{Component, Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::bounded_file::{read_bounded_stable_snapshot, StableReadError};
use crate::catalog::Channel;
use crate::saved_variables::{self, EmptyTable, ParseLimits};

pub use history::{
    EncounterHistoryService, EncounterIdentity, HistoryDiagnostic, HistoryDiagnosticKind,
    SavedRecordingReadiness, SavedSourceInspection,
};
pub use metrics::{
    calculate_projection, canonical_projection_bytes, project_encounter, AbilityDamageShare,
    CatalogJoinReceipt, EffectUptime, EncounterProjection, LossRange, MetricQuality, MetricResult,
    OrderedCastSequence, ProjectionRequest, ALGORITHM_VERSION, PROJECTION_SCHEMA_VERSION,
};
pub use model::{
    BackupReceipt, CaptureControllerState, CaptureFailure, CaptureFailureReason,
    CaptureImportReport, CaptureInterruption, CaptureInterruptionReason, CaptureMode,
    CaptureSession, CaptureSessionStatus, CaptureStateSummary, CaptureStatus, CaptureStopReason,
    DeleteReceipt, EncounterCapture, EncounterEvent, EncounterModuleState, EncounterSourceKind,
    EncounterSummary, ImportOutcome, ImportReceipt, NormalizationProfile, OrderedEncounterCapture,
    ParsedCaptureSet, PartialReason, PayloadValue, RawLoss, RawLossReason, RawObservation,
    RawSourceKind, RawValue, RawValueType, SourceProvenance,
};
pub use native::{
    canonical_native_bytes, import_native_log, inspect_native_log, parse_native_log,
    project_native_encounter, read_native_source, validate_native_encounter, NativeEncounter,
    NativeHeader, NativeImportReceipt, NativeImportReport, NativeProjection, NativeRecord,
    NativeRun, NativeSourceInspection, NativeSourceReport, ParsedNativeLog, MAX_NATIVE_FILE_BYTES,
    NATIVE_SCHEMA_VERSION,
};
pub use replay::{assess_replay, ReplayAssessment};
pub use store::{
    backup_store, delete_all, delete_encounter, list_encounters, load_encounter,
    load_native_encounter,
};

pub const CAPTURE_SCHEMA_VERSION: u32 = 2;
pub const STORE_SCHEMA_VERSION: u32 = 6;
pub const CANONICAL_FORMAT_VERSION: u32 = 2;
pub const MAX_CAPTURE_BYTES: u64 = crate::data_addon::MAX_SAVED_VARIABLES_BYTES;
pub const MAX_EVENTS: usize = 100_000;
pub const MAX_ESTIMATED_BYTES: u64 = 32 * 1024 * 1024;
pub const MAX_STRING_BYTES: usize = 64 * 1024;
pub const MAX_PARSE_DEPTH: usize = 16;
pub const MAX_PARSE_TOKENS: usize = crate::data_addon::MAX_SAVED_VARIABLES_TOKENS;
pub const MAX_TABLE_ENTRIES: usize = crate::data_addon::MAX_SAVED_VARIABLES_ENTRIES;
pub const MAX_ACTORS: u64 = 4_096;
pub const MAX_SESSION_ENCOUNTERS: usize = 1_024;
pub const MAX_RETAINED_SESSIONS: usize = 32;
pub const MAX_SESSION_SEQUENCE: u64 = 9_007_199_254_740_991;
pub const MAX_SESSION_INTERRUPTION_MARKERS: usize = 1_024;

const ROOT: &str = "EsoWeaveDataSaved";

#[derive(thiserror::Error, Debug)]
pub enum EncounterError {
    #[error("encounter I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("encounter database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("encounter input rejected: {0}")]
    Validation(String),
    #[error("unsupported encounter capture version")]
    UnsupportedVersion,
    #[error("encounter source channel does not match the expected channel")]
    EnvironmentMismatch,
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
    let (source_sha256, parsed) = read_capture_set(request)?;
    if parsed.records.len() != 1 {
        return invalid("single encounter import requires exactly one terminal record");
    }
    let canonical = canonical_bytes(&parsed.records[0].capture)?;
    let content_sha256 = sha256(&canonical);
    let report = store::append_set(
        &request.store_path,
        &parsed,
        &[(canonical, content_sha256)],
        source_sha256,
    )?;
    report.receipts.into_iter().next().ok_or_else(|| {
        EncounterError::Validation("single encounter import produced no receipt".into())
    })
}

pub fn import_capture_set(request: &ImportRequest) -> Result<CaptureImportReport, EncounterError> {
    let (source_sha256, parsed) = read_capture_set(request)?;
    append_capture_set(request, source_sha256, parsed)
}

fn append_capture_set(
    request: &ImportRequest,
    source_sha256: String,
    parsed: ParsedCaptureSet,
) -> Result<CaptureImportReport, EncounterError> {
    let prepared = parsed
        .records
        .iter()
        .map(|record| {
            let canonical = canonical_bytes(&record.capture)?;
            let content_sha256 = sha256(&canonical);
            Ok((canonical, content_sha256))
        })
        .collect::<Result<Vec<_>, EncounterError>>()?;
    store::append_set(&request.store_path, &parsed, &prepared, source_sha256)
}

fn read_capture_set(request: &ImportRequest) -> Result<(String, ParsedCaptureSet), EncounterError> {
    ensure_distinct_paths(&request.input_path, &request.store_path)?;
    let (source_sha256, parsed, _) =
        read_saved_source(&request.input_path, request.expected_channel)?;
    Ok((source_sha256, parsed))
}

fn read_saved_source(
    input_path: &Path,
    expected_channel: Channel,
) -> Result<(String, ParsedCaptureSet, Option<std::time::SystemTime>), EncounterError> {
    let absolute_input = absolute_path(input_path)?;
    let parent = absolute_input
        .parent()
        .ok_or_else(|| EncounterError::Validation("encounter input has no parent".into()))?;
    let canonical_parent = std::fs::canonicalize(parent)?;
    let (bytes, modified) =
        read_bounded_stable_snapshot(&canonical_parent, &absolute_input, MAX_CAPTURE_BYTES)
            .map_err(map_stable_read)?;
    let source_sha256 = sha256(&bytes);
    let parsed = parse_capture_set(&bytes, expected_channel)?;
    Ok((source_sha256, parsed, modified))
}

fn capture_version(value: &serde_json::Value, field: &str) -> Result<u64, EncounterError> {
    value
        .get(field)
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| EncounterError::Validation("invalid capture version field".into()))
}

pub fn parse_capture(
    bytes: &[u8],
    expected_channel: Channel,
) -> Result<EncounterCapture, EncounterError> {
    let parsed = parse_capture_set(bytes, expected_channel)?;
    if parsed.records.len() != 1 {
        return invalid("single capture parser requires exactly one terminal record");
    }
    Ok(parsed
        .records
        .into_iter()
        .next()
        .expect("validated one record")
        .capture)
}

pub fn parse_capture_set(
    bytes: &[u8],
    expected_channel: Channel,
) -> Result<ParsedCaptureSet, EncounterError> {
    if bytes.len() as u64 > MAX_CAPTURE_BYTES {
        return invalid(format!(
            "capture exceeds the {MAX_CAPTURE_BYTES} byte limit"
        ));
    }
    let source = std::str::from_utf8(bytes)
        .map_err(|_| EncounterError::Validation("capture is not valid UTF-8".into()))?;
    let mut root = saved_variables::parse_assignment(
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
    if root.is_object()
        && (capture_version(&root, "schema_version")?
            != crate::data_addon::SAVED_VARIABLES_SCHEMA_VERSION
            || capture_version(&root, "addon_version")? != crate::data_addon::DATA_ADDON_VERSION)
    {
        return Err(EncounterError::UnsupportedVersion);
    }
    let mut value = crate::data_addon::take_module(&mut root, "encounter")
        .map_err(EncounterError::Validation)?;
    if value.get("state_schema_version").is_some() {
        let version = (
            capture_version(&value, "state_schema_version")?,
            capture_version(&value, "addon_version")?,
        );
        if !matches!(version, (1, 4) | (2, 5)) {
            return Err(EncounterError::UnsupportedVersion);
        }
        let extension_fields = [
            "retained_sessions",
            "session_sequence",
            "next_session_sequence",
        ];
        if version == (1, 4) && extension_fields.iter().any(|key| value.get(key).is_some()) {
            return invalid("legacy controller has retention fields");
        }
        if version == (2, 5) {
            let archives = value
                .get("retained_sessions")
                .and_then(serde_json::Value::as_object)
                .ok_or_else(|| EncounterError::Validation("invalid retained sessions".into()))?;
            if archives.values().any(|archive| {
                extension_fields
                    .iter()
                    .any(|key| archive.get(key).is_some())
            }) {
                return invalid("nested retained sessions are unsupported");
            }
        }
        normalize_state_empty_arrays(&mut value);
        let state: EncounterModuleState = serde_json::from_value(value)
            .map_err(|_| EncounterError::Validation("invalid encounter state schema".into()))?;
        return validate::validate_state(state, expected_channel);
    }
    if !matches!(
        (
            capture_version(&value, "schema_version")?,
            capture_version(&value, "addon_version")?
        ),
        (1, 1) | (2, 2 | 3)
    ) {
        return Err(EncounterError::UnsupportedVersion);
    }
    normalize_empty_raw_value_arrays(&mut value);
    let capture: EncounterCapture = serde_json::from_value(value)
        .map_err(|_| EncounterError::Validation("invalid encounter schema".into()))?;
    if capture.channel != expected_channel {
        return Err(EncounterError::EnvironmentMismatch);
    }
    validate::validate(&capture)?;
    replay::enforce(&capture)?;
    Ok(ParsedCaptureSet {
        records: vec![OrderedEncounterCapture {
            mode: CaptureMode::Single,
            ordinal: 1,
            capture,
        }],
        state: None,
        sessions: Vec::new(),
    })
}

fn normalize_state_empty_arrays(value: &mut serde_json::Value) {
    if let Some(archives) = value
        .get_mut("retained_sessions")
        .and_then(serde_json::Value::as_object_mut)
    {
        for archive in archives.values_mut() {
            normalize_state_empty_arrays(archive);
        }
    }
    for field in ["interruptions"] {
        if value
            .get(field)
            .and_then(serde_json::Value::as_object)
            .is_some_and(serde_json::Map::is_empty)
        {
            value[field] = serde_json::Value::Array(Vec::new());
        }
    }
    let Some(records) = value
        .get_mut("records")
        .and_then(serde_json::Value::as_object_mut)
    else {
        return;
    };
    for record in records.values_mut() {
        if let Some(capture) = record.get_mut("capture") {
            normalize_empty_raw_value_arrays(capture);
        }
    }
}

fn normalize_empty_raw_value_arrays(value: &mut serde_json::Value) {
    let Some(observations) = value
        .get_mut("raw_observations")
        .and_then(serde_json::Value::as_array_mut)
    else {
        return;
    };
    for observation in observations {
        let Some(values) = observation.get_mut("values") else {
            continue;
        };
        if values.as_object().is_some_and(serde_json::Map::is_empty) {
            *values = serde_json::Value::Array(Vec::new());
        }
    }
}

pub fn canonical_bytes(capture: &EncounterCapture) -> Result<Vec<u8>, EncounterError> {
    validate::validate(capture)?;
    replay::enforce(capture)?;
    serde_json::to_vec(capture)
        .map_err(|error| EncounterError::Validation(format!("canonicalization failed: {error}")))
}

pub(crate) fn sha256(bytes: &[u8]) -> String {
    crate::hash::hex_lower(Sha256::digest(bytes))
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
