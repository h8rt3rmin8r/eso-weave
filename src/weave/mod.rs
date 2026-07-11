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
use crate::pixelbus::{ActiveBar, WeaponBarSignal, WeaponClass};

pub use sequence::{effective_delay, sequence_for, sequence_for_adapted};
pub use types::{
    InputOp, LatencyConfig, SkillSlot, SlotOverrides, TimingConfig, WeaveStep, WeaveType,
    MAX_LATENCY_BONUS_MS,
};

/// The maximum accepted timing value, in milliseconds (a generous upper bound).
const MAX_TIMING_MS: u32 = 60_000;

/// The seven skill slots plus the per-bar timing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeaveConfig {
    /// The seven skill slots (index 1 through 7).
    pub slots: [SkillSlot; 7],
    /// The front (primary) bar timing profile.
    pub timing: TimingConfig,
    /// The back (backup) bar timing profile.
    pub timing_back: TimingConfig,
    /// When true, each bar's heavy-attack delay follows the detected weapon class.
    pub auto_timing: bool,
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
            timing_back: TimingConfig::default(),
            auto_timing: false,
        }
    }
}

/// The default heavy-attack delay (ms) for a weapon class, or `None` to keep the
/// profile's own value. These are community estimates (see the master
/// specification R1 appendix) pending in-game validation; one-hand-and-shield is a
/// flagged estimate not yet measured.
pub fn heavy_preset(class: WeaponClass) -> Option<u32> {
    match class {
        WeaponClass::DualWield => Some(640),
        WeaponClass::TwoHanded => Some(1050),
        WeaponClass::SwordAndShield => Some(900),
        WeaponClass::Bow => Some(1380),
        WeaponClass::DestructionStaff => Some(1180),
        WeaponClass::RestorationStaff => Some(1360),
        WeaponClass::Unknown => None,
    }
}

/// The effective timing for the active bar: the back profile when the back bar is
/// active, otherwise the front profile (which also covers an unknown bar). When
/// auto timing is on and the class is known, the bar's heavy-attack delay is
/// replaced with the weapon-class preset.
pub fn effective_timing(
    front: &TimingConfig,
    back: &TimingConfig,
    auto_timing: bool,
    active_bar: ActiveBar,
    front_class: WeaponClass,
    back_class: WeaponClass,
) -> TimingConfig {
    let (base, class) = match active_bar {
        ActiveBar::Back => (back, back_class),
        ActiveBar::Front | ActiveBar::Unknown => (front, front_class),
    };
    let mut timing = *base;
    if auto_timing {
        if let Some(d_heavy) = heavy_preset(class) {
            timing.d_heavy = d_heavy;
        }
    }
    timing
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
    latency: LatencyConfig,
    current_latency: Option<u16>,
    active_bar: ActiveBar,
    front_class: WeaponClass,
    back_class: WeaponClass,
}

impl WeaveEngine {
    /// Creates an engine with the given configuration. Latency adaptation is off
    /// and no current latency or weapon-bar state is known until set.
    pub fn new(config: WeaveConfig) -> Self {
        Self {
            config,
            last_weave_ms: None,
            latency: LatencyConfig::default(),
            current_latency: None,
            active_bar: ActiveBar::Unknown,
            front_class: WeaponClass::Unknown,
            back_class: WeaponClass::Unknown,
        }
    }

    /// Records the decoded weapon-bar state (active bar and each bar's weapon
    /// class), which selects the active timing profile and the auto-timing preset.
    pub fn set_weapon_bar(&mut self, signal: WeaponBarSignal) {
        self.active_bar = signal.bar;
        self.front_class = signal.front;
        self.back_class = signal.back;
    }

    /// The currently active weapon bar.
    pub fn active_bar(&self) -> ActiveBar {
        self.active_bar
    }

    /// The detected weapon classes on the front and back bars.
    pub fn weapon_classes(&self) -> (WeaponClass, WeaponClass) {
        (self.front_class, self.back_class)
    }

    /// The effective timing for the currently active bar (profile plus any
    /// weapon-class preset when auto timing is on).
    pub fn current_timing(&self) -> TimingConfig {
        effective_timing(
            &self.config.timing,
            &self.config.timing_back,
            self.config.auto_timing,
            self.active_bar,
            self.front_class,
            self.back_class,
        )
    }

    /// Sets the current server latency, or clears it (for example on signal loss)
    /// when `None`. While the latency is cleared, the base delays are used
    /// unchanged.
    pub fn set_latency(&mut self, latency: Option<u16>) {
        self.current_latency = latency;
    }

    /// The current latency-adaptation configuration.
    pub fn latency_config(&self) -> &LatencyConfig {
        &self.latency
    }

    /// The current known latency, or `None` when unknown.
    pub fn current_latency(&self) -> Option<u16> {
        self.current_latency
    }

    /// Replaces the latency-adaptation configuration.
    pub fn set_latency_config(&mut self, latency: LatencyConfig) {
        self.latency = latency;
    }

    /// The current configuration.
    pub fn config(&self) -> &WeaveConfig {
        &self.config
    }

    /// Mutable access to the configuration, for editing skill slots and timing
    /// from the GUI.
    pub fn config_mut(&mut self) -> &mut WeaveConfig {
        &mut self.config
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
        let timing = self.current_timing();
        if let Some(last) = self.last_weave_ms {
            if now.saturating_sub(last) < u64::from(timing.global_cooldown) {
                return;
            }
        }
        self.last_weave_ms = Some(now);

        for step in sequence_for_adapted(&slot, &timing, self.current_latency, &self.latency) {
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
        let (front, back, auto) = load_timing(&settings.timing, &mut notices);
        self.config.timing = front;
        self.config.timing_back = back;
        self.config.auto_timing = auto;
        load_skills(&settings.skills, &mut self.config.slots, &mut notices);
        self.latency = load_latency(&settings.latency, &mut notices);
        notices
    }

    /// Writes the timing and skills sections into settings for persistence.
    pub fn store(&self, settings: &mut Settings) {
        let timing = TimingJson {
            global_cooldown: self.config.timing.global_cooldown,
            d_weave: self.config.timing.d_weave,
            d_heavy: self.config.timing.d_heavy,
            d_bash: self.config.timing.d_bash,
            back_global_cooldown: Some(self.config.timing_back.global_cooldown),
            back_d_weave: Some(self.config.timing_back.d_weave),
            back_d_heavy: Some(self.config.timing_back.d_heavy),
            back_d_bash: Some(self.config.timing_back.d_bash),
            auto_timing: self.config.auto_timing,
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

        let latency = LatencyJson {
            enabled: self.latency.enabled,
            k: self.latency.k,
        };
        settings.latency = serde_json::to_value(latency).unwrap_or(serde_json::Value::Null);
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
    // The back-bar profile and auto flag are optional so older configs (front
    // only) still load; when absent the back profile mirrors the front and auto is
    // off.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    back_global_cooldown: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    back_d_weave: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    back_d_heavy: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    back_d_bash: Option<u32>,
    #[serde(default)]
    auto_timing: bool,
}

#[derive(Deserialize, Default)]
struct RawTiming {
    global_cooldown: Option<u32>,
    d_weave: Option<u32>,
    d_heavy: Option<u32>,
    d_bash: Option<u32>,
    back_global_cooldown: Option<u32>,
    back_d_weave: Option<u32>,
    back_d_heavy: Option<u32>,
    back_d_bash: Option<u32>,
    #[serde(default)]
    auto_timing: bool,
}

/// The valid inclusive maximum for the latency scale factor `k` (minimum 0.0).
const K_MAX: f64 = 4.0;

fn load_latency(value: &serde_json::Value, notices: &mut Vec<Notice>) -> LatencyConfig {
    if value.is_null() {
        return LatencyConfig::default();
    }
    let raw: RawLatency = serde_json::from_value(value.clone()).unwrap_or_default();
    let defaults = LatencyConfig::default();
    let k = match raw.k {
        None => defaults.k,
        Some(k) if k.is_finite() && (0.0..=K_MAX).contains(&k) => k,
        Some(_) => {
            notices.push(Notice {
                kind: NoticeKind::InvalidValue,
                message: format!(
                    "latency k is not finite or out of range; using default {}",
                    defaults.k
                ),
            });
            defaults.k
        }
    };
    LatencyConfig {
        enabled: raw.enabled.unwrap_or(defaults.enabled),
        k,
    }
}

#[derive(Deserialize, Default)]
struct RawLatency {
    enabled: Option<bool>,
    k: Option<f64>,
}

#[derive(Serialize, Deserialize)]
struct LatencyJson {
    enabled: bool,
    k: f64,
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

/// Loads the front and back timing profiles and the auto-timing flag. The back
/// profile mirrors the front when its fields are absent (older configs), so the
/// section stays back-compatible.
fn load_timing(
    value: &serde_json::Value,
    notices: &mut Vec<Notice>,
) -> (TimingConfig, TimingConfig, bool) {
    if value.is_null() {
        let d = TimingConfig::default();
        return (d, d, false);
    }
    let raw: RawTiming = serde_json::from_value(value.clone()).unwrap_or_default();
    let defaults = TimingConfig::default();
    let front = TimingConfig {
        global_cooldown: checked(
            raw.global_cooldown,
            defaults.global_cooldown,
            "global_cooldown",
            notices,
        ),
        d_weave: checked(raw.d_weave, defaults.d_weave, "d_weave", notices),
        d_heavy: checked(raw.d_heavy, defaults.d_heavy, "d_heavy", notices),
        d_bash: checked(raw.d_bash, defaults.d_bash, "d_bash", notices),
    };
    let back = TimingConfig {
        global_cooldown: checked(
            raw.back_global_cooldown,
            front.global_cooldown,
            "back_global_cooldown",
            notices,
        ),
        d_weave: checked(raw.back_d_weave, front.d_weave, "back_d_weave", notices),
        d_heavy: checked(raw.back_d_heavy, front.d_heavy, "back_d_heavy", notices),
        d_bash: checked(raw.back_d_bash, front.d_bash, "back_d_bash", notices),
    };
    (front, back, raw.auto_timing)
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
