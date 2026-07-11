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
use std::sync::Mutex;

use crate::config::{Notice, Settings};

pub use action::Action;
pub use bindings::{BindingTable, Conflict};
pub use key::Key;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    suspended: AtomicBool,
    held: Mutex<HashSet<Key>>,
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
            suspended: AtomicBool::new(false),
            held: Mutex::new(HashSet::new()),
            active: Mutex::new(Action::ALL.into_iter().collect()),
            tx,
        };
        (engine, rx)
    }

    /// Sets whether the game window holds keyboard focus.
    pub fn set_focused(&self, focused: bool) {
        self.focused.store(focused, Ordering::Relaxed);
    }

    /// Sets whether the engine is suspended.
    pub fn set_suspended(&self, suspended: bool) {
        self.suspended.store(suspended, Ordering::Relaxed);
    }

    /// Whether the engine is suspended.
    pub fn is_suspended(&self) -> bool {
        self.suspended.load(Ordering::Relaxed)
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
        if !self.focused.load(Ordering::Relaxed) {
            return Decision::Pass;
        }

        let bound = self.bindings.lock().unwrap().lookup(event.key);
        let Some((action, suspend_exempt)) = bound else {
            return Decision::Pass;
        };
        if !self.active.lock().unwrap().contains(&action) {
            return Decision::Pass;
        }
        if self.suspended.load(Ordering::Relaxed) && !suspend_exempt {
            return Decision::Pass;
        }

        match event.transition {
            Transition::Down => {
                let newly_pressed = self.held.lock().unwrap().insert(event.key);
                if newly_pressed {
                    self.hand_off(action);
                }
            }
            Transition::Up => {
                self.held.lock().unwrap().remove(&event.key);
            }
        }
        Decision::Suppress
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
