# Requirements Quality Checklist: Primary and Skills Panel Controls

**Purpose**: Validate that the requirements for the addon Update control, status
alignment, dropdown stability, and the Delay column are complete, clear, and
consistent before planning.
**Created**: 2026-07-13
**Feature**: [spec.md](../spec.md)

## Requirement Completeness

- [ ] CHK001 - Is the Update control's placement (with Install and Uninstall) specified? [Completeness, Spec FR-001]
- [ ] CHK002 - Is the exact enable/disable condition for Update specified (disabled when not installed, enabled when installed even if current)? [Completeness, Spec FR-002]
- [ ] CHK003 - Is Update's behavior defined as uninstall-then-install with a defined end state (installed and current)? [Completeness, Spec FR-003]
- [ ] CHK004 - Are requirements defined for both delay states (override on and off)? [Completeness, Spec FR-008, FR-009]
- [ ] CHK005 - Is the delay edit-buffer behavior specified so in-progress input is not clobbered? [Completeness, Spec FR-011]

## Requirement Clarity

- [ ] CHK006 - Is "aligned in the same columns" defined precisely for the Weapon Bar row? [Clarity, Spec FR-004]
- [ ] CHK007 - Is "constant resting width" for dropdowns objectively verifiable (no width change across selections)? [Clarity, Spec FR-005, FR-006]
- [ ] CHK008 - Is "comfortably show at least four digits" and "right-aligned" specified for the delay field? [Clarity, Spec FR-010]
- [ ] CHK009 - Is the exact header text (Delay (ms)) stated? [Clarity, Spec FR-007]

## Requirement Consistency

- [ ] CHK010 - Is the same fixed-width dropdown treatment stated to be reusable for the later settings-dropdown slice? [Consistency, Spec FR-006]
- [ ] CHK011 - Do the override-on and override-off delay fields share the same width and appearance so toggling does not shift the row? [Consistency, Spec FR-009, SC-004]
- [ ] CHK012 - Is the managed-marker uninstall safety rule stated to remain unweakened by Update? [Consistency, Spec FR-003, FR-012]

## Acceptance Criteria Quality

- [ ] CHK013 - Are the success criteria measurable without implementation detail (0 px row shift, four-digit fit, exact header)? [Measurability, Spec SC-001..SC-005]
- [ ] CHK014 - Can "Update returns the addon to installed and current" be objectively verified? [Measurability, Spec SC-001]

## Scenario & Edge Case Coverage

- [ ] CHK015 - Is the unmanaged-folder Update path covered (no delete, reinstall proceeds)? [Coverage, Edge Case, Spec FR-003]
- [ ] CHK016 - Is the AddOns-not-found state covered for Update (no crash, control disabled)? [Coverage, Edge Case]
- [ ] CHK017 - Is a zero delay accepted, and is non-numeric input rejected and bounded to four digits? [Coverage, Spec FR-011]

## Constitution & Scope

- [ ] CHK018 - Is it stated that all changes are presentation-layer and do not touch safety-critical surfaces? [Assumption, Spec FR-012, Constitution II]
- [ ] CHK019 - Is the model-level Update intent identified as the unit-tested surface while the visual behavior is validated observationally? [Assumption, Spec Assumptions, Constitution III]

## Notes

- Items validate requirement quality, not the eventual code. All reference a spec
  section or an edge case/assumption for traceability.
