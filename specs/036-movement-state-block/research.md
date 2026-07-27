# Phase 0 Research: PixelBeacon Movement-State Block

**Feature**: [spec.md](spec.md) | **Date**: 2026-07-27

All decisions below were made under the build-phase autopilot decision policy.
None were escalated. No NEEDS CLARIFICATION markers remain at the end of this
phase.

## R0: Does the game expose a sprint observable?

**Decision**: No. Ship the mounted axis alone.

This was issue #11's blocking entry condition and it is resolved in the spec's
"The sprint verification" section with its evidence. It is restated here only as
the input to R2: two of the four code-table values are reserved rather than live
because of this finding.

**Rationale**: two independent sources agree that no sprint function, event, or
constant exists in the addon-visible API, and the only four `Sprint` references
in the entire interface source are a keybind action, a user preference, and a
`sprintf` false positive. The same evidence shows sprint is toggled on gamepad
and held on keyboard, so a reconstruction would be both indirect and
input-dependent.

**Alternatives considered**: reconstructing sprint from stamina drain, or from
combat and effect events on the Sprint ability. Both rejected. Issue #11 itself
directs against encoding a flaky signal into a contract shared byte for byte
between two codebases, and a heuristic that must model three input semantics is
the definition of flaky. Deferring costs nothing because R2's reservation makes
adding it later cheap.

## R1: Which green channel value identifies the movement block?

**Decision**: `0x43`.

**Rationale**: the ten greens already at block centers are `0x00`, `0x16`,
`0x2D`, `0x5A`, `0x6D`, `0x80`, `0xA5`, `0xBB`, `0xD2`, `0xFF`. Their interior
gaps are:

| Gap | Width | Midpoint |
| --- | --- | --- |
| `0x00` to `0x16` | 22 | `0x0B` |
| `0x16` to `0x2D` | 23 | `0x21` |
| `0x2D` to `0x5A` | 45 | `0x43` |
| `0x5A` to `0x6D` | 19 | `0x63` |
| `0x6D` to `0x80` | 19 | `0x76` |
| `0x80` to `0xA5` | 37 | `0x92` |
| `0xA5` to `0xBB` | 22 | `0xB0` |
| `0xBB` to `0xD2` | 23 | `0xC6` |
| `0xD2` to `0xFF` | 45 | `0xE8` |

The two widest gaps are tied at 45, so their midpoints `0x43` and `0xE8` are tied
at a nearest-neighbour distance of 22, which is eleven times the default
tolerance of 2 and equal to the tightest separation already shipping (the
resource markers). Every other candidate is worse.

The tiebreak is distance from the channel extremes. Unrelated screen content
behind the overlay is far more likely to be near black or near white than
mid-range, and `0x43` sits 67 from `0x00` and 188 from `0xFF` while `0xE8` sits
23 from `0xFF`. The `red + blue` checksum is the primary defense against a false
positive, so this only moves an already small probability, but it moves it in the
right direction for free.

**Alternatives considered**:

- `0xE8`: tied on separation, rejected on the extremes tiebreak above.
- `0x34`, the nibble swap of `0x43`, to continue the `0xA5`/`0x5A` and
  `0x2D`/`0xD2` pairing: rejected outright. It sits 7 from `0x2D`, which is far
  too close. The convention was a mnemonic for picking a second value near a
  first, and the resource markers already abandoned it; treating it as binding
  would trade a real safety margin for a naming pattern.
- Reusing an existing mark and distinguishing blocks by position alone: rejected.
  The mark is what makes a block's absence detectable, which User Story 2 depends
  on, and it is what stops a geometry error off by one block from decoding a
  neighbour's color as this block's state.

## R2: How is the movement state encoded, and how is the deferred axis reserved?

**Decision**: red carries a two-bit code, blue carries `255 - red` as a checksum.
Codes `0b00` (`0x20`, on foot) and `0b01` (`0x60`, mounted) are live; `0b10`
(`0xA0`) and `0b11` (`0xE0`) are reserved for the sprint axis and never emitted.

**Rationale**: this is exactly issue #11's proposal, with two of its four values
dormant. Bit 0 is mounted and bit 1 is sprint, so when sprint arrives the live
values keep their meanings and their colors, and the new axis is expressed by
codes that already exist in the table. Even spacing of 64 puts every pair 32
tolerances apart. Following combat's marker-and-checksum validation rather than
the weapon block's exact-match keeps the family consistent and gives a second
independent gate against unrelated screen content.

**Alternatives considered**:

- A bare boolean now, with sprint added later as a second block: rejected. It
  would cost a further block, a further mark, and a further sample point, which
  is the outcome issue #11 explicitly reasoned against, and it would make the
  eleventh block arrive sooner than the grid needs it.
- Live codes packed at `0x20` and `0xE0` (combat's own two values) with sprint
  squeezed in later: rejected. It maximizes separation today at the cost of
  renumbering, and therefore of changing shipped colors, when sprint lands.
  Reserving space now is free; reclaiming it later is a contract change.
- Encoding the two axes as two independent bits in two channels: rejected. Blue
  is already spoken for by the checksum, and dropping the checksum to free a
  channel would remove the second validation gate.

## R3: Where does the reserved code live?

**Decision**: documented and rejection-tested in the companion; not defined in
the addon.

**Rationale**: the cross-language check in `tests/beacon.rs` proves the two sides
agree by parsing the addon source for each constant the companion declares. A
constant defined in the addon but never emitted has no companion counterpart to
agree with, so the check would need a special case for values that exist on one
side only, weakening the very mechanism that makes the contract trustworthy. The
reservation's purpose is to stop a future feature from choosing a colliding code,
and a documented, tested rejection on the reading side achieves that completely.

**Alternatives considered**: defining the reserved constants in both sources for
symmetry. Rejected as above, and it would additionally put an unreachable branch
in the addon where dead code is hardest to notice.

## R4: How is the capture-extent invariant stated?

**Decision**: assert that the region is one row whenever the block count does not
exceed the column count, then assert the concrete instance for the shipping
constants.

**Rationale**: checklist item CHK028 found that the natural phrasing, "ten blocks
still fit one row", is true for the wrong reason. The property depends on
`NUM_BLOCKS <= COLUMNS`, and blocks eleven through sixteen would preserve it by
luck while the seventeenth breaks it. A test that encodes the number ten passes
for six more slices and then fails in a slice that has no idea it inherited the
assumption. A test that encodes the dependency describes the boundary before
anyone reaches it.

**Alternatives considered**: asserting only the concrete extent for the current
constants, which is what the literal reading of FR-017 would have produced.
Rejected for the reason above. Asserting only the general property was also
rejected: the concrete instance is what catches a mistake in the shipping
constants themselves.

## R5: Which existing block is the reference implementation?

**Decision**: the combat block (slice 031), throughout.

**Rationale**: it is the closest sibling in every dimension that matters. It is a
small-cardinality player-state signal, it is an observable with no consumer, it
uses marker-plus-checksum validation, it clears to its unknown state on any
sample that fails to decode rather than holding, and it renders unconditionally
so that absence has exactly one meaning. Following it byte for byte means this
slice adds no new pattern for a future reader to learn, which is the outcome
build plan 010 was sequenced to produce.

Two deliberate divergences, both recorded above: the code table reserves values
(combat has exactly two states and reserves nothing), and the naming is for the
growable concept rather than the shipping axis (combat's concept and axis are the
same thing, so the question never arose).

**Alternatives considered**: following the menu block, which is the most recently
added multi-value block. Rejected because its cardinality is open-ended and its
`Other` fallback exists precisely so an unrecognized surface still gates
correctly, which is the opposite of what this block needs: here an unrecognized
code must decode to unavailable, never to a state.
