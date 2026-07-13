# Bite Contract Requirements Checklist: Fishing Bite Signal Correction

**Purpose**: Validate requirements quality for the corrected bite contract,
regression bounds, and validation readiness before planning
**Created**: 2026-07-13
**Feature**: [spec.md](../spec.md)

## Bite Signal Correctness

- [x] CHK001 - Is the removal of the prompt-based trigger stated as an
  absolute prohibition (never a signal "in any form"), not a demotion?
  [Clarity, Spec FR-001]
- [x] CHK002 - Is the sole remaining bite signal fully specified with its
  scoping conditions (stack decrease, lure category, cast active, no menu)?
  [Completeness, Spec FR-002]
- [x] CHK003 - Is the standing reel-in prompt's true meaning documented as a
  requirement so the misreading cannot recur? [Coverage, Spec FR-007, Key
  Entities]
- [x] CHK004 - Is indefinite waiting (no timeout, no synthetic bite)
  explicitly required rather than implied? [Clarity, Spec FR-004]
- [x] CHK005 - Is the fast-bite race (bite lands before the waiting state is
  ever sampled) addressed? [Edge Case, Spec Edge Cases]
- [x] CHK006 - Is the benign failure mode for a never-firing bite event
  defined end to end (wait, escape, interaction end, recast, log evidence)?
  [Edge Case, Spec Edge Cases, Assumptions]
- [x] CHK007 - Are unrelated-consumable false bites covered with a
  measurable outcome? [Edge Case, Spec FR-002, SC-003]

## Regression Bounds

- [x] CHK008 - Is the frozen signal contract (colors, geometry, decoder,
  controller, timings) stated as unchanged and verifiable? [Consistency,
  Spec FR-005, SC-004]
- [x] CHK009 - Are the safety invariants restated as unchanged? [Coverage,
  Spec FR-008]
- [x] CHK010 - Is the rejected minimum-wait heuristic recorded with its
  rationale? [Assumption, plan-008 scope]

## Release and Validation Readiness

- [x] CHK011 - Is the addon delivery path (version 4 to 5, existing update
  control) a requirement? [Completeness, Spec FR-006]
- [x] CHK012 - Do the success criteria measure bait economy (one bait per
  catch, zero early-reel waste), the defect's actual cost? [Measurability,
  Spec SC-001, SC-002]
- [x] CHK013 - Are the live-session-only criteria identified as the owed
  in-game validation, targeting the one unverified link? [Dependency, Spec
  SC-001..SC-003, Assumptions]
- [x] CHK014 - Are the documentation corrections (spec contract, dated
  decision correcting slice 025, research citations) captured as
  requirements? [Completeness, Spec FR-007]

## Notes

- All items evaluated against spec.md on 2026-07-13 during the autopilot
  checklist pass; no requirement-quality gaps found.
