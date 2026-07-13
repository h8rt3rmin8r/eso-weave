# Requirements Quality Checklist: Settings Modal, Logging, and Keys

**Purpose**: Validate that the requirements for the F2 key, friendly key names,
log-level linkage, modal sizing, and the success toast are complete, clear, and
consistent before planning.
**Created**: 2026-07-13
**Feature**: [spec.md](../spec.md)

## Requirement Completeness

- [ ] CHK001 - Is the missing-F2 defect stated as a concrete requirement (F2 selectable and shown when bound)? [Completeness, Spec FR-001]
- [ ] CHK002 - Is the full set of friendly key names enumerated? [Completeness, Spec FR-002]
- [ ] CHK003 - Is the bidirectional log linkage specified for both directions plus the capture effect? [Completeness, Spec FR-003]
- [ ] CHK004 - Is the panel-visibility exclusion from verbosity specified? [Completeness, Spec FR-004]
- [ ] CHK005 - Is the modal sizing specified for both width and height and for both open-time and resize? [Completeness, Spec FR-005, FR-006]

## Requirement Clarity

- [ ] CHK006 - Is "progressively smaller fraction while absolute size increases up to a maximum" objectively describable and testable? [Clarity, Spec FR-006]
- [ ] CHK007 - Is "green success color" tied to a defined brand color and a legibility expectation? [Clarity, Spec FR-007, Assumptions]
- [ ] CHK008 - Is "stored and parsed key values unchanged" explicit so the friendly names are display-only? [Clarity, Spec FR-002]

## Requirement Consistency

- [ ] CHK009 - Is the log linkage consistent with the rule that hiding the panel does not change verbosity? [Consistency, Spec FR-003, FR-004]
- [ ] CHK010 - Is the settings-dropdown fixed-width requirement consistent with the shared helper from the prior slice? [Consistency, Spec FR-008, Assumptions]
- [ ] CHK011 - Is the modal never-exceeds-window rule consistent with the grow-to-maximum rule? [Consistency, Spec FR-006, Edge Cases]

## Acceptance Criteria Quality

- [ ] CHK012 - Are the success criteria measurable without implementation detail (F2 selectable, no raw strings, both controls match, modal bounded, toast visibly green)? [Measurability, Spec SC-001..SC-005]
- [ ] CHK013 - Is the unit-test expectation for the pure helpers and the linkage stated? [Measurability, Spec FR-010]

## Scenario & Edge Case Coverage

- [ ] CHK014 - Is the near-simultaneous change of the two log controls covered (last change wins, no inconsistent state)? [Coverage, Edge Case]
- [ ] CHK015 - Is the very-small-window case covered (modal never exceeds the window)? [Coverage, Edge Case, Spec FR-006]
- [ ] CHK016 - Is the very-large-window case covered (modal stops at its maximum)? [Coverage, Edge Case, Spec FR-006]

## Constitution & Scope

- [ ] CHK017 - Is it stated that all changes are presentation-layer and do not touch safety-critical surfaces? [Assumption, Spec FR-009, Constitution II]
- [ ] CHK018 - Are the tested surfaces (modal-sizing helper, key names, linkage) identified while the visual behavior is validated observationally? [Assumption, Spec FR-010, Constitution III]

## Notes

- Items validate requirement quality, not the eventual code. All reference a spec
  section or an edge case/assumption for traceability.
