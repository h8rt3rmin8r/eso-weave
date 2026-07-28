# Build Plan 011: Pixel Bus Grid Wrap

Plan: 011
Status: active
Master specification: `docs/ESO-Weave-Specification.md`
Constitution: `.specify/memory/constitution.md`

## Purpose

Build plan 010 set out to close the five open PixelBeacon issues. Four are done
(#9, #10, #2, #3) and the fifth (#11, movement state) is blocked on an external
unknown: no direct sprint API boolean exists, and that issue requires the real
observable be verified before any encoding is fixed. So the plan is not finished,
but it is stalled, and the thing it was clearing the way for is now reachable.

Slice 034 delivered the display descriptor, which was groundwork for wrapping the
beacon strip into a grid. The wrap itself was filed as issue #16 once the
descriptor existed, exactly as issue #3 said it should be. This plan is that
wrap, and it is one slice.

The wrap matters because the strip does not scale and every slice of build plan
010 made that worse. Nine blocks is 144 physical pixels at the default size; the
observables worth publishing run into the hundreds, and a two hundred block strip
is 3200 pixels wide, which fits on nothing. Wrapping the squares into rows
changes the asymptote. Nothing else in the design does.

It runs ahead of the blocked movement slice for two reasons. The obvious one is
that a blocked slice should not hold up an unblocked one. The less obvious one is
that the movement slice adds a block, and every block-adding slice is cheaper
after the wrap lands than before it, because the wrap is the last change to the
geometry contract that every future block inherits. Landing it while the block
count is nine, rather than after several more have been added, is the cheapest
this change will ever be.

## Slices

### Slice 035: Pixel Bus Grid Wrap

Scope: replace the linear block position with a grid position on both sides of
the pixel-bus contract, and validate the resulting extent against the measured
client area. Closes issue #16.

The geometry change is small and symmetric. The companion's `block_center`
currently returns `(block_px * index + block_px / 2, block_px / 2)` and the addon
anchors block `index` at `x = BLOCK_PX * index, y = 0`. Both become a column and
row derived from one shared count: `col = index % COLUMNS`,
`row = index // COLUMNS`. The captured region and the addon's root control follow
from the same numbers, with the width taken from `min(NUM_BLOCKS, COLUMNS)` so
the region stays as small as the blocks actually in use require. That last detail
is not cosmetic: the capture is a screen `BitBlt` running at up to 10 Hz, and
sizing it to the column count rather than the used extent would multiply its cost
by the width of a grid that is mostly empty.

The load-bearing decision is that the column count is a fixed constant shared
byte for byte, not a value each side computes from the live client width. Issue
#16 argues this at length and the feature spec should carry the argument rather
than restate the conclusion, because it reverses what issue #3 assumed. The short
form: two independently obtained measurements would have to produce the identical
integer, and a one-column disagreement does not degrade, it shifts every block
from the second row onward so the companion reads valid, checksum-passing colours
from the wrong squares. `COLUMNS` instead joins `NUM_BLOCKS` as a value stated
once per side and asserted equal by the cross-language check in `tests/beacon.rs`
that slice 031 established, so agreement holds by construction.

That reframes what slice 034's descriptor is for here, and the feature should say
so plainly rather than let the earlier framing stand. The descriptor is not the
source of the column count. It is the validator: it confirms that
`block_px * columns` by `block_px * rows` fits inside the measured client area,
and surfaces a notice when a configured block size and block count would put part
of the grid off-screen. That case is worth catching, because a block drawn past
the edge is captured as black, fails its marker check, and decodes as absent,
which looks exactly like an addon that is not installed.

The property that makes this slice safe is that it changes nothing today. At
`COLUMNS = 16` and `NUM_BLOCKS = 9` every block lands in row 0 at the coordinates
it occupies now and the captured region is unchanged, so the contract can land,
ship, and be seen working against the live game before any block depends on it.
A test asserting the wrapped layout at nine blocks is identical to the strip
layout it replaces is a required deliverable, not an incidental one; it is the
evidence that this is a contract change and not a behaviour change. The first
slice to add a seventeenth block gets wrapping for free and is the first to
exercise row 1, and it should be able to do so without revisiting any of this.

The choice of column count is a decision for the feature, evaluated against the
supported block-size range (2 to 32) and the smallest client area the game
supports. Sixteen is the value issue #16 works through and it has the useful
property of being at least the current block count, which is what makes the
no-op-today guarantee hold; a different value is fine if it holds that property
and the grid still fits at the maximum block size.

This is one slice rather than two. Splitting the geometry from the fit check
would mean two addon manifest versions for one contract change and would ship an
intermediate state where the grid exists but nothing checks it fits, which is the
half that fails silently. The manifest advances from version 8 to 9 because the
drawn contract changes, even though the drawn output at nine blocks does not.

Out of scope: any new block, any new observable, and any change to an existing
block's encoding, marker, or meaning. This slice moves squares and validates
where they land. `CHANGELOG.md` records the feature plus a dated decision for the
fixed column count, which is a contract every later slice inherits. The master
specification's section 10.3 block table gains grid coordinates and the wrap rule
alongside them. Feature under `specs/035-<name>/`.

## Note on slice numbering

Build plan 010 named its movement-state slice 035. Spec directories are numbered
by the next free index at the time the feature is specified, not reserved in
advance, so the grid wrap takes 035 and the movement slice takes the next free
number when its sprint blocker clears. Plan 010's ordering is unchanged; only the
directory number moves, and plan-010.md has been amended to match.
