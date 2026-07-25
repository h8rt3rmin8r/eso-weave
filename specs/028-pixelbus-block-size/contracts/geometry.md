# Contract: Pixel-Bus Block-Size Geometry

This is the byte-for-byte agreement between the PixelBeacon addon (draw side) and
the companion reader (sample side). Both sides derive all geometry from one value,
`block_px`, and MUST use these exact formulas.

## The single value

- `block_px`: the physical-pixel edge length of each square block. Even integer,
  `2 <= block_px <= 32`, default `16`.
- `NUM_BLOCKS = 4`: the fixed number of blocks (B0 status, B1 fishing, B2 latency,
  B3 weapon). Unchanged by this feature.

## Draw side (addon `PixelBeacon.lua`)

- Root strip size: `physicalToUi(block_px * NUM_BLOCKS)` by `physicalToUi(block_px)`.
- Block `N` (0-based) left edge: `physicalToUi(block_px * N)`; each block is
  `block_px` by `block_px` physical pixels, filled with its center color.
- The addon's `local BLOCK_PX` MUST equal the companion's `block_px`. The
  companion guarantees this by rewriting the `local BLOCK_PX = N` line when it
  deploys the addon (`render_lua(block_px)`), preserving every other line
  including the managed marker in the manifest.

## Sample side (companion reader)

- Block `N` center (the sampled point):
  `block_center(block_px, N) = (block_px * N + block_px / 2, block_px / 2)`.
  - B0 status: `block_center(block_px, 0)`
  - B1 fishing: `block_center(block_px, 1)`
  - B2 latency: `block_center(block_px, 2)`
  - B3 weapon: `block_center(block_px, 3)`
- Windows capture region: `capture_dims(block_px) = (block_px * NUM_BLOCKS, block_px)`.
  The captured strip's top-left is the client top-left (unchanged); a point is
  read from the strip at `block_center(block_px, N)`.
- Linux reads each `block_center` point directly (1x1); it has no capture region.

## Worked values (MUST match)

| block_px | B0 | B1 | B2 | B3 | capture (w x h) |
| --- | --- | --- | --- | --- | --- |
| 2  | (1, 1)   | (3, 1)   | (5, 1)   | (7, 1)   | 8 x 2   |
| 4  | (2, 2)   | (6, 2)   | (10, 2)  | (14, 2)  | 16 x 4  |
| 8  | (4, 4)   | (12, 4)  | (20, 4)  | (28, 4)  | 32 x 8  |
| 16 | (8, 8)   | (24, 8)  | (40, 8)  | (56, 8)  | 64 x 16 |
| 32 | (16, 16) | (48, 16) | (80, 16) | (112, 16)| 128 x 32|

The `block_px = 16` row MUST be identical to the current release
(`status_point (8,8)`, `fishing_point (24,8)`, `latency_point (40,8)`,
`weapon_point (56,8)`; capture 64 x 16).

## Color/marker contract (unchanged by this feature)

Block color encodings (status magenta, fishing waiting/bite, latency marker
`0xA5` + checksum, weapon marker `0x5A` + nibble-packed classes) are unchanged.
This feature changes only where the blocks are drawn and read, not what colors
they carry. The per-channel match `tolerance` (default 2) is unchanged.

## Invariants (tested)

1. For every supported `block_px`, the reader's four points equal the draw-side
   block centers (the table above).
2. `render_lua(block_px)` changes only the `local BLOCK_PX` line and preserves
   all other addon content, including the managed marker in the deployed manifest.
3. A block-size-driven re-deploy writes only a managed `PixelBeacon` folder; an
   unmanaged or absent folder is never written or deleted.
4. At `block_px = 16` the derived geometry equals the current release exactly.
5. `sanitize_block_px` maps any input to an even value in `[2, 32]` and records a
   notice when it changes the value.
