//! Weave sequence construction: the pure mapping from a slot and timing to an
//! ordered list of steps, with optional latency-adaptive scaling of the
//! `d_weave` and `d_bash` delays.

use crate::input::{CombatChordPlan, Transition};
use crate::weave::types::{
    InputOp, LatencyConfig, SkillSlot, TimingConfig, WeaveStep, WeaveType, MAX_LATENCY_BONUS_MS,
};

/// Computes the effective delay for a base delay under latency adaptation.
///
/// Returns `base` when the feature is disabled or no current latency is known.
/// Otherwise returns `base + clamp(round(k * latency_ms), 0, 300)` using
/// round-half-away-from-zero and saturating addition, so the result always lies
/// in `[base, base + MAX_LATENCY_BONUS_MS]`.
pub fn effective_delay(base: u32, latency_ms: Option<u16>, cfg: &LatencyConfig) -> u32 {
    if !cfg.enabled {
        return base;
    }
    let Some(latency) = latency_ms else {
        return base;
    };
    let bonus = (cfg.k * f64::from(latency))
        .round()
        .clamp(0.0, f64::from(MAX_LATENCY_BONUS_MS)) as u32;
    base.saturating_add(bonus)
}

/// Builds the ordered weave step sequence for a slot using the base delays,
/// substituting per-slot delay overrides where present and the global timing
/// otherwise. Pure: no side effects.
///
/// This is [`sequence_for_adapted`] with latency adaptation disabled, so it is
/// byte-for-byte the pre-latency-feature behavior.
pub fn sequence_for(
    slot: &SkillSlot,
    timing: &TimingConfig,
    plan: CombatChordPlan,
) -> Vec<WeaveStep> {
    sequence_for_adapted(slot, timing, plan, None, &LatencyConfig::default())
}

/// Builds the ordered weave step sequence for a slot, scaling the `d_weave` and
/// `d_bash` delays by the current latency when adaptation is enabled. `d_heavy`
/// and the global cooldown are never scaled. Pure: no side effects.
///
/// "Primary click" expands to primary down then primary up; "send skill key"
/// expands to key down then key up.
pub fn sequence_for_adapted(
    slot: &SkillSlot,
    timing: &TimingConfig,
    plan: CombatChordPlan,
    latency_ms: Option<u16>,
    latency: &LatencyConfig,
) -> Vec<WeaveStep> {
    let d_weave = effective_delay(slot.d_weave(timing), latency_ms, latency);
    let d_heavy = slot.d_heavy(timing);
    let d_bash = effective_delay(slot.d_bash(timing), latency_ms, latency);

    let skill_down = WeaveStep::Emit(InputOp::Chord(plan.skill, Transition::Down));
    let skill_up = WeaveStep::Emit(InputOp::Chord(plan.skill, Transition::Up));
    let attack_down = plan
        .attack
        .map(|attack| WeaveStep::Emit(InputOp::Chord(attack, Transition::Down)));
    let attack_up = plan
        .attack
        .map(|attack| WeaveStep::Emit(InputOp::Chord(attack, Transition::Up)));

    match slot.weave_type {
        WeaveType::LightAttack => vec![
            attack_down.expect("attack chord required by light attack"),
            attack_up.expect("attack chord required by light attack"),
            WeaveStep::Wait(d_weave),
            skill_down,
            skill_up,
        ],
        WeaveType::HeavyAttack => vec![
            attack_down.expect("attack chord required by heavy attack"),
            WeaveStep::Wait(d_heavy),
            skill_down,
            skill_up,
            attack_up.expect("attack chord required by heavy attack"),
        ],
        WeaveType::BashAttack => vec![
            attack_down.expect("attack chord required by bash"),
            attack_up.expect("attack chord required by bash"),
            WeaveStep::Wait(d_weave),
            skill_down,
            skill_up,
            WeaveStep::Wait(d_bash),
            WeaveStep::Emit(InputOp::Chord(
                plan.block.expect("block chord required by bash"),
                Transition::Down,
            )),
            attack_down.expect("attack chord required by bash"),
            attack_up.expect("attack chord required by bash"),
            WeaveStep::Emit(InputOp::Chord(
                plan.block.expect("block chord required by bash"),
                Transition::Up,
            )),
        ],
        WeaveType::BlockCasting => vec![
            WeaveStep::Emit(InputOp::Chord(
                plan.block.expect("block chord required by block casting"),
                Transition::Down,
            )),
            skill_down,
            skill_up,
            WeaveStep::Wait(d_weave),
            WeaveStep::Emit(InputOp::Chord(
                plan.block.expect("block chord required by block casting"),
                Transition::Up,
            )),
        ],
    }
}
