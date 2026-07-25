# Requirements Quality Checklist: Window Sizing Model Rebuild

**Purpose**: Validate that the sizing-model invariants and regression-avoidance
for issue #8 are completely, clearly, and consistently specified, and that each
requirement maps to a desk check or a pure unit test, before planning and
implementation.
**Created**: 2026-07-25
**Feature**: [spec.md](../spec.md)

**Note**: These are "unit tests for the requirements" - they check the spec's
quality, not the implementation.

## Boot-Floor Gating and Measured-Wins Minimum

- [x] CHK001 Is the "stable measurement" gate defined with an objective criterion (consecutive equal measurements within a stated tolerance)? [Clarity, Spec Clarifications, §FR-003]
- [x] CHK002 Is it specified that the boot floor applies only until the first stable measurement and then stops being maxed against? [Completeness, Spec §FR-002]
- [x] CHK003 Is it stated that, once stable, the enforced minimum can shrink when the content becomes smaller (no permanent latch)? [Clarity, Spec §FR-004, §SC-005]
- [x] CHK004 Is the closed-log minimum tied to the measured content height (not an oversized floor) with a measurable no-dead-band criterion? [Measurability, Spec §FR-001, §SC-001]
- [x] CHK005 Is the transient-first-frame case explicitly addressed so a large first layout cannot latch the minimum? [Edge Case, Spec §FR-003, Edge Cases]

## Log-Pane Resizability and No Phantom Band

- [x] CHK006 Is the log pane's available maximum specified as computed against the current measured content height rather than a running maximum? [Clarity, Spec Clarifications, §FR-005]
- [x] CHK007 Is "no phantom/reserved band" stated as a requirement, not just an implication? [Completeness, Spec §FR-005, §US2]
- [x] CHK008 Is the resizability-at-minimum invariant specified precisely (max strictly greater than min at the enforced minimum open height)? [Measurability, Spec §FR-006, §SC-002]
- [x] CHK009 Is the six-line minimum and the never-cover-controls constraint stated consistently with the resizability requirement? [Consistency, Spec §FR-005, §FR-011]
- [x] CHK010 Is the reserved drag room at the enforced minimum quantified (six lines plus one row)? [Clarity, Spec Clarifications, Assumptions]

## Proportional Window-Growth Split

- [x] CHK011 Is the split rule defined so a height change is distributed by the fraction each pane currently occupies? [Clarity, Spec §FR-007]
- [x] CHK012 Is it required that the split fraction be derived from live pane heights each resize, not a stored ratio? [Completeness, Spec §FR-008]
- [x] CHK013 Are per-pane clamps stated (log never below six lines, central never below its content) and where the rounding remainder goes (central)? [Completeness, Spec §FR-007]
- [x] CHK014 Is the shrink direction covered symmetrically (height removed from both panes in proportion, each held to its minimum)? [Coverage, Spec §US3, Acceptance Scenarios]

## Height-Neutral Open/Close

- [x] CHK015 Is open specified to grow by the log height actually shown and close to shrink by the pane's actual current height (not a fixed minimum delta)? [Clarity, Spec §FR-009]
- [x] CHK016 Is the resized-then-closed case explicitly required to return the window to its original height? [Coverage, Spec §US4, §SC-004]
- [x] CHK017 Is the persisted log height named as the single source of truth for the pane, restored consistently? [Consistency, Spec §FR-010, Edge Cases]

## Testability and Non-Functional

- [x] CHK018 Is it required that the sizing computations be pure, deterministic functions unit-testable without a live window? [Measurability, Spec §FR-012]
- [x] CHK019 Does each functional requirement map to either a pure unit test or a desk check (no requirement that can only be judged subjectively)? [Measurability, Spec Success Criteria]
- [x] CHK020 Is the never-clip-controls guarantee stated across all allowed sizes (open and closed)? [Coverage, Spec §FR-011]

## Scope and Consistency

- [x] CHK021 Are the out-of-scope subsystems (pixel-bus, beacon, input, fishing, settings modal) explicitly excluded to prevent scope creep? [Scope, Spec Out of Scope]
- [x] CHK022 Are the boot floor and six-line minimum stated as unchanged values (only their application changes), avoiding an unintended constant change? [Consistency, Spec Assumptions]

## Notes

- All items are satisfied by the current spec after the clarify pass; this
  checklist is the requirements-quality gate for the slice.
- CHK008/CHK010 encode the item-5 design decision (one extra line of drag room);
  they must stay consistent with the compressible-window requirement (CHK022).
