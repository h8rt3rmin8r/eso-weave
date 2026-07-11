//! Session state store: the live suspend and fishing intents, persisted to a
//! file separate from `config.json`.
//!
//! The constitution requires the configuration file to hold user settings only,
//! with no session, runtime, or derived state. Session state therefore lives
//! here, in `state.json`, and is restored on launch under the focus-scoped input
//! invariant (a restored running or fishing intent performs no input until the
//! game window is focused). Like the config store, loading never panics and
//! degrades to safe defaults (not suspended, not fishing) on any problem.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::config::{ConfigError, Notice, NoticeKind};

/// Current session-state schema version.
pub const CURRENT_STATE_VERSION: u32 = 1;

/// Session state file name within the config directory.
pub const STATE_FILE_NAME: &str = "state.json";

/// The persisted session state.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionState {
    /// The state schema version.
    #[serde(default = "default_version")]
    pub schema_version: u32,
    /// Whether the engine was suspended.
    #[serde(default)]
    pub suspended: bool,
    /// The fishing on/off intent (never a transient sub-state).
    #[serde(default)]
    pub fishing: bool,
}

fn default_version() -> u32 {
    CURRENT_STATE_VERSION
}

impl Default for SessionState {
    fn default() -> Self {
        Self {
            schema_version: CURRENT_STATE_VERSION,
            suspended: false,
            fishing: false,
        }
    }
}

/// Loads the session state from `<config_dir>/state.json`. A missing file yields
/// defaults with no notice; an unreadable or invalid file yields defaults with a
/// notice (the safe fallback is not suspended, not fishing).
pub fn load(config_dir: &Path) -> (SessionState, Vec<Notice>) {
    let path = config_dir.join(STATE_FILE_NAME);
    let raw = match std::fs::read_to_string(&path) {
        Ok(raw) => raw,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return (SessionState::default(), Vec::new());
        }
        Err(err) => {
            return (
                SessionState::default(),
                vec![Notice {
                    kind: NoticeKind::Unwritable,
                    message: format!("could not read session state: {err}; using defaults"),
                }],
            );
        }
    };
    match serde_json::from_str::<SessionState>(&raw) {
        Ok(state) => (state, Vec::new()),
        Err(err) => (
            SessionState::default(),
            vec![Notice {
                kind: NoticeKind::CorruptConfig,
                message: format!("session state was invalid: {err}; using defaults"),
            }],
        ),
    }
}

/// Saves the session state as pretty JSON, UTF-8 without a byte order mark, with
/// LF endings and a trailing newline.
pub fn save(config_dir: &Path, state: &SessionState) -> Result<(), ConfigError> {
    std::fs::create_dir_all(config_dir)?;
    let mut json = serde_json::to_string_pretty(state)?;
    json.push('\n');
    std::fs::write(config_dir.join(STATE_FILE_NAME), json.as_bytes())?;
    Ok(())
}
