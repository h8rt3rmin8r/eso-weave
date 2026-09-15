//! Input Engine: platform-abstracted key interception and synthesis.
//!
//! All safety-critical decisions live in the platform-agnostic [`InputEngine`]
//! core, which is fully testable through [`mock::MockBackend`]. The OS-specific
//! interception and synthesis live behind the [`InputBackend`] seam.

pub mod action;
pub mod bindings;
pub mod key;
pub mod mock;
pub mod native;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(windows)]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::LinuxBackend;
#[cfg(windows)]
pub use windows::WindowsBackend;

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, Ordering};
use std::sync::mpsc::{sync_channel, Receiver, RecvError, SyncSender, TryRecvError, TrySendError};
use std::sync::{Arc, Mutex};

use crate::config::{Notice, Settings};

pub use action::Action;
pub use bindings::{BindingTable, Conflict, RebindError};
pub use key::Key;
pub use native::{
    CombatChordPlan, CombatRequirement, KeyboardControl, ModifierSet, MouseControl, NativeAction,
    NativeBindingSet, NativeBindingState, NativeChord, NativeControl, NativeInput, NativeModifier,
};

/// A cheaply cloned, independently updateable life-state synthesis gate.
///
/// The pixel worker closes this before it waits for controller mutexes, and the
/// weave sink reads it between timed operations. That separation prevents a
/// running weave from delaying authoritative death evidence.
#[derive(Debug, Clone)]
struct AtomicGate {
    gated: Arc<AtomicBool>,
    weave_epoch: Option<Arc<AtomicU64>>,
    fishing_epoch: Option<Arc<AtomicU64>>,
}

impl Default for AtomicGate {
    fn default() -> Self {
        Self {
            gated: Arc::new(AtomicBool::new(true)),
            weave_epoch: None,
            fishing_epoch: None,
        }
    }
}

impl AtomicGate {
    fn new(
        gated: bool,
        weave_epoch: Option<Arc<AtomicU64>>,
        fishing_epoch: Option<Arc<AtomicU64>>,
    ) -> Self {
        Self {
            gated: Arc::new(AtomicBool::new(gated)),
            weave_epoch,
            fishing_epoch,
        }
    }

    fn set(&self, gated: bool) -> bool {
        let was_gated = self.gated.swap(gated, Ordering::AcqRel);
        if gated && !was_gated {
            if let Some(epoch) = &self.weave_epoch {
                epoch.fetch_add(1, Ordering::AcqRel);
            }
            if let Some(epoch) = &self.fishing_epoch {
                epoch.fetch_add(1, Ordering::AcqRel);
            }
        }
        was_gated != gated
    }

    fn is_gated(&self) -> bool {
        self.gated.load(Ordering::Acquire)
    }
}

#[derive(Debug, Clone, Default)]
pub struct LifeGate(AtomicGate);

impl LifeGate {
    /// Changes the gate. Only a validated Alive signal sets this to false.
    pub fn set(&self, gated: bool) -> bool {
        self.0.set(gated)
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

/// Shared world and travel authorities for autonomous synthesis controllers.
#[derive(Debug, Clone)]
pub struct WorldTravelGate {
    world: AtomicGate,
    travel: AtomicGate,
}

impl WorldTravelGate {
    /// Whether world lifecycle evidence currently forbids synthesis.
    pub fn world_is_gated(&self) -> bool {
        self.world.is_gated()
    }

    /// Whether travel evidence currently forbids synthesis.
    pub fn travel_is_gated(&self) -> bool {
        self.travel.is_gated()
    }

    /// Whether either authority currently forbids synthesis.
    pub fn is_gated(&self) -> bool {
        self.world_is_gated() || self.travel_is_gated()
    }
}

/// The independently updated safety gates observed by a running weave sequence.
#[derive(Debug, Clone)]
pub struct WeaveGates {
    game: AtomicGate,
    focus: AtomicGate,
    suspension: AtomicGate,
    menu: AtomicGate,
    life: LifeGate,
    roll: RollGate,
    world: AtomicGate,
    travel: AtomicGate,
    epoch: Arc<AtomicU64>,
    physical_modifier_bits: Arc<AtomicU8>,
}

impl WeaveGates {
    /// Whether any safety authority currently blocks generated weave work.
    pub fn is_gated(&self) -> bool {
        self.game.is_gated()
            || self.focus.is_gated()
            || self.suspension.is_gated()
            || self.menu.is_gated()
            || self.life.is_gated()
            || self.roll.is_gated()
            || self.world.is_gated()
            || self.travel.is_gated()
    }

    /// Captures the current weave authorization generation.
    pub fn current_epoch(&self) -> AuthorizationEpoch {
        AuthorizationEpoch(self.epoch.load(Ordering::Acquire))
    }

    /// Whether a request admitted in `epoch` remains safe to synthesize.
    pub fn admits(&self, epoch: AuthorizationEpoch) -> bool {
        !self.is_gated() && self.current_epoch() == epoch
    }

    /// The current real-device modifier set used by chord ownership checks.
    pub fn physical_modifiers(&self) -> ModifierSet {
        ModifierSet::from_bits(self.physical_modifier_bits.load(Ordering::Acquire))
            .unwrap_or(ModifierSet::EMPTY)
    }
}

/// An opaque generation captured when a weave request is admitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthorizationEpoch(u64);

/// The applicable runtime gates for Fishing synthesis.
#[derive(Debug, Clone)]
pub struct FishingGates {
    game: AtomicGate,
    focus: AtomicGate,
    suspension: AtomicGate,
    menu: AtomicGate,
    life: LifeGate,
    world: AtomicGate,
    travel: AtomicGate,
    epoch: Arc<AtomicU64>,
}

impl FishingGates {
    /// Whether an applicable authority currently blocks Fishing synthesis.
    pub fn is_gated(&self) -> bool {
        self.game.is_gated()
            || self.focus.is_gated()
            || self.suspension.is_gated()
            || self.menu.is_gated()
            || self.life.is_gated()
            || self.world.is_gated()
            || self.travel.is_gated()
    }

    /// Captures the current Fishing authorization generation.
    pub fn current_epoch(&self) -> FishingAuthorizationEpoch {
        FishingAuthorizationEpoch(self.epoch.load(Ordering::Acquire))
    }

    /// Whether Fishing work admitted in `epoch` remains safe to synthesize.
    pub fn admits(&self, epoch: FishingAuthorizationEpoch) -> bool {
        !self.is_gated() && self.current_epoch() == epoch
    }
}

/// An opaque generation captured when Fishing work is admitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FishingAuthorizationEpoch(u64);

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

/// One platform-native physical input transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeInputEvent {
    pub input: NativeInput,
    pub transition: Transition,
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

/// An action plus the authorization generation under which it was admitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueuedAction {
    action: Action,
    authorization_epoch: AuthorizationEpoch,
    combat_plan: Option<CombatChordPlan>,
}

impl QueuedAction {
    /// The requested application or weave action.
    pub fn action(self) -> Action {
        self.action
    }

    /// The weave authorization generation captured during interception.
    pub fn authorization_epoch(self) -> AuthorizationEpoch {
        self.authorization_epoch
    }

    /// The immutable native chord plan captured for a combat request.
    pub fn combat_plan(self) -> Option<CombatChordPlan> {
        self.combat_plan
    }
}

/// The receiving half of the hand-off channel, drained by the worker.
pub struct ActionReceiver(Receiver<QueuedAction>);

impl ActionReceiver {
    /// Receives an action while preserving the legacy action-only interface.
    pub fn recv(&self) -> Result<Action, RecvError> {
        self.0.recv().map(QueuedAction::action)
    }

    /// Tries to receive an action while preserving the legacy action-only interface.
    pub fn try_recv(&self) -> Result<Action, TryRecvError> {
        self.0.try_recv().map(QueuedAction::action)
    }

    /// Receives an action with its captured authorization generation.
    pub fn recv_authorized(&self) -> Result<QueuedAction, RecvError> {
        self.0.recv()
    }

    /// Tries to receive an action with its captured authorization generation.
    pub fn try_recv_authorized(&self) -> Result<QueuedAction, TryRecvError> {
        self.0.try_recv()
    }
}

/// The platform-agnostic engine core: holds bindings and state and makes the
/// safety-critical classification decision for each key event.
pub struct InputEngine {
    bindings: Mutex<BindingTable>,
    native_bindings: Mutex<NativeBindingSet>,
    combat_requirements: Mutex<HashMap<Action, CombatRequirement>>,
    focused: AtomicBool,
    game_active: AtomicBool,
    suspended: AtomicBool,
    menu_gated: AtomicBool,
    game_gate: AtomicGate,
    focus_gate: AtomicGate,
    suspension_gate: AtomicGate,
    menu_gate: AtomicGate,
    life_gate: LifeGate,
    roll_gate: RollGate,
    world_gate: AtomicGate,
    travel_gate: AtomicGate,
    weave_authorization_epoch: Arc<AtomicU64>,
    fishing_authorization_epoch: Arc<AtomicU64>,
    death_epoch: AtomicU64,
    safety_refresh_generation: AtomicU64,
    physical_modifier_bits: Arc<AtomicU8>,
    held: Mutex<HashSet<NativeControl>>,
    passed_through: Mutex<HashSet<NativeControl>>,
    active: Mutex<HashSet<Action>>,
    tx: SyncSender<QueuedAction>,
}

impl InputEngine {
    /// Creates an engine with the given bindings and hand-off channel capacity,
    /// returning the engine and the receiver the worker drains.
    pub fn new(bindings: BindingTable, channel_capacity: usize) -> (InputEngine, ActionReceiver) {
        let (tx, rx) = sync_channel(channel_capacity);
        let weave_epoch = Arc::new(AtomicU64::new(0));
        let fishing_epoch = Arc::new(AtomicU64::new(0));
        let shared_gate = |gated| {
            AtomicGate::new(
                gated,
                Some(Arc::clone(&weave_epoch)),
                Some(Arc::clone(&fishing_epoch)),
            )
        };
        let engine = InputEngine {
            bindings: Mutex::new(bindings),
            native_bindings: Mutex::new(NativeBindingSet::new_unavailable()),
            combat_requirements: Mutex::new(
                Action::COMBAT
                    .into_iter()
                    .map(|action| (action, CombatRequirement::LIGHT_OR_HEAVY))
                    .collect(),
            ),
            focused: AtomicBool::new(false),
            game_active: AtomicBool::new(false),
            suspended: AtomicBool::new(false),
            menu_gated: AtomicBool::new(true),
            game_gate: shared_gate(true),
            focus_gate: shared_gate(true),
            suspension_gate: shared_gate(false),
            menu_gate: shared_gate(true),
            life_gate: LifeGate(shared_gate(true)),
            roll_gate: RollGate(AtomicGate::new(true, Some(Arc::clone(&weave_epoch)), None)),
            world_gate: shared_gate(true),
            travel_gate: shared_gate(true),
            weave_authorization_epoch: weave_epoch,
            fishing_authorization_epoch: fishing_epoch,
            death_epoch: AtomicU64::new(0),
            safety_refresh_generation: AtomicU64::new(0),
            physical_modifier_bits: Arc::new(AtomicU8::new(0)),
            held: Mutex::new(HashSet::new()),
            passed_through: Mutex::new(HashSet::new()),
            active: Mutex::new(Action::ALL.into_iter().collect()),
            tx,
        };
        (engine, ActionReceiver(rx))
    }

    /// Sets whether the game window holds keyboard focus.
    pub fn set_focused(&self, focused: bool) {
        if focused {
            self.focused.store(true, Ordering::Release);
            self.focus_gate.set(false);
        } else {
            self.focus_gate.set(true);
            self.focused.store(false, Ordering::Release);
        }
        if !focused {
            self.held.lock().unwrap().clear();
        }
    }

    /// Sets whether the ESO client is currently present. Inactive is the safe
    /// startup value, so process detection must positively enable interception.
    pub fn set_game_active(&self, active: bool) {
        if active {
            self.game_active.store(true, Ordering::Release);
            self.game_gate.set(false);
        } else {
            self.game_gate.set(true);
            self.game_active.store(false, Ordering::Release);
        }
        if !active {
            self.menu_gated.store(true, Ordering::Relaxed);
            self.menu_gate.set(true);
            self.life_gate.set(true);
            self.roll_gate.set(true);
            self.world_gate.set(true);
            self.travel_gate.set(true);
            self.held.lock().unwrap().clear();
        }
    }

    /// Whether the ESO client is currently present.
    pub fn is_game_active(&self) -> bool {
        self.game_active.load(Ordering::Relaxed)
    }

    /// Sets whether the engine is suspended.
    ///
    /// Resuming closes the world and travel gates before interception reopens.
    /// The pixel worker observes the generation change and republishes fresh
    /// safety evidence before either gate may open again.
    pub fn set_suspended(&self, suspended: bool) {
        if !suspended && self.suspended.load(Ordering::Acquire) {
            self.world_gate.set(true);
            self.travel_gate.set(true);
            self.suspended.store(false, Ordering::Release);
            self.suspension_gate.set(false);
            self.safety_refresh_generation
                .fetch_add(1, Ordering::Release);
            return;
        }
        if suspended {
            self.suspension_gate.set(true);
            self.suspended.store(true, Ordering::Release);
        } else {
            self.suspended.store(false, Ordering::Release);
            self.suspension_gate.set(false);
        }
    }

    /// Whether the engine is suspended.
    pub fn is_suspended(&self) -> bool {
        self.suspended.load(Ordering::Acquire)
    }

    /// Monotonic request observed by the pixel worker after every resume.
    pub fn safety_refresh_generation(&self) -> u64 {
        self.safety_refresh_generation.load(Ordering::Acquire)
    }

    /// Sets whether a native game UI surface is active, as read from the beacon.
    ///
    /// While set, [`classify`](Self::classify) passes every non-exempt key through
    /// instead of intercepting it, so the operator can type in an in-game text
    /// field without a weave firing or a keystroke being swallowed. This is an
    /// automatic, game-driven form of suspend and carries the same exemption for
    /// the application's own toggle hotkeys.
    ///
    /// Defaults to `true`. Every unavailable-evidence mode (an addon too old to
    /// publish the signal, a sample that does not decode, or a lost beacon signal)
    /// keeps or returns this gate to its fail-closed state.
    pub fn set_menu_gated(&self, gated: bool) {
        if gated {
            self.menu_gate.set(true);
            self.menu_gated.store(true, Ordering::Release);
        } else {
            self.menu_gated.store(false, Ordering::Release);
            self.menu_gate.set(false);
        }
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
        let changed = self.life_gate.set(gated);
        if changed && gated {
            let death_epoch = self.death_epoch.fetch_add(1, Ordering::AcqRel) + 1;
            tracing::info!(
                target: "eso_weave::input",
                death_epoch,
                gated,
                "life authorization gate closed"
            );
        } else if changed {
            tracing::info!(
                target: "eso_weave::input",
                death_epoch = self.death_epoch.load(Ordering::Acquire),
                gated,
                "life authorization gate opened"
            );
        }
    }

    /// Whether player life state currently blocks synthesized work.
    pub fn is_life_gated(&self) -> bool {
        self.life_gate.is_gated()
    }

    /// Monotonic count of open-to-closed life authorization transitions.
    pub fn death_epoch(&self) -> u64 {
        self.death_epoch.load(Ordering::Acquire)
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

    /// Sets whether world lifecycle evidence blocks synthesized work.
    pub fn set_world_gated(&self, gated: bool) {
        self.world_gate.set(gated);
    }

    /// Whether world lifecycle evidence currently blocks synthesized work.
    pub fn is_world_gated(&self) -> bool {
        self.world_gate.is_gated()
    }

    /// Sets whether bounded travel evidence blocks synthesized work.
    pub fn set_travel_gated(&self, gated: bool) {
        self.travel_gate.set(gated);
    }

    /// Whether bounded travel evidence currently blocks synthesized work.
    pub fn is_travel_gated(&self) -> bool {
        self.travel_gate.is_gated()
    }

    /// Shared safety handles for the running weave sink.
    pub fn weave_gates(&self) -> WeaveGates {
        WeaveGates {
            game: self.game_gate.clone(),
            focus: self.focus_gate.clone(),
            suspension: self.suspension_gate.clone(),
            menu: self.menu_gate.clone(),
            life: self.life_gate.clone(),
            roll: self.roll_gate.clone(),
            world: self.world_gate.clone(),
            travel: self.travel_gate.clone(),
            epoch: Arc::clone(&self.weave_authorization_epoch),
            physical_modifier_bits: Arc::clone(&self.physical_modifier_bits),
        }
    }

    /// Shared runtime authorization for Fishing synthesis.
    pub fn fishing_gates(&self) -> FishingGates {
        FishingGates {
            game: self.game_gate.clone(),
            focus: self.focus_gate.clone(),
            suspension: self.suspension_gate.clone(),
            menu: self.menu_gate.clone(),
            life: self.life_gate.clone(),
            world: self.world_gate.clone(),
            travel: self.travel_gate.clone(),
            epoch: Arc::clone(&self.fishing_authorization_epoch),
        }
    }

    /// Captures the current weave authorization generation.
    pub fn authorization_epoch(&self) -> AuthorizationEpoch {
        AuthorizationEpoch(self.weave_authorization_epoch.load(Ordering::Acquire))
    }

    /// Shared pre-lock world and travel authorities for autonomous controllers.
    pub fn world_travel_gate(&self) -> WorldTravelGate {
        WorldTravelGate {
            world: self.world_gate.clone(),
            travel: self.travel_gate.clone(),
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

    /// Sets the native chords required by one combat action's weave type.
    pub fn set_combat_requirement(&self, action: Action, requirement: CombatRequirement) {
        if !action.is_app_toggle() {
            self.combat_requirements
                .lock()
                .unwrap()
                .insert(action, requirement);
        }
    }

    /// Replaces the coherent native evidence and invalidates every older combat
    /// request before the replacement can be observed.
    pub fn set_native_bindings(&self, bindings: NativeBindingSet) {
        let mut current = self.native_bindings.lock().unwrap();
        if *current != bindings {
            self.weave_authorization_epoch
                .fetch_add(1, Ordering::AcqRel);
            *current = bindings;
        }
    }

    /// Returns the latest coherent native evidence.
    pub fn native_bindings(&self) -> NativeBindingSet {
        *self.native_bindings.lock().unwrap()
    }

    /// The modifiers currently held on real physical devices.
    pub fn physical_modifiers(&self) -> ModifierSet {
        ModifierSet::from_bits(self.physical_modifier_bits.load(Ordering::Acquire))
            .unwrap_or(ModifierSet::EMPTY)
    }

    /// The single safety-critical decision, synchronous and non-blocking. Only
    /// reads state, looks up the binding, updates held-key state, and performs at
    /// most one non-blocking hand-off. Never sleeps or does timed work.
    pub fn classify(&self, event: KeyEvent) -> Decision {
        self.classify_native(NativeInputEvent {
            input: NativeInput::Primary(key_to_native(event.key)),
            transition: event.transition,
            origin: event.origin,
        })
    }

    /// The synchronous classification boundary used by both platform backends.
    pub fn classify_native(&self, event: NativeInputEvent) -> Decision {
        if event.origin == Origin::SelfOriginated {
            return Decision::Pass;
        }
        let NativeInput::Primary(primary) = event.input else {
            if let NativeInput::Modifier(modifier) = event.input {
                self.observe_physical_modifier(modifier, event.transition);
            }
            return Decision::Pass;
        };
        let authorization_epoch = self.authorization_epoch();
        // A release must retire physical held-key state even when a lifecycle or
        // focus transition makes the event pass through. Otherwise the first
        // press after ESO returns can be mistaken for auto-repeat and suppressed
        // without handing off its action.
        if event.transition == Transition::Up {
            self.held.lock().unwrap().remove(&primary);
            if self.passed_through.lock().unwrap().remove(&primary) {
                return Decision::Pass;
            }
        }
        if !self.game_active.load(Ordering::Relaxed) {
            return self.pass_physical(primary, event.transition);
        }
        if !self.focused.load(Ordering::Relaxed) {
            return self.pass_physical(primary, event.transition);
        }

        let toggle =
            native_to_key(primary).and_then(|key| self.bindings.lock().unwrap().lookup(key));
        let combat = self.resolve_combat(primary, self.physical_modifiers());
        let (action, suspend_exempt, combat_plan) = match (toggle, combat) {
            (Some((action, exempt)), _) => (action, exempt, None),
            (None, Some((action, plan))) => (action, false, Some(plan)),
            (None, None) => return self.pass_physical(primary, event.transition),
        };
        if !self.active.lock().unwrap().contains(&action) {
            return self.pass_physical(primary, event.transition);
        }
        if self.suspended.load(Ordering::Acquire) && !suspend_exempt {
            return self.pass_physical(primary, event.transition);
        }
        // The menu gate: a native game UI surface is up, so the operator may be
        // typing. Same shape and same exemption as the suspend check above, and
        // like every other check here it can only produce a Pass, which is what
        // makes it impossible for this gate to widen interception.
        if self.menu_gated.load(Ordering::Relaxed) && !suspend_exempt {
            return self.pass_physical(primary, event.transition);
        }
        if self.life_gate.is_gated() && !suspend_exempt {
            return self.pass_physical(primary, event.transition);
        }
        if self.roll_gate.is_gated() && !suspend_exempt {
            return self.pass_physical(primary, event.transition);
        }
        if self.world_gate.is_gated() && !suspend_exempt {
            return self.pass_physical(primary, event.transition);
        }
        if self.travel_gate.is_gated() && !suspend_exempt {
            return self.pass_physical(primary, event.transition);
        }

        match event.transition {
            Transition::Down => {
                if !suspend_exempt && self.authorization_epoch() != authorization_epoch {
                    return self.pass_physical(primary, event.transition);
                }
                let newly_pressed =
                    primary.is_momentary() || self.held.lock().unwrap().insert(primary);
                if newly_pressed {
                    self.hand_off(action, authorization_epoch, combat_plan);
                }
            }
            Transition::Up => {
                // Already retired before the safety gates so pass-through
                // releases cannot strand this state.
            }
        }
        Decision::Suppress
    }

    fn observe_physical_modifier(&self, modifier: NativeModifier, transition: Transition) {
        let flag = modifier.flag().bits();
        match transition {
            Transition::Down => {
                self.physical_modifier_bits.fetch_or(flag, Ordering::AcqRel);
            }
            Transition::Up => {
                self.physical_modifier_bits
                    .fetch_and(!flag, Ordering::AcqRel);
            }
        }
    }

    fn resolve_combat(
        &self,
        primary: NativeControl,
        physical: ModifierSet,
    ) -> Option<(Action, CombatChordPlan)> {
        let bindings = *self.native_bindings.lock().unwrap();
        let trigger = NativeChord {
            primary,
            modifiers: physical,
        };
        let mut matched = None;
        for native in NativeAction::ALL.into_iter().take(Action::COMBAT.len()) {
            if bindings.get(native) == NativeBindingState::Valid(trigger) {
                if matched.is_some() {
                    return None;
                }
                matched = native.combat_action();
            }
        }
        let action = matched?;
        let requirement = self
            .combat_requirements
            .lock()
            .unwrap()
            .get(&action)
            .copied()
            .unwrap_or(CombatRequirement::LIGHT_OR_HEAVY);
        let skill = valid_chord(bindings.get(native_for_action(action)))?;
        let attack = if requirement.attack {
            Some(valid_chord(bindings.get(NativeAction::Attack))?)
        } else {
            None
        };
        let block = if requirement.block {
            Some(valid_chord(bindings.get(NativeAction::Block))?)
        } else {
            None
        };
        let plan = CombatChordPlan {
            skill,
            attack,
            block,
        };
        plan.admits_physical(physical).then_some((action, plan))
    }

    fn pass_physical(&self, primary: NativeControl, transition: Transition) -> Decision {
        if transition == Transition::Down && !primary.is_momentary() {
            self.passed_through.lock().unwrap().insert(primary);
        }
        Decision::Pass
    }

    fn hand_off(
        &self,
        action: Action,
        authorization_epoch: AuthorizationEpoch,
        combat_plan: Option<CombatChordPlan>,
    ) {
        let queued = QueuedAction {
            action,
            authorization_epoch,
            combat_plan,
        };
        match self.tx.try_send(queued) {
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
    pub fn rebind(&self, action: Action, key: Key) -> Result<(), RebindError> {
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

fn valid_chord(state: NativeBindingState) -> Option<NativeChord> {
    match state {
        NativeBindingState::Valid(chord) => Some(chord),
        NativeBindingState::Unavailable
        | NativeBindingState::Unbound
        | NativeBindingState::Conflicting
        | NativeBindingState::Unsupported => None,
    }
}

fn native_for_action(action: Action) -> NativeAction {
    match action {
        Action::Skill1 => NativeAction::Skill1,
        Action::Skill2 => NativeAction::Skill2,
        Action::Skill3 => NativeAction::Skill3,
        Action::Skill4 => NativeAction::Skill4,
        Action::Skill5 => NativeAction::Skill5,
        Action::Ultimate => NativeAction::Ultimate,
        Action::Synergy => NativeAction::Synergy,
        Action::ToggleSuspend | Action::ToggleFishing | Action::ToggleAutoPotion => {
            unreachable!("application toggle has no native ESO action")
        }
    }
}

fn key_to_native(key: Key) -> NativeControl {
    NativeControl::Keyboard(match key {
        Key::Digit1 => KeyboardControl::Digit1,
        Key::Digit2 => KeyboardControl::Digit2,
        Key::Digit3 => KeyboardControl::Digit3,
        Key::Digit4 => KeyboardControl::Digit4,
        Key::Digit5 => KeyboardControl::Digit5,
        Key::E => KeyboardControl::E,
        Key::R => KeyboardControl::R,
        Key::X => KeyboardControl::X,
        Key::Q => KeyboardControl::Q,
        Key::Space => KeyboardControl::Space,
        Key::F1 => KeyboardControl::F1,
        Key::F2 => KeyboardControl::F2,
        Key::F3 => KeyboardControl::F3,
    })
}

fn native_to_key(control: NativeControl) -> Option<Key> {
    let NativeControl::Keyboard(key) = control else {
        return None;
    };
    match key {
        KeyboardControl::Digit1 => Some(Key::Digit1),
        KeyboardControl::Digit2 => Some(Key::Digit2),
        KeyboardControl::Digit3 => Some(Key::Digit3),
        KeyboardControl::Digit4 => Some(Key::Digit4),
        KeyboardControl::Digit5 => Some(Key::Digit5),
        KeyboardControl::E => Some(Key::E),
        KeyboardControl::R => Some(Key::R),
        KeyboardControl::X => Some(Key::X),
        KeyboardControl::Q => Some(Key::Q),
        KeyboardControl::Space => Some(Key::Space),
        KeyboardControl::F1 => Some(Key::F1),
        KeyboardControl::F2 => Some(Key::F2),
        KeyboardControl::F3 => Some(Key::F3),
        _ => None,
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

    /// Synthesizes one portable native keyboard or mouse primary.
    fn synthesize_native(
        &self,
        control: NativeControl,
        transition: Transition,
    ) -> Result<(), InputError>;

    /// Synthesizes one normalized native modifier.
    fn synthesize_modifier(
        &self,
        modifier: NativeModifier,
        transition: Transition,
    ) -> Result<(), InputError>;

    /// Starts interception, feeding the engine focus and classification. Blocks
    /// for the lifetime of interception. Returns an error if it cannot start.
    fn run(&self, engine: std::sync::Arc<InputEngine>) -> Result<(), InputError>;
}
