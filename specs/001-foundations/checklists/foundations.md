# Requirements Quality Checklist: Foundations

**Purpose**: Validate the clarity, completeness, and consistency of the
Config Store and Logging requirements before planning. These are unit tests for
the requirements themselves, not for the implementation.
**Created**: 2026-07-11
**Feature**: [spec.md](../spec.md)

## Requirement Completeness

- [x] CHK001 Are the corruption-fallback requirements complete (fall back to defaults, preserve the file, and surface a notice all specified)? [Completeness, Spec FR-009]
- [x] CHK002 Are forward-migration requirements specified for both recognized and unknown keys (what is migrated, what is warned, what is never discarded)? [Completeness, Spec FR-007, FR-008]
- [x] CHK003 Is the default settings form documented (what values a fresh install starts with)? [Completeness, Spec Key Entities]
- [x] CHK004 Is the ring buffer required to remain available independently of whether the file sink is enabled? [Completeness, Spec FR-012]
- [x] CHK005 Are the monthly file-sink requirements complete (file name, location, and rollover at a month boundary)? [Completeness, Spec FR-013, Edge Cases]
- [x] CHK006 Are read-failure and write-failure of the settings location both covered, not just one? [Coverage, Spec FR-010]

## Requirement Clarity

- [x] CHK007 Is the `.invalid` preservation naming and its collision behavior unambiguous? [Clarity, Spec FR-009]
- [x] CHK008 Is "settings only, no runtime or session state" clarified with concrete examples of excluded state? [Clarity, Spec FR-006]
- [x] CHK009 Are the on-disk encoding requirements (UTF-8 without BOM, line-feed endings, pretty-printed) explicit and objectively checkable? [Clarity, Spec FR-005]
- [x] CHK010 Is it clear that a runtime level change takes effect without a restart? [Clarity, Spec FR-011]
- [x] CHK011 Is the persisted log line format specified precisely (field set and order: timestamp, level, source, message)? [Clarity, Spec FR-014]
- [x] CHK012 Is the timestamp requirement unambiguous that it is coordinated universal time in ISO-8601 form? [Clarity, Spec FR-014]

## Requirement Consistency

- [x] CHK013 Are FR-006 (settings only) and FR-017 (logging initializes from and saves to settings) consistent, with no runtime state implied to leak into the file? [Consistency, Spec FR-006, FR-017]
- [x] CHK014 Is the Settings entity (schema version plus logging preferences) consistent with FR-016 and the Log Configuration entity? [Consistency, Spec Key Entities, FR-016]
- [x] CHK015 Does FR-015's reference to "while suspended" and "keystrokes" stay consistent with this slice excluding the Input Engine, or is that dependency called out? [Conflict, Spec FR-015]

## Acceptance Criteria Quality

- [x] CHK016 Can the privacy constraint (no input contents above debug) be objectively verified from the requirements as written? [Measurability, Spec FR-015, SC-005]
- [x] CHK017 Is the corruption-resilience success criterion measurable without reference to implementation? [Measurability, Spec SC-003]
- [x] CHK018 Are the quality-gate outcomes (format, lint, tests) expressed as measurable pass conditions? [Measurability, Spec SC-001]

## Scenario and Edge Case Coverage

- [x] CHK019 Is the ring buffer overflow or eviction behavior specified for when capacity is reached? [Coverage, Gap, Spec FR-012]
- [x] CHK020 Is the behavior at a month boundary while the file sink is enabled specified? [Coverage, Spec Edge Cases]
- [x] CHK021 Is the behavior specified when the preserved-name target already exists? [Coverage, Spec Edge Cases, FR-009]

## Ambiguities, Assumptions, and Dependencies

- [x] CHK022 Is "surface a notice" defined well enough to be verifiable in this slice, given the GUI is out of scope? [Ambiguity, Spec FR-009, FR-010]
- [x] CHK023 Are the chosen defaults (active level, file sink off, buffer capacity 1000) documented as deliberate decisions rather than left implicit? [Assumption, Spec Assumptions]
- [x] CHK024 Is a requirement and acceptance-criteria identifier scheme established and used consistently across the spec? [Traceability]

## Notes

- These items test whether the requirements are well written, not whether code works.
- CHK015 and CHK022 flag genuine cross-slice or cross-cutting tensions worth
  resolving in the spec before planning.
