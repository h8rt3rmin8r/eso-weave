//! Windows surface sampler: a screen-composited capture of the beacon strip.
//!
//! `GetPixel` on the game window device context reads that window's GDI front
//! buffer, which for a hardware-accelerated (DirectX) game does not contain the
//! rendered content, so it returns black or stale pixels and the beacon signal is
//! never read. Instead this backend captures a small strip from the composited
//! desktop (a `BitBlt` from the screen device context, the same mechanism as the
//! CopyFromScreen workaround that captures accelerated content) and reads the four
//! block points from it.

use std::cell::RefCell;
use std::mem::size_of;

use windows_sys::Win32::Foundation::{HWND, POINT, RECT};
use windows_sys::Win32::Graphics::Gdi::{
    BitBlt, ClientToScreen, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject,
    GetDC, GetDIBits, ReleaseDC, SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, CAPTUREBLT,
    DIB_RGB_COLORS, SRCCOPY,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{FindWindowW, GetClientRect};

use crate::pixelbus::{capture_dims, strip_pixel, Rgb, SurfaceSampler};

/// The captured beacon strip: a small top-left region of the client area, as
/// composited on screen, in 32-bit BGRA (the layout `GetDIBits` fills).
struct CapturedStrip {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

/// Samples the beacon strip from the composited desktop for one window. The strip
/// dimensions are derived from the block size (`capture_dims`), so the capture
/// region tracks the same single source of truth as the read points: at the
/// default block size it is the historical 64 by 16.
pub struct GdiSampler {
    hwnd: HWND,
    capture_w: i32,
    capture_h: i32,
    frame: RefCell<Option<CapturedStrip>>,
}

impl GdiSampler {
    /// Resolves the window by its exact title, sizing the capture region from
    /// `block_px`. Returns `None` if the window is not found.
    pub fn for_window(title: &str, block_px: u32) -> Option<Self> {
        let wide: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
        let hwnd = unsafe { FindWindowW(std::ptr::null(), wide.as_ptr()) };
        if hwnd.is_null() {
            None
        } else {
            let (w, h) = capture_dims(block_px);
            Some(Self {
                hwnd,
                capture_w: w as i32,
                capture_h: h as i32,
                frame: RefCell::new(None),
            })
        }
    }

    /// Captures the beacon strip from the composited desktop, or `None` if any GDI
    /// step fails (for example the window is minimized).
    fn capture(&self) -> Option<CapturedStrip> {
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
            let bitmap = CreateCompatibleBitmap(screen_dc, self.capture_w, self.capture_h);
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
                self.capture_w,
                self.capture_h,
                screen_dc,
                origin.x,
                origin.y,
                SRCCOPY | CAPTUREBLT,
            );

            let mut result = None;
            if blitted != 0 {
                let mut bmi: BITMAPINFO = std::mem::zeroed();
                bmi.bmiHeader.biSize = size_of::<BITMAPINFOHEADER>() as u32;
                bmi.bmiHeader.biWidth = self.capture_w;
                // A negative height requests top-down rows, so index 0 is the
                // top-left pixel and the block coordinates map directly.
                bmi.bmiHeader.biHeight = -self.capture_h;
                bmi.bmiHeader.biPlanes = 1;
                bmi.bmiHeader.biBitCount = 32;
                bmi.bmiHeader.biCompression = BI_RGB;

                let mut pixels = vec![0u8; (self.capture_w * self.capture_h * 4) as usize];
                let lines = GetDIBits(
                    mem_dc,
                    bitmap,
                    0,
                    self.capture_h as u32,
                    pixels.as_mut_ptr().cast(),
                    &mut bmi,
                    DIB_RGB_COLORS,
                );
                if lines != 0 {
                    result = Some(CapturedStrip {
                        width: self.capture_w as u32,
                        height: self.capture_h as u32,
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
}

impl SurfaceSampler for GdiSampler {
    fn prepare(&self) {
        *self.frame.borrow_mut() = self.capture();
    }

    fn sample(&self, x: u32, y: u32) -> Option<Rgb> {
        let frame = self.frame.borrow();
        let strip = frame.as_ref()?;
        strip_pixel(&strip.pixels, strip.width, strip.height, x, y)
    }
}
