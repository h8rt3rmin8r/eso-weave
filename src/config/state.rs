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

use crate::beacon::api_check::GameVersion;
use crate::config::{ConfigError, Notice, NoticeKind};

/// Current session-state schema version.
pub const CURRENT_STATE_VERSION: u32 = 2;

/// The derived API-version cache: the last known numeric API version and the last
/// seen game version, both remembered between runs. Runtime derived state, so it
/// lives here in `state.json`, never in the settings file. Additive and
/// backward compatible: an old `state.json` without this section loads as default.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ApiVersionCache {
    /// The highest numeric API version resolved so far; `None` before first run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_known_api_version: Option<u32>,
    /// The newest game version observed from the network signal; `None` before
    /// the first successful fetch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen_game_version: Option<GameVersion>,
}

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
    /// The derived API-version cache maintained by the startup version check.
    #[serde(default)]
    pub api_version: ApiVersionCache,
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
            api_version: ApiVersionCache::default(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v1_state_without_api_version_loads_as_default() {
        let v1 = r#"{"schema_version":1,"suspended":true,"fishing":false}"#;
        let state: SessionState = serde_json::from_str(v1).unwrap();
        assert!(state.suspended);
        assert_eq!(state.api_version, ApiVersionCache::default());
        assert_eq!(state.api_version.last_known_api_version, None);
    }

    #[test]
    fn api_version_round_trips() {
        let mut state = SessionState::default();
        state.api_version.last_known_api_version = Some(101050);
        state.api_version.last_seen_game_version = Some(GameVersion::new([12, 0, 6, 0]));
        let json = serde_json::to_string(&state).unwrap();
        let back: SessionState = serde_json::from_str(&json).unwrap();
        assert_eq!(back, state);
    }
}
