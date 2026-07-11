# Requirements Quality Checklist: Weave Engine

**Purpose**: Validate the clarity, completeness, and consistency of the Weave
Engine requirements before planning. Unit tests for the requirements, not the
implementation.
**Created**: 2026-07-11
**Feature**: [spec.md](../spec.md)

## Sequence Correctness

- [x] CHK001 Is each of the four weave sequences specified as a fully ordered list of operations and waits? [Completeness, Spec FR-003 to FR-006]
- [x] CHK002 Is the meaning of primary and secondary (left and right mouse) unambiguous? [Clarity, Spec FR-007]
- [x] CHK003 Is it clear which wait (d_weave, d_heavy, d_bash) appears at each point of each sequence? [Clarity, Spec FR-003 to FR-006]

## Timing and Cooldown

- [x] CHK004 Are the global timing defaults and their units stated? [Completeness, Spec FR-008]
- [x] CHK005 Is the per-slot override rule (blank uses global; irrelevant ignored) unambiguous? [Clarity, Spec FR-009, Edge Cases]
- [x] CHK006 Is cooldown gating defined precisely (dropped inside window, key still suppressed, no sequence)? [Clarity, Spec FR-010]
- [x] CHK007 Is the cooldown measured against a defined, testable clock seam? [Measurability, Spec FR-011, SC-006]

## Integration and Boundaries

- [x] CHK008 Is inactive-slot pass-through consistent with S002 suppression (activity fed to the Input Engine)? [Consistency, Spec FR-002]
- [x] CHK009 Is the boundary clear that toggles are not executed as weaves? [Clarity, Spec FR-014, Edge Cases]
- [x] CHK010 Is the synthesizer seam defined so sequences are testable without real input or waiting? [Completeness, Spec FR-013]
- [x] CHK011 Is the worker-thread execution consistent with the constitution's no-blocking-on-the-interception-path rule? [Consistency, Spec FR-012, FR-013]

## Persistence

- [x] CHK012 Is slot and timing persistence additive and backward compatible with prior settings? [Consistency, Spec FR-015]
- [x] CHK013 Is the fallback for missing or out-of-range timing values defined? [Coverage, Spec FR-016]

## Acceptance Criteria Quality

- [x] CHK014 Can sequence correctness be objectively verified per type? [Measurability, Spec SC-001]
- [x] CHK015 Can cooldown behavior be verified without real waiting? [Measurability, Spec SC-002, SC-006]
- [x] CHK016 Can a per-slot override's effect be isolated and verified? [Measurability, Spec SC-004]

## Notes

- All items pass. Sequence order, timing placement, cooldown semantics, and the
  clock and synthesizer seams are stated as testable behaviors.
