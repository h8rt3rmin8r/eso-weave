//! Safety-critical tests for the Input Engine core via the mock backend.

use std::collections::BTreeMap;

use eso_weave::config::Settings;
use eso_weave::input::action::Action;
use eso_weave::input::key::Key;
use eso_weave::input::mock::MockBackend;
use eso_weave::input::{
    BindingTable, Decision, InputBackend, InputEngine, KeyEvent, Origin, Transition,
};

fn engine() -> (InputEngine, eso_weave::input::ActionReceiver) {
    let pair = InputEngine::new(BindingTable::default(), 64);
    pair.0.set_game_active(true);
    pair
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
fn game_exit_clears_menu_gate_and_held_keys_for_restart() {
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
    assert!(!engine.is_menu_gated());
    assert_eq!(
        engine.classify(ev(Key::Digit1, Transition::Up, Origin::Real)),
        Decision::Pass
    );

    engine.set_game_active(true);
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
    engine.set_game_active(true);
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
    assert_eq!(decision, Decision::Suppress);
    assert_eq!(rx.try_recv().ok(), Some(Action::Skill1));
}

// US4: bindings.

#[test]
fn defaults_match_section_6_4() {
    let table = BindingTable::default();
    assert_eq!(table.key_for(Action::Skill1), Key::Digit1);
    assert_eq!(table.key_for(Action::Ultimate), Key::R);
    assert_eq!(table.key_for(Action::Synergy), Key::X);
    assert_eq!(table.key_for(Action::ToggleSuspend), Key::F1);
    assert_eq!(table.key_for(Action::ToggleFishing), Key::F2);
    assert!(Action::ToggleSuspend.suspend_exempt());
    assert!(Action::ToggleFishing.suspend_exempt());
    assert!(!Action::Skill1.suspend_exempt());
}

#[test]
fn rebind_to_free_key_persists_through_settings() {
    let dir = tempfile::tempdir().unwrap();
    let (first, _rx) = engine();

    // Q is unbound by default, so binding Skill1 to Q is accepted.
    first.rebind(Action::Skill1, Key::Q).unwrap();
    assert_eq!(first.bindings().key_for(Action::Skill1), Key::Q);

    let mut settings = Settings::default();
    first.store_bindings(&mut settings);
    eso_weave::config::save(dir.path(), &settings).unwrap();

    let loaded = eso_weave::config::load(dir.path());
    let (second, _rx2) = engine();
    let notices = second.load_bindings(&loaded.settings);
    assert!(notices.is_empty());
    assert_eq!(second.bindings().key_for(Action::Skill1), Key::Q);
}

#[test]
fn colliding_rebind_is_rejected() {
    let (engine, _rx) = engine();
    // Ultimate is R, Synergy is X. Rebind Ultimate to X (used by Synergy).
    let result = engine.rebind(Action::Ultimate, Key::X);
    assert!(result.is_err());
    // Both bindings unchanged.
    assert_eq!(engine.bindings().key_for(Action::Ultimate), Key::R);
    assert_eq!(engine.bindings().key_for(Action::Synergy), Key::X);
}

#[test]
fn persisted_conflict_falls_back_to_defaults_with_notice() {
    // Two actions mapped to the same key.
    let mut raw = BTreeMap::new();
    raw.insert("ultimate".to_string(), "x".to_string());
    raw.insert("synergy".to_string(), "x".to_string());
    let (table, notices) = BindingTable::from_settings_map(&raw);

    assert!(!notices.is_empty());
    // Affected actions fall back to their defaults.
    assert_eq!(table.key_for(Action::Ultimate), Key::R);
    assert_eq!(table.key_for(Action::Synergy), Key::X);
}

#[test]
fn persisted_unknown_key_falls_back_with_notice() {
    let mut raw = BTreeMap::new();
    raw.insert("ultimate".to_string(), "not_a_key".to_string());
    let (table, notices) = BindingTable::from_settings_map(&raw);

    assert!(!notices.is_empty());
    assert_eq!(table.key_for(Action::Ultimate), Key::R);
}

// Slice 032: the menu gate. Constitution principle II surface.
//
// The gate edits the safety-critical interception decision, so its correctness is
// established by exhaustive comparison rather than by chosen scenarios: the whole
// risk is the combination nobody thought to try.

/// Every input the interception decision reads, as a closed set.
fn decision_inputs() -> Vec<(Key, Transition, Origin, bool, bool, bool)> {
    // A weave key (not exempt), an exempt toggle key, and an unbound key.
    let weave_key = BindingTable::default().key_for(Action::Skill1);
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
fn a_fresh_engine_is_ungated() {
    // FR-013. The default is the value that reproduces the pre-feature behavior,
    // so an addon too old to publish the signal changes nothing.
    let (engine, _rx) = engine();
    assert!(!engine.is_menu_gated());
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

        assert_eq!(
            engine.classify(ev(key, transition, origin)),
            before,
            "ungating left residue at {input:?}"
        );
    }
}
