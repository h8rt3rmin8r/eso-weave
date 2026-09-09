use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::catalog::model::{Redistribution, SourceChannel};
use crate::catalog::version::CatalogVersionTuple;
use crate::catalog::Channel;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(super) enum PipelineMode {
    Live,
    Pts,
    Offline,
    UserCapture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(super) enum SourceRole {
    NormalizedBundle,
    CollectorCapture,
    Provenance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SourcePin {
    pub(super) id: String,
    pub(super) role: SourceRole,
    pub(super) channel: SourceChannel,
    pub(super) game_version: String,
    pub(super) api_version: u32,
    pub(super) locale: String,
    pub(super) revision: String,
    pub(super) sha256: String,
    pub(super) max_bytes: u64,
    pub(super) uri: String,
    pub(super) local_path: Option<String>,
    pub(super) license_scope: String,
    pub(super) redistribution: Redistribution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct NetworkPolicy {
    pub(super) enabled: bool,
    pub(super) refresh: bool,
    pub(super) allow_stale_cache: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Thresholds {
    pub(super) entities_removed: usize,
    pub(super) localized_text_removed: usize,
    pub(super) relations_removed: usize,
    pub(super) coverage_removed: usize,
    pub(super) icon_references_removed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct PipelineRequest {
    pub(super) schema_version: u32,
    pub(super) mode: PipelineMode,
    pub(super) version: CatalogVersionTuple,
    pub(super) input_source: String,
    pub(super) source_policy: String,
    pub(super) sources: Vec<SourcePin>,
    pub(super) network: NetworkPolicy,
    pub(super) baseline: Option<String>,
    pub(super) icon_source: Option<String>,
    pub(super) thresholds: Thresholds,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct SourceInventory {
    pub(super) id: String,
    pub(super) role: SourceRole,
    pub(super) channel: SourceChannel,
    pub(super) game_version: String,
    pub(super) api_version: u32,
    pub(super) locale: String,
    pub(super) revision: String,
    pub(super) sha256: String,
    pub(super) byte_count: u64,
    pub(super) uri: String,
    pub(super) license_scope: String,
    pub(super) redistribution: Redistribution,
    pub(super) acquisition: String,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct ValidationSummary {
    pub(super) status: &'static str,
    pub(super) channel: Channel,
    pub(super) game_version: String,
    pub(super) api_version: u32,
    pub(super) catalog_version: String,
    pub(super) findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct IconSummary {
    pub(super) generation_sha256: String,
    pub(super) transformation_id: String,
    pub(super) entry_count: usize,
    pub(super) object_count: usize,
    pub(super) placeholder_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct ArtifactRecord {
    pub(super) path: String,
    pub(super) sha256: String,
    pub(super) bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct CandidateManifest {
    pub(super) schema_version: u32,
    pub(super) state: String,
    pub(super) version: CatalogVersionTuple,
    pub(super) request_sha256: String,
    pub(super) source_inventory_sha256: String,
    pub(super) catalog_semantic_sha256: String,
    pub(super) catalog_artifact_sha256: String,
    pub(super) baseline_semantic_sha256: Option<String>,
    pub(super) icon_generation_sha256: String,
    pub(super) thresholds: Thresholds,
    pub(super) artifacts: Vec<ArtifactRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CandidateReceipt {
    pub relative_path: PathBuf,
    pub candidate_sha256: String,
    pub catalog_semantic_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CandidateVerification {
    pub candidate_sha256: String,
    pub channel: Channel,
}
