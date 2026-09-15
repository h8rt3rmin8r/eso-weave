use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::catalog::Channel;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CaptureStatus {
    Complete,
    Partial,
}

impl CaptureStatus {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PartialReason {
    StartedMidCombat,
    CaptureOverflow,
    ClockReset,
    UserStopped,
    PlayerDeactivated,
    RuntimeInterrupted,
    CallbackFailed,
    UnsupportedValue,
    RecordLimit,
    ByteLimit,
    StringLimit,
}

impl PartialReason {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::StartedMidCombat => "started-mid-combat",
            Self::CaptureOverflow => "capture-overflow",
            Self::ClockReset => "clock-reset",
            Self::UserStopped => "user-stopped",
            Self::PlayerDeactivated => "player-deactivated",
            Self::RuntimeInterrupted => "runtime-interrupted",
            Self::CallbackFailed => "callback-failed",
            Self::UnsupportedValue => "unsupported-value",
            Self::RecordLimit => "record-limit",
            Self::ByteLimit => "byte-limit",
            Self::StringLimit => "string-limit",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CaptureMode {
    Single,
    Continuous,
}

impl CaptureMode {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Continuous => "continuous",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CaptureControllerState {
    Stopped,
    Waiting,
    Capturing,
    Interrupted,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CaptureSessionStatus {
    Active,
    Stopped,
    Failed,
}

impl CaptureSessionStatus {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Stopped => "stopped",
            Self::Failed => "failed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CaptureStopReason {
    NeverStarted,
    SingleComplete,
    SinglePartial,
    SingleInterrupted,
    UserDisabled,
    Cleared,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CaptureFailureReason {
    StoragePressure,
    CallbackFailed,
    ClockReset,
    TerminalReserveExhausted,
    InterruptionLimit,
    StateInvalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CaptureInterruptionReason {
    PlayerDeactivated,
    RuntimeInterrupted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CaptureSession {
    pub session_id: String,
    pub mode: CaptureMode,
    pub channel: Channel,
    pub status: CaptureSessionStatus,
    pub started_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
    pub next_encounter_ordinal: u64,
    pub completed_encounter_count: usize,
    pub degraded_encounter_count: usize,
    pub aggregate_estimated_bytes: u64,
    pub aggregate_event_count: usize,
    pub aggregate_raw_observation_count: usize,
    pub interruption_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionEncounterRecord {
    pub ordinal: u64,
    pub capture: EncounterCapture,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CaptureInterruption {
    pub sequence: u64,
    pub occurred_at: String,
    pub after_encounter_ordinal: u64,
    pub reason: CaptureInterruptionReason,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CaptureFailure {
    pub reason: CaptureFailureReason,
    pub occurred_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encounter_ordinal: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EncounterModuleState {
    pub state_schema_version: u32,
    pub addon_version: u32,
    pub selected_mode: CaptureMode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_channel: Option<Channel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_mode: Option<CaptureMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_mode: Option<CaptureMode>,
    pub state: CaptureControllerState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_encounter_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<CaptureSession>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current: Option<serde_json::Value>,
    pub records: BTreeMap<String, SessionEncounterRecord>,
    pub interruptions: Vec<CaptureInterruption>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_reason: Option<CaptureStopReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure: Option<CaptureFailure>,
    pub revision: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OrderedEncounterCapture {
    pub mode: CaptureMode,
    pub ordinal: u64,
    pub capture: EncounterCapture,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ParsedCaptureSet {
    pub state: Option<EncounterModuleState>,
    pub records: Vec<OrderedEncounterCapture>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceProvenance {
    pub api_version: u32,
    pub game_version: String,
    pub locale: String,
    pub platform: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PayloadValue {
    Integer(i64),
    Bool(bool),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EncounterEvent {
    pub session_id: String,
    pub encounter_id: String,
    pub sequence: u64,
    pub monotonic_ms: u64,
    pub kind: String,
    pub payload: BTreeMap<String, PayloadValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_sequence: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projection_ordinal: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RawSourceKind {
    Callback,
    ApiSample,
    Lifecycle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RawValueType {
    Nil,
    Boolean,
    String,
    Number,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawValue {
    pub position: usize,
    pub value_type: RawValueType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boolean: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sign: Option<i8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub significand: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exponent: Option<i16>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawObservation {
    pub session_id: String,
    pub encounter_id: String,
    pub sequence: u64,
    pub monotonic_ms: u64,
    pub api_version: u32,
    pub source_kind: RawSourceKind,
    pub source_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_code: Option<i64>,
    pub source_version: u32,
    pub argument_count: usize,
    pub return_count: usize,
    pub values: Vec<RawValue>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RawLossReason {
    RecordLimit,
    ByteLimit,
    StringLimit,
    UnsupportedValue,
    CallbackFailed,
}

impl RawLossReason {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::RecordLimit => "record-limit",
            Self::ByteLimit => "byte-limit",
            Self::StringLimit => "string-limit",
            Self::UnsupportedValue => "unsupported-value",
            Self::CallbackFailed => "callback-failed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawLoss {
    pub missing_sequence_from: u64,
    pub missing_sequence_to: u64,
    pub reason: RawLossReason,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NormalizationProfile {
    pub version: u32,
    pub api_version: u32,
    pub player_combat_unit_type: i64,
    pub health_power_type: i64,
    pub quickslot_category: i64,
    pub damage_results: Vec<i64>,
    pub healing_results: Vec<i64>,
    pub death_results: Vec<i64>,
    pub resurrect_result: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EncounterCapture {
    pub schema_version: u32,
    pub addon_version: u32,
    pub status: CaptureStatus,
    pub channel: Channel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy_profile: Option<String>,
    pub source: SourceProvenance,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normalization_profile: Option<NormalizationProfile>,
    pub session_id: String,
    pub encounter_id: String,
    pub started_at: String,
    pub finished_at: String,
    pub started_monotonic_ms: u64,
    pub ended_monotonic_ms: u64,
    pub first_sequence: u64,
    pub last_sequence: u64,
    pub stored_event_count: usize,
    pub omitted_event_count: u64,
    pub estimated_bytes: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partial_reason: Option<PartialReason>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub warnings: BTreeMap<String, u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_first_sequence: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_last_sequence: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_observation_count: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_omitted_observation_count: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_loss: Option<RawLoss>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub raw_observations: Vec<RawObservation>,
    pub events: Vec<EncounterEvent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ImportOutcome {
    Imported,
    AlreadyPresent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ImportReceipt {
    pub outcome: ImportOutcome,
    pub source_sha256: String,
    pub content_sha256: String,
    pub channel: Channel,
    pub status: CaptureStatus,
    pub session_id: String,
    pub encounter_id: String,
    pub stored_event_count: usize,
    pub omitted_event_count: u64,
    pub capture_mode: CaptureMode,
    pub encounter_ordinal: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CaptureImportReport {
    pub source_sha256: String,
    pub imported_count: usize,
    pub already_present_count: usize,
    pub receipts: Vec<ImportReceipt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_saved_state: Option<CaptureStateSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CaptureStateSummary {
    pub selected_mode: CaptureMode,
    pub selected_channel: Option<Channel>,
    pub state: CaptureControllerState,
    pub revision: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_status: Option<CaptureSessionStatus>,
    pub completed_encounter_count: usize,
    pub degraded_encounter_count: usize,
    pub interruption_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<CaptureFailureReason>,
}

impl From<&EncounterModuleState> for CaptureStateSummary {
    fn from(state: &EncounterModuleState) -> Self {
        Self {
            selected_mode: state.selected_mode,
            selected_channel: state.selected_channel,
            state: state.state,
            revision: state.revision,
            session_id: state
                .session
                .as_ref()
                .map(|session| session.session_id.clone()),
            session_status: state.session.as_ref().map(|session| session.status),
            completed_encounter_count: state
                .session
                .as_ref()
                .map_or(0, |session| session.completed_encounter_count),
            degraded_encounter_count: state
                .session
                .as_ref()
                .map_or(0, |session| session.degraded_encounter_count),
            interruption_count: state
                .session
                .as_ref()
                .map_or(0, |session| session.interruption_count),
            failure_reason: state.failure.as_ref().map(|failure| failure.reason),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EncounterSummary {
    pub content_sha256: String,
    pub session_id: String,
    pub encounter_id: String,
    pub channel: Channel,
    pub status: CaptureStatus,
    pub started_at: String,
    pub finished_at: String,
    pub first_sequence: u64,
    pub last_sequence: u64,
    pub stored_event_count: usize,
    pub omitted_event_count: u64,
    pub capture_mode: CaptureMode,
    pub encounter_ordinal: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SessionEncounterReference {
    pub ordinal: u64,
    pub encounter_id: String,
    pub content_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SessionSnapshot {
    pub session_id: String,
    pub revision: u64,
    pub channel: Channel,
    pub mode: CaptureMode,
    pub disposition: CaptureSessionStatus,
    pub started_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
    pub interruptions: Vec<CaptureInterruption>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure: Option<CaptureFailure>,
    pub encounters: Vec<SessionEncounterReference>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeleteReceipt {
    pub deleted_records: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BackupReceipt {
    pub schema_version: u32,
    pub byte_length: u64,
    pub sha256: String,
}
