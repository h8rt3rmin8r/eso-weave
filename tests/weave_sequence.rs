//! Sequence-correctness tests for the Weave Engine (spec user story US1).

use eso_weave::input::{Key, MouseButton, Transition};
use eso_weave::weave::sequence_for;
use eso_weave::weave::types::{
    InputOp, SkillSlot, SlotOverrides, TimingConfig, WeaveStep, WeaveType,
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

fn key(transition: Transition) -> WeaveStep {
    WeaveStep::Emit(InputOp::Key(Key::Digit1, transition))
}

fn mouse(button: MouseButton, transition: Transition) -> WeaveStep {
    WeaveStep::Emit(InputOp::Mouse(button, transition))
}

#[test]
fn light_attack_sequence() {
    let timing = TimingConfig::default();
    let steps = sequence_for(&slot(WeaveType::LightAttack), &timing);
    assert_eq!(
        steps,
        vec![
            mouse(MouseButton::Primary, Transition::Down),
            mouse(MouseButton::Primary, Transition::Up),
            WeaveStep::Wait(timing.d_weave),
            key(Transition::Down),
            key(Transition::Up),
        ]
    );
}

#[test]
fn heavy_attack_sequence() {
    let timing = TimingConfig::default();
    let steps = sequence_for(&slot(WeaveType::HeavyAttack), &timing);
    assert_eq!(
        steps,
        vec![
            mouse(MouseButton::Primary, Transition::Down),
            WeaveStep::Wait(timing.d_heavy),
            key(Transition::Down),
            key(Transition::Up),
            mouse(MouseButton::Primary, Transition::Up),
        ]
    );
}

#[test]
fn bash_attack_sequence() {
    let timing = TimingConfig::default();
    let steps = sequence_for(&slot(WeaveType::BashAttack), &timing);
    assert_eq!(
        steps,
        vec![
            mouse(MouseButton::Primary, Transition::Down),
            mouse(MouseButton::Primary, Transition::Up),
            WeaveStep::Wait(timing.d_weave),
            key(Transition::Down),
            key(Transition::Up),
            WeaveStep::Wait(timing.d_bash),
            mouse(MouseButton::Secondary, Transition::Down),
            mouse(MouseButton::Primary, Transition::Down),
            mouse(MouseButton::Primary, Transition::Up),
            mouse(MouseButton::Secondary, Transition::Up),
        ]
    );
}

#[test]
fn block_casting_sequence() {
    let timing = TimingConfig::default();
    let steps = sequence_for(&slot(WeaveType::BlockCasting), &timing);
    assert_eq!(
        steps,
        vec![
            mouse(MouseButton::Secondary, Transition::Down),
            key(Transition::Down),
            key(Transition::Up),
            WeaveStep::Wait(timing.d_weave),
            mouse(MouseButton::Secondary, Transition::Up),
        ]
    );
}

#[test]
fn per_slot_override_changes_only_that_wait() {
    let timing = TimingConfig::default();
    let mut overridden = slot(WeaveType::LightAttack);
    overridden.overrides.d_weave = Some(275);

    let steps = sequence_for(&overridden, &timing);
    assert!(steps.contains(&WeaveStep::Wait(275)));
    assert!(!steps.contains(&WeaveStep::Wait(timing.d_weave)));

    // A slot without the override still uses the global default.
    let plain = sequence_for(&slot(WeaveType::LightAttack), &timing);
    assert!(plain.contains(&WeaveStep::Wait(timing.d_weave)));
}
