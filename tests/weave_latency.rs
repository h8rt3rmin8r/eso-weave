//! Latency-adaptive delay tests for the Weave Engine (spec section 7.4).
//!
//! The correctness-bearing logic (the effective-delay formula and its
//! integration) is exercised with crafted latency and configurations. The
//! no-regression guarantee (disabled or no-latency reproduces the base
//! sequences) is a required check per FR-008.

use eso_weave::config::{NoticeKind, Settings};
use eso_weave::input::{Action, Key};
use eso_weave::weave::types::{
    LatencyConfig, SkillSlot, SlotOverrides, TimingConfig, WeaveStep, WeaveType,
};
use eso_weave::weave::{
    effective_delay, sequence_for, sequence_for_adapted, MockSink, WeaveConfig, WeaveEngine,
};

fn slot(weave_type: WeaveType) -> SkillSlot {
    SkillSlot {
        index: 1,
        key: Key::Digit1,
        weave_type,
        active: true,
        overrides: SlotOverrides::default(),
    }
}

fn enabled(k: f64) -> LatencyConfig {
    LatencyConfig { enabled: true, k }
}

/// The wait durations of a sequence, in order.
fn waits(steps: &[WeaveStep]) -> Vec<u32> {
    steps
        .iter()
        .filter_map(|s| match s {
            WeaveStep::Wait(ms) => Some(*ms),
            _ => None,
        })
        .collect()
}

// US1: scaling when enabled.

#[test]
fn effective_delay_scales_and_rounds() {
    let cfg = enabled(0.25);
    assert_eq!(effective_delay(50, Some(120), &cfg), 80); // 50 + round(30)
    assert_eq!(effective_delay(50, Some(122), &cfg), 81); // 50 + round(30.5) = 50 + 31
    assert_eq!(effective_delay(125, Some(200), &cfg), 175); // 125 + 50
}

#[test]
fn light_attack_scales_d_weave() {
    let timing = TimingConfig::default();
    let cfg = enabled(0.25);
    let steps = sequence_for_adapted(&slot(WeaveType::LightAttack), &timing, Some(120), &cfg);
    // base d_weave 50 -> 80.
    assert_eq!(waits(&steps), vec![80]);
}

#[test]
fn bash_attack_scales_d_weave_and_d_bash_only() {
    let timing = TimingConfig::default();
    let cfg = enabled(0.25);
    let steps = sequence_for_adapted(&slot(WeaveType::BashAttack), &timing, Some(120), &cfg);
    // d_weave 50 -> 80, d_bash 125 -> 155.
    assert_eq!(waits(&steps), vec![80, 155]);
    // The non-wait steps match the base sequence exactly.
    let base = sequence_for(&slot(WeaveType::BashAttack), &timing);
    let strip = |steps: &[WeaveStep]| -> Vec<WeaveStep> {
        steps
            .iter()
            .filter(|s| !matches!(s, WeaveStep::Wait(_)))
            .copied()
            .collect()
    };
    assert_eq!(strip(&steps), strip(&base));
}

#[test]
fn heavy_attack_d_heavy_is_never_scaled() {
    let timing = TimingConfig::default();
    let cfg = enabled(0.25);
    let steps = sequence_for_adapted(&slot(WeaveType::HeavyAttack), &timing, Some(1000), &cfg);
    // d_heavy stays 1000 regardless of latency.
    assert_eq!(waits(&steps), vec![timing.d_heavy]);
}

#[test]
fn per_slot_override_base_is_scaled() {
    let timing = TimingConfig::default();
    let cfg = enabled(0.25);
    let mut s = slot(WeaveType::LightAttack);
    s.overrides.d_weave = Some(200);
    let steps = sequence_for_adapted(&s, &timing, Some(120), &cfg);
    // Override base 200 -> 200 + 30.
    assert_eq!(waits(&steps), vec![230]);
}

// US2: off by default and no-data (no-regression).

#[test]
fn disabled_reproduces_base_sequences_at_any_latency() {
    let timing = TimingConfig::default();
    let default_cfg = LatencyConfig::default(); // disabled
    for wt in [
        WeaveType::LightAttack,
        WeaveType::HeavyAttack,
        WeaveType::BashAttack,
        WeaveType::BlockCasting,
    ] {
        let base = sequence_for(&slot(wt), &timing);
        let adapted = sequence_for_adapted(&slot(wt), &timing, Some(500), &default_cfg);
        assert_eq!(adapted, base, "{wt:?} disabled must equal base");
    }
}

#[test]
fn enabled_without_latency_reproduces_base_sequences() {
    let timing = TimingConfig::default();
    let cfg = enabled(0.25);
    for wt in [
        WeaveType::LightAttack,
        WeaveType::HeavyAttack,
        WeaveType::BashAttack,
        WeaveType::BlockCasting,
    ] {
        let base = sequence_for(&slot(wt), &timing);
        let adapted = sequence_for_adapted(&slot(wt), &timing, None, &cfg);
        assert_eq!(adapted, base, "{wt:?} without latency must equal base");
    }
}

// US3: clamp bounds.

#[test]
fn bonus_is_capped_at_300() {
    let cfg = enabled(4.0);
    // round(4.0 * 200) = 800, capped to 300.
    assert_eq!(effective_delay(50, Some(200), &cfg), 350);
    // Exactly at the cap: round(k*lat) == 300.
    let cfg = enabled(0.3);
    assert_eq!(effective_delay(50, Some(1000), &cfg), 350); // round(300) capped to 300
}

#[test]
fn zero_bonus_yields_base() {
    let cfg = enabled(0.0);
    assert_eq!(effective_delay(50, Some(1000), &cfg), 50);
    let cfg = enabled(0.25);
    assert_eq!(effective_delay(50, Some(1), &cfg), 50); // round(0.25) = 0
}

// US4: config persistence.

#[test]
fn latency_config_round_trips_through_settings() {
    let mut engine = WeaveEngine::new(WeaveConfig::default());
    engine.set_latency_config(LatencyConfig {
        enabled: true,
        k: 0.5,
    });
    let mut settings = Settings::default();
    engine.store(&mut settings);

    let mut reloaded = WeaveEngine::new(WeaveConfig::default());
    let notices = reloaded.load(&settings);
    assert!(notices.is_empty());
    assert_eq!(
        reloaded.latency_config(),
        &LatencyConfig {
            enabled: true,
            k: 0.5
        }
    );
}

#[test]
fn absent_latency_section_yields_defaults() {
    let mut engine = WeaveEngine::new(WeaveConfig::default());
    let notices = engine.load(&Settings::default());
    assert!(notices.is_empty());
    assert_eq!(engine.latency_config(), &LatencyConfig::default());
}

#[test]
fn invalid_k_falls_back_with_notice() {
    let settings = Settings {
        latency: serde_json::json!({ "enabled": true, "k": 10.0 }),
        ..Settings::default()
    };
    let mut engine = WeaveEngine::new(WeaveConfig::default());
    let notices = engine.load(&settings);
    assert_eq!(engine.latency_config().k, LatencyConfig::default().k);
    assert!(engine.latency_config().enabled); // enabled preserved, only k fell back
    assert!(notices.iter().any(|n| n.kind == NoticeKind::InvalidValue));
}

// Engine intake and integration.

#[test]
fn set_latency_scales_handled_weave_and_clearing_reverts() {
    let mut engine = WeaveEngine::new(WeaveConfig::default());
    engine.set_latency_config(enabled(0.25));

    // With latency, slot 1 (light attack) d_weave 50 -> 80.
    engine.set_latency(Some(120));
    let mut sink = MockSink::new();
    engine.handle(Action::Skill1, &mut sink);
    let scaled_waits: Vec<u32> = sink
        .log
        .iter()
        .filter_map(|s| match s {
            WeaveStep::Wait(ms) => Some(*ms),
            _ => None,
        })
        .collect();
    assert_eq!(scaled_waits, vec![80]);

    // Clearing the latency reverts to base delays. Advance past the cooldown.
    engine.set_latency(None);
    let mut sink = MockSink::new();
    sink.set_now(10_000);
    engine.handle(Action::Skill1, &mut sink);
    let base_waits: Vec<u32> = sink
        .log
        .iter()
        .filter_map(|s| match s {
            WeaveStep::Wait(ms) => Some(*ms),
            _ => None,
        })
        .collect();
    assert_eq!(base_waits, vec![50]);
}
