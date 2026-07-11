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
    InputEngine::new(BindingTable::default(), 64)
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
