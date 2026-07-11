//! Windows GDI surface sampler: a single-pixel read from the game window.
//!
//! `GetPixel` on a hardware-accelerated game window is the mechanism the master
//! specification chose; its behavior on a DirectX surface is validated in-game.

use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::Graphics::Gdi::{GetDC, GetPixel, ReleaseDC, CLR_INVALID};
use windows_sys::Win32::UI::WindowsAndMessaging::FindWindowW;

use crate::pixelbus::{Rgb, SurfaceSampler};

/// Samples pixels from a window's client area via GDI.
pub struct GdiSampler {
    hwnd: HWND,
}

impl GdiSampler {
    /// Resolves the window by its exact title, returning `None` if not found.
    pub fn for_window(title: &str) -> Option<Self> {
        let wide: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
        let hwnd = unsafe { FindWindowW(std::ptr::null(), wide.as_ptr()) };
        if hwnd.is_null() {
            None
        } else {
            Some(Self { hwnd })
        }
    }
}

impl SurfaceSampler for GdiSampler {
    fn sample(&self, x: u32, y: u32) -> Option<Rgb> {
        unsafe {
            let hdc = GetDC(self.hwnd);
            if hdc.is_null() {
                return None;
            }
            let color = GetPixel(hdc, x as i32, y as i32);
            ReleaseDC(self.hwnd, hdc);
            if color == CLR_INVALID {
                return None;
            }
            // COLORREF is 0x00BBGGRR.
            Some(Rgb::new(
                (color & 0xFF) as u8,
                ((color >> 8) & 0xFF) as u8,
                ((color >> 16) & 0xFF) as u8,
            ))
        }
    }
}
