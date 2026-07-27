# Data Model: Pixel Bus Grid Wrap

**Feature**: `specs/035-grid-wrap/` | **Date**: 2026-07-27

There are no new data structures carrying state. The grid is arithmetic, and the
one stateful piece is a two-field change detector. This document is therefore
mostly about functions and the properties they must satisfy.

## The shared constant

```rust
/// The number of blocks in one row of the beacon grid.
pub const COLUMNS: u32 = 16;
```

Stated once on each side: `pixelbus::COLUMNS` and `local COLUMNS = 16` in
`PixelBeacon.lua`. `tests/beacon.rs` asserts the two are equal by reading the
addon source embedded in the binary, using the same
`beacon::parse_lua_constant` facility that already pins `NUM_BLOCKS` and every
marker value.

Not configurable, and that is a requirement rather than an omission: the two
sides must agree byte for byte, and a user-editable value is a way for them to
disagree that no test can close, since the addon reads its constant from a file
the application deploys.

Constraint check at the shipped values:

| Check | Requirement | Value |
| --- | --- | --- |
| At least the block count | FR-009 | `16 >= 9` |
| One row fits the narrowest client at the largest block | FR-010 | `16 * 32 = 512 <= 1024` |

## Grid arithmetic (`src/pixelbus/mod.rs`)

```rust
pub fn grid_position(index: u32, columns: u32) -> (u32, u32)
pub fn grid_rows(count: u32, columns: u32) -> u32
pub fn grid_extent(block_px: u32, count: u32, columns: u32) -> Size
```

| Function | Definition | Notes |
| --- | --- | --- |
| `grid_position` | `(index % columns, index / columns)` | Column first, then row. Panics are impossible for `columns >= 1`; the constant is 16 and the parameterised form is only called from tests and from the three functions below. |
| `grid_rows` | `count.div_ceil(columns)` | 0 for a count of 0. |
| `grid_extent` | `Size::new(block_px * min(count, columns), block_px * grid_rows(count, columns))` | The width is `min(count, columns)`, not `columns`, so a grid using a fraction of one row does not claim a full row's width. |

`block_center` and `capture_dims` keep their signatures and derive from the
above:

```rust
pub fn block_center(block_px: u32, index: u32) -> (u32, u32) {
    let (col, row) = grid_position(index, COLUMNS);
    (block_px * col + block_px / 2, block_px * row + block_px / 2)
}

pub fn capture_dims(block_px: u32) -> (u32, u32) {
    let extent = grid_extent(block_px, NUM_BLOCKS, COLUMNS);
    (extent.width, extent.height)
}
```

**Properties the tests pin** (FR-005, FR-012, FR-013):

- *Injective*: no two indices below the count share a position.
- *Contained*: every index below the count lands inside the extent.
- *Not surjective, and that is correct*: when the count is not a multiple of the
  column count the final row is partial, so the extent contains positions no
  index maps to. Requiring otherwise would forbid every block count that is not a
  multiple of 16. This was the defect the geometry checklist caught in FR-005.
- *Whole-pixel centres*: `block_px` is even by sanitization, so `block_px / 2` is
  exact on both axes.
- *Row 0 identity*: for `index < COLUMNS`, `grid_position` yields `(index, 0)`,
  so `block_center` reduces to `(block_px * index + block_px / 2, block_px / 2)`,
  which is the pre-wrap formula exactly.
- *Narrow-grid identity*: for `count <= columns`, `grid_extent` is
  `(block_px * count, block_px)`, which is the pre-wrap capture region exactly.

The last two are what FR-014 requires be asserted rather than reasoned about.

## The fit check (`src/pixelbus/display.rs`)

```rust
pub enum GridFit {
    Fits,
    Exceeds { grid: Size, surface: Size },
}

pub fn grid_fit(grid: Size, surface: Size) -> GridFit
```

`Fits` when `grid.width <= surface.width && grid.height <= surface.height`.

Extents only. The grid is anchored at the client area's top-left, so its offset
within that area is always zero, and the surface's own position on the desktop is
a capture concern the sampler already handles rather than a layout concern.

```rust
pub struct GridFitWatch { /* last outcome */ }

impl GridFitWatch {
    pub fn new() -> Self;
    pub fn observe(&mut self, grid: Size, descriptor: Option<&DisplayDescriptor>)
        -> Option<GridFit>;
}
```

`observe` returns `Some` only when the outcome changes, so a caller may report
unconditionally on `Some`.

| Input | Behaviour |
| --- | --- |
| No descriptor | Returns `None`, and clears the remembered outcome so a later measurement reports afresh. Nothing to be about. |
| A `Configured` descriptor | Same as no descriptor. It is produced only when there is no window, and no window means no drawn grid. |
| A `Measured` descriptor, outcome unchanged | `None`. |
| A `Measured` descriptor, outcome changed | `Some(outcome)`. |

The distinction between "changed descriptor" and "changed outcome" is the whole
reason this type exists rather than a bare comparison at the call site: two
successive descriptor changes can both be `Exceeds` (a too-small window resized to
a differently too-small one), and FR-019 asks for one report per outcome change.

## Addon side (`addon/PixelBeacon/PixelBeacon.lua`)

```lua
local COLUMNS = 16

local function positionBlock(control, index)
    local col = index % COLUMNS
    local row = math.floor(index / COLUMNS)
    control:ClearAnchors()
    control:SetAnchor(TOPLEFT, root, TOPLEFT,
        physicalToUi(BLOCK_PX * col), physicalToUi(BLOCK_PX * row))
    local dimension = physicalToUi(BLOCK_PX)
    control:SetDimensions(dimension, dimension)
end
```

`positionBlock` takes a block *index* rather than an x offset in physical pixels,
which is the one call-site change: `positionBlock(blocks.latency, BLOCK_PX * 2)`
becomes `positionBlock(blocks.latency, 2)`. That is a better interface
independently of the wrap, since the caller was previously multiplying by
`BLOCK_PX` at nine call sites to express what is fundamentally an index.

The root control's dimensions derive from the same grid:

```lua
local columnsUsed = math.min(NUM_BLOCKS, COLUMNS)
local rows = math.ceil(NUM_BLOCKS / COLUMNS)
root:SetDimensions(physicalToUi(BLOCK_PX * columnsUsed), physicalToUi(BLOCK_PX * rows))
```

Anchoring, draw layer, and every render function are untouched.

## What is not in this model

No new block, signal, marker, colour, event, or cadence. No change to any
decoder. No new configuration. The grid's extra capacity is created here and
consumed by a later feature.
