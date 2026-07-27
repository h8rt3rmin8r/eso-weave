# Beacon Block Contract Checklist: PixelBeacon Skill-Cooldown Blocks

**Purpose**: Validate the quality of the requirements governing this slice, which is unlike every block slice before it in three ways: it adds seven blocks at once rather than one to three, it is the first to cross the grid's column boundary onto a second row, and it deliberately trips a compile-time assertion a previous slice left behind. Each of those is a place a vague requirement would survive review.
**Created**: 2026-07-27
**Feature**: [spec.md](../spec.md)

**Note**: This checklist tests the requirements, not the implementation. Every item asks whether something is adequately specified, not whether it works.

## The Row Boundary

- [x] CHK001 Is it specified that the grid grows downward rather than past the column count? [Completeness, Spec §FR-012]
- [x] CHK002 Is it specified that every pre-existing square keeps its exact position, rather than merely that existing signals keep working? [Clarity, Spec §FR-013]
- [x] CHK003 Are requirements defined for the captured region changing shape, not just size? [Completeness, Spec §FR-012, §SC-006]
- [x] CHK004 Is the overflow report required to detect a vertical overflow specifically, rather than assumed to generalize from the horizontal case? [Coverage, Spec §FR-014]
- [x] CHK005 Is it specified what happens to the compile-time assertion that this slice trips, and is replacing it distinguished from removing it? [Clarity, Spec §FR-015]
- [x] CHK006 Is the reason the assertion must survive in some form stated, rather than treated as a one-time obstacle? [Completeness, Spec §FR-015]
- [x] CHK007 Is it specified how much of the row-two behaviour already exists versus must be built, so the slice does not rebuild working geometry? [Ambiguity, Spec §Clarifications, §Assumptions]
- [x] CHK008 Are requirements defined for what the operator sees when the overlay changes size on screen, as distinct from what the application computes? [Gap, Spec §FR-012] — dissolved rather than answered: the corrected six-block scope keeps the grid one row tall, so the overlay grows sideways to a bound it was always designed for and gains no height. It returns when slice 038 crosses.

## Multi-Block Contract Completeness

- [x] CHK009 Are requirements defined for how each new mark relates to every mark already on the grid? [Completeness, Spec §FR-006]
- [x] CHK010 Are requirements defined for the seven new marks relating to *each other*, not only to the incumbents? [Coverage, Spec §FR-006]
- [x] CHK011 Is the choice of seven distinct marks over one shared mark justified rather than asserted? [Clarity, Spec §Clarifications]
- [x] CHK012 Is it required that a value valid for one slot must not decode at another slot's position? [Coverage, Spec §SC-007]
- [x] CHK013 Is the required separation stated against a fixed reference rather than a runtime-adjustable one? [Measurability, Spec §FR-006]
- [x] CHK014 Is registration in the shared mark registry required, so separation is proven automatically rather than by the author? [Gap, Spec §FR-006]
- [x] CHK015 Is an integrity check beyond the mark required, so unrelated content matching a mark is still rejected? [Coverage, Spec §FR-007]

## The Numeric Payload

- [x] CHK016 Is the resolution specified, and justified against the sampling interval rather than chosen arbitrarily? [Measurability, Spec §Clarifications]
- [x] CHK017 Is the range specified, and is out-of-range behaviour defined rather than left to overflow? [Coverage, Spec §FR-003, §Edge Cases]
- [x] CHK018 Are the three cases (a duration, ready, unavailable) distinguished, and is unavailable defined as a reading failure rather than a duration? [Clarity, Spec §FR-002, §Key Entities]
- [x] CHK019 Is it specified that an empty slot and an unreadable square produce the same value, and is that collapse justified? [Ambiguity, Spec §Edge Cases]
- [x] CHK020 Is the mapping from the application's slots to the game's own slot numbering specified, or deferred with a stated reason? [Gap, Spec §Clarifications] — closed by verification: the game's action bar iterates from its first-normal-slot index through its ultimate-slot index, and Synergy is outside that range because it is not an action slot, which is what reduced the scope from seven blocks to six.

## Announcement and Log Volume

- [x] CHK021 Is it specified whether the seven values travel as one announcement or seven? [Completeness, Spec §FR-010]
- [x] CHK022 Is that choice justified against the existing precedent rather than invented? [Consistency, Spec §Clarifications]
- [x] CHK023 Are requirements defined for log volume, given seven values that change continuously in combat? [Coverage, Spec §FR-009]
- [x] CHK024 Is the log entry's content specified, so it is neither a flood nor useless? [Clarity, Spec §Clarifications]
- [x] CHK025 Is it specified that a steady state produces no announcements and no entries? [Clarity, Spec §FR-005, §SC-002]

## Backward Compatibility

- [x] CHK026 Are requirements defined for an addon that predates all seven squares? [Completeness, Spec §FR-016, §US2]
- [x] CHK027 Is it specified that an unreadable square never resolves to a real duration? [Clarity, Spec §FR-007]
- [x] CHK028 Is the behaviour specified for a beacon that is alive while a square stops decoding, distinctly from total signal loss? [Coverage, Spec §FR-008]
- [x] CHK029 Is it noted that this slice samples screen regions never previously read, raising the stakes of the false-reading case? [Completeness, Spec §US2]

## Interface Impact

- [x] CHK030 Is the placement of the readout specified relative to what the interface already shows? [Clarity, Spec §FR-011]
- [x] CHK031 Are the knock-on effects on the interface's pinned column count and computed window width called out, rather than left to be discovered at implementation? [Gap, Spec §FR-011, §Clarifications]
- [x] CHK032 Is extending the window-sizing tests required rather than optional? [Coverage, Spec §FR-011]

## Boundaries and Safety

- [x] CHK033 Is it specified that nothing may act on the values in this slice? [Clarity, Spec §FR-020]
- [x] CHK034 Are the safety invariants that must remain untouched enumerated rather than covered by a general assurance? [Completeness, Spec §FR-021]
- [x] CHK035 Is it specified that the sampling cadence must not change? [Clarity, Spec §FR-023]
- [x] CHK036 Are requirements defined for advancing the manifest and naming the new signal? [Completeness, Spec §FR-019]
- [x] CHK037 Is authoring the build plan that sequences this slice and its successors required as part of this feature? [Completeness, Spec §FR-022]

## Notes

- Check items off as completed: `[x]`
- Unchecked items are genuine gaps for `/speckit-plan` to close, not oversights to ignore.

### Findings and disposition

Both items that failed the first pass are now closed, and closing one of them
changed the feature.

- **CHK020** asked whether the application-slot to game-slot mapping was
  specified or deferred with a reason. Resolving it turned up the fact that
  reshaped this slice: the game's own action bar iterates from its first-normal
  slot index through its ultimate slot index, and Synergy is not in that range,
  because Synergy is a contextual prompt rather than an action slot. It has no
  cooldown to read in any state. The feature dropped from seven blocks to six.
- **CHK008** asked what the operator sees when the overlay gains a row. The
  six-block scope dissolved the question rather than answering it: sixteen blocks
  fill exactly one row, so the overlay gains width up to a bound the design
  always intended and gains no height at all. The question returns, legitimately,
  when slice 038 adds the seventeenth block.

Worth recording for the slices that follow: the checklist earned its place here
by asking for a mapping that nobody had actually looked up. The answer changed
the block count, the grid geometry, the headline risk of the slice, and which
slice owns the row crossing. That is the second consecutive slice in which the
checklist step, not the spec step, found the thing that mattered.

Status: 37 of 37 items pass.
