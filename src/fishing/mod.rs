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
use crate::input::{InputBackend, Key, Transition};

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
    state: FishingState,
    deadline: Option<(u64, TimerKind)>,
    stop_reason: Option<StopReason>,
}

impl FishingController {
    /// Creates a controller in the Disabled state.
    pub fn new(config: FishingConfig) -> Self {
        Self {
            config,
            state: FishingState::Disabled,
            deadline: None,
            stop_reason: None,
        }
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
        if enabled {
            if self.state == FishingState::Disabled {
                self.cast(now_ms, sink);
            }
        } else if self.state != FishingState::Disabled {
            self.disable(StopReason::UserStop);
        }
    }

    /// Handles a detector event.
    pub fn on_event(&mut self, event: DetectorEvent, now_ms: u64, sink: &mut dyn FishingSink) {
        match event {
            DetectorEvent::SignalLost => {
                if self.state != FishingState::Disabled {
                    self.disable(StopReason::SignalLost);
                }
            }
            DetectorEvent::Heartbeat => {}
            DetectorEvent::FishingStarted => {
                if matches!(self.state, FishingState::Armed | FishingState::Recast) {
                    self.state = FishingState::Waiting;
                    self.deadline = None;
                }
            }
            DetectorEvent::BiteDetected => {
                if matches!(self.state, FishingState::Waiting | FishingState::Armed) {
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
                    self.cast(now_ms, sink);
                }
            }
        }
    }

    /// Fires the pending deadline if it is due at `now_ms`.
    pub fn tick(&mut self, now_ms: u64, sink: &mut dyn FishingSink) {
        let Some((at_ms, kind)) = self.deadline else {
            return;
        };
        if now_ms < at_ms {
            return;
        }
        match kind {
            TimerKind::ArmTimeout => self.disable(StopReason::NoCastDetected),
            TimerKind::ReelDue => {
                self.send_interact(sink);
                self.state = FishingState::Recast;
                self.deadline = Some((
                    now_ms + u64::from(self.config.recast_delay_ms),
                    TimerKind::RecastDue,
                ));
            }
            TimerKind::RecastDue => {
                self.send_interact(sink);
                self.deadline = Some((
                    now_ms + u64::from(self.config.arm_timeout_ms),
                    TimerKind::RecastArmTimeout,
                ));
            }
            TimerKind::RecastArmTimeout => self.cast(now_ms, sink),
        }
    }

    /// Enters Armed, emits one interact (the cast), arms the arm timeout, and
    /// clears any prior stop reason now that a fresh session is starting.
    fn cast(&mut self, now_ms: u64, sink: &mut dyn FishingSink) {
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
        self.state = FishingState::Disabled;
        self.deadline = None;
        self.stop_reason = Some(reason);
    }

    /// Emits one interact: a key press followed by a key release.
    fn send_interact(&self, sink: &mut dyn FishingSink) {
        sink.key(self.config.interact_key, Transition::Down);
        sink.key(self.config.interact_key, Transition::Up);
    }
}
