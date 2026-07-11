//! Weave engine data types: weave sequences, skill slots, and timing.

use crate::input::{Key, MouseButton, Transition};

/// A weave type, which determines the operation sequence and relevant delays.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaveType {
    /// Primary click, wait `d_weave`, send skill key.
    LightAttack,
    /// Primary down, wait `d_heavy`, send skill key, primary up.
    HeavyAttack,
    /// Primary click, wait `d_weave`, skill key, wait `d_bash`, secondary down,
    /// primary click, secondary up.
    BashAttack,
    /// Secondary down, skill key, wait `d_weave`, secondary up.
    BlockCasting,
}

impl WeaveType {
    /// The canonical settings string for this weave type.
    pub fn as_str(self) -> &'static str {
        match self {
            WeaveType::LightAttack => "light_attack",
            WeaveType::HeavyAttack => "heavy_attack",
            WeaveType::BashAttack => "bash_attack",
            WeaveType::BlockCasting => "block_casting",
        }
    }

    /// Parses a weave type string, returning `None` for an unknown value.
    pub fn parse(value: &str) -> Option<WeaveType> {
        match value {
            "light_attack" => Some(WeaveType::LightAttack),
            "heavy_attack" => Some(WeaveType::HeavyAttack),
            "bash_attack" => Some(WeaveType::BashAttack),
            "block_casting" => Some(WeaveType::BlockCasting),
            _ => None,
        }
    }
}

/// A single synthesized operation in a weave sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputOp {
    /// A key transition (the slot's bound key).
    Key(Key, Transition),
    /// A mouse button transition.
    Mouse(MouseButton, Transition),
}

/// One step of a weave sequence: emit an operation or wait a duration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaveStep {
    /// Perform a synthesized operation.
    Emit(InputOp),
    /// Wait the given number of milliseconds.
    Wait(u32),
}

/// Global timing values, all in milliseconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimingConfig {
    /// Minimum interval between weave executions.
    pub global_cooldown: u32,
    /// Base gap between the basic attack and the skill key.
    pub d_weave: u32,
    /// Heavy attack hold before the skill key.
    pub d_heavy: u32,
    /// Gap before the bash action in a bash attack.
    pub d_bash: u32,
}

impl Default for TimingConfig {
    fn default() -> Self {
        Self {
            global_cooldown: 500,
            d_weave: 50,
            d_heavy: 1000,
            d_bash: 125,
        }
    }
}

/// Per-slot delay overrides. A `None` means use the global default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SlotOverrides {
    /// Override for `d_weave`.
    pub d_weave: Option<u32>,
    /// Override for `d_heavy`.
    pub d_heavy: Option<u32>,
    /// Override for `d_bash`.
    pub d_bash: Option<u32>,
}

/// One of the seven skill slots.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkillSlot {
    /// The slot index, 1 through 7 (6 is Ultimate, 7 is Synergy).
    pub index: u8,
    /// The slot's bound key.
    pub key: Key,
    /// The weave type to run.
    pub weave_type: WeaveType,
    /// Whether the slot is active. An inactive slot's key passes through.
    pub active: bool,
    /// Per-slot delay overrides.
    pub overrides: SlotOverrides,
}

impl SkillSlot {
    /// The effective `d_weave` for this slot (override or global default).
    pub fn d_weave(&self, timing: &TimingConfig) -> u32 {
        self.overrides.d_weave.unwrap_or(timing.d_weave)
    }

    /// The effective `d_heavy` for this slot.
    pub fn d_heavy(&self, timing: &TimingConfig) -> u32 {
        self.overrides.d_heavy.unwrap_or(timing.d_heavy)
    }

    /// The effective `d_bash` for this slot.
    pub fn d_bash(&self, timing: &TimingConfig) -> u32 {
        self.overrides.d_bash.unwrap_or(timing.d_bash)
    }
}
