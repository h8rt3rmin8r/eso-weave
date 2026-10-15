use std::collections::BTreeMap;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};

use crate::catalog::{CatalogAccess, CatalogDiagnosticKind, Channel};

use super::{
    append_capture_set, calculate_projection, delete_all, delete_encounter, ensure_distinct_paths,
    list_encounters, load_encounter, read_saved_source, CaptureControllerState,
    CaptureImportReport, CaptureStateSummary, CaptureStatus, DeleteReceipt, EncounterError,
    EncounterProjection, EncounterSummary, ImportRequest,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EncounterIdentity {
    pub session_id: String,
    pub encounter_id: String,
}

impl EncounterIdentity {
    pub fn new(session_id: impl Into<String>, encounter_id: impl Into<String>) -> Self {
        Self {
            session_id: session_id.into(),
            encounter_id: encounter_id.into(),
        }
    }
}

impl From<&EncounterSummary> for EncounterIdentity {
    fn from(summary: &EncounterSummary) -> Self {
        Self::new(&summary.session_id, &summary.encounter_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryDiagnosticKind {
    SourceUnavailable,
    SourceInaccessible,
    SourceInvalid,
    SourceVersionUnsupported,
    SourceEnvironmentMismatch,
    StoreInvalid,
    CatalogUnavailable,
    CatalogInvalid,
    VersionMismatch,
    EncounterMissing,
    OperationFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryDiagnostic {
    pub kind: HistoryDiagnosticKind,
    pub message: String,
}

impl HistoryDiagnostic {
    fn new(kind: HistoryDiagnosticKind, message: &'static str) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub fn source_unavailable() -> Self {
        Self::new(
            HistoryDiagnosticKind::SourceUnavailable,
            "The retained saved addon data location is unavailable. Existing local history remains unchanged.",
        )
    }
}

#[derive(Debug, Clone)]
pub struct EncounterHistoryService {
    store_path: PathBuf,
    catalog_path: Arc<RwLock<PathBuf>>,
    catalog_registry_root: Option<PathBuf>,
    catalog_registry_paths: Arc<RwLock<BTreeMap<PathBuf, PathBuf>>>,
    catalog_generation: Arc<AtomicU64>,
    catalog_commit: Arc<Mutex<()>>,
}

impl EncounterHistoryService {
    pub fn new(application_root: impl AsRef<Path>, catalog_path: impl Into<PathBuf>) -> Self {
        let mut service = Self::from_paths(
            application_root
                .as_ref()
                .join("encounters")
                .join("encounters.sqlite"),
            catalog_path,
        );
        service.catalog_registry_root = Some(application_root.as_ref().to_path_buf());
        service
    }

    pub fn from_paths(store_path: impl Into<PathBuf>, catalog_path: impl Into<PathBuf>) -> Self {
        Self {
            store_path: store_path.into(),
            catalog_path: Arc::new(RwLock::new(catalog_path.into())),
            catalog_registry_root: None,
            catalog_registry_paths: Arc::new(RwLock::new(BTreeMap::new())),
            catalog_generation: Arc::new(AtomicU64::new(1)),
            catalog_commit: Arc::new(Mutex::new(())),
        }
    }

    pub fn store_path(&self) -> &Path {
        &self.store_path
    }

    pub fn set_catalog_path(&self, catalog_path: impl Into<PathBuf>) {
        let catalog_path = catalog_path.into();
        self.register_catalog_path(catalog_path.clone());
        let _admission = self
            .catalog_commit
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut current = self
            .catalog_path
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if *current != catalog_path {
            *current = catalog_path;
            self.catalog_generation.fetch_add(1, Ordering::AcqRel);
        }
    }

    /// Historical registries are shared by every service clone. Registration does
    /// not replace the main Live catalog or change any selected generation.
    pub fn register_catalog_path(&self, path: PathBuf) {
        let Some(registry) = crate::catalog_update::CatalogUpdateService::for_catalog_path(&path)
        else {
            return;
        };
        let Ok(root) = fs::canonicalize(registry.roots().root()) else {
            return;
        };
        if self
            .catalog_registry_paths
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .get(&root)
            == Some(&path)
        {
            return;
        }
        let Some(directory) = path.parent() else {
            return;
        };
        if crate::catalog_pipeline::inspect_candidate(directory).is_err() {
            return;
        }
        let _admission = self
            .catalog_commit
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut paths = self
            .catalog_registry_paths
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if paths.contains_key(&root) || paths.len() < 16 {
            paths.insert(root, path);
            self.catalog_generation.fetch_add(1, Ordering::AcqRel);
        }
    }

    pub fn snapshot(&self) -> Result<Vec<EncounterSummary>, HistoryDiagnostic> {
        match store_is_absent(&self.store_path) {
            Ok(true) => return Ok(Vec::new()),
            Ok(false) => {}
            Err(()) => return Err(invalid_store_diagnostic()),
        }
        list_encounters(&self.store_path).map_err(store_diagnostic)
    }

    pub fn import_current(
        &self,
        source_path: impl AsRef<Path>,
        expected_channel: Channel,
    ) -> Result<CaptureImportReport, HistoryDiagnostic> {
        let source_path = source_path.as_ref();
        let request = ImportRequest::new(source_path, &self.store_path, expected_channel);
        ensure_distinct_paths(source_path, &self.store_path).map_err(source_diagnostic)?;
        let (hash, parsed, _) =
            read_saved_source(source_path, expected_channel).map_err(source_diagnostic)?;
        // Parsing is complete before the store is opened. Store errors cannot
        // masquerade as source errors, and failed parsing never creates a store.
        append_capture_set(&request, hash, parsed).map_err(store_diagnostic)
    }

    pub fn inspect_source(
        &self,
        source_path: impl AsRef<Path>,
        expected_channel: Channel,
    ) -> Result<SavedSourceInspection, HistoryDiagnostic> {
        let source_path = source_path.as_ref();
        let (_, parsed, modified) =
            read_saved_source(source_path, expected_channel).map_err(source_diagnostic)?;
        let state = parsed.last_saved_summary();
        let sessions = parsed
            .sessions
            .iter()
            .filter(|state| state.session.is_some())
            .map(CaptureStateSummary::from)
            .collect();
        let spool_session_count = parsed
            .state
            .as_ref()
            .map_or(usize::from(!parsed.records.is_empty()), |state| {
                state.retained_sessions.len() + usize::from(state.session.is_some())
            });
        let spool_estimated_bytes = parsed.state.as_ref().map_or_else(
            || {
                parsed
                    .records
                    .iter()
                    .map(|record| record.capture.estimated_bytes)
                    .sum()
            },
            |state| {
                state
                    .retained_sessions
                    .values()
                    .chain(std::iter::once(state))
                    .filter_map(|controller| controller.session.as_ref())
                    .map(|session| session.aggregate_estimated_bytes)
                    .sum()
            },
        );

        let partial_count = parsed
            .records
            .iter()
            .filter(|record| record.capture.status == CaptureStatus::Partial)
            .count();
        Ok(SavedSourceInspection {
            path: source_path.to_path_buf(),
            channel: expected_channel,
            modified,
            sessions,
            spool_session_count,
            spool_estimated_bytes,
            finished_count: parsed.records.len() - partial_count,
            partial_count,
            readiness: SavedRecordingReadiness::from_state(state.as_ref(), parsed.records.len()),
            state,
        })
    }

    pub fn inspect_native_source(
        &self,
        source_path: impl AsRef<Path>,
        expected_channel: Channel,
    ) -> Result<super::native::NativeSourceInspection, HistoryDiagnostic> {
        super::native::inspect_native_log(source_path, expected_channel)
            .map_err(native_source_diagnostic)
    }

    pub fn import_native_current(
        &self,
        source_path: impl AsRef<Path>,
        expected_channel: Channel,
    ) -> Result<super::native::NativeImportReport, HistoryDiagnostic> {
        self.import_native_checked(source_path.as_ref(), expected_channel, None)
    }

    pub fn import_native_inspected(
        &self,
        source_path: impl AsRef<Path>,
        expected_channel: Channel,
        source_sha256: &str,
    ) -> Result<super::native::NativeImportReport, HistoryDiagnostic> {
        self.import_native_checked(source_path.as_ref(), expected_channel, Some(source_sha256))
    }

    pub fn native_detail(
        &self,
        identity: &EncounterIdentity,
    ) -> Result<Option<super::native::NativeProjection>, HistoryDiagnostic> {
        super::load_native_encounter(
            &self.store_path,
            &identity.session_id,
            &identity.encounter_id,
        )
        .map_err(store_diagnostic)?
        .map(|capture| super::native::project_native_encounter(&capture).map_err(store_diagnostic))
        .transpose()
    }

    fn import_native_checked(
        &self,
        source_path: &Path,
        expected_channel: Channel,
        expected_hash: Option<&str>,
    ) -> Result<super::native::NativeImportReport, HistoryDiagnostic> {
        ensure_distinct_paths(source_path, &self.store_path).map_err(native_source_diagnostic)?;
        let (hash, parsed, _) = super::native::read_native_source(source_path, expected_channel)
            .map_err(native_source_diagnostic)?;
        if expected_hash.is_some_and(|expected| expected != hash) {
            return Err(HistoryDiagnostic::new(HistoryDiagnosticKind::SourceInvalid, "The native source changed during observation. Existing local history remains unchanged."));
        }
        if parsed.encounters.is_empty() {
            return Ok(super::native::NativeImportReport {
                source_sha256: hash,
                imported_count: 0,
                already_present_count: 0,
                source: parsed.report,
                receipts: Vec::new(),
            });
        }
        let prepared = parsed
            .encounters
            .iter()
            .map(|capture| {
                let bytes = super::native::canonical_native_bytes(capture)
                    .map_err(native_source_diagnostic)?;
                let hash = super::sha256(&bytes);
                Ok((bytes, hash))
            })
            .collect::<Result<Vec<_>, HistoryDiagnostic>>()?;
        super::store::append_native_set(&self.store_path, &parsed, &prepared, hash)
            .map_err(store_diagnostic)
    }
    pub fn detail(
        &self,
        identity: &EncounterIdentity,
    ) -> Result<EncounterProjection, HistoryDiagnostic> {
        let capture = load_encounter(
            &self.store_path,
            &identity.session_id,
            &identity.encounter_id,
        )
        .map_err(store_diagnostic)?
        .ok_or_else(|| {
            HistoryDiagnostic::new(
                HistoryDiagnosticKind::EncounterMissing,
                "The requested encounter is no longer present.",
            )
        })?;

        self.project_capture(&capture)
    }

    fn project_capture(
        &self,
        capture: &super::EncounterCapture,
    ) -> Result<EncounterProjection, HistoryDiagnostic> {
        let catalog_path = self
            .catalog_path
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone();
        let mut catalog = CatalogAccess::open_or_empty(&catalog_path);
        let matches_version = catalog.release().ok().flatten().is_some_and(|release| {
            release.channel == capture.channel
                && release.api_version == capture.source.api_version
                && release.game_version == capture.source.game_version
        });
        if !matches_version {
            let mut registries =
                crate::catalog_update::CatalogUpdateService::for_catalog_path(&catalog_path)
                    .into_iter()
                    .collect::<Vec<_>>();
            let paths = self
                .catalog_registry_paths
                .read()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .values()
                .cloned()
                .collect::<Vec<_>>();
            registries.extend(paths.iter().filter_map(|path| {
                crate::catalog_update::CatalogUpdateService::for_catalog_path(path)
            }));
            if let Some(root) = &self.catalog_registry_root {
                registries.push(
                    crate::catalog_update::CatalogUpdateService::new_for_channel(
                        root,
                        &catalog_path,
                        capture.channel,
                    ),
                );
            }
            let resolution = registries
                .iter()
                .find_map(|registry| {
                    registry.resolve_catalog_for_version(
                        capture.channel,
                        capture.source.api_version,
                        &capture.source.game_version,
                    )
                })
                .or_else(|| {
                    registries.iter().find_map(|registry| {
                        registry
                            .resolve_catalog_for_api(capture.channel, capture.source.api_version)
                    })
                });
            if let Some(resolution) = resolution {
                catalog = resolution.access;
            }
        }
        if let Some(diagnostic) = catalog.diagnostic() {
            let (kind, message) = match diagnostic.kind {
                CatalogDiagnosticKind::Missing | CatalogDiagnosticKind::Unavailable => (
                    HistoryDiagnosticKind::CatalogUnavailable,
                    "Game-data definitions are unavailable, so encounter metrics cannot be calculated. Your imported recordings remain stored.",
                ),
                CatalogDiagnosticKind::Corrupt
                | CatalogDiagnosticKind::Incompatible
                | CatalogDiagnosticKind::Checksum => (
                    HistoryDiagnosticKind::CatalogInvalid,
                    "The catalog of game-data definitions is damaged or unsupported, so metrics cannot be calculated. Your imported recordings remain stored.",
                ),
            };
            return Err(HistoryDiagnostic::new(kind, message));
        }
        let release = catalog
            .release()
            .map_err(|_| {
                HistoryDiagnostic::new(
                    HistoryDiagnosticKind::CatalogInvalid,
                    "The catalog's game-data identity could not be checked, so metrics cannot be calculated. Your imported recordings remain stored.",
                )
            })?
            .ok_or_else(|| {
                HistoryDiagnostic::new(
                    HistoryDiagnosticKind::CatalogUnavailable,
                    "Game-data definitions are unavailable, so catalog-dependent encounter metrics are unavailable. Retained recordings remain stored.",
                )
            })?;
        if release.channel != capture.channel || release.api_version != capture.source.api_version {
            return Err(HistoryDiagnostic::new(
                HistoryDiagnosticKind::VersionMismatch,
                "Matching definitions for this recording's environment and game API version are unavailable, so metrics cannot be calculated. Your imported recording remains stored.",
            ));
        }

        calculate_projection(capture, &catalog).map_err(|_| {
            HistoryDiagnostic::new(
                HistoryDiagnosticKind::OperationFailed,
                "Metrics could not be calculated from this recording. The retained original remains stored; a controlled diagnostic is available in the application log.",
            )
        })
    }

    pub fn delete_one(
        &self,
        identity: &EncounterIdentity,
    ) -> Result<DeleteReceipt, HistoryDiagnostic> {
        delete_encounter(
            &self.store_path,
            &identity.session_id,
            &identity.encounter_id,
        )
        .map_err(store_diagnostic)
    }

    pub fn delete_all(&self) -> Result<DeleteReceipt, HistoryDiagnostic> {
        delete_all(&self.store_path).map_err(store_diagnostic)
    }

    /// A persistent domain processor, independent of selection and presentation.
    pub fn processor(&self) -> Result<EncounterProcessor, HistoryDiagnostic> {
        self.processor_with_cancellation(crate::catalog_update::CancellationToken::new())
    }

    pub fn processor_with_cancellation(
        &self,
        token: crate::catalog_update::CancellationToken,
    ) -> Result<EncounterProcessor, HistoryDiagnostic> {
        let writer = super::NativeIngestWriter::open_with_cancellation(&self.store_path, token)
            .map_err(store_diagnostic)?;
        Ok(EncounterProcessor {
            service: self.clone(),
            writer,
            context: None,
            cursor: super::store::PendingCursor::default(),
        })
    }

    fn processing_context(&self) -> String {
        let active = self
            .catalog_path
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone();
        let mut paths = self
            .catalog_registry_paths
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .values()
            .cloned()
            .collect::<Vec<_>>();
        paths.push(active);
        paths.sort();
        paths.dedup();
        // Stable immutable Catalog paths and actual release receipts survive
        // restart. Only this digest enters derived bookkeeping, never private paths.
        let identities = paths
            .iter()
            .map(|path| {
                let access = CatalogAccess::open_or_empty(path);
                (
                    path.to_string_lossy().into_owned(),
                    access.release().ok().flatten(),
                )
            })
            .collect::<Vec<_>>();
        super::sha256(&serde_json::to_vec(&identities).unwrap_or_default())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ProcessingReceipt {
    pub processed_count: usize,
    pub deferred_count: usize,
    pub skipped_count: usize,
}

pub struct EncounterProcessor {
    service: EncounterHistoryService,
    writer: super::NativeIngestWriter,
    context: Option<(u64, String)>,
    cursor: super::store::PendingCursor,
}

impl EncounterProcessor {
    pub fn process_pending<F>(
        &mut self,
        limit: usize,
        finalize: F,
    ) -> Result<ProcessingReceipt, HistoryDiagnostic>
    where
        F: FnOnce(&mut super::NativeCommit<'_>) -> Result<(), EncounterError>,
    {
        let generation = self.service.catalog_generation.load(Ordering::Acquire);
        if self
            .context
            .as_ref()
            .is_none_or(|(epoch, _)| *epoch != generation)
        {
            self.context = Some((generation, self.service.processing_context()));
            self.cursor = super::store::PendingCursor::default();
        }
        let context = self
            .context
            .as_ref()
            .map(|(_, context)| context.clone())
            .unwrap_or_default();
        let mut cursor = self.cursor.clone();
        let pending = self
            .writer
            .pending(limit, &context, &mut cursor)
            .map_err(store_diagnostic)?;
        if pending.is_empty() {
            self.cursor = cursor;
            return Ok(ProcessingReceipt::default());
        }
        let mut receipt = ProcessingReceipt::default();
        let mut results = Vec::new();
        for record in pending {
            let (algorithm, catalog, result) = if record.source_kind == "native-log" {
                let capture: super::NativeEncounter = serde_json::from_slice(&record.canonical)
                    .map_err(|_| invalid_store_diagnostic())?;
                let projection =
                    super::project_native_encounter(&capture).map_err(store_diagnostic)?;
                (
                    format!("native-v{}", projection.algorithm_version),
                    String::new(),
                    serde_json::to_vec(&projection).map_err(|_| invalid_store_diagnostic())?,
                )
            } else {
                let capture: super::EncounterCapture = serde_json::from_slice(&record.canonical)
                    .map_err(|_| invalid_store_diagnostic())?;
                super::validate::validate(&capture).map_err(store_diagnostic)?;
                let (catalog, value) = match self.service.project_capture(&capture) {
                    Ok(projection) => {
                        let recommendations =
                            crate::recommendation::generate_recommendations(&projection);
                        let catalog = projection.catalog_join.catalog_semantic_sha256.clone();
                        (
                            catalog,
                            serde_json::json!({"availability":"available","projection":projection,"recommendations":recommendations}),
                        )
                    }
                    Err(diagnostic) => {
                        receipt.deferred_count += 1;
                        (
                            String::new(),
                            serde_json::json!({"availability":"unavailable","reason":format!("{:?}",diagnostic.kind),"raw_content_sha256":record.hash}),
                        )
                    }
                };
                (
                    format!(
                        "{}+{}",
                        super::ALGORITHM_VERSION,
                        crate::recommendation::RECOMMENDATION_POLICY_VERSION
                    ),
                    catalog,
                    serde_json::to_vec(&value).map_err(|_| invalid_store_diagnostic())?,
                )
            };
            results.push(super::store::PreparedDerivation {
                content_sha256: record.hash,
                source_kind: record.source_kind,
                algorithm_version: algorithm,
                catalog_sha256: catalog,
                context_sha256: context.clone(),
                generation,
                result,
            });
        }
        let prepared = results.len();
        let service = &self.service;
        let stored = self
            .writer
            .commit_derivations(results, |commit| {
                let _admission = service
                    .catalog_commit
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                if service.catalog_generation.load(Ordering::Acquire) != generation {
                    return super::invalid("catalog changed during encounter processing");
                }
                finalize(commit)
            })
            .map_err(store_diagnostic)?;
        receipt.processed_count = stored;
        receipt.skipped_count = prepared - stored;
        self.cursor = cursor;
        Ok(receipt)
    }
}

fn store_diagnostic(_error: EncounterError) -> HistoryDiagnostic {
    invalid_store_diagnostic()
}

fn invalid_store_diagnostic() -> HistoryDiagnostic {
    HistoryDiagnostic::new(
        HistoryDiagnosticKind::StoreInvalid,
        "Imported encounter history on this computer could not be opened. Its database may be unavailable, damaged, or from an unsupported version; it was left unchanged. Check the application log and the application data folder's access permissions before retrying. Keep a copy before attempting recovery.",
    )
}

fn store_is_absent(path: &Path) -> Result<bool, ()> {
    match fs::metadata(path) {
        Ok(_) => Ok(false),
        Err(error) if error.kind() == ErrorKind::NotFound => {
            let mut candidate = Some(path);
            while let Some(current) = candidate {
                match fs::symlink_metadata(current) {
                    Ok(metadata) => return Ok(metadata.is_dir()),
                    Err(error) if error.kind() == ErrorKind::NotFound => {
                        candidate = current.parent();
                    }
                    Err(_) => return Err(()),
                }
            }
            Err(())
        }
        Err(_) => Err(()),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SavedRecordingReadiness {
    NeverStarted,
    Waiting,
    Recording,
    Stopped,
    StoppedEmpty,
    Interrupted,
    Failed,
    LegacyFinished,
}

impl SavedRecordingReadiness {
    pub(crate) fn from_state(state: Option<&CaptureStateSummary>, records: usize) -> Self {
        match state {
            None if records == 0 => Self::NeverStarted,
            None => Self::LegacyFinished,
            Some(state) => match state.state {
                CaptureControllerState::Stopped if state.session_id.is_none() && records == 0 => {
                    Self::NeverStarted
                }
                CaptureControllerState::Stopped if records == 0 => Self::StoppedEmpty,
                CaptureControllerState::Stopped => Self::Stopped,
                CaptureControllerState::Waiting => Self::Waiting,
                CaptureControllerState::Capturing => Self::Recording,
                CaptureControllerState::Interrupted => Self::Interrupted,
                CaptureControllerState::Failed => Self::Failed,
            },
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::NeverStarted => "Historical recorder metadata contains no retained session.",
            Self::Waiting => "Historical recorder metadata was waiting for combat.",
            Self::Recording => "Historical recorder metadata contains an unfinished recording.",
            Self::Stopped => "Historical recorder metadata contains a stopped session.",
            Self::StoppedEmpty => "Historical recorder metadata contains an empty stopped session.",
            Self::Interrupted => {
                "The historical recording was interrupted; some observations may be missing."
            }
            Self::Failed => "Historical recorder metadata contains a failure.",
            Self::LegacyFinished => "Historical saved data contains a finished recording.",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SavedSourceInspection {
    pub path: PathBuf,
    pub channel: Channel,
    pub modified: Option<std::time::SystemTime>,
    pub state: Option<CaptureStateSummary>,
    pub sessions: Vec<CaptureStateSummary>,
    pub spool_session_count: usize,
    pub spool_estimated_bytes: u64,
    pub finished_count: usize,
    pub partial_count: usize,
    pub readiness: SavedRecordingReadiness,
}

fn source_diagnostic(error: EncounterError) -> HistoryDiagnostic {
    let (kind, message) = match error {
        EncounterError::Io(error) if error.kind() == ErrorKind::NotFound => (
            HistoryDiagnosticKind::SourceUnavailable,
            "No retained saved addon data file was found. Existing local history remains unchanged.",
        ),
        EncounterError::Io(_) => (
            HistoryDiagnosticKind::SourceInaccessible,
            "The retained saved addon data could not be read. Existing local history remains unchanged.",
        ),
        EncounterError::UnsupportedVersion => (
            HistoryDiagnosticKind::SourceVersionUnsupported,
            "The saved addon data uses an unsupported version. Keep the file unchanged and use a compatible ESO Weave version.",
        ),
        EncounterError::EnvironmentMismatch => (
            HistoryDiagnosticKind::SourceEnvironmentMismatch,
            "The retained saved recording belongs to a different source environment. Existing local history remains unchanged.",
        ),
        EncounterError::Validation(_) | EncounterError::Database(_) | EncounterError::NativeWorkLimit => (
            HistoryDiagnosticKind::SourceInvalid,
            "The retained saved addon data is malformed, incomplete or changed during reading. Existing local history remains unchanged.",
        ),
    };
    HistoryDiagnostic::new(kind, message)
}

fn native_source_diagnostic(error: EncounterError) -> HistoryDiagnostic {
    match error {
        EncounterError::NativeWorkLimit => HistoryDiagnostic::new(
            HistoryDiagnosticKind::SourceInvalid,
            "This completed native source exceeds the compatibility reader's cumulative context-work limit (20 million record visits or 512 MiB). No source or history data was changed.",
        ),
        EncounterError::UnsupportedVersion=>HistoryDiagnostic::new(
            HistoryDiagnosticKind::SourceVersionUnsupported,
            "This native recording version is unsupported. Processing supports normal/separate version15; inline and verbose layouts remain unsupported. The source and history were kept."
        ),
        EncounterError::Io(ref error) if error.kind()==ErrorKind::NotFound=>HistoryDiagnostic::new(
            HistoryDiagnosticKind::SourceUnavailable,
            "The native recording source is unavailable. Existing local history remains unchanged."
        ),
        EncounterError::Io(_)=>HistoryDiagnostic::new(
            HistoryDiagnosticKind::SourceInaccessible,
            "The native recording could not be read. No source or history data was changed."
        ),
        _=>HistoryDiagnostic::new(
            HistoryDiagnosticKind::SourceInvalid,
            "The native recording is malformed, changed during reading, exceeds a declared limit, or does not match supported normal/separate version15. No source or history data was changed."
        ),
    }
}

#[cfg(test)]
mod source_tests {
    use super::*;
    #[test]
    fn permission_failure_uses_a_controlled_source_access_diagnostic() {
        let diagnostic = source_diagnostic(EncounterError::Io(std::io::Error::new(
            ErrorKind::PermissionDenied,
            "PRIVATE_PATH_CANARY",
        )));
        assert_eq!(diagnostic.kind, HistoryDiagnosticKind::SourceInaccessible);
        assert!(!diagnostic.message.contains("PRIVATE_PATH_CANARY"));
    }
}
