use serde::{Deserialize, Serialize};

use crate::catalog::Channel;

use super::UpdateError;

pub const SELECTION_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum CatalogTarget {
    Bundled,
    User { candidate_sha256: String },
}

impl CatalogTarget {
    pub(crate) fn validate(&self) -> Result<(), UpdateError> {
        if let Self::User { candidate_sha256 } = self {
            if candidate_sha256.len() != 64
                || !candidate_sha256
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
            {
                return Err(UpdateError::InvalidSelection(
                    "user candidate identity must be a lowercase SHA-256 digest".into(),
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CatalogSelection {
    pub schema_version: u32,
    pub generation: u64,
    pub active: CatalogTarget,
    pub previous: Option<CatalogTarget>,
    pub last_receipt: Option<u64>,
}

impl Default for CatalogSelection {
    fn default() -> Self {
        Self {
            schema_version: SELECTION_SCHEMA_VERSION,
            generation: 0,
            active: CatalogTarget::Bundled,
            previous: None,
            last_receipt: None,
        }
    }
}

impl CatalogSelection {
    pub(crate) fn validate(&self) -> Result<(), UpdateError> {
        if self.schema_version != SELECTION_SCHEMA_VERSION {
            return Err(UpdateError::InvalidSelection(format!(
                "selection schema {} is unsupported",
                self.schema_version
            )));
        }
        self.active.validate()?;
        if let Some(previous) = &self.previous {
            previous.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UpdateStage {
    Checking,
    LocatingSources,
    WaitingForCapture,
    Validating,
    Normalizing,
    Building,
    ResolvingIcons,
    IntegrityChecking,
    WaitingForLock,
    Installing,
    Opening,
    Committing,
    RollingBack,
    Complete,
    Cancelled,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UpdateOperation {
    Install,
    CollectorBuild,
    Rollback,
    Recovery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UpdateResult {
    Complete,
    Cancelled,
    Failed,
    Interrupted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateReceipt {
    pub schema_version: u32,
    pub operation: UpdateOperation,
    pub sequence: u64,
    pub result: UpdateResult,
    pub stage: UpdateStage,
    pub old_target: CatalogTarget,
    pub new_target: Option<CatalogTarget>,
    pub candidate_sha256: Option<String>,
    pub catalog_semantic_sha256: Option<String>,
    pub channel: Option<Channel>,
    pub catalog_version: Option<String>,
    pub game_version: Option<String>,
    pub api_version: Option<u32>,
    pub source_count: usize,
    pub finding_codes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateProgress {
    pub stage: UpdateStage,
    pub completed_bytes: u64,
    pub total_bytes: u64,
    pub message: String,
    pub elapsed_millis: u64,
}

impl UpdateProgress {
    pub(crate) fn new(
        stage: UpdateStage,
        completed_bytes: u64,
        total_bytes: u64,
        message: impl Into<String>,
    ) -> Self {
        Self {
            stage,
            completed_bytes,
            total_bytes,
            message: message.into(),
            elapsed_millis: 0,
        }
    }
}
