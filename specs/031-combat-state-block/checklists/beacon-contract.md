# Beacon Block Contract Checklist: PixelBeacon In-Combat State Block

**Purpose**: Validate the quality of the requirements governing the addon-to-companion block contract, since build plan 010 makes this slice the reference implementation that slices 032, 033, 034, and 035 inherit. A requirement that is vague here is a defect repeated four times.
**Created**: 2026-07-27
**Feature**: [spec.md](../spec.md)

**Note**: This checklist tests the requirements, not the implementation. Every item asks whether something is adequately specified, not whether it works.

## Color Contract Completeness

- [x] CHK001 Are requirements defined for how the combat square's validity mark must relate to every mark already on the strip? [Completeness, Spec §FR-006]
- [x] CHK002 Is the required separation between marks stated against a fixed, stated reference value rather than a value the operator can change at runtime? [Measurability, Spec §FR-006]
- [x] CHK003 Are requirements defined for the separation between the two encoded combat states themselves, not only between marks of different squares? [Gap, Spec §FR-006]
- [x] CHK004 Is it specified that the encoding must be identical on both sides of the contract, rather than merely compatible? [Clarity, Spec §FR-007]
- [x] CHK005 Are requirements defined for what the companion does with a color that carries a valid mark but an unrecognized state value? [Coverage, Spec §FR-007]
- [ ] CHK006 Is the full set of colors already in use on the strip enumerated somewhere a later slice can find it, so slices 032 to 035 can pick non-colliding marks without rediscovering them? [Gap]

## Backward and Forward Compatibility

- [x] CHK007 Are requirements defined for an addon that predates the combat square? [Completeness, Spec §FR-011, §US2]
- [x] CHK008 Are requirements defined for a companion that predates a square the addon draws? [Coverage, Spec §Edge Cases]
- [x] CHK009 Is it specified that an unreadable square must never be resolved as a real state rather than left to a default? [Clarity, Spec §FR-007, §FR-011]
- [x] CHK010 Are the unavailable state and the genuine out-of-combat state required to remain distinguishable to every consumer? [Consistency, Spec §Assumptions]
- [x] CHK011 Is the behavior specified for the case where the beacon is alive but the square stops decoding, distinctly from total signal loss? [Coverage, Spec §FR-008, §Clarifications]
- [x] CHK012 Is the divergence from the weapon-bar square's hold-on-non-decode behavior stated explicitly, with rationale, rather than left as an inconsistency a reader must notice? [Consistency, Spec §Clarifications]

## Geometry and Single Source of Truth

- [x] CHK013 Are requirements defined for where the number of squares on the strip is stated? [Completeness, Spec §FR-013]
- [x] CHK014 Is the single-source requirement stated in a form that is achievable across the two separate codebases that share the contract? [Conflict, Spec §FR-013, §SC-006]
- [x] CHK015 Is an automated means of detecting disagreement between the two sides required, rather than relying on a reviewer noticing? [Gap, Spec §FR-013]
- [x] CHK016 Is it specified that the combat square's position derives from the configured square size rather than assuming the default? [Clarity, Spec §FR-012]
- [x] CHK017 Are requirements defined for the strip's captured region growing with the square count? [Completeness, Spec §FR-013]
- [x] CHK018 Is "does not change the observing function's shape for each square added" stated as a verifiable property rather than a style preference? [Measurability, Spec §FR-014]

## State Semantics and Update Discipline

- [x] CHK019 Are requirements defined for what drives an update to the published state, and how promptly? [Completeness, Spec §FR-003]
- [x] CHK020 Are requirements defined for re-establishing state after a loading screen, when no transition notification fires? [Coverage, Spec §FR-004]
- [x] CHK021 Is it specified that a steady state must produce no repeated announcements or log entries? [Clarity, Spec §FR-005, §FR-008, §SC-002]
- [x] CHK022 Is it specified whether the square may express a state by being absent, and is the consequence for compatibility drawn out? [Ambiguity, Spec §FR-001, §Clarifications]
- [x] CHK023 Is the state the companion reports before it has ever decoded the square specified? [Gap, Spec §FR-002]

## Distribution and Rollout

- [x] CHK024 Are requirements defined for how an operator running the previous addon learns an update exists? [Completeness, Spec §FR-015]
- [x] CHK025 Is the manifest's operator-facing description required to stay accurate to the signals actually published? [Consistency, Spec §FR-015]

## Non-Regression and Safety Boundaries

- [x] CHK026 Are the behaviors this feature must not change stated explicitly rather than left implicit in the absence of requirements? [Completeness, Spec §FR-016, §FR-017]
- [x] CHK027 Are the safety-critical surfaces named individually, so a reviewer can confirm each is untouched without inferring the list? [Clarity, Spec §FR-017]
- [x] CHK028 Is it specified that the four existing signals must behave identically, with a stated means of confirming it? [Measurability, Spec §SC-005]
- [x] CHK029 Are requirements defined for the sampling cadence, so a reader knows whether this feature is permitted to change it? [Gap, Spec §FR-019]

## Traceability and Acceptance

- [x] CHK030 Can each success criterion be evaluated without knowing how the feature is built? [Measurability, Spec §SC-001 to §SC-007]
- [x] CHK031 Is the in-game portion of validation distinguished from what is verifiable at the desk? [Clarity, Spec §US1, §US2, §US3]
- [x] CHK032 Are the contracts that later slices inherit identified as such, so they are recorded as decisions rather than buried as implementation detail? [Traceability, Spec §FR-018]

## Notes

### Findings from the first run (2026-07-27)

Three items failed on the first pass. Two were real defects in the spec and were
fixed; one is left open by design.

- **CHK002 failed, now fixed.** FR-006 originally required the validity mark to
  be separated from every other mark "by more than the reader's per-channel
  color-match tolerance". That tolerance is an operator-editable setting with no
  range validation, so as written the requirement was unverifiable: at a high
  enough tolerance no separation satisfies it. FR-006 now states the separation
  against the default tolerance as the fixed reference and records that an
  operator who raises the tolerance far enough to collide the marks is a
  pre-existing condition of the whole strip, not something this feature
  introduces or is scoped to fix.
- **CHK003 failed, now fixed.** FR-006 constrained the mark against other
  squares' marks but said nothing about the two combat encodings being
  distinguishable from each other, which is the separation the decoder actually
  depends on to tell in combat from out of combat. Added to FR-006.
- **CHK014 and CHK015 failed, now fixed.** FR-013 required the square count to be
  "stated in exactly one place" and SC-006 required "editing exactly one
  location". Both are impossible as written: the addon is Lua and the companion
  is Rust, and only the square size is rewritten into the addon at deploy time,
  not the count. The requirement is now one authoritative statement per side plus
  a required automated check that the two agree, which is achievable, is the
  discipline the weapon-class codes already follow, and is testable at the desk
  because the addon source is embedded in the companion binary. Worth noting that
  the existing doc comment on the deploy-time rewrite already describes the addon
  as deriving its width from a `NUM_BLOCKS` the addon does not currently have, so
  the fix also makes that comment true.
- **CHK006 remains open by design.** No enumerated registry of the colors in use
  exists, and creating one is arguably this slice's job as the reference
  implementation. It is deliberately deferred to `plan.md`, where the marker
  value is chosen: a registry with one entry and no second consumer is
  speculative, whereas the plan can site it where slice 032 will actually reach
  for it. Carried forward as an open item rather than silently dropped.

### Second run (2026-07-27, after spec fixes)

CHK002, CHK003, CHK014, and CHK015 now pass. CHK006 remains open and is tracked
into the planning phase. 31 of 32 items passing.
