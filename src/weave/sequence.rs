//! Weave sequence construction: the pure mapping from a slot and timing to an
//! ordered list of steps.

use crate::input::{MouseButton, Transition};
use crate::weave::types::{InputOp, SkillSlot, TimingConfig, WeaveStep, WeaveType};

/// Builds the ordered weave step sequence for a slot, substituting per-slot delay
/// overrides where present and the global timing otherwise. Pure: no side effects.
///
/// "Primary click" expands to primary down then primary up; "send skill key"
/// expands to key down then key up.
pub fn sequence_for(slot: &SkillSlot, timing: &TimingConfig) -> Vec<WeaveStep> {
    let key = slot.key;
    let d_weave = slot.d_weave(timing);
    let d_heavy = slot.d_heavy(timing);
    let d_bash = slot.d_bash(timing);

    let key_down = WeaveStep::Emit(InputOp::Key(key, Transition::Down));
    let key_up = WeaveStep::Emit(InputOp::Key(key, Transition::Up));
    let primary_down = WeaveStep::Emit(InputOp::Mouse(MouseButton::Primary, Transition::Down));
    let primary_up = WeaveStep::Emit(InputOp::Mouse(MouseButton::Primary, Transition::Up));
    let secondary_down = WeaveStep::Emit(InputOp::Mouse(MouseButton::Secondary, Transition::Down));
    let secondary_up = WeaveStep::Emit(InputOp::Mouse(MouseButton::Secondary, Transition::Up));

    match slot.weave_type {
        WeaveType::LightAttack => vec![
            primary_down,
            primary_up,
            WeaveStep::Wait(d_weave),
            key_down,
            key_up,
        ],
        WeaveType::HeavyAttack => vec![
            primary_down,
            WeaveStep::Wait(d_heavy),
            key_down,
            key_up,
            primary_up,
        ],
        WeaveType::BashAttack => vec![
            primary_down,
            primary_up,
            WeaveStep::Wait(d_weave),
            key_down,
            key_up,
            WeaveStep::Wait(d_bash),
            secondary_down,
            primary_down,
            primary_up,
            secondary_up,
        ],
        WeaveType::BlockCasting => vec![
            secondary_down,
            key_down,
            key_up,
            WeaveStep::Wait(d_weave),
            secondary_up,
        ],
    }
}
