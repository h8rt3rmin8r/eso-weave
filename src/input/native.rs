//! Portable, read-only ESO action-binding evidence.
//!
//! These types describe what PixelBeacon observed and the bounded combat plans
//! admitted from that evidence. ESO bindings remain read-only throughout.

use std::ops::{BitOr, BitOrAssign};

/// Native ESO actions published in fixed pixel-bus order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NativeAction {
    Skill1,
    Skill2,
    Skill3,
    Skill4,
    Skill5,
    Ultimate,
    Synergy,
    Attack,
    Block,
    Interact,
    Quickslot,
}

impl NativeAction {
    pub const COUNT: usize = 11;
    pub const ALL: [Self; Self::COUNT] = [
        Self::Skill1,
        Self::Skill2,
        Self::Skill3,
        Self::Skill4,
        Self::Skill5,
        Self::Ultimate,
        Self::Synergy,
        Self::Attack,
        Self::Block,
        Self::Interact,
        Self::Quickslot,
    ];

    /// Stable zero-based action position in the binding payload.
    pub const fn index(self) -> usize {
        match self {
            Self::Skill1 => 0,
            Self::Skill2 => 1,
            Self::Skill3 => 2,
            Self::Skill4 => 3,
            Self::Skill5 => 4,
            Self::Ultimate => 5,
            Self::Synergy => 6,
            Self::Attack => 7,
            Self::Block => 8,
            Self::Interact => 9,
            Self::Quickslot => 10,
        }
    }

    /// ESO's invariant action name used by read-only discovery.
    pub const fn eso_name(self) -> &'static str {
        match self {
            Self::Skill1 => "ACTION_BUTTON_3",
            Self::Skill2 => "ACTION_BUTTON_4",
            Self::Skill3 => "ACTION_BUTTON_5",
            Self::Skill4 => "ACTION_BUTTON_6",
            Self::Skill5 => "ACTION_BUTTON_7",
            Self::Ultimate => "ACTION_BUTTON_8",
            Self::Synergy => "USE_SYNERGY",
            Self::Attack => "SPECIAL_MOVE_ATTACK",
            Self::Block => "SPECIAL_MOVE_BLOCK",
            Self::Interact => "GAME_CAMERA_INTERACT",
            Self::Quickslot => "ACTION_BUTTON_9",
        }
    }

    /// The corresponding application combat action, when this native action is
    /// one of the seven weave triggers.
    pub const fn combat_action(self) -> Option<super::Action> {
        match self {
            Self::Skill1 => Some(super::Action::Skill1),
            Self::Skill2 => Some(super::Action::Skill2),
            Self::Skill3 => Some(super::Action::Skill3),
            Self::Skill4 => Some(super::Action::Skill4),
            Self::Skill5 => Some(super::Action::Skill5),
            Self::Ultimate => Some(super::Action::Ultimate),
            Self::Synergy => Some(super::Action::Synergy),
            Self::Attack | Self::Block | Self::Interact | Self::Quickslot => None,
        }
    }
}

/// Ordinary keyboard primaries supported by the native binding contract.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum KeyboardControl {
    Digit0 = 16,
    Digit1 = 17,
    Digit2 = 18,
    Digit3 = 19,
    Digit4 = 20,
    Digit5 = 21,
    Digit6 = 22,
    Digit7 = 23,
    Digit8 = 24,
    Digit9 = 25,
    A = 26,
    B = 27,
    C = 28,
    D = 29,
    E = 30,
    F = 31,
    G = 32,
    H = 33,
    I = 34,
    J = 35,
    K = 36,
    L = 37,
    M = 38,
    N = 39,
    O = 40,
    P = 41,
    Q = 42,
    R = 43,
    S = 44,
    T = 45,
    U = 46,
    V = 47,
    W = 48,
    X = 49,
    Y = 50,
    Z = 51,
    F1 = 52,
    F2 = 53,
    F3 = 54,
    F4 = 55,
    F5 = 56,
    F6 = 57,
    F7 = 58,
    F8 = 59,
    F9 = 60,
    F10 = 61,
    F11 = 62,
    F12 = 63,
    F13 = 64,
    F14 = 65,
    F15 = 66,
    F16 = 67,
    F17 = 68,
    F18 = 69,
    F19 = 70,
    F20 = 71,
    F21 = 72,
    F22 = 73,
    F23 = 74,
    F24 = 75,
    Backspace = 76,
    CapsLock = 77,
    Delete = 78,
    DownArrow = 79,
    End = 80,
    Enter = 81,
    Escape = 82,
    Home = 83,
    Insert = 84,
    LeftArrow = 85,
    NumLock = 86,
    Numpad0 = 87,
    Numpad1 = 88,
    Numpad2 = 89,
    Numpad3 = 90,
    Numpad4 = 91,
    Numpad5 = 92,
    Numpad6 = 93,
    Numpad7 = 94,
    Numpad8 = 95,
    Numpad9 = 96,
    NumpadAdd = 97,
    NumpadDot = 98,
    NumpadEnter = 99,
    NumpadMinus = 100,
    NumpadSlash = 101,
    NumpadStar = 102,
    PageDown = 103,
    PageUp = 104,
    Pause = 105,
    PrintScreen = 106,
    RightArrow = 107,
    ScrollLock = 108,
    Space = 109,
    Tab = 110,
    UpArrow = 111,
    Oem102GermanLessThan = 112,
    Oem1Semicolon = 113,
    Oem2ForwardSlash = 114,
    Oem3Tick = 115,
    Oem4LeftSquareBracket = 116,
    Oem5BackSlash = 117,
    Oem6RightSquareBracket = 118,
    Oem7SingleQuote = 119,
    Oem8BackTick = 120,
    OemComma = 121,
    OemMinus = 122,
    OemPeriod = 123,
    OemPlus = 124,
    LeftWindows = 125,
    RightWindows = 126,
}

impl KeyboardControl {
    pub const ALL: [Self; 111] = [
        Self::Digit0,
        Self::Digit1,
        Self::Digit2,
        Self::Digit3,
        Self::Digit4,
        Self::Digit5,
        Self::Digit6,
        Self::Digit7,
        Self::Digit8,
        Self::Digit9,
        Self::A,
        Self::B,
        Self::C,
        Self::D,
        Self::E,
        Self::F,
        Self::G,
        Self::H,
        Self::I,
        Self::J,
        Self::K,
        Self::L,
        Self::M,
        Self::N,
        Self::O,
        Self::P,
        Self::Q,
        Self::R,
        Self::S,
        Self::T,
        Self::U,
        Self::V,
        Self::W,
        Self::X,
        Self::Y,
        Self::Z,
        Self::F1,
        Self::F2,
        Self::F3,
        Self::F4,
        Self::F5,
        Self::F6,
        Self::F7,
        Self::F8,
        Self::F9,
        Self::F10,
        Self::F11,
        Self::F12,
        Self::F13,
        Self::F14,
        Self::F15,
        Self::F16,
        Self::F17,
        Self::F18,
        Self::F19,
        Self::F20,
        Self::F21,
        Self::F22,
        Self::F23,
        Self::F24,
        Self::Backspace,
        Self::CapsLock,
        Self::Delete,
        Self::DownArrow,
        Self::End,
        Self::Enter,
        Self::Escape,
        Self::Home,
        Self::Insert,
        Self::LeftArrow,
        Self::NumLock,
        Self::Numpad0,
        Self::Numpad1,
        Self::Numpad2,
        Self::Numpad3,
        Self::Numpad4,
        Self::Numpad5,
        Self::Numpad6,
        Self::Numpad7,
        Self::Numpad8,
        Self::Numpad9,
        Self::NumpadAdd,
        Self::NumpadDot,
        Self::NumpadEnter,
        Self::NumpadMinus,
        Self::NumpadSlash,
        Self::NumpadStar,
        Self::PageDown,
        Self::PageUp,
        Self::Pause,
        Self::PrintScreen,
        Self::RightArrow,
        Self::ScrollLock,
        Self::Space,
        Self::Tab,
        Self::UpArrow,
        Self::Oem102GermanLessThan,
        Self::Oem1Semicolon,
        Self::Oem2ForwardSlash,
        Self::Oem3Tick,
        Self::Oem4LeftSquareBracket,
        Self::Oem5BackSlash,
        Self::Oem6RightSquareBracket,
        Self::Oem7SingleQuote,
        Self::Oem8BackTick,
        Self::OemComma,
        Self::OemMinus,
        Self::OemPeriod,
        Self::OemPlus,
        Self::LeftWindows,
        Self::RightWindows,
    ];

    const fn from_wire_code(code: u8) -> Option<Self> {
        let mut index = 0;
        while index < Self::ALL.len() {
            let key = Self::ALL[index];
            if key as u8 == code {
                return Some(key);
            }
            index += 1;
        }
        None
    }
}

/// Mouse primaries supported by the native binding contract.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MouseControl {
    Left = 200,
    Right = 201,
    Middle = 202,
    Button4 = 203,
    Button5 = 204,
    WheelUp = 205,
    WheelDown = 206,
}

impl MouseControl {
    pub const ALL: [Self; 7] = [
        Self::Left,
        Self::Right,
        Self::Middle,
        Self::Button4,
        Self::Button5,
        Self::WheelUp,
        Self::WheelDown,
    ];

    const fn from_wire_code(code: u8) -> Option<Self> {
        let mut index = 0;
        while index < Self::ALL.len() {
            let button = Self::ALL[index];
            if button as u8 == code {
                return Some(button);
            }
            index += 1;
        }
        None
    }
}

/// A platform-neutral keyboard or mouse primary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NativeControl {
    Keyboard(KeyboardControl),
    Mouse(MouseControl),
}

impl NativeControl {
    pub const fn wire_code(self) -> u8 {
        match self {
            Self::Keyboard(key) => key as u8,
            Self::Mouse(button) => button as u8,
        }
    }

    pub const fn from_wire_code(code: u8) -> Option<Self> {
        match KeyboardControl::from_wire_code(code) {
            Some(key) => Some(Self::Keyboard(key)),
            None => match MouseControl::from_wire_code(code) {
                Some(button) => Some(Self::Mouse(button)),
                None => None,
            },
        }
    }

    /// Wheel directions are relative pulses and have no held release state.
    pub const fn is_momentary(self) -> bool {
        matches!(
            self,
            Self::Mouse(MouseControl::WheelUp | MouseControl::WheelDown)
        )
    }
}

/// Normalized native modifier bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ModifierSet(u8);

impl ModifierSet {
    pub const EMPTY: Self = Self(0);
    pub const SHIFT: Self = Self(1 << 0);
    pub const CONTROL: Self = Self(1 << 1);
    pub const ALT: Self = Self(1 << 2);
    pub const COMMAND: Self = Self(1 << 3);

    pub const fn from_bits(bits: u8) -> Option<Self> {
        if bits <= 0x0F {
            Some(Self(bits))
        } else {
            None
        }
    }

    pub const fn bits(self) -> u8 {
        self.0
    }

    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Whether every modifier in this set is also present in `other`.
    pub const fn is_subset_of(self, other: Self) -> bool {
        other.contains(self)
    }

    /// Returns this set with one modifier included.
    pub const fn with(self, modifier: NativeModifier) -> Self {
        Self(self.0 | modifier.flag().0)
    }

    /// Returns this set with one modifier removed.
    pub const fn without(self, modifier: NativeModifier) -> Self {
        Self(self.0 & !modifier.flag().0)
    }
}

impl BitOr for ModifierSet {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for ModifierSet {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// A normalized physical or generated modifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NativeModifier {
    Control,
    Alt,
    Shift,
    Command,
}

impl NativeModifier {
    /// Canonical generated press order. Cleanup uses the reverse order.
    pub const ORDERED: [Self; 4] = [Self::Control, Self::Alt, Self::Shift, Self::Command];

    pub const fn flag(self) -> ModifierSet {
        match self {
            Self::Shift => ModifierSet::SHIFT,
            Self::Control => ModifierSet::CONTROL,
            Self::Alt => ModifierSet::ALT,
            Self::Command => ModifierSet::COMMAND,
        }
    }
}

/// One supported primary plus its normalized native modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NativeChord {
    pub primary: NativeControl,
    pub modifiers: ModifierSet,
}

/// One physical input identity seen by the platform interception layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NativeInput {
    Primary(NativeControl),
    Modifier(NativeModifier),
}

/// The native controls copied into one admitted combat request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CombatChordPlan {
    pub skill: NativeChord,
    pub attack: Option<NativeChord>,
    pub block: Option<NativeChord>,
}

impl CombatChordPlan {
    /// Whether every target chord can execute without releasing a physically
    /// held modifier.
    pub const fn admits_physical(self, physical: ModifierSet) -> bool {
        if !physical.is_subset_of(self.skill.modifiers) {
            return false;
        }
        if let Some(attack) = self.attack {
            if !physical.is_subset_of(attack.modifiers) {
                return false;
            }
        }
        if let Some(block) = self.block {
            if !physical.is_subset_of(block.modifiers) {
                return false;
            }
        }
        true
    }
}

/// The controls a configured weave type needs from one binding snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CombatRequirement {
    pub attack: bool,
    pub block: bool,
}

impl CombatRequirement {
    pub const LIGHT_OR_HEAVY: Self = Self {
        attack: true,
        block: false,
    };
    pub const BASH: Self = Self {
        attack: true,
        block: true,
    };
    pub const BLOCK_CAST: Self = Self {
        attack: false,
        block: true,
    };
}

/// The bounded result of native discovery or desktop evidence decoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NativeBindingState {
    Unavailable,
    Unbound,
    Conflicting,
    Unsupported,
    Valid(NativeChord),
}

/// One coherent fixed-order snapshot of every required native action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NativeBindingSet {
    states: [NativeBindingState; NativeAction::COUNT],
}

impl NativeBindingSet {
    pub const fn new_unavailable() -> Self {
        Self {
            states: [NativeBindingState::Unavailable; NativeAction::COUNT],
        }
    }

    pub const fn get(self, action: NativeAction) -> NativeBindingState {
        self.states[action.index()]
    }

    pub fn set(&mut self, action: NativeAction, state: NativeBindingState) {
        self.states[action.index()] = state;
    }

    pub const fn as_array(self) -> [NativeBindingState; NativeAction::COUNT] {
        self.states
    }
}

impl Default for NativeBindingSet {
    fn default() -> Self {
        Self::new_unavailable()
    }
}
