//! Fishing Controller: a pure, event-and-tick-driven fishing state machine.
//!
//! The [`FishingController`] consumes [`DetectorEvent`]s (from a [`BiteDetector`])
//! and clock ticks, driving the Disabled, Armed, Waiting, Reeling, Recast state
//! machine. All delays and timeouts are deadlines evaluated against an injected
//! millisecond clock, so the controller never blocks. The interact key is emitted
//! through the [`FishingSink`] seam. On [`DetectorEvent::SignalLost`] the
//! controller disables fishing and cancels any pending interact rather than
//! blind-firing. The controller depends on the input engine and the pixel bus
//! reader, not on the weave engine.

pub mod detector;

pub use detector::{map_event, BiteDetector, PixelBusDetector, StubDetector};

use serde::Deserialize;

use crate::config::{Notice, NoticeKind};
use crate::input::{InputBackend, Key, LifeGate, Transition};
use crate::pixelbus::LifeState;

/// The maximum accepted value for a fishing timing parameter, in milliseconds.
const MAX_TIMING_MS: u32 = 60_000;

/// The typed events a [`BiteDetector`] emits (no latency).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetectorEvent {
    /// The signal is live (observed; drives no state change on its own).
    Heartbeat,
    /// A cast became active and is waiting.
    FishingStarted,
    /// A bite occurred.
    BiteDetected,
    /// Fishing stopped (the cast ended without an active bite path).
    FishingStopped,
    /// The beacon heartbeat was lost.
    SignalLost,
}

/// The observable controller state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FishingState {
    /// Not fishing; emits nothing.
    Disabled,
    /// Cast sent; awaiting FishingStarted until the arm timeout.
    Armed,
    /// Cast active; awaiting a bite.
    Waiting,
    /// Bite seen; awaiting the reel deadline to emit the reel interact.
    Reeling,
    /// Reel emitted; awaiting the recast deadline, then the recast interact, then
    /// FishingStarted.
    Recast,
}

/// Why the controller last returned to Disabled. Recorded when it disables and
/// cleared when a new cast starts, so the UI can explain an idle state instead of
/// reverting silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopReason {
    /// The player turned fishing off.
    UserStop,
    /// The arm timeout fired without a cast confirmation.
    NoCastDetected,
    /// The beacon signal was lost while a session was active.
    SignalLost,
    /// The ESO client exited while fishing was requested.
    GameInactive,
    /// The ESO client lost keyboard focus while a session was active.
    Unfocused,
    /// The player is dead, reincarnating, or not authoritatively known alive.
    PlayerUnavailable,
}

/// The kind of the controller's single pending deadline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TimerKind {
    /// In Armed: disarm to Disabled if it fires.
    ArmTimeout,
    /// In Reeling: emit the reel interact if it fires.
    ReelDue,
    /// In Recast: emit the recast interact if it fires.
    RecastDue,
    /// In Recast after the recast interact: re-cast (return to Armed) if it fires.
    RecastArmTimeout,
}

/// User-configurable fishing timing and the interact key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FishingConfig {
    /// Maximum wait for FishingStarted after a cast or recast, in milliseconds.
    pub arm_timeout_ms: u32,
    /// Delay after BiteDetected before the reel interact, in milliseconds.
    pub reel_delay_ms: u32,
    /// Delay after reeling before the recast interact, in milliseconds.
    pub recast_delay_ms: u32,
    /// The key synthesized to cast, reel, and recast.
    pub interact_key: Key,
}

impl Default for FishingConfig {
    fn default() -> Self {
        Self {
            // 8000 ms gives the cast confirmation adequate margin over addon
            // render and interact-registration latency now that the worker polls
            // at the fishing cadence; provisional pending in-game validation.
            arm_timeout_ms: 8000,
            reel_delay_ms: 100,
            recast_delay_ms: 3000,
            interact_key: Key::E,
        }
    }
}

#[derive(Deserialize, Default)]
struct RawFishing {
    #[serde(default)]
    arm_timeout_ms: Option<u32>,
    #[serde(default)]
    reel_delay_ms: Option<u32>,
    #[serde(default)]
    recast_delay_ms: Option<u32>,
    #[serde(default)]
    interact_key: Option<String>,
}

impl FishingConfig {
    /// Loads the fishing configuration from the opaque `fishing` settings value.
    /// A null value yields defaults; an out-of-range timing or an unparsable
    /// interact key falls back to its default with an [`NoticeKind::InvalidValue`]
    /// notice.
    pub fn load(value: &serde_json::Value, notices: &mut Vec<Notice>) -> FishingConfig {
        if value.is_null() {
            return FishingConfig::default();
        }
        let raw: RawFishing = serde_json::from_value(value.clone()).unwrap_or_default();
        let defaults = FishingConfig::default();
        FishingConfig {
            arm_timeout_ms: checked(
                raw.arm_timeout_ms,
                defaults.arm_timeout_ms,
                "arm_timeout_ms",
                notices,
            ),
            reel_delay_ms: checked(
                raw.reel_delay_ms,
                defaults.reel_delay_ms,
                "reel_delay_ms",
                notices,
            ),
            recast_delay_ms: checked(
                raw.recast_delay_ms,
                defaults.recast_delay_ms,
                "recast_delay_ms",
                notices,
            ),
            interact_key: match raw.interact_key {
                None => defaults.interact_key,
                Some(name) => match Key::parse(&name) {
                    Some(key) => key,
                    None => {
                        notices.push(Notice {
                            kind: NoticeKind::InvalidValue,
                            message: format!(
                                "fishing interact_key '{name}' is not a known key; using default {}",
                                defaults.interact_key
                            ),
                        });
                        defaults.interact_key
                    }
                },
            },
        }
    }

    /// Serializes the fishing configuration to the opaque `fishing` settings value.
    pub fn store(&self) -> serde_json::Value {
        serde_json::json!({
            "arm_timeout_ms": self.arm_timeout_ms,
            "reel_delay_ms": self.reel_delay_ms,
            "recast_delay_ms": self.recast_delay_ms,
            "interact_key": self.interact_key.as_str(),
        })
    }
}

fn checked(value: Option<u32>, default: u32, name: &str, notices: &mut Vec<Notice>) -> u32 {
    match value {
        None => default,
        Some(ms) if ms <= MAX_TIMING_MS => ms,
        Some(_) => {
            notices.push(Notice {
                kind: NoticeKind::InvalidValue,
                message: format!("fishing {name} is out of range; using default {default}"),
            });
            default
        }
    }
}

/// The seam through which the controller synthesizes the interact key.
pub trait FishingSink {
    /// Synthesizes one key transition (a press or a release) of the given key.
    fn key(&mut self, key: Key, transition: Transition);
}

/// A test sink that records each emitted key transition in order.
#[derive(Debug, Default)]
pub struct MockFishingSink {
    /// The ordered log of emitted key operations.
    pub ops: Vec<(Key, Transition)>,
}

impl MockFishingSink {
    /// Creates an empty mock sink.
    pub fn new() -> Self {
        Self::default()
    }

    /// Clears the recorded operations.
    pub fn clear(&mut self) {
        self.ops.clear();
    }
}

impl FishingSink for MockFishingSink {
    fn key(&mut self, key: Key, transition: Transition) {
        self.ops.push((key, transition));
    }
}

/// A real sink that drives the input engine's synthesis. Never panics or blocks.
pub struct RealFishingSink<B> {
    backend: B,
}

impl<B: InputBackend> RealFishingSink<B> {
    /// Creates a real sink over the given input backend.
    pub fn new(backend: B) -> Self {
        Self { backend }
    }
}

impl<B: InputBackend> FishingSink for RealFishingSink<B> {
    fn key(&mut self, key: Key, transition: Transition) {
        if let Err(err) = self.backend.synthesize(key, transition) {
            tracing::warn!(target: "eso_weave::fishing", "interact synthesis failed: {err}");
        }
    }
}

/// The fishing controller state machine.
pub struct FishingController {
    config: FishingConfig,
    requested_enabled: bool,
    state: FishingState,
    deadline: Option<(u64, TimerKind)>,
    stop_reason: Option<StopReason>,
    gated: bool,
    game_active: bool,
    focused: bool,
    life: LifeState,
    life_gate: LifeGate,
}

/// How long an interact deferred by the menu gate waits before trying again.
/// Short enough that fishing resumes promptly when the surface closes, and long
/// enough that a gated session is not busy-retrying.
const GATE_DEFER_MS: u64 = 100;

impl FishingController {
    /// Creates a controller in the Disabled state.
    pub fn new(config: FishingConfig) -> Self {
        Self::with_life_gate(config, LifeGate::default())
    }

    /// Creates a controller attached to the independently updated synthesis gate.
    pub fn with_life_gate(config: FishingConfig, life_gate: LifeGate) -> Self {
        Self {
            config,
            requested_enabled: false,
            state: FishingState::Disabled,
            deadline: None,
            stop_reason: None,
            gated: false,
            game_active: false,
            focused: false,
            life: LifeState::Unknown,
            life_gate,
        }
    }

    /// Whether the operator has requested fishing. Runtime and focus can pause
    /// the effective session without silently changing this choice.
    pub fn enabled(&self) -> bool {
        self.requested_enabled
    }

    /// The current observable state.
    pub fn state(&self) -> FishingState {
        self.state
    }

    /// Why fishing last returned to Disabled, if it has since startup and a new
    /// cast has not since cleared it. Only meaningful while [`state`](Self::state)
    /// is Disabled.
    pub fn stop_reason(&self) -> Option<StopReason> {
        self.stop_reason
    }

    /// The controller's configuration.
    pub fn config(&self) -> &FishingConfig {
        &self.config
    }

    /// Enables or disables fishing. Enabling from Disabled arms and casts once;
    /// enabling when already active, or disabling when already Disabled, is a
    /// no-op. Disabling from any active state returns to Disabled and cancels any
    /// pending interact, emitting nothing.
    pub fn set_enabled(&mut self, enabled: bool, now_ms: u64, sink: &mut dyn FishingSink) {
        self.requested_enabled = enabled;
        if enabled {
            if self.state == FishingState::Disabled {
                if !self.game_active {
                    self.stop_reason = Some(StopReason::GameInactive);
                    return;
                }
                if !self.focused {
                    self.stop_reason = Some(StopReason::Unfocused);
                    return;
                }
                if self.block_for_life() {
                    return;
                }
                tracing::debug!(target: "eso_weave::fishing", "fishing enabled");
                self.cast(now_ms, sink);
            }
        } else {
            if self.state != FishingState::Disabled {
                self.disable(StopReason::UserStop);
            } else {
                self.stop_reason = Some(StopReason::UserStop);
            }
        }
    }

    /// Handles a detector event.
    pub fn on_event(&mut self, event: DetectorEvent, now_ms: u64, sink: &mut dyn FishingSink) {
        if !matches!(event, DetectorEvent::SignalLost | DetectorEvent::Heartbeat)
            && self.block_for_life()
        {
            return;
        }
        match event {
            DetectorEvent::SignalLost => {
                if self.requested_enabled || self.state != FishingState::Disabled {
                    self.requested_enabled = false;
                    if self.state != FishingState::Disabled {
                        self.disable(StopReason::SignalLost);
                    } else {
                        self.stop_reason = Some(StopReason::SignalLost);
                    }
                }
            }
            DetectorEvent::Heartbeat => {}
            DetectorEvent::FishingStarted => {
                if self.requested_enabled
                    && self.state == FishingState::Disabled
                    && self.game_active
                    && self.focused
                    && !self.life_gate.is_gated()
                {
                    tracing::debug!(target: "eso_weave::fishing", "fresh manual cast detected after life-state recovery");
                    self.state = FishingState::Waiting;
                    self.stop_reason = None;
                    self.deadline = None;
                    return;
                }
                if matches!(self.state, FishingState::Armed | FishingState::Recast) {
                    tracing::debug!(
                        target: "eso_weave::fishing",
                        "cast detected; waiting for bite"
                    );
                    self.state = FishingState::Waiting;
                    self.deadline = None;
                }
            }
            DetectorEvent::BiteDetected => {
                if matches!(self.state, FishingState::Waiting | FishingState::Armed) {
                    tracing::debug!(
                        target: "eso_weave::fishing",
                        "bite detected; reeling in {} ms",
                        self.config.reel_delay_ms
                    );
                    self.state = FishingState::Reeling;
                    self.deadline = Some((
                        now_ms + u64::from(self.config.reel_delay_ms),
                        TimerKind::ReelDue,
                    ));
                }
            }
            DetectorEvent::FishingStopped => {
                if matches!(
                    self.state,
                    FishingState::Waiting | FishingState::Reeling | FishingState::Recast
                ) {
                    // Cancel any pending interact and re-cast (heartbeat is live).
                    tracing::debug!(
                        target: "eso_weave::fishing",
                        "cast ended without a resolved bite; recasting"
                    );
                    self.cast(now_ms, sink);
                }
            }
        }
    }

    /// Fires the pending deadline if it is due at `now_ms`.
    pub fn tick(&mut self, now_ms: u64, sink: &mut dyn FishingSink) {
        if self.block_for_life() {
            return;
        }
        let Some((at_ms, kind)) = self.deadline else {
            return;
        };
        if now_ms < at_ms {
            return;
        }
        // The menu gate defers the two autonomous interacts. They are the ones
        // that fire unbidden while the operator may be typing; re-arming the same
        // deadline keeps the state machine consistent with what the game actually
        // received, which dropping the interact would not.
        if self.gated && matches!(kind, TimerKind::ReelDue | TimerKind::RecastDue) {
            tracing::debug!(
                target: "eso_weave::fishing",
                "menu gate active; deferring {kind:?}"
            );
            self.deadline = Some((now_ms + GATE_DEFER_MS, kind));
            return;
        }
        match kind {
            TimerKind::ArmTimeout => {
                self.requested_enabled = false;
                self.disable(StopReason::NoCastDetected);
            }
            TimerKind::ReelDue => {
                tracing::debug!(
                    target: "eso_weave::fishing",
                    "reel interact sent; recast in {} ms",
                    self.config.recast_delay_ms
                );
                self.send_interact(sink);
                self.state = FishingState::Recast;
                self.deadline = Some((
                    now_ms + u64::from(self.config.recast_delay_ms),
                    TimerKind::RecastDue,
                ));
            }
            TimerKind::RecastDue => {
                tracing::debug!(
                    target: "eso_weave::fishing",
                    "recast interact sent; awaiting cast confirmation for {} ms",
                    self.config.arm_timeout_ms
                );
                self.send_interact(sink);
                self.deadline = Some((
                    now_ms + u64::from(self.config.arm_timeout_ms),
                    TimerKind::RecastArmTimeout,
                ));
            }
            TimerKind::RecastArmTimeout => {
                tracing::debug!(
                    target: "eso_weave::fishing",
                    "recast unconfirmed; casting again"
                );
                self.cast(now_ms, sink);
            }
        }
    }

    /// Sets whether a native game UI surface is gating input.
    ///
    /// While set, the controller defers the reel and recast interacts rather than
    /// sending them, so an autonomous keypress cannot land in a chat message the
    /// operator is composing. A deferred interact retries shortly, so a session
    /// resumes on its own when the surface closes; the state machine is never
    /// advanced past an interact that was not actually sent, so its state always
    /// matches what the game received.
    ///
    /// Defaults to `false`, the value that reproduces the controller's behavior
    /// before this gate existed.
    pub fn set_gated(&mut self, gated: bool) {
        self.gated = gated;
    }

    /// Applies the authoritative life-state gate.
    ///
    /// A non-Alive state cancels pending autonomous work but preserves the
    /// operator's request. Returning to Alive emits nothing; a fresh manual cast
    /// observation or a new toggle is required before fishing can proceed.
    pub fn set_life_state(&mut self, life: LifeState) {
        self.life = life;
        self.life_gate.set(life.gates());
        self.block_for_life();
    }

    /// Updates the process and focus gates together. Losing either gate pauses an
    /// active session without changing the requested toggle; restoring both
    /// gates re-arms that request as a fresh cast.
    pub fn set_game_environment(
        &mut self,
        active: bool,
        focused: bool,
        now_ms: u64,
        sink: &mut dyn FishingSink,
    ) {
        self.game_active = active;
        self.focused = focused;
        if !active {
            self.gated = false;
            self.life = LifeState::Unknown;
            self.life_gate.set(true);
            self.on_game_inactive();
        } else if !focused {
            if self.state != FishingState::Disabled {
                self.disable(StopReason::Unfocused);
            } else if self.requested_enabled {
                self.stop_reason = Some(StopReason::Unfocused);
            }
        } else if self.requested_enabled
            && self.state == FishingState::Disabled
            && !self.life_gate.is_gated()
            && matches!(
                self.stop_reason,
                Some(StopReason::GameInactive | StopReason::Unfocused)
            )
        {
            tracing::debug!(target: "eso_weave::fishing", "fishing resumed after game context returned");
            self.cast(now_ms, sink);
        }
    }

    /// Cancels an active session when the ESO process leaves the active state.
    pub fn on_game_inactive(&mut self) {
        if self.state != FishingState::Disabled {
            self.disable(StopReason::GameInactive);
        } else if self.requested_enabled {
            self.stop_reason = Some(StopReason::GameInactive);
        }
    }

    /// Enters Armed, emits one interact (the cast), arms the arm timeout, and
    /// clears any prior stop reason now that a fresh session is starting.
    fn cast(&mut self, now_ms: u64, sink: &mut dyn FishingSink) {
        if self.block_for_life() {
            return;
        }
        tracing::debug!(
            target: "eso_weave::fishing",
            "cast interact sent; armed with a {} ms cast-confirmation window",
            self.config.arm_timeout_ms
        );
        self.stop_reason = None;
        self.send_interact(sink);
        self.state = FishingState::Armed;
        self.deadline = Some((
            now_ms + u64::from(self.config.arm_timeout_ms),
            TimerKind::ArmTimeout,
        ));
    }

    /// Returns to Disabled, clears any pending deadline, and records why; emits
    /// nothing.
    fn disable(&mut self, reason: StopReason) {
        tracing::debug!(target: "eso_weave::fishing", "fishing disabled: {reason:?}");
        self.state = FishingState::Disabled;
        self.deadline = None;
        self.stop_reason = Some(reason);
    }

    fn block_for_life(&mut self) -> bool {
        if !self.life_gate.is_gated() {
            return false;
        }
        if self.requested_enabled || self.state != FishingState::Disabled {
            if self.state != FishingState::Disabled {
                self.disable(StopReason::PlayerUnavailable);
            } else {
                self.stop_reason = Some(StopReason::PlayerUnavailable);
                self.deadline = None;
            }
        }
        true
    }

    /// Emits one interact: a key press followed by a key release.
    fn send_interact(&self, sink: &mut dyn FishingSink) {
        sink.key(self.config.interact_key, Transition::Down);
        sink.key(self.config.interact_key, Transition::Up);
    }
}
