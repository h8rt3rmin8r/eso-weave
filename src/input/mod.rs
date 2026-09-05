//! Input Engine: platform-abstracted key interception and synthesis.
//!
//! All safety-critical decisions live in the platform-agnostic [`InputEngine`]
//! core, which is fully testable through [`mock::MockBackend`]. The OS-specific
//! interception and synthesis live behind the [`InputBackend`] seam.

pub mod action;
pub mod bindings;
pub mod key;
pub mod mock;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(windows)]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::LinuxBackend;
#[cfg(windows)]
pub use windows::WindowsBackend;

use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TrySendError};
use std::sync::{Arc, Mutex};

use crate::config::{Notice, Settings};

pub use action::Action;
pub use bindings::{BindingTable, Conflict};
pub use key::Key;

/// A cheaply cloned, independently updateable life-state synthesis gate.
///
/// The pixel worker closes this before it waits for controller mutexes, and the
/// weave sink reads it between timed operations. That separation prevents a
/// running weave from delaying authoritative death evidence.
#[derive(Debug, Clone)]
struct AtomicGate(Arc<AtomicBool>);

impl Default for AtomicGate {
    fn default() -> Self {
        Self(Arc::new(AtomicBool::new(true)))
    }
}

impl AtomicGate {
    fn set(&self, gated: bool) {
        self.0.store(gated, Ordering::Relaxed);
    }

    fn is_gated(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

#[derive(Debug, Clone, Default)]
pub struct LifeGate(AtomicGate);

impl LifeGate {
    /// Changes the gate. Only a validated Alive signal sets this to false.
    pub fn set(&self, gated: bool) {
        self.0.set(gated);
    }

    /// Whether new synthesized presses are currently forbidden.
    pub fn is_gated(&self) -> bool {
        self.0.is_gated()
    }
}

/// A cheaply cloned gate for the generated-weave roll-dodge boundary.
#[derive(Debug, Clone, Default)]
pub struct RollGate(AtomicGate);

impl RollGate {
    fn set(&self, gated: bool) {
        self.0.set(gated);
    }

    /// Whether roll-dodge evidence currently forbids generated weave work.
    pub fn is_gated(&self) -> bool {
        self.0.is_gated()
    }
}

/// The independently updated safety gates observed by a running weave sequence.
#[derive(Debug, Clone)]
pub struct WeaveGates {
    life: LifeGate,
    roll: RollGate,
}

impl WeaveGates {
    /// Whether any safety authority currently blocks generated weave work.
    pub fn is_gated(&self) -> bool {
        self.life.is_gated() || self.roll.is_gated()
    }
}

/// Whether a key event is a press or a release.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transition {
    /// Key pressed.
    Down,
    /// Key released.
    Up,
}

/// Whether a key event came from a real device or was synthesized by the engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Origin {
    /// A real device event.
    Real,
    /// An event the engine synthesized (never intercepted).
    SelfOriginated,
}

/// A mouse button the engine can synthesize (used by weave sequences).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    /// The left mouse button (basic attack).
    Primary,
    /// The right mouse button (block or bash modifier).
    Secondary,
}

/// A single key transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyEvent {
    /// The key identity.
    pub key: Key,
    /// Press or release.
    pub transition: Transition,
    /// Real or self-originated.
    pub origin: Origin,
}

/// The classification result for a key event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    /// Suppress the original keystroke.
    Suppress,
    /// Let the keystroke pass through untouched.
    Pass,
}

/// An error from a platform backend.
#[derive(thiserror::Error, Debug)]
pub enum InputError {
    /// Interception could not be started (for example a missing permission).
    #[error("could not start interception: {0}")]
    Start(String),
    /// Synthesizing a key failed.
    #[error("synthesis failed: {0}")]
    Synth(String),
}

/// The receiving half of the hand-off channel, drained by the worker.
pub type ActionReceiver = Receiver<Action>;

/// The platform-agnostic engine core: holds bindings and state and makes the
/// safety-critical classification decision for each key event.
pub struct InputEngine {
    bindings: Mutex<BindingTable>,
    focused: AtomicBool,
    game_active: AtomicBool,
    suspended: AtomicBool,
    menu_gated: AtomicBool,
    life_gate: LifeGate,
    roll_gate: RollGate,
    held: Mutex<HashSet<Key>>,
    passed_through: Mutex<HashSet<Key>>,
    active: Mutex<HashSet<Action>>,
    tx: SyncSender<Action>,
}

impl InputEngine {
    /// Creates an engine with the given bindings and hand-off channel capacity,
    /// returning the engine and the receiver the worker drains.
    pub fn new(bindings: BindingTable, channel_capacity: usize) -> (InputEngine, ActionReceiver) {
        let (tx, rx) = sync_channel(channel_capacity);
        let engine = InputEngine {
            bindings: Mutex::new(bindings),
            focused: AtomicBool::new(false),
            game_active: AtomicBool::new(false),
            suspended: AtomicBool::new(false),
            menu_gated: AtomicBool::new(false),
            life_gate: LifeGate::default(),
            roll_gate: RollGate::default(),
            held: Mutex::new(HashSet::new()),
            passed_through: Mutex::new(HashSet::new()),
            active: Mutex::new(Action::ALL.into_iter().collect()),
            tx,
        };
        (engine, rx)
    }

    /// Sets whether the game window holds keyboard focus.
    pub fn set_focused(&self, focused: bool) {
        self.focused.store(focused, Ordering::Relaxed);
        if !focused {
            self.held.lock().unwrap().clear();
        }
    }

    /// Sets whether the ESO client is currently present. Inactive is the safe
    /// startup value, so process detection must positively enable interception.
    pub fn set_game_active(&self, active: bool) {
        self.game_active.store(active, Ordering::Relaxed);
        if !active {
            self.menu_gated.store(false, Ordering::Relaxed);
            self.life_gate.set(true);
            self.roll_gate.set(true);
            self.held.lock().unwrap().clear();
        }
    }

    /// Whether the ESO client is currently present.
    pub fn is_game_active(&self) -> bool {
        self.game_active.load(Ordering::Relaxed)
    }

    /// Sets whether the engine is suspended.
    pub fn set_suspended(&self, suspended: bool) {
        self.suspended.store(suspended, Ordering::Relaxed);
    }

    /// Whether the engine is suspended.
    pub fn is_suspended(&self) -> bool {
        self.suspended.load(Ordering::Relaxed)
    }

    /// Sets whether a native game UI surface is active, as read from the beacon.
    ///
    /// While set, [`classify`](Self::classify) passes every non-exempt key through
    /// instead of intercepting it, so the operator can type in an in-game text
    /// field without a weave firing or a keystroke being swallowed. This is an
    /// automatic, game-driven form of suspend and carries the same exemption for
    /// the application's own toggle hotkeys.
    ///
    /// Defaults to `false`, which is the value that reproduces the engine's
    /// behavior before this gate existed. Every failure mode (an addon too old to
    /// publish the signal, a sample that does not decode, a lost beacon signal)
    /// leaves it there, so the gate can never fail closed.
    pub fn set_menu_gated(&self, gated: bool) {
        self.menu_gated.store(gated, Ordering::Relaxed);
    }

    /// Whether a native game UI surface is currently gating input.
    pub fn is_menu_gated(&self) -> bool {
        self.menu_gated.load(Ordering::Relaxed)
    }

    /// Sets whether the authoritative player life state blocks input.
    ///
    /// This defaults true and is released only by a valid Alive signal. Like the
    /// menu gate it can only make a bound physical key pass through, and it keeps
    /// application toggle hotkeys available.
    pub fn set_life_gated(&self, gated: bool) {
        self.life_gate.set(gated);
    }

    /// Whether player life state currently blocks synthesized work.
    pub fn is_life_gated(&self) -> bool {
        self.life_gate.is_gated()
    }

    /// A shared handle for synthesis workers that must observe life transitions
    /// without waiting for a controller mutex.
    pub fn life_gate(&self) -> LifeGate {
        self.life_gate.clone()
    }

    /// Sets whether roll-dodge evidence blocks generated weave work.
    ///
    /// This defaults true and is released only by a valid Inactive signal. Like
    /// the life gate, it passes physical skill input through and leaves toggle
    /// hotkeys available.
    pub fn set_roll_gated(&self, gated: bool) {
        self.roll_gate.set(gated);
    }

    /// Whether roll-dodge evidence currently blocks generated weave work.
    pub fn is_roll_gated(&self) -> bool {
        self.roll_gate.is_gated()
    }

    /// Shared safety handles for the running weave sink.
    pub fn weave_gates(&self) -> WeaveGates {
        WeaveGates {
            life: self.life_gate.clone(),
            roll: self.roll_gate.clone(),
        }
    }

    /// Sets whether an action is active. An inactive action's bound key passes
    /// through to the game instead of being intercepted (master specification
    /// section 7.1: an inactive slot's key passes through unmodified).
    pub fn set_action_active(&self, action: Action, active: bool) {
        let mut set = self.active.lock().unwrap();
        if active {
            set.insert(action);
        } else {
            set.remove(&action);
        }
    }

    /// The single safety-critical decision, synchronous and non-blocking. Only
    /// reads state, looks up the binding, updates held-key state, and performs at
    /// most one non-blocking hand-off. Never sleeps or does timed work.
    pub fn classify(&self, event: KeyEvent) -> Decision {
        if event.origin == Origin::SelfOriginated {
            return Decision::Pass;
        }
        // A release must retire physical held-key state even when a lifecycle or
        // focus transition makes the event pass through. Otherwise the first
        // press after ESO returns can be mistaken for auto-repeat and suppressed
        // without handing off its action.
        if event.transition == Transition::Up {
            self.held.lock().unwrap().remove(&event.key);
            if self.passed_through.lock().unwrap().remove(&event.key) {
                return Decision::Pass;
            }
        }
        if !self.game_active.load(Ordering::Relaxed) {
            return self.pass_physical(event);
        }
        if !self.focused.load(Ordering::Relaxed) {
            return self.pass_physical(event);
        }

        let bound = self.bindings.lock().unwrap().lookup(event.key);
        let Some((action, suspend_exempt)) = bound else {
            return self.pass_physical(event);
        };
        if !self.active.lock().unwrap().contains(&action) {
            return self.pass_physical(event);
        }
        if self.suspended.load(Ordering::Relaxed) && !suspend_exempt {
            return self.pass_physical(event);
        }
        // The menu gate: a native game UI surface is up, so the operator may be
        // typing. Same shape and same exemption as the suspend check above, and
        // like every other check here it can only produce a Pass, which is what
        // makes it impossible for this gate to widen interception.
        if self.menu_gated.load(Ordering::Relaxed) && !suspend_exempt {
            return self.pass_physical(event);
        }
        if self.life_gate.is_gated() && !suspend_exempt {
            return self.pass_physical(event);
        }
        if self.roll_gate.is_gated() && !suspend_exempt {
            return self.pass_physical(event);
        }

        match event.transition {
            Transition::Down => {
                let newly_pressed = self.held.lock().unwrap().insert(event.key);
                if newly_pressed {
                    self.hand_off(action);
                }
            }
            Transition::Up => {
                // Already retired before the safety gates so pass-through
                // releases cannot strand this state.
            }
        }
        Decision::Suppress
    }

    fn pass_physical(&self, event: KeyEvent) -> Decision {
        if event.transition == Transition::Down {
            self.passed_through.lock().unwrap().insert(event.key);
        }
        Decision::Pass
    }

    fn hand_off(&self, action: Action) {
        match self.tx.try_send(action) {
            Ok(()) => {}
            Err(TrySendError::Full(_)) => {
                tracing::warn!(
                    target: "eso_weave::input",
                    "hand-off channel full; dropping {action:?}"
                );
            }
            Err(TrySendError::Disconnected(_)) => {
                tracing::warn!(
                    target: "eso_weave::input",
                    "hand-off channel disconnected; dropping {action:?}"
                );
            }
        }
    }

    /// A snapshot copy of the current binding table.
    pub fn bindings(&self) -> BindingTable {
        self.bindings.lock().unwrap().clone()
    }

    /// Rebinds an action, rejecting a conflicting key.
    pub fn rebind(&self, action: Action, key: Key) -> Result<(), Conflict> {
        self.bindings.lock().unwrap().rebind(action, key)
    }

    /// Loads the binding table from settings, returning any fallback notices.
    pub fn load_bindings(&self, settings: &Settings) -> Vec<Notice> {
        let (table, notices) = BindingTable::from_settings_map(&settings.bindings);
        *self.bindings.lock().unwrap() = table;
        notices
    }

    /// Writes the current binding table into settings for persistence.
    pub fn store_bindings(&self, settings: &mut Settings) {
        settings.bindings = self.bindings.lock().unwrap().to_settings_map();
    }
}

/// The OS seam: interception and synthesis. Implemented by the mock and the
/// platform backends.
pub trait InputBackend {
    /// Synthesizes a key transition, marked so the engine treats it as
    /// self-originated.
    fn synthesize(&self, key: Key, transition: Transition) -> Result<(), InputError>;

    /// Synthesizes a mouse button transition, marked self-originated.
    fn synthesize_mouse(
        &self,
        button: MouseButton,
        transition: Transition,
    ) -> Result<(), InputError>;

    /// Starts interception, feeding the engine focus and classification. Blocks
    /// for the lifetime of interception. Returns an error if it cannot start.
    fn run(&self, engine: std::sync::Arc<InputEngine>) -> Result<(), InputError>;
}
