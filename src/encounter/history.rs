use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use crate::catalog::{CatalogAccess, CatalogDiagnosticKind, Channel};

use super::{
    calculate_projection, delete_all, delete_encounter, import_capture_set, list_encounters,
    load_encounter, CaptureImportReport, DeleteReceipt, EncounterError, EncounterProjection,
    EncounterSummary, ImportRequest,
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
        if !source_path.is_file() {
            return Err(HistoryDiagnostic::new(
                HistoryDiagnosticKind::SourceUnavailable,
                "No saved addon data file was found for the selected environment. Check Live or PTS and the AddOns folder in Settings. Record with ESO Weave Data, then use /reloadui, logout, or exit to save before importing.",
            ));
        }
        import_capture_set(&ImportRequest::new(
            source_path,
            &self.store_path,
            expected_channel,
        ))
        .map_err(|_| {
            HistoryDiagnostic::new(
                HistoryDiagnosticKind::OperationFailed,
                "The saved recording could not be imported. Existing history was preserved. Check the selected Live or PTS environment, finish or stop recording in ESO, save with /reloadui, logout, or exit, and try Import Saved Capture again. Unsupported or damaged saved data must remain unchanged for troubleshooting.",
            )
        })
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
