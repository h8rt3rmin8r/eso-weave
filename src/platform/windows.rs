//! Windows path specifics: logs live under the roaming app data directory,
//! since Windows has no XDG state directory.

use std::path::PathBuf;

use windows_sys::Win32::UI::HiDpi::GetDpiForSystem;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    GetSystemMetrics, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN,
};

pub fn log_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join(super::APP_DIR).join("logs"))
}

/// Reads the desktop virtual-screen rectangle (physical pixels) and converts it to
/// egui points using the system DPI, so it can be compared against point-space
/// window geometry. Returns `None` if the reported extent is non-positive.
pub fn virtual_screen_bounds_points() -> Option<(i32, i32, i32, i32)> {
    // SAFETY: these are simple metric reads with no pointers or handles.
    let (px, py, pw, ph) = unsafe {
        (
            GetSystemMetrics(SM_XVIRTUALSCREEN),
            GetSystemMetrics(SM_YVIRTUALSCREEN),
            GetSystemMetrics(SM_CXVIRTUALSCREEN),
            GetSystemMetrics(SM_CYVIRTUALSCREEN),
        )
    };
    if pw <= 0 || ph <= 0 {
        return None;
    }
    // SAFETY: GetDpiForSystem takes no arguments and always returns a value.
    let dpi = unsafe { GetDpiForSystem() };
    let scale = if dpi == 0 { 1.0 } else { dpi as f32 / 96.0 };
    let to_points = |v: i32| (v as f32 / scale).round() as i32;
    Some((to_points(px), to_points(py), to_points(pw), to_points(ph)))
}

pub fn window_work_area_points(
    outer: (f32, f32, f32, f32),
    pixels_per_point: f32,
) -> Option<(f32, f32, f32, f32)> {
    use windows_sys::Win32::Foundation::RECT;
    use windows_sys::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MonitorFromRect, MONITORINFO, MONITOR_DEFAULTTONEAREST,
    };
    if !pixels_per_point.is_finite()
        || pixels_per_point <= 0.0
        || ![outer.0, outer.1, outer.2, outer.3]
            .iter()
            .all(|v| v.is_finite())
    {
        return None;
    }
    let physical = RECT {
        left: (outer.0 * pixels_per_point).floor() as i32,
        top: (outer.1 * pixels_per_point).floor() as i32,
        right: (outer.2 * pixels_per_point).ceil() as i32,
        bottom: (outer.3 * pixels_per_point).ceil() as i32,
    };
    let mut info = MONITORINFO {
        cbSize: std::mem::size_of::<MONITORINFO>() as u32,
        ..Default::default()
    };
    // SAFETY: both structures are initialized, correctly sized, and valid for
    // these read-only calls. No window or monitor ownership is transferred.
    unsafe {
        let monitor = MonitorFromRect(&physical, MONITOR_DEFAULTTONEAREST);
        if monitor.is_null() || GetMonitorInfoW(monitor, &mut info) == 0 {
            return None;
        }
    }
    let work = info.rcWork;
    (work.right > work.left && work.bottom > work.top).then_some((
        work.left as f32 / pixels_per_point,
        work.top as f32 / pixels_per_point,
        work.right as f32 / pixels_per_point,
        work.bottom as f32 / pixels_per_point,
    ))
}
