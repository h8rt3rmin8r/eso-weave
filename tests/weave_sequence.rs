//! Native-chord sequence correctness tests for the Weave Engine.

use eso_weave::input::{
    CombatChordPlan, KeyboardControl, ModifierSet, MouseControl, NativeChord, NativeControl,
    Transition,
};
use eso_weave::weave::sequence_for;
use eso_weave::weave::types::{
    InputOp, SkillSlot, SlotOverrides, TimingConfig, WeaveStep, WeaveType,
};

fn slot(weave_type: WeaveType) -> SkillSlot {
    SkillSlot {
        index: 1,
        weave_type,
        active: true,
        overrides: SlotOverrides::default(),
    }
}

fn chord(primary: NativeControl, modifiers: ModifierSet) -> NativeChord {
    NativeChord { primary, modifiers }
}

fn plan() -> CombatChordPlan {
    CombatChordPlan {
        skill: chord(
            NativeControl::Keyboard(KeyboardControl::K),
            ModifierSet::CONTROL,
        ),
        attack: Some(chord(
            NativeControl::Mouse(MouseControl::Button4),
            ModifierSet::SHIFT,
        )),
        block: Some(chord(
            NativeControl::Keyboard(KeyboardControl::B),
            ModifierSet::ALT,
        )),
    }
}

fn emit(chord: NativeChord, transition: Transition) -> WeaveStep {
    WeaveStep::Emit(InputOp::Chord(chord, transition))
}

#[test]
fn light_attack_sequence_uses_detected_chords() {
    let timing = TimingConfig::default();
    let plan = plan();
    assert_eq!(
        sequence_for(&slot(WeaveType::LightAttack), &timing, plan),
        vec![
            emit(plan.attack.unwrap(), Transition::Down),
            emit(plan.attack.unwrap(), Transition::Up),
            WeaveStep::Wait(timing.d_weave),
            emit(plan.skill, Transition::Down),
            emit(plan.skill, Transition::Up),
        ]
    );
}

#[test]
fn heavy_attack_sequence_uses_detected_chords() {
    let timing = TimingConfig::default();
    let plan = plan();
    assert_eq!(
        sequence_for(&slot(WeaveType::HeavyAttack), &timing, plan),
        vec![
            emit(plan.attack.unwrap(), Transition::Down),
            WeaveStep::Wait(timing.d_heavy),
            emit(plan.skill, Transition::Down),
            emit(plan.skill, Transition::Up),
            emit(plan.attack.unwrap(), Transition::Up),
        ]
    );
}

#[test]
fn bash_sequence_uses_detected_attack_skill_and_block() {
    let timing = TimingConfig::default();
    let plan = plan();
    assert_eq!(
        sequence_for(&slot(WeaveType::BashAttack), &timing, plan),
        vec![
            emit(plan.attack.unwrap(), Transition::Down),
            emit(plan.attack.unwrap(), Transition::Up),
            WeaveStep::Wait(timing.d_weave),
            emit(plan.skill, Transition::Down),
            emit(plan.skill, Transition::Up),
            WeaveStep::Wait(timing.d_bash),
            emit(plan.block.unwrap(), Transition::Down),
            emit(plan.attack.unwrap(), Transition::Down),
            emit(plan.attack.unwrap(), Transition::Up),
            emit(plan.block.unwrap(), Transition::Up),
        ]
    );
}

#[test]
fn block_casting_sequence_needs_no_attack_chord() {
    let timing = TimingConfig::default();
    let mut plan = plan();
    plan.attack = None;
    assert_eq!(
        sequence_for(&slot(WeaveType::BlockCasting), &timing, plan),
        vec![
            emit(plan.block.unwrap(), Transition::Down),
            emit(plan.skill, Transition::Down),
            emit(plan.skill, Transition::Up),
            WeaveStep::Wait(timing.d_weave),
            emit(plan.block.unwrap(), Transition::Up),
        ]
    );
}

#[test]
fn per_slot_override_changes_only_that_wait() {
    let timing = TimingConfig::default();
    let mut overridden = slot(WeaveType::LightAttack);
    overridden.overrides.d_weave = Some(275);
    let steps = sequence_for(&overridden, &timing, plan());
    assert!(steps.contains(&WeaveStep::Wait(275)));
    assert!(!steps.contains(&WeaveStep::Wait(timing.d_weave)));
}
