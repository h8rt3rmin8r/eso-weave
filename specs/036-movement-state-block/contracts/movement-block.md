# Contract: Movement Block (B9)

**Feature**: [spec.md](../spec.md) | **Date**: 2026-07-27

This is the wire contract between the PixelBeacon addon and the ESO Weave
companion for block index 9. Both sides implement it independently and
`tests/beacon.rs` proves they agree by parsing the addon source embedded in the
companion binary. Changing either side without the other is a defect the build
must catch, not a review must catch.

## Position

| Property | Value |
| --- | --- |
| Block index | 9 (the tenth block) |
| Grid position | `grid_position(9, COLUMNS)` = column 9, row 0 |
| Sample point | `block_center(9, block_px)`, the block's center pixel |
| Introduced in addon version | 10 |

Position is derived on both sides from the block index, the configured block
size, and the shared column count. Neither side restates the coordinate.

## Encoding

One pixel, three channels.

| Channel | Carries | Rule |
| --- | --- | --- |
| Green | Validity mark | Constant `0x43` |
| Red | State code | One of the table below |
| Blue | Checksum | `255 - red` |

### Code table

| Code | Bits (sprint, mounted) | Meaning | Red | Blue | Emitted |
| --- | --- | --- | --- | --- | --- |
| 0 | `0b00` | On foot | `0x20` | `0xDF` | Yes |
| 1 | `0b01` | Mounted | `0x60` | `0x9F` | Yes |
| 2 | `0b10` | Sprinting, on foot | `0xA0` | `0x5F` | **No, reserved** |
| 3 | `0b11` | Sprinting, mounted | `0xE0` | `0x1F` | **No, reserved** |

Codes 2 and 3 are reserved for the sprint axis, which is deferred because the
game exposes no sprint observable (see [research.md](../research.md) §R0). The
addon MUST NOT emit them. The companion MUST decode them as unavailable. They are
defined on the companion side only, so the cross-language agreement check needs
no special case for values that exist on one side (§R3).

A future sprint feature sets bit 1 and emits codes 2 and 3. The two live codes
keep their meanings and their exact colors, so that feature changes no shipped
pixel.

## Decoding

Applied in order. Any failure yields the unavailable state; there is no nearest
match and no default to a real state.

1. Green within `tolerance` of `0x43`, else unavailable.
2. `red + blue` within `tolerance` of 255, else unavailable.
3. Red within `tolerance` of `0x20` gives on foot; within `tolerance` of `0x60`
   gives mounted; anything else, including `0xA0` and `0xE0`, gives unavailable.

`tolerance` is `ReaderConfig::tolerance`, default 2.

## Separation guarantees

| Guarantee | Margin | Multiple of default tolerance |
| --- | --- | --- |
| `0x43` from the nearest other block-center green (`0x2D`) | 22 | 11x |
| Between adjacent codes | 64 | 32x |

The green margin is enforced automatically: `0x43` is registered in
`BLOCK_CENTER_GREENS`, and `tests/pixelbus.rs` asserts every pair in that
registry is separated by more than the default tolerance. A future block choosing
a colliding mark fails the build and is told which mark it collided with.

## Rendering discipline

- The block is drawn whenever the status block is drawn, and is hidden only when
  the status block is. It is never hidden to express a state, so its absence
  means exactly one thing: the installed addon predates version 10.
- The addon redraws only on a real state change, so a steady state produces a
  steady signal and the companion emits no repeated events.
- The state is driven by `EVENT_MOUNTED_STATE_CHANGED` for instant transitions
  and re-baselined from `IsMounted()` on `EVENT_PLAYER_ACTIVATED`, because no
  transition event fires for a state that is already true when the world loads.

## Compatibility

| Case | Behavior |
| --- | --- |
| Addon older than version 10 | No block at index 9. Whatever is behind the overlay fails the marker or checksum check. Companion reports unavailable and emits no movement event. |
| Companion older than the addon | Block 9 is drawn but never sampled. No existing signal is disturbed, because every block is read at its own position. |
| Beacon alive, block stops decoding | Companion clears to unavailable on that sample rather than holding the last value. |
| Beacon signal lost | Companion clears to unavailable, through the same signal-loss path every other block uses. |
