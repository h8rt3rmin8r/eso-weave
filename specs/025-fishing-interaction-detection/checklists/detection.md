# Detection Requirements Checklist: Fishing Interaction Detection Rewrite

**Purpose**: Validate requirements quality for the addon detection contract,
observability, safety invariants, and release readiness before planning
**Created**: 2026-07-13
**Feature**: [spec.md](../spec.md)

## Addon Detection Correctness

- [x] CHK001 - Is the polling cadence bound explicitly quantified rather than
  described with a vague adjective? [Clarity, Spec FR-001]
- [x] CHK002 - Is the removal of the failure-alert event dependency stated as
  its own testable requirement, not implied? [Completeness, Spec FR-002]
- [x] CHK003 - Are both bite signals (interact-prompt primary and
  bait-consumption secondary) individually specified with their scoping
  rules? [Completeness, Spec FR-003, FR-004]
- [x] CHK004 - Is bite precedence defined with an exhaustive list of clearing
  conditions, so routine polling can never demote a bite? [Clarity, Spec
  FR-005, Edge Cases]
- [x] CHK005 - Is language independence a requirement on the prompt
  comparison rather than only an assumption? [Coverage, Spec FR-003, Edge
  Cases]
- [x] CHK006 - Is the stability of the rendered signal contract (colors,
  positions, geometry, unchanged decoder) explicit and objectively
  verifiable? [Consistency, Spec FR-007, SC-005]
- [x] CHK007 - Is the idle-return timing on interaction end quantified?
  [Measurability, Spec FR-006]
- [x] CHK008 - Are false-bite scenarios from unrelated inventory changes
  addressed with a measurable outcome? [Edge Case, Spec FR-004, SC-004]
- [x] CHK009 - Is the no-bait-selected scenario's expected behavior defined
  end to end (game, addon, application, log)? [Edge Case, Spec Edge Cases]
- [x] CHK010 - Is the mid-cast menu/dialog interruption scenario defined?
  [Edge Case, Spec Edge Cases]

## Observability Completeness

- [x] CHK011 - Is the set of logged transitions enumerated exhaustively
  rather than described as "all transitions"? [Completeness, Spec FR-009]
- [x] CHK012 - Are the log level and component attribution for fishing-engine
  entries specified? [Clarity, Spec FR-009]
- [x] CHK013 - Is the log-only constraint (no behavior change, existing tests
  unchanged) stated testably? [Measurability, Spec FR-010]
- [x] CHK014 - Is post-failure diagnosability from the log alone a measurable
  success criterion? [Measurability, Spec SC-003]

## Safety Invariants

- [x] CHK015 - Is signal-loss degradation to disabled restated as an
  unchanged requirement of this feature? [Coverage, Spec FR-012]
- [x] CHK016 - Is the managed-marker uninstall guarantee restated as an
  unchanged requirement covering the update path? [Coverage, Spec FR-012,
  User Story 3]
- [x] CHK017 - Does the scope stay within the screen-signal contract with no
  new in-game surface beyond the corrected detection? [Consistency,
  Constitution V, Spec FR-007]

## Release and Validation Readiness

- [x] CHK018 - Is the addon delivery path (manifest version advance, existing
  update control) specified as a requirement? [Completeness, Spec FR-008]
- [x] CHK019 - Are the success criteria that require a live game session
  identified as the owed in-game validation? [Dependency, Spec SC-001,
  SC-002, SC-004, Assumptions]
- [x] CHK020 - Are the documentation obligations (master specification
  language, dated changelog decision, research citations) captured as
  requirements? [Completeness, Spec FR-011]
- [x] CHK021 - Is the rejected arm-timeout migration recorded with its
  rationale? [Assumption, Spec Assumptions]

## Notes

- All items evaluated against spec.md on 2026-07-13 during the autopilot
  checklist pass; no requirement-quality gaps found. Live-game success
  criteria (CHK019) are tracked as owed validation, consistent with slice
  024's precedent.
