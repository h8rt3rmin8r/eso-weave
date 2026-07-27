//! Pixel Bus Reader: samples the PixelBeacon blocks from the game window surface
//! and decodes them into typed events.
//!
//! The decoders and the [`PixelBusReader`] state machine are pure and fully
//! tested with crafted samples and an injected clock. Surface sampling sits
//! behind the [`SurfaceSampler`] seam with a mock plus thin OS backends.

pub mod display;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(windows)]
mod windows;

pub use display::{
    parse_user_settings, reconcile, DisplayDescriptor, DisplayDetector, DisplaySource,
    DisplayUpdate, MeasuredDisplay, Point, Reconciliation, Size, StoredPair, StoredVideoSettings,
};
#[cfg(target_os = "linux")]
pub use linux::X11Sampler;
#[cfg(windows)]
pub use windows::GdiSampler;

use std::collections::HashMap;

use serde::Deserialize;

use crate::config::{Notice, NoticeKind};

/// A red-green-blue color triple sampled from a beacon point.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    /// Red channel.
    pub r: u8,
    /// Green channel.
    pub g: u8,
    /// Blue channel.
    pub b: u8,
}

impl Rgb {
    /// Creates a color from its channels.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

/// The decoded fishing signal from the fishing block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FishingSignal {
    /// No fishing block present.
    None,
    /// A cast is active and waiting.
    Waiting,
    /// A bite is detected.
    Bite,
}

/// The active weapon bar decoded from the weapon-bar block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveBar {
    /// The bar could not be determined.
    Unknown,
    /// The front (primary) bar is active.
    Front,
    /// The back (backup) bar is active.
    Back,
}

impl ActiveBar {
    /// Decodes the active bar from its wire code (0 unknown, 1 front, 2 back).
    pub fn from_code(code: u8) -> Self {
        match code {
            1 => ActiveBar::Front,
            2 => ActiveBar::Back,
            _ => ActiveBar::Unknown,
        }
    }
}

/// A normalized weapon class on one bar. The integer codes are a fixed contract
/// shared byte-for-byte with the PixelBeacon addon (`PixelBeacon.lua`), which maps
/// the game `WEAPONTYPE_*` constants to these codes, so the reader never depends on
/// the raw game enum integers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponClass {
    /// No weapon or an unrecognized class (code 0).
    Unknown,
    /// Dual wield (code 1).
    DualWield,
    /// Two handed (code 2).
    TwoHanded,
    /// One hand and shield (code 3).
    SwordAndShield,
    /// Bow (code 4).
    Bow,
    /// Destruction staff (code 5).
    DestructionStaff,
    /// Restoration staff (code 6).
    RestorationStaff,
}

impl WeaponClass {
    /// Decodes a weapon class from its wire code (a 0..6 nibble).
    pub fn from_code(code: u8) -> Self {
        match code {
            1 => WeaponClass::DualWield,
            2 => WeaponClass::TwoHanded,
            3 => WeaponClass::SwordAndShield,
            4 => WeaponClass::Bow,
            5 => WeaponClass::DestructionStaff,
            6 => WeaponClass::RestorationStaff,
            _ => WeaponClass::Unknown,
        }
    }
}

/// The decoded weapon-bar signal: the active bar and each bar's weapon class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeaponBarSignal {
    /// The active weapon bar.
    pub bar: ActiveBar,
    /// The weapon class on the front bar.
    pub front: WeaponClass,
    /// The weapon class on the back bar.
    pub back: WeaponClass,
}

/// The green marker that identifies a weapon-bar sample (distinct from the latency
/// marker `0xA5` so tolerance can never confuse the two).
const WEAPON_MARKER: u8 = 0x5A;

/// The decoded combat signal from the combat block.
///
/// [`CombatSignal::Unknown`] is not a fourth game state. It means the companion
/// could not read the signal: the block is absent (an addon older than version 6
/// draws only four blocks), the sample failed validation, or the beacon signal is
/// lost. It is deliberately distinct from [`CombatSignal::OutOfCombat`], because
/// collapsing the two would make a missing addon look like a peaceful session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CombatSignal {
    /// The combat state could not be read.
    #[default]
    Unknown,
    /// The player is not in combat.
    OutOfCombat,
    /// The player is in combat.
    InCombat,
}

/// The green marker that identifies a combat sample.
///
/// Chosen at least 45 away from every other green that appears at a block center
/// (see [`BLOCK_CENTER_GREENS`]), which is more than twenty times the default
/// tolerance. Its nibble swap `0xD2` is reserved as the natural marker for the
/// next block added to the strip, continuing the `0xA5` and `0x5A` pairing.
const COMBAT_MARKER: u8 = 0x2D;
/// The red channel encoding the in-combat state.
const COMBAT_IN_RED: u8 = 0xE0;
/// The red channel encoding the out-of-combat state.
const COMBAT_OUT_RED: u8 = 0x20;

/// Which native game UI surface is active, decoded from the menu block.
///
/// [`MenuSurface::None`] means gameplay: no surface is active. It is also the
/// value produced by every failure mode (an addon older than version 7 draws no
/// menu block, a sample fails validation, or the beacon signal is lost), which is
/// what makes every failure degrade to the application's behavior without the
/// menu gate rather than to a gate stuck on.
///
/// [`MenuSurface::Other`] is a surface the addon could not name. It gates exactly
/// like a named one: the addon decides that a surface is active from the game's
/// UI-mode state before it tries to label it, so an unrecognized or renamed scene
/// costs precision in the readout and never costs the gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MenuSurface {
    /// No surface is active; the operator is playing.
    #[default]
    None,
    /// The system (Escape) menu.
    SystemMenu,
    /// The world map.
    Map,
    /// Inventory.
    Inventory,
    /// Mail.
    Mail,
    /// Character and skills.
    Character,
    /// The guild store.
    GuildStore,
    /// The crown store.
    CrownStore,
    /// The journal.
    Journal,
    /// Chat text entry is open.
    ChatEntry,
    /// A surface the addon does not enumerate.
    Other,
}

impl MenuSurface {
    /// Decodes a surface from its wire code (`0..=MENU_CODE_MAX`).
    fn from_code(code: u8) -> Option<Self> {
        Some(match code {
            0 => MenuSurface::None,
            1 => MenuSurface::SystemMenu,
            2 => MenuSurface::Map,
            3 => MenuSurface::Inventory,
            4 => MenuSurface::Mail,
            5 => MenuSurface::Character,
            6 => MenuSurface::GuildStore,
            7 => MenuSurface::CrownStore,
            8 => MenuSurface::Journal,
            9 => MenuSurface::ChatEntry,
            10 => MenuSurface::Other,
            _ => return None,
        })
    }

    /// Whether this surface gates input. True for everything except gameplay.
    ///
    /// The gate never asks which surface it is, only that it is not
    /// [`MenuSurface::None`], so a surface the addon could not name still gates.
    pub fn gates(self) -> bool {
        self != MenuSurface::None
    }
}

/// The green marker that identifies a menu sample. The nibble swap of the combat
/// marker, as that block's documentation reserved.
const MENU_MARKER: u8 = 0xD2;
/// The red-channel spacing between adjacent surface codes. Twelve times the
/// default tolerance, so a code can never be read as its neighbour.
const MENU_CODE_STEP: u8 = 24;
/// The highest surface code (the generic "other" value).
const MENU_CODE_MAX: u8 = 10;

/// A decoded resource reading: a whole percentage, or unreadable.
///
/// [`ResourceLevel::Unknown`] is deliberately distinct from `Percent(0)`. Zero
/// means the pool is genuinely empty, which is a real and important state; unknown
/// means there is no reading at all (an addon older than version 8 draws no
/// resource blocks, the sample failed validation, or the beacon signal is lost).
/// Collapsing them would make a missing addon look like a dead character.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ResourceLevel {
    /// The resource could not be read.
    #[default]
    Unknown,
    /// A whole percentage of the resource current maximum, 0 to 100.
    Percent(u8),
}

/// The three resource pools as one value, so they travel and are stored together.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ResourceSet {
    /// Health, as a percentage of its current maximum.
    pub health: ResourceLevel,
    /// Stamina, as a percentage of its current maximum.
    pub stamina: ResourceLevel,
    /// Magicka, as a percentage of its current maximum.
    pub magicka: ResourceLevel,
}

impl ResourceSet {
    /// A set with every resource unreadable. This is both the starting value and
    /// the value every failure mode produces.
    pub fn new_unknown() -> Self {
        Self::default()
    }
}

/// The green marker identifying a health sample.
const HEALTH_MARKER: u8 = 0x16;
/// The green marker identifying a stamina sample.
const STAMINA_MARKER: u8 = 0x6D;
/// The green marker identifying a magicka sample.
const MAGICKA_MARKER: u8 = 0xBB;
/// The payload the addon publishes when a resource maximum is zero or unreadable.
/// It passes the marker and checksum checks and fails the range check, so it needs
/// no special case in the decoder and can never be confused with a real value.
pub const RESOURCE_UNAVAILABLE: u8 = 0xFF;
/// The highest publishable percentage.
const RESOURCE_MAX_PERCENT: u8 = 100;

/// Every green-channel value that appears at the center of a beacon block, with
/// the block it belongs to.
///
/// This is the registry a new block's marker is chosen against. Adding a block
/// means adding its green here; `tests/pixelbus.rs` asserts every pair is
/// separated by more than the default tolerance, so a colliding marker fails the
/// build and names the collision instead of silently decoding as its neighbour
/// when the strip geometry is off by a block.
pub const BLOCK_CENTER_GREENS: [(&str, u8); 10] = [
    ("B0 status", 0x00),
    ("B1 fishing waiting", 0x80),
    ("B1 fishing bite", 0xFF),
    ("B2 latency marker", 0xA5),
    ("B3 weapon marker", WEAPON_MARKER),
    ("B4 combat marker", COMBAT_MARKER),
    ("B5 menu marker", MENU_MARKER),
    ("B6 health marker", HEALTH_MARKER),
    ("B7 stamina marker", STAMINA_MARKER),
    ("B8 magicka marker", MAGICKA_MARKER),
];

/// A typed event decoded from the pixel bus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelBusEvent {
    /// The status block is present.
    Heartbeat,
    /// The status block has been absent past the timeout.
    SignalLost,
    /// A cast became active and waiting.
    FishingStarted,
    /// A bite was detected.
    BiteDetected,
    /// Fishing stopped (block absent).
    FishingStopped,
    /// A decoded server latency in milliseconds.
    Latency(u16),
    /// A change in the active weapon bar or a bar's weapon class.
    WeaponBar(WeaponBarSignal),
    /// A change in the decoded combat state, including a change to
    /// [`CombatSignal::Unknown`] when the block stops decoding.
    Combat(CombatSignal),
    /// A change in the decoded menu surface, including a change to
    /// [`MenuSurface::None`] when the block stops decoding. While the carried
    /// surface gates, the application starts no new weave and no new fishing
    /// interaction.
    MenuGate(MenuSurface),
    /// A change in any decoded resource level. Carries all three, so a sample in
    /// which several move at once (the common case in combat) is one event rather
    /// than three.
    Resources(ResourceSet),
}

/// The raw samples taken from one strip read, one field per block.
///
/// `None` means the surface could not be sampled at that point, which is distinct
/// from a sample that was taken but does not decode. The derived [`Default`] is
/// what makes this extensible: a construction using `..Default::default()` keeps
/// compiling when a later feature adds a block, so adding one costs a field
/// rather than a rewrite of every call site. Named fields also make it impossible
/// to transpose two blocks, which the previous four positional arguments of the
/// same type allowed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BlockSamples {
    /// B0, the status block.
    pub status: Option<Rgb>,
    /// B1, the fishing block.
    pub fishing: Option<Rgb>,
    /// B2, the latency block.
    pub latency: Option<Rgb>,
    /// B3, the weapon-bar block.
    pub weapon: Option<Rgb>,
    /// B4, the combat block.
    pub combat: Option<Rgb>,
    /// B5, the menu-state block.
    pub menu: Option<Rgb>,
    /// B6, the health block.
    pub health: Option<Rgb>,
    /// B7, the stamina block.
    pub stamina: Option<Rgb>,
    /// B8, the magicka block.
    pub magicka: Option<Rgb>,
}

/// The surface sampling seam: reads one client-area pixel.
pub trait SurfaceSampler {
    /// Captures a fresh frame if the backend needs to. The reader calls this once
    /// before sampling the block points, so a backend that captures a whole strip
    /// (the Windows screen-composited capture) does so once per batch rather than
    /// per point. The default is a no-op (the mock and the X11 backend read points
    /// directly).
    fn prepare(&self) {}

    /// The color at a client-area point, or `None` when the surface cannot be
    /// sampled.
    fn sample(&self, x: u32, y: u32) -> Option<Rgb>;

    /// The measured display for this sampler's window, or `None` when it cannot
    /// be resolved.
    ///
    /// This is the out-of-band half of the pixel bus: it answers how big the
    /// render surface is without reading a single pixel of it, which is what a
    /// future grid layout needs before it can know where any block is. It shares
    /// this seam rather than getting its own because the boundary is the same
    /// one (here is where the operating system starts) and both backends already
    /// hold exactly the handle it needs.
    ///
    /// The default returns no measurement, so an implementation opts in rather
    /// than being broken by the addition. An implementation that supplies it
    /// MUST return `None` rather than a zero or partial surface, MUST report
    /// physical pixels, and MUST NOT block: it is called once per sampling
    /// iteration on the pixel bus worker thread.
    fn display(&self) -> Option<display::MeasuredDisplay> {
        None
    }
}

/// Extracts one pixel from a captured 32-bit BGRA strip (the byte layout
/// `GetDIBits` fills for a top-down `BI_RGB` bitmap): at offset
/// `(y * width + x) * 4` the bytes are blue, green, red, alpha. Returns `None`
/// when the point is out of range or the buffer is too short. Pure and tested, so
/// the only decodable part of the screen-capture path is covered while the OS
/// calls stay in the thin backend.
pub fn strip_pixel(buffer: &[u8], width: u32, height: u32, x: u32, y: u32) -> Option<Rgb> {
    if x >= width || y >= height {
        return None;
    }
    let offset = ((y as usize) * (width as usize) + (x as usize)) * 4;
    let b = *buffer.get(offset)?;
    let g = *buffer.get(offset + 1)?;
    let r = *buffer.get(offset + 2)?;
    Some(Rgb::new(r, g, b))
}

/// A test sampler that returns crafted colors for specific points.
#[derive(Debug, Default)]
pub struct MockSampler {
    points: HashMap<(u32, u32), Rgb>,
    display: Option<display::MeasuredDisplay>,
}

impl MockSampler {
    /// Creates an empty mock sampler (every point returns `None`).
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the color returned for a point.
    pub fn set(&mut self, x: u32, y: u32, color: Rgb) {
        self.points.insert((x, y), color);
    }

    /// Clears the color for a point (it will return `None`).
    pub fn clear(&mut self, x: u32, y: u32) {
        self.points.remove(&(x, y));
    }

    /// Sets the measurement [`SurfaceSampler::display`] returns, so the display
    /// detection can be exercised with no window and no display hardware.
    pub fn set_display(&mut self, measured: Option<display::MeasuredDisplay>) {
        self.display = measured;
    }
}

impl SurfaceSampler for MockSampler {
    fn sample(&self, x: u32, y: u32) -> Option<Rgb> {
        self.points.get(&(x, y)).copied()
    }

    fn display(&self) -> Option<display::MeasuredDisplay> {
        self.display
    }
}

/// The number of beacon blocks on the bus: B0 status, B1 fishing, B2 latency,
/// B3 weapon, B4 combat, B5 menu, B6 health, B7 stamina, B8 magicka.
///
/// This is the companion's single statement of the strip length; the drawn width
/// and the capture region both derive from it. The addon states the same number
/// once, as `local NUM_BLOCKS` in `PixelBeacon.lua`, and `tests/beacon.rs`
/// asserts the two agree by parsing the embedded addon source. Adding a block is
/// a matter of raising this value, adding a sample point, and adding a field to
/// [`BlockSamples`].
pub const NUM_BLOCKS: u32 = 9;
/// The default block edge length in physical pixels (the historical value; a
/// fresh or unchanged install behaves exactly as before).
pub const DEFAULT_BLOCK_PX: u32 = 16;
/// The smallest supported block edge length in physical pixels.
pub const MIN_BLOCK_PX: u32 = 2;
/// The largest supported block edge length in physical pixels.
pub const MAX_BLOCK_PX: u32 = 32;

/// The sampled center of block `index`, derived solely from `block_px`. This is
/// the single geometry contract shared byte for byte with the PixelBeacon addon,
/// which draws block `index` spanning `[block_px * index, block_px * index +
/// block_px]` and fills it with its center color. `block_px` is even, so the
/// center is always a whole pixel.
pub fn block_center(block_px: u32, index: u32) -> (u32, u32) {
    (block_px * index + block_px / 2, block_px / 2)
}

/// The screen-capture region for the whole strip, derived from `block_px`: wide
/// enough for all `NUM_BLOCKS` blocks and one block tall.
pub fn capture_dims(block_px: u32) -> (u32, u32) {
    (block_px * NUM_BLOCKS, block_px)
}

/// Corrects a block size to the supported set (even, `MIN_BLOCK_PX..=MAX_BLOCK_PX`),
/// recording a non-fatal notice when the value is changed. An odd value rounds
/// down to the next even, a below-range value clamps to the minimum, an
/// above-range value clamps to the maximum. Never panics.
pub fn sanitize_block_px(value: u32, notices: &mut Vec<Notice>) -> u32 {
    // Round an odd value down to the next even so the sampled center stays a
    // whole pixel, then clamp into the supported range.
    let corrected = (value & !1).clamp(MIN_BLOCK_PX, MAX_BLOCK_PX);
    if corrected != value {
        notices.push(Notice {
            kind: NoticeKind::InvalidValue,
            message: format!(
                "pixelbus block_px {value} is not a supported size; using {corrected}"
            ),
        });
    }
    corrected
}

/// Reader configuration. Beacon geometry derives entirely from `block_px`: the
/// four block-center read points and the screen-capture region are computed, not
/// stored, so the reader and the addon can never disagree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReaderConfig {
    /// Per-channel color match tolerance.
    pub tolerance: u8,
    /// Absence past this after the last heartbeat raises signal loss.
    pub heartbeat_timeout_ms: u64,
    /// The single source of truth for block geometry: the physical-pixel edge
    /// length of each square block. Even, `MIN_BLOCK_PX..=MAX_BLOCK_PX`.
    pub block_px: u32,
    /// Sampling interval while fishing is enabled.
    pub interval_fishing_ms: u64,
    /// Sampling interval otherwise.
    pub interval_idle_ms: u64,
}

impl ReaderConfig {
    /// The status block (B0) sample point, derived from `block_px`.
    pub fn status_point(&self) -> (u32, u32) {
        block_center(self.block_px, 0)
    }
    /// The fishing block (B1) sample point, derived from `block_px`.
    pub fn fishing_point(&self) -> (u32, u32) {
        block_center(self.block_px, 1)
    }
    /// The latency block (B2) sample point, derived from `block_px`.
    pub fn latency_point(&self) -> (u32, u32) {
        block_center(self.block_px, 2)
    }
    /// The weapon-bar block (B3) sample point, derived from `block_px`.
    pub fn weapon_point(&self) -> (u32, u32) {
        block_center(self.block_px, 3)
    }
    /// The combat block (B4) sample point, derived from `block_px`.
    pub fn combat_point(&self) -> (u32, u32) {
        block_center(self.block_px, 4)
    }
    /// The menu block (B5) sample point, derived from `block_px`.
    pub fn menu_point(&self) -> (u32, u32) {
        block_center(self.block_px, 5)
    }
    /// The health block (B6) sample point, derived from `block_px`.
    pub fn health_point(&self) -> (u32, u32) {
        block_center(self.block_px, 6)
    }
    /// The stamina block (B7) sample point, derived from `block_px`.
    pub fn stamina_point(&self) -> (u32, u32) {
        block_center(self.block_px, 7)
    }
    /// The magicka block (B8) sample point, derived from `block_px`.
    pub fn magicka_point(&self) -> (u32, u32) {
        block_center(self.block_px, 8)
    }
}

impl Default for ReaderConfig {
    fn default() -> Self {
        Self {
            tolerance: 2,
            heartbeat_timeout_ms: 2000,
            block_px: DEFAULT_BLOCK_PX,
            interval_fishing_ms: 100,
            interval_idle_ms: 1000,
        }
    }
}

/// Selects the worker poll interval from whether a fishing session is active and
/// whether the application is in a position to intercept input.
///
/// While fishing is active the reader must sample, and the fishing state machine
/// must tick, at the fast cadence so transient cast and bite signals are observed
/// in time and the reel is not delayed.
///
/// The fast cadence also applies whenever the application can intercept, because
/// the menu gate is only useful if it engages before the operator has typed half a
/// sentence. Note what this deliberately is not: an unconditional fast cadence.
/// That would leave `interval_idle_ms` with no effect at all, which is the mirror
/// image of the defect where `interval_fishing_ms` was the dead setting and every
/// fishing session sampled once a second. A suspended application intercepts
/// nothing and synthesizes nothing, so it has no gate to keep current and can
/// sample slowly; both settings keep a real meaning and the extra capture cost is
/// paid only while the application is actually working.
pub fn poll_interval(fishing_active: bool, can_intercept: bool, cfg: &ReaderConfig) -> u64 {
    if fishing_active || can_intercept {
        cfg.interval_fishing_ms
    } else {
        cfg.interval_idle_ms
    }
}

/// The minimum accepted sampling interval, in milliseconds.
const MIN_INTERVAL_MS: u64 = 1;
/// The maximum accepted sampling interval, in milliseconds.
const MAX_INTERVAL_MS: u64 = 60_000;

#[derive(Deserialize, Default)]
struct RawPixelBus {
    #[serde(default)]
    tolerance: Option<u8>,
    #[serde(default)]
    block_px: Option<u32>,
    #[serde(default)]
    interval_fishing_ms: Option<u64>,
    #[serde(default)]
    interval_idle_ms: Option<u64>,
}

/// Loads the user-editable reader configuration (tolerance, block size, and
/// sampling intervals) from the opaque `pixelbus` settings value onto a default
/// [`ReaderConfig`]. The heartbeat timeout is not a user setting and keeps its
/// default. Null or absent yields defaults; an out-of-range interval or an
/// unsupported block size falls back with a notice. An absent `block_px` (an
/// older config) defaults to [`DEFAULT_BLOCK_PX`], so existing installs are
/// unchanged.
pub fn load_reader_config(value: &serde_json::Value, notices: &mut Vec<Notice>) -> ReaderConfig {
    let defaults = ReaderConfig::default();
    if value.is_null() {
        return defaults;
    }
    let raw: RawPixelBus = serde_json::from_value(value.clone()).unwrap_or_default();
    ReaderConfig {
        tolerance: raw.tolerance.unwrap_or(defaults.tolerance),
        block_px: raw
            .block_px
            .map(|v| sanitize_block_px(v, notices))
            .unwrap_or(defaults.block_px),
        interval_fishing_ms: checked_interval(
            raw.interval_fishing_ms,
            defaults.interval_fishing_ms,
            "interval_fishing_ms",
            notices,
        ),
        interval_idle_ms: checked_interval(
            raw.interval_idle_ms,
            defaults.interval_idle_ms,
            "interval_idle_ms",
            notices,
        ),
        ..defaults
    }
}

/// Serializes the user-editable reader configuration to the opaque `pixelbus`
/// settings value.
pub fn store_reader_config(config: &ReaderConfig) -> serde_json::Value {
    serde_json::json!({
        "tolerance": config.tolerance,
        "block_px": config.block_px,
        "interval_fishing_ms": config.interval_fishing_ms,
        "interval_idle_ms": config.interval_idle_ms,
    })
}

fn checked_interval(
    value: Option<u64>,
    default: u64,
    name: &str,
    notices: &mut Vec<Notice>,
) -> u64 {
    match value {
        None => default,
        Some(ms) if (MIN_INTERVAL_MS..=MAX_INTERVAL_MS).contains(&ms) => ms,
        Some(_) => {
            notices.push(Notice {
                kind: NoticeKind::InvalidValue,
                message: format!("pixelbus {name} is out of range; using default {default}"),
            });
            default
        }
    }
}

fn within(a: u8, b: u8, tolerance: u8) -> bool {
    a.abs_diff(b) <= tolerance
}

/// Whether a sample matches the status block magenta within tolerance.
pub fn status_present(sample: Rgb, tolerance: u8) -> bool {
    within(sample.r, 0xFF, tolerance)
        && within(sample.g, 0x00, tolerance)
        && within(sample.b, 0xFF, tolerance)
}

/// Decodes the fishing signal from a sample.
pub fn fishing_signal(sample: Rgb, tolerance: u8) -> FishingSignal {
    if within(sample.r, 0x00, tolerance)
        && within(sample.g, 0x80, tolerance)
        && within(sample.b, 0xFF, tolerance)
    {
        FishingSignal::Waiting
    } else if within(sample.r, 0x00, tolerance)
        && within(sample.g, 0xFF, tolerance)
        && within(sample.b, 0x00, tolerance)
    {
        FishingSignal::Bite
    } else {
        FishingSignal::None
    }
}

/// Decodes latency from the latency block, or `None` when the marker or checksum
/// fails validation. The value is the red channel times four.
pub fn decode_latency(sample: Rgb, tolerance: u8) -> Option<u16> {
    let checksum = u16::from(sample.r) + u16::from(sample.b);
    if within(sample.g, 0xA5, tolerance) && checksum.abs_diff(255) <= u16::from(tolerance) {
        Some(u16::from(sample.r) * 4)
    } else {
        None
    }
}

/// Decodes the weapon-bar block, or `None` when the marker fails validation. The
/// red channel packs the front weapon class in its high nibble and the back class
/// in its low nibble; the blue channel is the active-bar code.
pub fn decode_weapon_bar(sample: Rgb, tolerance: u8) -> Option<WeaponBarSignal> {
    if !within(sample.g, WEAPON_MARKER, tolerance) {
        return None;
    }
    Some(WeaponBarSignal {
        bar: ActiveBar::from_code(sample.b),
        front: WeaponClass::from_code(sample.r >> 4),
        back: WeaponClass::from_code(sample.r & 0x0F),
    })
}

/// Decodes the combat block into its tri-state.
///
/// Validation follows the latency block's marker-and-checksum pattern rather than
/// the weapon block's exact code match: the green marker and the `red + blue`
/// complement are both checked within tolerance, then the red channel selects the
/// state. Any failure yields [`CombatSignal::Unknown`]; there is no nearest match
/// and no default to out of combat, so an addon that draws no combat block (or
/// any unrelated screen content behind that point) can never be read as a state.
pub fn decode_combat(sample: Rgb, tolerance: u8) -> CombatSignal {
    let checksum = u16::from(sample.r) + u16::from(sample.b);
    if !within(sample.g, COMBAT_MARKER, tolerance) || checksum.abs_diff(255) > u16::from(tolerance)
    {
        return CombatSignal::Unknown;
    }
    if within(sample.r, COMBAT_IN_RED, tolerance) {
        CombatSignal::InCombat
    } else if within(sample.r, COMBAT_OUT_RED, tolerance) {
        CombatSignal::OutOfCombat
    } else {
        CombatSignal::Unknown
    }
}

/// Decodes the menu block into its surface.
///
/// Validation mirrors the combat block: the green marker and the `red + blue`
/// complement checksum are both checked within tolerance, then the red channel
/// selects the surface code. Any failure yields [`MenuSurface::None`], which is
/// the safe value, because it means the application behaves exactly as it does
/// without the menu gate.
pub fn decode_menu(sample: Rgb, tolerance: u8) -> MenuSurface {
    let checksum = u16::from(sample.r) + u16::from(sample.b);
    if !within(sample.g, MENU_MARKER, tolerance) || checksum.abs_diff(255) > u16::from(tolerance) {
        return MenuSurface::None;
    }
    // Codes are spaced MENU_CODE_STEP apart, so the nearest code is found by
    // rounding and then confirming the sample really is within tolerance of it
    // rather than sitting between two codes.
    let step = u16::from(MENU_CODE_STEP);
    let code = (u16::from(sample.r) + step / 2) / step;
    let Ok(code) = u8::try_from(code) else {
        return MenuSurface::None;
    };
    if code > MENU_CODE_MAX {
        return MenuSurface::None;
    }
    let expected = code.saturating_mul(MENU_CODE_STEP);
    if !within(sample.r, expected, tolerance) {
        return MenuSurface::None;
    }
    MenuSurface::from_code(code).unwrap_or(MenuSurface::None)
}

/// Decodes one resource block against its marker.
///
/// Validation follows the latency and combat blocks: the marker and the
/// `red + blue` complement checksum are both checked within tolerance, then the
/// red channel carries the percentage directly rather than indexing a colour
/// table. That choice is what bounds the error: a capture that shifts the payload
/// by one reads as one percent off, where a lookup table would land on whichever
/// entry happened to be nearest in colour space and could be wrong by any amount.
///
/// Payloads slightly above [`RESOURCE_MAX_PERCENT`], within tolerance, clamp to it
/// rather than being rejected. A full resource is the ordinary out-of-combat
/// state, so rejecting it on any upward drift would make the most common value the
/// least stable reading on the strip.
pub fn decode_resource(sample: Rgb, marker: u8, tolerance: u8) -> ResourceLevel {
    let checksum = u16::from(sample.r) + u16::from(sample.b);
    if !within(sample.g, marker, tolerance) || checksum.abs_diff(255) > u16::from(tolerance) {
        return ResourceLevel::Unknown;
    }
    if sample.r > RESOURCE_MAX_PERCENT.saturating_add(tolerance) {
        return ResourceLevel::Unknown;
    }
    ResourceLevel::Percent(sample.r.min(RESOURCE_MAX_PERCENT))
}

/// Decodes all three resource blocks. Each is independent: a sample that fails
/// validation yields [`ResourceLevel::Unknown`] for that resource only.
pub fn decode_resources(
    health: Option<Rgb>,
    stamina: Option<Rgb>,
    magicka: Option<Rgb>,
    tolerance: u8,
) -> ResourceSet {
    ResourceSet {
        health: health.map_or(ResourceLevel::Unknown, |c| {
            decode_resource(c, HEALTH_MARKER, tolerance)
        }),
        stamina: stamina.map_or(ResourceLevel::Unknown, |c| {
            decode_resource(c, STAMINA_MARKER, tolerance)
        }),
        magicka: magicka.map_or(ResourceLevel::Unknown, |c| {
            decode_resource(c, MAGICKA_MARKER, tolerance)
        }),
    }
}

/// The pixel bus reader state machine.
pub struct PixelBusReader {
    config: ReaderConfig,
    last_heartbeat_ms: Option<u64>,
    signal_lost: bool,
    fishing: FishingSignal,
    weapon: Option<WeaponBarSignal>,
    combat: CombatSignal,
    menu: MenuSurface,
    resources: ResourceSet,
    had_heartbeat: bool,
}

impl PixelBusReader {
    /// Creates a reader with the given configuration.
    pub fn new(config: ReaderConfig) -> Self {
        Self {
            config,
            last_heartbeat_ms: None,
            signal_lost: false,
            fishing: FishingSignal::None,
            weapon: None,
            combat: CombatSignal::Unknown,
            menu: MenuSurface::None,
            resources: ResourceSet::new_unknown(),
            had_heartbeat: false,
        }
    }

    /// Whether the signal is currently lost.
    pub fn signal_lost(&self) -> bool {
        self.signal_lost
    }

    /// Observes one set of block samples at `now_ms` and returns the resulting
    /// events.
    pub fn observe(&mut self, samples: BlockSamples, now_ms: u64) -> Vec<PixelBusEvent> {
        let BlockSamples {
            status: b0,
            fishing: b1,
            latency: b2,
            weapon: b3,
            combat: b4,
            menu: b5,
            health: b6,
            stamina: b7,
            magicka: b8,
        } = samples;
        let mut events = Vec::new();
        let tolerance = self.config.tolerance;
        let heartbeat = b0.is_some_and(|c| status_present(c, tolerance));

        // Raw per-sample diagnostic (TRACE only, never at the default level). This
        // is the in-game signature the operator reads to tell a present heartbeat
        // apart from no heartbeat at all (the strip is not being read), and, once a
        // heartbeat is present, whether the fishing block ever decodes to Waiting.
        let fishing_decoded = b1.map_or(FishingSignal::None, |c| fishing_signal(c, tolerance));
        let heartbeat_age_ms = self
            .last_heartbeat_ms
            .map(|last| now_ms.saturating_sub(last));
        tracing::trace!(
            target: "eso_weave::pixelbus",
            heartbeat,
            ?fishing_decoded,
            ?heartbeat_age_ms,
            ?b0,
            ?b1,
            ?b2,
            ?b3,
            ?b4,
            ?b5,
            now_ms,
            "pixel bus sample"
        );

        // A clear DEBUG signature on the heartbeat acquire and lose transitions, so
        // the operator can tell at a glance whether B0 (the addon signal) is being
        // read at all, without reading every TRACE sample. This changes no emitted
        // event.
        if heartbeat && !self.had_heartbeat {
            tracing::debug!(
                target: "eso_weave::pixelbus",
                ?b0,
                "pixel bus heartbeat acquired"
            );
        } else if !heartbeat && self.had_heartbeat {
            tracing::debug!(
                target: "eso_weave::pixelbus",
                "pixel bus heartbeat lost"
            );
        }
        self.had_heartbeat = heartbeat;

        if heartbeat {
            self.last_heartbeat_ms = Some(now_ms);
            self.signal_lost = false;
            events.push(PixelBusEvent::Heartbeat);

            let signal = b1.map_or(FishingSignal::None, |c| fishing_signal(c, tolerance));
            if signal != self.fishing {
                match signal {
                    FishingSignal::Waiting => events.push(PixelBusEvent::FishingStarted),
                    FishingSignal::Bite => events.push(PixelBusEvent::BiteDetected),
                    FishingSignal::None => events.push(PixelBusEvent::FishingStopped),
                }
                self.fishing = signal;
            }

            if let Some(latency) = b2.and_then(|c| decode_latency(c, tolerance)) {
                events.push(PixelBusEvent::Latency(latency));
            }

            // The weapon-bar block is optional (older addons omit it). Emit only on
            // a change in the decoded signal, so per-attack redraws never churn.
            //
            // Note the deliberate difference from the combat block below: a weapon
            // sample that does not decode leaves the last good value in place, and
            // only signal loss clears it. Combat state does not hold. See the
            // clarification recorded in specs/031-combat-state-block/spec.md.
            let weapon = b3.and_then(|c| decode_weapon_bar(c, tolerance));
            if let Some(signal) = weapon {
                if self.weapon != Some(signal) {
                    self.weapon = Some(signal);
                    tracing::debug!(
                        target: "eso_weave::pixelbus",
                        ?signal,
                        "weapon bar detected"
                    );
                    events.push(PixelBusEvent::WeaponBar(signal));
                }
            }

            // The combat block is optional in the same sense, but a sample that
            // does not decode clears the state to Unknown rather than holding it.
            // Holding would let a stale "in combat" survive an addon downgrade or a
            // mid-session reload, which is exactly the false reading the tri-state
            // exists to prevent.
            let combat = b4.map_or(CombatSignal::Unknown, |c| decode_combat(c, tolerance));
            if combat != self.combat {
                self.combat = combat;
                tracing::debug!(
                    target: "eso_weave::pixelbus",
                    signal = ?combat,
                    "combat state detected"
                );
                events.push(PixelBusEvent::Combat(combat));
            }

            // The menu block gates input, so a sample that does not decode must
            // clear it rather than hold it: holding a stale gate would leave the
            // application silently not intercepting long after the menu closed,
            // which looks exactly like a crash.
            let menu = b5.map_or(MenuSurface::None, |c| decode_menu(c, tolerance));
            if menu != self.menu {
                self.menu = menu;
                tracing::debug!(
                    target: "eso_weave::pixelbus",
                    surface = ?menu,
                    gates = menu.gates(),
                    "menu surface changed"
                );
                events.push(PixelBusEvent::MenuGate(menu));
            }

            // Resources clear on a non-decoding sample like the two blocks above.
            // Logged at TRACE, not DEBUG: these change many times a second in
            // combat, and at DEBUG they would push every other line out of the
            // live log, which is the tool used to diagnose everything else.
            let resources = decode_resources(b6, b7, b8, tolerance);
            if resources != self.resources {
                self.resources = resources;
                tracing::trace!(
                    target: "eso_weave::pixelbus",
                    ?resources,
                    "resources changed"
                );
                events.push(PixelBusEvent::Resources(resources));
            }
        } else if let Some(last) = self.last_heartbeat_ms {
            if !self.signal_lost && now_ms.saturating_sub(last) > self.config.heartbeat_timeout_ms {
                self.signal_lost = true;
                self.fishing = FishingSignal::None;
                if self.weapon.is_some() {
                    tracing::debug!(
                        target: "eso_weave::pixelbus",
                        "weapon bar cleared (signal lost)"
                    );
                }
                self.weapon = None;
                events.push(PixelBusEvent::SignalLost);
                if self.combat != CombatSignal::Unknown {
                    self.combat = CombatSignal::Unknown;
                    events.push(PixelBusEvent::Combat(CombatSignal::Unknown));
                }
                // Losing the signal must open the gate, never close it.
                if self.menu != MenuSurface::None {
                    self.menu = MenuSurface::None;
                    events.push(PixelBusEvent::MenuGate(MenuSurface::None));
                }
                let cleared = ResourceSet::new_unknown();
                if self.resources != cleared {
                    self.resources = cleared;
                    events.push(PixelBusEvent::Resources(cleared));
                }
            }
        }

        events
    }

    /// Samples every configured block point and observes them (the runtime path).
    pub fn sample_and_observe(
        &mut self,
        sampler: &dyn SurfaceSampler,
        now_ms: u64,
    ) -> Vec<PixelBusEvent> {
        // Let the backend capture a fresh frame once, before the point reads.
        sampler.prepare();
        let (sx, sy) = self.config.status_point();
        let (fx, fy) = self.config.fishing_point();
        let (lx, ly) = self.config.latency_point();
        let (wx, wy) = self.config.weapon_point();
        let (cx, cy) = self.config.combat_point();
        let (mx, my) = self.config.menu_point();
        let (hx, hy) = self.config.health_point();
        let (tx, ty) = self.config.stamina_point();
        let (gx, gy) = self.config.magicka_point();
        let samples = BlockSamples {
            status: sampler.sample(sx, sy),
            fishing: sampler.sample(fx, fy),
            latency: sampler.sample(lx, ly),
            weapon: sampler.sample(wx, wy),
            combat: sampler.sample(cx, cy),
            menu: sampler.sample(mx, my),
            health: sampler.sample(hx, hy),
            stamina: sampler.sample(tx, ty),
            magicka: sampler.sample(gx, gy),
        };
        self.observe(samples, now_ms)
    }
}
