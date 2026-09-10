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
}

impl PartialReason {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::CaptureOverflow => "capture-overflow",
            Self::ClockReset => "clock-reset",
            Self::UserStopped => "user-stopped",
            Self::PlayerDeactivated => "player-deactivated",
            Self::CallbackFailed => "callback-failed",
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EncounterCapture {
    pub schema_version: u32,
    pub addon_version: u32,
    pub status: CaptureStatus,
    pub channel: Channel,
    pub privacy_profile: String,
    pub source: SourceProvenance,
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
