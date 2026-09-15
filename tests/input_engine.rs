//! Safety-critical tests for the Input Engine core via the mock backend.

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use eso_weave::config::Settings;
use eso_weave::input::action::Action;
use eso_weave::input::key::Key;
use eso_weave::input::mock::MockBackend;
use eso_weave::input::{
    BindingTable, CombatRequirement, Decision, InputBackend, InputEngine, InputError, KeyEvent,
    KeyboardControl, ModifierSet, ModifierSide, MouseButton, MouseControl, NativeAction,
    NativeActionExecutor, NativeBindingSet, NativeBindingState, NativeChord, NativeControl,
    NativeInput, NativeInputEvent, NativeModifier, Origin, Transition,
};

struct FailTwoPrimaryReleasesBackend {
    inner: MockBackend,
    remaining_failures: AtomicUsize,
}

impl InputBackend for FailTwoPrimaryReleasesBackend {
    fn synthesize(&self, key: Key, transition: Transition) -> Result<(), InputError> {
        self.inner.synthesize(key, transition)
    }

    fn synthesize_mouse(
        &self,
        button: MouseButton,
        transition: Transition,
    ) -> Result<(), InputError> {
        self.inner.synthesize_mouse(button, transition)
    }

    fn synthesize_native(
        &self,
        control: NativeControl,
        transition: Transition,
    ) -> Result<(), InputError> {
        self.inner.synthesize_native(control, transition)?;
        if transition == Transition::Up
            && self
                .remaining_failures
                .fetch_update(Ordering::AcqRel, Ordering::Acquire, |remaining| {
                    remaining.checked_sub(1)
                })
                .is_ok()
        {
            return Err(InputError::Synth(
                "injected primary release failure".to_string(),
            ));
        }
        Ok(())
    }

    fn synthesize_modifier(
        &self,
        modifier: NativeModifier,
        transition: Transition,
    ) -> Result<(), InputError> {
        self.inner.synthesize_modifier(modifier, transition)
    }

    fn run(&self, _engine: Arc<InputEngine>) -> Result<(), InputError> {
        Ok(())
    }
}

fn install_native(engine: &InputEngine) {
    let mut bindings = NativeBindingSet::new_unavailable();
    let keyboard = [
        KeyboardControl::Digit1,
        KeyboardControl::Digit2,
        KeyboardControl::Digit3,
        KeyboardControl::Digit4,
        KeyboardControl::Digit5,
        KeyboardControl::R,
        KeyboardControl::X,
    ];
    for (action, key) in NativeAction::ALL.into_iter().take(7).zip(keyboard) {
        bindings.set(
            action,
            NativeBindingState::Valid(NativeChord {
                primary: NativeControl::Keyboard(key),
                modifiers: ModifierSet::EMPTY,
            }),
        );
    }
    bindings.set(
        NativeAction::Attack,
        NativeBindingState::Valid(NativeChord {
            primary: NativeControl::Mouse(MouseControl::Left),
            modifiers: ModifierSet::EMPTY,
        }),
    );
    bindings.set(
        NativeAction::Block,
        NativeBindingState::Valid(NativeChord {
            primary: NativeControl::Mouse(MouseControl::Right),
            modifiers: ModifierSet::EMPTY,
        }),
    );
    engine.set_native_bindings(bindings);
}

fn native_event(input: NativeInput, transition: Transition) -> NativeInputEvent {
    NativeInputEvent {
        input,
        transition,
        origin: Origin::Real,
    }
}

fn engine() -> (InputEngine, eso_weave::input::ActionReceiver) {
    let pair = InputEngine::new(BindingTable::default(), 64);
    install_native(&pair.0);
    pair.0.set_game_active(true);
    pair.0.set_life_gated(false);
    pair.0.set_roll_gated(false);
    pair.0.set_world_gated(false);
    pair.0.set_travel_gated(false);
    pair.0.set_menu_gated(false);
    pair
}

#[test]
fn exact_modifier_chord_captures_one_immutable_native_plan() {
    let (engine, rx) = engine();
    engine.set_focused(true);
    let mut bindings = engine.native_bindings();
    let skill = NativeChord {
        primary: NativeControl::Keyboard(KeyboardControl::K),
        modifiers: ModifierSet::SHIFT,
    };
    let attack = NativeChord {
        primary: NativeControl::Mouse(MouseControl::Button4),
        modifiers: ModifierSet::SHIFT | ModifierSet::CONTROL,
    };
    bindings.set(NativeAction::Skill1, NativeBindingState::Valid(skill));
    bindings.set(NativeAction::Attack, NativeBindingState::Valid(attack));
    engine.set_native_bindings(bindings);

    assert_eq!(
        engine.classify_native(native_event(
            NativeInput::Modifier(NativeModifier::Shift),
            Transition::Down,
        )),
        Decision::Pass
    );
    assert_eq!(
        engine.classify_native(native_event(
            NativeInput::Primary(skill.primary),
            Transition::Down,
        )),
        Decision::Suppress
    );
    let queued = rx.try_recv_authorized().unwrap();
    assert_eq!(queued.action(), Action::Skill1);
    assert_eq!(queued.combat_plan().unwrap().skill, skill);
    assert_eq!(queued.combat_plan().unwrap().attack, Some(attack));
}

#[test]
fn extra_or_incompatible_physical_modifiers_pass_without_queueing() {
    let (engine, rx) = engine();
    engine.set_focused(true);
    engine.classify_native(native_event(
        NativeInput::Modifier(NativeModifier::Alt),
        Transition::Down,
    ));
    assert_eq!(
        engine.classify_native(native_event(
            NativeInput::Primary(NativeControl::Keyboard(KeyboardControl::Digit1)),
            Transition::Down,
        )),
        Decision::Pass
    );
    assert!(rx.try_recv().is_err());
}

#[test]
fn physical_modifier_ownership_survives_focus_and_process_transitions() {
    let (engine, rx) = engine();
    install_native(&engine);
    engine.set_focused(false);
    engine.classify_native(NativeInputEvent {
        input: NativeInput::Modifier(NativeModifier::Control),
        transition: Transition::Down,
        origin: Origin::Real,
    });
    engine.set_game_active(false);
    engine.set_game_active(true);
    engine.set_focused(true);
    engine.set_life_gated(false);
    engine.set_roll_gated(false);
    engine.set_world_gated(false);
    engine.set_travel_gated(false);
    engine.set_menu_gated(false);

    assert_eq!(engine.physical_modifiers(), ModifierSet::CONTROL);
    assert_eq!(
        engine.classify_native(NativeInputEvent {
            input: NativeInput::Primary(NativeControl::Keyboard(KeyboardControl::Digit1)),
            transition: Transition::Down,
            origin: Origin::Real,
        }),
        Decision::Pass
    );
    assert!(rx.try_recv().is_err());

    engine.classify_native(NativeInputEvent {
        input: NativeInput::Modifier(NativeModifier::Control),
        transition: Transition::Up,
        origin: Origin::Real,
    });
    assert_eq!(engine.physical_modifiers(), ModifierSet::EMPTY);
}

#[test]
fn mouse_bound_skill_is_intercepted_from_native_evidence() {
    let (engine, rx) = engine();
    engine.set_focused(true);
    let mut bindings = engine.native_bindings();
    let mouse_skill = NativeChord {
        primary: NativeControl::Mouse(MouseControl::Button5),
        modifiers: ModifierSet::EMPTY,
    };
    bindings.set(NativeAction::Skill1, NativeBindingState::Valid(mouse_skill));
    engine.set_native_bindings(bindings);
    assert_eq!(
        engine.classify_native(native_event(
            NativeInput::Primary(mouse_skill.primary),
            Transition::Down,
        )),
        Decision::Suppress
    );
    assert_eq!(rx.try_recv().unwrap(), Action::Skill1);
}

#[test]
fn wheel_bound_skill_treats_each_pulse_as_a_complete_activation() {
    let (engine, rx) = engine();
    engine.set_focused(true);
    let mut bindings = engine.native_bindings();
    bindings.set(
        NativeAction::Skill1,
        NativeBindingState::Valid(NativeChord {
            primary: NativeControl::Mouse(MouseControl::WheelUp),
            modifiers: ModifierSet::EMPTY,
        }),
    );
    engine.set_native_bindings(bindings);
    let event = NativeInputEvent {
        input: NativeInput::Primary(NativeControl::Mouse(MouseControl::WheelUp)),
        transition: Transition::Down,
        origin: Origin::Real,
    };

    assert_eq!(engine.classify_native(event), Decision::Suppress);
    assert_eq!(engine.classify_native(event), Decision::Suppress);
    assert_eq!(rx.try_recv().unwrap(), Action::Skill1);
    assert_eq!(rx.try_recv().unwrap(), Action::Skill1);
}

#[test]
fn duplicate_native_triggers_and_invalid_requirements_fail_closed() {
    let (engine, rx) = engine();
    engine.set_focused(true);
    let mut bindings = engine.native_bindings();
    let duplicate = bindings.get(NativeAction::Skill1);
    bindings.set(NativeAction::Skill2, duplicate);
    engine.set_native_bindings(bindings);
    assert_eq!(
        engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real)),
        Decision::Pass
    );
    assert!(rx.try_recv().is_err());

    bindings.set(NativeAction::Skill2, NativeBindingState::Unbound);
    bindings.set(NativeAction::Attack, NativeBindingState::Conflicting);
    engine.set_native_bindings(bindings);
    engine.classify(ev(Key::Digit1, Transition::Up, Origin::Real));
    assert_eq!(
        engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real)),
        Decision::Pass
    );
    assert!(rx.try_recv().is_err());
}

#[test]
fn binding_replacement_invalidates_queued_work_and_retires_old_trigger() {
    let (engine, rx) = engine();
    engine.set_focused(true);
    assert_eq!(
        engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real)),
        Decision::Suppress
    );
    let queued = rx.try_recv_authorized().unwrap();
    let mut bindings = engine.native_bindings();
    bindings.set(
        NativeAction::Skill1,
        NativeBindingState::Valid(NativeChord {
            primary: NativeControl::Keyboard(KeyboardControl::K),
            modifiers: ModifierSet::EMPTY,
        }),
    );
    engine.set_native_bindings(bindings);
    assert!(!engine.weave_gates().admits(queued.authorization_epoch()));
    engine.classify(ev(Key::Digit1, Transition::Up, Origin::Real));
    assert_eq!(
        engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real)),
        Decision::Pass
    );
}

#[test]
fn combat_requirement_replacement_invalidates_queued_work() {
    let (engine, rx) = engine();
    engine.set_focused(true);
    assert_eq!(
        engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real)),
        Decision::Suppress
    );
    let queued = rx.try_recv_authorized().unwrap();

    engine.set_combat_requirement(Action::Skill1, CombatRequirement::BASH);

    assert!(!engine.weave_gates().admits(queued.authorization_epoch()));
}

#[test]
fn exact_modified_combat_chord_outranks_unmodified_app_toggle() {
    let (engine, rx) = engine();
    engine.set_focused(true);
    let mut bindings = engine.native_bindings();
    let shifted_f1 = NativeChord {
        primary: NativeControl::Keyboard(KeyboardControl::F1),
        modifiers: ModifierSet::SHIFT,
    };
    bindings.set(NativeAction::Skill1, NativeBindingState::Valid(shifted_f1));
    bindings.set(
        NativeAction::Attack,
        NativeBindingState::Valid(NativeChord {
            primary: NativeControl::Mouse(MouseControl::Left),
            modifiers: ModifierSet::SHIFT,
        }),
    );
    engine.set_native_bindings(bindings);
    engine.classify_native(native_event(
        NativeInput::SidedModifier {
            modifier: NativeModifier::Shift,
            side: ModifierSide::Left,
        },
        Transition::Down,
    ));

    assert_eq!(
        engine.classify_native(native_event(
            NativeInput::Primary(shifted_f1.primary),
            Transition::Down,
        )),
        Decision::Suppress
    );
    assert_eq!(rx.try_recv_authorized().unwrap().action(), Action::Skill1);
}

#[test]
fn invalid_native_plan_blocks_toggle_fallback_on_the_same_primary() {
    let (engine, rx) = engine();
    engine.set_focused(true);
    let mut bindings = engine.native_bindings();
    bindings.set(
        NativeAction::Skill1,
        NativeBindingState::Valid(NativeChord {
            primary: NativeControl::Keyboard(KeyboardControl::F1),
            modifiers: ModifierSet::EMPTY,
        }),
    );
    bindings.set(NativeAction::Attack, NativeBindingState::Unbound);
    engine.set_native_bindings(bindings);

    assert_eq!(
        engine.classify(ev(Key::F1, Transition::Down, Origin::Real)),
        Decision::Pass
    );
    assert!(rx.try_recv().is_err());
}

#[test]
fn passed_primary_repeats_stay_passed_after_modifier_release() {
    let (engine, rx) = engine();
    engine.set_focused(true);
    engine.classify_native(native_event(
        NativeInput::Modifier(NativeModifier::Alt),
        Transition::Down,
    ));
    assert_eq!(
        engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real)),
        Decision::Pass
    );
    engine.classify_native(native_event(
        NativeInput::Modifier(NativeModifier::Alt),
        Transition::Up,
    ));

    assert_eq!(
        engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real)),
        Decision::Pass
    );
    assert!(rx.try_recv().is_err());

    engine.classify(ev(Key::Digit1, Transition::Up, Origin::Real));
    assert_eq!(
        engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real)),
        Decision::Suppress
    );
    assert_eq!(rx.try_recv().unwrap(), Action::Skill1);
}

#[test]
fn duplicate_required_combat_chords_reject_the_native_trigger() {
    let (engine, rx) = engine();
    engine.set_focused(true);
    let mut bindings = engine.native_bindings();
    bindings.set(NativeAction::Attack, bindings.get(NativeAction::Skill1));
    engine.set_native_bindings(bindings);

    assert_eq!(
        engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real)),
        Decision::Pass
    );
    assert!(rx.try_recv().is_err());
}

#[test]
fn dual_role_windows_primary_does_not_become_its_own_extra_modifier() {
    let (engine, rx) = engine();
    engine.set_focused(true);
    let mut bindings = engine.native_bindings();
    bindings.set(
        NativeAction::Skill1,
        NativeBindingState::Valid(NativeChord {
            primary: NativeControl::Keyboard(KeyboardControl::LeftWindows),
            modifiers: ModifierSet::EMPTY,
        }),
    );
    engine.set_native_bindings(bindings);

    assert_eq!(
        engine.classify_native(native_event(
            NativeInput::ModifierPrimary {
                modifier: NativeModifier::Command,
                side: ModifierSide::Left,
                primary: NativeControl::Keyboard(KeyboardControl::LeftWindows),
            },
            Transition::Down,
        )),
        Decision::Suppress
    );
    assert_eq!(engine.physical_modifiers(), ModifierSet::EMPTY);
    assert_eq!(rx.try_recv_authorized().unwrap().action(), Action::Skill1);
}

#[test]
fn passed_windows_primary_retains_its_modifier_role_until_release() {
    let (engine, rx) = engine();
    engine.set_focused(true);
    let windows = NativeInput::ModifierPrimary {
        modifier: NativeModifier::Command,
        side: ModifierSide::Right,
        primary: NativeControl::Keyboard(KeyboardControl::RightWindows),
    };

    assert_eq!(
        engine.classify_native(native_event(windows, Transition::Down)),
        Decision::Pass
    );
    assert_eq!(engine.physical_modifiers(), ModifierSet::COMMAND);
    assert!(rx.try_recv().is_err());

    assert_eq!(
        engine.classify_native(native_event(windows, Transition::Up)),
        Decision::Pass
    );
    assert_eq!(engine.physical_modifiers(), ModifierSet::EMPTY);
}

#[test]
fn releasing_one_modifier_side_preserves_the_other_side() {
    let (engine, _) = engine();
    let modifier = |side, transition| {
        engine.classify_native(native_event(
            NativeInput::SidedModifier {
                modifier: NativeModifier::Shift,
                side,
            },
            transition,
        ));
    };

    modifier(ModifierSide::Left, Transition::Down);
    modifier(ModifierSide::Right, Transition::Down);
    modifier(ModifierSide::Left, Transition::Up);
    assert_eq!(engine.physical_modifiers(), ModifierSet::SHIFT);

    modifier(ModifierSide::Right, Transition::Up);
    assert_eq!(engine.physical_modifiers(), ModifierSet::EMPTY);
}

fn ev(key: Key, transition: Transition, origin: Origin) -> KeyEvent {
    KeyEvent {
        key,
        transition,
        origin,
    }
}

// US1: focused interception with non-blocking hand-off.

#[test]
fn focused_bound_key_down_suppresses_and_hands_off_once() {
    let (engine, rx) = engine();
    engine.set_focused(true);

    let decision = engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real));
    assert_eq!(decision, Decision::Suppress);
    assert_eq!(rx.try_recv().ok(), Some(Action::Skill1));
    assert!(rx.try_recv().is_err());
}

#[test]
fn unbound_key_passes_through() {
    let (engine, rx) = engine();
    engine.set_focused(true);

    // Q is not in the default bindings.
    let decision = engine.classify(ev(Key::Q, Transition::Down, Origin::Real));
    assert_eq!(decision, Decision::Pass);
    assert!(rx.try_recv().is_err());
}

#[test]
fn unfocused_never_intercepts() {
    let (engine, rx) = engine();
    engine.set_focused(false);

    let decision = engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real));
    assert_eq!(decision, Decision::Pass);
    assert!(rx.try_recv().is_err());
}

#[test]
fn inactive_game_never_intercepts_even_when_focused() {
    let (engine, rx) = engine();
    engine.set_focused(true);
    engine.set_game_active(false);
    assert_eq!(
        engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real)),
        Decision::Pass
    );
    assert!(rx.try_recv().is_err());
}

#[test]
fn game_exit_closes_menu_gate_and_clears_held_keys_for_restart() {
    let (engine, rx) = engine();
    engine.set_focused(true);
    engine.set_menu_gated(false);

    assert_eq!(
        engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real)),
        Decision::Suppress
    );
    assert_eq!(rx.try_recv().ok(), Some(Action::Skill1));

    engine.set_menu_gated(true);
    engine.set_game_active(false);
    assert!(engine.is_menu_gated());
    assert_eq!(
        engine.classify(ev(Key::Digit1, Transition::Up, Origin::Real)),
        Decision::Pass
    );

    engine.set_game_active(true);
    assert!(engine.is_life_gated());
    engine.set_life_gated(false);
    assert!(engine.is_roll_gated());
    engine.set_roll_gated(false);
    engine.set_world_gated(false);
    engine.set_travel_gated(false);
    engine.set_menu_gated(false);
    assert_eq!(
        engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real)),
        Decision::Suppress
    );
    assert_eq!(rx.try_recv().ok(), Some(Action::Skill1));
}

#[test]
fn pass_through_release_while_unfocused_retires_held_key() {
    let (engine, rx) = engine();
    engine.set_focused(true);
    engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real));
    assert_eq!(rx.try_recv().ok(), Some(Action::Skill1));

    engine.set_focused(false);
    assert_eq!(
        engine.classify(ev(Key::Digit1, Transition::Up, Origin::Real)),
        Decision::Pass
    );
    engine.set_focused(true);
    engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real));
    assert_eq!(rx.try_recv().ok(), Some(Action::Skill1));
}

#[test]
fn bound_key_up_is_suppressed_and_hands_off_nothing() {
    let (engine, rx) = engine();
    engine.set_focused(true);

    assert_eq!(
        engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real)),
        Decision::Suppress
    );
    let _ = rx.try_recv();
    assert_eq!(
        engine.classify(ev(Key::Digit1, Transition::Up, Origin::Real)),
        Decision::Suppress
    );
    assert!(rx.try_recv().is_err());
}

#[test]
fn auto_repeat_down_hands_off_only_once() {
    let (engine, rx) = engine();
    engine.set_focused(true);

    engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real));
    engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real)); // repeat
    engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real)); // repeat

    assert_eq!(rx.try_recv().ok(), Some(Action::Skill1));
    assert!(rx.try_recv().is_err());

    // After release, a fresh press hands off again.
    engine.classify(ev(Key::Digit1, Transition::Up, Origin::Real));
    engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real));
    assert_eq!(rx.try_recv().ok(), Some(Action::Skill1));
}

#[test]
fn full_channel_drops_without_blocking() {
    let (engine, _rx) = InputEngine::new(BindingTable::default(), 1);
    install_native(&engine);
    engine.set_game_active(true);
    engine.set_life_gated(false);
    engine.set_roll_gated(false);
    engine.set_world_gated(false);
    engine.set_travel_gated(false);
    engine.set_menu_gated(false);
    engine.set_focused(true);

    // First press fills the capacity-1 channel; further distinct presses must not
    // block and must still suppress.
    engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real));
    engine.classify(ev(Key::Digit1, Transition::Up, Origin::Real));
    let decision = engine.classify(ev(Key::Digit2, Transition::Down, Origin::Real));
    assert_eq!(decision, Decision::Suppress);
}

// US2: recursion breaking.

#[test]
fn self_originated_event_is_never_intercepted() {
    let (engine, rx) = engine();
    engine.set_focused(true);

    let decision = engine.classify(ev(Key::Digit1, Transition::Down, Origin::SelfOriginated));
    assert_eq!(decision, Decision::Pass);
    assert!(rx.try_recv().is_err());

    // A later real press of the same key is still intercepted.
    let decision = engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real));
    assert_eq!(decision, Decision::Suppress);
    assert_eq!(rx.try_recv().ok(), Some(Action::Skill1));
}

#[test]
fn mock_backend_records_synthesis() {
    let backend = MockBackend::new();
    backend.synthesize(Key::R, Transition::Down).unwrap();
    backend.synthesize(Key::R, Transition::Up).unwrap();
    assert_eq!(
        backend.synthesized(),
        vec![(Key::R, Transition::Down), (Key::R, Transition::Up)]
    );
}

// US3: suspend semantics.

#[test]
fn suspended_non_exempt_key_passes_through() {
    let (engine, rx) = engine();
    engine.set_focused(true);
    engine.set_suspended(true);

    let decision = engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real));
    assert_eq!(decision, Decision::Pass);
    assert!(rx.try_recv().is_err());
}

#[test]
fn suspended_exempt_key_is_intercepted() {
    let (engine, rx) = engine();
    engine.set_focused(true);
    engine.set_suspended(true);

    let decision = engine.classify(ev(Key::F1, Transition::Down, Origin::Real));
    assert_eq!(decision, Decision::Suppress);
    assert_eq!(rx.try_recv().ok(), Some(Action::ToggleSuspend));
}

#[test]
fn resume_restores_interception() {
    let (engine, rx) = engine();
    engine.set_focused(true);
    engine.set_suspended(true);
    engine.set_suspended(false);

    let decision = engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real));
    assert_eq!(decision, Decision::Pass);
    assert!(rx.try_recv().is_err());
    assert!(engine.is_world_gated());
    assert!(engine.is_travel_gated());
    assert_eq!(engine.safety_refresh_generation(), 1);

    engine.set_world_gated(false);
    engine.set_travel_gated(false);
    engine.classify(ev(Key::Digit1, Transition::Up, Origin::Real));
    let decision = engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real));
    assert_eq!(decision, Decision::Suppress);
    assert_eq!(rx.try_recv().ok(), Some(Action::Skill1));

    engine.set_suspended(false);
    assert_eq!(engine.safety_refresh_generation(), 1);
}

// US4: bindings.

#[test]
fn defaults_match_section_6_4() {
    let table = BindingTable::default();
    assert_eq!(table.key_for(Action::ToggleSuspend), Key::F1);
    assert_eq!(table.key_for(Action::ToggleFishing), Key::F2);
    assert_eq!(table.key_for(Action::ToggleAutoPotion), Key::F3);
    assert_eq!(table.to_settings_map().len(), 3);
    assert!(Action::ToggleSuspend.suspend_exempt());
    assert!(Action::ToggleFishing.suspend_exempt());
    assert!(!Action::Skill1.suspend_exempt());
}

#[test]
fn rebind_to_free_key_persists_through_settings() {
    let dir = tempfile::tempdir().unwrap();
    let (first, _rx) = engine();

    first.rebind(Action::ToggleSuspend, Key::Q).unwrap();
    assert_eq!(first.bindings().key_for(Action::ToggleSuspend), Key::Q);

    let mut settings = Settings::default();
    first.store_bindings(&mut settings);
    eso_weave::config::save(dir.path(), &settings).unwrap();

    let loaded = eso_weave::config::load(dir.path());
    let (second, _rx2) = engine();
    let notices = second.load_bindings(&loaded.settings);
    assert!(notices.is_empty());
    assert_eq!(second.bindings().key_for(Action::ToggleSuspend), Key::Q);
}

#[test]
fn colliding_rebind_is_rejected() {
    let (engine, _rx) = engine();
    let result = engine.rebind(Action::ToggleSuspend, Key::F2);
    assert!(result.is_err());
    // Both bindings unchanged.
    assert_eq!(engine.bindings().key_for(Action::ToggleSuspend), Key::F1);
    assert_eq!(engine.bindings().key_for(Action::ToggleFishing), Key::F2);
}

#[test]
fn combat_rebind_is_rejected_below_the_settings_interface() {
    let (engine, _rx) = engine();
    assert!(engine.rebind(Action::Skill1, Key::Q).is_err());
    assert!(!engine
        .bindings()
        .to_settings_map()
        .contains_key(Action::Skill1.as_str()));
}

#[test]
fn persisted_conflict_falls_back_to_defaults_with_notice() {
    // Two actions mapped to the same key.
    let mut raw = BTreeMap::new();
    raw.insert("toggle_suspend".to_string(), "f2".to_string());
    raw.insert("toggle_fishing".to_string(), "f2".to_string());
    let (table, notices) = BindingTable::from_settings_map(&raw);

    assert!(!notices.is_empty());
    // Affected actions fall back to their defaults.
    assert_eq!(table.key_for(Action::ToggleSuspend), Key::F1);
    assert_eq!(table.key_for(Action::ToggleFishing), Key::F2);
}

#[test]
fn persisted_unknown_key_falls_back_with_notice() {
    let mut raw = BTreeMap::new();
    raw.insert("toggle_suspend".to_string(), "not_a_key".to_string());
    let (table, notices) = BindingTable::from_settings_map(&raw);

    assert!(!notices.is_empty());
    assert_eq!(table.key_for(Action::ToggleSuspend), Key::F1);
}

// Slice 032: the menu gate. Constitution principle II surface.
//
// The gate edits the safety-critical interception decision, so its correctness is
// established by exhaustive comparison rather than by chosen scenarios: the whole
// risk is the combination nobody thought to try.

/// Every input the interception decision reads, as a closed set.
fn decision_inputs() -> Vec<(Key, Transition, Origin, bool, bool, bool)> {
    // A weave key (not exempt), an exempt toggle key, and an unbound key.
    let weave_key = Key::Digit1;
    let exempt_key = BindingTable::default().key_for(Action::ToggleSuspend);
    let unbound = Key::Q;
    let keys = [weave_key, exempt_key, unbound];

    let mut out = Vec::new();
    for key in keys {
        for transition in [Transition::Down, Transition::Up] {
            for origin in [Origin::Real, Origin::SelfOriginated] {
                for focused in [false, true] {
                    for suspended in [false, true] {
                        for active in [false, true] {
                            out.push((key, transition, origin, focused, suspended, active));
                        }
                    }
                }
            }
        }
    }
    out
}

fn classify_with(input: (Key, Transition, Origin, bool, bool, bool), gated: bool) -> Decision {
    let (key, transition, origin, focused, suspended, active) = input;
    let (engine, _rx) = engine();
    engine.set_focused(focused);
    engine.set_suspended(suspended);
    engine.set_menu_gated(gated);
    for action in Action::ALL {
        engine.set_action_active(action, active);
    }
    engine.classify(ev(key, transition, origin))
}

#[test]
fn menu_gate_can_only_relax_interception_never_tighten_it() {
    // FR-015. For every point in the decision's input space, the gated outcome is
    // either identical to the ungated one or more permissive. There is no input
    // for which turning the gate on causes a key to be suppressed.
    let inputs = decision_inputs();
    // 3 keys (weave, exempt toggle, unbound) x 2 transitions x 2 origins x
    // focused x suspended x active. Pinned rather than bounded, so a change that
    // silently shrinks the space fails here instead of quietly proving less.
    assert_eq!(inputs.len(), 3 * 2 * 2 * 2 * 2 * 2);

    let mut relaxed = 0;
    for input in inputs {
        let ungated = classify_with(input, false);
        let gated = classify_with(input, true);

        if ungated == Decision::Pass {
            assert_eq!(
                gated,
                Decision::Pass,
                "gate turned a pass into {gated:?} for {input:?}"
            );
        }
        if gated != ungated {
            assert_eq!(
                (ungated, gated),
                (Decision::Suppress, Decision::Pass),
                "the only permitted difference is suppress becoming pass, at {input:?}"
            );
            relaxed += 1;
        }
    }
    assert!(
        relaxed > 0,
        "the gate must actually change something, or this test proves nothing"
    );
}

#[test]
fn focus_scoping_is_unconditional_regardless_of_the_gate() {
    // FR-016. An unfocused game window passes everything, whatever else is true.
    for input in decision_inputs() {
        let (_, _, _, focused, _, _) = input;
        if focused {
            continue;
        }
        for gated in [false, true] {
            assert_eq!(
                classify_with(input, gated),
                Decision::Pass,
                "unfocused window suppressed a key at {input:?} gated={gated}"
            );
        }
    }
}

#[test]
fn a_fresh_engine_is_menu_gated_until_valid_evidence_arrives() {
    // S061 FR-010. Unknown startup evidence cannot authorize synthesis.
    let (engine, _rx) = InputEngine::new(BindingTable::default(), 64);
    assert!(engine.is_menu_gated());
}

#[test]
fn the_gate_exempts_the_toggle_hotkeys() {
    // FR-010. The operator keeps control from inside a menu, exactly as they do
    // while manually suspended.
    let exempt_key = BindingTable::default().key_for(Action::ToggleSuspend);
    let (engine, _rx) = engine();
    engine.set_focused(true);
    engine.set_action_active(Action::ToggleSuspend, true);
    engine.set_menu_gated(true);
    assert_eq!(
        engine.classify(ev(exempt_key, Transition::Down, Origin::Real)),
        Decision::Suppress,
        "the suspend hotkey must still be intercepted while gated"
    );
}

#[test]
fn life_gate_defaults_closed_passes_weaves_and_exempts_toggles() {
    let (engine, rx) = InputEngine::new(BindingTable::default(), 16);
    engine.set_game_active(true);
    engine.set_focused(true);
    assert!(engine.is_life_gated());
    assert_eq!(
        engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real)),
        Decision::Pass
    );
    assert!(rx.try_recv().is_err());

    let suspend = BindingTable::default().key_for(Action::ToggleSuspend);
    assert_eq!(
        engine.classify(ev(suspend, Transition::Down, Origin::Real)),
        Decision::Suppress
    );
    assert_eq!(rx.try_recv().ok(), Some(Action::ToggleSuspend));
}

#[test]
fn roll_gate_defaults_closed_passes_physical_weaves_and_exempts_toggles() {
    let (engine, rx) = InputEngine::new(BindingTable::default(), 4);
    install_native(&engine);
    engine.set_game_active(true);
    engine.set_focused(true);
    engine.set_life_gated(false);
    engine.set_world_gated(false);
    engine.set_travel_gated(false);
    engine.set_menu_gated(false);
    assert!(engine.is_roll_gated());

    let skill = Key::Digit1;
    assert_eq!(
        engine.classify(ev(skill, Transition::Down, Origin::Real)),
        Decision::Pass
    );
    assert!(rx.try_recv().is_err());

    let toggle = engine.bindings().key_for(Action::ToggleSuspend);
    assert_eq!(
        engine.classify(ev(toggle, Transition::Down, Origin::Real)),
        Decision::Suppress
    );
    assert_eq!(rx.try_recv().unwrap(), Action::ToggleSuspend);

    engine.set_roll_gated(false);
    assert_eq!(
        engine.classify(ev(skill, Transition::Up, Origin::Real)),
        Decision::Pass
    );
    assert_eq!(
        engine.classify(ev(skill, Transition::Down, Origin::Real)),
        Decision::Suppress
    );
    assert_eq!(rx.try_recv().unwrap(), Action::Skill1);
}

#[test]
fn life_gated_press_keeps_its_release_passed_after_recovery() {
    let (engine, rx) = engine();
    engine.set_focused(true);
    engine.set_life_gated(true);

    assert_eq!(
        engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real)),
        Decision::Pass
    );
    engine.set_life_gated(false);
    assert_eq!(
        engine.classify(ev(Key::Digit1, Transition::Up, Origin::Real)),
        Decision::Pass,
        "a pass-through key-down must keep its matching key-up pass-through"
    );
    assert!(rx.try_recv().is_err());

    assert_eq!(
        engine.classify(ev(Key::Digit1, Transition::Down, Origin::Real)),
        Decision::Suppress,
        "the next complete press may be intercepted after recovery"
    );
    assert_eq!(rx.try_recv().ok(), Some(Action::Skill1));
}

#[test]
fn travel_gate_passes_physical_skills_exempts_toggles_and_does_not_replay() {
    let (engine, rx) = engine();
    engine.set_focused(true);
    engine.set_travel_gated(true);
    let skill = Key::Digit1;
    assert_eq!(
        engine.classify(ev(skill, Transition::Down, Origin::Real)),
        Decision::Pass
    );
    assert!(rx.try_recv().is_err());

    let toggle = engine.bindings().key_for(Action::ToggleSuspend);
    assert_eq!(
        engine.classify(ev(toggle, Transition::Down, Origin::Real)),
        Decision::Suppress
    );
    assert_eq!(rx.try_recv().ok(), Some(Action::ToggleSuspend));

    engine.set_travel_gated(false);
    assert!(rx.try_recv().is_err(), "recovery must not replay the skill");
    assert_eq!(
        engine.classify(ev(skill, Transition::Up, Origin::Real)),
        Decision::Pass
    );
    assert_eq!(
        engine.classify(ev(skill, Transition::Down, Origin::Real)),
        Decision::Suppress
    );
    assert_eq!(rx.try_recv().ok(), Some(Action::Skill1));
}

#[test]
fn s060_queued_weave_epoch_is_invalid_after_each_runtime_gate_closes() {
    fn assert_invalidated(close: impl FnOnce(&InputEngine), reopen: impl FnOnce(&InputEngine)) {
        let (input, rx) = engine();
        input.set_focused(true);

        assert_eq!(
            input.classify(ev(Key::Digit1, Transition::Down, Origin::Real)),
            Decision::Suppress
        );
        let queued = rx
            .try_recv_authorized()
            .expect("the weave action must be queued");
        assert_eq!(queued.action(), Action::Skill1);
        assert!(input.weave_gates().admits(queued.authorization_epoch()));

        close(&input);
        reopen(&input);
        assert!(!input.weave_gates().is_gated());
        assert!(
            !input.weave_gates().admits(queued.authorization_epoch()),
            "closing and reopening a gate must not revive queued work"
        );
    }

    assert_invalidated(
        |input| input.set_game_active(false),
        |input| {
            input.set_game_active(true);
            input.set_life_gated(false);
            input.set_roll_gated(false);
            input.set_world_gated(false);
            input.set_travel_gated(false);
            input.set_menu_gated(false);
        },
    );
    assert_invalidated(
        |input| input.set_focused(false),
        |input| input.set_focused(true),
    );
    assert_invalidated(
        |input| input.set_suspended(true),
        |input| {
            input.set_suspended(false);
            input.set_world_gated(false);
            input.set_travel_gated(false);
        },
    );
    assert_invalidated(
        |input| input.set_menu_gated(true),
        |input| input.set_menu_gated(false),
    );
    assert_invalidated(
        |input| input.set_life_gated(true),
        |input| input.set_life_gated(false),
    );
    assert_invalidated(
        |input| input.set_roll_gated(true),
        |input| input.set_roll_gated(false),
    );
    assert_invalidated(
        |input| input.set_world_gated(true),
        |input| input.set_world_gated(false),
    );
    assert_invalidated(
        |input| input.set_travel_gated(true),
        |input| input.set_travel_gated(false),
    );
}

#[test]
fn s067_death_epoch_advances_once_per_open_to_closed_life_transition() {
    let (input, _rx) = engine();
    input.set_life_gated(false);
    assert_eq!(input.death_epoch(), 0);

    input.set_life_gated(true);
    assert_eq!(input.death_epoch(), 1);
    input.set_life_gated(true);
    assert_eq!(
        input.death_epoch(),
        1,
        "duplicate unsafe evidence is idempotent"
    );

    input.set_life_gated(false);
    input.set_life_gated(true);
    assert_eq!(input.death_epoch(), 2);
}

#[test]
fn s060_toggle_handoff_remains_identifiable_across_authorization_epochs() {
    let (input, rx) = engine();
    input.set_focused(true);
    input.set_suspended(true);

    assert_eq!(
        input.classify(ev(Key::F1, Transition::Down, Origin::Real)),
        Decision::Suppress
    );
    let queued = rx
        .try_recv_authorized()
        .expect("the suspend toggle must be queued");
    assert_eq!(queued.action(), Action::ToggleSuspend);
    assert!(queued.action().is_app_toggle());
}

#[test]
fn s060_fishing_authorization_excludes_roll_but_tracks_shared_runtime_gates() {
    let (input, _rx) = engine();
    input.set_focused(true);
    let gates = input.fishing_gates();
    let admitted = gates.current_epoch();

    input.set_roll_gated(true);
    input.set_roll_gated(false);
    assert!(gates.admits(admitted), "roll is not a Fishing gate");

    input.set_menu_gated(true);
    input.set_menu_gated(false);
    assert!(
        !gates.admits(admitted),
        "a transient applicable closure must invalidate Fishing work"
    );
}

#[test]
fn ungating_restores_the_previous_decision_everywhere() {
    // FR-012. A gate that engages but never releases is worse than no gate.
    for input in decision_inputs() {
        let before = classify_with(input, false);

        let (key, transition, origin, focused, suspended, active) = input;
        let (engine, _rx) = engine();
        engine.set_focused(focused);
        engine.set_suspended(suspended);
        for action in Action::ALL {
            engine.set_action_active(action, active);
        }
        engine.set_menu_gated(true);
        engine.classify(ev(key, transition, origin));
        engine.set_menu_gated(false);
        if transition == Transition::Down {
            engine.classify(ev(key, Transition::Up, origin));
        }

        assert_eq!(
            engine.classify(ev(key, transition, origin)),
            before,
            "ungating left residue at {input:?}"
        );
    }
}

#[test]
fn s102_autonomous_executor_owns_modified_mouse_chords_and_balances_cleanup() {
    let (engine, _rx) = engine();
    engine.set_focused(true);
    let chord = NativeChord {
        primary: NativeControl::Mouse(MouseControl::Button4),
        modifiers: ModifierSet::CONTROL | ModifierSet::SHIFT,
    };
    let mut bindings = engine.native_bindings();
    bindings.set(NativeAction::Quickslot, NativeBindingState::Valid(chord));
    engine.set_native_bindings(bindings);

    let backend = MockBackend::new();
    let primaries = backend.synthesized_native.clone();
    let modifiers = backend.synthesized_modifiers.clone();
    let mut executor = NativeActionExecutor::new(backend, engine.fishing_gates());

    assert!(executor.execute_current(NativeAction::Quickslot));
    assert_eq!(
        *primaries.lock().unwrap(),
        vec![
            (chord.primary, Transition::Down),
            (chord.primary, Transition::Up)
        ]
    );
    assert_eq!(
        *modifiers.lock().unwrap(),
        vec![
            (NativeModifier::Control, Transition::Down),
            (NativeModifier::Shift, Transition::Down),
            (NativeModifier::Shift, Transition::Up),
            (NativeModifier::Control, Transition::Up),
        ]
    );
}

#[test]
fn s102_autonomous_executor_rejects_every_non_valid_state_and_stale_epoch() {
    let (engine, _rx) = engine();
    engine.set_focused(true);
    let backend = MockBackend::new();
    let primaries = backend.synthesized_native.clone();
    let mut executor = NativeActionExecutor::new(backend, engine.fishing_gates());

    for state in [
        NativeBindingState::Unavailable,
        NativeBindingState::Unbound,
        NativeBindingState::Conflicting,
        NativeBindingState::Unsupported,
    ] {
        let mut bindings = engine.native_bindings();
        bindings.set(NativeAction::Interact, state);
        engine.set_native_bindings(bindings);
        assert!(!executor.execute_current(NativeAction::Interact));
    }

    let mut bindings = engine.native_bindings();
    bindings.set(
        NativeAction::Interact,
        NativeBindingState::Valid(NativeChord {
            primary: NativeControl::Keyboard(KeyboardControl::E),
            modifiers: ModifierSet::EMPTY,
        }),
    );
    engine.set_native_bindings(bindings);
    let stale = executor.current_epoch();
    bindings.set(
        NativeAction::Interact,
        NativeBindingState::Valid(NativeChord {
            primary: NativeControl::Keyboard(KeyboardControl::R),
            modifiers: ModifierSet::EMPTY,
        }),
    );
    engine.set_native_bindings(bindings);
    assert!(!executor.execute(NativeAction::Interact, stale));
    assert!(primaries.lock().unwrap().is_empty());
}

#[test]
fn s102_autonomous_executor_rejects_extra_physical_modifiers_and_never_releases_them() {
    let (engine, _rx) = engine();
    engine.set_focused(true);
    let mut bindings = engine.native_bindings();
    bindings.set(
        NativeAction::Interact,
        NativeBindingState::Valid(NativeChord {
            primary: NativeControl::Keyboard(KeyboardControl::E),
            modifiers: ModifierSet::EMPTY,
        }),
    );
    engine.set_native_bindings(bindings);
    engine.classify_native(native_event(
        NativeInput::Modifier(NativeModifier::Alt),
        Transition::Down,
    ));

    let backend = MockBackend::new();
    let primaries = backend.synthesized_native.clone();
    let modifiers = backend.synthesized_modifiers.clone();
    let mut executor = NativeActionExecutor::new(backend, engine.fishing_gates());
    assert!(!executor.execute_current(NativeAction::Interact));
    assert!(primaries.lock().unwrap().is_empty());
    assert!(modifiers.lock().unwrap().is_empty());
}

#[test]
fn s102_autonomous_executor_emits_wheel_primaries_without_invented_release() {
    let (engine, _rx) = engine();
    engine.set_focused(true);
    let wheel = NativeControl::Mouse(MouseControl::WheelUp);
    let mut bindings = engine.native_bindings();
    bindings.set(
        NativeAction::Quickslot,
        NativeBindingState::Valid(NativeChord {
            primary: wheel,
            modifiers: ModifierSet::EMPTY,
        }),
    );
    engine.set_native_bindings(bindings);

    let backend = MockBackend::new();
    let primaries = backend.synthesized_native.clone();
    let mut executor = NativeActionExecutor::new(backend, engine.fishing_gates());
    assert!(executor.execute_current(NativeAction::Quickslot));
    assert_eq!(*primaries.lock().unwrap(), vec![(wheel, Transition::Down)]);
}

#[test]
fn s102_autonomous_executor_retries_owned_primary_cleanup_before_new_work() {
    let (engine, _rx) = engine();
    engine.set_focused(true);
    let primary = NativeControl::Keyboard(KeyboardControl::E);
    let mut bindings = engine.native_bindings();
    bindings.set(
        NativeAction::Interact,
        NativeBindingState::Valid(NativeChord {
            primary,
            modifiers: ModifierSet::EMPTY,
        }),
    );
    engine.set_native_bindings(bindings);

    let backend = FailTwoPrimaryReleasesBackend {
        inner: MockBackend::new(),
        remaining_failures: AtomicUsize::new(2),
    };
    let primaries = backend.inner.synthesized_native.clone();
    let mut executor = NativeActionExecutor::new(backend, engine.autonomous_gates());

    assert!(executor.execute_current(NativeAction::Interact));
    assert!(!executor.execute_current(NativeAction::Interact));
    assert_eq!(
        *primaries.lock().unwrap(),
        vec![
            (primary, Transition::Down),
            (primary, Transition::Up),
            (primary, Transition::Up),
        ]
    );

    assert!(executor.execute_current(NativeAction::Interact));
    assert_eq!(
        *primaries.lock().unwrap(),
        vec![
            (primary, Transition::Down),
            (primary, Transition::Up),
            (primary, Transition::Up),
            (primary, Transition::Up),
            (primary, Transition::Down),
            (primary, Transition::Up),
        ]
    );
}
