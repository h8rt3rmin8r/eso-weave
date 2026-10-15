//! Verified Catalog maintenance and compatible selection/rollback domain services.

mod availability;
mod contract;

use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use serde_json::json;
use sha2::{Digest, Sha256};

pub use availability::{
    resolve_availability, AvailabilityInput, CatalogAvailability, CheckFreshness, LiveUpdateState,
};
pub use contract::{
    CatalogSelection, CatalogTarget, UpdateOperation, UpdateProgress, UpdateReceipt, UpdateResult,
    UpdateStage,
};

use crate::bounded_file::{
    is_link_like, is_same_or_nested, open_bounded_stable, read_bounded_stable, StableReadError,
};
use crate::catalog::accumulate::{
    accumulate, content_sha256, latest_observation, read_bundle, same_domain,
};
use crate::catalog::schema::SCHEMA_VERSION;
use crate::catalog::{CatalogAccess, Channel};
use crate::catalog_pipeline::{build_candidate_with_cancel, PipelineRun};
use crate::catalog_pipeline::{
    inspect_candidate, CandidateSummary, PipelineError, CANDIDATE_FILES,
};
use crate::collector::{canonical_catalog_capture, parse_capture, MAX_CAPTURE_BYTES};

const SELECTION_LIMIT: u64 = 16 * 1024;

#[derive(thiserror::Error, Debug)]
pub enum UpdateError {
    #[error("catalog update I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("catalog update JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("catalog candidate failed verification: {0}")]
    Pipeline(#[from] PipelineError),
    #[error("catalog candidate origin was not acknowledged")]
    OriginNotAcknowledged,
    #[error("catalog update was cancelled")]
    Cancelled,
    #[error("another catalog operation is already running")]
    ConcurrentOperation,
    #[error("catalog selection is invalid: {0}")]
    InvalidSelection(String),
    #[error("catalog update is invalid: {0}")]
    Validation(String),
    #[error("no rollback target is available")]
    NoRollbackTarget,
}

#[derive(Debug, Clone, Default)]
pub struct CancellationToken(Arc<CancellationState>);

#[derive(Debug, Default)]
struct CancellationState {
    cancelled: AtomicBool,
    commit: Mutex<()>,
}

impl CancellationToken {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cancel(&self) {
        let _guard = self
            .0
            .commit
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        self.0.cancelled.store(true, Ordering::Release);
    }

    pub fn is_cancelled(&self) -> bool {
        self.0.cancelled.load(Ordering::Acquire)
    }

    pub(crate) fn same_generation(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }

    fn check(&self) -> Result<(), UpdateError> {
        if self.is_cancelled() {
            Err(UpdateError::Cancelled)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Clone)]
pub struct UpdateRoots {
    root: PathBuf,
    channel: Channel,
}

impl UpdateRoots {
    fn new(config: &Path, channel: Channel) -> Self {
        Self {
            root: if channel == Channel::Live {
                config.join("catalog")
            } else {
                config.join("catalog").join("pts")
            },
            channel,
        }
    }

    pub fn root(&self) -> PathBuf {
        self.root.clone()
    }

    pub fn import_root(&self) -> PathBuf {
        self.root.join("import")
    }

    pub fn source_cache(&self) -> PathBuf {
        self.root.join("sources")
    }

    pub fn icon_cache(&self) -> PathBuf {
        self.root.join("icons")
    }

    fn live_versions(&self) -> PathBuf {
        self.root.join("versions").join(self.channel.as_str())
    }

    fn staging(&self) -> PathBuf {
        self.root.join("staging")
    }

    fn receipts(&self) -> PathBuf {
        self.root.join("receipts")
    }

    fn selection(&self) -> PathBuf {
        self.root.join("selection.json")
    }

    fn operation_lock(&self) -> PathBuf {
        self.root.join("operation.lock")
    }

    fn prepare(&self) -> Result<(), UpdateError> {
        for path in [
            self.root(),
            self.import_root(),
            self.import_root().join("live"),
            self.import_root().join("pts"),
            self.source_cache(),
            self.icon_cache(),
            self.live_versions(),
            self.staging(),
            self.receipts(),
        ] {
            ensure_real_directory(&path)?;
        }
        Ok(())
    }
}

pub struct CatalogResolution {
    pub access: CatalogAccess,
    pub path: PathBuf,
    pub target: CatalogTarget,
    pub warning: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InstallOutcome {
    pub candidate: CandidateSummary,
    pub selection: CatalogSelection,
    pub receipt_warning: Option<String>,
}

#[derive(Debug, Clone)]
pub enum AutomaticCatalogOutcome {
    Activated(Box<InstallOutcome>),
    Unchanged(CatalogSelection),
}

#[derive(Debug, Clone)]
pub struct RollbackOutcome {
    pub selection: CatalogSelection,
    pub receipt_warning: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaptureFingerprint {
    pub size: u64,
    pub modified_unix_nanos: Option<u128>,
    pub sha256: String,
}

#[derive(Debug, Clone)]
pub struct CatalogUpdateService {
    roots: UpdateRoots,
    bundled_catalog: PathBuf,
}

impl CatalogUpdateService {
    pub fn new(config: impl AsRef<Path>, bundled_catalog: impl AsRef<Path>) -> Self {
        Self::new_for_channel(config, bundled_catalog, Channel::Live)
    }

    pub fn new_for_channel(
        config: impl AsRef<Path>,
        bundled_catalog: impl AsRef<Path>,
        channel: Channel,
    ) -> Self {
        Self {
            roots: UpdateRoots::new(config.as_ref(), channel),
            bundled_catalog: bundled_catalog.as_ref().to_path_buf(),
        }
    }

    pub fn roots(&self) -> UpdateRoots {
        self.roots.clone()
    }

    /// Locates the registry that owns a published immutable Catalog path.
    pub fn for_catalog_path(path: &Path) -> Option<Self> {
        let identity = path.parent()?;
        let channel_directory = identity.parent()?;
        let channel = match channel_directory.file_name()?.to_str()? {
            "live" => Channel::Live,
            "pts" => Channel::Pts,
            _ => return None,
        };
        let versions = channel_directory.parent()?;
        if versions.file_name()?.to_str()? != "versions" {
            return None;
        }
        Some(Self {
            roots: UpdateRoots {
                root: versions.parent()?.to_path_buf(),
                channel,
            },
            bundled_catalog: path.to_path_buf(),
        })
    }

    /// Resolves an exact historical version domain without changing selection.
    pub fn resolve_catalog_for_version(
        &self,
        channel: Channel,
        api_version: u32,
        game_version: &str,
    ) -> Option<CatalogResolution> {
        self.resolve_catalog_version(channel, api_version, Some(game_version))
    }

    /// Preserves the existing same-API patch compatibility of encounter metrics.
    pub fn resolve_catalog_for_api(
        &self,
        channel: Channel,
        api_version: u32,
    ) -> Option<CatalogResolution> {
        self.resolve_catalog_version(channel, api_version, None)
    }

    fn resolve_catalog_version(
        &self,
        channel: Channel,
        api_version: u32,
        game_version: Option<&str>,
    ) -> Option<CatalogResolution> {
        if channel != self.roots.channel {
            return None;
        }
        let current = self.resolve_catalog();
        if current
            .access
            .release()
            .ok()
            .flatten()
            .is_some_and(|release| {
                release.channel == channel
                    && release.api_version == api_version
                    && game_version.is_none_or(|game| release.game_version == game)
            })
        {
            return Some(current);
        }
        let mut best = None;
        for target in self.accepted_targets().unwrap_or_default() {
            let CatalogTarget::User { candidate_sha256 } = target else {
                continue;
            };
            let directory = self.roots.live_versions().join(candidate_sha256);
            let Ok(summary) = inspect_candidate(&directory) else {
                continue;
            };
            if summary.channel != channel
                || summary.api_version != api_version
                || game_version.is_some_and(|game| summary.game_version != game)
            {
                continue;
            }
            let path = directory.join("catalog.sqlite");
            let Ok(bundle) = read_bundle(&path) else {
                continue;
            };
            let order = (
                latest_observation(&bundle),
                summary.candidate_sha256.clone(),
            );
            if best.as_ref().is_none_or(|(previous, _)| &order > previous) {
                let target = CatalogTarget::User {
                    candidate_sha256: summary.candidate_sha256,
                };
                best = Some((
                    order,
                    CatalogResolution {
                        access: CatalogAccess::open_or_empty(&path),
                        path,
                        target,
                        warning: None,
                    },
                ));
            }
        }
        best.map(|(_, resolution)| resolution)
    }

    pub fn prepare(&self) -> Result<(), UpdateError> {
        self.roots.prepare()
    }

    /// Finds fully verified review candidates without exposing their local paths.
    pub fn discover_candidates(&self) -> Result<Vec<CandidateSummary>, UpdateError> {
        self.prepare()?;
        let mut summaries = Vec::new();
        for channel in [Channel::Live, Channel::Pts] {
            let channel_root = self.roots.import_root().join(channel.as_str());
            for entry in fs::read_dir(&channel_root)?.take(256) {
                let entry = entry?;
                let metadata = fs::symlink_metadata(entry.path())?;
                if !metadata.is_dir() || is_link_like(&metadata) {
                    continue;
                }
                let summary = match inspect_candidate(entry.path()) {
                    Ok(summary) => summary,
                    Err(error) => {
                        tracing::warn!(
                            target: "eso_weave::catalog_update",
                            channel = channel.as_str(),
                            "ignored invalid imported candidate: {error}"
                        );
                        continue;
                    }
                };
                if summary.channel == channel {
                    summaries.push(summary);
                }
            }
        }
        summaries.sort_by(|left, right| {
            left.channel
                .as_str()
                .cmp(right.channel.as_str())
                .then_with(|| right.api_version.cmp(&left.api_version))
                .then_with(|| right.catalog_version.cmp(&left.catalog_version))
                .then_with(|| right.candidate_sha256.cmp(&left.candidate_sha256))
        });
        Ok(summaries)
    }

    /// Removes only abandoned private staging directories. Accepted versions are
    /// outside this root and are never considered by recovery.
    pub fn recover_staging(&self) -> Result<usize, UpdateError> {
        self.prepare()?;
        let _lock = OperationLock::acquire(&self.roots.operation_lock())?;
        let mut recovered = 0;
        for (directory, prefixes) in [
            (
                self.roots.staging(),
                &[".update-", ".collector-build-", ".collector-input-"][..],
            ),
            (self.roots.root(), &[".catalog-pipeline-"][..]),
        ] {
            let canonical_parent = fs::canonicalize(&directory)?;
            for entry in fs::read_dir(&directory)?.take(256) {
                let entry = entry?;
                let name = entry.file_name();
                if !prefixes
                    .iter()
                    .any(|prefix| name.to_string_lossy().starts_with(prefix))
                {
                    continue;
                }
                let metadata = fs::symlink_metadata(entry.path())?;
                if metadata.is_dir() && !is_link_like(&metadata) {
                    let target = fs::canonicalize(entry.path())?;
                    if target.parent() == Some(canonical_parent.as_path()) {
                        fs::remove_dir_all(target)?;
                        recovered += 1;
                    }
                }
            }
        }
        Ok(recovered)
    }

    /// Records the private in-memory boundary used to prove a later capture was
    /// flushed after the user saw the save instructions.
    pub fn capture_fingerprint(
        &self,
        capture_path: impl AsRef<Path>,
    ) -> Result<CaptureFingerprint, UpdateError> {
        fingerprint_capture(capture_path.as_ref())
    }

    pub fn capture_wait_boundary(
        &self,
        capture_path: impl AsRef<Path>,
    ) -> Result<CaptureFingerprint, UpdateError> {
        if !capture_path.as_ref().exists() {
            return Ok(empty_capture_fingerprint());
        }
        fingerprint_capture(capture_path.as_ref())
    }

    /// Builds a local S073 review candidate from a later stable S071 capture.
    /// The active catalog is a zero-removal baseline, so a partial capture cannot
    /// silently replace accepted coverage.
    pub fn build_collector_candidate<F>(
        &self,
        capture_path: impl AsRef<Path>,
        waiting_fingerprint: &CaptureFingerprint,
        cancel: &CancellationToken,
        mut progress: F,
    ) -> Result<CandidateSummary, UpdateError>
    where
        F: FnMut(UpdateProgress),
    {
        self.build_collector_candidate_inner(
            capture_path.as_ref(),
            Some(waiting_fingerprint),
            cancel,
            &mut progress,
        )
    }

    fn build_collector_candidate_inner<F>(
        &self,
        capture_path: &Path,
        waiting_fingerprint: Option<&CaptureFingerprint>,
        cancel: &CancellationToken,
        mut progress: F,
    ) -> Result<CandidateSummary, UpdateError>
    where
        F: FnMut(UpdateProgress),
    {
        cancel.check()?;
        self.prepare()?;
        if let Some(waiting) = waiting_fingerprint {
            progress(UpdateProgress::new(
                UpdateStage::WaitingForCapture,
                0,
                0,
                "Checking whether ESO saved new catalog data since observation began",
            ));
            let first = fingerprint_capture(capture_path)?;
            if first.sha256 == waiting.sha256 {
                return Err(UpdateError::Validation(
                "The saved Catalog generation has not changed. Automatic maintenance waits for new complete evidence.".into(),
            ));
            }
            let second = fingerprint_capture(capture_path)?;
            if first != second {
                return Err(UpdateError::Validation(
                "ESO's saved addon data is still changing. Automatic maintenance retains the last complete generation and retries.".into(),
            ));
            }
        }
        let canonical_parent = fs::canonicalize(
            capture_path
                .parent()
                .ok_or_else(|| UpdateError::Validation("capture has no parent".into()))?,
        )?;
        let shared_bytes = read_bounded_stable(&canonical_parent, capture_path, MAX_CAPTURE_BYTES)
            .map_err(stable_capture_error)?;
        let envelope = parse_capture(&shared_bytes).map_err(PipelineError::Collector)?;
        let capture_bytes = canonical_catalog_capture(&shared_bytes)
            .map_err(PipelineError::Collector)?
            .ok_or_else(|| {
                UpdateError::Validation("Saved addon data contains no complete Catalog generation. Retained data is unchanged.".into())
            })?;
        if envelope.channel != self.roots.channel {
            return Err(UpdateError::Validation(
                format!("The completed capture does not match this {} context; environments remain separate.",self.roots.channel),
            ));
        }
        cancel.check()?;

        let workspace = tempfile::Builder::new()
            .prefix(".collector-build-")
            .tempdir_in(self.roots.staging())?;
        let capture_name = "capture.lua";
        fs::write(workspace.path().join(capture_name), &capture_bytes)?;
        let baseline = workspace.path().join("baseline.sqlite");
        let baseline_source = self.active_catalog_path();
        let baseline_report = crate::catalog::compiler::verify_catalog(&baseline_source).ok();
        let matching_baseline = baseline_report
            .as_ref()
            .is_some_and(|report| report.channel == envelope.channel);
        if matching_baseline {
            fs::copy(&baseline_source, &baseline)?;
        }
        let capture_hash = sha256_bytes(&capture_bytes);
        let catalog_version = format!(
            "local-collector-{}-{}",
            envelope.api_version,
            &capture_hash[..12]
        );
        let request = json!({
            "schema_version": 1,
            "mode": "user-capture",
            "version": {
                "channel": envelope.channel,
                "game_version": envelope.game_version,
                "api_version": envelope.api_version,
                "catalog_version": catalog_version,
                "catalog_schema": SCHEMA_VERSION,
                "locales": [envelope.locale],
                "tool_version": env!("CARGO_PKG_VERSION")
            },
            "input_source": "collector-capture",
            "source_policy": "source-policy.json",
            "sources": [{
                "id": "collector-capture",
                "role": "collector-capture",
                "channel": envelope.channel,
                "game_version": envelope.game_version,
                "api_version": envelope.api_version,
                "locale": envelope.locale,
                "revision": envelope.collector_checksum,
                "sha256": capture_hash,
                "max_bytes": MAX_CAPTURE_BYTES,
                "uri": "user-local-savedvariables",
                "local_path": capture_name,
                "license_scope": "user-generated-local-only",
                "redistribution": "user-generated-only"
            }],
            "network": { "enabled": false, "refresh": false, "allow_stale_cache": false },
            "baseline": if matching_baseline { Some("baseline.sqlite") } else { None },
            "icon_source": null,
            "thresholds": {
                "entities_removed": 0,
                "localized_text_removed": 0,
                "relations_removed": 0,
                "coverage_removed": 0,
                "icon_references_removed": 0
            }
        });
        fs::write(
            workspace.path().join("request.json"),
            canonical_json(&request)?,
        )?;
        fs::write(
            workspace.path().join("source-policy.json"),
            b"{\"schema_version\":1,\"source_snapshots\":[]}\n",
        )?;
        progress(UpdateProgress::new(
            UpdateStage::Building,
            0,
            0,
            "Building a local catalog update for review from saved game definitions",
        ));
        cancel.check()?;
        let receipt = build_candidate_with_cancel(
            &PipelineRun::new(
                workspace.path().join("request.json"),
                workspace.path(),
                self.roots.source_cache(),
                self.roots.icon_cache(),
                self.roots.import_root(),
                false,
            ),
            || cancel.is_cancelled(),
        )
        .map_err(|error| match error {
            PipelineError::Cancelled => UpdateError::Cancelled,
            error => UpdateError::Pipeline(error),
        })?;
        cancel.check()?;
        progress(UpdateProgress::new(
            UpdateStage::IntegrityChecking,
            0,
            0,
            "Verifying the complete local review candidate",
        ));
        inspect_candidate(self.roots.import_root().join(receipt.relative_path)).map_err(Into::into)
    }

    /// Reconciles naturally saved complete data, including on the first desktop
    /// startup. The lock covers the baseline, build and short selection boundary.
    pub fn maintain_collector_capture<F>(
        &self,
        capture_path: impl AsRef<Path>,
        cancel: &CancellationToken,
        mut progress: F,
    ) -> Result<AutomaticCatalogOutcome, UpdateError>
    where
        F: FnMut(UpdateProgress),
    {
        cancel.check()?;
        self.prepare()?;
        let _lock = OperationLock::acquire(&self.roots.operation_lock())?;
        let path = capture_path.as_ref();
        let parent = fs::canonicalize(
            path.parent()
                .ok_or_else(|| UpdateError::Validation("capture has no parent".into()))?,
        )?;
        let bytes =
            read_bounded_stable(&parent, path, MAX_CAPTURE_BYTES).map_err(stable_capture_error)?;
        let envelope = parse_capture(&bytes).map_err(PipelineError::Collector)?;
        if envelope.channel != self.roots.channel {
            return Err(UpdateError::Validation(
                "PTS captures remain in their separate version domain".into(),
            ));
        }
        let canonical = canonical_catalog_capture(&bytes)
            .map_err(PipelineError::Collector)?
            .ok_or_else(|| UpdateError::Validation("saved data has no completed catalog".into()))?;
        let capture_hash = sha256_bytes(&canonical);
        let incoming = crate::collector::import::bundle_from_capture(
            &envelope,
            &capture_hash,
            "automatic-preview",
        )
        .map_err(PipelineError::Collector)?
        .normalize_and_validate()
        .map_err(PipelineError::Catalog)?;
        let resolution = self.resolve_catalog();
        let saved_selection = self.load_selection().unwrap_or_default();
        if saved_selection.active != resolution.target {
            let _commit = cancel
                .0
                .commit
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            cancel.check()?;
            self.write_selection(&CatalogSelection {
                active: resolution.target.clone(),
                previous: Some(CatalogTarget::Bundled),
                generation: saved_selection.generation.saturating_add(1),
                last_receipt: None,
                ..CatalogSelection::default()
            })?;
            drop(_commit);
        }
        let baseline = read_bundle(&resolution.path)
            .ok()
            .filter(|baseline| baseline.release.channel == self.roots.channel);
        if let Some(baseline) = baseline.filter(|baseline| same_domain(baseline, &incoming)) {
            let merged = accumulate(baseline.clone(), incoming).map_err(PipelineError::Catalog)?;
            if content_sha256(&baseline).map_err(PipelineError::Catalog)?
                == content_sha256(&merged).map_err(PipelineError::Catalog)?
            {
                cancel.check()?;
                let selection = self.load_selection()?;
                self.register_accepted(&selection)?;
                self.prune_automatic_history(&selection)?;
                return Ok(AutomaticCatalogOutcome::Unchanged(selection));
            }
        }
        cancel.check()?;
        // Build from the stable snapshot rather than reopening a game-owned file.
        let snapshot = tempfile::Builder::new()
            .prefix(".collector-input-")
            .tempdir_in(self.roots.staging())?;
        let captured = snapshot.path().join("capture.lua");
        fs::write(&captured, canonical)?;
        let candidate =
            self.build_collector_candidate_inner(&captured, None, cancel, &mut progress)?;
        let candidate_path = self
            .roots
            .import_root()
            .join(self.roots.channel.as_str())
            .join(&candidate.candidate_sha256);
        let mut outcome = match self.install_inner(&candidate_path, cancel, &mut progress) {
            Ok(outcome) => outcome,
            Err(error) => {
                if let Ok(selection) = self.load_selection() {
                    let _ = self.prune_automatic_history(&selection);
                }
                return Err(error);
            }
        };
        if outcome.receipt_warning.is_none() {
            if let Err(error) = self.prune_automatic_history(&outcome.selection) {
                let warning = format!(
                    "Catalog selection succeeded, but old generation cleanup is deferred: {error}"
                );
                outcome.receipt_warning = Some(match outcome.receipt_warning {
                    Some(previous) => format!("{previous} {warning}"),
                    None => warning,
                });
            }
        }
        Ok(AutomaticCatalogOutcome::Activated(Box::new(outcome)))
    }

    fn prune_automatic_history(&self, selection: &CatalogSelection) -> Result<(), UpdateError> {
        let mut keep: BTreeSet<_> = std::iter::once(&selection.active)
            .chain(selection.previous.iter())
            .filter_map(|target| match target {
                CatalogTarget::User { candidate_sha256 } => Some(candidate_sha256.clone()),
                _ => None,
            })
            .collect();
        keep.extend(
            self.accepted_targets()?
                .into_iter()
                .filter_map(|target| match target {
                    CatalogTarget::User { candidate_sha256 } => Some(candidate_sha256),
                    _ => None,
                }),
        );
        let mut obsolete_sources = BTreeSet::new();
        let mut retained_sources = BTreeSet::new();
        let mut obsolete_icons = BTreeSet::new();
        let mut retained_icons = BTreeSet::new();
        for directory in [
            self.roots.live_versions(),
            self.roots.import_root().join(self.roots.channel.as_str()),
        ] {
            let canonical = fs::canonicalize(&directory)?;
            for entry in fs::read_dir(&canonical)?.take(256) {
                let entry = entry?;
                let metadata = fs::symlink_metadata(entry.path())?;
                if !metadata.is_dir() || is_link_like(&metadata) {
                    continue;
                }
                let Ok(candidate) = inspect_candidate(entry.path()) else {
                    continue;
                };
                if !candidate.catalog_version.starts_with("local-collector-") {
                    continue;
                }
                let sources: serde_json::Value = serde_json::from_slice(
                    &read_bounded_stable(
                        &canonical,
                        &entry.path().join("sources.json"),
                        64 * 1024 * 1024,
                    )
                    .map_err(stable_read_error)?,
                )?;
                let hashes = sources
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(|source| source["sha256"].as_str())
                    .map(str::to_owned);
                let icons: serde_json::Value = serde_json::from_slice(
                    &read_bounded_stable(
                        &canonical,
                        &entry.path().join("icons.json"),
                        64 * 1024 * 1024,
                    )
                    .map_err(stable_read_error)?,
                )?;
                let icon = icons["generation_sha256"].as_str().map(str::to_owned);
                if keep.contains(&candidate.candidate_sha256) {
                    retained_sources.extend(hashes);
                    retained_icons.extend(icon);
                    continue;
                }
                obsolete_sources.extend(hashes);
                obsolete_icons.extend(icon);
                // Only exact hash-named, verified local generations are owned here.
                let resolved = fs::canonicalize(entry.path())?;
                if resolved.parent() == Some(canonical.as_path()) {
                    fs::remove_dir_all(resolved)?;
                }
            }
        }
        let icons_root = self.roots.icon_cache();
        if let Ok(generations) = fs::canonicalize(icons_root.join("generations")) {
            for identity in obsolete_icons.difference(&retained_icons) {
                if crate::icon_cache::open_generation(&icons_root, identity).is_ok() {
                    let path = fs::canonicalize(generations.join(identity))?;
                    if path.parent() == Some(generations.as_path()) {
                        fs::remove_dir_all(path)?;
                    }
                }
            }
        }
        let source_root = fs::canonicalize(self.roots.source_cache())?;
        for digest in obsolete_sources.difference(&retained_sources) {
            let path = source_root.join(format!("{digest}.bin"));
            if let Ok(metadata) = fs::symlink_metadata(&path) {
                if metadata.is_file() && !is_link_like(&metadata) {
                    fs::remove_file(path)?;
                }
            }
        }
        let mut receipts = fs::read_dir(self.roots.receipts())?
            .take(10_000)
            .filter_map(Result::ok)
            .filter_map(|entry| {
                let name = entry.file_name().to_string_lossy().into_owned();
                let sequence = name
                    .strip_suffix(".json")?
                    .strip_prefix("event-")
                    .unwrap_or(name.strip_suffix(".json")?)
                    .parse::<u64>()
                    .ok()?;
                Some((sequence, name, entry.path()))
            })
            .collect::<Vec<_>>();
        receipts.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| b.1.cmp(&a.1)));
        let mut complete_count = 0;
        let mut event_count = 0;
        for (_, name, path) in receipts {
            let count = if name.starts_with("event-") {
                &mut event_count
            } else {
                &mut complete_count
            };
            *count += 1;
            if *count > 16 {
                let metadata = fs::symlink_metadata(&path)?;
                if metadata.is_file() && !is_link_like(&metadata) {
                    fs::remove_file(path)?;
                }
            }
        }
        Ok(())
    }

    fn ensure_retention_capacity(&self, candidate: &CandidateSummary) -> Result<(), UpdateError> {
        // This bounds publication, never evicts support for historical domains.
        const MAX_DOMAINS: usize = 256;
        const MAX_RETAINED_BYTES: u64 = 4 * 1024 * 1024 * 1024;
        let entries = fs::read_dir(self.roots.live_versions())?
            .take(1025)
            .collect::<Result<Vec<_>, _>>()?;
        if entries.len() > 1024 {
            return Err(UpdateError::Validation("catalog storage pressure: too many retained generations; existing data was preserved".into()));
        }
        let mut domains = BTreeMap::new();
        domains.insert(
            (
                candidate.channel,
                candidate.api_version,
                candidate.game_version.clone(),
            ),
            candidate.total_bytes,
        );
        for entry in entries {
            let Ok(summary) = inspect_candidate(entry.path()) else {
                continue;
            };
            let key = (summary.channel, summary.api_version, summary.game_version);
            domains
                .entry(key)
                .and_modify(|size: &mut u64| *size = (*size).max(summary.total_bytes))
                .or_insert(summary.total_bytes);
        }
        let total = domains
            .values()
            .try_fold(0_u64, |sum, size| sum.checked_add(*size));
        if domains.len() > MAX_DOMAINS || total.is_none_or(|total| total > MAX_RETAINED_BYTES) {
            return Err(UpdateError::Validation("catalog storage pressure: publication paused to preserve historical version support".into()));
        }
        Ok(())
    }

    fn accepted_targets(&self) -> Result<Vec<CatalogTarget>, UpdateError> {
        let path = self.roots.root().join("accepted-domains.json");
        if !path.exists() {
            return Ok(Vec::new());
        }
        let canonical = fs::canonicalize(self.roots.root())?;
        let bytes = read_bounded_stable(&canonical, &path, 64 * 1024).map_err(stable_read_error)?;
        let targets: Vec<CatalogTarget> = serde_json::from_slice(&bytes)?;
        if targets.len() > 256 {
            return Err(UpdateError::InvalidSelection(
                "accepted version domain index exceeds its bound".into(),
            ));
        }
        for target in &targets {
            target.validate()?;
        }
        Ok(targets)
    }

    fn register_accepted(&self, selection: &CatalogSelection) -> Result<(), UpdateError> {
        let mut targets = self.accepted_targets()?;
        targets.push(selection.active.clone());
        targets.extend(selection.previous.clone());
        let mut domains = BTreeMap::new();
        for target in targets {
            let CatalogTarget::User {
                ref candidate_sha256,
            } = target
            else {
                continue;
            };
            let directory = self.roots.live_versions().join(candidate_sha256);
            let Ok(summary) = self.validate_installed_candidate(&directory, candidate_sha256)
            else {
                continue;
            };
            let Ok(bundle) = read_bundle(&directory.join("catalog.sqlite")) else {
                continue;
            };
            let key = (summary.channel, summary.api_version, summary.game_version);
            let order = (latest_observation(&bundle), summary.candidate_sha256);
            if domains
                .get(&key)
                .is_none_or(|(previous, _)| &order > previous)
            {
                domains.insert(key, (order, target));
            }
        }
        let targets = domains
            .into_values()
            .map(|(_, target)| target)
            .collect::<Vec<_>>();
        let path = self.roots.root().join("accepted-domains.json");
        let bytes = canonical_json(&targets)?;
        if fs::read(&path).ok().as_deref() == Some(bytes.as_slice()) {
            return Ok(());
        }
        write_atomic(&path, &bytes)
    }

    pub fn load_selection(&self) -> Result<CatalogSelection, UpdateError> {
        if !self.roots.selection().exists() {
            return Ok(CatalogSelection::default());
        }
        let canonical_root = fs::canonicalize(self.roots.root())?;
        let bytes = read_bounded_stable(&canonical_root, &self.roots.selection(), SELECTION_LIMIT)
            .map_err(stable_read_error)?;
        let selection: CatalogSelection = serde_json::from_slice(&bytes)?;
        selection.validate()?;
        if canonical_json(&selection)? != bytes {
            return Err(UpdateError::InvalidSelection(
                "selection file is not in canonical project format".into(),
            ));
        }
        Ok(selection)
    }

    pub fn resolve_catalog(&self) -> CatalogResolution {
        if !self.roots.selection().exists()
            && self
                .accepted_targets()
                .is_ok_and(|targets| !targets.is_empty())
        {
            return self.retained_resolution(None, "The catalog selection is missing.".into());
        }
        let selection = match self.load_selection() {
            Ok(selection) => selection,
            Err(error) => {
                return self.retained_resolution(
                    None,
                    format!("User catalog selection was ignored: {error}"),
                )
            }
        };
        match &selection.active {
            CatalogTarget::Bundled => self.bundled_resolution(None),
            CatalogTarget::User { candidate_sha256 } => {
                let candidate = self.roots.live_versions().join(candidate_sha256);
                match self.validate_installed_candidate(&candidate, candidate_sha256) {
                    Ok(_) => {
                        let path = candidate.join("catalog.sqlite");
                        let access = CatalogAccess::open_or_empty(&path);
                        if access.is_available() {
                            CatalogResolution {
                                access,
                                path,
                                target: selection.active,
                                warning: None,
                            }
                        } else {
                            self.retained_resolution(
                                selection.previous.as_ref(),
                                "The selected catalog could not be opened.".into(),
                            )
                        }
                    }
                    Err(error) => self.retained_resolution(
                        selection.previous.as_ref(),
                        format!("The selected catalog failed verification: {error}"),
                    ),
                }
            }
        }
    }

    fn retained_resolution(
        &self,
        preferred: Option<&CatalogTarget>,
        warning: String,
    ) -> CatalogResolution {
        let mut targets = preferred.cloned().into_iter().collect::<Vec<_>>();
        if let Ok(entries) = fs::read_dir(self.roots.receipts()) {
            let mut receipts = entries
                .take(256)
                .filter_map(Result::ok)
                .filter_map(|entry| {
                    let number = entry
                        .file_name()
                        .to_string_lossy()
                        .strip_suffix(".json")?
                        .parse::<u64>()
                        .ok()?;
                    Some((number, entry.path()))
                })
                .collect::<Vec<_>>();
            receipts.sort_by_key(|value| std::cmp::Reverse(value.0));
            if let Ok(root) = fs::canonicalize(self.roots.receipts()) {
                for (_, path) in receipts {
                    if let Ok(bytes) = read_bounded_stable(&root, &path, SELECTION_LIMIT) {
                        if let Ok(receipt) = serde_json::from_slice::<UpdateReceipt>(&bytes) {
                            targets.extend(receipt.new_target);
                            targets.push(receipt.old_target);
                        }
                    }
                }
            }
        }
        targets.extend(self.accepted_targets().unwrap_or_default());
        let mut checked = BTreeSet::new();
        for target in targets {
            let CatalogTarget::User {
                ref candidate_sha256,
            } = target
            else {
                continue;
            };
            if !checked.insert(candidate_sha256.clone()) {
                continue;
            }
            let candidate = self.roots.live_versions().join(candidate_sha256);
            if self
                .validate_installed_candidate(&candidate, candidate_sha256)
                .is_ok()
            {
                let path = candidate.join("catalog.sqlite");
                let access = CatalogAccess::open_or_empty(&path);
                if access.is_available() {
                    return CatalogResolution {
                        access,
                        path,
                        target,
                        warning: Some(format!(
                            "{warning} Recovered the last verified retained catalog."
                        )),
                    };
                }
            }
        }
        self.bundled_resolution(Some(warning))
    }

    pub fn install<F>(
        &self,
        candidate_path: impl AsRef<Path>,
        origin_acknowledged: bool,
        cancel: &CancellationToken,
        mut progress: F,
    ) -> Result<InstallOutcome, UpdateError>
    where
        F: FnMut(UpdateProgress),
    {
        if !origin_acknowledged {
            return Err(UpdateError::OriginNotAcknowledged);
        }
        cancel.check()?;
        self.prepare()?;
        progress(UpdateProgress::new(
            UpdateStage::WaitingForLock,
            0,
            0,
            "Waiting for exclusive access to catalog files; another operation may be using them",
        ));
        let _lock = OperationLock::acquire(&self.roots.operation_lock())?;
        self.install_inner(candidate_path.as_ref(), cancel, progress)
    }

    fn install_inner<F>(
        &self,
        candidate_path: &Path,
        cancel: &CancellationToken,
        mut progress: F,
    ) -> Result<InstallOutcome, UpdateError>
    where
        F: FnMut(UpdateProgress),
    {
        cancel.check()?;
        let previous_resolution = self.resolve_catalog();
        let previous_target = previous_resolution.target;
        let active_release = previous_resolution
            .access
            .release()
            .map_err(|error| UpdateError::Validation(error.to_string()))?;
        let source = fs::canonicalize(candidate_path)?;
        let import_root = fs::canonicalize(self.roots.import_root())?;
        if !is_same_or_nested(&source, &import_root) {
            return Err(UpdateError::Validation(
                "candidate must be inside the catalog import directory".into(),
            ));
        }
        progress(UpdateProgress::new(
            UpdateStage::Validating,
            0,
            0,
            "Verifying reviewed catalog candidate",
        ));
        let candidate = inspect_candidate(&source)?;
        self.ensure_retention_capacity(&candidate)?;
        if candidate.channel != self.roots.channel || candidate.catalog_schema != SCHEMA_VERSION {
            return Err(UpdateError::Validation(
                "catalog candidate does not match this context channel or schema".into(),
            ));
        }
        if self.roots.channel == Channel::Live {
            candidate_compatibility(&candidate, active_release.as_ref())?;
        } else if let Some(active) =
            active_release.filter(|active| active.channel == candidate.channel)
        {
            if candidate.api_version < active.api_version
                || (candidate.api_version == active.api_version
                    && client_version_components(&candidate.game_version)
                        .zip(client_version_components(&active.game_version))
                        .is_some_and(|(candidate, active)| candidate < active))
            {
                return Err(UpdateError::Validation(
                    "catalog is older than the selected context version".into(),
                ));
            }
        }
        cancel.check()?;

        let temporary = tempfile::Builder::new()
            .prefix(".update-")
            .tempdir_in(self.roots.staging())?;
        let staged = temporary
            .path()
            .join(self.roots.channel.as_str())
            .join(&candidate.candidate_sha256);
        fs::create_dir_all(&staged)?;
        let mut copied = 0;
        for name in CANDIDATE_FILES {
            copy_candidate_file(
                &source,
                &source.join(name),
                &staged.join(name),
                candidate.total_bytes,
                &mut copied,
                cancel,
                &mut progress,
            )?;
        }
        let staged_summary = inspect_candidate(&staged)?;
        if staged_summary != candidate {
            return Err(UpdateError::Validation(
                "staged candidate identity changed during installation".into(),
            ));
        }
        cancel.check()?;

        let destination = self.roots.live_versions().join(&candidate.candidate_sha256);
        if destination.exists() {
            self.validate_installed_candidate(&destination, &candidate.candidate_sha256)?;
        } else {
            fs::rename(&staged, &destination)?;
        }
        progress(UpdateProgress::new(
            UpdateStage::Opening,
            candidate.total_bytes,
            candidate.total_bytes,
            "Opening installed catalog read-only",
        ));
        validate_catalog_access(&destination.join("catalog.sqlite"), self.roots.channel)?;
        cancel.check()?;

        // Selection replacement is intentionally a short noninterruptible boundary.
        progress(UpdateProgress::new(
            UpdateStage::Committing,
            candidate.total_bytes,
            candidate.total_bytes,
            "Selecting installed catalog",
        ));
        let _commit = cancel
            .0
            .commit
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        cancel.check()?;
        let previous = self.load_selection()?;
        let sequence = previous.generation.saturating_add(1);
        let selection = CatalogSelection {
            schema_version: contract::SELECTION_SCHEMA_VERSION,
            generation: sequence,
            active: CatalogTarget::User {
                candidate_sha256: candidate.candidate_sha256.clone(),
            },
            previous: Some(previous_target),
            last_receipt: Some(sequence),
        };
        self.write_selection(&selection)?;
        drop(_commit);
        let mut receipt_warning = self
            .write_receipt(
                UpdateOperation::Install,
                &selection,
                Some(&candidate),
                Vec::new(),
            )
            .err()
            .map(|error| {
                format!("Catalog selection succeeded, but its receipt could not be saved: {error}")
            });
        if let Err(error) = self.register_accepted(&selection) {
            let warning = format!("Catalog selection succeeded, but its historical domain index could not be saved: {error}");
            receipt_warning = Some(match receipt_warning {
                Some(previous) => format!("{previous} {warning}"),
                None => warning,
            });
        }
        progress(UpdateProgress::new(
            UpdateStage::Complete,
            candidate.total_bytes,
            candidate.total_bytes,
            "Catalog installation complete",
        ));
        Ok(InstallOutcome {
            candidate,
            selection,
            receipt_warning,
        })
    }

    pub fn rollback<F>(
        &self,
        cancel: &CancellationToken,
        mut progress: F,
    ) -> Result<RollbackOutcome, UpdateError>
    where
        F: FnMut(UpdateProgress),
    {
        cancel.check()?;
        self.prepare()?;
        progress(UpdateProgress::new(
            UpdateStage::WaitingForLock,
            0,
            0,
            "Waiting for exclusive access to catalog files; another operation may be using them",
        ));
        let _lock = OperationLock::acquire(&self.roots.operation_lock())?;
        cancel.check()?;
        let current = self.load_selection()?;
        let target = current
            .previous
            .clone()
            .ok_or(UpdateError::NoRollbackTarget)?;
        progress(UpdateProgress::new(
            UpdateStage::RollingBack,
            0,
            0,
            "Verifying rollback catalog",
        ));
        self.validate_target(&target)?;
        cancel.check()?;

        progress(UpdateProgress::new(
            UpdateStage::Committing,
            0,
            0,
            "Selecting rollback catalog",
        ));
        let _commit = cancel
            .0
            .commit
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        cancel.check()?;
        let sequence = current.generation.saturating_add(1);
        let selection = CatalogSelection {
            schema_version: contract::SELECTION_SCHEMA_VERSION,
            generation: sequence,
            active: target,
            previous: Some(current.active),
            last_receipt: Some(sequence),
        };
        self.write_selection(&selection)?;
        drop(_commit);
        let receipt_warning = self
            .write_receipt(UpdateOperation::Rollback, &selection, None, Vec::new())
            .err()
            .map(|error| {
                format!("Catalog rollback succeeded, but its receipt could not be saved: {error}")
            });
        progress(UpdateProgress::new(
            UpdateStage::Complete,
            0,
            0,
            "Catalog rollback complete",
        ));
        Ok(RollbackOutcome {
            selection,
            receipt_warning,
        })
    }

    fn bundled_resolution(&self, warning: Option<String>) -> CatalogResolution {
        let mut access = CatalogAccess::open_or_empty(&self.bundled_catalog);
        if access
            .release()
            .ok()
            .flatten()
            .is_some_and(|release| release.channel != self.roots.channel)
        {
            access = CatalogAccess::empty(crate::catalog::CatalogDiagnosticKind::Unavailable,"No packaged catalog exists for this environment; naturally saved data will populate its separate registry.");
        }
        CatalogResolution {
            access,
            path: self.bundled_catalog.clone(),
            target: CatalogTarget::Bundled,
            warning,
        }
    }

    fn validate_target(&self, target: &CatalogTarget) -> Result<(), UpdateError> {
        match target {
            CatalogTarget::Bundled => {
                validate_catalog_access(&self.bundled_catalog, self.roots.channel)
            }
            CatalogTarget::User { candidate_sha256 } => {
                let path = self.roots.live_versions().join(candidate_sha256);
                self.validate_installed_candidate(&path, candidate_sha256)?;
                validate_catalog_access(&path.join("catalog.sqlite"), self.roots.channel)
            }
        }
    }

    fn validate_installed_candidate(
        &self,
        path: &Path,
        expected_hash: &str,
    ) -> Result<CandidateSummary, UpdateError> {
        let summary = inspect_candidate(path)?;
        if summary.channel != self.roots.channel || summary.catalog_schema != SCHEMA_VERSION {
            return Err(UpdateError::Validation(
                "installed catalog does not match this channel or schema".into(),
            ));
        }
        if summary.candidate_sha256 != expected_hash {
            return Err(UpdateError::Validation(
                "installed directory does not match its candidate identity".into(),
            ));
        }
        Ok(summary)
    }

    fn active_catalog_path(&self) -> PathBuf {
        self.resolve_catalog().path
    }

    fn write_selection(&self, selection: &CatalogSelection) -> Result<(), UpdateError> {
        selection.validate()?;
        write_atomic(&self.roots.selection(), &canonical_json(selection)?)
    }

    fn write_receipt(
        &self,
        operation: UpdateOperation,
        selection: &CatalogSelection,
        candidate: Option<&CandidateSummary>,
        finding_codes: Vec<String>,
    ) -> Result<(), UpdateError> {
        let receipt = UpdateReceipt {
            schema_version: 1,
            operation,
            sequence: selection.generation,
            result: UpdateResult::Complete,
            stage: UpdateStage::Complete,
            old_target: selection.previous.clone().unwrap_or(CatalogTarget::Bundled),
            new_target: Some(selection.active.clone()),
            candidate_sha256: candidate.map(|value| value.candidate_sha256.clone()),
            catalog_semantic_sha256: candidate.map(|value| value.catalog_semantic_sha256.clone()),
            channel: candidate.map(|value| value.channel),
            catalog_version: candidate.map(|value| value.catalog_version.clone()),
            game_version: candidate.map(|value| value.game_version.clone()),
            api_version: candidate.map(|value| value.api_version),
            source_count: candidate.map_or(0, |value| value.source_count),
            finding_codes,
        };
        write_atomic(
            &self
                .roots
                .receipts()
                .join(format!("{}.json", selection.generation)),
            &canonical_json(&receipt)?,
        )
    }
}

fn validate_live_candidate(candidate: &CandidateSummary) -> Result<(), UpdateError> {
    if candidate.channel != Channel::Live {
        return Err(UpdateError::Validation(
            "The Catalog environment does not match the Live context. Existing environment-specific generations remain separate.".into(),
        ));
    }
    if candidate.catalog_schema != SCHEMA_VERSION {
        return Err(UpdateError::Validation(format!(
            "This proposed catalog uses unsupported data format {}. The active generation remains unchanged.",
            candidate.catalog_schema
        )));
    }
    Ok(())
}

pub(crate) fn candidate_compatibility(
    candidate: &CandidateSummary,
    active: Option<&crate::catalog::CatalogRelease>,
) -> Result<(), UpdateError> {
    validate_live_candidate(candidate)?;
    let Some(active) = active else {
        return Ok(());
    };
    let older = candidate.api_version < active.api_version
        || (candidate.api_version == active.api_version
            && client_version_components(&candidate.game_version)
                .zip(client_version_components(&active.game_version))
                .is_some_and(|(candidate, active)| candidate < active));
    if older {
        return Err(UpdateError::Validation(
            "This proposed catalog is older than the selected Live catalog. The active generation remains unchanged.".into(),
        ));
    }
    Ok(())
}

fn client_version_components(value: &str) -> Option<Vec<u64>> {
    let token = value.split_whitespace().next()?;
    let token = token
        .strip_prefix("eso.live.")
        .or_else(|| token.strip_prefix("eso.pts."))
        .unwrap_or(token);
    let mut components = token
        .split('.')
        .map(str::parse)
        .collect::<Result<Vec<u64>, _>>()
        .ok()?;
    while components.len() > 2 && components.last() == Some(&0) {
        components.pop();
    }
    (!components.is_empty()).then_some(components)
}

fn validate_catalog_access(path: &Path, channel: Channel) -> Result<(), UpdateError> {
    let access = CatalogAccess::open_or_empty(path);
    let release = access
        .release()
        .map_err(|error| UpdateError::Validation(error.to_string()))?
        .ok_or_else(|| {
            UpdateError::Validation(access.diagnostic().map_or_else(
                || "catalog is unavailable".into(),
                |value| value.message.clone(),
            ))
        })?;
    if release.channel != channel || release.schema_version != SCHEMA_VERSION {
        return Err(UpdateError::Validation(
            "selected catalog does not match the context channel or schema".into(),
        ));
    }
    Ok(())
}

fn copy_candidate_file<F>(
    canonical_source_root: &Path,
    source: &Path,
    destination: &Path,
    total: u64,
    copied: &mut u64,
    cancel: &CancellationToken,
    progress: &mut F,
) -> Result<(), UpdateError>
where
    F: FnMut(UpdateProgress),
{
    let mut input =
        open_bounded_stable(canonical_source_root, source, total).map_err(stable_capture_error)?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(destination)?;
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        cancel.check()?;
        let count = input.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        output.write_all(&buffer[..count])?;
        *copied = copied.saturating_add(count as u64);
        progress(UpdateProgress::new(
            UpdateStage::Installing,
            *copied,
            total,
            "Copying verified catalog files",
        ));
    }
    output.sync_all()?;
    Ok(())
}

fn ensure_real_directory(path: &Path) -> Result<(), UpdateError> {
    if path.exists() {
        let metadata = fs::symlink_metadata(path)?;
        if !metadata.is_dir() || is_link_like(&metadata) {
            return Err(UpdateError::Validation(
                "catalog storage path must be a real directory".into(),
            ));
        }
    } else {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

fn canonical_json(value: &impl serde::Serialize) -> Result<Vec<u8>, UpdateError> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn stable_read_error(error: StableReadError) -> UpdateError {
    match error {
        StableReadError::Io(error) => UpdateError::Io(error),
        StableReadError::Invalid(message) => UpdateError::InvalidSelection(message.into()),
    }
}

fn stable_capture_error(error: StableReadError) -> UpdateError {
    match error {
        StableReadError::Io(error) => UpdateError::Io(error),
        StableReadError::Invalid(message) => UpdateError::Validation(message.into()),
    }
}

fn fingerprint_capture(path: &Path) -> Result<CaptureFingerprint, UpdateError> {
    let parent = fs::canonicalize(
        path.parent()
            .ok_or_else(|| UpdateError::Validation("capture has no parent".into()))?,
    )?;
    let bytes =
        read_bounded_stable(&parent, path, MAX_CAPTURE_BYTES).map_err(stable_capture_error)?;
    let Some(identity) = canonical_catalog_capture(&bytes).map_err(PipelineError::Collector)?
    else {
        return Ok(empty_capture_fingerprint());
    };
    Ok(CaptureFingerprint {
        size: identity.len() as u64,
        modified_unix_nanos: None,
        sha256: sha256_bytes(&identity),
    })
}

fn empty_capture_fingerprint() -> CaptureFingerprint {
    CaptureFingerprint {
        size: 0,
        modified_unix_nanos: None,
        sha256: "0".repeat(64),
    }
}

fn sha256_bytes(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), UpdateError> {
    let parent = path
        .parent()
        .ok_or_else(|| UpdateError::Validation("atomic file has no parent".into()))?;
    let mut temporary = tempfile::Builder::new()
        .prefix(".catalog-selection-")
        .tempfile_in(parent)?;
    temporary.as_file_mut().write_all(bytes)?;
    temporary.as_file().sync_all()?;
    crate::atomic_file::persist(temporary.into_temp_path(), path)?;
    Ok(())
}

struct OperationLock(File);

impl OperationLock {
    fn open(path: &Path) -> Result<File, UpdateError> {
        Ok(OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?)
    }

    fn acquire(path: &Path) -> Result<Self, UpdateError> {
        let file = Self::open(path)?;
        file.try_lock().map_err(|error| match error {
            std::fs::TryLockError::WouldBlock => UpdateError::ConcurrentOperation,
            std::fs::TryLockError::Error(error) => UpdateError::Io(error),
        })?;
        Ok(Self(file))
    }
}

impl Drop for OperationLock {
    fn drop(&mut self) {
        let _ = self.0.unlock();
    }
}
