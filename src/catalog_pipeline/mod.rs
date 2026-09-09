//! Reviewed, maintainer-only catalog candidate orchestration.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::bounded_file::{is_link_like, is_same_or_nested, read_bounded_stable, StableReadError};
use crate::catalog::compiler::{
    build_catalog, diff_catalogs, verify_catalog, BuildRequest, CatalogDiff,
};
use crate::catalog::model::{CatalogBundle, SourceChannel, MAX_INPUT_BYTES};
use crate::catalog::version::parse_commit_message_version;
use crate::catalog::{CatalogError, Channel};
use crate::collector::{import_capture, CollectorError, ImportRequest};
use crate::icon_cache::{
    build_generation, publish_staged_generation, IconCacheError, IconCacheRequest,
};

mod acquire;
mod manifest;

pub use acquire::SourceFetcher;
use acquire::{acquire_sources, publish_source_cache, AcquiredSource, HttpsFetcher};
use manifest::{
    ArtifactRecord, CandidateManifest, IconSummary, PipelineMode, PipelineRequest, SourceRole,
    Thresholds, ValidationSummary,
};
pub use manifest::{CandidateReceipt, CandidateVerification};

const REQUEST_LIMIT: u64 = 1024 * 1024;
const REPORT_LIMIT: u64 = 64 * 1024 * 1024;
const CATALOG_LIMIT: u64 = 256 * 1024 * 1024;
const MAX_SOURCES: usize = 128;
const CANDIDATE_FILES: [&str; 9] = [
    "build-report.json",
    "catalog.sqlite",
    "checksums.json",
    "diff.json",
    "icons.json",
    "manifest.json",
    "sources.json",
    "validation.json",
    "verify-report.json",
];

#[derive(thiserror::Error, Debug)]
pub enum PipelineError {
    #[error("catalog pipeline I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("catalog pipeline JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("catalog pipeline catalog error: {0}")]
    Catalog(#[from] CatalogError),
    #[error("catalog pipeline collector error: {0}")]
    Collector(#[from] CollectorError),
    #[error("catalog pipeline icon error: {0}")]
    Icon(#[from] IconCacheError),
    #[error("catalog pipeline validation failed: {0}")]
    Validation(String),
    #[error("catalog pipeline acquisition failed: {0}")]
    Acquisition(String),
}

#[derive(Debug, Clone)]
pub struct PipelineRun {
    pub request: PathBuf,
    pub workspace: PathBuf,
    pub source_cache: PathBuf,
    pub icon_cache: PathBuf,
    pub candidates: PathBuf,
    pub allow_network: bool,
}

impl PipelineRun {
    pub fn new(
        request: impl Into<PathBuf>,
        workspace: impl Into<PathBuf>,
        source_cache: impl Into<PathBuf>,
        icon_cache: impl Into<PathBuf>,
        candidates: impl Into<PathBuf>,
        allow_network: bool,
    ) -> Self {
        Self {
            request: request.into(),
            workspace: workspace.into(),
            source_cache: source_cache.into(),
            icon_cache: icon_cache.into(),
            candidates: candidates.into(),
            allow_network,
        }
    }
}

pub fn build_candidate(run: &PipelineRun) -> Result<CandidateReceipt, PipelineError> {
    build_candidate_with_fetcher(run, &HttpsFetcher)
}

pub fn build_candidate_with_fetcher(
    run: &PipelineRun,
    fetcher: &dyn SourceFetcher,
) -> Result<CandidateReceipt, PipelineError> {
    validate_output_roots(run)?;
    let workspace = canonical_real_directory(&run.workspace, "workspace")?;
    let request_path = if run.request.is_absolute() {
        run.request.clone()
    } else {
        workspace.join(&run.request)
    };
    let request_bytes = read(&workspace, &request_path, REQUEST_LIMIT)?;
    let request: PipelineRequest = serde_json::from_slice(&request_bytes)?;
    validate_request(&request, &workspace)?;
    validate_channel_output(&run.candidates, request.version.channel)?;
    validate_input_output_separation(&request, &request_path, run, &workspace)?;
    validate_policy(&request, &workspace)?;

    let staging_parent = run.candidates.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(staging_parent)?;
    let staging = tempfile::Builder::new()
        .prefix(".catalog-pipeline-")
        .tempdir_in(staging_parent)?;
    let acquired = acquire_sources(&request, run, &workspace, staging.path(), fetcher)?;
    let input = acquired
        .get(&request.input_source)
        .ok_or_else(|| PipelineError::Validation("input source is missing".into()))?;

    let normalized = staging.path().join("normalized.json");
    let input_pin = request
        .sources
        .iter()
        .find(|source| source.id == request.input_source)
        .expect("validated input source");
    match input_pin.role {
        SourceRole::NormalizedBundle => {
            let bytes = read_acquired_file(input, input_pin.max_bytes)?;
            fs::write(&normalized, bytes)?;
        }
        SourceRole::CollectorCapture => {
            let capture = staging.path().join("capture.lua");
            let bytes = read_acquired_file(input, input_pin.max_bytes)?;
            fs::write(&capture, bytes)?;
            import_capture(&ImportRequest::new(
                &capture,
                &normalized,
                request.version.channel,
                &request.version.catalog_version,
            ))?;
        }
        SourceRole::Provenance => {
            return validation("input source cannot have the provenance role")
        }
    }

    let bundle_bytes = fs::read(&normalized)?;
    let bundle: CatalogBundle = serde_json::from_slice(&bundle_bytes)?;
    let bundle = bundle.normalize_and_validate()?;
    validate_version(&request, &bundle)?;
    validate_bundle_sources(&bundle, &acquired)?;

    let catalog = staging.path().join("catalog.sqlite");
    let build_report = build_catalog(&BuildRequest::new(
        &normalized,
        &catalog,
        request.version.channel,
    ))?;
    let verify_report = verify_catalog(&catalog)?;
    let (diff, baseline_semantic_sha256) =
        baseline_diff(&request, &workspace, staging.path(), &catalog)?;
    enforce_thresholds(&request.thresholds, &diff)?;

    let icon_source = match &request.icon_source {
        Some(relative) => resolve_relative(&workspace, relative, true)?,
        None => {
            let path = staging.path().join("empty-icons");
            fs::create_dir(&path)?;
            path
        }
    };
    let references = bundle
        .icon_references
        .iter()
        .filter(|reference| reference.availability != "placeholder")
        .filter(|reference| reference.virtual_path.starts_with('/'))
        .map(|reference| reference.virtual_path.clone())
        .collect::<Vec<_>>();
    let staged_icon_cache = staging.path().join("icon-cache");
    let icon_receipt = build_generation(&IconCacheRequest::new(
        icon_source,
        &staged_icon_cache,
        &verify_report.semantic_sha256,
        references,
    ))?;

    let candidate_stage = staging.path().join("candidate");
    fs::create_dir(&candidate_stage)?;
    fs::copy(&catalog, candidate_stage.join("catalog.sqlite"))?;
    write_json(&candidate_stage.join("build-report.json"), &build_report)?;
    write_json(&candidate_stage.join("verify-report.json"), &verify_report)?;
    write_json(&candidate_stage.join("diff.json"), &diff)?;
    let inventories = acquired
        .values()
        .map(|source| source.inventory.clone())
        .collect::<Vec<_>>();
    let source_inventory_bytes = canonical_bytes(&inventories)?;
    fs::write(
        candidate_stage.join("sources.json"),
        &source_inventory_bytes,
    )?;
    let validation_findings = acquired
        .values()
        .filter(|source| source.inventory.acquisition == "stale-cache")
        .map(|source| format!("stale-cache-reuse:{}", source.inventory.id))
        .collect();
    write_json(
        &candidate_stage.join("validation.json"),
        &ValidationSummary {
            status: "passed",
            channel: request.version.channel,
            game_version: request.version.game_version.clone(),
            api_version: request.version.api_version,
            catalog_version: request.version.catalog_version.clone(),
            findings: validation_findings,
        },
    )?;
    write_json(
        &candidate_stage.join("icons.json"),
        &IconSummary {
            generation_sha256: icon_receipt.generation_sha256.clone(),
            transformation_id: icon_receipt.transformation_id,
            entry_count: icon_receipt.entry_count,
            object_count: icon_receipt.object_count,
            placeholder_count: icon_receipt.placeholder_count,
        },
    )?;

    let mut artifacts = records_for(
        &candidate_stage,
        &[
            "build-report.json",
            "catalog.sqlite",
            "diff.json",
            "icons.json",
            "sources.json",
            "validation.json",
            "verify-report.json",
        ],
    )?;
    write_json(&candidate_stage.join("checksums.json"), &artifacts)?;
    artifacts.push(record_for(&candidate_stage, "checksums.json")?);
    artifacts.sort_by(|left, right| left.path.cmp(&right.path));
    let request_sha256 = sha256(&serde_json::to_vec(&request)?);
    let manifest = CandidateManifest {
        schema_version: 1,
        state: "ready-for-review".into(),
        version: request.version,
        request_sha256,
        source_inventory_sha256: sha256(&source_inventory_bytes),
        catalog_semantic_sha256: verify_report.semantic_sha256.clone(),
        catalog_artifact_sha256: verify_report.artifact_sha256.clone(),
        baseline_semantic_sha256,
        icon_generation_sha256: icon_receipt.generation_sha256.clone(),
        thresholds: request.thresholds,
        artifacts,
    };
    let manifest_bytes = canonical_bytes(&manifest)?;
    let candidate_sha256 = sha256(&manifest_bytes);
    fs::write(candidate_stage.join("manifest.json"), manifest_bytes)?;

    let staged_channel = staging.path().join(manifest.version.channel.as_str());
    fs::create_dir(&staged_channel)?;
    let staged_candidate = staged_channel.join(&candidate_sha256);
    fs::rename(&candidate_stage, &staged_candidate)?;
    let staged_verification = verify_candidate(&staged_candidate)?;
    if staged_verification.candidate_sha256 != candidate_sha256 {
        return validation("staged candidate does not match its manifest identity");
    }
    fs::create_dir_all(run.candidates.join(manifest.version.channel.as_str()))?;
    let destination = run
        .candidates
        .join(manifest.version.channel.as_str())
        .join(&candidate_sha256);
    if destination.exists() {
        let verified = verify_candidate(&destination)?;
        if verified.candidate_sha256 != candidate_sha256 {
            return validation("existing candidate does not match the requested candidate");
        }
    } else if let Err(error) = fs::rename(&staged_candidate, &destination) {
        if error.kind() != std::io::ErrorKind::AlreadyExists || !destination.is_dir() {
            return Err(error.into());
        }
        verify_candidate(&destination)?;
    }
    let installed = verify_candidate(&destination)?;
    if installed.candidate_sha256 != candidate_sha256 {
        return validation("installed candidate does not match its manifest identity");
    }
    publish_staged_generation(
        &staged_icon_cache,
        &run.icon_cache,
        &icon_receipt.generation_sha256,
    )?;
    publish_source_cache(&acquired)?;
    Ok(CandidateReceipt {
        relative_path: PathBuf::from(manifest.version.channel.as_str()).join(&candidate_sha256),
        candidate_sha256,
        catalog_semantic_sha256: verify_report.semantic_sha256,
    })
}

pub fn verify_candidate(path: impl AsRef<Path>) -> Result<CandidateVerification, PipelineError> {
    let path = path.as_ref();
    let metadata = fs::symlink_metadata(path)?;
    if !metadata.is_dir() || is_link_like(&metadata) {
        return validation("candidate must be a real directory");
    }
    let canonical = fs::canonicalize(path)?;
    let mut names = fs::read_dir(&canonical)?
        .map(|entry| entry.map(|entry| entry.file_name().to_string_lossy().into_owned()))
        .collect::<Result<Vec<_>, _>>()?;
    names.sort();
    if names != CANDIDATE_FILES {
        return validation("candidate file allowlist is incomplete or contains extras");
    }
    let manifest_bytes = read_bounded(&canonical, &canonical.join("manifest.json"), REPORT_LIMIT)?;
    let manifest: CandidateManifest = serde_json::from_slice(&manifest_bytes)?;
    if manifest.schema_version != 1 || manifest.artifacts.len() != 8 {
        return validation("candidate manifest contract is invalid");
    }
    if manifest.state != "ready-for-review" {
        return validation("candidate is not ready for review");
    }
    let canonical_manifest = canonical_bytes(&manifest)?;
    if canonical_manifest != manifest_bytes {
        return validation("candidate manifest is not canonical");
    }
    let candidate_sha256 = sha256(&manifest_bytes);
    if path.file_name().and_then(|name| name.to_str()) != Some(candidate_sha256.as_str()) {
        return validation("candidate directory does not match its manifest hash");
    }
    let expected = manifest
        .artifacts
        .iter()
        .map(|record| (record.path.as_str(), record))
        .collect::<BTreeMap<_, _>>();
    for name in CANDIDATE_FILES
        .iter()
        .copied()
        .filter(|name| *name != "manifest.json")
    {
        let record = expected
            .get(name)
            .ok_or_else(|| PipelineError::Validation(format!("manifest omits {name}")))?;
        let limit = if name == "catalog.sqlite" {
            CATALOG_LIMIT
        } else {
            REPORT_LIMIT
        };
        let bytes = read_bounded(&canonical, &canonical.join(name), limit)?;
        if bytes.len() as u64 != record.bytes || sha256(&bytes) != record.sha256 {
            return validation(format!("candidate artifact {name} failed verification"));
        }
    }
    let checksum_bytes = read_bounded(&canonical, &canonical.join("checksums.json"), REPORT_LIMIT)?;
    let checksums: Vec<ArtifactRecord> = serde_json::from_slice(&checksum_bytes)?;
    let expected_checksums = manifest
        .artifacts
        .iter()
        .filter(|record| record.path != "checksums.json")
        .cloned()
        .collect::<Vec<_>>();
    if checksums != expected_checksums {
        return validation("candidate checksum report does not match the manifest");
    }
    let sources = read_bounded(&canonical, &canonical.join("sources.json"), REPORT_LIMIT)?;
    if sha256(&sources) != manifest.source_inventory_sha256 {
        return validation("candidate source inventory identity does not match the manifest");
    }
    let icons: IconSummary = serde_json::from_slice(&read_bounded(
        &canonical,
        &canonical.join("icons.json"),
        REPORT_LIMIT,
    )?)?;
    if icons.generation_sha256 != manifest.icon_generation_sha256 {
        return validation("candidate icon generation identity does not match the manifest");
    }
    let catalog = verify_catalog(canonical.join("catalog.sqlite"))?;
    if catalog.channel != manifest.version.channel
        || catalog.semantic_sha256 != manifest.catalog_semantic_sha256
        || catalog.artifact_sha256 != manifest.catalog_artifact_sha256
        || catalog.schema_version != manifest.version.catalog_schema
        || catalog.game_version != manifest.version.game_version
        || catalog.api_version != manifest.version.api_version
        || catalog.catalog_version != manifest.version.catalog_version
    {
        return validation("candidate catalog identity does not match its manifest");
    }
    if path
        .parent()
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
        != Some(manifest.version.channel.as_str())
    {
        return validation("candidate channel directory does not match its manifest");
    }
    Ok(CandidateVerification {
        candidate_sha256,
        channel: manifest.version.channel,
    })
}

fn validate_request(request: &PipelineRequest, workspace: &Path) -> Result<(), PipelineError> {
    if request.schema_version != 1 || request.version.catalog_schema != 1 {
        return validation("unsupported request or catalog schema");
    }
    if request.sources.is_empty() || request.sources.len() > MAX_SOURCES {
        return validation("source count is outside the accepted range");
    }
    if request.version.game_version.trim().is_empty()
        || request.version.catalog_version.trim().is_empty()
        || request.version.tool_version.trim().is_empty()
        || request.version.locales.is_empty()
    {
        return validation("version tuple contains an empty value");
    }
    if request.version.api_version == 0
        || request.version.catalog_version.len() > 128
        || request.version.tool_version.len() > 128
        || request.version.locales.len() > 32
        || request
            .version
            .locales
            .iter()
            .any(|locale| !valid_locale(locale))
        || request
            .version
            .locales
            .iter()
            .collect::<BTreeSet<_>>()
            .len()
            != request.version.locales.len()
    {
        return validation("version tuple violates its bounded identity contract");
    }
    if (request.network.refresh && !request.network.enabled)
        || (request.network.allow_stale_cache && !request.network.refresh)
    {
        return validation("network refresh and stale-cache flags are inconsistent");
    }
    if parse_commit_message_version(&request.version.game_version)
        .is_none_or(|version| version.to_string() != request.version.game_version)
    {
        return validation("game version must be a canonical dotted numeric version");
    }
    match request.mode {
        PipelineMode::Live if request.version.channel != Channel::Live => {
            return validation("live mode requires the live channel")
        }
        PipelineMode::Pts if request.version.channel != Channel::Pts => {
            return validation("pts mode requires the pts channel")
        }
        PipelineMode::Offline
            if request.network.enabled
                || request.network.refresh
                || request.network.allow_stale_cache =>
        {
            return validation("offline mode cannot enable network behavior")
        }
        _ => {}
    }
    let mut ids = BTreeSet::new();
    for source in &request.sources {
        if !ids.insert(&source.id) || !valid_id(&source.id) {
            return validation("source identifiers must be non-empty and unique");
        }
        if source.game_version.trim().is_empty()
            || source.locale.trim().is_empty()
            || source.revision.trim().is_empty()
            || source.license_scope.trim().is_empty()
            || source.game_version.len() > 64
            || source.locale.len() > 32
            || source.revision.len() > 128
            || source.uri.is_empty()
            || source.uri.len() > 2048
            || source.license_scope.len() > 512
        {
            return validation("source identity contains an empty value");
        }
        if !valid_sha256(&source.sha256)
            || source.max_bytes == 0
            || source.max_bytes > MAX_INPUT_BYTES
        {
            return validation("source hash or byte limit is invalid");
        }
        if source.channel != SourceChannel::NotApplicable
            && source.channel.as_str() != request.version.channel.as_str()
        {
            return validation("source channel does not match the request channel");
        }
        if source.channel != SourceChannel::NotApplicable
            && (source.game_version != request.version.game_version
                || source.api_version != request.version.api_version)
        {
            return validation("source version does not match the request version");
        }
        if let Some(relative) = &source.local_path {
            resolve_relative(workspace, relative, false)?;
        }
    }
    let input = request
        .sources
        .iter()
        .find(|source| source.id == request.input_source)
        .ok_or_else(|| PipelineError::Validation("input source is not pinned".into()))?;
    match request.mode {
        PipelineMode::UserCapture if input.role != SourceRole::CollectorCapture => {
            return validation("user-capture mode requires a collector capture")
        }
        PipelineMode::UserCapture => {}
        _ if input.role != SourceRole::NormalizedBundle => {
            return validation("catalog modes require a normalized bundle")
        }
        _ => {}
    }
    resolve_relative(workspace, &request.source_policy, false)?;
    if let Some(baseline) = &request.baseline {
        resolve_relative(workspace, baseline, false)?;
    }
    Ok(())
}

fn validate_policy(request: &PipelineRequest, workspace: &Path) -> Result<(), PipelineError> {
    let policy_path = resolve_relative(workspace, &request.source_policy, false)?;
    let policy_bytes = read(workspace, &policy_path, REPORT_LIMIT)?;
    let policy: serde_json::Value = serde_json::from_slice(&policy_bytes)?;
    if policy
        .get("schema_version")
        .and_then(serde_json::Value::as_u64)
        != Some(1)
    {
        return validation("source policy schema is invalid");
    }
    let entries = policy
        .get("source_snapshots")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| PipelineError::Validation("source policy has no snapshot list".into()))?;
    for source in &request.sources {
        let fixture = source.uri.starts_with("project://")
            && source.local_path.is_some()
            && source.license_scope == "project-authored-synthetic";
        let local_user_capture = request.mode == PipelineMode::UserCapture
            && source.id == request.input_source
            && source.role == SourceRole::CollectorCapture
            && source.local_path.is_some()
            && source.uri == "user-local-savedvariables"
            && source.license_scope == "user-generated-local-only"
            && source.redistribution == crate::catalog::model::Redistribution::UserGeneratedOnly;
        let listed = entries.iter().any(|entry| {
            entry.get("id").and_then(serde_json::Value::as_str) == Some(source.id.as_str())
                && entry.get("channel").and_then(serde_json::Value::as_str)
                    == Some(source.channel.as_str())
                && entry
                    .get("game_version")
                    .and_then(serde_json::Value::as_str)
                    == Some(source.game_version.as_str())
                && entry.get("api_version").and_then(serde_json::Value::as_u64)
                    == Some(u64::from(source.api_version))
                && entry.get("locale").and_then(serde_json::Value::as_str)
                    == Some(source.locale.as_str())
                && entry.get("revision").and_then(serde_json::Value::as_str)
                    == Some(source.revision.as_str())
                && entry.get("sha256").and_then(serde_json::Value::as_str)
                    == Some(source.sha256.as_str())
                && entry
                    .get("license_scope")
                    .and_then(serde_json::Value::as_str)
                    == Some(source.license_scope.as_str())
                && entry
                    .get("uri")
                    .and_then(serde_json::Value::as_str)
                    .is_some_and(|uri| uri_matches_policy(uri, &source.uri))
        });
        if !fixture && !local_user_capture && !listed {
            return validation(format!("source {} is not admitted by policy", source.id));
        }
    }
    Ok(())
}

fn validate_version(
    request: &PipelineRequest,
    bundle: &CatalogBundle,
) -> Result<(), PipelineError> {
    let release = &bundle.release;
    if release.channel != request.version.channel
        || release.game_version != request.version.game_version
        || release.api_version != request.version.api_version
        || release.catalog_version != request.version.catalog_version
        || release.locales != request.version.locales
        || release.tool_version != request.version.tool_version
    {
        return validation("normalized catalog version tuple does not match the request");
    }
    Ok(())
}

fn validate_bundle_sources(
    bundle: &CatalogBundle,
    acquired: &BTreeMap<String, AcquiredSource>,
) -> Result<(), PipelineError> {
    if bundle.source_snapshots.iter().any(|snapshot| {
        !acquired.values().any(|source| {
            let inventory = &source.inventory;
            let common_identity = inventory.channel == snapshot.channel
                && inventory.game_version == snapshot.game_version
                && inventory.api_version == snapshot.api_version
                && inventory.locale == snapshot.locale
                && inventory.revision == snapshot.revision
                && inventory.sha256 == snapshot.raw_sha256
                && inventory.license_scope == snapshot.license_scope
                && inventory.redistribution == snapshot.redistribution;
            if inventory.role == SourceRole::CollectorCapture {
                common_identity && inventory.uri == snapshot.uri
            } else {
                common_identity
                    && inventory.id == snapshot.snapshot_id
                    && uri_matches_policy(&snapshot.uri, &inventory.uri)
            }
        })
    }) {
        return validation(
            "normalized catalog source snapshot does not match acquired identity or rights",
        );
    }
    Ok(())
}

fn baseline_diff(
    request: &PipelineRequest,
    workspace: &Path,
    staging: &Path,
    catalog: &Path,
) -> Result<(CatalogDiff, Option<String>), PipelineError> {
    let Some(relative) = &request.baseline else {
        return Ok((CatalogDiff::default(), None));
    };
    let source = resolve_relative(workspace, relative, false)?;
    let bytes = read(workspace, &source, CATALOG_LIMIT)?;
    let baseline = staging.join("baseline.sqlite");
    fs::write(&baseline, bytes)?;
    let report = verify_catalog(&baseline)?;
    if report.channel != request.version.channel {
        return validation("baseline channel does not match the candidate channel");
    }
    Ok((
        diff_catalogs(&baseline, catalog)?,
        Some(report.semantic_sha256),
    ))
}

fn enforce_thresholds(thresholds: &Thresholds, diff: &CatalogDiff) -> Result<(), PipelineError> {
    if !diff.localized_text_redistribution_changes.is_empty() {
        return validation(format!(
            "localized text redistribution change count {} blocks publication",
            diff.localized_text_redistribution_changes.len()
        ));
    }
    let checks = [
        (
            "entities",
            diff.entities.removed.len(),
            thresholds.entities_removed,
        ),
        (
            "localized text",
            diff.localized_text.removed.len(),
            thresholds.localized_text_removed,
        ),
        (
            "relations",
            diff.relations.removed.len(),
            thresholds.relations_removed,
        ),
        (
            "coverage",
            diff.coverage.removed.len() + diff.coverage_regressions.len(),
            thresholds.coverage_removed,
        ),
        (
            "icon references",
            diff.icon_references.removed.len(),
            thresholds.icon_references_removed,
        ),
    ];
    if let Some((name, count, allowed)) = checks.iter().find(|(_, count, allowed)| count > allowed)
    {
        return validation(format!(
            "{name} removal count {count} exceeds threshold {allowed}"
        ));
    }
    Ok(())
}

fn canonical_real_directory(path: &Path, name: &str) -> Result<PathBuf, PipelineError> {
    let metadata = fs::symlink_metadata(path)?;
    if !metadata.is_dir() || is_link_like(&metadata) {
        return validation(format!("{name} must be a real directory"));
    }
    Ok(fs::canonicalize(path)?)
}

fn validate_output_roots(run: &PipelineRun) -> Result<(), PipelineError> {
    for root in [&run.source_cache, &run.icon_cache, &run.candidates] {
        if root.exists() {
            let metadata = fs::symlink_metadata(root)?;
            if !metadata.is_dir() || is_link_like(&metadata) {
                return validation("pipeline output roots must be real directories");
            }
        }
    }
    let roots = [
        resolve_for_alias(&run.source_cache)?,
        resolve_for_alias(&run.icon_cache)?,
        resolve_for_alias(&run.candidates)?,
    ];
    for (index, left) in roots.iter().enumerate() {
        for right in roots.iter().skip(index + 1) {
            if is_same_or_nested(left, right) || is_same_or_nested(right, left) {
                return validation("pipeline output roots must be distinct and unnested");
            }
        }
    }
    Ok(())
}

fn validate_channel_output(root: &Path, channel: Channel) -> Result<(), PipelineError> {
    let path = root.join(channel.as_str());
    if path.exists() {
        let metadata = fs::symlink_metadata(path)?;
        if !metadata.is_dir() || is_link_like(&metadata) {
            return validation("candidate channel output must be a real directory");
        }
    }
    Ok(())
}

fn validate_input_output_separation(
    request: &PipelineRequest,
    request_path: &Path,
    run: &PipelineRun,
    workspace: &Path,
) -> Result<(), PipelineError> {
    let outputs = [
        resolve_for_alias(&run.source_cache)?,
        resolve_for_alias(&run.icon_cache)?,
        resolve_for_alias(&run.candidates)?,
    ];
    let mut inputs = vec![fs::canonicalize(request_path)?];
    inputs.push(fs::canonicalize(resolve_relative(
        workspace,
        &request.source_policy,
        false,
    )?)?);
    for source in &request.sources {
        if let Some(relative) = &source.local_path {
            inputs.push(fs::canonicalize(resolve_relative(
                workspace, relative, false,
            )?)?);
        }
    }
    if let Some(relative) = &request.baseline {
        inputs.push(fs::canonicalize(resolve_relative(
            workspace, relative, false,
        )?)?);
    }
    if let Some(relative) = &request.icon_source {
        inputs.push(fs::canonicalize(resolve_relative(
            workspace, relative, true,
        )?)?);
    }
    if inputs.iter().any(|input| {
        outputs
            .iter()
            .any(|output| is_same_or_nested(input, output))
    }) {
        return validation("pipeline inputs must remain outside every output root");
    }
    Ok(())
}

fn resolve_for_alias(path: &Path) -> Result<PathBuf, PipelineError> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };
    if absolute.exists() {
        return Ok(fs::canonicalize(absolute)?);
    }
    let mut missing = Vec::new();
    let mut ancestor = absolute.as_path();
    while !ancestor.exists() {
        let name = ancestor.file_name().ok_or_else(|| {
            PipelineError::Validation("output root has no existing ancestor".into())
        })?;
        missing.push(name.to_os_string());
        ancestor = ancestor
            .parent()
            .ok_or_else(|| PipelineError::Validation("output root has no parent".into()))?;
    }
    let mut resolved = fs::canonicalize(ancestor)?;
    for component in missing.iter().rev() {
        resolved.push(component);
    }
    Ok(resolved)
}

fn resolve_relative(root: &Path, value: &str, directory: bool) -> Result<PathBuf, PipelineError> {
    let path = Path::new(value);
    if value.is_empty()
        || value.len() > 1024
        || path.is_absolute()
        || value.contains(['\\', ':', '\0'])
        || path
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
    {
        return validation("pipeline paths must be safe forward-slash relative paths");
    }
    let joined = root.join(path);
    if directory && joined.exists() {
        let metadata = fs::symlink_metadata(&joined)?;
        if !metadata.is_dir() || is_link_like(&metadata) {
            return validation("pipeline directory path is not a real directory");
        }
        let canonical = fs::canonicalize(&joined)?;
        if !is_same_or_nested(&canonical, root) {
            return validation("pipeline directory path escaped the workspace");
        }
        return Ok(canonical);
    }
    Ok(joined)
}

fn read(root: &Path, path: &Path, max_bytes: u64) -> Result<Vec<u8>, PipelineError> {
    read_bounded(root, path, max_bytes)
}

fn read_acquired_file(source: &AcquiredSource, max_bytes: u64) -> Result<Vec<u8>, PipelineError> {
    read_bounded(&source.root, &source.path, max_bytes)
}

fn read_bounded(root: &Path, path: &Path, max_bytes: u64) -> Result<Vec<u8>, PipelineError> {
    read_bounded_stable(root, path, max_bytes).map_err(|error| match error {
        StableReadError::Io(error) => PipelineError::Io(error),
        StableReadError::Invalid(message) => PipelineError::Validation(message.into()),
    })
}

fn uri_matches_policy(policy: &str, requested: &str) -> bool {
    if policy == requested {
        return true;
    }
    let Some(path) = policy.strip_prefix("https://github.com/esoui/esoui/blob/") else {
        return false;
    };
    requested == format!("https://raw.githubusercontent.com/esoui/esoui/{path}")
}

fn records_for(root: &Path, names: &[&str]) -> Result<Vec<ArtifactRecord>, PipelineError> {
    names.iter().map(|name| record_for(root, name)).collect()
}

fn record_for(root: &Path, name: &str) -> Result<ArtifactRecord, PipelineError> {
    let bytes = fs::read(root.join(name))?;
    Ok(ArtifactRecord {
        path: name.to_string(),
        sha256: sha256(&bytes),
        bytes: bytes.len() as u64,
    })
}

fn write_json(path: &Path, value: &impl Serialize) -> Result<(), PipelineError> {
    fs::write(path, canonical_bytes(value)?)?;
    Ok(())
}

fn canonical_bytes(value: &impl Serialize) -> Result<Vec<u8>, PipelineError> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn valid_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn valid_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value.as_bytes()[0].is_ascii_alphanumeric()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

fn valid_locale(value: &str) -> bool {
    let mut parts = value.split('-');
    let Some(language) = parts.next() else {
        return false;
    };
    if !(2..=3).contains(&language.len())
        || !language.bytes().all(|byte| byte.is_ascii_alphabetic())
    {
        return false;
    }
    parts.all(|part| {
        (2..=8).contains(&part.len()) && part.bytes().all(|byte| byte.is_ascii_alphanumeric())
    })
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn validation<T>(message: impl Into<String>) -> Result<T, PipelineError> {
    Err(PipelineError::Validation(message.into()))
}
