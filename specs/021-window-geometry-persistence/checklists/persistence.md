# Requirements Quality Checklist: Window Geometry Persistence

**Purpose**: Validate that the requirements for capturing and restoring window
geometry are complete, clear, consistent, and measurable before planning.
**Created**: 2026-07-13
**Feature**: [spec.md](../spec.md)

## Requirement Completeness

- [ ] CHK001 - Are the exact geometry attributes to persist (position, size, maximized) explicitly enumerated? [Completeness, Spec FR-001]
- [ ] CHK002 - Is the trigger for capturing geometry (on change while running) specified? [Completeness, Spec FR-001]
- [ ] CHK003 - Is the trigger for restoring geometry (on launch) specified? [Completeness, Spec FR-002]
- [ ] CHK004 - Are requirements defined for the case where geometry changes immediately before close? [Completeness, Spec FR-008]
- [ ] CHK005 - Is the storage location (session state, separate from user settings) specified as a requirement rather than left implicit? [Completeness, Spec FR-003]
- [ ] CHK006 - Are requirements defined for loading a pre-feature saved state with no geometry? [Completeness, Spec FR-007]

## Requirement Clarity

- [ ] CHK007 - Is "same monitor" defined precisely enough to be verifiable (e.g., same monitor when layout unchanged)? [Clarity, Spec FR-004]
- [ ] CHK008 - Is "off-screen" / "no longer visible on any connected monitor" defined with an objective boundary? [Clarity, Spec FR-005]
- [ ] CHK009 - Is "degenerate or out-of-range size" quantified (zero, negative, non-finite, above a bound, below minimum)? [Clarity, Spec FR-006]
- [ ] CHK010 - Is the default fallback geometry unambiguously identified (the existing default size and minimum)? [Clarity, Spec Assumptions]

## Requirement Consistency

- [ ] CHK011 - Do the maximized-restore requirement and the exact-size-restore requirement stay consistent (unmaximize then move restores normal size, not maximized)? [Consistency, Spec US2]
- [ ] CHK012 - Is the persistence-format requirement (UTF-8 no BOM, LF, trailing newline) consistent with the existing state-file convention? [Consistency, Spec FR-010]
- [ ] CHK013 - Is the requirement that suspend/fishing restore is unchanged consistent with adding geometry to the same session record? [Consistency, Spec FR-009]

## Acceptance Criteria Quality

- [ ] CHK014 - Are the success criteria measurable without reference to implementation (e.g., 100% of relaunches restore within reported precision)? [Measurability, Spec SC-001..SC-005]
- [ ] CHK015 - Can "opens fully visible on a connected monitor" be objectively verified? [Measurability, Spec SC-003]
- [ ] CHK016 - Is "no lost final change" expressed as a testable outcome? [Measurability, Spec SC-005]

## Scenario & Edge Case Coverage

- [ ] CHK017 - Are requirements defined for first-ever launch with no recorded geometry? [Coverage, Spec US1 scenario 2]
- [ ] CHK018 - Are recovery requirements defined for a disconnected/changed monitor layout? [Coverage, Edge Case]
- [ ] CHK019 - Are requirements defined for a missing or unreadable state file (open at default, no crash)? [Coverage, Edge Case]
- [ ] CHK020 - Is the migration-failure / malformed-geometry path covered (treat as no recorded geometry)? [Coverage, Spec FR-010]

## Constitution & Dependencies

- [ ] CHK021 - Is the requirement that new correctness logic (change detection, sanitize/clamp, migration) be unit-testable via pure helpers reflected, keeping the rendering layer thin? [Assumption, Constitution III]
- [ ] CHK022 - Is the schema-version bump with forward migration stated as an additive, backward-compatible change? [Consistency, Spec FR-010]
- [ ] CHK023 - Is the off-screen recovery's platform coverage (virtual-screen bounds where available; size sanity otherwise) documented as an assumption rather than left ambiguous? [Assumption, Spec Assumptions]

## Notes

- Items validate the quality of the written requirements, not the eventual code.
- All items reference a spec section or are marked as a gap/assumption for
  traceability.
