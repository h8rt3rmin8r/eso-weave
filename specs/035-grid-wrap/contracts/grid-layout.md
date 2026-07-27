# Contract: The Beacon Grid Layout

**Feature**: `specs/035-grid-wrap/` | **Date**: 2026-07-27

This is the coordinate contract two independently written programs must satisfy
identically: the PixelBeacon addon, which draws, and the ESO Weave companion,
which reads. It supersedes the linear strip contract that preceded it, and at the
current block count the two are the same contract.

## The rule

Given a block index `i`, a column count `C`, and a block edge length `p` in
physical pixels:

```
col    = i mod C
row    = i div C
origin = (p * col, p * row)          -- top-left of the block, client-relative
centre = (p * col + p/2, p * row + p/2)
```

The grid is anchored to the top-left of the game window's client area, so
`origin` is client-relative and the first block's origin is `(0, 0)`.

The occupied extent, for a block count `N`:

```
rows   = ceil(N / C)
width  = p * min(N, C)
height = p * rows
```

The addon draws its container at that extent and the companion captures exactly
that region.

## The column count

`C = 16`, fixed.

It is stated once in each program and the companion's test suite asserts the two
agree by parsing the addon source it embeds. **It is not derived from the display
resolution, the client area, the UI scale, or anything else measured, on either
side.** A derived count would require both programs to compute the identical
integer from independent measurements; a disagreement of one shifts every block
from row 1 onward, and the companion would then read real blocks that pass their
marker and checksum checks and report each signal as another signal's value. The
error would sit underneath the validation that exists to catch exactly that.

Changing `C` is a breaking change to this contract. Both sides must change
together, in one commit, and the manifest version must advance. The build fails
if only one side changes, which is the intended safety net rather than an
inconvenience.

## Guarantees

1. **Distinct positions.** No two indices below the block count share a position.
2. **Containment.** Every index below the block count lands inside the extent.
3. **Not the converse.** When `N` is not a multiple of `C` the final row is
   partial, so the extent contains cells no index maps to. The addon does not
   draw them and the companion does not read them; whatever the game renders
   there is irrelevant. Do not assume a full final row.
4. **Whole-pixel centres.** `p` is even by sanitization, so `p/2` is exact and
   every sampled centre is a whole pixel on both axes.
5. **The heartbeat block is at `(0, 0)`** for any `C`, so signal-loss detection
   is independent of the layout.
6. **Row 0 is the old strip.** For `i < C`, `centre` reduces to
   `(p * i + p/2, p/2)`, which is the pre-wrap formula. For `N <= C`, the extent
   reduces to `(p * N, p)`, which is the pre-wrap capture region. At `N = 9` and
   `C = 16`, this contract and the strip contract it replaces are the same
   contract, block for block and pixel for pixel.

Guarantee 6 is why an addon at the previous version works with a companion at
this one, and why the reverse also works. That compatibility is a consequence of
the arithmetic rather than a compatibility path anyone has to maintain, and it
holds only until the block count exceeds `C`.

## The fit check

The companion evaluates whether `width by height` lies inside the measured client
area, and reports when it does not. This is **advisory**. Nothing branches on it:
sampling, decoding, and every signal behave identically whether the grid fits or
not, because the blocks that do fit still decode correctly and refusing to sample
would turn a partial loss into a total one.

The check compares extents only. The grid's offset within the client area is
always zero, and the window's position on the desktop is a capture concern rather
than a layout one.

It runs only against a live measurement. A descriptor derived from stored
settings is produced only when there is no window, and no window means no drawn
grid to be about.

The failure it exists to catch is worth naming: a block drawn past the client
edge is captured as black, fails its marker check, and decodes as absent, which
is indistinguishable from an addon that was never installed.

## Stability

The extent grows downward as blocks are added and never sideways. A consumer may
rely on the width being bounded by `p * C` forever, and on the height being the
only dimension that grows.

Adding a block remains what it was: raise the block count on both sides, add a
sample point, add a field, add a marker to the registry. Nothing about adding a
block needs to think about the grid, which is the point of landing this before
the seventeenth block rather than with it.
