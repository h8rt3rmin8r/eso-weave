//! Auto-Potion: the first consumer that acts on a beacon-derived value.
//!
//! Every beacon signal before this one is deliberately inert, and a test for each
//! asserts the engine behaves identically for every value it can take. This module
//! is the first thing to break that pattern, and it breaks it by synthesizing
//! input, which puts it on a constitution NON-NEGOTIABLE surface.
//!
//! Three structural choices follow from that, and each is load-bearing rather than
//! stylistic:
//!
//! 1. **No new input path.** Synthesis goes through [`AutoPotionSink`], whose real
//!    implementation calls the same [`InputBackend::synthesize`] the fishing
//!    controller uses, so focus scoping and recursion flagging are inherited from
//!    the input engine rather than re-implemented here.
//! 2. **The rule is a pure function.** [`evaluate`] takes everything it reads as
//!    arguments and returns a typed reason for declining, so every blocking
//!    condition can be tested in isolation with all the others satisfied. A bare
//!    boolean would let a test pass because a *different* condition happened to be
//!    false, which is the failure mode this feature can least afford.
//! 3. **No state machine.** Unlike [`FishingController`](crate::fishing::FishingController)
//!    there is no sequence to be partway through: every evaluation is a function
//!    of the current readings plus the last attempt time.
//!
//! The single most important rule in the module is that **an unreadable value is
//! never a permissive one**. An unknown resource is not low, an unknown quickslot
//! is not a potion, and an unknown cooldown is not zero. The failure directions are
//! not symmetric: treating unknown as permissive fires a potion on every beacon
//! hiccup, addon reload, and loading screen, while treating it as blocking means
//! the feature quietly does nothing during an outage. See
//! `specs/039-auto-potion/research.md` R1.

use serde::Deserialize;

use crate::config::{Notice, NoticeKind};
use crate::input::{InputBackend, Key, Transition};
use crate::pixelbus::{QuickslotState, ResourceLevel, ResourceSet, SlotCooldown};

/// The largest accepted retry interval, in milliseconds.
const MAX_RETRY_MS: u32 = 600_000;

/// One resource's participation in the trigger rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceWatch {
    /// Whether this resource contributes to the trigger at all.
    pub enabled: bool,
    /// Fire at or below this percentage of the current maximum.
    pub threshold: u8,
}

impl Default for ResourceWatch {
    fn default() -> Self {
        Self {
            // Disabled by default, like the feature itself. A feature that presses
            // keys does not arrive switched on, and neither do its triggers.
            enabled: false,
            threshold: 35,
        }
    }
}

/// The auto-potion configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AutoPotionConfig {
    /// The health watch.
    pub health: ResourceWatch,
    /// The magicka watch.
    pub magicka: ResourceWatch,
    /// The stamina watch.
    pub stamina: ResourceWatch,
    /// The key synthesized to drink.
    pub quickslot_key: Key,
    /// The minimum time between two attempts, in milliseconds.
    ///
    /// This is not a duplicate of the quickslot cooldown. The cooldown is read
    /// from the screen, so it does not change until the addon redraws the block
    /// and the companion samples it, which is at least one sampling interval after
    /// the key is pressed. In that window the rule still evaluates to "low, potion
    /// present, cooldown zero" on every sample. This is the floor that covers
    /// exactly that lag; the cooldown is the authority once it updates.
    pub retry_interval_ms: u32,
}

impl Default for AutoPotionConfig {
    fn default() -> Self {
        Self {
            health: ResourceWatch::default(),
            magicka: ResourceWatch::default(),
            stamina: ResourceWatch::default(),
            // The game's default quickslot bind.
            quickslot_key: Key::Q,
            retry_interval_ms: 1500,
        }
    }
}

/// Why the trigger rule declined to fire.
///
/// A typed reason rather than a boolean, because the tests must assert *which*
/// condition blocked. Without that, a test for one condition passes when a
/// different condition is accidentally false, and the gate it thinks it is
/// checking is never exercised.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Block {
    /// Auto-potion is switched off.
    Disabled,
    /// The ESO client is not active.
    GameInactive,
    /// The ESO client does not hold keyboard focus.
    Unfocused,
    /// The application is suspended.
    Suspended,
    /// A native game UI surface or text field is up.
    Gated,
    /// The minimum retry interval has not elapsed.
    RetryTooSoon,
    /// The active quickslot holds no usable potion, or could not be read.
    NoPotion,
    /// The quickslot is still counting down.
    OnCooldown,
    /// No enabled resource is readable and at or below its threshold.
    NoResourceLow,
}

/// What the pixel bus decoded, which is all the caller supplies per tick.
///
/// Deliberately separate from [`PotionInputs`]: the gates are owned by the
/// controller and must have exactly one source of truth. An earlier draft put
/// them in this struct as well, so a caller could pass `gated: false` while the
/// controller was gated and the rule would happily fire. The tests caught it, and
/// splitting the types is what makes that state unrepresentable rather than
/// merely tested against.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PotionReadings {
    /// The three decoded resource pools.
    pub resources: ResourceSet,
    /// The decoded quickslot.
    pub quickslot: QuickslotState,
}

/// Everything the rule reads, gathered explicitly so no condition is implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PotionInputs {
    /// What the bus decoded.
    pub readings: PotionReadings,
    /// Whether the ESO client is active.
    pub game_active: bool,
    /// Whether the ESO client holds keyboard focus.
    pub focused: bool,
    /// Whether the application is suspended.
    ///
    /// Pushed in rather than read from the input engine, so the rule stays a pure
    /// function of its arguments and the suspended case is testable without
    /// constructing an engine.
    pub suspended: bool,
    /// Whether a native game UI surface is gating input.
    pub gated: bool,
}

/// Whether one resource satisfies its watch.
///
/// Three things must hold, and the middle one is the safety-critical one: the
/// level must be a real reading. [`ResourceLevel::Unknown`] never satisfies a
/// watch, at any threshold, including 100.
fn watch_satisfied(watch: ResourceWatch, level: ResourceLevel) -> bool {
    if !watch.enabled {
        return false;
    }
    match level {
        ResourceLevel::Unknown => false,
        ResourceLevel::Percent(p) => p <= watch.threshold,
    }
}

/// The complete trigger rule.
///
/// `Ok(())` means fire. `Err(block)` names the first condition that declined, in
/// the order given in `specs/039-auto-potion/contracts/trigger-rule.md`. The
/// order affects only which reason is reported; the outcome is the conjunction
/// either way.
///
/// Signal loss is deliberately absent: it is handled one level up, where the
/// routing layer disables the controller, exactly as it does for fishing. Putting
/// it here would leave the controller enabled with a stale reading in front of it.
pub fn evaluate(
    inputs: PotionInputs,
    config: &AutoPotionConfig,
    enabled: bool,
    last_attempt_ms: Option<u64>,
    now_ms: u64,
) -> Result<(), Block> {
    if !enabled {
        return Err(Block::Disabled);
    }
    if !inputs.game_active {
        return Err(Block::GameInactive);
    }
    if !inputs.focused {
        return Err(Block::Unfocused);
    }
    if inputs.suspended {
        return Err(Block::Suspended);
    }
    if inputs.gated {
        return Err(Block::Gated);
    }
    if let Some(last) = last_attempt_ms {
        if now_ms.saturating_sub(last) < u64::from(config.retry_interval_ms) {
            return Err(Block::RetryTooSoon);
        }
    }
    // `has_potion` is `cooldown != Unknown`, so this rejects an unreadable
    // quickslot as well as an empty or non-potion one. The cooldown check below is
    // still needed: `RemainingMs` passes here and fails there.
    if !inputs.readings.quickslot.has_potion() {
        return Err(Block::NoPotion);
    }
    if inputs.readings.quickslot.cooldown != SlotCooldown::Ready {
        return Err(Block::OnCooldown);
    }
    let any_low = watch_satisfied(config.health, inputs.readings.resources.health)
        || watch_satisfied(config.magicka, inputs.readings.resources.magicka)
        || watch_satisfied(config.stamina, inputs.readings.resources.stamina);
    if !any_low {
        return Err(Block::NoResourceLow);
    }
    Ok(())
}

/// The seam through which the controller synthesizes the quickslot key.
///
/// Identical in shape to [`FishingSink`](crate::fishing::FishingSink), and for the
/// same reason: it is the only place this feature reaches synthesis, so the real
/// implementation is the one line that has to be right for the whole feature to
/// stay inside the input engine's safety properties.
pub trait AutoPotionSink {
    /// Synthesizes one key transition of the given key.
    fn key(&mut self, key: Key, transition: Transition);
}

/// A test sink that records each emitted key transition in order.
#[derive(Debug, Default)]
pub struct MockAutoPotionSink {
    /// The ordered log of emitted key operations.
    pub ops: Vec<(Key, Transition)>,
}

impl MockAutoPotionSink {
    /// Creates an empty mock sink.
    pub fn new() -> Self {
        Self::default()
    }

    /// Clears the recorded operations.
    pub fn clear(&mut self) {
        self.ops.clear();
    }
}

impl AutoPotionSink for MockAutoPotionSink {
    fn key(&mut self, key: Key, transition: Transition) {
        self.ops.push((key, transition));
    }
}

/// A real sink driving the input engine's synthesis. Never panics or blocks.
pub struct RealAutoPotionSink<B> {
    backend: B,
}

impl<B: InputBackend> RealAutoPotionSink<B> {
    /// Creates a real sink over the given input backend.
    pub fn new(backend: B) -> Self {
        Self { backend }
    }
}

impl<B: InputBackend> AutoPotionSink for RealAutoPotionSink<B> {
    fn key(&mut self, key: Key, transition: Transition) {
        if let Err(err) = self.backend.synthesize(key, transition) {
            tracing::warn!(target: "eso_weave::potion", "quickslot synthesis failed: {err}");
        }
    }
}

/// The auto-potion controller.
///
/// Holds the operator's toggle, the gates pushed in from outside, and when the key
/// was last pressed. Everything else is computed per tick.
pub struct AutoPotionController {
    config: AutoPotionConfig,
    enabled: bool,
    game_active: bool,
    focused: bool,
    gated: bool,
    suspended: bool,
    last_attempt_ms: Option<u64>,
}

impl AutoPotionController {
    /// Creates a controller that is switched off.
    ///
    /// Off is the only correct starting state, and it is not restored from the
    /// previous session either. A restored fishing session does nothing until the
    /// operator stands at a fishing hole; a restored auto-potion waits silently to
    /// press a key days later in a fight the operator does not associate with this
    /// application.
    pub fn new(config: AutoPotionConfig) -> Self {
        Self {
            config,
            enabled: false,
            game_active: false,
            focused: false,
            gated: false,
            suspended: false,
            last_attempt_ms: None,
        }
    }

    /// Whether auto-potion is switched on.
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// The controller's configuration.
    pub fn config(&self) -> &AutoPotionConfig {
        &self.config
    }

    /// When the key was last pressed, if ever.
    pub fn last_attempt_ms(&self) -> Option<u64> {
        self.last_attempt_ms
    }

    /// Switches auto-potion on or off.
    pub fn set_enabled(&mut self, enabled: bool) {
        if enabled != self.enabled {
            tracing::debug!(
                target: "eso_weave::potion",
                enabled,
                "auto-potion toggled"
            );
        }
        self.enabled = enabled;
    }

    /// Sets the process-derived game-active gate without changing the requested
    /// enable toggle.
    pub fn set_game_active(&mut self, active: bool) {
        self.game_active = active;
        if !active {
            self.gated = false;
        }
    }

    /// Sets the operating-system focus gate without changing the requested
    /// enable toggle.
    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    /// Sets whether a native game UI surface is gating input.
    ///
    /// Applied to the controller directly rather than only to the interception
    /// path, because this controller acts on its own timers and never passes
    /// through interception. That distinction was found the hard way when the menu
    /// gate landed and the fishing controller kept synthesizing through it.
    pub fn set_gated(&mut self, gated: bool) {
        self.gated = gated;
    }

    /// Sets whether the application is suspended.
    ///
    /// Suspend is the operator saying "stop touching my game", so it is a checked
    /// condition rather than something that happens to hold because of how the
    /// worker loop is wired.
    pub fn set_suspended(&mut self, suspended: bool) {
        self.suspended = suspended;
    }

    /// Switches auto-potion off because the beacon signal was lost.
    ///
    /// Matches fishing: without readings the rule has nothing trustworthy to act
    /// on, and continuing to evaluate against stale values is precisely the
    /// blind firing this feature must not do.
    pub fn on_signal_lost(&mut self) {
        if self.enabled {
            tracing::debug!(
                target: "eso_weave::potion",
                "auto-potion disabled: beacon signal lost"
            );
            self.enabled = false;
        }
    }

    /// Evaluates the rule and, if it fires, presses the quickslot key once.
    ///
    /// Returns the reason it declined, for the caller to log or a test to assert.
    /// The last-attempt time is recorded on the *attempt*, not on a confirmed
    /// drink, because the game's confirmation is the quickslot cooldown and that is
    /// exactly the reading that lags.
    pub fn tick(
        &mut self,
        readings: PotionReadings,
        now_ms: u64,
        sink: &mut dyn AutoPotionSink,
    ) -> Result<(), Block> {
        // The gates come from the controller, never from the caller. That is the
        // single source of truth the split between the two input types exists to
        // enforce.
        let inputs = PotionInputs {
            readings,
            game_active: self.game_active,
            focused: self.focused,
            suspended: self.suspended,
            gated: self.gated,
        };
        let outcome = evaluate(
            inputs,
            &self.config,
            self.enabled,
            self.last_attempt_ms,
            now_ms,
        );
        if outcome.is_ok() {
            tracing::debug!(
                target: "eso_weave::potion",
                key = %self.config.quickslot_key,
                "auto-potion firing"
            );
            sink.key(self.config.quickslot_key, Transition::Down);
            sink.key(self.config.quickslot_key, Transition::Up);
            self.last_attempt_ms = Some(now_ms);
        }
        outcome
    }
}

#[derive(Deserialize, Default)]
struct RawWatch {
    #[serde(default)]
    enabled: Option<bool>,
    #[serde(default)]
    threshold: Option<u32>,
}

#[derive(Deserialize, Default)]
struct RawPotion {
    #[serde(default)]
    health: Option<RawWatch>,
    #[serde(default)]
    magicka: Option<RawWatch>,
    #[serde(default)]
    stamina: Option<RawWatch>,
    #[serde(default)]
    quickslot_key: Option<String>,
    #[serde(default)]
    retry_interval_ms: Option<u32>,
}

fn load_watch(raw: Option<RawWatch>, name: &str, notices: &mut Vec<Notice>) -> ResourceWatch {
    let defaults = ResourceWatch::default();
    let Some(raw) = raw else {
        return defaults;
    };
    let threshold = match raw.threshold {
        None => defaults.threshold,
        // 0 and 100 are both meaningful; only past 100 is out of range.
        Some(value) if value <= 100 => value as u8,
        Some(value) => {
            notices.push(Notice {
                kind: NoticeKind::InvalidValue,
                message: format!(
                    "auto-potion {name} threshold {value} is out of range; using default {}",
                    defaults.threshold
                ),
            });
            defaults.threshold
        }
    };
    ResourceWatch {
        enabled: raw.enabled.unwrap_or(defaults.enabled),
        threshold,
    }
}

impl AutoPotionConfig {
    /// Loads the auto-potion configuration from the opaque `potion` settings
    /// value. A null value yields defaults; an out-of-range threshold or interval,
    /// or an unparsable key, falls back to its default with a notice.
    pub fn load(value: &serde_json::Value, notices: &mut Vec<Notice>) -> AutoPotionConfig {
        if value.is_null() {
            return AutoPotionConfig::default();
        }
        let raw: RawPotion = serde_json::from_value(value.clone()).unwrap_or_default();
        let defaults = AutoPotionConfig::default();
        AutoPotionConfig {
            health: load_watch(raw.health, "health", notices),
            magicka: load_watch(raw.magicka, "magicka", notices),
            stamina: load_watch(raw.stamina, "stamina", notices),
            quickslot_key: match raw.quickslot_key {
                None => defaults.quickslot_key,
                Some(name) => match Key::parse(&name) {
                    Some(key) => key,
                    None => {
                        notices.push(Notice {
                            kind: NoticeKind::InvalidValue,
                            message: format!(
                                "auto-potion quickslot_key '{name}' is not a known key; using default {}",
                                defaults.quickslot_key
                            ),
                        });
                        defaults.quickslot_key
                    }
                },
            },
            retry_interval_ms: match raw.retry_interval_ms {
                None => defaults.retry_interval_ms,
                Some(ms) if ms <= MAX_RETRY_MS => ms,
                Some(ms) => {
                    notices.push(Notice {
                        kind: NoticeKind::InvalidValue,
                        message: format!(
                            "auto-potion retry_interval_ms {ms} is out of range; using default {}",
                            defaults.retry_interval_ms
                        ),
                    });
                    defaults.retry_interval_ms
                }
            },
        }
    }

    /// Serializes the auto-potion configuration to the opaque `potion` value.
    pub fn store(&self) -> serde_json::Value {
        let watch = |w: &ResourceWatch| serde_json::json!({ "enabled": w.enabled, "threshold": w.threshold });
        serde_json::json!({
            "health": watch(&self.health),
            "magicka": watch(&self.magicka),
            "stamina": watch(&self.stamina),
            "quickslot_key": self.quickslot_key.as_str(),
            "retry_interval_ms": self.retry_interval_ms,
        })
    }
}
