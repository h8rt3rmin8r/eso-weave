//! Linux X11 surface sampler: a single-pixel read from the game window via
//! `x11rb`. It samples the active window when its title matches, so it does not
//! read the wrong window. Pure-Wayland sessions without an XWayland surface are
//! out of scope (Open Item R3).

use x11rb::connection::Connection;
use x11rb::protocol::xproto::{AtomEnum, ConnectionExt, ImageFormat};

use crate::pixelbus::display::{MeasuredDisplay, Point, Size};
use crate::pixelbus::{Rgb, SurfaceSampler};

/// Samples pixels from the game window's client area via X11.
pub struct X11Sampler {
    title: String,
}

impl X11Sampler {
    /// Creates a sampler that reads the active window when its title matches.
    pub fn for_window(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
        }
    }
}

fn active_window<C: Connection>(conn: &C, root: u32) -> Option<u32> {
    let atom = conn
        .intern_atom(false, b"_NET_ACTIVE_WINDOW")
        .ok()?
        .reply()
        .ok()?
        .atom;
    conn.get_property(false, root, atom, AtomEnum::WINDOW, 0, 1)
        .ok()?
        .reply()
        .ok()?
        .value32()?
        .next()
}

fn window_title<C: Connection>(conn: &C, window: u32) -> Option<String> {
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
    let reply = conn
        .get_property(false, window, name_atom, utf8_atom, 0, 1024)
        .ok()?
        .reply()
        .ok()?;
    Some(String::from_utf8_lossy(&reply.value).into_owned())
}

impl SurfaceSampler for X11Sampler {
    fn sample(&self, x: u32, y: u32) -> Option<Rgb> {
        let (conn, screen_num) = x11rb::connect(None).ok()?;
        let root = conn.setup().roots[screen_num].root;
        let window = active_window(&conn, root)?;
        if !window_title(&conn, window)?.contains(&self.title) {
            return None;
        }
        let image = conn
            .get_image(
                ImageFormat::Z_PIXMAP,
                window,
                x as i16,
                y as i16,
                1,
                1,
                u32::MAX,
            )
            .ok()?
            .reply()
            .ok()?;
        let data = image.data;
        if data.len() >= 3 {
            // Common little-endian BGR(X) byte order for 24 and 32 bit visuals.
            Some(Rgb::new(data[2], data[1], data[0]))
        } else {
            None
        }
    }

    /// Measures the game window and the screen it is on, using only the core X
    /// protocol.
    ///
    /// Two limits are inherent to that choice and are stated rather than worked
    /// around. The reported display size is the X screen, which on a multi-head
    /// session is the union of every head rather than the head the window is on;
    /// per-monitor rectangles need the RandR extension. And the scale is always
    /// absent, because the core protocol has no scale factor at all. Deriving one
    /// from the screen's millimetre dimensions was considered and rejected: those
    /// values are routinely fabricated by drivers, and a plausible wrong scale is
    /// worse than an honest unknown.
    fn display(&self) -> Option<MeasuredDisplay> {
        let (conn, screen_num) = x11rb::connect(None).ok()?;
        let screen = &conn.setup().roots[screen_num];
        let root = screen.root;
        let window = active_window(&conn, root)?;
        if !window_title(&conn, window)?.contains(&self.title) {
            return None;
        }

        let geometry = conn.get_geometry(window).ok()?.reply().ok()?;
        if geometry.width == 0 || geometry.height == 0 {
            return None;
        }
        // The window's own origin is relative to its parent, so the absolute
        // position comes from translating its top-left into root coordinates.
        let origin = conn
            .translate_coordinates(window, root, 0, 0)
            .ok()?
            .reply()
            .ok()?;

        let display_size = if screen.width_in_pixels > 0 && screen.height_in_pixels > 0 {
            Some(Size::new(
                u32::from(screen.width_in_pixels),
                u32::from(screen.height_in_pixels),
            ))
        } else {
            None
        };

        Some(MeasuredDisplay {
            surface: Size::new(u32::from(geometry.width), u32::from(geometry.height)),
            surface_origin: Point::new(i32::from(origin.dst_x), i32::from(origin.dst_y)),
            display_origin: display_size.map(|_| Point::new(0, 0)),
            display_size,
            dpi: None,
        })
    }
}
