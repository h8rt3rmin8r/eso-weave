use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use crate::catalog::{CatalogAccess, CatalogDiagnosticKind, Channel};

use super::{
    calculate_projection, delete_all, delete_encounter, import_encounter, list_encounters,
    load_encounter, DeleteReceipt, EncounterError, EncounterProjection, EncounterSummary,
    ImportReceipt, ImportRequest,
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
            "The selected ESO environment could not provide a terminal encounter capture.",
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
    ) -> Result<ImportReceipt, HistoryDiagnostic> {
        let source_path = source_path.as_ref();
        if !source_path.is_file() {
            return Err(HistoryDiagnostic::new(
                HistoryDiagnosticKind::SourceUnavailable,
                "The selected ESO environment has no terminal encounter capture to import.",
            ));
        }
        import_encounter(&ImportRequest::new(
            source_path,
            &self.store_path,
            expected_channel,
        ))
        .map_err(|_| {
            HistoryDiagnostic::new(
                HistoryDiagnosticKind::OperationFailed,
                "The encounter capture was rejected or could not be imported. Existing history was preserved.",
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
                    "The active catalog is unavailable. Raw encounter history remains available.",
                ),
                CatalogDiagnosticKind::Corrupt
                | CatalogDiagnosticKind::Incompatible
                | CatalogDiagnosticKind::Checksum => (
                    HistoryDiagnosticKind::CatalogInvalid,
                    "The active catalog is invalid or unsupported. Raw encounter history remains available.",
                ),
            };
            return Err(HistoryDiagnostic::new(kind, message));
        }
        let release = catalog
            .release()
            .map_err(|_| {
                HistoryDiagnostic::new(
                    HistoryDiagnosticKind::CatalogInvalid,
                    "The active catalog metadata could not be validated. Raw encounter history remains available.",
                )
            })?
            .ok_or_else(|| {
                HistoryDiagnostic::new(
                    HistoryDiagnosticKind::CatalogUnavailable,
                    "The active catalog is unavailable. Raw encounter history remains available.",
                )
            })?;
        if release.channel != capture.channel || release.api_version != capture.source.api_version {
            return Err(HistoryDiagnostic::new(
                HistoryDiagnosticKind::VersionMismatch,
                "The active catalog channel or API version does not match this capture. Raw encounter history remains available.",
            ));
        }

        calculate_projection(&capture, &catalog).map_err(|_| {
            HistoryDiagnostic::new(
                HistoryDiagnosticKind::OperationFailed,
                "Observed metrics could not be calculated. Raw encounter history remains available.",
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
        "The local encounter store is unavailable, corrupt, or unsupported. It was left unchanged.",
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
