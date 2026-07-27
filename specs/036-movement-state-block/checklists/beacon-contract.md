# Beacon Block Contract Checklist: PixelBeacon Movement-State Block

**Purpose**: Validate the quality of the requirements governing the addon-to-companion block contract for the tenth and final block of build plan 010. This slice is unusual in the family: it ships a reduced scope against its issue, it reserves an encoding position for work that does not exist yet, and it is the first block added since the grid wrap changed how position is computed. Each of those is a place a vague requirement would survive review.
**Created**: 2026-07-27
**Feature**: [spec.md](../spec.md)

**Note**: This checklist tests the requirements, not the implementation. Every item asks whether something is adequately specified, not whether it works.

## Color Contract Completeness

- [x] CHK001 Are requirements defined for how the movement square's validity mark must relate to every mark already on the grid? [Completeness, Spec §FR-006]
- [x] CHK002 Is the required separation stated against a fixed, stated reference value rather than one the operator can change at runtime? [Measurability, Spec §FR-006]
- [x] CHK003 Are requirements defined for the separation between the encoded movement states themselves, not only between marks of different squares? [Coverage, Spec §FR-006]
- [x] CHK004 Is it specified that the encoding must be identical on both sides of the contract rather than merely compatible? [Clarity, Spec §FR-016]
- [x] CHK005 Are requirements defined for what the companion does with a color carrying a valid mark but an unrecognized state value? [Coverage, Spec §FR-007]
- [x] CHK006 Is the set of marks already in use discoverable, so this slice's mark can be chosen against it rather than by rediscovery? [Gap, Spec §FR-006] — the shared registry introduced after slice 031 closes the gap that slice left open.
- [x] CHK007 Is it required that the new mark be added to that shared registry, so its separation is proven automatically rather than asserted by the author? [Gap, Spec §FR-006]
- [x] CHK008 Are requirements defined for an integrity check beyond the mark, so unrelated screen content that happens to match the mark is still rejected? [Coverage, Spec §FR-007]
- [x] CHK009 Is the behavior specified for an operator who raises the color-match tolerance far enough to collide two encodings? [Consistency, Spec §FR-006]

## The Reduced Scope and the Reserved Axis

- [x] CHK010 Is the reason the sprint axis is excluded documented with its evidence, rather than asserted? [Completeness, Spec §The sprint verification]
- [x] CHK011 Is the excluded axis's evidence traceable to named, checkable sources rather than to a claim of having looked? [Traceability, Spec §The sprint verification]
- [x] CHK012 Are requirements defined for reserving an encoding position for the deferred axis? [Completeness, Spec §FR-011]
- [x] CHK013 Is it specified that the reserved position must never be emitted, and must decode as unavailable rather than as a state? [Clarity, Spec §FR-012]
- [x] CHK014 Is it specified on which side of the contract the reservation is expressed, so the cross-language check does not need a special case? [Consistency, Spec §FR-012, §Clarifications]
- [x] CHK015 Is the naming contract specified, so adding the deferred axis later does not force a rename of the square, the value, the event, and the label? [Clarity, Spec §FR-011]
- [x] CHK016 Are requirements defined for tracking the deferral so it is not silently lost? [Gap, Spec §FR-013]
- [x] CHK017 Is the condition under which the deferred axis becomes buildable stated, rather than left to a future reader to re-derive? [Completeness, Spec §Assumptions]

## Backward and Forward Compatibility

- [x] CHK018 Are requirements defined for an addon that predates the movement square? [Completeness, Spec §FR-014, §US2]
- [x] CHK019 Are requirements defined for a companion that predates a square the addon draws? [Coverage, Spec §Edge Cases]
- [x] CHK020 Is it specified that an unreadable square must never resolve to a real state rather than to a default? [Clarity, Spec §FR-007, §FR-014]
- [x] CHK021 Are the unavailable state and the genuine on-foot state required to remain distinguishable? [Consistency, Spec §Key Entities]
- [x] CHK022 Is the behavior specified for a beacon that is alive while the square stops decoding, distinctly from total signal loss? [Coverage, Spec §FR-008]
- [x] CHK023 Is the inherited clear-on-non-decode decision identified as inherited, rather than restated as if it were novel? [Consistency, Spec §Clarifications]

## Geometry Under the Wrapped Grid

- [x] CHK024 Is it specified that the square's position derives from both the configured square size and the shared column count, rather than from either alone? [Clarity, Spec §FR-015]
- [x] CHK025 Is the claim that ten blocks leave the captured region unchanged required to be asserted rather than assumed? [Measurability, Spec §FR-017]
- [x] CHK026 Are requirements defined for keeping the block count stated exactly once per side with dependent geometry derived from it? [Completeness, Spec §FR-016]
- [x] CHK027 Is an automated means of detecting disagreement between the two sides required, rather than relying on a reviewer noticing? [Gap, Spec §FR-016]
- [x] CHK028 Are requirements defined for what happens at the column boundary, where the next block added would start a second row and the extent claim in FR-017 stops holding? [Gap, Spec §FR-017]
- [x] CHK029 Is the existing out-of-bounds extent warning required to remain unregressed? [Coverage, Spec §Edge Cases]

## State Semantics and Update Discipline

- [x] CHK030 Are requirements defined for what drives an update to the published state, and how promptly? [Completeness, Spec §FR-003]
- [x] CHK031 Are requirements defined for re-establishing state after a loading screen, when no transition notification fires? [Coverage, Spec §FR-004]
- [x] CHK032 Is it specified that a steady state must produce no repeated announcements or log entries? [Clarity, Spec §FR-005, §FR-008, §SC-002]
- [x] CHK033 Is it specified that the square must not be hidden to express a state, so absence has exactly one meaning? [Clarity, Spec §FR-001, §Clarifications]
- [x] CHK034 Are requirements defined for state transitions that occur entirely between two samples? [Edge Case, Spec §Edge Cases]
- [x] CHK035 Is independence from the other player-state squares specified, so no combination of other states can alter this one? [Consistency, Spec §Edge Cases]

## Observability and Operator Surface

- [x] CHK036 Is the log level for a movement change specified, with a stated reason rather than by convention alone? [Clarity, Spec §FR-009]
- [x] CHK037 Is the operator-facing treatment of the unavailable case specified and consistent with the sibling readouts? [Consistency, Spec §FR-010]
- [x] CHK038 Is the operator-facing wording for the reachable states themselves specified, or only for the unavailable case? [Gap, Spec §FR-010, Plan §D4]
- [x] CHK039 Are requirements defined for where the value appears relative to the existing decoded player-state readouts? [Completeness, Spec §FR-010]

## Boundaries and Safety

- [x] CHK040 Is it specified that nothing may act on the new value in this slice? [Clarity, Spec §FR-019]
- [x] CHK041 Are the safety invariants that must remain untouched enumerated, rather than covered by a general assurance? [Completeness, Spec §FR-020]
- [x] CHK042 Is it specified that the sampling cadence must not change? [Clarity, Spec §FR-022]
- [x] CHK043 Are requirements defined for advancing the manifest version and naming the new signal in its description? [Completeness, Spec §FR-018]
- [x] CHK044 Are requirements defined for updating the architecture of record and recording the dated decisions this slice sets? [Completeness, Spec §FR-021]

## Notes

- Check items off as completed: `[x]`
- Unchecked items are genuine gaps for `/speckit-plan` to close, not oversights to ignore.

### Findings and disposition

Four items failed on the first pass. Three were spec defects and were fixed
before planning; one is left for the plan.

- **CHK007** and **CHK009**, both on the color contract, were fixed in FR-006.
  CHK009 was a regression against slice 031, which stated the raised-tolerance
  caveat explicitly while this spec had dropped it.
- **CHK028** was the most valuable finding. FR-017 originally asserted the
  captured region is unchanged "at ten blocks", which is true but for the wrong
  reason: it holds because ten is under the sixteen-column wrap, and blocks
  eleven through sixteen would keep it true by luck while the seventeenth breaks
  it. FR-017 now states the dependency (block count not exceeding the column
  count) rather than the number, so the slice that eventually crosses that
  boundary inherits a requirement that anticipates it.
- **CHK038** was deferred to the plan by design, not left open by oversight.
  FR-010 fixed the unavailable case's treatment and the readout's placement but
  not the wording for the reachable states, because that wording belongs with the
  sibling readouts it must match. Plan decision D4 closed it by reading
  `combat_view` directly: "Mounted", "On foot", "Not detected", with the active
  and muted roles mapped exactly as the combat readout maps them.

Status: 44 of 44 items pass.
