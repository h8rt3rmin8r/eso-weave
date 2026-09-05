//! Windows surface sampler: a screen-composited capture of the beacon grid.
//!
//! `GetPixel` on the game window device context reads that window's GDI front
//! buffer, which for a hardware-accelerated (DirectX) game does not contain the
//! rendered content, so it returns black or stale pixels and the beacon signal is
//! never read. Instead this backend captures a small region from the composited
//! desktop (a `BitBlt` from the screen device context, the same mechanism as the
//! CopyFromScreen workaround that captures accelerated content) and reads the
//! block points from it.
//!
//! The reader requests the extent derived from the layout header for each batch.
//! That keeps steady-state capture to the small occupied region while allowing
//! a resize to change the row width without restarting the sampler.

use std::cell::RefCell;
use std::mem::size_of;

use windows_sys::Win32::Foundation::{HWND, POINT, RECT};
use windows_sys::Win32::Graphics::Gdi::{
    BitBlt, ClientToScreen, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject,
    GetDC, GetDIBits, GetMonitorInfoW, MonitorFromWindow, ReleaseDC, SelectObject, BITMAPINFO,
    BITMAPINFOHEADER, BI_RGB, CAPTUREBLT, DIB_RGB_COLORS, MONITORINFO, MONITOR_DEFAULTTONULL,
    SRCCOPY,
};
use windows_sys::Win32::UI::HiDpi::{GetDpiForMonitor, MDT_EFFECTIVE_DPI};
use windows_sys::Win32::UI::WindowsAndMessaging::{FindWindowW, GetClientRect};

use crate::pixelbus::display::{MeasuredDisplay, Point, Size};
use crate::pixelbus::{strip_pixel, Rgb, SurfaceSampler};

/// The captured beacon strip: a small top-left region of the client area, as
/// composited on screen, in 32-bit BGRA (the layout `GetDIBits` fills).
struct CapturedStrip {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

/// Samples the beacon grid from the composited desktop for one window. The
/// reader supplies a validated per-batch capture extent, which is also the
/// geometry used for all point reads from that captured frame.
pub struct GdiSampler {
    hwnd: HWND,
    frame: RefCell<Option<CapturedStrip>>,
}

impl GdiSampler {
    /// Resolves the window by its exact title. The reader supplies the validated
    /// capture extent for each batch. Returns `None` if the window is not found.
    pub fn for_window(title: &str) -> Option<Self> {
        let wide: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
        let hwnd = unsafe { FindWindowW(std::ptr::null(), wide.as_ptr()) };
        if hwnd.is_null() {
            None
        } else {
            Some(Self {
                hwnd,
                frame: RefCell::new(None),
            })
        }
    }

    /// Captures the beacon strip from the composited desktop, or `None` if any GDI
    /// step fails (for example the window is minimized).
    fn capture(&self, extent: Size) -> Option<CapturedStrip> {
        let capture_w = i32::try_from(extent.width).ok()?;
        let capture_h = i32::try_from(extent.height).ok()?;
        if capture_w <= 0 || capture_h <= 0 {
            return None;
        }
        // SAFETY: a sequence of GDI calls whose handles are each released on every
        // exit path; all pointers passed are to local, correctly sized values.
        unsafe {
            // The strip's screen origin is the client top-left in screen space.
            let mut origin = POINT { x: 0, y: 0 };
            if ClientToScreen(self.hwnd, &mut origin) == 0 {
                return None;
            }
            // A zero client rect means the window is not currently drawable.
            let mut rect = RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
            if GetClientRect(self.hwnd, &mut rect) == 0 || rect.right <= 0 || rect.bottom <= 0 {
                return None;
            }

            let screen_dc = GetDC(std::ptr::null_mut());
            if screen_dc.is_null() {
                return None;
            }
            let mem_dc = CreateCompatibleDC(screen_dc);
            if mem_dc.is_null() {
                ReleaseDC(std::ptr::null_mut(), screen_dc);
                return None;
            }
            let bitmap = CreateCompatibleBitmap(screen_dc, capture_w, capture_h);
            if bitmap.is_null() {
                DeleteDC(mem_dc);
                ReleaseDC(std::ptr::null_mut(), screen_dc);
                return None;
            }
            let prev = SelectObject(mem_dc, bitmap);

            // CAPTUREBLT includes any layered content composited over the region.
            let blitted = BitBlt(
                mem_dc,
                0,
                0,
                capture_w,
                capture_h,
                screen_dc,
                origin.x,
                origin.y,
                SRCCOPY | CAPTUREBLT,
            );

            let mut result = None;
            if blitted != 0 {
                let mut bmi: BITMAPINFO = std::mem::zeroed();
                bmi.bmiHeader.biSize = size_of::<BITMAPINFOHEADER>() as u32;
                bmi.bmiHeader.biWidth = capture_w;
                // A negative height requests top-down rows, so index 0 is the
                // top-left pixel and the block coordinates map directly.
                bmi.bmiHeader.biHeight = -capture_h;
                bmi.bmiHeader.biPlanes = 1;
                bmi.bmiHeader.biBitCount = 32;
                bmi.bmiHeader.biCompression = BI_RGB;

                let mut pixels = vec![0u8; (capture_w * capture_h * 4) as usize];
                let lines = GetDIBits(
                    mem_dc,
                    bitmap,
                    0,
                    capture_h as u32,
                    pixels.as_mut_ptr().cast(),
                    &mut bmi,
                    DIB_RGB_COLORS,
                );
                if lines != 0 {
                    result = Some(CapturedStrip {
                        width: capture_w as u32,
                        height: capture_h as u32,
                        pixels,
                    });
                }
            }

            SelectObject(mem_dc, prev);
            DeleteObject(bitmap);
            DeleteDC(mem_dc);
            ReleaseDC(std::ptr::null_mut(), screen_dc);
            result
        }
    }

    /// Measures the client area and the monitor it sits on.
    ///
    /// `GetDpiForMonitor` with `MDT_EFFECTIVE_DPI` is used rather than
    /// `GetDpiForWindow`, because the window belongs to the game and the latter
    /// answers "what DPI should this window be drawn at given its own awareness
    /// context", which is 96 for a process that declared none. We want to know
    /// about the monitor, and that question has the same answer whoever asks it.
    ///
    /// The monitor and DPI fields degrade independently: a failure to resolve
    /// either leaves it absent rather than discarding the surface, and the DPI is
    /// never substituted with an unscaled default, because a fabricated 96 is
    /// indistinguishable from a genuinely unscaled display.
    fn measure(&self) -> Option<MeasuredDisplay> {
        // SAFETY: each call takes a pointer to a local, correctly sized value,
        // and the monitor handle is not owned so it is never released.
        unsafe {
            let mut origin = POINT { x: 0, y: 0 };
            if ClientToScreen(self.hwnd, &mut origin) == 0 {
                return None;
            }
            let mut rect = RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
            // A zero or negative client rect means the window is not currently
            // drawable (minimized, for example), which is an absent measurement
            // rather than a zero-sized one.
            if GetClientRect(self.hwnd, &mut rect) == 0 || rect.right <= 0 || rect.bottom <= 0 {
                return None;
            }

            let mut display_origin = None;
            let mut display_size = None;
            let mut dpi = None;

            let monitor = MonitorFromWindow(self.hwnd, MONITOR_DEFAULTTONULL);
            if !monitor.is_null() {
                let mut info: MONITORINFO = std::mem::zeroed();
                info.cbSize = size_of::<MONITORINFO>() as u32;
                if GetMonitorInfoW(monitor, &mut info) != 0 {
                    let width = info.rcMonitor.right - info.rcMonitor.left;
                    let height = info.rcMonitor.bottom - info.rcMonitor.top;
                    if width > 0 && height > 0 {
                        display_origin = Some(Point::new(info.rcMonitor.left, info.rcMonitor.top));
                        display_size = Some(Size::new(width as u32, height as u32));
                    }
                }
                let mut dpi_x = 0u32;
                let mut dpi_y = 0u32;
                if GetDpiForMonitor(monitor, MDT_EFFECTIVE_DPI, &mut dpi_x, &mut dpi_y) == 0
                    && dpi_x > 0
                {
                    dpi = Some(dpi_x);
                }
            }

            Some(MeasuredDisplay {
                surface: Size::new(rect.right as u32, rect.bottom as u32),
                surface_origin: Point::new(origin.x, origin.y),
                display_origin,
                display_size,
                dpi,
            })
        }
    }
}

impl SurfaceSampler for GdiSampler {
    fn prepare(&self, extent: Size) {
        *self.frame.borrow_mut() = self.capture(extent);
    }

    fn sample(&self, x: u32, y: u32) -> Option<Rgb> {
        let frame = self.frame.borrow();
        let strip = frame.as_ref()?;
        strip_pixel(&strip.pixels, strip.width, strip.height, x, y)
    }

    fn display(&self) -> Option<MeasuredDisplay> {
        self.measure()
    }
}
