//! Platform path resolution.
//!
//! `config_dir` is common across platforms; `log_dir` differs (Windows keeps
//! logs under the roaming app data directory, Linux under the XDG state
//! directory), so it lives behind a per-platform backend module. This is the
//! seam that later input and sampling backends follow.

use std::path::PathBuf;

#[cfg(unix)]
mod linux;
#[cfg(windows)]
mod windows;

#[cfg(unix)]
use linux as backend;
#[cfg(windows)]
use windows as backend;

/// Application directory name under the platform config and state roots.
pub const APP_DIR: &str = "eso-weave";

/// The per-user configuration directory for eso-weave, if one can be resolved.
///
/// Windows: `%APPDATA%/eso-weave`. Linux: `$XDG_CONFIG_HOME/eso-weave`
/// (falling back to `~/.config/eso-weave`).
pub fn config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join(APP_DIR))
}

/// The per-user log directory for eso-weave, if one can be resolved.
pub fn log_dir() -> Option<PathBuf> {
    backend::log_dir()
}

/// The desktop virtual-screen rectangle `(x, y, w, h)` in egui points, used to
/// decide whether a restored window position is still on-screen. Returns `None`
/// when the bounds are not available (the window manager is then trusted to place
/// the window). This is the platform seam; the point-space value it returns is
/// fed into the pure `sanitize_geometry` helper.
pub fn virtual_screen_bounds_points() -> Option<(i32, i32, i32, i32)> {
    backend::virtual_screen_bounds_points()
}
