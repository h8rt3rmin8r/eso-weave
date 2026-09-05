# Contract: PixelBeacon Layout Header Version 1

**Feature**: `specs/045-negotiated-width-geometry/` | **Date**: 2026-09-04

This contract supersedes the fixed 16-column shipping geometry from slice 035.
It leaves 16 columns as the read-only legacy contract.

## Authority

PixelBeacon alone selects `columns`. The companion MUST NOT independently
derive or substitute a negotiated count. Its operating-system measurement is a
validation ceiling only.

## Invariant header

The first three physical cells are always at row zero, columns zero through two:

```text
H0 = (0x45, 0x53, 0x20) # spaced wire code for protocol version 1
H1 = (columns_high, 0x64, 255 - columns_high)
H2 = (columns_low,  0x9C, 255 - columns_low)
```

The H0 channels, the two byte markers, and both complements are matched with the
configured per-channel tolerance. Version 1 uses wire code `0x20`; future
versions MUST use codes separated by more than twice the maximum supported
tolerance. A complement passes when `abs((red + blue) - 255) <= tolerance`.

The column count is `(columns_high << 8) | columns_low` and MUST be between 3 and
65535 inclusive.

## Physical capacity

```text
physical_width = floor(GuiRoot:GetWidth() * GetUIGlobalScale())
columns = clamp(floor(physical_width / BLOCK_PX), 3, 65535)
```

Reserved right-edge margin is zero. The count includes header and payload cells.
When a measured companion surface is available, `columns * BLOCK_PX` MUST NOT
exceed its width. The full occupied height MUST NOT exceed its height.

## Payload geometry

For payload signal index `i`:

```text
cell = 3 + i
col = cell mod columns
row = cell div columns
center = (BLOCK_PX * col + BLOCK_PX / 2,
          BLOCK_PX * row + BLOCK_PX / 2)
```

For `N` payload signals, the complete occupied extent is:

```text
total_cells = 3 + N
width = BLOCK_PX * min(total_cells, columns)
height = BLOCK_PX * ceil(total_cells / columns)
```

The three header centers use the same cell-center formula with cell indices
zero through two.

## Failure and compatibility

- H0 magic valid plus any version, marker, checksum, bound, or fit failure is
  unavailable. It MUST NOT fall back.
- H0 not magic plus a valid legacy magenta heartbeat is Legacy.
- H0 neither magic nor heartbeat is Missing.
- Legacy uses 16 columns, payload offset zero, and no header cells.
- The manifest protocol generation is version 14.

## Lifecycle

PixelBeacon recomputes capacity on `EVENT_SCREEN_RESIZED` and its one-second
tick. The reader decodes H0 through H2 every batch. A state change is emitted
once, and all payload points for that batch use the newly validated object.

## Capture budget

- Steady layout: one prepared frame.
- Initial acquisition: header frame plus complete frame.
- Growth beyond the prepared extent: one additional complete frame.
- Shrink or same-sized reflow: reuse the already prepared containing frame.
