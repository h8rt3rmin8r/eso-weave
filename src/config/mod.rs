//! Config Store: user-settings-only JSON with corruption resilience and forward
//! migration.
//!
//! [`load`] never panics; on any problem it degrades to defaults and returns a
//! [`Notice`] describing what happened. [`save`] writes UTF-8 without a byte
//! order mark, with LF endings and a trailing newline.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Current settings schema version.
pub const CURRENT_SCHEMA_VERSION: u32 = 1;

/// Settings file name within the config directory.
pub const CONFIG_FILE_NAME: &str = "config.json";

/// A logging verbosity level as stored in settings.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum LevelName {
    /// No events are captured.
    Off,
    /// Error events only.
    Error,
    /// Warnings and errors.
    Warn,
    /// Informational events and above (the default).
    #[default]
    Info,
    /// Debug events and above.
    Debug,
    /// All events, including trace.
    Trace,
}

impl LevelName {
    /// Parses a level name. Returns the parsed level and whether the input was a
    /// recognized value. An absent value maps to the default without a warning;
    /// an unrecognized value maps to the default and reports `false`.
    fn from_opt_str(value: Option<&str>) -> (LevelName, bool) {
        match value {
            None => (LevelName::Info, true),
            Some(raw) => match raw.to_ascii_lowercase().as_str() {
                "off" => (LevelName::Off, true),
                "error" => (LevelName::Error, true),
                "warn" => (LevelName::Warn, true),
                "info" => (LevelName::Info, true),
                "debug" => (LevelName::Debug, true),
                "trace" => (LevelName::Trace, true),
                _ => (LevelName::Info, false),
            },
        }
    }
}

/// Operator-chosen logging preferences that belong to user settings.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct LoggingPrefs {
    /// The active logging level.
    pub level: LevelName,
    /// Whether the monthly file sink is enabled.
    pub file_enabled: bool,
}

impl Default for LoggingPrefs {
    fn default() -> Self {
        Self {
            level: LevelName::Info,
            file_enabled: false,
        }
    }
}

/// The persisted user configuration.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    /// The settings schema version.
    pub schema_version: u32,
    /// Logging preferences.
    pub logging: LoggingPrefs,
    /// Input bindings as an action-name to key-name map. Absent or empty means
    /// the default bindings are used. This section is additive and backward
    /// compatible, so no schema version bump is required.
    #[serde(default)]
    pub bindings: BTreeMap<String, String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            schema_version: CURRENT_SCHEMA_VERSION,
            logging: LoggingPrefs::default(),
            bindings: BTreeMap::new(),
        }
    }
}

/// The category of a [`Notice`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoticeKind {
    /// The settings file was unreadable as valid settings and was preserved.
    CorruptConfig,
    /// Unknown top-level keys were present and ignored.
    UnknownKeys,
    /// A field held an invalid value and fell back to its default.
    InvalidValue,
    /// Settings were migrated across schema versions.
    Migrated,
    /// The settings location could not be read.
    Unwritable,
}

/// A non-fatal condition surfaced from a Config Store operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notice {
    /// The category of condition.
    pub kind: NoticeKind,
    /// A human-readable summary suitable for a warn-level log event.
    pub message: String,
}

/// The result of [`load`]: the resulting settings plus any notices.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LoadOutcome {
    /// The loaded (or default) settings.
    pub settings: Settings,
    /// Any non-fatal notices raised during loading.
    pub notices: Vec<Notice>,
}

/// An error returned from [`save`].
#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    /// A filesystem error occurred.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    /// The settings could not be serialized.
    #[error("serialization error: {0}")]
    Serialize(#[from] serde_json::Error),
}

#[derive(Deserialize, Default)]
struct RawSettings {
    #[serde(default)]
    schema_version: u32,
    #[serde(default)]
    logging: RawLogging,
    #[serde(default)]
    bindings: BTreeMap<String, String>,
}

#[derive(Deserialize, Default)]
struct RawLogging {
    #[serde(default)]
    level: Option<String>,
    #[serde(default)]
    file_enabled: Option<bool>,
}

/// Loads settings from `<config_dir>/config.json`.
///
/// A missing file yields defaults with no notice. A corrupt file is preserved
/// under a `.invalid` name and yields defaults with a [`NoticeKind::CorruptConfig`]
/// notice. Older schema versions are migrated forward; unknown keys and invalid
/// values are reported as notices rather than errors. Never panics.
pub fn load(config_dir: &Path) -> LoadOutcome {
    let path = config_dir.join(CONFIG_FILE_NAME);

    let bytes = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return LoadOutcome::default(),
        Err(err) => {
            return LoadOutcome {
                settings: Settings::default(),
                notices: vec![Notice {
                    kind: NoticeKind::Unwritable,
                    message: format!("could not read settings file: {err}"),
                }],
            };
        }
    };

    let value: serde_json::Value = match serde_json::from_slice(&bytes) {
        Ok(value) => value,
        Err(_) => return corrupt(&path, "settings file is not valid JSON"),
    };

    let Some(object) = value.as_object() else {
        return corrupt(&path, "settings file is not a JSON object");
    };

    let mut notices = Vec::new();

    let known: BTreeSet<&str> = ["schema_version", "logging", "bindings"]
        .into_iter()
        .collect();
    let unknown: Vec<String> = object
        .keys()
        .filter(|key| !known.contains(key.as_str()))
        .cloned()
        .collect();
    if !unknown.is_empty() {
        notices.push(Notice {
            kind: NoticeKind::UnknownKeys,
            message: format!("ignoring unknown settings keys: {}", unknown.join(", ")),
        });
    }

    let raw: RawSettings = match serde_json::from_value(value) {
        Ok(raw) => raw,
        Err(_) => return corrupt(&path, "settings file does not match the expected shape"),
    };

    let (level, level_ok) = LevelName::from_opt_str(raw.logging.level.as_deref());
    if !level_ok {
        notices.push(Notice {
            kind: NoticeKind::InvalidValue,
            message: "invalid logging.level; using default (info)".to_string(),
        });
    }

    let mut settings = Settings {
        schema_version: raw.schema_version,
        logging: LoggingPrefs {
            level,
            file_enabled: raw.logging.file_enabled.unwrap_or(false),
        },
        bindings: raw.bindings,
    };

    if settings.schema_version < CURRENT_SCHEMA_VERSION {
        notices.push(Notice {
            kind: NoticeKind::Migrated,
            message: format!(
                "migrated settings from schema {} to {CURRENT_SCHEMA_VERSION}",
                settings.schema_version
            ),
        });
        settings.schema_version = CURRENT_SCHEMA_VERSION;
    } else if settings.schema_version > CURRENT_SCHEMA_VERSION {
        notices.push(Notice {
            kind: NoticeKind::Migrated,
            message: format!(
                "settings schema {} is newer than supported {CURRENT_SCHEMA_VERSION}; loading best effort",
                settings.schema_version
            ),
        });
    }

    LoadOutcome { settings, notices }
}

/// Saves settings to `<config_dir>/config.json` as pretty JSON, UTF-8 without a
/// byte order mark, with LF endings and a trailing newline.
pub fn save(config_dir: &Path, settings: &Settings) -> Result<(), ConfigError> {
    std::fs::create_dir_all(config_dir)?;
    let mut json = serde_json::to_string_pretty(settings)?;
    json.push('\n');
    std::fs::write(config_dir.join(CONFIG_FILE_NAME), json.as_bytes())?;
    Ok(())
}

fn corrupt(path: &Path, reason: &str) -> LoadOutcome {
    let message = match preserve_invalid(path) {
        Some(preserved) => format!(
            "{reason}; preserved corrupt file as {}",
            preserved.display()
        ),
        None => format!("{reason}; could not preserve corrupt file"),
    };
    LoadOutcome {
        settings: Settings::default(),
        notices: vec![Notice {
            kind: NoticeKind::CorruptConfig,
            message,
        }],
    }
}

/// Renames a corrupt file by appending `.invalid`, or `.invalid.N` when that
/// name is already taken, so a previously preserved file is never overwritten.
fn preserve_invalid(path: &Path) -> Option<PathBuf> {
    let mut base = path.as_os_str().to_owned();
    base.push(".invalid");
    let first = PathBuf::from(base);

    let target = if !first.exists() {
        first
    } else {
        let mut n = 2u32;
        loop {
            let mut candidate = path.as_os_str().to_owned();
            candidate.push(format!(".invalid.{n}"));
            let candidate = PathBuf::from(candidate);
            if !candidate.exists() {
                break candidate;
            }
            n += 1;
        }
    };

    std::fs::rename(path, &target).ok().map(|()| target)
}
