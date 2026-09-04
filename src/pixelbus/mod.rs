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
    grid_fit, parse_user_settings, reconcile, DisplayDescriptor, DisplayDetector, DisplaySource,
    DisplayUpdate, GridFit, GridFitWatch, MeasuredDisplay, Point, Reconciliation, Size, StoredPair,
    StoredVideoSettings,
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
/// [`MenuSurface::None`] means gameplay: a valid block authoritatively reports
/// that no surface is active. Missing or invalid evidence is represented by an
/// outer `Option` at the decoder and event boundaries.
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

/// The player's movement mode, decoded from the movement block.
///
/// The sprint axis issue #11 proposed is absent on purpose. The game exposes no
/// sprint observable to an addon: `IsUnitSprinting`, `IsPlayerSprinting`, and
/// `EVENT_SPRINT_STATE_CHANGED` do not exist, and the only `Sprint` references in
/// the interface source are a keybind action, a hold-versus-toggle preference,
/// and a `sprintf`. The verification is recorded in
/// `specs/036-movement-state-block/spec.md`. The wire encoding reserves two codes
/// for it, so adding the axis later adds variants here without changing the
/// meaning or the colour of either existing one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MovementSignal {
    /// The movement state could not be read.
    #[default]
    Unknown,
    /// The player is not mounted.
    OnFoot,
    /// The player is mounted.
    Mounted,
}

/// The green marker that identifies a movement sample.
///
/// Chosen as the midpoint of the widest gap left in [`BLOCK_CENTER_GREENS`]
/// (`0x2D` to `0x5A`), which puts it 22 from its nearest neighbour, eleven times
/// the default tolerance. `0xE8`, the midpoint of the equally wide `0xD2` to
/// `0xFF` gap, ties on separation and lost the tiebreak: unrelated screen content
/// behind the overlay clusters at the channel extremes, and `0xE8` sits 23 from
/// `0xFF` where this sits 67 from `0x00` and 188 from `0xFF`.
///
/// The nibble-swap convention (`0xA5`/`0x5A`, then `0x2D`/`0xD2`) is deliberately
/// not continued: it was a mnemonic for picking a second value near a first, the
/// resource markers already abandoned it, and `0x34` would sit 7 from `0x2D`.
const MOVEMENT_MARKER: u8 = 0x43;
/// The red channel encoding the on-foot state. Code `0b00`.
const MOVEMENT_ON_FOOT_RED: u8 = 0x20;
/// The red channel encoding the mounted state. Code `0b01`.
const MOVEMENT_MOUNTED_RED: u8 = 0x60;
/// Reserved for the deferred sprint axis, on foot. Code `0b10`.
///
/// Never emitted by the addon, which defines no constant for it, and decoded as
/// [`MovementSignal::Unknown`]. Bit 1 is the sprint bit, so a future feature that
/// gains a sprint observable emits this and [`MOVEMENT_SPRINT_MOUNTED_RED`]
/// without renumbering, recolouring, or reserving anything further.
const MOVEMENT_SPRINT_ON_FOOT_RED: u8 = 0xA0;
/// Reserved for the deferred sprint axis, mounted. Code `0b11`.
const MOVEMENT_SPRINT_MOUNTED_RED: u8 = 0xE0;

/// One skill slot's cooldown, decoded from its block.
///
/// [`SlotCooldown::Ready`] is a distinct variant rather than a zero duration so
/// that "usable now" cannot be confused with "a duration that rounds to zero",
/// and so a consumer can match on readiness without a comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SlotCooldown {
    /// The cooldown could not be read, or the game reports none for this slot.
    #[default]
    Unknown,
    /// The slot is off cooldown.
    Ready,
    /// Milliseconds remaining, quantized to [`COOLDOWN_STEP_MS`] steps and
    /// saturating at `COOLDOWN_STEP_MS * COOLDOWN_MAX_STEPS`.
    RemainingMs(u16),
}

/// The six action slots the game exposes a cooldown for, as one value, so they
/// travel and are stored together.
///
/// Synergy has no field. It is a contextual prompt rather than an action slot,
/// so the game reports no cooldown for it in any state; see
/// `specs/037-cooldown-blocks/research.md` R1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CooldownSet {
    /// Skill slot 1.
    pub skill_1: SlotCooldown,
    /// Skill slot 2.
    pub skill_2: SlotCooldown,
    /// Skill slot 3.
    pub skill_3: SlotCooldown,
    /// Skill slot 4.
    pub skill_4: SlotCooldown,
    /// Skill slot 5.
    pub skill_5: SlotCooldown,
    /// The ultimate slot.
    pub ultimate: SlotCooldown,
}

impl CooldownSet {
    /// A set in which every slot is unknown.
    pub fn new_unknown() -> Self {
        Self::default()
    }
}

/// The green markers identifying each cooldown sample.
///
/// Chosen as the midpoints of the six widest gaps left in
/// [`BLOCK_CENTER_GREENS`], which puts the minimum separation across the whole
/// registry at 11, five and a half times the default tolerance. That is tighter
/// than a single added mark can achieve, and it is the honest price of adding six
/// at once: with seventeen values in a 256-wide channel and the incumbents fixed,
/// 11 is the best achievable minimum.
///
/// Six distinct marks rather than one shared mark, because six adjacent squares
/// carrying the same kind of value are exactly where a geometry error off by one
/// block would otherwise decode a neighbour's cooldown as this slot's, silently
/// and plausibly.
const COOLDOWN_SKILL1_MARKER: u8 = 0x0B;
/// The green marker identifying the skill 2 cooldown sample.
const COOLDOWN_SKILL2_MARKER: u8 = 0x21;
/// The green marker identifying the skill 3 cooldown sample.
const COOLDOWN_SKILL3_MARKER: u8 = 0x4E;
/// The green marker identifying the skill 4 cooldown sample.
const COOLDOWN_SKILL4_MARKER: u8 = 0x92;
/// The green marker identifying the skill 5 cooldown sample.
const COOLDOWN_SKILL5_MARKER: u8 = 0xC6;
/// The green marker identifying the ultimate cooldown sample.
const COOLDOWN_ULTIMATE_MARKER: u8 = 0xE8;
/// Milliseconds per encoded step.
const COOLDOWN_STEP_MS: u16 = 50;
/// The largest encodable step count. A longer cooldown saturates here rather than
/// wrapping, so it reads as "at least this long" instead of as a small number.
const COOLDOWN_MAX_STEPS: u8 = 254;
/// The payload the addon publishes when the game reports no cooldown. Like
/// [`RESOURCE_UNAVAILABLE`] it passes the marker and checksum checks and fails
/// the range check, so it needs no special case in the decoder.
const COOLDOWN_UNAVAILABLE: u8 = 255;

/// Why a selected quickslot observation cannot be trusted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuickslotUnavailableReason {
    /// No fresh beacon signal exists.
    NoSignal,
    /// The addon publishes the legacy four-block contract without B20.
    LegacyProtocol,
    /// B20 is present but fails its marker, checksum, or code validation.
    CorruptProtocol,
    /// A required ESO API primitive or constant is absent.
    UnsupportedApi,
    /// ESO did not provide a valid selected slot.
    InvalidSelection,
    /// The independently sampled slot facts contradict one another.
    InconsistentFacts,
}

/// The bounded kind of a selected quickslot entry that is not a potion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuickslotNonPotionKind {
    Item,
    Collectible,
    QuestItem,
    Emote,
    QuickChat,
    Other,
}

/// Whether a positively classified potion can be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuickslotPotionAvailability {
    /// The selected potion's stack is empty.
    Depleted,
    /// The stack is positive but ESO reports that the slot is not usable.
    Blocked,
    /// The stack is positive and ESO reports that the slot is usable.
    Usable,
}

/// The explicit classification carried by B20.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuickslotClassification {
    Unavailable(QuickslotUnavailableReason),
    Empty,
    NonPotion(QuickslotNonPotionKind),
    Potion(QuickslotPotionAvailability),
}

/// The active quickslot, decoded from its five blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuickslotState {
    /// The sole authority for what the selected slot represents.
    pub classification: QuickslotClassification,
    /// How long the quickslot has left before it can be used.
    pub cooldown: SlotCooldown,
    /// The potion identity when all three identity blocks decode.
    pub item_id: Option<u32>,
}

impl QuickslotState {
    /// The initial state and the cleared state: nothing known.
    pub fn new_unknown() -> Self {
        Self {
            classification: QuickslotClassification::Unavailable(
                QuickslotUnavailableReason::NoSignal,
            ),
            cooldown: SlotCooldown::Unknown,
            item_id: None,
        }
    }

    /// Whether the active quickslot explicitly holds a potion.
    pub fn is_potion(&self) -> bool {
        matches!(self.classification, QuickslotClassification::Potion(_))
    }

    /// Whether ESO explicitly reports a positive-stack usable potion.
    pub fn is_usable_potion(&self) -> bool {
        matches!(
            self.classification,
            QuickslotClassification::Potion(QuickslotPotionAvailability::Usable)
        )
    }

    /// Whether the observation itself satisfies the explicit quickslot half of
    /// the future automation contract. Runtime activation remains disabled in
    /// S042; issue #25 adopts this predicate end to end.
    pub fn authorizes_auto_potion(&self) -> bool {
        self.is_usable_potion()
    }
}

/// The green marker identifying the quickslot cooldown sample (B16).
///
/// Chosen, like every mark since slice 031, as the midpoint of one of the widest
/// gaps left in [`BLOCK_CENTER_GREENS`]. See
/// `specs/038-quickslot-blocks/research.md` R5: the four marks this slice adds
/// take the four widest remaining gaps and leave the minimum adjacent separation
/// across the whole registry unchanged at 11, which is more than five times the
/// default tolerance.
const QUICKSLOT_MARKER: u8 = 0x38;
/// The green marker identifying the quickslot item identity's high byte (B17).
const QUICKSLOT_ID_HI_MARKER: u8 = 0xB0;
/// The green marker identifying the quickslot item identity's middle byte (B18).
const QUICKSLOT_ID_MID_MARKER: u8 = 0xDD;
/// The green marker identifying the quickslot item identity's low byte (B19).
const QUICKSLOT_ID_LO_MARKER: u8 = 0xF3;
/// The green marker identifying the explicit quickslot classification (B20).
const QUICKSLOT_STATE_MARKER: u8 = 0x76;

const QUICKSLOT_UNAVAILABLE_API: u8 = 0x10;
const QUICKSLOT_INVALID_SELECTION: u8 = 0x20;
const QUICKSLOT_INCONSISTENT: u8 = 0x30;
const QUICKSLOT_EMPTY: u8 = 0x40;
const QUICKSLOT_NON_POTION_ITEM: u8 = 0x50;
const QUICKSLOT_NON_POTION_COLLECTIBLE: u8 = 0x60;
const QUICKSLOT_NON_POTION_QUEST_ITEM: u8 = 0x70;
const QUICKSLOT_NON_POTION_EMOTE: u8 = 0x80;
const QUICKSLOT_NON_POTION_QUICK_CHAT: u8 = 0x90;
const QUICKSLOT_NON_POTION_OTHER: u8 = 0xA0;
const QUICKSLOT_POTION_DEPLETED: u8 = 0xB0;
const QUICKSLOT_POTION_BLOCKED: u8 = 0xC0;
const QUICKSLOT_POTION_USABLE: u8 = 0xD0;

/// Every green-channel value that appears at the center of a beacon block, with
/// the block it belongs to.
///
/// This is the registry a new block's marker is chosen against. Adding a block
/// means adding its green here; `tests/pixelbus.rs` asserts every pair is
/// separated by more than the default tolerance, so a colliding marker fails the
/// build and names the collision instead of silently decoding as its neighbour
/// when the strip geometry is off by a block.
pub const BLOCK_CENTER_GREENS: [(&str, u8); 22] = [
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
    ("B9 movement marker", MOVEMENT_MARKER),
    ("B10 cooldown skill 1 marker", COOLDOWN_SKILL1_MARKER),
    ("B11 cooldown skill 2 marker", COOLDOWN_SKILL2_MARKER),
    ("B12 cooldown skill 3 marker", COOLDOWN_SKILL3_MARKER),
    ("B13 cooldown skill 4 marker", COOLDOWN_SKILL4_MARKER),
    ("B14 cooldown skill 5 marker", COOLDOWN_SKILL5_MARKER),
    ("B15 cooldown ultimate marker", COOLDOWN_ULTIMATE_MARKER),
    ("B16 quickslot marker", QUICKSLOT_MARKER),
    ("B17 quickslot id high marker", QUICKSLOT_ID_HI_MARKER),
    ("B18 quickslot id middle marker", QUICKSLOT_ID_MID_MARKER),
    ("B19 quickslot id low marker", QUICKSLOT_ID_LO_MARKER),
    ("B20 quickslot state marker", QUICKSLOT_STATE_MARKER),
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
    /// A change in the decoded menu observation. `Some(MenuSurface::None)` is a
    /// valid gameplay observation; `None` means the block is unavailable.
    MenuGate(Option<MenuSurface>),
    /// A change in any decoded resource level. Carries all three, so a sample in
    /// which several move at once (the common case in combat) is one event rather
    /// than three.
    Resources(ResourceSet),
    /// A change in the decoded movement state. Nothing acts on it; it is an
    /// observable, like combat state.
    Movement(MovementSignal),
    /// A change in any decoded slot cooldown. Carries all six, so a single weave
    /// that moves several slots is one event rather than six, following the
    /// resource blocks.
    Cooldowns(CooldownSet),
    /// A change in the decoded quickslot. Carries the whole state, because a
    /// swap moves the identity and the cooldown in the same sample and four
    /// events for one swap would be four log entries for one thing happening.
    Quickslot(QuickslotState),
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
    /// B9, the movement block.
    pub movement: Option<Rgb>,
    /// B10, the skill 1 cooldown block.
    pub cooldown_skill_1: Option<Rgb>,
    /// B11, the skill 2 cooldown block.
    pub cooldown_skill_2: Option<Rgb>,
    /// B12, the skill 3 cooldown block.
    pub cooldown_skill_3: Option<Rgb>,
    /// B13, the skill 4 cooldown block.
    pub cooldown_skill_4: Option<Rgb>,
    /// B14, the skill 5 cooldown block.
    pub cooldown_skill_5: Option<Rgb>,
    /// B15, the ultimate cooldown block.
    pub cooldown_ultimate: Option<Rgb>,
    /// B16, the quickslot cooldown block. The first block on row 1.
    pub quickslot_status: Option<Rgb>,
    /// B17, the quickslot item identity's high byte.
    pub quickslot_id_hi: Option<Rgb>,
    /// B18, the quickslot item identity's middle byte.
    pub quickslot_id_mid: Option<Rgb>,
    /// B19, the quickslot item identity's low byte.
    pub quickslot_id_lo: Option<Rgb>,
    /// B20, the explicit quickslot classification.
    pub quickslot_state: Option<Rgb>,
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
/// B3 weapon, B4 combat, B5 menu, B6 health, B7 stamina, B8 magicka,
/// B9 movement, B10 to B15 skill cooldowns, B16 quickslot cooldown, B17 to B19
/// the quickslot item identity, and B20 its explicit classification.
///
/// This is the companion's single statement of the grid length; the drawn extent
/// and the capture region both derive from it. The addon states the same number
/// once, as `local NUM_BLOCKS` in `PixelBeacon.lua`, and `tests/beacon.rs`
/// asserts the two agree by parsing the embedded addon source. Adding a block is
/// a matter of raising this value, adding a sample point, and adding a field to
/// [`BlockSamples`].
///
/// **Twenty-one blocks occupy two rows.** Slice 038 is the first shipping count to
/// cross [`COLUMNS`]: row 0 is full at sixteen and the four original quickslot
/// blocks are the first four positions of row 1; slice 042 adds the fifth.
/// Everything that depends on the shape
/// derives it from this value and [`COLUMNS`] rather than restating it, and
/// `tests/pixelbus.rs` asserts the two-row shape at compile time.
pub const NUM_BLOCKS: u32 = 21;
/// The default block edge length in physical pixels (the historical value; a
/// fresh or unchanged install behaves exactly as before).
pub const DEFAULT_BLOCK_PX: u32 = 16;
/// The smallest supported block edge length in physical pixels.
pub const MIN_BLOCK_PX: u32 = 2;
/// The largest supported block edge length in physical pixels.
pub const MAX_BLOCK_PX: u32 = 32;

/// The number of blocks in one row of the beacon grid.
///
/// Blocks wrap to the next row when a row is full, so the beacon grows downward
/// and its width is bounded forever at `block_px * COLUMNS`. Without this the
/// strip widened by one block per signal and would have run off the side of the
/// client area long before the observables worth publishing were exhausted.
///
/// Like [`NUM_BLOCKS`], this is stated once on each side of the contract (`local
/// COLUMNS` in `PixelBeacon.lua`) and `tests/beacon.rs` asserts the two agree by
/// parsing the embedded addon source.
///
/// It is deliberately **not** derived from the display resolution or the client
/// area on either side, which is what the issue that prompted the out-of-band
/// display detection originally assumed it would be. Two independently obtained
/// measurements would have to yield the identical integer, and a disagreement of
/// one does not degrade: it shifts every block from row 1 onward, so the reader
/// samples real blocks that pass their marker and checksum checks and reports
/// each signal as another signal's value. That error would sit underneath the
/// validation built to catch exactly it. The measured surface is used to check
/// the grid *fits* (see [`display::grid_fit`]), which is a job only a
/// measurement can do.
///
/// The value 16 satisfies two bounds: it is at least [`NUM_BLOCKS`], so no
/// existing block moved when the wrap landed, and one row at [`MAX_BLOCK_PX`] is
/// 512 pixels, half the narrowest client width the game supports.
pub const COLUMNS: u32 = 16;

/// The column and row of block `index` in a grid `columns` wide.
///
/// Takes the column count as a parameter rather than reading [`COLUMNS`] so the
/// wrap's properties can be exercised at other widths without changing what
/// ships.
///
/// `const` so the grid's shape can be asserted at compile time (see the
/// assertions in `tests/pixelbus.rs`) by calling this function rather than by
/// open-coding its arithmetic, which would let the assertion and the function
/// drift into disagreeing.
pub const fn grid_position(index: u32, columns: u32) -> (u32, u32) {
    (index % columns, index / columns)
}

/// The number of rows `count` blocks occupy in a grid `columns` wide. An exact
/// multiple does not gain an empty trailing row.
///
/// `const` for the same reason as [`grid_position`].
pub const fn grid_rows(count: u32, columns: u32) -> u32 {
    count.div_ceil(columns)
}

/// The region `count` blocks occupy, in physical pixels.
///
/// The width is the lesser of the block count and the column count, so a grid
/// using a fraction of one row does not claim a full row's width. That keeps the
/// captured region as small as the blocks in use require, which matters because
/// the capture is a screen blit running at up to 10 Hz.
pub fn grid_extent(block_px: u32, count: u32, columns: u32) -> display::Size {
    display::Size::new(
        block_px * count.min(columns),
        block_px * grid_rows(count, columns),
    )
}

/// The sampled center of block `index`, derived solely from `block_px`. This is
/// the single geometry contract shared byte for byte with the PixelBeacon addon,
/// which draws block `index` at grid position `grid_position(index, COLUMNS)`
/// and fills it with its center color. `block_px` is even, so the center is
/// always a whole pixel on both axes.
///
/// For `index < COLUMNS` this reduces to the pre-wrap strip formula
/// `(block_px * index + block_px / 2, block_px / 2)`, which is why introducing
/// the grid moved no existing block.
pub fn block_center(block_px: u32, index: u32) -> (u32, u32) {
    let (col, row) = grid_position(index, COLUMNS);
    (block_px * col + block_px / 2, block_px * row + block_px / 2)
}

/// The screen-capture region for the whole grid, derived from `block_px`: wide
/// enough for the blocks in use and tall enough for the rows they occupy.
///
/// For `NUM_BLOCKS <= COLUMNS` this reduces to the pre-wrap
/// `(block_px * NUM_BLOCKS, block_px)`.
pub fn capture_dims(block_px: u32) -> (u32, u32) {
    let extent = grid_extent(block_px, NUM_BLOCKS, COLUMNS);
    (extent.width, extent.height)
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
    /// The movement block (B9) sample point, derived from `block_px`.
    pub fn movement_point(&self) -> (u32, u32) {
        block_center(self.block_px, 9)
    }
    /// The skill 1 cooldown block (B10) sample point, derived from `block_px`.
    pub fn cooldown_skill_1_point(&self) -> (u32, u32) {
        block_center(self.block_px, 10)
    }
    /// The skill 2 cooldown block (B11) sample point, derived from `block_px`.
    pub fn cooldown_skill_2_point(&self) -> (u32, u32) {
        block_center(self.block_px, 11)
    }
    /// The skill 3 cooldown block (B12) sample point, derived from `block_px`.
    pub fn cooldown_skill_3_point(&self) -> (u32, u32) {
        block_center(self.block_px, 12)
    }
    /// The skill 4 cooldown block (B13) sample point, derived from `block_px`.
    pub fn cooldown_skill_4_point(&self) -> (u32, u32) {
        block_center(self.block_px, 13)
    }
    /// The skill 5 cooldown block (B14) sample point, derived from `block_px`.
    pub fn cooldown_skill_5_point(&self) -> (u32, u32) {
        block_center(self.block_px, 14)
    }
    /// The ultimate cooldown block (B15) sample point, derived from `block_px`.
    pub fn cooldown_ultimate_point(&self) -> (u32, u32) {
        block_center(self.block_px, 15)
    }
    /// The quickslot cooldown block (B16) sample point, derived from `block_px`.
    ///
    /// The first sample point in the project whose `y` is not `block_px / 2`:
    /// index 16 wraps to row 1. Nothing here says so, which is the point. The
    /// row is [`block_center`]'s business and this is the same one-line
    /// derivation every other point uses.
    pub fn quickslot_status_point(&self) -> (u32, u32) {
        block_center(self.block_px, 16)
    }
    /// The quickslot identity high byte block (B17) sample point.
    pub fn quickslot_id_hi_point(&self) -> (u32, u32) {
        block_center(self.block_px, 17)
    }
    /// The quickslot identity middle byte block (B18) sample point.
    pub fn quickslot_id_mid_point(&self) -> (u32, u32) {
        block_center(self.block_px, 18)
    }
    /// The quickslot identity low byte block (B19) sample point.
    pub fn quickslot_id_lo_point(&self) -> (u32, u32) {
        block_center(self.block_px, 19)
    }
    /// The explicit quickslot classification block (B20) sample point.
    pub fn quickslot_state_point(&self) -> (u32, u32) {
        block_center(self.block_px, 20)
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

/// Decodes the movement block into its tri-state.
///
/// Validation mirrors the combat block exactly: the green marker and the
/// `red + blue` complement are both checked within tolerance, then the red
/// channel selects the state. Any failure yields [`MovementSignal::Unknown`];
/// there is no nearest match and no default to on foot, so an addon older than
/// version 10 (which draws no movement block) and any unrelated screen content
/// behind that point can never be read as a state.
///
/// The two reserved sprint codes are matched explicitly rather than left to the
/// catch-all. The behavior is the same either way, but the explicit arm documents
/// the deferred axis in executable form, and it means a future companion that
/// gains sprint support changes this arm rather than discovering the reservation
/// in a comment somewhere else.
pub fn decode_movement(sample: Rgb, tolerance: u8) -> MovementSignal {
    let checksum = u16::from(sample.r) + u16::from(sample.b);
    if !within(sample.g, MOVEMENT_MARKER, tolerance)
        || checksum.abs_diff(255) > u16::from(tolerance)
    {
        return MovementSignal::Unknown;
    }
    if within(sample.r, MOVEMENT_ON_FOOT_RED, tolerance) {
        MovementSignal::OnFoot
    } else if within(sample.r, MOVEMENT_MOUNTED_RED, tolerance) {
        MovementSignal::Mounted
    } else if within(sample.r, MOVEMENT_SPRINT_ON_FOOT_RED, tolerance)
        || within(sample.r, MOVEMENT_SPRINT_MOUNTED_RED, tolerance)
    {
        // Reserved for the deferred sprint axis. No addon emits these, and until
        // one does they are unavailable rather than a state, so no half-built
        // sprint behavior is ever observable.
        MovementSignal::Unknown
    } else {
        MovementSignal::Unknown
    }
}

/// Decodes the menu block into its surface.
///
/// Validation mirrors the combat block: the green marker and the `red + blue`
/// complement checksum are both checked within tolerance, then the red channel
/// selects the surface code. Any failure yields `None`; a valid wire code zero
/// yields `Some(MenuSurface::None)`.
pub fn decode_menu(sample: Rgb, tolerance: u8) -> Option<MenuSurface> {
    let checksum = u16::from(sample.r) + u16::from(sample.b);
    if !within(sample.g, MENU_MARKER, tolerance) || checksum.abs_diff(255) > u16::from(tolerance) {
        return None;
    }
    // Codes are spaced MENU_CODE_STEP apart, so the nearest code is found by
    // rounding and then confirming the sample really is within tolerance of it
    // rather than sitting between two codes.
    let step = u16::from(MENU_CODE_STEP);
    let code = (u16::from(sample.r) + step / 2) / step;
    let Ok(code) = u8::try_from(code) else {
        return None;
    };
    if code > MENU_CODE_MAX {
        return None;
    }
    let expected = code.saturating_mul(MENU_CODE_STEP);
    if !within(sample.r, expected, tolerance) {
        return None;
    }
    MenuSurface::from_code(code)
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

/// Decodes one cooldown block against its marker.
///
/// Validation follows [`decode_resource`]: the marker and the `red + blue`
/// complement are checked within tolerance, then the red channel is read as a
/// step count. Any failure yields [`SlotCooldown::Unknown`]; there is no nearest
/// match and no default to ready, so an addon older than version 11 (which draws
/// no cooldown blocks) and any unrelated screen content behind those points can
/// never be read as a cooldown.
///
/// The step count saturates at [`COOLDOWN_MAX_STEPS`] rather than wrapping, so a
/// cooldown longer than the encodable range reads as "at least this long" instead
/// of as a small number.
pub fn decode_cooldown(sample: Rgb, marker: u8, tolerance: u8) -> SlotCooldown {
    let checksum = u16::from(sample.r) + u16::from(sample.b);
    if !within(sample.g, marker, tolerance) || checksum.abs_diff(255) > u16::from(tolerance) {
        return SlotCooldown::Unknown;
    }
    match sample.r {
        COOLDOWN_UNAVAILABLE => SlotCooldown::Unknown,
        0 => SlotCooldown::Ready,
        steps => {
            SlotCooldown::RemainingMs(u16::from(steps.min(COOLDOWN_MAX_STEPS)) * COOLDOWN_STEP_MS)
        }
    }
}

/// Names the slots whose cooldown changed between two sets, for the log entry.
///
/// One entry per changed sample naming only what moved, rather than all six every
/// time. Six slots change constantly during combat, so an entry that always
/// listed all of them would bury the change it exists to make visible, and an
/// entry that said only "cooldowns changed" would be useless for confirming the
/// signal in the field.
fn changed_cooldown_slots(previous: CooldownSet, current: CooldownSet) -> String {
    let mut names = Vec::new();
    for (name, before, after) in [
        ("skill1", previous.skill_1, current.skill_1),
        ("skill2", previous.skill_2, current.skill_2),
        ("skill3", previous.skill_3, current.skill_3),
        ("skill4", previous.skill_4, current.skill_4),
        ("skill5", previous.skill_5, current.skill_5),
        ("ultimate", previous.ultimate, current.ultimate),
    ] {
        if before != after {
            names.push(name);
        }
    }
    names.join(",")
}

/// Decodes the six cooldown blocks into one set, each against its own marker.
#[allow(clippy::too_many_arguments)]
pub fn decode_cooldowns(
    skill_1: Option<Rgb>,
    skill_2: Option<Rgb>,
    skill_3: Option<Rgb>,
    skill_4: Option<Rgb>,
    skill_5: Option<Rgb>,
    ultimate: Option<Rgb>,
    tolerance: u8,
) -> CooldownSet {
    let decode = |sample: Option<Rgb>, marker: u8| {
        sample.map_or(SlotCooldown::Unknown, |c| {
            decode_cooldown(c, marker, tolerance)
        })
    };
    CooldownSet {
        skill_1: decode(skill_1, COOLDOWN_SKILL1_MARKER),
        skill_2: decode(skill_2, COOLDOWN_SKILL2_MARKER),
        skill_3: decode(skill_3, COOLDOWN_SKILL3_MARKER),
        skill_4: decode(skill_4, COOLDOWN_SKILL4_MARKER),
        skill_5: decode(skill_5, COOLDOWN_SKILL5_MARKER),
        ultimate: decode(ultimate, COOLDOWN_ULTIMATE_MARKER),
    }
}

/// Decodes one identity byte block: the byte in red, its mark in green, the
/// complement in blue.
///
/// Unlike every other numeric payload on the bus this one has no reserved value,
/// because all 256 byte values are legal identity bytes. Validity therefore rests
/// entirely on the mark and the complement, which is why each of the three blocks
/// carries its own distinct mark rather than sharing one: without that, an
/// off-by-one in the geometry would read the middle byte as the high byte and
/// every check would pass.
fn decode_id_byte(sample: Rgb, marker: u8, tolerance: u8) -> Option<u8> {
    let checksum = u16::from(sample.r) + u16::from(sample.b);
    if !within(sample.g, marker, tolerance) || checksum.abs_diff(255) > u16::from(tolerance) {
        return None;
    }
    Some(sample.r)
}

fn decode_quickslot_classification(sample: Rgb, tolerance: u8) -> Option<QuickslotClassification> {
    let checksum = u16::from(sample.r) + u16::from(sample.b);
    if !within(sample.g, QUICKSLOT_STATE_MARKER, tolerance)
        || checksum.abs_diff(255) > u16::from(tolerance)
    {
        return None;
    }
    let codes = [
        (
            QUICKSLOT_UNAVAILABLE_API,
            QuickslotClassification::Unavailable(QuickslotUnavailableReason::UnsupportedApi),
        ),
        (
            QUICKSLOT_INVALID_SELECTION,
            QuickslotClassification::Unavailable(QuickslotUnavailableReason::InvalidSelection),
        ),
        (
            QUICKSLOT_INCONSISTENT,
            QuickslotClassification::Unavailable(QuickslotUnavailableReason::InconsistentFacts),
        ),
        (QUICKSLOT_EMPTY, QuickslotClassification::Empty),
        (
            QUICKSLOT_NON_POTION_ITEM,
            QuickslotClassification::NonPotion(QuickslotNonPotionKind::Item),
        ),
        (
            QUICKSLOT_NON_POTION_COLLECTIBLE,
            QuickslotClassification::NonPotion(QuickslotNonPotionKind::Collectible),
        ),
        (
            QUICKSLOT_NON_POTION_QUEST_ITEM,
            QuickslotClassification::NonPotion(QuickslotNonPotionKind::QuestItem),
        ),
        (
            QUICKSLOT_NON_POTION_EMOTE,
            QuickslotClassification::NonPotion(QuickslotNonPotionKind::Emote),
        ),
        (
            QUICKSLOT_NON_POTION_QUICK_CHAT,
            QuickslotClassification::NonPotion(QuickslotNonPotionKind::QuickChat),
        ),
        (
            QUICKSLOT_NON_POTION_OTHER,
            QuickslotClassification::NonPotion(QuickslotNonPotionKind::Other),
        ),
        (
            QUICKSLOT_POTION_DEPLETED,
            QuickslotClassification::Potion(QuickslotPotionAvailability::Depleted),
        ),
        (
            QUICKSLOT_POTION_BLOCKED,
            QuickslotClassification::Potion(QuickslotPotionAvailability::Blocked),
        ),
        (
            QUICKSLOT_POTION_USABLE,
            QuickslotClassification::Potion(QuickslotPotionAvailability::Usable),
        ),
    ];
    let mut matches = codes
        .into_iter()
        .filter(|(code, _)| within(sample.r, *code, tolerance));
    let (_, classification) = matches.next()?;
    if matches.next().is_some() {
        return None;
    }
    Some(classification)
}

/// Decodes the five quickslot blocks into one state.
///
/// The cooldown reuses [`decode_cooldown`] against the quickslot mark, so the
/// quantization, the saturation rule, and the ready case are shared with the
/// skill cooldown blocks by construction rather than reimplemented beside them.
///
/// The identity is reported only when there is a potion to identify **and** all
/// three of its blocks decoded. A partial read yields no identity rather than a
/// number assembled from the bytes that happened to survive: two thirds of an
/// identity is a different item, not an approximate one. Classification,
/// cooldown, and identity otherwise degrade independently.
pub fn decode_quickslot(
    status: Option<Rgb>,
    id_hi: Option<Rgb>,
    id_mid: Option<Rgb>,
    id_lo: Option<Rgb>,
    state: Option<Rgb>,
    tolerance: u8,
) -> QuickslotState {
    let cooldown = status.map_or(SlotCooldown::Unknown, |c| {
        decode_cooldown(c, QUICKSLOT_MARKER, tolerance)
    });
    let classification = match state {
        Some(sample) => decode_quickslot_classification(sample, tolerance).unwrap_or(
            QuickslotClassification::Unavailable(QuickslotUnavailableReason::CorruptProtocol),
        ),
        None if status
            .and_then(|sample| decode_id_byte(sample, QUICKSLOT_MARKER, tolerance))
            .is_some() =>
        {
            QuickslotClassification::Unavailable(QuickslotUnavailableReason::LegacyProtocol)
        }
        None => QuickslotClassification::Unavailable(QuickslotUnavailableReason::NoSignal),
    };
    if !matches!(classification, QuickslotClassification::Potion(_)) {
        return QuickslotState {
            classification,
            cooldown,
            item_id: None,
        };
    }
    let byte =
        |sample: Option<Rgb>, marker: u8| sample.and_then(|c| decode_id_byte(c, marker, tolerance));
    let item_id = match (
        byte(id_hi, QUICKSLOT_ID_HI_MARKER),
        byte(id_mid, QUICKSLOT_ID_MID_MARKER),
        byte(id_lo, QUICKSLOT_ID_LO_MARKER),
    ) {
        (Some(hi), Some(mid), Some(lo)) => {
            Some((u32::from(hi) << 16) | (u32::from(mid) << 8) | u32::from(lo))
        }
        _ => None,
    };
    QuickslotState {
        classification,
        cooldown,
        item_id,
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
    menu: Option<MenuSurface>,
    resources: ResourceSet,
    movement: MovementSignal,
    cooldowns: CooldownSet,
    quickslot: QuickslotState,
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
            menu: None,
            resources: ResourceSet::new_unknown(),
            movement: MovementSignal::Unknown,
            cooldowns: CooldownSet::new_unknown(),
            quickslot: QuickslotState::new_unknown(),
            had_heartbeat: false,
        }
    }

    /// Whether the signal is currently lost.
    pub fn signal_lost(&self) -> bool {
        self.signal_lost
    }

    /// Clears all history so a restarted game republishes even unchanged values.
    pub fn reset(&mut self) {
        let config = self.config;
        *self = Self::new(config);
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
            movement: b9,
            cooldown_skill_1: b10,
            cooldown_skill_2: b11,
            cooldown_skill_3: b12,
            cooldown_skill_4: b13,
            cooldown_skill_5: b14,
            cooldown_ultimate: b15,
            quickslot_status: b16,
            quickslot_id_hi: b17,
            quickslot_id_mid: b18,
            quickslot_id_lo: b19,
            quickslot_state: b20,
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

            // The movement block follows the combat block byte for byte, including
            // the clear-on-non-decode behaviour: a stale "mounted" surviving an
            // addon downgrade is the same false reading in a different signal.
            let movement = b9.map_or(MovementSignal::Unknown, |c| decode_movement(c, tolerance));
            if movement != self.movement {
                self.movement = movement;
                tracing::debug!(
                    target: "eso_weave::pixelbus",
                    signal = ?movement,
                    "movement state detected"
                );
                events.push(PixelBusEvent::Movement(movement));
            }

            // The six cooldown blocks travel as one set, following the resource
            // blocks: a single weave can move most of them within one sample, and
            // six events plus six log lines for one action would bury the signal
            // this block exists to make visible.
            let cooldowns = decode_cooldowns(b10, b11, b12, b13, b14, b15, tolerance);
            if cooldowns != self.cooldowns {
                let previous = self.cooldowns;
                self.cooldowns = cooldowns;
                tracing::debug!(
                    target: "eso_weave::pixelbus",
                    changed = %changed_cooldown_slots(previous, cooldowns),
                    "slot cooldowns detected"
                );
                events.push(PixelBusEvent::Cooldowns(cooldowns));
            }

            // The five quickslot blocks travel as one state, following the
            // resource and cooldown blocks. Clearing on a non-decoding sample
            // rather than holding follows the combat block, and matters more
            // here than anywhere before it: the consumer this observable exists
            // for synthesizes a keypress, so a stale "there is a ready potion"
            // surviving an addon downgrade would become a stale action.
            let quickslot = decode_quickslot(b16, b17, b18, b19, b20, tolerance);
            if quickslot != self.quickslot {
                self.quickslot = quickslot;
                tracing::debug!(
                    target: "eso_weave::pixelbus",
                    cooldown = ?quickslot.cooldown,
                    item_id = ?quickslot.item_id,
                    classification = ?quickslot.classification,
                    is_usable_potion = quickslot.is_usable_potion(),
                    "quickslot state detected"
                );
                events.push(PixelBusEvent::Quickslot(quickslot));
            }

            // The menu block gates input, so a sample that does not decode must
            // clear it rather than hold it: holding a stale gate would leave the
            // application silently not intercepting long after the menu closed,
            // which looks exactly like a crash.
            let menu = b5.and_then(|c| decode_menu(c, tolerance));
            if menu != self.menu {
                self.menu = menu;
                tracing::debug!(
                    target: "eso_weave::pixelbus",
                    surface = ?menu,
                    gates = menu.is_some_and(MenuSurface::gates),
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
                if self.menu.is_some() {
                    self.menu = None;
                    events.push(PixelBusEvent::MenuGate(None));
                }
                let cleared = ResourceSet::new_unknown();
                if self.resources != cleared {
                    self.resources = cleared;
                    events.push(PixelBusEvent::Resources(cleared));
                }
                if self.movement != MovementSignal::Unknown {
                    self.movement = MovementSignal::Unknown;
                    events.push(PixelBusEvent::Movement(MovementSignal::Unknown));
                }
                let cleared_cooldowns = CooldownSet::new_unknown();
                if self.cooldowns != cleared_cooldowns {
                    self.cooldowns = cleared_cooldowns;
                    events.push(PixelBusEvent::Cooldowns(cleared_cooldowns));
                }
                let cleared_quickslot = QuickslotState::new_unknown();
                if self.quickslot != cleared_quickslot {
                    self.quickslot = cleared_quickslot;
                    events.push(PixelBusEvent::Quickslot(cleared_quickslot));
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
        let (vx, vy) = self.config.movement_point();
        let (c1x, c1y) = self.config.cooldown_skill_1_point();
        let (c2x, c2y) = self.config.cooldown_skill_2_point();
        let (c3x, c3y) = self.config.cooldown_skill_3_point();
        let (c4x, c4y) = self.config.cooldown_skill_4_point();
        let (c5x, c5y) = self.config.cooldown_skill_5_point();
        let (cux, cuy) = self.config.cooldown_ultimate_point();
        let (qsx, qsy) = self.config.quickslot_status_point();
        let (qhx, qhy) = self.config.quickslot_id_hi_point();
        let (qmx, qmy) = self.config.quickslot_id_mid_point();
        let (qlx, qly) = self.config.quickslot_id_lo_point();
        let (qtx, qty) = self.config.quickslot_state_point();
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
            movement: sampler.sample(vx, vy),
            cooldown_skill_1: sampler.sample(c1x, c1y),
            cooldown_skill_2: sampler.sample(c2x, c2y),
            cooldown_skill_3: sampler.sample(c3x, c3y),
            cooldown_skill_4: sampler.sample(c4x, c4y),
            cooldown_skill_5: sampler.sample(c5x, c5y),
            cooldown_ultimate: sampler.sample(cux, cuy),
            quickslot_status: sampler.sample(qsx, qsy),
            quickslot_id_hi: sampler.sample(qhx, qhy),
            quickslot_id_mid: sampler.sample(qmx, qmy),
            quickslot_id_lo: sampler.sample(qlx, qly),
            quickslot_state: sampler.sample(qtx, qty),
        };
        self.observe(samples, now_ms)
    }
}
