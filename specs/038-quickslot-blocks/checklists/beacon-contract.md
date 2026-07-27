# Beacon Block Contract Checklist: PixelBeacon Quickslot-State Blocks

**Purpose**: Validate the quality of the requirements governing this slice, which is the first in the project's life to ship a block count that wraps the grid onto a second row, the first to publish a multi-block composite value rather than a run of independent ones, and the slice that inherits a question the previous one deliberately handed forward. Each of those is a place a vague requirement would survive review.
**Created**: 2026-07-27
**Feature**: [spec.md](../spec.md)

**Note**: This checklist tests the requirements, not the implementation. Every item asks whether something is adequately specified, not whether it works.

## The Row Crossing

- [x] CHK001 Is the grid's end state specified as a shape (rows and fill) rather than only as a count? [Clarity, Spec §FR-013]
- [x] CHK002 Is it specified that every pre-existing square keeps its exact position, rather than merely that existing signals keep working? [Clarity, Spec §FR-014]
- [x] CHK003 Are requirements defined for where the four new squares land, not just that they exist? [Completeness, Spec §US3, §FR-013]
- [x] CHK004 Is the captured region's new height required to be asserted rather than assumed to follow from the count? [Measurability, Spec §FR-013, §SC-006]
- [x] CHK005 Is the disposition of the compile-time assertion specified, and is replacing it distinguished from relaxing, widening, and removing it? [Clarity, Spec §FR-015]
- [x] CHK006 Is the replacement assertion required to be as specific as the one it succeeds, rather than merely permissive of the new count? [Clarity, Spec §FR-015]
- [x] CHK007 Is there a requirement covering expectations elsewhere that were written when one row was the only possibility, as a class rather than one by one? [Coverage, Spec §FR-016]
- [x] CHK008 Is it specified that those expectations must be updated deliberately rather than left passing by coincidence? [Clarity, Spec §FR-016]
- [x] CHK009 Is the overflow report required to detect a vertical overflow specifically, and is it noted that this becomes reachable for the first time? [Coverage, Spec §FR-017, §Edge Cases]
- [x] CHK010 Is it specified how much multi-row behaviour already exists versus must be built, so the slice does not rebuild working geometry? [Ambiguity, Spec §Assumptions, §Dependencies]
- [x] CHK011 Is it required that the new squares' positions derive from the same rule as every other square, with no special case for the second row? [Consistency, Spec §FR-021]

## The Overlay Footprint

- [x] CHK012 Is the question handed forward by the previous slice (what the operator sees when the overlay doubles in height) actually answered rather than deferred again? [Gap, Spec §Clarifications]
- [x] CHK013 Is the answer stated as a requirement on the application, not only as a narrative observation? [Measurability, Spec §FR-018]
- [x] CHK014 Is the unit of the reported extent specified, so "extent" is not ambiguous between squares and pixels? [Clarity, Spec §FR-018]
- [x] CHK015 Is the location of the report specified, and is the reason for each location given rather than assumed? [Completeness, Spec §FR-018, §Clarifications]
- [x] CHK016 Is the remedy available to an operator who dislikes the footprint named, rather than left implicit? [Completeness, Spec §FR-019]
- [x] CHK017 Are the things this slice must NOT do about the footprint (move the anchor, auto-shrink) stated as exclusions rather than merely omitted? [Coverage, Spec §FR-019, §Clarifications]
- [x] CHK018 Is the reason for excluding the anchor move given, so a later slice inherits the reasoning rather than only the exclusion? [Clarity, Spec §Clarifications]

## Multi-Block Contract Completeness

- [x] CHK019 Are requirements defined for how each new mark relates to every mark already on the grid? [Completeness, Spec §FR-006]
- [x] CHK020 Are requirements defined for the four new marks relating to each other, not only to the incumbents? [Coverage, Spec §FR-006]
- [x] CHK021 Is it required that a value valid for one new square must not decode at another new square's position? [Coverage, Spec §SC-007]
- [x] CHK022 Is the required separation stated against a fixed reference rather than a runtime-adjustable one? [Measurability, Spec §FR-006]
- [x] CHK023 Is registration in the shared mark registry required, so separation is proven automatically rather than by the author? [Gap, Spec §FR-006]
- [x] CHK024 Is an integrity check beyond the mark required, so unrelated content matching a mark is still rejected? [Coverage, Spec §FR-006, §FR-007]
- [x] CHK025 Is the cross-language agreement requirement extended to the new count, marks, and encoding constants, rather than assumed to cover them? [Traceability, Spec §FR-022]

## The Composite Payload

- [x] CHK026 Is it specified that the cooldown reuses the existing cooldown value rather than defining a parallel one? [Consistency, Spec §FR-001, §FR-007]
- [x] CHK027 Are the three cases distinguished, and is unknown defined as covering both a reading failure and a real absence? [Clarity, Spec §FR-002, §Key Entities]
- [x] CHK028 Is the collapse of "empty", "not a potion", "no readable cooldown", and "unreadable square" into one outcome stated explicitly and justified? [Ambiguity, Spec §FR-002, §Clarifications]
- [x] CHK029 Is the decision NOT to spend a block on an is-a-potion flag justified rather than asserted? [Clarity, Spec §Clarifications]
- [x] CHK030 Is the identity's width and byte order specified, and is the byte order justified rather than picked? [Clarity, Spec §FR-003, §Clarifications]
- [x] CHK031 Is partial-decode behaviour for a multi-block value specified, so an identity is never assembled from the bytes that happened to read? [Coverage, Spec §FR-008]
- [x] CHK032 Is it specified what the identity squares carry when there is nothing to identify, and that they keep being drawn in that state? [Completeness, Spec §FR-003]
- [x] CHK033 Is out-of-range identity behaviour defined on the publishing side, rather than left to produce an unencodable byte? [Coverage, Spec §FR-003] - closed by amendment, see Findings.
- [x] CHK034 Is it specified that a state claiming a potion with no cooldown, or a cooldown with no potion, must be unrepresentable rather than merely unused? [Measurability, Spec §FR-007]
- [x] CHK035 Is the decision to carry an identity rather than the potion's restore types justified against what the game actually exposes? [Clarity, Spec §Clarifications]
- [x] CHK036 Is the resolution and range of the cooldown inherited by reference rather than restated, so the two cannot drift? [Consistency, Spec §FR-001]

## Announcement, Log, and Interface

- [x] CHK037 Is it specified whether the four values travel as one announcement or four? [Completeness, Spec §FR-010]
- [x] CHK038 Is that choice justified against the existing precedent rather than invented? [Consistency, Spec §Clarifications]
- [x] CHK039 Is it specified that a steady state produces no announcements and no entries? [Clarity, Spec §FR-005, §FR-009, §SC-002]
- [x] CHK040 Is the placement of the readout specified relative to what the interface already shows, and is the rejected alternative explained? [Clarity, Spec §FR-012, §Clarifications]
- [x] CHK041 Is the presentation of the identity specified, given the companion cannot resolve it to a name? [Gap, Spec §FR-012, §Clarifications]
- [x] CHK042 Is the partial state (cooldown decoded, identity not) defined for the interface, rather than only for the decoder? [Coverage, Spec §FR-012] - closed by amendment, see Findings.

## Backward Compatibility

- [x] CHK043 Are requirements defined for an addon that predates all four squares? [Completeness, Spec §FR-020, §US2]
- [x] CHK044 Is it noted that the absent squares are now sampled from a screen region the beacon has never drawn on, rather than from elsewhere on a row it already occupies? [Completeness, Spec §US2]
- [x] CHK045 Is it specified that an unreadable square never resolves to a real value? [Clarity, Spec §FR-007]
- [x] CHK046 Is the behaviour specified for a beacon that is alive while a square stops decoding, distinctly from total signal loss? [Coverage, Spec §FR-009]
- [x] CHK047 Is the consequence of a false reading here stated in terms of the consumer one slice away, so the stakes are not understated? [Completeness, Spec §US2]

## Boundaries and Safety

- [x] CHK048 Is it specified that nothing may act on the values in this slice? [Clarity, Spec §FR-024]
- [x] CHK049 Are the safety invariants that must remain untouched enumerated rather than covered by a general assurance? [Completeness, Spec §FR-025]
- [x] CHK050 Is it specified that the sampling cadence must not change? [Clarity, Spec §FR-027]
- [x] CHK051 Are requirements defined for advancing the manifest and naming the new signal? [Completeness, Spec §FR-023]
- [x] CHK052 Is the documentation obligation split between the architecture of record and the operator-facing text, rather than treated as one? [Completeness, Spec §FR-019, §FR-026]

## Notes

- Check items off as completed: `[x]`
- Unchecked items are genuine gaps for `/speckit-plan` to close, not oversights to ignore.

### Findings and disposition

Two items failed the first pass. Both were requirement gaps rather than
implementation questions, so both were closed by amending the spec before
planning.

- **CHK033** asked what the publishing side does with an identity wider than the
  encodable width. The spec's Assumptions acknowledged that an oversized identity
  would alias to a different one, and treated that as an accepted risk, but no
  requirement said the reduction had to happen at all. Without one, the natural
  implementation lets an out-of-range value produce a byte that fails its own
  integrity check, and a failed integrity check means unknown, which the consumer
  reads as "there is no potion here". That converts a benign inability to name
  the item into a false statement about whether there is one. FR-003 now requires
  a deterministic reduction on the publishing side and forbids the unencodable
  byte.
- **CHK042** asked what the interface shows when the cooldown decodes and the
  identity does not, a state that is reachable whenever exactly one of the three
  identity squares is disturbed. FR-008 defined it for the decoder and nothing
  defined it for the readout, which left "collapse the whole readout to unknown"
  as a plausible reading. That would discard a value that was read correctly and
  would make a one-square disturbance indistinguishable from a missing addon,
  which is precisely the confusion the grid-fit report exists to prevent
  elsewhere. FR-012 now requires the two halves to degrade independently.

Also worth recording: **CHK012** is the question the previous slice's CHK008
explicitly handed forward. It is answered here rather than dissolved again.

The pattern from the last two slices held once more. The spec step produced a
document that read as complete; the checklist step found two places where a
requirement's absence had a specific and non-obvious failure attached to it, and
both failures were of the same shape, an unreadable part being reported as an
absent whole.

Status: 52 of 52 items pass.
