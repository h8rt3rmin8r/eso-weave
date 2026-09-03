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

/// Returns the active X11 or XWayland window title. Pure Wayland sessions and
/// X11 query failures return `None`, which callers treat as unknown focus.
pub fn active_window_title() -> Option<String> {
    use x11rb::connection::Connection;
    use x11rb::protocol::xproto::{AtomEnum, ConnectionExt};

    let (conn, screen_num) = x11rb::connect(None).ok()?;
    let root = conn.setup().roots[screen_num].root;
    let active_atom = conn
        .intern_atom(false, b"_NET_ACTIVE_WINDOW")
        .ok()?
        .reply()
        .ok()?
        .atom;
    let active = conn
        .get_property(false, root, active_atom, AtomEnum::WINDOW, 0, 1)
        .ok()?
        .reply()
        .ok()?;
    let window = active.value32()?.next()?;
    let name_atom = conn
        .intern_atom(false, b"_NET_WM_NAME")
        .ok()?
        .reply()
        .ok()?
        .atom;
    let utf8_atom = conn
        .intern_atom(false, b"UTF8_STRING")
        .ok()?
        .reply()
        .ok()?
        .atom;
    let name = conn
        .get_property(false, window, name_atom, utf8_atom, 0, 1024)
        .ok()?
        .reply()
        .ok()?;
    if !name.value.is_empty() {
        return Some(String::from_utf8_lossy(&name.value).into_owned());
    }
    let wm_name = conn
        .get_property(false, window, AtomEnum::WM_NAME, AtomEnum::STRING, 0, 1024)
        .ok()?
        .reply()
        .ok()?;
    Some(String::from_utf8_lossy(&wm_name.value).into_owned())
}
