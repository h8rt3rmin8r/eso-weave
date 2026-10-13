use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

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
            "The saved addon data location is unavailable. Choose Live or PTS and the AddOns folder in Settings, then record with ESO Weave Data and save using /reloadui, logout, or exit.",
        )
    }
}

#[derive(Debug, Clone)]
pub struct EncounterHistoryService {
    store_path: PathBuf,
    catalog_path: Arc<RwLock<PathBuf>>,
}

impl EncounterHistoryService {
    pub fn new(application_root: impl AsRef<Path>, catalog_path: impl Into<PathBuf>) -> Self {
        Self::from_paths(
            application_root
                .as_ref()
                .join("encounters")
                .join("encounters.sqlite"),
            catalog_path,
        )
    }

    pub fn from_paths(store_path: impl Into<PathBuf>, catalog_path: impl Into<PathBuf>) -> Self {
        Self {
            store_path: store_path.into(),
            catalog_path: Arc::new(RwLock::new(catalog_path.into())),
        }
    }

    pub fn store_path(&self) -> &Path {
        &self.store_path
    }

    pub fn set_catalog_path(&self, catalog_path: impl Into<PathBuf>) {
        *self
            .catalog_path
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = catalog_path.into();
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
            return Err(HistoryDiagnostic::new(HistoryDiagnosticKind::SourceInvalid, "The native file changed since inspection. Inspect the selected file again before importing."));
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
                "The selected encounter is no longer present. Refresh history.",
            )
        })?;

        let catalog_path = self
            .catalog_path
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone();
        let catalog = CatalogAccess::open_or_empty(catalog_path);
        if let Some(diagnostic) = catalog.diagnostic() {
            let (kind, message) = match diagnostic.kind {
                CatalogDiagnosticKind::Missing | CatalogDiagnosticKind::Unavailable => (
                    HistoryDiagnosticKind::CatalogUnavailable,
                    "Game-data definitions are unavailable, so encounter metrics cannot be calculated. Your imported recordings remain stored. Open Catalog Update to select a compatible catalog when one is available.",
                ),
                CatalogDiagnosticKind::Corrupt
                | CatalogDiagnosticKind::Incompatible
                | CatalogDiagnosticKind::Checksum => (
                    HistoryDiagnosticKind::CatalogInvalid,
                    "The catalog of game-data definitions is damaged or unsupported, so metrics cannot be calculated. Your imported recordings remain stored. Open Catalog Update to select a compatible catalog or roll back to a previous verified one.",
                ),
            };
            return Err(HistoryDiagnostic::new(kind, message));
        }
        let release = catalog
            .release()
            .map_err(|_| {
                HistoryDiagnostic::new(
                    HistoryDiagnosticKind::CatalogInvalid,
                    "The catalog's game-data identity could not be checked, so metrics cannot be calculated. Your imported recordings remain stored. Open Catalog Update to select a compatible catalog or roll back to a previous verified one.",
                )
            })?
            .ok_or_else(|| {
                HistoryDiagnostic::new(
                    HistoryDiagnosticKind::CatalogUnavailable,
                    "Game-data definitions are unavailable, so encounter metrics cannot be calculated. Your imported recordings remain stored. Open Catalog Update to select a compatible catalog when one is available.",
                )
            })?;
        if release.channel != capture.channel || release.api_version != capture.source.api_version {
            return Err(HistoryDiagnostic::new(
                HistoryDiagnosticKind::VersionMismatch,
                "The catalog's Live or PTS environment or game API version differs from this recording, so metrics cannot be calculated. Your imported recording remains stored. It needs matching game-data definitions; Catalog Update currently installs Live catalogs only, so PTS metrics may remain unavailable.",
            ));
        }

        calculate_projection(&capture, &catalog).map_err(|_| {
            HistoryDiagnostic::new(
                HistoryDiagnosticKind::OperationFailed,
                "Metrics could not be calculated from this recording. The imported original remains stored. Try Refresh and select it again; if the failure persists, use the application log for troubleshooting.",
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
            Self::NeverStarted => "Recording was OFF with no retained session. Start one fight in ESO.",
            Self::Waiting => "The saved recorder was WAITING FOR COMBAT. Fight, then save new data.",
            Self::Recording => "The saved recorder was RECORDING. Finish or stop, then save new data.",
            Self::Stopped => "The saved recorder was OFF. Import retained encounters or start a new session; earlier recordings stay retained.",
            Self::StoppedEmpty => "The saved recorder was OFF with an empty session. Start a new fight; no clear is required.",
            Self::Interrupted => "The saved recording was interrupted. Import retained encounters; some observations may be missing.",
            Self::Failed => "The saved recorder stopped after a failure. Import usable retained encounters before recovery.",
            Self::LegacyFinished => "A finished recording is saved and ready to import.",
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
            "No saved addon data file was found. Check Live or PTS and the AddOns folder in Settings. Enable ESO Weave Data, record, then /reloadui, logout or exit to save.",
        ),
        EncounterError::Io(_) => (
            HistoryDiagnosticKind::SourceInaccessible,
            "The saved addon data file could not be read. Check the selected folder and read permissions, then refresh the saved source. Existing history was preserved.",
        ),
        EncounterError::UnsupportedVersion => (
            HistoryDiagnosticKind::SourceVersionUnsupported,
            "The saved addon data uses an unsupported version. Keep the file unchanged and use a compatible ESO Weave version.",
        ),
        EncounterError::EnvironmentMismatch => (
            HistoryDiagnosticKind::SourceEnvironmentMismatch,
            "The saved recording belongs to a different environment. Select its Live or PTS environment in Settings, then refresh the saved source.",
        ),
        EncounterError::Validation(_) | EncounterError::Database(_) | EncounterError::NativeWorkLimit => (
            HistoryDiagnosticKind::SourceInvalid,
            "The saved addon data is malformed, incomplete or changed during reading. Keep it unchanged for troubleshooting; finish or stop recording in ESO, save again, then refresh the saved source.",
        ),
    };
    HistoryDiagnostic::new(kind, message)
}

fn native_source_diagnostic(error: EncounterError) -> HistoryDiagnostic {
    match error {
        EncounterError::NativeWorkLimit => HistoryDiagnostic::new(
            HistoryDiagnosticKind::SourceInvalid,
            "This native file exceeds the cumulative context-work limit (20 million record visits or 512 MiB of inspected context). Choose a smaller completed recording. No source or history data was changed.",
        ),
        EncounterError::UnsupportedVersion=>HistoryDiagnostic::new(
            HistoryDiagnosticKind::SourceVersionUnsupported,
            "This native recording version is unsupported. Native import supports normal version15 completed files; inline and verbose layouts remain unsupported. The source and history were kept."
        ),
        EncounterError::Io(ref error) if error.kind()==ErrorKind::NotFound=>HistoryDiagnostic::new(
            HistoryDiagnosticKind::SourceUnavailable,
            "The selected native recording file is missing. Choose its completed local file path and inspect again."
        ),
        EncounterError::Io(_)=>HistoryDiagnostic::new(
            HistoryDiagnosticKind::SourceInaccessible,
            "The native recording could not be read. Check file access or use a completed copy, then inspect again. No source or history data was changed."
        ),
        _=>HistoryDiagnostic::new(
            HistoryDiagnosticKind::SourceInvalid,
            "The native recording is malformed, changed during reading, exceeds a declared limit, or does not match supported normal version15 layouts. Choose a stable completed file and inspect again. No source or history data was changed."
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
