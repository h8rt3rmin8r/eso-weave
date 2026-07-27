# Input-Safety Checklist: PixelBeacon Menu-State Input Gate

**Purpose**: Validate the quality of the requirements governing the change to the interception decision, which constitution principle II names as a surface whose tests are never weakened, skipped, or made conditional. This slice is the first in the project to edit that decision since it was written. A requirement that is vague here is a safety defect, not a documentation defect.
**Created**: 2026-07-27
**Feature**: [spec.md](../spec.md)

**Note**: This checklist tests the requirements, not the implementation. Every item asks whether something is adequately specified.

## The One-Way Property (gate can only relax)

- [x] CHK001 Is the "can only relax, never tighten" property stated as a requirement rather than left as a consequence of a chosen implementation? [Completeness, Spec §FR-015]
- [x] CHK002 Is that property expressed over the full input space rather than over named examples, so it cannot be satisfied by spot checks? [Measurability, Spec §FR-015, §SC-005]
- [x] CHK003 Is the input space of the interception decision bounded and enumerable, so "every combination" is a claim that can actually be discharged? [Measurability, Spec §SC-005]
- [x] CHK004 Is the no-feature baseline the requirement compares against defined unambiguously? [Clarity, Spec §FR-015, §SC-006]
- [ ] CHK005 Does the spec distinguish the load-bearing safety property from incidental implementation ordering, so a correct implementation cannot be rejected for the wrong reason? [Ambiguity, Spec §FR-016]

## Focus Scoping (the constitutional invariant)

- [x] CHK006 Is it required that focus scoping remains unconditional, not merely that it remains present? [Clarity, Spec §FR-016]
- [x] CHK007 Is it stated that the gate composes with focus rather than substituting for it? [Consistency, Spec §FR-016]
- [x] CHK008 Are requirements defined for behavior when the game window is unfocused while the gate is active, so the interaction of the two is not left to inference? [Coverage, Spec §US3]
- [x] CHK009 Is it explicit that this feature cannot cause suppression outside the focused game window? [Completeness, Spec §FR-016]

## Synthesis Scope (what "must not synthesize" actually covers)

- [ ] CHK010 Is "must not synthesize input" scoped to identify every path that can synthesize, or does it read as universal while addressing only one? [Conflict, Spec §FR-009]
- [ ] CHK011 Are requirements defined for the fishing path, which synthesizes on its own schedule rather than in response to an intercepted key? [Gap, Spec §FR-009]
- [ ] CHK012 Do the no-synthesis requirement and the in-flight-completion requirement contradict each other as written? [Conflict, Spec §FR-009, §FR-011]
- [x] CHK013 Is the in-flight boundary stated with its rationale, so a reader understands why aborting was rejected rather than overlooked? [Clarity, Spec §FR-011]
- [x] CHK014 Are requirements defined for the state left behind after an in-flight sequence completes under the gate? [Coverage, Spec §US2]

## Fail-Safe Direction

- [x] CHK015 Is the default value of the gate specified, and is it the value that reproduces current behavior? [Completeness, Spec §FR-013]
- [x] CHK016 Are all three failure modes (absent block, undecodable sample, lost signal) required to resolve to the same safe value? [Coverage, Spec §FR-013]
- [x] CHK017 Is it stated that no failure mode can produce the more restrictive value? [Clarity, Spec §FR-013, §FR-015]
- [x] CHK018 Are requirements defined for an addon too old to publish the signal? [Coverage, Spec §US3, §SC-006]

## Composition With Existing Behavior

- [x] CHK019 Are requirements defined for how the gate interacts with the existing manual suspend? [Completeness, Spec §Edge Cases]
- [x] CHK020 Is the treatment of the exempt toggle hotkeys specified, and is it consistent with how the manual suspend treats them? [Consistency, Spec §FR-010]
- [x] CHK021 Is the rationale for reusing the suspend semantics recorded, rather than introducing a second concept without justification? [Clarity, Spec §Clarifications]

## Non-Functional Constraints on the Decision

- [x] CHK022 Is the requirement that the decision stays synchronous and non-blocking carried forward explicitly for the added guard? [Completeness, Spec §FR-017]
- [x] CHK023 Is "no added timed work" stated in a form a reviewer can check against the added code? [Measurability, Spec §FR-017]

## Test-Surface Protection

- [x] CHK024 Is it required that the existing safety tests are extended rather than replaced? [Completeness, Spec §FR-018]
- [ ] CHK025 Does the requirement distinguish weakening a test from mechanically updating it, so an unavoidable signature change does not read as a violation? [Ambiguity, Spec §FR-018]
- [x] CHK026 Is there a requirement that the new safety property itself is covered by a test, not only that old tests survive? [Coverage, Spec §SC-005]

## Latency and Promptness

- [x] CHK027 Is the sampling latency stated as a limitation rather than implied to be zero? [Clarity, Spec §Overview, §SC-002]
- [ ] CHK028 Are requirements defined for how promptly the addon must publish a change, as distinct from how often the companion samples it? [Gap, Spec §FR-019, §SC-002]
- [x] CHK029 Is the cadence change justified against the decision it reverses from the previous slice? [Traceability, Spec §Clarifications]
- [x] CHK030 Is the cost of the cadence change acknowledged? [Completeness, Spec §Clarifications]

## Signal Correctness

- [x] CHK031 Is the choice of authoritative signal justified against the alternative the issue proposed, rather than asserted? [Traceability, Spec §Clarifications, §FR-002]
- [x] CHK032 Is the requirement that the gate stays correct for an unenumerated surface stated independently of the surface code table? [Clarity, Spec §FR-003]
- [x] CHK033 Is the bias direction (false positive versus false negative) stated with its justification? [Completeness, Spec §Assumptions]

## Notes

### Findings from the first run (2026-07-27)

29 of 33 passed. Four failed, and one of them is a genuine safety defect rather
than a wording problem.

**CHK010, CHK011, CHK012 failed together, and this is the important one.**
FR-009 said "while the gate is active, the application MUST NOT synthesize
input". That is false as written in two different ways:

1. It contradicts FR-011, which requires an in-flight sequence to run to
   completion. A running sequence *is* synthesis. As written the spec both
   forbade and required the same behavior.
2. More seriously, it reads as universal but the design behind it (a guard in
   the interception decision) only covers synthesis that *starts from an
   intercepted key*. The fishing controller synthesizes on its own schedule,
   driven by beacon events rather than by a key press, through its own sink. A
   gate placed only in the interception decision would not stop a reel keypress
   from landing in a chat message the operator is typing, which is precisely the
   harm this feature exists to prevent. Fishing is also a likely context for it,
   since waiting for a bite is exactly when someone opens chat.

   Fixed by splitting FR-009 into FR-009 (no new weave sequence) and FR-009a (no
   new fishing interaction), both scoped to *starting* rather than to all
   synthesis, with FR-011 as the stated carve-out for work already in motion.
   This widens the slice's surface, and that is the correct outcome: the
   alternative was shipping a requirement that the implementation could not
   honor.

**CHK028 failed.** SC-002 promised the gate engages within one sampling
interval, but nothing required the addon to *publish* the change promptly. The
addon's general-purpose tick runs at one second; if the gate rode that, real
latency would be about eleven times the promise, and SC-002 would be unmeetable
while every stated requirement was satisfied. Added FR-019a requiring the addon
to publish gate changes at its fast cadence, so the end-to-end latency claim is
actually derivable from the requirements.

**CHK005 and CHK025 failed as wording defects, both fixed.** FR-016 stated
"evaluated before the gate" as a MUST, conflating the load-bearing property
(focus alone decides for an unfocused window, whatever the gate says) with an
incidental ordering that does not affect the outcome, since both conditions
produce the same result. FR-018 forbade "modifying" existing safety tests, which
a mechanical signature update would violate; the real requirement is that no
scenario or assertion is weakened. The previous slice hit exactly this when a
version-pin test had to advance.

### Second run (2026-07-27, after spec fixes)

All 33 items pass. Item count grew by one requirement split (FR-009a) and one
addition (FR-019a).
