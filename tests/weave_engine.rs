//! Engine tests for the Weave Engine: cooldown, mapping, inactive pass-through,
//! and persistence (spec user stories US2, US3, US4).

use eso_weave::config::{self, Settings};
use eso_weave::input::{
    Action, BindingTable, Decision, InputEngine, Key, KeyEvent, Origin, Transition,
};
use eso_weave::pixelbus::{ActiveBar, WeaponBarSignal, WeaponClass};
use eso_weave::weave::types::{TimingConfig, WeaveType};
use eso_weave::weave::{effective_timing, heavy_preset, MockSink, WeaveConfig, WeaveEngine};

fn down(key: Key) -> KeyEvent {
    KeyEvent {
        key,
        transition: Transition::Down,
        origin: Origin::Real,
    }
}

// US2: cooldown gating and action mapping.

#[test]
fn cooldown_gates_repeated_weaves() {
    let mut engine = WeaveEngine::new(WeaveConfig::default());
    let mut sink = MockSink::new();

    sink.set_now(0);
    engine.handle(Action::Skill1, &mut sink);
    let after_first = sink.log.len();
    assert!(after_first > 0, "first weave should run");

    // Within the 500 ms cooldown: dropped.
    sink.set_now(100);
    engine.handle(Action::Skill1, &mut sink);
    assert_eq!(
        sink.log.len(),
        after_first,
        "request inside cooldown is dropped"
    );

    // After the cooldown: runs again.
    sink.set_now(600);
    engine.handle(Action::Skill1, &mut sink);
    assert!(sink.log.len() > after_first, "request after cooldown runs");
}

#[test]
fn toggle_actions_run_no_weave() {
    let mut engine = WeaveEngine::new(WeaveConfig::default());
    let mut sink = MockSink::new();

    engine.handle(Action::ToggleSuspend, &mut sink);
    engine.handle(Action::ToggleFishing, &mut sink);
    assert!(sink.log.is_empty());
}

#[test]
fn each_skill_action_maps_to_a_slot() {
    let engine = WeaveEngine::new(WeaveConfig::default());
    assert_eq!(
        engine.slot_for_action(Action::Skill1).map(|s| s.index),
        Some(1)
    );
    assert_eq!(
        engine.slot_for_action(Action::Ultimate).map(|s| s.index),
        Some(6)
    );
    assert_eq!(
        engine.slot_for_action(Action::Synergy).map(|s| s.index),
        Some(7)
    );
    assert!(engine.slot_for_action(Action::ToggleSuspend).is_none());
}

// US3: inactive slots pass through via the input engine.

#[test]
fn inactive_slot_key_passes_through() {
    let (input, _rx) = InputEngine::new(BindingTable::default(), 16);
    input.set_focused(true);

    // Default config: Ultimate (slot 6, key R) is inactive; Skill1 is active.
    let weave = WeaveEngine::new(WeaveConfig::default());
    weave.apply_activity(&input);

    assert_eq!(input.classify(down(Key::R)), Decision::Pass);
    assert_eq!(input.classify(down(Key::Digit1)), Decision::Suppress);
}

#[test]
fn activating_slot_restores_interception() {
    let (input, _rx) = InputEngine::new(BindingTable::default(), 16);
    input.set_focused(true);

    let mut config = WeaveConfig::default();
    config.slots[5].active = true; // slot index 6 (Ultimate)
    let weave = WeaveEngine::new(config);
    weave.apply_activity(&input);

    assert_eq!(input.classify(down(Key::R)), Decision::Suppress);
}

// US4: persistence and timing fallback.

#[test]
fn config_round_trips_through_settings() {
    let dir = tempfile::tempdir().unwrap();

    let mut config = WeaveConfig::default();
    config.timing.d_weave = 77;
    config.slots[0].weave_type = WeaveType::HeavyAttack;
    config.slots[0].overrides.d_heavy = Some(1500);
    let engine = WeaveEngine::new(config);

    let mut settings = Settings::default();
    engine.store(&mut settings);
    config::save(dir.path(), &settings).unwrap();

    let loaded = config::load(dir.path());
    let mut restored = WeaveEngine::new(WeaveConfig::default());
    let notices = restored.load(&loaded.settings);

    assert!(notices.is_empty());
    assert_eq!(restored.config().timing.d_weave, 77);
    assert_eq!(
        restored.config().slots[0].weave_type,
        WeaveType::HeavyAttack
    );
    assert_eq!(restored.config().slots[0].overrides.d_heavy, Some(1500));
}

#[test]
fn out_of_range_timing_falls_back_with_notice() {
    let settings = Settings {
        timing: serde_json::json!({
            "global_cooldown": 10_000_000,
            "d_weave": 50,
            "d_heavy": 1000,
            "d_bash": 125
        }),
        ..Default::default()
    };

    let mut engine = WeaveEngine::new(WeaveConfig::default());
    let notices = engine.load(&settings);

    assert!(!notices.is_empty());
    assert_eq!(engine.config().timing.global_cooldown, 500);
}

#[test]
fn unknown_weave_type_falls_back_with_notice() {
    let settings = Settings {
        skills: serde_json::json!({
            "slot1": { "weave_type": "not_a_type", "active": true }
        }),
        ..Default::default()
    };

    let mut engine = WeaveEngine::new(WeaveConfig::default());
    let notices = engine.load(&settings);

    assert!(!notices.is_empty());
    // Falls back to the default weave type for slot 1.
    assert_eq!(engine.config().slots[0].weave_type, WeaveType::LightAttack);
}

// Slice 014: per-bar timing selection and weapon-class presets.

fn timing(global: u32, weave: u32, heavy: u32, bash: u32) -> TimingConfig {
    TimingConfig {
        global_cooldown: global,
        d_weave: weave,
        d_heavy: heavy,
        d_bash: bash,
    }
}

#[test]
fn effective_timing_selects_profile_by_active_bar() {
    let front = timing(500, 50, 640, 125);
    let back = timing(500, 50, 1360, 125);

    // Front and Unknown use the front profile; Back uses the back profile.
    let f = effective_timing(
        &front,
        &back,
        false,
        ActiveBar::Front,
        WeaponClass::Unknown,
        WeaponClass::Unknown,
    );
    assert_eq!(f.d_heavy, 640);
    let u = effective_timing(
        &front,
        &back,
        false,
        ActiveBar::Unknown,
        WeaponClass::Unknown,
        WeaponClass::Unknown,
    );
    assert_eq!(
        u.d_heavy, 640,
        "unknown bar falls back to the front profile"
    );
    let b = effective_timing(
        &front,
        &back,
        false,
        ActiveBar::Back,
        WeaponClass::Unknown,
        WeaponClass::Unknown,
    );
    assert_eq!(b.d_heavy, 1360);
}

#[test]
fn heavy_preset_orders_dual_wield_fastest() {
    let dw = heavy_preset(WeaponClass::DualWield).unwrap();
    let th = heavy_preset(WeaponClass::TwoHanded).unwrap();
    let staff = heavy_preset(WeaponClass::RestorationStaff).unwrap();
    let bow = heavy_preset(WeaponClass::Bow).unwrap();
    assert!(dw < th, "dual wield heavy is shorter than two handed");
    assert!(
        th < staff && th < bow,
        "two handed is shorter than staves and bow"
    );
    assert_eq!(heavy_preset(WeaponClass::Unknown), None);
}

#[test]
fn auto_timing_applies_preset_per_bar() {
    // Base profiles both use 900; auto derives d_heavy from each bar's class.
    let front = timing(500, 50, 900, 125);
    let back = timing(500, 50, 900, 125);

    // Front bar dual wield -> 640; back bar restoration staff -> 1360.
    let f = effective_timing(
        &front,
        &back,
        true,
        ActiveBar::Front,
        WeaponClass::DualWield,
        WeaponClass::RestorationStaff,
    );
    assert_eq!(f.d_heavy, 640);
    let b = effective_timing(
        &front,
        &back,
        true,
        ActiveBar::Back,
        WeaponClass::DualWield,
        WeaponClass::RestorationStaff,
    );
    assert_eq!(b.d_heavy, 1360);
    assert!(
        f.d_heavy < b.d_heavy,
        "the faster-heavy bar has the shorter delay"
    );

    // Auto off uses the manual profile unchanged.
    let manual = effective_timing(
        &front,
        &back,
        false,
        ActiveBar::Front,
        WeaponClass::DualWield,
        WeaponClass::RestorationStaff,
    );
    assert_eq!(manual.d_heavy, 900);
}

#[test]
fn set_weapon_bar_drives_current_timing() {
    let config = WeaveConfig {
        timing: timing(500, 50, 640, 125),
        timing_back: timing(500, 50, 1360, 125),
        ..Default::default()
    };
    let mut engine = WeaveEngine::new(config);

    // Default (unknown) bar uses the front profile.
    assert_eq!(engine.current_timing().d_heavy, 640);

    engine.set_weapon_bar(WeaponBarSignal {
        bar: ActiveBar::Back,
        front: WeaponClass::DualWield,
        back: WeaponClass::RestorationStaff,
    });
    assert_eq!(engine.active_bar(), ActiveBar::Back);
    assert_eq!(engine.current_timing().d_heavy, 1360);
}

#[test]
fn back_profile_and_auto_flag_round_trip() {
    let config = WeaveConfig {
        timing: timing(500, 50, 640, 100),
        timing_back: timing(600, 40, 1360, 150),
        auto_timing: true,
        ..Default::default()
    };
    let engine = WeaveEngine::new(config);

    let mut settings = Settings::default();
    engine.store(&mut settings);

    let mut loaded = WeaveEngine::new(WeaveConfig::default());
    let notices = loaded.load(&settings);
    assert!(notices.is_empty());
    assert_eq!(loaded.config().timing_back.d_heavy, 1360);
    assert_eq!(loaded.config().timing_back.global_cooldown, 600);
    assert!(loaded.config().auto_timing);
}

#[test]
fn old_config_without_back_defaults_to_front() {
    // A timing section from before slice 014 has only the front fields.
    let settings = Settings {
        timing: serde_json::json!({
            "global_cooldown": 500, "d_weave": 50, "d_heavy": 700, "d_bash": 125
        }),
        ..Default::default()
    };
    let mut engine = WeaveEngine::new(WeaveConfig::default());
    engine.load(&settings);
    assert_eq!(
        engine.config().timing_back.d_heavy,
        700,
        "back mirrors front when absent"
    );
    assert!(!engine.config().auto_timing);
}
