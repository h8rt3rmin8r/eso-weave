# Computation and Gating Requirements Checklist: Latency-Adaptive Delays

**Purpose**: Validate that the requirements governing the effective-delay
computation, the scope of scaling, the enablement and data gating, and the
no-regression guarantee are complete, unambiguous, consistent, and measurable
before planning and implementation.
**Created**: 2026-07-11
**Feature**: [spec.md](../spec.md)

## Formula and Clamp Clarity

- [x] CHK001 Is the effective-delay formula stated exactly, including the clamp (`base + clamp(round(k * latency), 0, 300)`)? [Clarity, Spec FR-001]
- [x] CHK002 Is the rounding rule specified unambiguously (round-half-away-from-zero on `k * latency`)? [Clarity, Spec FR-002]
- [x] CHK003 Is the clamp range and its inclusivity defined (effective delay in `[base, base + 300]`, cap inclusive)? [Completeness, Spec FR-001, Edge Cases]
- [x] CHK004 Is it stated that the effective delay is never below the base (a non-positive bonus yields the base)? [Clarity, Spec US3, FR-001]
- [x] CHK005 Is overflow/saturation behavior for the computation addressed? [Edge Case, Spec Edge Cases]

## Scope of Scaling

- [x] CHK006 Is it stated that only `d_weave` and `d_bash` are scaled? [Completeness, Spec FR-003]
- [x] CHK007 Is it stated that `d_heavy` and `global_cooldown` are never scaled at any latency? [Consistency, Spec FR-003, SC-002]
- [x] CHK008 Is `base_delay` defined as the per-slot-resolved (override-or-global) value, so a slot override scales from its own base? [Clarity, Spec Clarifications, Edge Cases]

## Enablement and Data Gating

- [x] CHK009 Is the off-by-default requirement stated, with base delays when off? [Completeness, Spec FR-004]
- [x] CHK010 Is the no-current-latency case defined as yielding base delays even when enabled? [Completeness, Spec FR-005]
- [x] CHK011 Is the intake that sets or clears the current latency specified, including that clearing (as on signal loss) reverts to base delays? [Completeness, Spec FR-006, US2]
- [x] CHK012 Is the behavior of a latency value arriving while the feature is disabled defined (no scaling applied)? [Edge Case, Spec Edge Cases]

## No-Regression Guarantee

- [x] CHK013 Is the byte-for-byte no-regression guarantee for existing base sequences (feature off, or no latency) stated? [Consistency, Spec FR-008, SC-003]
- [x] CHK014 Is it clear that no existing weave timing changes unless the feature is explicitly enabled with live latency? [Clarity, Spec FR-008]

## Testability

- [x] CHK015 Is the computation required to live in the pure sequence builder so it is unit-testable with crafted latency and configs, independent of a real reader or clock? [Completeness, Spec FR-007, Constitution III]
- [x] CHK016 Are the success criteria expressed as measurable outcomes at representative latencies including the clamp boundary? [Measurability, Spec SC-001, SC-003]

## Configuration

- [x] CHK017 Is the configuration (enabled flag default off, `k` default 0.25) required to be user-configurable and to persist as an additive settings section? [Completeness, Spec FR-009, FR-010]
- [x] CHK018 Is a valid `k` defined concretely (finite, in `[0.0, 4.0]`) and the fallback-with-notice behavior for an invalid `k` specified? [Clarity, Spec FR-009, FR-010, Clarifications]
- [x] CHK019 Is the absent-section-yields-defaults behavior specified? [Completeness, Spec FR-010, US4]

## Consistency and Traceability

- [x] CHK020 Does every functional requirement have at least one acceptance scenario or measurable success criterion? [Traceability, Spec FR/US/SC]
- [x] CHK021 Are the master-specification constraints (k default 0.25, cap 300, only d_weave and d_bash scaled, off by default) all reflected without contradiction? [Consistency, Spec Section 7.4]

## Notes

- This checklist tests the quality of the requirements, not the implementation.
- All items were evaluated against the spec as written and pass; it is retained as
  the definition-of-done reference for the pre-push review.
- FR-008 (no regression to existing base sequences) is the key safety property for
  this enhancement and MUST remain covered by tests that assert the disabled and
  no-latency paths reproduce the existing sequences exactly.
