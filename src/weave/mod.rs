//! Weave Engine: maps handed-off actions to skill slots and runs their weave
//! sequences under a global cooldown, through a testable sink seam.
//!
//! The correctness-critical logic is the pure [`sequence::sequence_for`] builder
//! and the cooldown gate in [`WeaveEngine::handle`], both testable with
//! [`MockSink`] and a virtual clock. [`RealSink`] and real waiting are thin.

pub mod sequence;
pub mod types;

use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

use crate::config::{Notice, NoticeKind, Settings};
use crate::input::bindings::BindingTable;
use crate::input::{Action, InputBackend, InputEngine, Key};

pub use sequence::sequence_for;
pub use types::{InputOp, SkillSlot, SlotOverrides, TimingConfig, WeaveStep, WeaveType};

/// The maximum accepted timing value, in milliseconds (a generous upper bound).
const MAX_TIMING_MS: u32 = 60_000;

/// The seven skill slots plus the global timing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeaveConfig {
    /// The seven skill slots (index 1 through 7).
    pub slots: [SkillSlot; 7],
    /// The global timing configuration.
    pub timing: TimingConfig,
}

impl Default for WeaveConfig {
    fn default() -> Self {
        let slot = |index: u8, key: Key, active: bool| SkillSlot {
            index,
            key,
            weave_type: WeaveType::LightAttack,
            active,
            overrides: SlotOverrides::default(),
        };
        WeaveConfig {
            slots: [
                slot(1, Key::Digit1, true),
                slot(2, Key::Digit2, true),
                slot(3, Key::Digit3, true),
                slot(4, Key::Digit4, true),
                slot(5, Key::Digit5, true),
                slot(6, Key::R, false),
                slot(7, Key::X, false),
            ],
            timing: TimingConfig::default(),
        }
    }
}

impl WeaveConfig {
    /// Updates each slot's key from the binding table so slot keys stay
    /// consistent with the S002 bindings section.
    pub fn sync_keys(&mut self, bindings: &BindingTable) {
        for slot in &mut self.slots {
            if let Some(action) = action_for_index(slot.index) {
                slot.key = bindings.key_for(action);
            }
        }
    }
}

/// The execution seam: synthesized operations, waits, and a monotonic clock.
pub trait WeaveSink {
    /// Performs one synthesized operation.
    fn emit(&mut self, op: InputOp);
    /// Waits the given number of milliseconds.
    fn wait(&mut self, ms: u32);
    /// Returns a monotonic millisecond timestamp.
    fn now_ms(&self) -> u64;
}

/// A test sink: records each operation and wait as an ordered [`WeaveStep`] log
/// and advances a virtual clock on `wait`.
#[derive(Debug, Default)]
pub struct MockSink {
    /// The ordered log of emitted operations and waits.
    pub log: Vec<WeaveStep>,
    clock_ms: u64,
}

impl MockSink {
    /// Creates an empty mock sink with the clock at zero.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the virtual clock to a specific millisecond value.
    pub fn set_now(&mut self, ms: u64) {
        self.clock_ms = ms;
    }

    /// Clears the recorded log without changing the clock.
    pub fn clear_log(&mut self) {
        self.log.clear();
    }
}

impl WeaveSink for MockSink {
    fn emit(&mut self, op: InputOp) {
        self.log.push(WeaveStep::Emit(op));
    }

    fn wait(&mut self, ms: u32) {
        self.log.push(WeaveStep::Wait(ms));
        self.clock_ms += u64::from(ms);
    }

    fn now_ms(&self) -> u64 {
        self.clock_ms
    }
}

/// A real sink: drives Input Engine synthesis and sleeps the worker thread.
pub struct RealSink<B> {
    backend: B,
    origin: Instant,
}

impl<B: InputBackend> RealSink<B> {
    /// Creates a real sink over the given input backend.
    pub fn new(backend: B) -> Self {
        Self {
            backend,
            origin: Instant::now(),
        }
    }
}

impl<B: InputBackend> WeaveSink for RealSink<B> {
    fn emit(&mut self, op: InputOp) {
        let result = match op {
            InputOp::Key(key, transition) => self.backend.synthesize(key, transition),
            InputOp::Mouse(button, transition) => self.backend.synthesize_mouse(button, transition),
        };
        if let Err(err) = result {
            tracing::warn!(target: "eso_weave::weave", "synthesis failed: {err}");
        }
    }

    fn wait(&mut self, ms: u32) {
        std::thread::sleep(Duration::from_millis(u64::from(ms)));
    }

    fn now_ms(&self) -> u64 {
        self.origin.elapsed().as_millis() as u64
    }
}

/// The weave engine: maps actions to slots and runs sequences under cooldown.
pub struct WeaveEngine {
    config: WeaveConfig,
    last_weave_ms: Option<u64>,
}

impl WeaveEngine {
    /// Creates an engine with the given configuration.
    pub fn new(config: WeaveConfig) -> Self {
        Self {
            config,
            last_weave_ms: None,
        }
    }

    /// The current configuration.
    pub fn config(&self) -> &WeaveConfig {
        &self.config
    }

    /// The slot bound to an action, if the action is a weave action.
    pub fn slot_for_action(&self, action: Action) -> Option<&SkillSlot> {
        let index = index_for_action(action)?;
        self.config.slots.iter().find(|slot| slot.index == index)
    }

    /// Handles a handed-off action: if it maps to an active slot and the global
    /// cooldown has elapsed, runs the slot's sequence through the sink. A request
    /// inside the cooldown window is dropped without running a sequence.
    pub fn handle<S: WeaveSink>(&mut self, action: Action, sink: &mut S) {
        let Some(slot) = self.slot_for_action(action).copied() else {
            return;
        };
        if !slot.active {
            return;
        }

        let now = sink.now_ms();
        if let Some(last) = self.last_weave_ms {
            if now.saturating_sub(last) < u64::from(self.config.timing.global_cooldown) {
                return;
            }
        }
        self.last_weave_ms = Some(now);

        for step in sequence_for(&slot, &self.config.timing) {
            match step {
                WeaveStep::Emit(op) => sink.emit(op),
                WeaveStep::Wait(ms) => sink.wait(ms),
            }
        }
    }

    /// Sets each weave action's activity in the Input Engine from its slot's
    /// active flag, so inactive slots pass their key through (FR-002).
    pub fn apply_activity(&self, engine: &InputEngine) {
        for slot in &self.config.slots {
            if let Some(action) = action_for_index(slot.index) {
                engine.set_action_active(action, slot.active);
            }
        }
    }

    /// Loads the timing and skills sections from settings, returning fallback
    /// notices for invalid values.
    pub fn load(&mut self, settings: &Settings) -> Vec<Notice> {
        let mut notices = Vec::new();
        self.config.timing = load_timing(&settings.timing, &mut notices);
        load_skills(&settings.skills, &mut self.config.slots, &mut notices);
        notices
    }

    /// Writes the timing and skills sections into settings for persistence.
    pub fn store(&self, settings: &mut Settings) {
        let timing = TimingJson {
            global_cooldown: self.config.timing.global_cooldown,
            d_weave: self.config.timing.d_weave,
            d_heavy: self.config.timing.d_heavy,
            d_bash: self.config.timing.d_bash,
        };
        settings.timing = serde_json::to_value(timing).unwrap_or(serde_json::Value::Null);

        let skills: BTreeMap<String, SkillJson> = self
            .config
            .slots
            .iter()
            .map(|slot| {
                (
                    format!("slot{}", slot.index),
                    SkillJson {
                        weave_type: slot.weave_type.as_str().to_string(),
                        active: slot.active,
                        d_weave: slot.overrides.d_weave,
                        d_heavy: slot.overrides.d_heavy,
                        d_bash: slot.overrides.d_bash,
                    },
                )
            })
            .collect();
        settings.skills = serde_json::to_value(skills).unwrap_or(serde_json::Value::Null);
    }
}

fn action_for_index(index: u8) -> Option<Action> {
    match index {
        1 => Some(Action::Skill1),
        2 => Some(Action::Skill2),
        3 => Some(Action::Skill3),
        4 => Some(Action::Skill4),
        5 => Some(Action::Skill5),
        6 => Some(Action::Ultimate),
        7 => Some(Action::Synergy),
        _ => None,
    }
}

fn index_for_action(action: Action) -> Option<u8> {
    match action {
        Action::Skill1 => Some(1),
        Action::Skill2 => Some(2),
        Action::Skill3 => Some(3),
        Action::Skill4 => Some(4),
        Action::Skill5 => Some(5),
        Action::Ultimate => Some(6),
        Action::Synergy => Some(7),
        Action::ToggleSuspend | Action::ToggleFishing => None,
    }
}

#[derive(Serialize, Deserialize)]
struct TimingJson {
    global_cooldown: u32,
    d_weave: u32,
    d_heavy: u32,
    d_bash: u32,
}

#[derive(Deserialize, Default)]
struct RawTiming {
    global_cooldown: Option<u32>,
    d_weave: Option<u32>,
    d_heavy: Option<u32>,
    d_bash: Option<u32>,
}

#[derive(Serialize, Deserialize)]
struct SkillJson {
    weave_type: String,
    active: bool,
    #[serde(default)]
    d_weave: Option<u32>,
    #[serde(default)]
    d_heavy: Option<u32>,
    #[serde(default)]
    d_bash: Option<u32>,
}

fn load_timing(value: &serde_json::Value, notices: &mut Vec<Notice>) -> TimingConfig {
    if value.is_null() {
        return TimingConfig::default();
    }
    let raw: RawTiming = serde_json::from_value(value.clone()).unwrap_or_default();
    let defaults = TimingConfig::default();
    TimingConfig {
        global_cooldown: checked(
            raw.global_cooldown,
            defaults.global_cooldown,
            "global_cooldown",
            notices,
        ),
        d_weave: checked(raw.d_weave, defaults.d_weave, "d_weave", notices),
        d_heavy: checked(raw.d_heavy, defaults.d_heavy, "d_heavy", notices),
        d_bash: checked(raw.d_bash, defaults.d_bash, "d_bash", notices),
    }
}

fn checked(value: Option<u32>, default: u32, name: &str, notices: &mut Vec<Notice>) -> u32 {
    match value {
        None => default,
        Some(ms) if ms <= MAX_TIMING_MS => ms,
        Some(_) => {
            notices.push(Notice {
                kind: NoticeKind::InvalidValue,
                message: format!("timing {name} is out of range; using default {default}"),
            });
            default
        }
    }
}

fn load_skills(value: &serde_json::Value, slots: &mut [SkillSlot; 7], notices: &mut Vec<Notice>) {
    if value.is_null() {
        return;
    }
    let raw: BTreeMap<String, SkillJson> = match serde_json::from_value(value.clone()) {
        Ok(raw) => raw,
        Err(_) => {
            notices.push(Notice {
                kind: NoticeKind::InvalidValue,
                message: "skills section is malformed; using defaults".to_string(),
            });
            return;
        }
    };

    for slot in slots.iter_mut() {
        let Some(entry) = raw.get(&format!("slot{}", slot.index)) else {
            continue;
        };
        match WeaveType::parse(&entry.weave_type) {
            Some(weave_type) => slot.weave_type = weave_type,
            None => notices.push(Notice {
                kind: NoticeKind::InvalidValue,
                message: format!(
                    "slot{} has unknown weave type {}; using default",
                    slot.index, entry.weave_type
                ),
            }),
        }
        slot.active = entry.active;
        slot.overrides = SlotOverrides {
            d_weave: entry.d_weave,
            d_heavy: entry.d_heavy,
            d_bash: entry.d_bash,
        };
    }
}
