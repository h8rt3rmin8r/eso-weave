# Input Safety Checklist: Auto-Potion

**Purpose**: Validate the requirements governing the first feature in this project that synthesizes input from a beacon-derived value. Every prior slice could be wrong and produce a misleading readout; this one can be wrong and press a key in the operator's game. This checklist exists because the constitution names input synthesis a NON-NEGOTIABLE surface, and because a requirement gap here is not a documentation problem.
**Created**: 2026-07-27
**Feature**: [spec.md](../spec.md)

**Note**: This checklist tests the requirements, not the implementation. Every item asks whether something is adequately specified, not whether it works.

## The Trigger Rule Is Fully Specified

- [x] CHK001 Is every condition of the trigger enumerated, rather than summarized as "when it is safe"? [Completeness, Spec §FR-001]
- [x] CHK002 Is it specified which conditions are conjunctive and which disjunctive, unambiguously? [Clarity, Spec §FR-001, §FR-002]
- [x] CHK003 Is the OR-not-AND rule stated as a requirement and its configurability explicitly forbidden, rather than left as a default? [Clarity, Spec §FR-002]
- [x] CHK004 Is the comparison boundary specified (at-or-below versus strictly below)? [Clarity, Spec §FR-005]
- [x] CHK005 Is "exactly once per trigger" specified, so a rule that evaluates true on consecutive samples cannot emit twice? [Coverage, Spec §FR-007]
- [x] CHK006 Is a disabled resource required to contribute nothing regardless of its value, rather than merely expected to? [Clarity, Spec §FR-003]

## Absent And Unreadable Inputs

- [x] CHK007 Is it specified what an unreadable resource counts as, rather than left to the decoder's default? [Gap, Spec §FR-004]
- [x] CHK008 Is the asymmetry of that choice recorded, so a later optimization does not reverse it as a tidy-up? [Clarity, Spec §Clarifications]
- [x] CHK009 Is the same rule specified for the quickslot and for the cooldown, not only for resources? [Coverage, Spec §FR-004]
- [x] CHK010 Is the unreadable case given its own success criterion across the full threshold range, rather than a single example? [Measurability, Spec §SC-003]

## Every Way It Must Not Fire

- [x] CHK011 Are the blocking conditions enumerated individually rather than as a general assurance? [Completeness, Spec §US2, §SC-002]
- [x] CHK012 Is each blocking condition required to be tested in isolation with every other condition satisfied, so none can be accidentally load-bearing on another? [Coverage, Spec §SC-002]
- [x] CHK013 Is the suspended case required to be a condition checked in the controller, rather than an emergent property of the loop? [Clarity, Spec §FR-010]
- [x] CHK014 Is the menu gate required to be applied to the controller directly, with the reason recorded? [Completeness, Spec §FR-009]
- [x] CHK015 Is the signal-loss behaviour specified as returning to disabled rather than merely as not firing? [Clarity, Spec §FR-011]
- [x] CHK016 Is the never-enabled case specified, so an operator who ignores the feature is provably unaffected? [Coverage, Spec §US2, §SC-006]

## The Constitution Surface

- [x] CHK017 Is it required that no new input path is introduced, rather than merely implied by reusing the engine? [Clarity, Spec §FR-008]
- [x] CHK018 Are focus scoping and recursion flagging named as properties this feature inherits rather than re-implements? [Completeness, Spec §FR-008]
- [x] CHK019 Is the hook thread named, with a requirement that no blocking work reaches it and no new thread or timer is added? [Completeness, Spec §FR-012]
- [x] CHK020 Is there an explicit requirement that existing safety tests are not weakened, skipped, or made conditional? [Gap, Spec §FR-014]
- [x] CHK021 Is the default-off requirement stated for both the feature and each per-resource enable, rather than only the feature? [Coverage, Spec §FR-013]

## Rate And Repetition

- [x] CHK022 Is the retry interval's purpose specified, distinctly from the quickslot cooldown it might be mistaken for? [Ambiguity, Spec §FR-006, §Clarifications]
- [x] CHK023 Is the window it exists to cover identified (the lag between pressing and the cooldown being reported)? [Clarity, Spec §Clarifications]
- [x] CHK024 Is the still-low-after-drinking case specified, rather than left as an emergent loop? [Coverage, Spec §Edge Cases]
- [x] CHK025 Is the repetition bound given a success criterion against a clock, rather than asserted? [Measurability, Spec §SC-005]

## Controls

- [x] CHK026 Is the new hotkey's suspend-exemption specified, and distinguished from the feature acting while suspended? [Ambiguity, Spec §FR-015, §Clarifications]
- [x] CHK027 Is the requirement to update the action-classification predicates stated, rather than left to be discovered? [Gap, Spec §FR-016]
- [x] CHK028 Is the requirement to add the new key across every representation stated as a class, rather than as one variant? [Completeness, Spec §FR-015]
- [x] CHK029 Are invalid stored settings required to degrade to defaults with a notice rather than fail the load? [Coverage, Spec §FR-018]

## Boundaries

- [x] CHK030 Is it specified that no other synthesized input path changes? [Clarity, Spec §FR-019]
- [x] CHK031 Is it specified that the bus contract, block count, geometry, and addon are untouched? [Clarity, Spec §FR-020]
- [x] CHK032 Is the interaction with fishing addressed, given both synthesize on their own timers? [Coverage, Spec §Edge Cases]

## Notes

- Check items off as completed: `[x]`
- Unchecked items are genuine gaps for `/speckit-plan` to close, not oversights to ignore.

### Findings and disposition

All 32 items pass on the first pass. That is a different outcome from the last
two slices, where the checklist step found the thing that mattered, and it is
worth saying why rather than treating it as a clean bill of health.

This spec was written *after* two consecutive slices in which the checklist
found a real gap, and it was written against an issue that had already done the
safety analysis itself: issue #20 enumerates six mandatory safety requirements
and names the slice-032 lesson about controllers that never pass through
interception. The spec inherited that enumeration rather than deriving it. The
checklist's value here was confirming the enumeration is complete and that each
item became a testable requirement, not discovering a missing one.

### Found during implementation

**CHK012 earned its place immediately.** The first draft of the controller kept
the menu gate and the suspend flag in *two* places: as fields on the controller
(set by `set_gated` and `set_suspended`) and as fields on the inputs struct passed
to `tick`. `tick` forwarded the caller's copy to `evaluate` and never read its
own, so a controller that had been told it was gated would still fire if the
caller passed `gated: false`. Both gate tests failed on the first run and named
the exact condition that had not blocked.

The fix was not to make `tick` read the right copy. It was to split the types:
`PotionReadings` carries only what the bus decoded, and the controller builds the
`PotionInputs` it evaluates, so there is exactly one source of truth for each gate
and the broken state cannot be constructed. That is the difference between a bug
that is tested against and one that is unrepresentable.

Worth noting for the record: this is precisely the failure the checklist item
describes, in the feature where it would have mattered most, and a test asserting
only "nothing was emitted" would have passed, because the disabled-by-default
controller blocks first for an unrelated reason.

The item most worth re-reading at review time is **CHK012**. Requiring each
blocking condition to be tested *in isolation with every other condition
satisfied* is what catches the failure this feature is most exposed to: a
condition that appears to block but is actually only blocking because a different
condition happened to be false in the test. That is not a hypothetical; it is how
a gate that checks the wrong variable passes its own test.
