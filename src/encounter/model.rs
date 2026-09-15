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
    CaptureOverflow,
    ClockReset,
    UserStopped,
    PlayerDeactivated,
    CallbackFailed,
    UnsupportedValue,
    RecordLimit,
    ByteLimit,
    StringLimit,
}

impl PartialReason {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::CaptureOverflow => "capture-overflow",
            Self::ClockReset => "clock-reset",
            Self::UserStopped => "user-stopped",
            Self::PlayerDeactivated => "player-deactivated",
            Self::CallbackFailed => "callback-failed",
            Self::UnsupportedValue => "unsupported-value",
            Self::RecordLimit => "record-limit",
            Self::ByteLimit => "byte-limit",
            Self::StringLimit => "string-limit",
        }
    }
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
