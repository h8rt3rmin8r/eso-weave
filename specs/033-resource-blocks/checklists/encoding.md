# Encoding Checklist: PixelBeacon Resource Blocks

**Purpose**: Validate the requirements governing how an ordered numeric quantity is put on the wire and read back. This is the first signal on this strip that is a number rather than a state, so the properties that matter are different from every previous block's, and the requirements have to say so precisely enough to be dischargeable.
**Created**: 2026-07-27
**Feature**: [spec.md](../spec.md)

**Note**: Tests the requirements, not the implementation.

## Error Behaviour

- [ ] CHK001 Is the bounded-error guarantee stated in a form that is true of the encoding actually proposed, including what a checksum does to samples perturbed in the same direction? [Conflict, Spec §FR-009]
- [x] CHK002 Is the distinction between a wrong value and a refusal to decode drawn, and is one of them clearly acceptable? [Clarity, Spec §FR-009, §US2]
- [x] CHK003 Is monotonicity required, not just accuracy, so ordering survives decoding? [Completeness, Spec §FR-010]
- [x] CHK004 Are the error requirements stated over the whole input space rather than for chosen values, so they can be discharged exhaustively? [Measurability, Spec §SC-002, §SC-003]
- [x] CHK005 Is the argument for the chosen encoding recorded against the alternative the source issue specified, rather than asserted? [Traceability, Spec §Clarifications]

## Value Domain

- [x] CHK006 Is the published range explicitly bounded, including both endpoints? [Clarity, Spec §FR-001, §US1]
- [ ] CHK007 Does the specification define how "unavailable" is represented on the wire, given that the square is never hidden and the value range does not obviously leave room for it? [Gap, Spec §FR-003, §FR-005]
- [x] CHK008 Is a value outside the published range required to decode as unavailable rather than being clamped into range? [Coverage, Spec §FR-008]
- [x] CHK009 Is the meaning of zero distinguished from the meaning of unavailable? [Ambiguity, Spec §Key Entities, §Edge Cases]
- [x] CHK010 Is the denominator specified, and required to be current rather than cached? [Clarity, Spec §FR-002]
- [x] CHK011 Is the zero-maximum case addressed rather than left to divide by zero? [Edge Case, Spec §FR-003]

## Independence and Compatibility

- [x] CHK012 Are the three resources required to decode independently, so one bad sample does not poison the others? [Completeness, Spec §FR-011]
- [x] CHK013 Are requirements defined for an addon that predates these blocks? [Coverage, Spec §US3, §SC-005]
- [x] CHK014 Is clearing-on-failure required rather than holding a stale value, consistent with the other blocks added recently? [Consistency, Spec §FR-012]
- [x] CHK015 Is each square required to carry a validity mark distinct from every other on the strip? [Completeness, Spec §FR-004]

## Volume and Observability

- [x] CHK016 Is the update volume of these signals acknowledged as different from every previous block's? [Completeness, Spec §Clarifications]
- [x] CHK017 Is the logging level requirement stated with its reason, given that it contradicts the precedent set by the two preceding slices? [Consistency, Spec §FR-014]
- [x] CHK018 Is there a measurable criterion for "does not flood the log"? [Measurability, Spec §SC-006]
- [x] CHK019 Is change-only emission required, so a steady resource is quiet? [Completeness, Spec §FR-006]

## Boundaries

- [x] CHK020 Is it explicit that nothing consumes these values? [Clarity, Spec §FR-015]
- [x] CHK021 Are the safety surfaces this feature must not touch named? [Completeness, Spec §FR-016]
- [x] CHK022 Is the strip's growth acknowledged, and its relationship to the grid-wrap work noted? [Traceability, Spec §Edge Cases]

## Notes

### Findings from the first run (2026-07-27)

20 of 22 passed. Both failures are real, and the first one matters.

**CHK001 failed.** FR-009 promised that a sample perturbed within tolerance decodes
to a value within tolerance of the published one. That is not true of the proposed
encoding, and would not be true of any encoding on this strip. The payload channel
and the checksum channel are validated together: if both drift in the same
direction by the full tolerance, their sum drifts by twice it and the sample is
**rejected**, decoding to unavailable rather than to a near-correct value. So the
requirement as written is false, and an implementation satisfying it literally
would have to abandon the checksum.

Fixed by stating the property that is both true and actually wanted: a perturbed
sample decodes either to a value within tolerance or to unavailable, and **never to
a wrong value**. Rejection is a safe outcome; a plausible but wrong percentage is
not. Worth noting the existing latency block has exactly this characteristic today
and nobody had written it down.

**CHK007 failed.** FR-003 requires publishing "unavailable" when a maximum cannot
be read, and FR-005 forbids expressing state by hiding the square, but nothing said
how unavailable is represented on the wire. Since the payload range is 0 to 100 and
a channel holds 0 to 255, there is ample room; the point is that both sides must
agree on the value rather than each inventing one. Added to FR-003, with the
observation that any out-of-range payload already decodes to unavailable, so the
sentinel needs agreement but no new decode rule.

### Second run (2026-07-27, after spec fixes)

All 22 pass.
