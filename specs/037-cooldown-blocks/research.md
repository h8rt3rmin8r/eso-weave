# Phase 0 Research: PixelBeacon Skill-Cooldown Blocks

**Feature**: [spec.md](spec.md) | **Date**: 2026-07-27

All decisions were made under the build-phase autopilot decision policy. None
were escalated. No NEEDS CLARIFICATION markers remain.

## R1: Which slots does the game expose a cooldown for?

**Decision**: the five normal action slots and the ultimate. Six blocks. Synergy
gets none.

**Rationale**: this was checked rather than assumed, and it changed the feature.
The game's own action bar, in `esoui/ingame/actionbar/actionbar.lua`, iterates
its slots from `ACTION_BAR_FIRST_NORMAL_SLOT_INDEX + 1` through
`ACTION_BAR_ULTIMATE_SLOT_INDEX + 1`. That range is the five skills plus the
ultimate. Synergy is not in it, because Synergy is a contextual prompt bound to a
key rather than an action slot, so `GetSlotCooldownInfo` has nothing to return
for it in any state.

The addon derives its indices from those named constants rather than hardcoding
integers, so a future change to the bar layout cannot silently misalign the
blocks against the slots.

**Alternatives considered**:

- **Seven blocks, with Synergy permanently unavailable.** Rejected. It buys
  positional symmetry with an interface row and costs a square, a validity mark,
  a permanently muted interface cell, and, decisively, a seventeenth block that
  would push the grid onto a second row purely to carry nothing.
- **Five blocks, skills only.** Rejected. Ultimate is a real action slot in the
  game's own iteration range; excluding it would drop a signal that exists.

## R2: The six validity marks

**Decision**: `0x0B`, `0x21`, `0x4E`, `0x92`, `0xC6`, `0xE8`, assigned to B10
through B15 in ascending slot order.

**Rationale**: eleven greens are already in use. Sorted, the gaps between them
are 22, 23, 22, 23, 19, 19, 37, 22, 23, and 45 wide. Taking the midpoints of the
six widest gaps yields the set above, whose nearest-neighbour distances are 11,
11, 11, 18, 11, and 22.

The resulting minimum separation across the whole seventeen-value registry is 11,
which is 5.5 times the default tolerance of 2. This is tighter than the 22 that
slice 036 achieved with a single mark, and that is the honest cost of adding six
marks at once rather than one: with seventeen values in a 256-wide channel and
the incumbents fixed, 11 is the best achievable minimum. Placing two marks in the
widest gap and fewer elsewhere was evaluated and does not improve it, because the
binding constraint is the 22-wide gaps, not the 45-wide one.

Eleven remains a wide margin against the tolerance that actually governs
decoding, and the margin is enforced automatically once the marks are registered.

**Alternatives considered**:

- **One shared mark for all six blocks.** Rejected, and the spec records why: the
  registry exists so a geometry error off by one block fails loudly instead of
  decoding a neighbour's value as this slot's. Six adjacent squares carrying the
  same kind of value are exactly where that failure would be silent and
  plausible.
- **Reusing the two 19-wide gaps to spread the set further.** Rejected: it lowers
  the minimum separation to 9, worse on the only metric that matters.

## R3: Quantization

**Decision**: `red = min(remaining_ms / 50, 254)`, with `0` meaning ready and
`0xFF` meaning unavailable.

**Rationale**: 50 ms per step across 254 steps covers 12.7 seconds, which spans
ordinary skill cooldowns. The resolution is far finer than the sampling interval,
so the transport is never the limiting factor on precision; making it finer would
buy accuracy the sampling cadence cannot deliver, and making the range longer
would cost resolution in the sub-second region where weaving actually happens.

Saturating at 254 rather than wrapping is what makes a longer-than-encodable
cooldown read as "at least this long" instead of as a small number, which is the
difference between a degraded reading and a wrong one.

`0xFF` for unavailable is the same choice `RESOURCE_UNAVAILABLE` already made: it
passes the marker and checksum checks and fails the range check, so it needs no
special case in the decoder and a reader of either decoder recognizes the other.

**Alternatives considered**:

- **Milliseconds scaled to the full byte (about 50 ms per step to 12.75 s).**
  Effectively the same, but it leaves no room for a distinct unavailable value,
  forcing a second block or a marker trick to express it.
- **A coarse bucket table (ready, under a second, a few seconds, long).**
  Rejected: it discards exactly the precision a future scheduling consumer needs,
  which is the entire reason for the slice.

## R4: One aggregate value or six separate ones

**Decision**: one `CooldownSet` and one `PixelBusEvent::Cooldowns`.

**Rationale**: the resource blocks already settled this shape, and their doc
comment states the reason: a sample in which several values move at once is the
common case in combat, and should be one event rather than several. Six slots
strengthen that argument, because a single weave can move most of them within one
sample.

It also discharges the log-volume requirement structurally rather than by
discipline. One event per changed sample is one log entry per changed sample, so
the flooding the requirement forbids cannot arise from the event shape at all.

**Alternatives considered**:

- **Six separate events.** Rejected: it makes a single weave produce up to six
  events and six log lines, and it would push the flooding problem onto the
  logging layer to solve by rate-limiting, which is a worse place to solve it.

## R5: What the row count does

**Decision**: the grid reaches exactly sixteen blocks, one full row, and the
compile-time assertion guarding the boundary is left untouched.

**Rationale**: `grid_rows(16, 16)` is 1 and `grid_extent(px, 16, 16)` is
`(px * 16, px)`. The geometry needs no change: the width already takes the lesser
of the count and the column count, the height already derives from the row count,
the captured region already derives from both, positions already resolve through
the wrap, and the overflow check already compares both axes.

The assertion `NUM_BLOCKS <= COLUMNS` now holds with no margin. Leaving it is the
whole point: the next block added in this family trips it, and that block belongs
to the next slice. Relaxing it here would throw away the warning exactly when it
becomes useful.

**Alternatives considered**:

- **Pre-emptively widening the assertion to allow two rows.** Rejected. It would
  silence the one signal that tells the next slice it is crossing a boundary, in
  the slice that does not cross it.
