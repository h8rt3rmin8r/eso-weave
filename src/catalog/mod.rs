//! Immutable ESO catalog compilation and read-only application access.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rusqlite::Connection;
use serde::{Deserialize, Serialize};

pub mod compiler;
pub mod model;
pub mod schema;

pub use model::{Channel, EntityKind};

#[derive(thiserror::Error, Debug)]
pub enum CatalogError {
    #[error("catalog I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("catalog database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("catalog JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("catalog validation failed: {0}")]
    Validation(String),
    #[error("catalog schema {found} is incompatible with supported schema {supported}")]
    IncompatibleSchema { found: u32, supported: u32 },
    #[error("catalog semantic checksum mismatch: expected {expected}, calculated {actual}")]
    Checksum { expected: String, actual: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CatalogDiagnosticKind {
    Missing,
    Corrupt,
    Incompatible,
    Checksum,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogDiagnostic {
    pub kind: CatalogDiagnosticKind,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogRelease {
    pub schema_version: u32,
    pub catalog_version: String,
    pub channel: Channel,
    pub game_version: String,
    pub api_version: u32,
    pub semantic_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "kebab-case")]
pub enum CatalogValue {
    Integer(i64),
    Real(f64),
    Text(String),
    Boolean(bool),
    Json(serde_json::Value),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogEntity {
    pub kind: EntityKind,
    pub stable_id: i64,
    pub observed_only: bool,
    pub attributes: BTreeMap<String, CatalogValue>,
}

impl CatalogEntity {
    pub fn integer_attribute(&self, name: &str) -> Option<i64> {
        match self.attributes.get(name) {
            Some(CatalogValue::Integer(value)) => Some(*value),
            _ => None,
        }
    }
}

pub struct CatalogReader {
    connection: Connection,
    release: CatalogRelease,
}

impl CatalogReader {
    fn open(path: &Path) -> Result<Self, CatalogError> {
        let connection = schema::open_read_only(path)?;
        let diagnostics = schema::verify_connection(&connection)?;
        let (catalog_version, channel, game_version, api_version): (String, String, String, u32) =
            connection.query_row(
                "SELECT catalog_version, channel, game_version, api_version
                 FROM catalog_release WHERE singleton = 1",
                [],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )?;
        let channel = match channel.as_str() {
            "live" => Channel::Live,
            "pts" => Channel::Pts,
            _ => {
                return Err(CatalogError::Validation(
                    "invalid catalog channel".to_string(),
                ))
            }
        };
        Ok(Self {
            connection,
            release: CatalogRelease {
                schema_version: diagnostics.schema_version,
                catalog_version,
                channel,
                game_version,
                api_version,
                semantic_sha256: diagnostics.semantic_sha256,
            },
        })
    }

    pub fn release(&self) -> CatalogRelease {
        self.release.clone()
    }

    pub fn entity(
        &self,
        kind: EntityKind,
        stable_id: i64,
    ) -> Result<Option<CatalogEntity>, CatalogError> {
        let result = self.connection.query_row(
            "SELECT observed_only FROM entity
             WHERE kind = ?1 AND stable_id = ?2 AND channel = ?3 AND api_version = ?4",
            (
                kind.as_str(),
                stable_id,
                self.release.channel.as_str(),
                self.release.api_version,
            ),
            |row| row.get::<_, bool>(0),
        );
        let observed_only = match result {
            Ok(value) => value,
            Err(rusqlite::Error::QueryReturnedNoRows) => return Ok(None),
            Err(error) => return Err(error.into()),
        };
        let mut statement = self.connection.prepare(
            "SELECT name, value_type, integer_value, real_value, text_value
             FROM entity_attribute
             WHERE kind = ?1 AND stable_id = ?2 AND channel = ?3 AND api_version = ?4
             ORDER BY name",
        )?;
        let rows = statement.query_map(
            (
                kind.as_str(),
                stable_id,
                self.release.channel.as_str(),
                self.release.api_version,
            ),
            |row| {
                let name: String = row.get(0)?;
                let value_type: String = row.get(1)?;
                let value = match value_type.as_str() {
                    "integer" => CatalogValue::Integer(row.get(2)?),
                    "real" => CatalogValue::Real(row.get(3)?),
                    "text" => CatalogValue::Text(row.get(4)?),
                    "boolean" => CatalogValue::Boolean(row.get::<_, i64>(2)? != 0),
                    "json" => {
                        let value: String = row.get(4)?;
                        CatalogValue::Json(serde_json::from_str(&value).map_err(|error| {
                            rusqlite::Error::FromSqlConversionFailure(
                                value.len(),
                                rusqlite::types::Type::Text,
                                Box::new(error),
                            )
                        })?)
                    }
                    _ => {
                        return Err(rusqlite::Error::InvalidColumnType(
                            1,
                            "value_type".to_string(),
                            rusqlite::types::Type::Text,
                        ))
                    }
                };
                Ok((name, value))
            },
        )?;
        let attributes = rows
            .map(|row| row.map_err(CatalogError::from))
            .collect::<Result<_, _>>()?;
        Ok(Some(CatalogEntity {
            kind,
            stable_id,
            observed_only,
            attributes,
        }))
    }
}

pub enum CatalogAccess {
    Available(CatalogReader),
    Empty(CatalogDiagnostic),
}

impl CatalogAccess {
    pub fn empty(kind: CatalogDiagnosticKind, message: impl Into<String>) -> Self {
        Self::Empty(CatalogDiagnostic {
            kind,
            message: message.into(),
        })
    }

    pub fn open_or_empty(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();
        if !path.is_file() {
            return Self::Empty(CatalogDiagnostic {
                kind: CatalogDiagnosticKind::Missing,
                message: format!("Catalog unavailable: {} was not found", path.display()),
            });
        }
        match CatalogReader::open(path) {
            Ok(reader) => Self::Available(reader),
            Err(error) => Self::Empty(CatalogDiagnostic {
                kind: diagnostic_kind(&error),
                message: format!("Catalog unavailable: {error}"),
            }),
        }
    }

    pub fn is_available(&self) -> bool {
        matches!(self, Self::Available(_))
    }

    pub fn diagnostic(&self) -> Option<&CatalogDiagnostic> {
        match self {
            Self::Available(_) => None,
            Self::Empty(diagnostic) => Some(diagnostic),
        }
    }

    pub fn release(&self) -> Result<Option<CatalogRelease>, CatalogError> {
        Ok(match self {
            Self::Available(reader) => Some(reader.release()),
            Self::Empty(_) => None,
        })
    }

    pub fn entity(
        &self,
        kind: EntityKind,
        stable_id: i64,
    ) -> Result<Option<CatalogEntity>, CatalogError> {
        match self {
            Self::Available(reader) => reader.entity(kind, stable_id),
            Self::Empty(_) => Ok(None),
        }
    }

    pub fn ability(&self, stable_id: i64) -> Result<Option<CatalogEntity>, CatalogError> {
        self.entity(EntityKind::Ability, stable_id)
    }

    pub fn status(&self) -> (String, bool) {
        match self {
            Self::Available(reader) => (
                format!(
                    "{} ({}, API {})",
                    reader.release.catalog_version,
                    reader.release.channel.as_str(),
                    reader.release.api_version
                ),
                true,
            ),
            Self::Empty(diagnostic) => (diagnostic.message.clone(), false),
        }
    }
}

fn diagnostic_kind(error: &CatalogError) -> CatalogDiagnosticKind {
    match error {
        CatalogError::IncompatibleSchema { .. } => CatalogDiagnosticKind::Incompatible,
        CatalogError::Checksum { .. } => CatalogDiagnosticKind::Checksum,
        CatalogError::Database(_) | CatalogError::Validation(_) => CatalogDiagnosticKind::Corrupt,
        CatalogError::Io(_) | CatalogError::Json(_) => CatalogDiagnosticKind::Unavailable,
    }
}

pub fn catalog_candidates(executable: &Path, debug_root: Option<&Path>) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    if let Some(parent) = executable.parent() {
        candidates.push(parent.join("catalog").join("catalog.sqlite"));
    }
    #[cfg(target_os = "linux")]
    {
        if let Some(app_root) = executable.parent().and_then(Path::parent) {
            candidates.push(app_root.join("share/eso-weave/catalog/catalog.sqlite"));
        }
        candidates.push(PathBuf::from("/usr/share/eso-weave/catalog/catalog.sqlite"));
    }
    if let Some(root) = debug_root {
        candidates.push(root.join("assets/catalog/catalog.sqlite"));
    }
    candidates
}

pub fn locate_catalog(executable: &Path, debug_root: Option<&Path>) -> PathBuf {
    let candidates = catalog_candidates(executable, debug_root);
    candidates
        .iter()
        .find(|path| path.is_file())
        .cloned()
        .or_else(|| candidates.into_iter().next())
        .unwrap_or_else(|| PathBuf::from("catalog/catalog.sqlite"))
}
