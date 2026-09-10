//! Verified, user-controlled catalog installation, selection, and rollback.

mod availability;
mod contract;
mod worker;

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use serde_json::json;
use sha2::{Digest, Sha256};

pub use availability::{
    resolve_availability, AvailabilityInput, CatalogAvailability, CheckFreshness, LiveUpdateState,
};
pub use contract::{
    CatalogSelection, CatalogTarget, UpdateOperation, UpdateProgress, UpdateReceipt, UpdateResult,
    UpdateStage,
};
pub use worker::{CatalogUpdateWorker, WorkerEvent, WorkerFailure};

use crate::bounded_file::{
    is_link_like, is_same_or_nested, open_bounded_stable, read_bounded_stable, StableReadError,
};
use crate::catalog::schema::SCHEMA_VERSION;
use crate::catalog::version::parse_commit_message_version;
use crate::catalog::{CatalogAccess, Channel};
use crate::catalog_pipeline::{build_candidate_with_cancel, PipelineRun};
use crate::catalog_pipeline::{
    inspect_candidate, CandidateSummary, PipelineError, CANDIDATE_FILES,
};
use crate::collector::{parse_capture, MAX_CAPTURE_BYTES};

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
pub struct CancellationToken(Arc<AtomicBool>);

impl CancellationToken {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cancel(&self) {
        self.0.store(true, Ordering::Release);
    }

    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::Acquire)
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
}

impl UpdateRoots {
    fn new(config: &Path) -> Self {
        Self {
            root: config.join("catalog"),
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
        self.root.join("versions").join("live")
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

    fn receipt_lock(&self) -> PathBuf {
        self.root.join("receipt.lock")
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
        Self {
            roots: UpdateRoots::new(config.as_ref()),
            bundled_catalog: bundled_catalog.as_ref().to_path_buf(),
        }
    }

    pub fn roots(&self) -> UpdateRoots {
        self.roots.clone()
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
        for entry in fs::read_dir(self.roots.staging())?.take(256) {
            let entry = entry?;
            let name = entry.file_name();
            if !name.to_string_lossy().starts_with(".update-") {
                continue;
            }
            let metadata = fs::symlink_metadata(entry.path())?;
            if metadata.is_dir() && !is_link_like(&metadata) {
                fs::remove_dir_all(entry.path())?;
                recovered += 1;
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
            return Ok(CaptureFingerprint {
                size: 0,
                modified_unix_nanos: None,
                sha256: "0".repeat(64),
            });
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
        cancel.check()?;
        self.prepare()?;
        progress(UpdateProgress::new(
            UpdateStage::WaitingForCapture,
            0,
            0,
            "Checking for a SavedVariables flush after the waiting boundary",
        ));
        let first = fingerprint_capture(capture_path.as_ref())?;
        if first.sha256 == waiting_fingerprint.sha256 {
            return Err(UpdateError::Validation(
                "collector capture is unchanged; run /reloadui, log out, or exit ESO first".into(),
            ));
        }
        let second = fingerprint_capture(capture_path.as_ref())?;
        if first != second {
            return Err(UpdateError::Validation(
                "collector capture is still changing; wait for the save to finish".into(),
            ));
        }
        let canonical_parent = fs::canonicalize(
            capture_path
                .as_ref()
                .parent()
                .ok_or_else(|| UpdateError::Validation("capture has no parent".into()))?,
        )?;
        let capture_bytes =
            read_bounded_stable(&canonical_parent, capture_path.as_ref(), MAX_CAPTURE_BYTES)
                .map_err(stable_capture_error)?;
        let envelope = parse_capture(&capture_bytes).map_err(PipelineError::Collector)?;
        if envelope.channel != Channel::Live {
            return Err(UpdateError::Validation(
                "only a complete Live collector capture can build an active candidate".into(),
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
        fs::copy(&baseline_source, &baseline)?;
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
                "channel": "live",
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
                "channel": "live",
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
            "baseline": "baseline.sqlite",
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
            "Building a local review candidate through the S073 pipeline",
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
        let selection = match self.load_selection() {
            Ok(selection) => selection,
            Err(error) => {
                return self.bundled_resolution(Some(format!(
                    "User catalog selection was ignored: {error}"
                )))
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
                            self.bundled_resolution(Some(
                                "The selected user catalog could not be opened; using the bundled catalog."
                                    .into(),
                            ))
                        }
                    }
                    Err(error) => self.bundled_resolution(Some(format!(
                        "The selected user catalog failed verification; using the bundled catalog: {error}"
                    ))),
                }
            }
        }
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
            "Waiting for catalog operation lock",
        ));
        let _lock = OperationLock::acquire(&self.roots.operation_lock())?;
        cancel.check()?;
        let previous_resolution = self.resolve_catalog();
        let previous_target = previous_resolution.target;
        let active_release = previous_resolution
            .access
            .release()
            .map_err(|error| UpdateError::Validation(error.to_string()))?;
        let source = fs::canonicalize(candidate_path.as_ref())?;
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
        candidate_compatibility(&candidate, active_release.as_ref())?;
        cancel.check()?;

        let temporary = tempfile::Builder::new()
            .prefix(".update-")
            .tempdir_in(self.roots.staging())?;
        let staged = temporary
            .path()
            .join("live")
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
        validate_catalog_access(&destination.join("catalog.sqlite"))?;
        cancel.check()?;

        // Selection replacement is intentionally a short noninterruptible boundary.
        progress(UpdateProgress::new(
            UpdateStage::Committing,
            candidate.total_bytes,
            candidate.total_bytes,
            "Selecting installed catalog",
        ));
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
        let receipt_warning = self
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
            "Waiting for catalog operation lock",
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
        let sequence = current.generation.saturating_add(1);
        let selection = CatalogSelection {
            schema_version: contract::SELECTION_SCHEMA_VERSION,
            generation: sequence,
            active: target,
            previous: Some(current.active),
            last_receipt: Some(sequence),
        };
        self.write_selection(&selection)?;
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
        CatalogResolution {
            access: CatalogAccess::open_or_empty(&self.bundled_catalog),
            path: self.bundled_catalog.clone(),
            target: CatalogTarget::Bundled,
            warning,
        }
    }

    fn validate_target(&self, target: &CatalogTarget) -> Result<(), UpdateError> {
        match target {
            CatalogTarget::Bundled => validate_catalog_access(&self.bundled_catalog),
            CatalogTarget::User { candidate_sha256 } => {
                let path = self.roots.live_versions().join(candidate_sha256);
                self.validate_installed_candidate(&path, candidate_sha256)?;
                validate_catalog_access(&path.join("catalog.sqlite"))
            }
        }
    }

    fn validate_installed_candidate(
        &self,
        path: &Path,
        expected_hash: &str,
    ) -> Result<CandidateSummary, UpdateError> {
        let summary = inspect_candidate(path)?;
        validate_live_candidate(&summary)?;
        if summary.candidate_sha256 != expected_hash {
            return Err(UpdateError::Validation(
                "installed directory does not match its candidate identity".into(),
            ));
        }
        Ok(summary)
    }

    fn import_candidate_path(&self, candidate_sha256: &str) -> Result<PathBuf, UpdateError> {
        CatalogTarget::User {
            candidate_sha256: candidate_sha256.to_string(),
        }
        .validate()?;
        for channel in ["live", "pts"] {
            let path = self
                .roots
                .import_root()
                .join(channel)
                .join(candidate_sha256);
            if path.exists() {
                return Ok(path);
            }
        }
        Err(UpdateError::Validation(
            "the selected imported candidate is no longer available".into(),
        ))
    }

    fn active_catalog_path(&self) -> PathBuf {
        match self.load_selection().map(|selection| selection.active) {
            Ok(CatalogTarget::User { candidate_sha256 }) => self
                .roots
                .live_versions()
                .join(candidate_sha256)
                .join("catalog.sqlite"),
            _ => self.bundled_catalog.clone(),
        }
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

    pub(crate) fn write_terminal_receipt(
        &self,
        operation: UpdateOperation,
        result: UpdateResult,
        stage: UpdateStage,
        candidate_sha256: Option<String>,
        finding_code: &str,
    ) -> Result<(), UpdateError> {
        self.prepare()?;
        let _lock = OperationLock::acquire_blocking(&self.roots.receipt_lock())?;
        let sequence = next_event_sequence(&self.roots.receipts())?;
        let old_target = self
            .load_selection()
            .map(|selection| selection.active)
            .unwrap_or(CatalogTarget::Bundled);
        let receipt = UpdateReceipt {
            schema_version: 1,
            operation,
            sequence,
            result,
            stage,
            old_target,
            new_target: None,
            candidate_sha256,
            catalog_semantic_sha256: None,
            channel: None,
            catalog_version: None,
            game_version: None,
            api_version: None,
            source_count: 0,
            finding_codes: vec![finding_code.to_string()],
        };
        write_atomic(
            &self.roots.receipts().join(format!("event-{sequence}.json")),
            &canonical_json(&receipt)?,
        )
    }
}

fn validate_live_candidate(candidate: &CandidateSummary) -> Result<(), UpdateError> {
    if candidate.channel != Channel::Live {
        return Err(UpdateError::Validation(
            "only Live candidates can become the active catalog".into(),
        ));
    }
    if candidate.catalog_schema != SCHEMA_VERSION {
        return Err(UpdateError::Validation(format!(
            "candidate schema {} is unsupported",
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
            && parse_commit_message_version(&candidate.game_version)
                < parse_commit_message_version(&active.game_version));
    if older {
        return Err(UpdateError::Validation(
            "candidate is older than the active Live catalog".into(),
        ));
    }
    Ok(())
}

fn validate_catalog_access(path: &Path) -> Result<(), UpdateError> {
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
    if release.channel != Channel::Live || release.schema_version != SCHEMA_VERSION {
        return Err(UpdateError::Validation(
            "selected catalog is not a supported Live catalog".into(),
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
    let metadata = fs::symlink_metadata(path)?;
    let modified_unix_nanos = metadata
        .modified()
        .ok()
        .and_then(|value| value.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|value| value.as_nanos());
    Ok(CaptureFingerprint {
        size: bytes.len() as u64,
        modified_unix_nanos,
        sha256: sha256_bytes(&bytes),
    })
}

fn sha256_bytes(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn next_event_sequence(receipts: &Path) -> Result<u64, UpdateError> {
    let mut maximum = 0_u64;
    for entry in fs::read_dir(receipts)?.take(10_000) {
        let name = entry?.file_name();
        let name = name.to_string_lossy();
        let Some(number) = name
            .strip_prefix("event-")
            .and_then(|value| value.strip_suffix(".json"))
            .and_then(|value| value.parse::<u64>().ok())
        else {
            continue;
        };
        maximum = maximum.max(number);
    }
    Ok(maximum.saturating_add(1))
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

    fn acquire_blocking(path: &Path) -> Result<Self, UpdateError> {
        let file = Self::open(path)?;
        file.lock()?;
        Ok(Self(file))
    }
}

impl Drop for OperationLock {
    fn drop(&mut self) {
        let _ = self.0.unlock();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Barrier};

    #[test]
    fn concurrent_terminal_receipts_keep_distinct_sequences() {
        let root = tempfile::tempdir().unwrap();
        let service = CatalogUpdateService::new(root.path(), root.path().join("bundled.sqlite"));
        service.prepare().unwrap();
        let participants = 8;
        let barrier = Arc::new(Barrier::new(participants));
        let handles = (0..participants)
            .map(|_| {
                let service = service.clone();
                let barrier = barrier.clone();
                std::thread::spawn(move || {
                    barrier.wait();
                    service
                        .write_terminal_receipt(
                            UpdateOperation::Install,
                            UpdateResult::Failed,
                            UpdateStage::Failed,
                            None,
                            "concurrent-test",
                        )
                        .unwrap();
                })
            })
            .collect::<Vec<_>>();
        for handle in handles {
            handle.join().unwrap();
        }
        let receipts = fs::read_dir(service.roots.receipts())
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_name().to_string_lossy().starts_with("event-"))
            .count();
        assert_eq!(receipts, participants);
    }
}
