# Geometry Checklist: Pixel Bus Grid Wrap

**Purpose**: Validate the requirements governing a coordinate contract that two independently written programs must satisfy identically. Unlike the encoding checklists of the previous slices, nothing here is about colour: the risk is entirely that a position is computed two different ways, or that a region is described in a way that is true of the shipped configuration and false of the next one.
**Created**: 2026-07-27
**Feature**: [spec.md](../spec.md)

**Note**: Tests the requirements, not the implementation.

## The Mapping

- [x] CHK001 Is the index-to-position rule stated concretely enough that two independent implementations must agree? [Clarity, Spec §FR-001]
- [x] CHK002 Is the mapping required to be injective, so no two squares can be drawn on top of each other? [Completeness, Spec §FR-005]
- [ ] CHK003 Are the mapping's coverage claims true for a partial final row, not only for counts that fill their rows exactly? [Conflict, Spec §FR-005, §Edge Cases]
- [x] CHK004 Is the heartbeat square's position pinned independently of the column count? [Completeness, Spec §FR-004, §US3]
- [x] CHK005 Is the whole-pixel sample-point property carried forward to the new axis rather than assumed? [Coverage, Spec §FR-006]

## The Shared Constant

- [x] CHK006 Is the column count required to be fixed and forbidden from being measured on either side? [Clarity, Spec §FR-007]
- [x] CHK007 Is agreement enforced by the build rather than by review, with a failure that names the disagreement? [Completeness, Spec §FR-008, §US2]
- [x] CHK008 Is the argument for a fixed count recorded against the derived alternative the prerequisite issue assumed, rather than asserted? [Traceability, Spec §Clarifications]
- [x] CHK009 Is the lower bound on the column count tied to its reason (the no-change-today property) rather than stated as a bare number? [Clarity, Spec §FR-009]
- [ ] CHK010 Is the upper bound dischargeable, given that "the smallest client area the game supports" is an assumption rather than a measured fact? [Measurability, Spec §FR-010, §Assumptions]
- [x] CHK011 Can the wrap's properties be tested at column counts other than the shipped one? [Measurability, Spec §FR-011]

## The Region

- [x] CHK012 Is the sampled region required to cover every occupied square and no unoccupied row or column? [Completeness, Spec §FR-003]
- [x] CHK013 Is the narrow-grid case (fewer squares than one row) distinguished from the full-row case, so a nine-square grid does not sample a sixteen-square width? [Coverage, Spec §FR-003, §Edge Cases]
- [x] CHK014 Is the addon's drawn extent required to derive from the same arithmetic as the sampled region, rather than merely to match it? [Consistency, Spec §FR-003]
- [ ] CHK015 Is it explicit that the compile-time bound constrains width only, and that height is the runtime fit check's responsibility? [Gap, Spec §FR-010, §FR-016]

## Nothing Changes Today

- [x] CHK016 Is the no-change property stated as a requirement rather than left as a consequence of the arithmetic? [Completeness, Spec §FR-012, §FR-013]
- [x] CHK017 Is it required to be *tested*, given that an implementer could reasonably consider it self-evident? [Measurability, Spec §FR-014, §SC-002]
- [x] CHK018 Is the mixed-version case (previous addon, wrapped application) addressed? [Coverage, Spec §Edge Cases]
- [x] CHK019 Are the existing encodings, markers, and cadences named as untouched? [Constraint, Spec §FR-015, §FR-022]

## The Fit Check

- [x] CHK020 Is the check required to be advisory, with an explicit statement that nothing branches on it? [Constraint, Spec §FR-017]
- [x] CHK021 Is the no-measurement case defined, including the stored-settings descriptor? [Edge Case, Spec §FR-018, §US4]
- [x] CHK022 Is the report's level and cadence specified, with the reason it differs from the recent slices' debug-level convention? [Clarity, Spec §FR-019]
- [x] CHK023 Is the failure the check exists to catch described, so its value is assessable? [Traceability, Spec §US4, §Clarifications]
- [ ] CHK024 Is repeated crossing of the fit boundary (a window dragged across the threshold) considered, given that each crossing is a genuine outcome change? [Edge Case, Spec §FR-019, §Edge Cases]

## Boundaries

- [x] CHK025 Is it explicit that no square, signal, or marker is added or repurposed? [Clarity, Spec §FR-022]
- [x] CHK026 Is the capacity this creates explicitly left unconsumed? [Clarity, Spec §FR-024]
- [x] CHK027 Is the manifest advance justified, given that the drawn output does not change? [Traceability, Spec §FR-020, §Clarifications]

## Notes

### Findings from the first run (2026-07-27)

23 of 27 passed. One failure is a factual error in a requirement and the other
three are gaps of the "true today, misleading later" kind that this checklist
exists to find.

**CHK003 failed, and it is a genuine defect.** FR-005 required that "no position
within the occupied region may be unreachable from some index below the square
count". That is false the moment the square count is not a multiple of the column
count. At seventeen squares and sixteen columns the region is sixteen wide and two
tall, so it contains thirty-two positions and only seventeen are reachable; the
other fifteen are empty cells beside the last square. An implementation satisfying
FR-005 literally would have to forbid partial rows, which would forbid every block
count that is not a multiple of sixteen. The injectivity half was right and is
what actually matters (two squares must never collide); the coverage half was
wrong and has been replaced with the property that was meant: every index below
the count maps to a position inside the region.

**CHK010 failed.** FR-010 required the widest grid at the largest square size to
fit "the smallest client area the game supports", which is a quantity the
specification never states, so the requirement could not be discharged without
the reader supplying a number of their own. The Assumptions section had the
number (1024 pixels wide) but nothing connected them. FR-010 now names it and
points at the assumption, so the check is arithmetic rather than judgement.

**CHK015 failed.** FR-010 bounds the grid's width at compile time and nothing
said why height gets no equivalent bound, which reads as an oversight. It is not:
the row count grows with the square count over the project's life, so no fixed
bound could be stated, and that is precisely the job the runtime fit check exists
to do. Stated in FR-010 so the asymmetry is visibly deliberate.

**CHK024 failed, mildly.** FR-019 reports on every change of the fit outcome,
and a window dragged slowly across the threshold changes the outcome repeatedly.
Each of those is a real transition rather than a repeat, so the change-detection
rule is doing its job, but the possibility deserved acknowledging rather than
discovering. Added as an edge case with the reason it is acceptable: the
transitions are human-paced, they stop when the drag stops, and a warning that
appears and disappears as the window crosses a boundary is describing exactly
what is happening.

### Second run (2026-07-27, after spec fixes)

All 27 pass.
