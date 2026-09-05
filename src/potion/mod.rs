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
//! 1. **One established backend path.** Synthesis goes through [`AutoPotionSink`],
//!    whose real implementation calls the same [`InputBackend::synthesize`] the
//!    fishing controller uses. This preserves recursion flagging; focus is an
//!    explicit fail-closed controller input because autonomous synthesis does not
//!    pass through [`crate::input::InputEngine::classify`].
//! 2. **The rule is a pure function.** [`evaluate`] takes everything it reads as
//!    arguments and returns a typed reason for declining, so every blocking
//!    condition can be tested in isolation with all the others satisfied. A bare
//!    boolean would let a test pass because a *different* condition happened to be
//!    false, which is the failure mode this feature can least afford.
//! 3. **One effective state.** The pure rule produces the controller-owned state
//!    shown by the UI. Requested enablement remains separate, so temporary game,
//!    focus, or beacon loss cannot silently rewrite the operator's choice.
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
use crate::pixelbus::{
    LifeState, QuickslotClassification, QuickslotPotionAvailability, QuickslotState, ResourceLevel,
    ResourceSet, SlotCooldown, TravelState, WorldState,
};

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

/// A watched resource that can authorize an auto-potion attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoPotionResource {
    /// The player's Health pool.
    Health,
    /// The player's Magicka pool.
    Magicka,
    /// The player's Stamina pool.
    Stamina,
}

/// The low-resource observation that authorized one input attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TriggerCause {
    /// The watched resource that crossed its threshold.
    pub resource: AutoPotionResource,
    /// The fresh observed percentage.
    pub observed_percent: u8,
    /// The configured threshold percentage.
    pub threshold_percent: u8,
}

/// A normal lifecycle condition that makes a requested feature dormant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DormantReason {
    /// The ESO client is not active.
    GameInactive,
    /// The ESO client does not hold keyboard focus.
    Unfocused,
}

/// A current safety or observation condition that prevents input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockReason {
    /// No fresh PixelBus heartbeat is available.
    BeaconUnavailable,
    /// The application is suspended.
    Suspended,
    /// A native game UI surface or text field is up.
    GameContext,
    /// The player is not authoritatively alive.
    PlayerUnavailable(LifeState),
    /// The world is loading or lacks an authoritative active baseline.
    WorldUnavailable,
    /// A travel attempt is pending or cannot be ruled out.
    TravelPending,
    /// None of the three resource watches is enabled.
    NoWatchedResource,
    /// No enabled resource has a fresh percentage.
    ResourcesUnavailable,
    /// The quickslot observation is unavailable or untrusted.
    QuickslotUnavailable,
    /// The quickslot is empty or holds a non-potion action.
    NoPotion,
    /// The selected potion is depleted or blocked by ESO.
    PotionUnavailable,
    /// The quickslot cooldown is active or unavailable.
    PotionCooldown,
    /// The minimum retry interval has not elapsed.
    RetryInterval,
}

/// The truthful effective state of auto-potion at the latest evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AutoPotionState {
    /// The operator has not requested the feature.
    #[default]
    Off,
    /// The request is retained while the game lifecycle is not actionable.
    Dormant(DormantReason),
    /// The request is retained while a safety or observation condition blocks.
    Blocked(BlockReason),
    /// Every precondition is satisfied, but no watched resource is low.
    Ready,
    /// One complete input attempt was submitted for this cause.
    Triggered(TriggerCause),
}

impl AutoPotionState {
    fn diagnostic_name(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Dormant(DormantReason::GameInactive) => "dormant_game_inactive",
            Self::Dormant(DormantReason::Unfocused) => "dormant_unfocused",
            Self::Blocked(BlockReason::BeaconUnavailable) => "blocked_beacon_unavailable",
            Self::Blocked(BlockReason::Suspended) => "blocked_suspended",
            Self::Blocked(BlockReason::GameContext) => "blocked_game_context",
            Self::Blocked(BlockReason::PlayerUnavailable(LifeState::Unknown)) => {
                "blocked_player_unknown"
            }
            Self::Blocked(BlockReason::PlayerUnavailable(LifeState::Dead)) => "blocked_player_dead",
            Self::Blocked(BlockReason::PlayerUnavailable(LifeState::Reincarnating)) => {
                "blocked_player_reincarnating"
            }
            Self::Blocked(BlockReason::PlayerUnavailable(LifeState::Alive)) => {
                "blocked_player_alive_invalid"
            }
            Self::Blocked(BlockReason::WorldUnavailable) => "blocked_world_unavailable",
            Self::Blocked(BlockReason::TravelPending) => "blocked_travel_pending",
            Self::Blocked(BlockReason::NoWatchedResource) => "blocked_no_watched_resource",
            Self::Blocked(BlockReason::ResourcesUnavailable) => "blocked_resources_unavailable",
            Self::Blocked(BlockReason::QuickslotUnavailable) => "blocked_quickslot_unavailable",
            Self::Blocked(BlockReason::NoPotion) => "blocked_no_potion",
            Self::Blocked(BlockReason::PotionUnavailable) => "blocked_potion_unavailable",
            Self::Blocked(BlockReason::PotionCooldown) => "blocked_potion_cooldown",
            Self::Blocked(BlockReason::RetryInterval) => "blocked_retry_interval",
            Self::Ready => "ready",
            Self::Triggered(TriggerCause {
                resource: AutoPotionResource::Health,
                ..
            }) => "triggered_health",
            Self::Triggered(TriggerCause {
                resource: AutoPotionResource::Magicka,
                ..
            }) => "triggered_magicka",
            Self::Triggered(TriggerCause {
                resource: AutoPotionResource::Stamina,
                ..
            }) => "triggered_stamina",
        }
    }
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
    /// Whether a fresh PixelBus heartbeat is available.
    pub beacon_available: bool,
    /// Whether the application is suspended.
    ///
    /// Pushed in rather than read from the input engine, so the rule stays a pure
    /// function of its arguments and the suspended case is testable without
    /// constructing an engine.
    pub suspended: bool,
    /// Whether a native game UI surface is gating input.
    pub gated: bool,
    /// The authoritative player life state. Only Alive permits synthesis.
    pub life: LifeState,
    /// Authoritative world lifecycle state. Only Active permits synthesis.
    pub world: WorldState,
    /// Bounded travel state. Only Inactive permits synthesis.
    pub travel: TravelState,
}

fn low_resource(
    config: &AutoPotionConfig,
    resources: ResourceSet,
) -> Result<Option<TriggerCause>, BlockReason> {
    let watches = [
        (AutoPotionResource::Health, config.health, resources.health),
        (
            AutoPotionResource::Magicka,
            config.magicka,
            resources.magicka,
        ),
        (
            AutoPotionResource::Stamina,
            config.stamina,
            resources.stamina,
        ),
    ];
    if watches.iter().all(|(_, watch, _)| !watch.enabled) {
        return Err(BlockReason::NoWatchedResource);
    }

    let mut any_fresh = false;
    for (resource, watch, level) in watches {
        if !watch.enabled {
            continue;
        }
        if let ResourceLevel::Percent(observed_percent) = level {
            any_fresh = true;
            if observed_percent <= watch.threshold {
                return Ok(Some(TriggerCause {
                    resource,
                    observed_percent,
                    threshold_percent: watch.threshold,
                }));
            }
        }
    }
    if any_fresh {
        Ok(None)
    } else {
        Err(BlockReason::ResourcesUnavailable)
    }
}

/// The complete trigger rule.
///
/// The result names the first non-actionable condition in the S043 contract. A
/// caller may synthesize input only for [`AutoPotionState::Triggered`].
pub fn evaluate(
    inputs: PotionInputs,
    config: &AutoPotionConfig,
    enabled: bool,
    last_attempt_ms: Option<u64>,
    now_ms: u64,
) -> AutoPotionState {
    if !enabled {
        return AutoPotionState::Off;
    }
    if !inputs.game_active {
        return AutoPotionState::Dormant(DormantReason::GameInactive);
    }
    if !inputs.focused {
        return AutoPotionState::Dormant(DormantReason::Unfocused);
    }
    if !inputs.beacon_available {
        return AutoPotionState::Blocked(BlockReason::BeaconUnavailable);
    }
    if inputs.suspended {
        return AutoPotionState::Blocked(BlockReason::Suspended);
    }
    if inputs.gated {
        return AutoPotionState::Blocked(BlockReason::GameContext);
    }
    if inputs.life.gates() {
        return AutoPotionState::Blocked(BlockReason::PlayerUnavailable(inputs.life));
    }
    if inputs.world.gates() {
        return AutoPotionState::Blocked(BlockReason::WorldUnavailable);
    }
    if inputs.travel.gates() {
        return AutoPotionState::Blocked(BlockReason::TravelPending);
    }

    let cause = match low_resource(config, inputs.readings.resources) {
        Ok(cause) => cause,
        Err(reason) => return AutoPotionState::Blocked(reason),
    };

    match inputs.readings.quickslot.classification {
        QuickslotClassification::Unavailable(_) => {
            return AutoPotionState::Blocked(BlockReason::QuickslotUnavailable);
        }
        QuickslotClassification::Empty | QuickslotClassification::NonPotion(_) => {
            return AutoPotionState::Blocked(BlockReason::NoPotion);
        }
        QuickslotClassification::Potion(
            QuickslotPotionAvailability::Depleted | QuickslotPotionAvailability::Blocked,
        ) => {
            return AutoPotionState::Blocked(BlockReason::PotionUnavailable);
        }
        QuickslotClassification::Potion(QuickslotPotionAvailability::Usable) => {}
    }
    if inputs.readings.quickslot.cooldown != SlotCooldown::Ready {
        return AutoPotionState::Blocked(BlockReason::PotionCooldown);
    }
    if let Some(last) = last_attempt_ms {
        if now_ms.saturating_sub(last) < u64::from(config.retry_interval_ms) {
            return AutoPotionState::Blocked(BlockReason::RetryInterval);
        }
    }
    cause.map_or(AutoPotionState::Ready, AutoPotionState::Triggered)
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
    beacon_available: bool,
    gated: bool,
    suspended: bool,
    life: LifeState,
    world: WorldState,
    travel: TravelState,
    last_attempt_ms: Option<u64>,
    state: AutoPotionState,
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
            beacon_available: false,
            // Fail closed until the reader positively observes the gameplay
            // surface. A missing initial surface block emits no routing event,
            // so an ungated default could synthesize before gameplay was proven.
            gated: true,
            suspended: false,
            life: LifeState::Unknown,
            world: WorldState::Unknown,
            travel: TravelState::Unknown,
            last_attempt_ms: None,
            state: AutoPotionState::Off,
        }
    }

    /// Whether the operator requests auto-potion for this session.
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// The controller's configuration.
    pub fn config(&self) -> &AutoPotionConfig {
        &self.config
    }

    /// Replaces the live settings without changing request or lifecycle state.
    pub fn set_config(&mut self, config: AutoPotionConfig) {
        if config != self.config {
            tracing::debug!(target: "eso_weave::potion", "auto-potion settings updated");
        }
        self.config = config;
    }

    /// When the key was last pressed, if ever.
    pub fn last_attempt_ms(&self) -> Option<u64> {
        self.last_attempt_ms
    }

    /// The effective result of the most recent evaluation or lifecycle change.
    pub fn state(&self) -> AutoPotionState {
        self.state
    }

    /// The authoritative player life state used by the synthesis gate.
    pub fn life_state(&self) -> LifeState {
        self.life
    }

    fn set_state(&mut self, state: AutoPotionState) {
        if state == self.state {
            return;
        }
        tracing::info!(
            target: "eso_weave::potion",
            previous = self.state.diagnostic_name(),
            current = state.diagnostic_name(),
            "auto-potion state changed"
        );
        self.state = state;
    }

    fn apply_immediate_state(&mut self) {
        let state = if !self.enabled {
            Some(AutoPotionState::Off)
        } else if !self.game_active {
            Some(AutoPotionState::Dormant(DormantReason::GameInactive))
        } else if !self.focused {
            Some(AutoPotionState::Dormant(DormantReason::Unfocused))
        } else if !self.beacon_available {
            Some(AutoPotionState::Blocked(BlockReason::BeaconUnavailable))
        } else if self.suspended {
            Some(AutoPotionState::Blocked(BlockReason::Suspended))
        } else if self.gated {
            Some(AutoPotionState::Blocked(BlockReason::GameContext))
        } else if self.life.gates() {
            Some(AutoPotionState::Blocked(BlockReason::PlayerUnavailable(
                self.life,
            )))
        } else if self.world.gates() {
            Some(AutoPotionState::Blocked(BlockReason::WorldUnavailable))
        } else if self.travel.gates() {
            Some(AutoPotionState::Blocked(BlockReason::TravelPending))
        } else {
            None
        };
        if let Some(state) = state {
            self.set_state(state);
        }
    }

    /// Changes requested auto-potion enablement for this session.
    pub fn set_enabled(&mut self, enabled: bool) {
        if enabled != self.enabled {
            tracing::debug!(
                target: "eso_weave::potion",
                enabled,
                "auto-potion toggled"
            );
        }
        self.enabled = enabled;
        self.apply_immediate_state();
    }

    /// Sets the process-derived game-active gate without changing the requested
    /// enable toggle.
    pub fn set_game_active(&mut self, active: bool) {
        self.game_active = active;
        if !active {
            // Surface evidence belongs to one game session. Require a fresh
            // positive gameplay observation after the process returns.
            self.gated = true;
            self.life = LifeState::Unknown;
            self.world = WorldState::Unknown;
            self.travel = TravelState::Unknown;
        }
        self.apply_immediate_state();
    }

    /// Sets the operating-system focus gate without changing the requested
    /// enable toggle.
    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
        self.apply_immediate_state();
    }

    /// Sets whether a native game UI surface is gating input.
    ///
    /// Applied to the controller directly rather than only to the interception
    /// path, because this controller acts on its own timers and never passes
    /// through interception. That distinction was found the hard way when the menu
    /// gate landed and the fishing controller kept synthesizing through it.
    pub fn set_gated(&mut self, gated: bool) {
        self.gated = gated;
        self.apply_immediate_state();
    }

    /// Sets the authoritative player life state without changing requested enablement.
    pub fn set_life_state(&mut self, life: LifeState) {
        self.life = life;
        self.apply_immediate_state();
    }

    /// Sets authoritative world lifecycle state.
    pub fn set_world_state(&mut self, world: WorldState) {
        self.world = world;
        self.apply_immediate_state();
    }

    /// Sets bounded travel state.
    pub fn set_travel_state(&mut self, travel: TravelState) {
        self.travel = travel;
        self.apply_immediate_state();
    }

    /// Sets whether the application is suspended.
    ///
    /// Suspend is the operator saying "stop touching my game", so it is a checked
    /// condition rather than something that happens to hold because of how the
    /// worker loop is wired.
    pub fn set_suspended(&mut self, suspended: bool) {
        self.suspended = suspended;
        self.apply_immediate_state();
    }

    /// Records a fresh PixelBus heartbeat without changing requested enablement.
    pub fn on_heartbeat(&mut self) {
        self.beacon_available = true;
        self.apply_immediate_state();
    }

    /// Records beacon signal loss without changing requested enablement.
    pub fn on_signal_lost(&mut self) {
        self.beacon_available = false;
        // Do not reuse a gameplay observation across a signal outage. The first
        // recovered heartbeat can precede a decodable surface block.
        self.gated = true;
        self.life = LifeState::Unknown;
        self.world = WorldState::Unknown;
        self.travel = TravelState::Unknown;
        self.apply_immediate_state();
    }

    /// Evaluates the rule and, if it fires, presses the quickslot key once.
    ///
    /// Returns and stores the effective state. The last-attempt time is recorded
    /// on the *attempt*, not on a confirmed
    /// drink, because the game's confirmation is the quickslot cooldown and that is
    /// exactly the reading that lags.
    pub fn tick(
        &mut self,
        readings: PotionReadings,
        now_ms: u64,
        sink: &mut dyn AutoPotionSink,
    ) -> AutoPotionState {
        // The gates come from the controller, never from the caller. That is the
        // single source of truth the split between the two input types exists to
        // enforce.
        let inputs = PotionInputs {
            readings,
            game_active: self.game_active,
            focused: self.focused,
            beacon_available: self.beacon_available,
            suspended: self.suspended,
            gated: self.gated,
            life: self.life,
            world: self.world,
            travel: self.travel,
        };
        let outcome = evaluate(
            inputs,
            &self.config,
            self.enabled,
            self.last_attempt_ms,
            now_ms,
        );
        if matches!(outcome, AutoPotionState::Triggered(_)) {
            tracing::debug!(
                target: "eso_weave::potion",
                key = %self.config.quickslot_key,
                "auto-potion firing"
            );
            sink.key(self.config.quickslot_key, Transition::Down);
            sink.key(self.config.quickslot_key, Transition::Up);
            self.last_attempt_ms = Some(now_ms);
        }
        self.set_state(outcome);
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
