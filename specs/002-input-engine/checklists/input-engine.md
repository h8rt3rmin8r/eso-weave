# Requirements Quality Checklist: Input Engine

**Purpose**: Validate the clarity, completeness, and consistency of the Input
Engine requirements, with emphasis on the safety-critical surfaces, before
planning. These are unit tests for the requirements, not the implementation.
**Created**: 2026-07-11
**Feature**: [spec.md](../spec.md)

## Safety-Critical Surfaces

- [x] CHK001 Is the non-blocking interception path stated as a hard requirement with a clear list of what the path may and may not do? [Clarity, Spec FR-006]
- [x] CHK002 Is recursion breaking defined precisely (synthesized input distinguishable, per-event, never handed off)? [Clarity, Spec FR-009, FR-010]
- [x] CHK003 Is focused-window-only suppression stated with an explicit prohibition on global interception? [Completeness, Spec FR-005]
- [x] CHK004 Are the safety-critical surfaces each backed by a measurable success criterion? [Measurability, Spec SC-001..SC-005]

## Requirement Completeness

- [x] CHK005 Are both key-down and key-up transitions addressed for a bound key, not just the press? [Completeness, Spec FR-020]
- [x] CHK006 Is auto-repeat behavior specified? [Completeness, Spec FR-020]
- [x] CHK007 Is the hand-off channel behavior specified for the full-channel case? [Coverage, Spec FR-023]
- [x] CHK008 Are the default bindings enumerated, including which are suspend-exempt? [Completeness, Spec FR-014]
- [x] CHK009 Is the platform-startup failure path (for example Linux permission) specified as surfaced rather than silent? [Completeness, Spec FR-019]

## Requirement Clarity

- [x] CHK010 Is "hand off exactly one action" unambiguous about count per press? [Clarity, Spec FR-004, FR-020]
- [x] CHK011 Is the boundary with the Weave Engine clear (this slice classifies and hands off; it does not execute actions)? [Clarity, Spec Assumptions]
- [x] CHK012 Is the key identity representation defined so backends and the core agree? [Clarity, Spec FR-022]

## Requirement Consistency

- [x] CHK013 Is the binding persistence consistent with the S001 Config Store rules (additive, settings-only, no schema bump)? [Consistency, Spec FR-021, FR-016]
- [x] CHK014 Do the suspend requirements consistently exempt exactly the toggle-suspend and toggle-fishing bindings? [Consistency, Spec FR-011, FR-012]
- [x] CHK015 Is the mock/test backend requirement consistent with the safety surfaces it must exercise? [Consistency, Spec FR-018]

## Scenario and Edge Case Coverage

- [x] CHK016 Is focus-change between key-down and key-up addressed? [Coverage, Spec Edge Cases]
- [x] CHK017 Is a persisted conflicting or unknown binding entry covered with a defined fallback? [Coverage, Spec FR-017, Edge Cases]
- [x] CHK018 Is the case of a synthesized key that is also bound covered? [Coverage, Spec Edge Cases, FR-009]

## Acceptance Criteria Quality

- [x] CHK019 Can "the interception path performs no blocking work" be objectively evaluated from the requirements? [Measurability, Spec SC-004, FR-006]
- [x] CHK020 Is conflict rejection expressed as a verifiable, all-cases outcome? [Measurability, Spec SC-006, FR-015]

## Notes

- All items pass after the clarifications session added key-transition handling
  (FR-020), additive binding persistence (FR-021), platform-neutral key identity
  (FR-022), and non-blocking bounded hand-off (FR-023).
- The OS hook wiring is acknowledged as manually validated (Assumptions); the
  requirements deliberately push the safety logic behind the testable input
  abstraction.
