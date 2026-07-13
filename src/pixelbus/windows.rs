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

use crate::pixelbus::{strip_pixel, Rgb, SurfaceSampler};

/// The captured beacon strip: a small top-left region of the client area, as
/// composited on screen, in 32-bit BGRA (the layout `GetDIBits` fills).
struct CapturedStrip {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

/// The strip captured each frame, covering the four block sample points
/// (x = 8, 24, 40, 56) across 16 px blocks, so 64 by 16 points is enough.
const CAPTURE_W: i32 = 64;
const CAPTURE_H: i32 = 16;

/// Samples the beacon strip from the composited desktop for one window.
pub struct GdiSampler {
    hwnd: HWND,
    frame: RefCell<Option<CapturedStrip>>,
}

impl GdiSampler {
    /// Resolves the window by its exact title, returning `None` if not found.
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
            let bitmap = CreateCompatibleBitmap(screen_dc, CAPTURE_W, CAPTURE_H);
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
                CAPTURE_W,
                CAPTURE_H,
                screen_dc,
                origin.x,
                origin.y,
                SRCCOPY | CAPTUREBLT,
            );

            let mut result = None;
            if blitted != 0 {
                let mut bmi: BITMAPINFO = std::mem::zeroed();
                bmi.bmiHeader.biSize = size_of::<BITMAPINFOHEADER>() as u32;
                bmi.bmiHeader.biWidth = CAPTURE_W;
                // A negative height requests top-down rows, so index 0 is the
                // top-left pixel and the block coordinates map directly.
                bmi.bmiHeader.biHeight = -CAPTURE_H;
                bmi.bmiHeader.biPlanes = 1;
                bmi.bmiHeader.biBitCount = 32;
                bmi.bmiHeader.biCompression = BI_RGB;

                let mut pixels = vec![0u8; (CAPTURE_W * CAPTURE_H * 4) as usize];
                let lines = GetDIBits(
                    mem_dc,
                    bitmap,
                    0,
                    CAPTURE_H as u32,
                    pixels.as_mut_ptr().cast(),
                    &mut bmi,
                    DIB_RGB_COLORS,
                );
                if lines != 0 {
                    result = Some(CapturedStrip {
                        width: CAPTURE_W as u32,
                        height: CAPTURE_H as u32,
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
