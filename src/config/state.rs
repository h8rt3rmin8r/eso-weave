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
pub const CURRENT_STATE_VERSION: u32 = 3;

/// The recorded window geometry: automatically captured runtime state (where and
/// how large the window last was), stored here in `state.json` rather than the
/// settings file. Positions and sizes are egui points, rounded to integers;
/// the position may be negative on a secondary monitor. Additive and backward
/// compatible: an old `state.json` without this section loads as `None`.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WindowGeometry {
    /// Outer window left edge, in points (desktop virtual coordinates).
    pub x: i32,
    /// Outer window top edge, in points.
    pub y: i32,
    /// Inner (client) width, in points.
    pub width: u32,
    /// Inner (client) height, in points.
    pub height: u32,
    /// Whether the window was maximized.
    #[serde(default)]
    pub maximized: bool,
}

/// The minimum on-screen strip (points) that a restored window must expose to be
/// considered reachable: at least this wide and, vertically, at least a title-bar
/// height, so the user can always grab it.
const MIN_VISIBLE_W: i32 = 80;
const MIN_VISIBLE_H: i32 = 32;

/// Inputs to [`sanitize_geometry`], supplied by the caller so the helper itself
/// needs no platform access and stays fully unit-testable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RestoreBounds {
    /// Minimum inner width (points).
    pub min_w: u32,
    /// Minimum inner height (points).
    pub min_h: u32,
    /// Maximum plausible inner width (points).
    pub max_w: u32,
    /// Maximum plausible inner height (points).
    pub max_h: u32,
    /// Point-space desktop rectangle `(x, y, w, h)`, or `None` to skip the
    /// off-screen position check (the window manager is trusted to place it).
    pub virtual_screen: Option<(i32, i32, i32, i32)>,
}

/// The sanitized geometry to apply at launch. Not persisted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeometryRestore {
    /// Inner width to apply (points), clamped to the valid range.
    pub width: u32,
    /// Inner height to apply (points), clamped to the valid range.
    pub height: u32,
    /// Position to apply (points), or `None` to open at the default placement.
    pub position: Option<(i32, i32)>,
    /// Whether to open maximized.
    pub maximized: bool,
}

/// Sanitizes a recorded [`WindowGeometry`] for restoring the window: clamps the
/// size into the valid range, drops a position that is no longer visible on the
/// desktop (so the window never opens off-screen), and passes the maximized flag
/// through. Pure and deterministic: no I/O, no platform calls, no clock.
pub fn sanitize_geometry(geo: WindowGeometry, bounds: RestoreBounds) -> GeometryRestore {
    // clamp requires lo <= hi; guard against inverted or degenerate bounds.
    let hi_w = bounds.max_w.max(bounds.min_w);
    let hi_h = bounds.max_h.max(bounds.min_h);
    let width = geo.width.clamp(bounds.min_w, hi_w);
    let height = geo.height.clamp(bounds.min_h, hi_h);

    let position = match bounds.virtual_screen {
        None => Some((geo.x, geo.y)),
        Some((vx, vy, vw, vh)) => {
            let ww = width as i32;
            let wh = height as i32;
            let ox1 = geo.x.max(vx);
            let oy1 = geo.y.max(vy);
            let ox2 = (geo.x + ww).min(vx + vw);
            let oy2 = (geo.y + wh).min(vy + vh);
            let overlap_w = (ox2 - ox1).max(0);
            let overlap_h = (oy2 - oy1).max(0);
            if overlap_w >= MIN_VISIBLE_W && overlap_h >= MIN_VISIBLE_H {
                Some((geo.x, geo.y))
            } else {
                None
            }
        }
    };

    GeometryRestore {
        width,
        height,
        position,
        maximized: geo.maximized,
    }
}

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
    /// The last recorded window geometry, restored on launch. `None` means no
    /// geometry has been recorded (open at the default). Additive and backward
    /// compatible: an old state file without this key loads as `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<WindowGeometry>,
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
            window: None,
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

    #[test]
    fn current_state_version_is_three() {
        assert_eq!(CURRENT_STATE_VERSION, 3);
    }

    #[test]
    fn state_without_window_loads_as_none() {
        let prior = r#"{"schema_version":2,"suspended":false,"fishing":true}"#;
        let state: SessionState = serde_json::from_str(prior).unwrap();
        assert!(state.fishing);
        assert_eq!(state.window, None);
    }

    #[test]
    fn window_geometry_round_trips() {
        let state = SessionState {
            window: Some(WindowGeometry {
                x: -1920,
                y: 40,
                width: 800,
                height: 900,
                maximized: false,
            }),
            ..SessionState::default()
        };
        let json = serde_json::to_string(&state).unwrap();
        let back: SessionState = serde_json::from_str(&json).unwrap();
        assert_eq!(back, state);
    }

    #[test]
    fn window_geometry_is_omitted_when_none() {
        let state = SessionState::default();
        let json = serde_json::to_string(&state).unwrap();
        assert!(!json.contains("window"));
    }

    /// Bounds for a single 1920x1080 desktop at the origin, min 480x420.
    fn single_screen_bounds() -> RestoreBounds {
        RestoreBounds {
            min_w: 480,
            min_h: 420,
            max_w: 1920,
            max_h: 1080,
            virtual_screen: Some((0, 0, 1920, 1080)),
        }
    }

    #[test]
    fn sanitize_in_range_keeps_size_and_position() {
        let geo = WindowGeometry {
            x: 100,
            y: 80,
            width: 600,
            height: 720,
            maximized: false,
        };
        let out = sanitize_geometry(geo, single_screen_bounds());
        assert_eq!(out.width, 600);
        assert_eq!(out.height, 720);
        assert_eq!(out.position, Some((100, 80)));
        assert!(!out.maximized);
    }

    #[test]
    fn sanitize_zero_size_clamps_up_to_minimum() {
        let geo = WindowGeometry {
            x: 10,
            y: 10,
            width: 0,
            height: 0,
            maximized: false,
        };
        let out = sanitize_geometry(geo, single_screen_bounds());
        assert_eq!(out.width, 480);
        assert_eq!(out.height, 420);
    }

    #[test]
    fn sanitize_sub_minimum_size_clamps_up_to_minimum() {
        let geo = WindowGeometry {
            x: 10,
            y: 10,
            width: 100,
            height: 100,
            maximized: false,
        };
        let out = sanitize_geometry(geo, single_screen_bounds());
        assert_eq!(out.width, 480);
        assert_eq!(out.height, 420);
    }

    #[test]
    fn sanitize_oversized_size_clamps_down_to_maximum() {
        let geo = WindowGeometry {
            x: 0,
            y: 0,
            width: 9000,
            height: 9000,
            maximized: false,
        };
        let out = sanitize_geometry(geo, single_screen_bounds());
        assert_eq!(out.width, 1920);
        assert_eq!(out.height, 1080);
    }

    #[test]
    fn sanitize_offscreen_position_is_dropped_size_preserved() {
        // A window on a now-disconnected monitor at x=2600, beyond the 1920 desktop.
        let geo = WindowGeometry {
            x: 2600,
            y: 200,
            width: 640,
            height: 480,
            maximized: false,
        };
        let out = sanitize_geometry(geo, single_screen_bounds());
        assert_eq!(out.position, None);
        assert_eq!(out.width, 640);
        assert_eq!(out.height, 480);
    }

    #[test]
    fn sanitize_partial_overlap_beyond_margin_keeps_position() {
        // Mostly off the right edge, but more than MIN_VISIBLE_W remains on screen.
        let geo = WindowGeometry {
            x: 1800,
            y: 100,
            width: 640,
            height: 480,
            maximized: false,
        };
        let out = sanitize_geometry(geo, single_screen_bounds());
        assert_eq!(out.position, Some((1800, 100)));
    }

    #[test]
    fn sanitize_without_bounds_trusts_position() {
        let geo = WindowGeometry {
            x: 5000,
            y: 5000,
            width: 600,
            height: 720,
            maximized: false,
        };
        let bounds = RestoreBounds {
            virtual_screen: None,
            ..single_screen_bounds()
        };
        let out = sanitize_geometry(geo, bounds);
        assert_eq!(out.position, Some((5000, 5000)));
    }

    #[test]
    fn sanitize_preserves_maximized_regardless_of_position() {
        let geo = WindowGeometry {
            x: 9000,
            y: 9000,
            width: 600,
            height: 720,
            maximized: true,
        };
        let out = sanitize_geometry(geo, single_screen_bounds());
        assert!(out.maximized);
        assert_eq!(out.position, None);
    }
}
