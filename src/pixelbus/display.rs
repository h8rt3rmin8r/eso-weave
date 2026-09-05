//! Out-of-band display detection: how big the game's render surface is, which
//! physical display it is on, and how that display is scaled, resolved without
//! reading the pixel bus.
//!
//! The bus cannot be used to locate the bus. A future grid layout has to know
//! how many blocks fit across the client area before it can know where any
//! block is, so the surface size has to come from somewhere that is not the
//! surface. This module is that somewhere: the operating system, which is
//! authoritative for what is on screen right now, and the game's own stored
//! video settings, which are a cross-check and a pre-launch fallback and are
//! never allowed to override a live reading.
//!
//! Everything here is pure. The operating system stays behind
//! [`SurfaceSampler::display`](super::SurfaceSampler::display) and the settings
//! file is supplied to [`DisplayDetector::update`] as a closure the detector
//! calls only when it decides a read is warranted, so every decision in the
//! feature is reachable from a unit test with no window, no file, and no
//! display hardware.
//!
//! Two limits are deliberate and are stated rather than papered over. The
//! stored window-mode value is reported raw and never mapped to a named mode,
//! because no verified mapping exists and a guess would be a confident wrong
//! answer. And on X11 the reported display size is the whole X screen, which on
//! a multi-head session is the union of every head rather than the head the
//! window is on; the core X protocol has no per-monitor rectangle and no scale
//! factor at all.

/// A width and height in physical device pixels.
///
/// Unsigned because a negative extent is not a thing that exists: platform code
/// converting a signed rectangle rejects a non-positive value at the boundary
/// rather than carrying it inward.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Size {
    /// Width in physical pixels.
    pub width: u32,
    /// Height in physical pixels.
    pub height: u32,
}

impl Size {
    /// Creates a size.
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Whether either dimension is zero. A zero-extent size is never published
    /// in a descriptor, because a consumer would compute a zero column count
    /// from it.
    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }
}

/// A point in physical device pixels, in screen coordinates.
///
/// Signed because a display arranged to the left of the primary one has a
/// negative origin, and so does a window on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Point {
    /// Horizontal coordinate.
    pub x: i32,
    /// Vertical coordinate.
    pub y: i32,
}

impl Point {
    /// Creates a point.
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Whether the beacon grid fits inside the game's client area.
///
/// Advisory in the strongest sense: nothing in the application branches on this.
/// Refusing to sample a grid that overflows would turn a partial loss into a
/// total one, because the blocks that do fit still decode correctly.
///
/// The failure it exists to name is otherwise very hard to attribute. A block
/// drawn past the client edge is captured as black, fails its marker check, and
/// decodes as absent, which looks exactly like an addon that was never
/// installed, so an operator can spend a long time debugging an addon that is
/// installed, loaded, and drawing correctly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridFit {
    /// The grid lies within the client area.
    Fits,
    /// The grid is wider or taller than the client area.
    Exceeds {
        /// The grid's extent in physical pixels.
        grid: Size,
        /// The client area it did not fit inside.
        surface: Size,
    },
}

/// Compares the grid's extent against a measured client area.
///
/// Extents only. The grid is anchored at the client area's top-left, so its
/// offset within that area is always zero, and where the window sits on the
/// desktop is a capture concern the sampler already handles rather than a layout
/// one. Equality on either axis fits: a grid exactly as wide as the client area
/// is entirely on screen.
pub fn grid_fit(grid: Size, surface: Size) -> GridFit {
    if grid.width <= surface.width && grid.height <= surface.height {
        GridFit::Fits
    } else {
        GridFit::Exceeds { grid, surface }
    }
}

/// Reports a change in whether the grid fits, and only a change.
///
/// This is worth being a type rather than a comparison at the call site because
/// the distinction it draws is easy to lose: two successive descriptor changes
/// can both overflow (a too-small window resized to a differently too-small
/// one), and what is wanted is one report per change of *outcome*, not one per
/// change of descriptor.
#[derive(Debug, Clone, Copy, Default)]
pub struct GridFitWatch {
    /// Whether the grid fitted last time, and nothing else.
    ///
    /// Deliberately a boolean rather than the [`GridFit`] itself. `Exceeds`
    /// carries the extents that did not fit, so comparing whole values would
    /// make a resize from one too-small window to a differently too-small window
    /// look like a change of outcome, which is exactly the repeat this type
    /// exists to suppress. The extents are still reported, they are just not
    /// what "changed" is decided on.
    last_fitted: Option<bool>,
}

impl GridFitWatch {
    /// Creates a watch with no remembered outcome.
    pub fn new() -> Self {
        Self::default()
    }

    /// Folds in the current grid extent and descriptor, returning `Some` only
    /// when the outcome changed.
    ///
    /// A missing descriptor, or one derived from stored settings rather than a
    /// live window, yields `None` and forgets the previous outcome, so a later
    /// measurement reports afresh. A configured descriptor is produced only when
    /// there is no window, and no window means no drawn grid to be about.
    pub fn observe(
        &mut self,
        grid: Size,
        descriptor: Option<&DisplayDescriptor>,
    ) -> Option<GridFit> {
        let surface = match descriptor {
            Some(descriptor) if descriptor.source == DisplaySource::Measured => descriptor.surface,
            _ => {
                self.last_fitted = None;
                return None;
            }
        };
        let outcome = grid_fit(grid, surface);
        let fitted = outcome == GridFit::Fits;
        if self.last_fitted == Some(fitted) {
            return None;
        }
        self.last_fitted = Some(fitted);
        Some(outcome)
    }
}

/// What a platform probe measured about the game window and the display it is
/// on. Produced only by [`SurfaceSampler::display`](super::SurfaceSampler::display).
///
/// The fields degrade independently: a backend that can resolve the window but
/// not the monitor supplies the first two and leaves the rest absent, which is
/// the X11 situation for `dpi` on every session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeasuredDisplay {
    /// The client area in physical pixels. A probe returns `None` rather than a
    /// zero here.
    pub surface: Size,
    /// The client top-left in screen coordinates.
    pub surface_origin: Point,
    /// The top-left of the display the surface sits on.
    pub display_origin: Option<Point>,
    /// The extent of that display.
    pub display_size: Option<Size>,
    /// The effective dots per inch of that display.
    pub dpi: Option<u32>,
}

/// Where a [`DisplayDescriptor`] came from.
///
/// This is not decoration. A measured descriptor is a statement about what is on
/// screen right now; a configured one is a statement about what the game was set
/// up to do, which can be stale, hand-edited, or describe a display the game is
/// no longer on. A consumer that needs the live surface must require
/// [`DisplaySource::Measured`], and this type is what makes that check possible
/// rather than leaving it to a comment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplaySource {
    /// The operating system was asked about a live window.
    Measured,
    /// The game's stored video settings were read from disk.
    Configured,
}

/// The feature's output: the render surface, where it is, and what it is on.
///
/// Every value is in physical device pixels, the same unit as
/// [`BusLayout`](super::BusLayout) points and extents, so validating published
/// columns against `surface.width` compares like with like. The scale is never
/// applied to those values; it is carried so a consumer can use it.
///
/// `surface` is the only field always present. Everything else is absent-capable
/// because a configured descriptor can supply the surface size and nothing else,
/// and because a platform may not expose a scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayDescriptor {
    /// The render surface in physical pixels. Never zero in either dimension.
    pub surface: Size,
    /// The surface's top-left in screen coordinates.
    pub surface_origin: Option<Point>,
    /// The top-left of the display the surface sits on.
    pub display_origin: Option<Point>,
    /// The extent of that display.
    pub display_size: Option<Size>,
    /// The effective dots per inch of that display.
    pub dpi: Option<u32>,
    /// Where this reading came from.
    pub source: DisplaySource,
}

/// The reference dots per inch that [`DisplayDescriptor::scale`] divides by: the
/// platform value at which no scaling is applied.
const UNSCALED_DPI: f32 = 96.0;

impl DisplayDescriptor {
    /// Builds a measured descriptor, or `None` when the surface has a zero
    /// dimension. A zero-extent descriptor is never published, because a
    /// consumer would compute a zero column count from it.
    pub fn from_measured(measured: MeasuredDisplay) -> Option<Self> {
        if measured.surface.is_empty() {
            return None;
        }
        Some(Self {
            surface: measured.surface,
            surface_origin: Some(measured.surface_origin),
            display_origin: measured.display_origin,
            display_size: measured.display_size,
            dpi: measured.dpi,
            source: DisplaySource::Measured,
        })
    }

    /// Builds a configured descriptor from stored settings, or `None`.
    ///
    /// Produced only when both stored resolution pairs are present, equal, and
    /// non-zero. That looks narrow and is: because the stored window-mode value
    /// is never mapped (see [`StoredVideoSettings::window_mode_raw`]), identical
    /// pairs are the only configuration in which the file determines the live
    /// surface without a guess. The result carries the surface size alone; the
    /// file records a display *index*, and an index is not geometry.
    pub fn from_stored(stored: &StoredVideoSettings) -> Option<Self> {
        let fullscreen = stored.fullscreen?;
        let windowed = stored.windowed?;
        if fullscreen != windowed || fullscreen.is_empty() {
            return None;
        }
        Some(Self {
            surface: fullscreen,
            surface_origin: None,
            display_origin: None,
            display_size: None,
            dpi: None,
            source: DisplaySource::Configured,
        })
    }

    /// The display's scale factor (1.0 meaning unscaled), or `None` when the
    /// platform did not expose one.
    ///
    /// Computed rather than stored, so the descriptor stays integral and change
    /// detection is an exact comparison rather than one with a tolerance.
    pub fn scale(&self) -> Option<f32> {
        self.dpi.map(|dpi| dpi as f32 / UNSCALED_DPI)
    }
}

/// The game's stored video settings, as found in its settings file.
///
/// Every field is independently optional: a missing, malformed, or unparsable
/// entry leaves that field absent and affects no other. A resolution pair is
/// present only when both its width and its height parsed, because half a
/// resolution is not a resolution.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StoredVideoSettings {
    /// The render size stored for a fullscreen mode.
    pub fullscreen: Option<Size>,
    /// The render size stored for windowed mode.
    pub windowed: Option<Size>,
    /// The window-mode enum, exactly as read.
    ///
    /// Deliberately raw. No verified integer-to-mode mapping exists, and there
    /// is no named mode anywhere in this module to map it to. Guessing would
    /// produce a confident wrong answer about which resolution pair is live on
    /// exactly the installs where the two pairs differ, which is where it
    /// matters. When a window exists the operating system answers that question
    /// directly, and when one does not, the answer has no consumer.
    pub window_mode_raw: Option<i64>,
    /// Exclusive versus borderless fullscreen preference.
    pub prefer_exclusive_fullscreen: Option<i64>,
    /// Maximized-window preference.
    pub prefer_maximized_window: Option<i64>,
    /// Which display the game targets, as an index. An index is not geometry.
    pub active_display: Option<i64>,
    /// The overscan width and height adjustments. Recorded, never applied:
    /// applying them would be a layout decision, and layout is a later feature.
    pub overscan: Option<Point>,
    /// The custom interface scale factor.
    pub custom_ui_scale: Option<f64>,
    /// Whether the custom interface scale is in use.
    pub use_custom_ui_scale: Option<i64>,
    /// The gamepad-mode custom interface scale factor.
    pub gamepad_custom_ui_scale: Option<f64>,
    /// Whether the gamepad-mode custom interface scale is in use.
    pub use_gamepad_custom_ui_scale: Option<i64>,
}

/// Strips an optional trailing `.N` version suffix and lowercases the rest.
///
/// The game bumps that suffix when a setting's meaning changes (seen on
/// `UseCustomUIScale.2`), so matching a fixed literal would silently stop
/// matching after a patch. Base keys are compared whole rather than by prefix,
/// so `FULLSCREEN` never matches `FullscreenWidth`.
fn normalize_key(key: &str) -> String {
    let base = match key.rfind('.') {
        Some(dot) if dot + 1 < key.len() && key[dot + 1..].bytes().all(|b| b.is_ascii_digit()) => {
            &key[..dot]
        }
        _ => key,
    };
    base.to_ascii_lowercase()
}

/// Parses the game's stored video settings.
///
/// Total: every input produces a value and no input panics. Degradation is per
/// key rather than per file, so one renamed or malformed entry never discards
/// the rest. Where a key appears more than once the last assignment wins, which
/// is how a sequentially written settings file reads back.
///
/// Each line is expected to be `SET <Key> "<value>"`, with the value quoted even
/// when numeric. Anything else is skipped.
pub fn parse_user_settings(text: &str) -> StoredVideoSettings {
    let mut settings = StoredVideoSettings::default();
    // The width and height of each pair are separate keys, so they are collected
    // apart and only combined once both halves are known.
    let (mut fw, mut fh) = (None, None);
    let (mut ww, mut wh) = (None, None);
    let (mut ow, mut oh) = (None, None);

    for line in text.lines() {
        let line = line.trim();
        let mut parts = line.splitn(3, char::is_whitespace);
        let Some(verb) = parts.next() else { continue };
        if !verb.eq_ignore_ascii_case("SET") {
            continue;
        }
        let Some(key) = parts.next() else { continue };
        let Some(raw) = parts.next() else { continue };
        // Quotes are trimmed permissively: a line truncated mid-value leaves an
        // unmatched quote, and losing the value over that would be a per-file
        // failure where the format only justifies a per-line one.
        let value = raw.trim().trim_matches('"');

        match normalize_key(key).as_str() {
            "fullscreenwidth" => fw = value.parse().ok().or(fw),
            "fullscreenheight" => fh = value.parse().ok().or(fh),
            "windowedwidth" => ww = value.parse().ok().or(ww),
            "windowedheight" => wh = value.parse().ok().or(wh),
            "overscanwidthadjustment" => ow = value.parse().ok().or(ow),
            "overscanheightadjustment" => oh = value.parse().ok().or(oh),
            "fullscreen" => settings.window_mode_raw = value.parse().ok(),
            "preferexclusivefullscreen" => {
                settings.prefer_exclusive_fullscreen = value.parse().ok()
            }
            "prefermaximizedwindow" => settings.prefer_maximized_window = value.parse().ok(),
            "active_display" => settings.active_display = value.parse().ok(),
            "customuiscale" => settings.custom_ui_scale = value.parse().ok(),
            "usecustomuiscale" => settings.use_custom_ui_scale = value.parse().ok(),
            "gamepadcustomuiscale" => settings.gamepad_custom_ui_scale = value.parse().ok(),
            "usegamepadcustomuiscale" => settings.use_gamepad_custom_ui_scale = value.parse().ok(),
            _ => {}
        }
    }

    settings.fullscreen = fw.zip(fh).map(|(w, h)| Size::new(w, h));
    settings.windowed = ww.zip(wh).map(|(w, h)| Size::new(w, h));
    settings.overscan = ow.zip(oh).map(|(x, y)| Point::new(x, y));
    settings
}

/// Which stored resolution pair a measured surface matched.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoredPair {
    /// The pair stored for a fullscreen mode.
    Fullscreen,
    /// The pair stored for windowed mode.
    Windowed,
}

/// How the measured surface related to the stored settings.
///
/// Observational. Nothing in this feature branches on it; it exists to be
/// written to the log.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reconciliation {
    /// The settings file produced nothing usable.
    NoStored,
    /// Settings were read but neither resolution pair is present.
    NoPairs,
    /// The measured surface matched exactly one stored pair.
    ///
    /// This variant is the useful one. The pairing of a matched pair with the
    /// raw window-mode value is evidence about what that unmapped integer meant
    /// on this install, accumulated from ordinary use rather than from anyone
    /// toggling modes and diffing the file. The feature records it and does not
    /// act on it.
    Agreed {
        /// The pair the measured surface matched.
        pair: StoredPair,
        /// The raw window-mode value that accompanied it.
        mode_raw: Option<i64>,
    },
    /// The measured surface matched both pairs, which happens when the two
    /// stored pairs are identical. Says nothing about the mode value.
    Ambiguous,
    /// The measured surface matched neither pair. Recorded and ignored: the
    /// measurement is authoritative and is used unchanged.
    Disagreed {
        /// The surface that matched nothing.
        measured: Size,
    },
}

/// Compares a measured surface against the stored settings.
///
/// Returns an observation and never a descriptor: reconciliation observes, it
/// does not decide.
pub fn reconcile(measured: Size, stored: Option<&StoredVideoSettings>) -> Reconciliation {
    let Some(stored) = stored else {
        return Reconciliation::NoStored;
    };
    if stored.fullscreen.is_none() && stored.windowed.is_none() {
        return Reconciliation::NoPairs;
    }
    let matched = Some(measured);
    match (stored.fullscreen == matched, stored.windowed == matched) {
        (true, true) => Reconciliation::Ambiguous,
        (true, false) => Reconciliation::Agreed {
            pair: StoredPair::Fullscreen,
            mode_raw: stored.window_mode_raw,
        },
        (false, true) => Reconciliation::Agreed {
            pair: StoredPair::Windowed,
            mode_raw: stored.window_mode_raw,
        },
        (false, false) => Reconciliation::Disagreed { measured },
    }
}

/// What changed, reported by [`DisplayDetector::update`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayUpdate {
    /// The new descriptor, or `None` when the surface became unreadable.
    pub descriptor: Option<DisplayDescriptor>,
    /// How the new descriptor related to the stored settings, when they were
    /// consulted. Absent when they were not.
    pub reconciliation: Option<Reconciliation>,
}

/// Keeps the descriptor current and decides when the settings file is worth
/// reading.
///
/// The whole point of this type is that it performs no input or output. The
/// settings read is supplied to [`update`](Self::update) as a closure that is
/// called only when a read is warranted, which is what makes "a stationary
/// window reads no files" a property a test can count rather than an intention.
///
/// It stores the current descriptor and nothing else. The last reconciliation
/// outcome is deliberately not kept: reconciliation is recomputed only when the
/// descriptor changes, so a stored outcome could never change independently of
/// the descriptor and would be state that exists to be compared against itself.
#[derive(Debug, Clone, Default)]
pub struct DisplayDetector {
    current: Option<DisplayDescriptor>,
    /// Whether the pre-launch configured fallback has already been tried, so a
    /// session with the game closed does not re-read the file every cycle.
    configured_attempted: bool,
}

impl DisplayDetector {
    /// Creates a detector with no descriptor.
    pub fn new() -> Self {
        Self::default()
    }

    /// The current descriptor, or `None` when none has been resolved.
    pub fn current(&self) -> Option<&DisplayDescriptor> {
        self.current.as_ref()
    }

    /// Folds a fresh measurement in, returning `Some` only when the descriptor
    /// changed, so a caller may log unconditionally on `Some` without flooding.
    ///
    /// `stored` is called at most once, and is not called at all when the
    /// measurement is unchanged or when a previously present measurement has
    /// disappeared: losing the window is not a reason to consult a file.
    pub fn update<F>(
        &mut self,
        measured: Option<MeasuredDisplay>,
        stored: F,
    ) -> Option<DisplayUpdate>
    where
        F: FnOnce() -> Option<StoredVideoSettings>,
    {
        match measured {
            Some(measured) => {
                let candidate = DisplayDescriptor::from_measured(measured);
                if candidate == self.current {
                    return None;
                }
                // Only a live surface is worth cross-checking against the file;
                // a surface that just became unreadable is not.
                let reconciliation =
                    candidate.map(|descriptor| reconcile(descriptor.surface, stored().as_ref()));
                self.current = candidate;
                self.configured_attempted = true;
                Some(DisplayUpdate {
                    descriptor: candidate,
                    reconciliation,
                })
            }
            None => {
                if let Some(current) = self.current {
                    // A configured descriptor is what the absence of a window
                    // looks like, so it persists rather than being cleared by it.
                    if current.source == DisplaySource::Configured {
                        return None;
                    }
                    self.current = None;
                    self.configured_attempted = true;
                    return Some(DisplayUpdate {
                        descriptor: None,
                        reconciliation: None,
                    });
                }
                if self.configured_attempted {
                    return None;
                }
                self.configured_attempted = true;
                let candidate = stored().as_ref().and_then(DisplayDescriptor::from_stored);
                candidate?;
                self.current = candidate;
                Some(DisplayUpdate {
                    descriptor: candidate,
                    reconciliation: None,
                })
            }
        }
    }
}
