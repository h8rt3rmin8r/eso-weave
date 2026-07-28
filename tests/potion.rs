//! Auto-potion trigger-rule tests (slice 039, issue #20).
//!
//! This is the first feature in the project that synthesizes input from a
//! beacon-derived value, so the tests here are the point of the slice rather than
//! a follow-up to it. They are written directly against
//! `specs/039-auto-potion/contracts/trigger-rule.md`.
//!
//! The governing discipline: **every blocking condition is tested in isolation
//! with all the others satisfied, and each asserts which condition blocked.**
//! Asserting only that nothing was emitted would let a test pass because a
//! different condition happened to be false, which would leave the gate it thinks
//! it is checking completely unexercised.

use eso_weave::input::{Key, Transition};
use eso_weave::pixelbus::{QuickslotState, ResourceLevel, ResourceSet, SlotCooldown};
use eso_weave::potion::{
    evaluate, AutoPotionConfig, AutoPotionController, Block, MockAutoPotionSink, PotionInputs,
    PotionReadings, ResourceWatch,
};

/// A configuration with all three resources watched at 50 percent.
fn config_all_watched() -> AutoPotionConfig {
    let watch = ResourceWatch {
        enabled: true,
        threshold: 50,
    };
    AutoPotionConfig {
        health: watch,
        magicka: watch,
        stamina: watch,
        ..AutoPotionConfig::default()
    }
}

/// Resource levels from three percentages.
fn levels(health: u8, magicka: u8, stamina: u8) -> ResourceSet {
    ResourceSet {
        health: ResourceLevel::Percent(health),
        stamina: ResourceLevel::Percent(stamina),
        magicka: ResourceLevel::Percent(magicka),
    }
}

/// A quickslot holding a ready potion.
fn ready_potion() -> QuickslotState {
    QuickslotState {
        cooldown: SlotCooldown::Ready,
        item_id: Some(0x12_3456),
    }
}

/// Readings in which the bus half of the rule is satisfied: health is low and a
/// ready potion is slotted.
fn eligible_readings() -> PotionReadings {
    PotionReadings {
        resources: levels(10, 90, 90),
        quickslot: ready_potion(),
    }
}

/// Inputs in which every condition is satisfied: eligible readings, and neither
/// gate set. Each test below breaks exactly one thing.
fn eligible_inputs() -> PotionInputs {
    PotionInputs {
        readings: eligible_readings(),
        suspended: false,
        gated: false,
    }
}

/// Eligible inputs with the resource levels replaced and everything else
/// satisfied, so a test that varies only the levels reads as exactly that.
fn inputs_at(resources: ResourceSet) -> PotionInputs {
    PotionInputs {
        readings: PotionReadings {
            resources,
            quickslot: ready_potion(),
        },
        ..eligible_inputs()
    }
}

/// Evaluates with everything satisfied unless the caller has changed it.
fn eval(inputs: PotionInputs, enabled: bool, last: Option<u64>, now: u64) -> Result<(), Block> {
    evaluate(inputs, &config_all_watched(), enabled, last, now)
}

// ---------------------------------------------------------------------------
// The baseline: with everything satisfied, it fires.
// ---------------------------------------------------------------------------

#[test]
fn it_fires_when_every_condition_is_satisfied() {
    // If this ever stops holding, every isolation test below becomes vacuous:
    // they would all "pass" by blocking for the wrong reason.
    assert_eq!(eval(eligible_inputs(), true, None, 10_000), Ok(()));
}

// ---------------------------------------------------------------------------
// Each of the seven conditions, failing in isolation, with all others satisfied.
// Each asserts WHICH condition blocked (contract, and safety checklist CHK012).
// ---------------------------------------------------------------------------

#[test]
fn condition_1_disabled_blocks_and_says_so() {
    assert_eq!(
        eval(eligible_inputs(), false, None, 10_000),
        Err(Block::Disabled)
    );
}

#[test]
fn condition_2_suspended_blocks_and_says_so() {
    let mut inputs = eligible_inputs();
    inputs.suspended = true;
    assert_eq!(eval(inputs, true, None, 10_000), Err(Block::Suspended));
}

#[test]
fn condition_3_gated_blocks_and_says_so() {
    let mut inputs = eligible_inputs();
    inputs.gated = true;
    assert_eq!(eval(inputs, true, None, 10_000), Err(Block::Gated));
}

#[test]
fn condition_4_retry_too_soon_blocks_and_says_so() {
    let interval = u64::from(config_all_watched().retry_interval_ms);
    assert_eq!(
        eval(eligible_inputs(), true, Some(10_000), 10_000 + interval - 1),
        Err(Block::RetryTooSoon)
    );
}

#[test]
fn condition_5_no_potion_blocks_and_says_so() {
    let mut inputs = eligible_inputs();
    inputs.readings.quickslot = QuickslotState::new_unknown();
    assert_eq!(eval(inputs, true, None, 10_000), Err(Block::NoPotion));
}

#[test]
fn condition_6_on_cooldown_blocks_and_says_so() {
    let mut inputs = eligible_inputs();
    inputs.readings.quickslot = QuickslotState {
        cooldown: SlotCooldown::RemainingMs(4000),
        item_id: Some(0x12_3456),
    };
    assert_eq!(eval(inputs, true, None, 10_000), Err(Block::OnCooldown));
}

#[test]
fn condition_7_no_resource_low_blocks_and_says_so() {
    let mut inputs = eligible_inputs();
    inputs.readings.resources = levels(90, 90, 90);
    assert_eq!(eval(inputs, true, None, 10_000), Err(Block::NoResourceLow));
}

// ---------------------------------------------------------------------------
// The OR across resources (FR-002, SC-004).
// ---------------------------------------------------------------------------

#[test]
fn any_single_low_resource_fires_because_the_rule_is_an_or() {
    // Each resource in turn: that one low, the other two high, all three enabled.
    // An AND would fail every one of these, which is the whole point.
    for (name, resources) in [
        ("health", levels(10, 90, 90)),
        ("magicka", levels(90, 10, 90)),
        ("stamina", levels(90, 90, 10)),
    ] {
        assert_eq!(
            eval(inputs_at(resources), true, None, 10_000),
            Ok(()),
            "{name} alone being low should fire"
        );
    }
}

#[test]
fn a_disabled_watch_contributes_nothing_however_low_it_is() {
    // Health at zero but its watch off; the other two enabled and full.
    let mut config = config_all_watched();
    config.health.enabled = false;
    let inputs = inputs_at(levels(0, 100, 100));
    assert_eq!(
        evaluate(inputs, &config, true, None, 10_000),
        Err(Block::NoResourceLow)
    );
}

#[test]
fn with_every_watch_disabled_nothing_ever_fires() {
    let config = AutoPotionConfig::default(); // all three disabled
    let inputs = inputs_at(levels(0, 0, 0));
    assert_eq!(
        evaluate(inputs, &config, true, None, 10_000),
        Err(Block::NoResourceLow),
        "an empty watch set must not fall back to watching anything"
    );
}

// ---------------------------------------------------------------------------
// Unknown is never low (FR-004, SC-003). The most important rule in the feature.
// ---------------------------------------------------------------------------

#[test]
fn an_unreadable_resource_is_never_low_at_any_threshold() {
    // Across every threshold including 100, for each resource in turn, with only
    // that resource enabled so nothing else can carry the trigger.
    for resource in ["health", "magicka", "stamina"] {
        for threshold in 0..=100u8 {
            let watch = ResourceWatch {
                enabled: true,
                threshold,
            };
            let mut config = AutoPotionConfig::default();
            let mut resources = ResourceSet::new_unknown();
            match resource {
                "health" => {
                    config.health = watch;
                    resources.health = ResourceLevel::Unknown;
                }
                "magicka" => {
                    config.magicka = watch;
                    resources.magicka = ResourceLevel::Unknown;
                }
                _ => {
                    config.stamina = watch;
                    resources.stamina = ResourceLevel::Unknown;
                }
            }
            assert_eq!(
                evaluate(inputs_at(resources), &config, true, None, 10_000),
                Err(Block::NoResourceLow),
                "unknown {resource} satisfied a threshold of {threshold}"
            );
        }
    }
}

#[test]
fn an_unreadable_quickslot_is_not_a_potion_and_an_unknown_cooldown_is_not_zero() {
    // The quickslot half of the same rule. `has_potion` is `cooldown != Unknown`,
    // so an unreadable quickslot is categorically not a potion.
    let mut inputs = eligible_inputs();
    inputs.readings.quickslot = QuickslotState {
        cooldown: SlotCooldown::Unknown,
        item_id: None,
    };
    assert_eq!(eval(inputs, true, None, 10_000), Err(Block::NoPotion));

    // And an identity present with an unknown cooldown is still not a potion:
    // the cooldown is what `has_potion` reads, not the identity.
    inputs.readings.quickslot = QuickslotState {
        cooldown: SlotCooldown::Unknown,
        item_id: Some(42),
    };
    assert_eq!(eval(inputs, true, None, 10_000), Err(Block::NoPotion));
}

// ---------------------------------------------------------------------------
// The threshold boundary (FR-005).
// ---------------------------------------------------------------------------

#[test]
fn the_comparison_is_at_or_below_not_strictly_below() {
    let threshold = 50u8;
    for (level, should_fire) in [
        (threshold - 1, true),
        (threshold, true),
        (threshold + 1, false),
    ] {
        let inputs = inputs_at(levels(level, 100, 100));
        let outcome = eval(inputs, true, None, 10_000);
        assert_eq!(
            outcome.is_ok(),
            should_fire,
            "at level {level} against threshold {threshold}"
        );
    }
}

#[test]
fn thresholds_of_zero_and_one_hundred_are_both_valid() {
    // Zero fires only on an empty pool; it is not a synonym for "off", which is
    // what the per-resource enable is for.
    let mut config = AutoPotionConfig {
        health: ResourceWatch {
            enabled: true,
            threshold: 0,
        },
        ..AutoPotionConfig::default()
    };
    let empty = inputs_at(levels(0, 100, 100));
    assert_eq!(evaluate(empty, &config, true, None, 10_000), Ok(()));
    let one_percent = inputs_at(levels(1, 100, 100));
    assert_eq!(
        evaluate(one_percent, &config, true, None, 10_000),
        Err(Block::NoResourceLow)
    );

    // A hundred fires whenever the resource is readable. Unusual, coherent, and
    // the operator's choice.
    config.health.threshold = 100;
    let full = inputs_at(levels(100, 100, 100));
    assert_eq!(evaluate(full, &config, true, None, 10_000), Ok(()));
}

// ---------------------------------------------------------------------------
// The retry interval against a virtual clock (FR-006, SC-005).
// ---------------------------------------------------------------------------

#[test]
fn the_retry_interval_bounds_the_rate_independently_of_the_cooldown() {
    let interval = u64::from(config_all_watched().retry_interval_ms);
    let last = 10_000u64;
    // Note the quickslot still reports Ready throughout, which is exactly the lag
    // window this interval exists to cover: the game has not yet reported the
    // cooldown the last press caused.
    assert_eq!(
        eval(eligible_inputs(), true, Some(last), last),
        Err(Block::RetryTooSoon)
    );
    assert_eq!(
        eval(eligible_inputs(), true, Some(last), last + interval - 1),
        Err(Block::RetryTooSoon)
    );
    assert_eq!(
        eval(eligible_inputs(), true, Some(last), last + interval),
        Ok(())
    );
}

// ---------------------------------------------------------------------------
// The controller: emission, defaults, and the gates.
// ---------------------------------------------------------------------------

/// A controller with all three resources watched at 50, switched on.
fn armed_controller() -> AutoPotionController {
    let mut controller = AutoPotionController::new(config_all_watched());
    controller.set_enabled(true);
    controller
}

#[test]
fn one_trigger_emits_exactly_one_press_and_one_release() {
    let mut controller = armed_controller();
    let mut sink = MockAutoPotionSink::new();
    assert_eq!(
        controller.tick(eligible_readings(), 10_000, &mut sink),
        Ok(())
    );
    assert_eq!(
        sink.ops,
        vec![(Key::Q, Transition::Down), (Key::Q, Transition::Up)]
    );
}

#[test]
fn repeated_eligible_ticks_emit_only_what_the_interval_allows() {
    // SC-001: twenty consecutive ticks under unchanging eligible conditions. The
    // quickslot never reports a cooldown here, so the retry interval is the only
    // thing bounding the rate, which is precisely the case it exists for.
    let mut controller = armed_controller();
    let mut sink = MockAutoPotionSink::new();

    let mut now = 10_000u64;
    let step = 100u64;
    let mut fired = 0;
    for _ in 0..20 {
        if controller.tick(eligible_readings(), now, &mut sink).is_ok() {
            fired += 1;
        }
        now += step;
    }
    // Over 2000 ms at a 1500 ms interval: the first tick and one more.
    assert_eq!(fired, 2, "the interval must bound the firing rate");
    assert_eq!(sink.ops.len(), fired * 2, "two operations per firing");
}

#[test]
fn a_fresh_install_never_fires() {
    // FR-013 / SC-006. The shipped defaults, not a hand-built disabled config:
    // all three watches off and the controller off.
    let defaults = AutoPotionConfig::default();
    assert!(!defaults.health.enabled);
    assert!(!defaults.magicka.enabled);
    assert!(!defaults.stamina.enabled);

    let mut controller = AutoPotionController::new(defaults);
    assert!(!controller.enabled(), "a fresh controller must be off");

    let mut sink = MockAutoPotionSink::new();
    assert_eq!(
        controller.tick(eligible_readings(), 10_000, &mut sink),
        Err(Block::Disabled)
    );
    assert!(sink.ops.is_empty());
}

#[test]
fn a_controller_never_enabled_emits_nothing_under_any_readings() {
    // The unit-level half of SC-006. The suite-level half is the existing weave
    // and fishing tests continuing to pass unchanged.
    let mut controller = AutoPotionController::new(config_all_watched());
    let mut sink = MockAutoPotionSink::new();
    let mut now = 0u64;
    for health in [0u8, 1, 25, 50, 51, 100] {
        for quickslot in [ready_potion(), QuickslotState::new_unknown()] {
            for suspended in [false, true] {
                for gated in [false, true] {
                    controller.set_suspended(suspended);
                    controller.set_gated(gated);
                    let readings = PotionReadings {
                        resources: levels(health, health, health),
                        quickslot,
                    };
                    assert_eq!(
                        controller.tick(readings, now, &mut sink),
                        Err(Block::Disabled),
                        "a disabled controller must report Disabled before any other reason"
                    );
                    now += 10_000;
                }
            }
        }
    }
    assert!(sink.ops.is_empty(), "a disabled controller emitted input");
}

#[test]
fn the_menu_gate_reaches_the_controller_directly() {
    // FR-009 and the slice 032 lesson: this controller acts on its own timers and
    // never passes through interception, so gating interception alone would leave
    // it firing into a chat message.
    let mut controller = armed_controller();
    let mut sink = MockAutoPotionSink::new();
    controller.set_gated(true);
    assert_eq!(
        controller.tick(eligible_readings(), 10_000, &mut sink),
        Err(Block::Gated)
    );
    assert!(sink.ops.is_empty());

    controller.set_gated(false);
    assert_eq!(
        controller.tick(eligible_readings(), 20_000, &mut sink),
        Ok(())
    );
}

#[test]
fn suspend_stops_it_and_is_checked_rather_than_incidental() {
    let mut controller = armed_controller();
    let mut sink = MockAutoPotionSink::new();
    controller.set_suspended(true);
    assert_eq!(
        controller.tick(eligible_readings(), 10_000, &mut sink),
        Err(Block::Suspended)
    );
    assert!(sink.ops.is_empty());

    controller.set_suspended(false);
    assert_eq!(
        controller.tick(eligible_readings(), 20_000, &mut sink),
        Ok(())
    );
}

#[test]
fn signal_loss_switches_it_off_rather_than_leaving_it_evaluating() {
    // FR-011, matching fishing. Not a block inside the rule: the controller is
    // switched off, so it stays off until the operator turns it back on.
    let mut controller = armed_controller();
    let mut sink = MockAutoPotionSink::new();
    controller.on_signal_lost();
    assert!(!controller.enabled());
    assert_eq!(
        controller.tick(eligible_readings(), 10_000, &mut sink),
        Err(Block::Disabled)
    );
    assert!(sink.ops.is_empty());
}

#[test]
fn the_last_attempt_is_recorded_on_the_attempt_not_on_a_confirmed_drink() {
    let mut controller = armed_controller();
    let mut sink = MockAutoPotionSink::new();
    assert_eq!(controller.last_attempt_ms(), None);
    controller
        .tick(eligible_readings(), 7_777, &mut sink)
        .unwrap();
    assert_eq!(controller.last_attempt_ms(), Some(7_777));
}

// ---------------------------------------------------------------------------
// Configuration round-trips and degradation (FR-018).
// ---------------------------------------------------------------------------

#[test]
fn config_round_trips_through_settings() {
    let config = AutoPotionConfig {
        health: ResourceWatch {
            enabled: true,
            threshold: 40,
        },
        stamina: ResourceWatch {
            enabled: true,
            threshold: 0,
        },
        quickslot_key: Key::X,
        retry_interval_ms: 2500,
        ..AutoPotionConfig::default()
    };

    let mut notices = Vec::new();
    let loaded = AutoPotionConfig::load(&config.store(), &mut notices);
    assert!(notices.is_empty());
    assert_eq!(loaded, config);
}

#[test]
fn a_null_value_yields_defaults_without_notices() {
    let mut notices = Vec::new();
    let loaded = AutoPotionConfig::load(&serde_json::Value::Null, &mut notices);
    assert!(notices.is_empty());
    assert_eq!(loaded, AutoPotionConfig::default());
}

#[test]
fn invalid_stored_values_degrade_to_defaults_with_notices() {
    let raw = serde_json::json!({
        "health": { "enabled": true, "threshold": 250 },
        "quickslot_key": "not_a_key",
        "retry_interval_ms": 9_999_999u32,
    });
    let mut notices = Vec::new();
    let loaded = AutoPotionConfig::load(&raw, &mut notices);
    let defaults = AutoPotionConfig::default();

    // The bad values fell back, and the good one beside them survived.
    assert_eq!(loaded.health.threshold, defaults.health.threshold);
    assert!(
        loaded.health.enabled,
        "a valid field must survive its neighbour"
    );
    assert_eq!(loaded.quickslot_key, defaults.quickslot_key);
    assert_eq!(loaded.retry_interval_ms, defaults.retry_interval_ms);
    assert_eq!(
        notices.len(),
        3,
        "one notice per invalid value: {notices:?}"
    );
}
