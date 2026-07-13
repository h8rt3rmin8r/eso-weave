//! Linux path specifics: logs live under the XDG state directory, falling back
//! to the config directory when no state directory is defined.

use std::path::PathBuf;

pub fn log_dir() -> Option<PathBuf> {
    dirs::state_dir()
        .or_else(dirs::config_dir)
        .map(|d| d.join(super::APP_DIR).join("logs"))
}

/// Position placement is left to the window manager on Linux (Wayland cannot
/// report or set absolute window position), so no virtual-screen bounds are
/// supplied and the restored position is trusted.
pub fn virtual_screen_bounds_points() -> Option<(i32, i32, i32, i32)> {
    None
}
