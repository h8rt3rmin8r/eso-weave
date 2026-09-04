# Phase 0 Research: Auto-potion Restoration

**Feature**: [spec.md](spec.md) | **Date**: 2026-09-03

No NEEDS CLARIFICATION markers remain. Decisions follow the autopilot policy.

## R1: Proven failure boundary

**Decision**: Treat the current compile-time consumer gate as the direct reason the complete controller never runs in production, while retaining the existing controller as a starting point rather than replacing it wholesale.

`src/main.rs` executes `AutoPotionController::tick` only when `EXPLICIT_QUICKSLOT_AUTOMATION_ENABLED` is true. S042 deliberately pins that constant false. The existing rule and input sink are therefore covered by tests but unreachable in the application. S043 must remove this gate after strengthening the runtime contract.

## R2: Requested setting versus effective state

**Decision**: Keep `enabled` as the session-only user request and add a separate typed effective state owned by the controller.

The S039 `on_signal_lost` method clears `enabled`. That turns a temporary observation failure into a settings mutation and prevents automatic recovery. The correction preserves the request and changes only the effective state. This matches the S041 distinction between user intent and detected game lifecycle.

**Alternative rejected**: Re-enabling automatically after signal recovery while still clearing the request would require hidden memory or persistence and would leave the UI untruthful during the outage.

## R3: Positive beacon availability

**Decision**: Route PixelBus Heartbeat as positive signal availability and SignalLost as unavailable. Signal loss blocks immediately, and fresh heartbeat allows evaluation to resume.

Resource and quickslot values already clear on their established loss paths, but an explicit beacon gate makes the automation conjunction auditable and prevents a future retained observation from accidentally authorizing input.

## R4: Evaluation model

**Decision**: Use one ordered, pure evaluation that returns a typed effective outcome rather than `Result<(), Block>`.

Order is safety-significant: request, game, focus, beacon, suspension, context gate, resource watches and freshness, explicit quickslot class and usability, cooldown, retry, then threshold. This order makes exactly one current reason visible while preserving fail-closed behavior.

Ready means all non-threshold preconditions pass and at least one watched resource is fresh, but none is low. Triggered is stored only until the next evaluation and carries the first low resource in deterministic Health, Magicka, Stamina order.

## R5: Quickslot semantics

**Decision**: Match S042 variants directly instead of using the convenience boolean alone.

This retains the same authorization boundary while distinguishing Unavailable, Empty, Non-potion, Depleted, Blocked, and Usable states for the user. Cooldown remains an independent fact checked after explicit Potion(Usable).

## R6: State ownership and logging

**Decision**: Store the last effective state in `AutoPotionController` and update it on each evaluation or immediate lifecycle blocker. Emit a normal diagnostic only when the value changes.

The UI can read a coherent status without reproducing the safety rule. Equality on categorical blockers prevents retry polling and unchanged observations from spamming logs. Trigger details contain only the resource kind, percentage, and configured threshold. Other raw observations remain debug-only.

## R7: Input submission

**Decision**: Retain the existing `AutoPotionSink` and configured binding. One eligible evaluation calls Down then Up once and records the attempt time before the next evaluation.

The platform backend already provides the cross-platform submission seam and platform safety controls. A new input path would add risk without improving correctness.

## R8: Verification boundary

**Decision**: Complete deterministic coverage and the full repository gate in S043, but leave issue #25 open for the post-release real-client matrix.

The user cannot perform client verification until a fresh release exists. The pull request must state that limitation, link issue #25 without a closing keyword, and avoid claiming field evidence.
