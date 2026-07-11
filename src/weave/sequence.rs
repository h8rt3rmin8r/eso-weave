//! Weave sequence construction: the pure mapping from a slot and timing to an
//! ordered list of steps, with optional latency-adaptive scaling of the
//! `d_weave` and `d_bash` delays.

use crate::input::{MouseButton, Transition};
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
pub fn sequence_for(slot: &SkillSlot, timing: &TimingConfig) -> Vec<WeaveStep> {
    sequence_for_adapted(slot, timing, None, &LatencyConfig::default())
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
    latency_ms: Option<u16>,
    latency: &LatencyConfig,
) -> Vec<WeaveStep> {
    let key = slot.key;
    let d_weave = effective_delay(slot.d_weave(timing), latency_ms, latency);
    let d_heavy = slot.d_heavy(timing);
    let d_bash = effective_delay(slot.d_bash(timing), latency_ms, latency);

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
