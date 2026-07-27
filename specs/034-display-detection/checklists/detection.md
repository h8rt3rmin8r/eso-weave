# Detection Checklist: Out-Of-Band Display Detection

**Purpose**: Validate the requirements governing a feature whose entire job is to observe the machine it runs on from two sources of unequal trust, one of which is a file that a game patch can change under it. The risk here is not arithmetic, as it was for the previous slice; it is a descriptor that confidently reports something it does not actually know.
**Created**: 2026-07-27
**Feature**: [spec.md](../spec.md)

**Note**: Tests the requirements, not the implementation.

## Provenance and Confidence

- [x] CHK001 Is the source of a descriptor required to travel with it, so a measured reading and a configured one are never interchangeable? [Clarity, Spec §FR-003, §US1]
- [x] CHK002 Is the measured source made authoritative without a tie-break rule that could let the file win in some corner? [Completeness, Spec §FR-018]
- [ ] CHK003 Given that the window mode may not be mapped, does the specification state the conditions under which a configured descriptor can actually be produced at all? [Conflict, Spec §FR-015, §FR-022]
- [x] CHK004 Is the refusal to name a window mode stated for both sources, not only the file? [Consistency, Spec §FR-015, §Clarifications]
- [x] CHK005 Is the reasoning for not guessing the mode mapping recorded, rather than the conclusion asserted? [Traceability, Spec §Clarifications]

## Absence and Degradation

- [x] CHK006 Is a zero or negative surface size required to be represented as absence rather than as a descriptor? [Edge Case, Spec §FR-004, §SC-006]
- [ ] CHK007 Is every field of the descriptor either always available or explicitly permitted to be absent, given that a configured reading cannot supply monitor geometry? [Gap, Spec §FR-001, §FR-022]
- [ ] CHK008 Where a platform cannot supply the display scale, is the outcome a single defined behaviour rather than a choice between two? [Ambiguity, Spec §FR-006]
- [x] CHK009 Is recovery after an undrawable window required, so a minimize does not permanently disable detection? [Coverage, Spec §FR-010, §US2]
- [x] CHK010 Is the pre-launch case (no window at all) distinguished from a failure? [Clarity, Spec §Edge Cases]

## The File

- [x] CHK011 Is per-key degradation required rather than per-file, so one renamed key does not discard the rest? [Completeness, Spec §FR-014, §Edge Cases]
- [x] CHK012 Is key matching required to tolerate the version suffix the game adds, rather than matching a literal? [Coverage, Spec §FR-013]
- [x] CHK013 Are the malformed-input cases enumerated concretely enough to be turned into tests? [Measurability, Spec §FR-014, §SC-004]
- [x] CHK014 Is the file's location required to derive from the existing directory resolution rather than a second copy of that logic? [Consistency, Spec §FR-012]
- [x] CHK015 Is it required that detection never writes, creates, or removes anything? [Completeness, Spec §FR-017, §SC-008]

## Currency and Cost

- [x] CHK016 Is re-resolution required to happen without operator action, covering move, resize, monitor change, and mode change? [Completeness, Spec §FR-007, §US2]
- [x] CHK017 Is re-resolution constrained to the existing cycle, with no new thread or timer and nothing on the hook thread? [Constraint, Spec §FR-008]
- [x] CHK018 Is the file read tied to change detection rather than given a cadence of its own? [Efficiency, Spec §FR-016, §SC-003]
- [x] CHK019 Are the diagnostics bounded in volume, given the previous slice's log-flood finding? [Consistency, Spec §FR-021, §SC-003]

## Boundaries

- [x] CHK020 Is it explicit that nothing consumes the descriptor and that the wrap layout is not built here? [Clarity, Spec §FR-025]
- [x] CHK021 Is the addon and the existing pixel-bus contract named as untouched, with a verifiable criterion? [Traceability, Spec §FR-023, §SC-009]
- [x] CHK022 Is a seam required that makes the desk-testable portion actually testable without hardware? [Completeness, Spec §FR-026]
- [x] CHK023 Is the inference about the mode mapping explicitly barred from affecting behaviour? [Constraint, Spec §FR-020]

## Notes

### Findings from the first run (2026-07-27)

20 of 23 passed. The three failures are all the same underlying mistake, and it is
worth naming: the specification described the configured path as though it were a
slightly worse version of the measured path, when in fact it is a much narrower
thing that can answer far fewer questions.

**CHK003 failed, and it is the substantive one.** FR-022 permitted a configured
descriptor "where the stored settings determine which resolution pair is live",
while FR-015 forbids using the mode value to determine exactly that. Taken
together, the condition in FR-022 can essentially never be met, so the requirement
was dead text that read like a working feature. Two honest options: drop the
configured path entirely, or state the one case where it genuinely is
determinable. Kept, narrowly: a configured descriptor is produced only when both
stored pairs are identical, because then the unmapped mode value does not matter.
That is a real case (a great many installs run one resolution) and it is the only
one that can be served without guessing. FR-022 now says so.

**CHK007 failed.** FR-001 requires the descriptor to carry the display's position,
size, and scale, but the file records only a display *index*, which cannot be
turned into geometry without asking the operating system, which is precisely what
the configured path does not have. So a configured descriptor could never satisfy
FR-001 as written. Fixed by making the display fields explicitly absent-capable
and stating that a configured descriptor carries the surface size and nothing
else. The surface size is the field the wrap layout is bounded by, so a descriptor
without display geometry is still useful; one that fabricated it would not be.

**CHK008 failed.** FR-006 allowed a missing scale to degrade to "an explicit
unknown or an unscaled default", which is two behaviours joined by an "or", so two
implementations could both satisfy it and disagree. Fixed to require unknown. A
fabricated 1.0 is indistinguishable from a genuinely unscaled display, which makes
it exactly the kind of confident wrong answer this checklist exists to catch.

### Second run (2026-07-27, after spec fixes)

All 23 pass.
