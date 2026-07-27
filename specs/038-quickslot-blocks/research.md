# Phase 0 Research: PixelBeacon Quickslot-State Blocks

**Feature**: [spec.md](spec.md) | **Date**: 2026-07-27

All decisions were made under the build-phase autopilot decision policy. None
were escalated. No NEEDS CLARIFICATION markers remain.

## R1: Which calls does the addon use, and are they all available?

**Decision**: five calls, all verified present in the current API and all
unprotected.

| Call | Signature | Used for |
| --- | --- | --- |
| `GetCurrentQuickslot` | `GetCurrentQuickslot()` | the active quickslot's action slot index |
| `GetSlotItemLink` | `GetSlotItemLink(luaindex actionSlotIndex, HotBarCategory hotbarCategory)` | the item in that slot |
| `GetItemLinkItemType` | `GetItemLinkItemType(string itemLink)` | whether it is a potion |
| `GetItemLinkItemId` | `GetItemLinkItemId(string itemLink)` | the identity to publish |
| `GetSlotCooldownInfo` | `GetSlotCooldownInfo(luaindex actionSlotIndex, HotBarCategory:nilable hotbarCategory)` | the remaining cooldown |

**Rationale**: each was looked up rather than assumed, because the whole slice is
worthless if one of them turns out to be protected or renamed. `is_protected` is
0 on all five.

## R2: Where does the cooldown come from?

**Decision**: `GetSlotCooldownInfo(GetCurrentQuickslot(), HOTBAR_CATEGORY_QUICKSLOT_WHEEL)`.

**This is a deliberate deviation from GitHub issue #19**, which proposed reading
`remainingCooldown` from `GetItemLinkOnUseAbilityInfo`. The deviation is
recorded here and surfaced at the pre-push halt.

**Rationale**: three reasons, in order of weight.

1. It answers the right question. `GetSlotCooldownInfo` returns what the *slot*
   has left, which is what "can this be drunk right now" actually depends on.
   `GetItemLinkOnUseAbilityInfo` describes the item's on-use ability. Potions in
   this game share a cooldown, so the slot is the authority on usability and the
   item link is not.
2. It is the same call the skill cooldown blocks already make. The previous
   slice's quantization, saturation, and unavailable rules were written against
   this function's return value. Reusing it makes the encoding contract literally
   shared rather than a parallel implementation that can drift, which is the
   failure mode the whole single-source-of-truth discipline in this codebase
   exists to prevent.
3. The signature settles the doubt that motivated the issue's choice. The second
   parameter is a nilable `HotBarCategory`, so the quickslot hotbar can be named
   explicitly rather than hoped for. The issue was written before that signature
   was checked.

`GetItemLinkOnUseAbilityInfo` is still used, for one thing only: its `hasAbility`
return, as a second condition alongside the item type. An item that is typed as a
potion but has no on-use ability is not something to fire a key at.

**Alternatives considered**:

- **The issue's `remainingCooldown`.** Rejected for reason 1 above: it can report
  an item ready while the slot is not, which is the exact false-positive the
  consumer one slice away would act on.
- **Both, with a precedence rule.** Rejected. Two sources with a tiebreak is a
  heuristic, and this project has now twice rejected heuristics in this contract
  (the sprint observable, the localized description). One authoritative source or
  nothing.

## R3: What does the addon do if a needed game constant is absent?

**Decision**: publish the unavailable payload on all four blocks and read
nothing. Never guess a hotbar category and never guess an item type.

**Rationale**: `HOTBAR_CATEGORY_QUICKSLOT_WHEEL` and `ITEMTYPE_POTION` are game
globals rather than function returns, so a rename lands as `nil` rather than as
an error. Passing `nil` where a hotbar category belongs would let the game
resolve some other hotbar, and the reader would then receive a valid,
checksum-passing colour describing a slot nobody asked about. That is a false
reading with full integrity checks behind it, which is worse than no reading at
all and is precisely what User Story 2 forbids. The unavailable payload costs the
operator a muted readout and costs the consumer nothing, because the consumer's
precondition is not met either way.

## R4: The identity, not the restore types

**Decision**: publish `GetItemLinkItemId`. Do not attempt to publish what the
potion restores.

**Rationale**: `GetItemLinkOnUseAbilityInfo` returns `hasAbility`,
`abilityHeader`, `abilityDescription`, `cooldown`, `hasScaling`, `minLevel`,
`maxLevel`, `isChampionPoints`, `remainingCooldown`. The restore types appear
only inside `abilityDescription`, which is a localized human-readable string.
The game's own interface consumes it as tooltip text. Extracting restore types
from it would be a locale-dependent parse baked into a colour contract shared
byte for byte between two codebases, breaking on any non-English client and on
any wording change in a patch. This is the same class of construct the sprint
verification rejected in slice 036 and the same reason the menu block uses a
scene and UI-mode test rather than a title string.

The identity is machine-readable, stable, and sufficient for the consumer's two
needs: naming the thing in a log or readout, and noticing a swap. Restore
awareness can be layered on later, entirely inside the companion, without
touching the bus contract.

## R5: The four validity marks

**Decision**: `0x38` (B16 status), `0xB0` (B17 identity high), `0xDD` (B18
identity middle), `0xF3` (B19 identity low).

**Rationale**: chosen the way every mark since slice 031 has been chosen, as the
midpoints of the widest gaps left in `BLOCK_CENTER_GREENS`. The registry before
this feature, sorted, is:

```
00 0B 16 21 2D 43 4E 5A 6D 80 92 A5 BB C6 D2 E8 FF
```

The four widest gaps are `E8..FF` (23), `2D..43` (22), `A5..BB` (22), and
`D2..E8` (22). Taking each midpoint yields the four values above and leaves the
registry:

```
00 0B 16 21 2D 38 43 4E 5A 6D 80 92 A5 B0 BB C6 D2 DD E8 F3 FF
```

The minimum adjacent separation is 11, unchanged from before this feature, which
is what matters: adding four marks did not make the tightest pair any tighter.
Eleven is more than five times the default reader tolerance of 2. The existing
automated separation check proves this rather than the author asserting it, which
is why every new mark is registered.

## R6: The identity's width and byte order

**Decision**: 24 bits, most significant byte first, reduced modulo 2^24 on the
publishing side.

**Rationale**: three blocks at one byte each is the smallest layout that covers
the live identity range with headroom. Most-significant-first is chosen because
the ordering has to be written down identically in two languages, and the order
that matches how the number is written is the one an author is least likely to
transcribe backwards. The reduction happens in the addon, before encoding, so
that every block always carries a whole byte. Letting an oversized value produce
an out-of-range byte would fail the checksum, and a failed checksum means
unknown, which the consumer reads as "there is no potion here". Turning "an
identity we cannot name" into "there is nothing here" is a much worse claim than
the aliasing the reduction accepts.

## R7: The row crossing, and every expectation that assumed one row

**Decision**: six sites, found by searching rather than remembered, each updated
to state the two-row shape.

| Site | Today | After |
| --- | --- | --- |
| `tests/pixelbus.rs` const assert (slice 037's, left to fail here) | `NUM_BLOCKS <= COLUMNS` | three const asserts: exactly two rows, first row full, last row partial |
| `tests/pixelbus.rs` `the_column_count_satisfies_both_bounds...` | `COLUMNS >= NUM_BLOCKS` | `COLUMNS >= BLOCKS_AT_WRAP`, the durable form of the same bound |
| `tests/pixelbus.rs` `block_center_and_capture_dims_match_contract_table` | 16 centres, one-row capture | 20 centres, two-row capture |
| `tests/pixelbus.rs` `grid_position_wraps_column_then_row` | every index in row 0 | row 0 below `COLUMNS`, row 1 above |
| `tests/pixelbus.rs` `every_current_block_sits_exactly_where_the_strip_put_it` | strip formula for all | strip formula below `COLUMNS`, wrapped formula above |
| `tests/pixelbus.rs` `the_captured_region_is_exactly_what_the_strip_captured` | one row | one full row wide, two rows tall |
| `tests/pixelbus.rs` `the_capture_region_is_one_row_while_the_count_fits...` | asserts `NUM_BLOCKS == COLUMNS` | asserts the crossing, renamed |

**The bound that changes meaning is worth calling out.** The const assert
`COLUMNS >= NUM_BLOCKS` was written at the wrap with the justification "or the
wrap would move an existing block and forfeit the no-change property". That
justification was always about the blocks that existed *when the wrap shipped*,
which was nine. It was expressed in terms of `NUM_BLOCKS` because at the time the
two were the same thing. They are not any more. Restating it as
`COLUMNS >= BLOCKS_AT_WRAP` with the wrap-era count named as its own constant
keeps the actual invariant and lets the block count grow, which is the whole
point of having wrapped.

**Making the shape assertable at compile time** requires `grid_rows` to be a
`const fn`. It is two arithmetic operations and `u32::div_ceil` is const, so this
is a keyword, not a rewrite. `grid_position` gets the same treatment for
symmetry. This lets the replacement assertion say `grid_rows(NUM_BLOCKS,
COLUMNS) == 2` rather than open-coding the arithmetic, so the assertion and the
function cannot disagree.

## R8: The overlay footprint

**Decision**: report it, document it, change nothing about where the overlay is
or how big its squares are.

At the default square size the overlay is 256 by 32 physical pixels, in the
top-left corner of the game's client area. Before this feature it was 256 by 16.
At the smallest supported square size it is 32 by 4.

**Rationale**: the two things that would actually shrink or move it are both
worse than reporting it.

- **Auto-shrinking the squares.** The square size is a single value shared by
  both sides of the contract; the companion derives its sample points from it and
  the addon derives its draw positions from it. Changing it on one side is how
  every block gets read at the wrong place. It is already an operator setting
  with a supported range down to an eighth of the default, so the remedy exists
  and keeps both sides in agreement.
- **Moving the anchor.** A real option, and deliberately out of scope. The anchor
  is part of the shared geometry contract; relocating it means the addon's anchor
  and the companion's capture origin must agree on a new origin, with a new
  failure mode (an origin disagreement reads valid colours from the wrong place,
  exactly like a column-count disagreement). Bundling that with the first release
  that uses two rows would put two untested geometry changes in one ship.

What is left is making the footprint knowable. Two places, because two different
questions are being asked: a derived caption beside the square-size setting
answers "what will it be if I change this", and a debug log line answers "what
was it during that session". The block size is fixed for the sampling thread's
lifetime, so the log line is emitted once when the thread starts.

## R9: Update cadence

**Decision**: `EVENT_ACTIVE_QUICKSLOT_CHANGED` and `EVENT_ACTION_SLOT_UPDATED`
drive updates, `EVENT_PLAYER_ACTIVATED` re-baselines after a loading screen, and
the existing periodic tick is the backstop.

**Rationale**: the two events cover the two ways the published values move by an
operator action (switching the active quickslot, and the slot's contents
changing). `EVENT_ACTIVE_QUICKSLOT_CHANGED` is verified present and carries the
new `actionSlotIndex`. The re-baseline on `EVENT_PLAYER_ACTIVATED` follows every
other block on the bus and is what satisfies FR-004's loading-screen
requirement.

The tick backstop is not optional here and is the reason the cooldown value is
correct at all: a cooldown counts down continuously and no event fires for each
50 ms step. The tick already exists and already redraws the six skill cooldown
blocks; this adds a fourth block group to the same pass and no new timer.

**No cadence change on either side.** FR-027 forbids it, and nothing consumes the
values, so nothing has a latency requirement to justify one.
