# Contract: Pixel Bus Block B4, Combat State

**Feature**: 031-combat-state-block | **Date**: 2026-07-27

This is the wire contract between the PixelBeacon addon (the producer, Lua) and
the ESO Weave pixel-bus reader (the consumer, Rust). Both sides implement it
independently, so every value here is shared byte for byte and the agreement is
asserted by an automated test rather than trusted.

## Geometry

The strip is a horizontal run of `NUM_BLOCKS` squares, each `BLOCK_PX` physical
pixels on a side, anchored to the top-left of the game client area.

| Property | Value |
| --- | --- |
| Block index | 4 (the fifth block, after status, fishing, latency, weapon) |
| Block count after this feature | 5 |
| Span, in physical pixels | `[BLOCK_PX * 4, BLOCK_PX * 5)` horizontally, `[0, BLOCK_PX)` vertically |
| Sample point | `(BLOCK_PX * 4 + BLOCK_PX / 2, BLOCK_PX / 2)`, the block center |

`BLOCK_PX` is even and in `2..=32`, so the center is always a whole pixel. The
sample point is computed by the existing `block_center(block_px, 4)`; no new
geometry rule is introduced.

The block count is stated once per side:

- Addon: `local NUM_BLOCKS = 5` in `addon/PixelBeacon/PixelBeacon.lua`, with the
  root width computed as `BLOCK_PX * NUM_BLOCKS`.
- Companion: `pub const NUM_BLOCKS: u32 = 5` in `src/pixelbus/mod.rs`, with
  `capture_dims` deriving the capture region from it.

## Encoding

The block is a solid color. Channels carry:

| Channel | Meaning |
| --- | --- |
| Green | Validity marker, constant `0x2D` |
| Red | State code |
| Blue | Complement checksum, `255 - red` |

State codes:

| State | Red | Blue | Full color |
| --- | --- | --- | --- |
| In combat | `0xE0` | `0x1F` | `(0xE0, 0x2D, 0x1F)` |
| Out of combat | `0x20` | `0xDF` | `(0x20, 0x2D, 0xDF)` |

## Decoding

A sample decodes to a state only if all three hold, each within the reader's
per-channel tolerance:

1. Green matches `0x2D`.
2. `red + blue` equals `255`.
3. Red matches one of the two state codes.

If any check fails, the result is the unavailable state. There is no fallback,
no nearest-match, and no default to out of combat.

## Separation guarantees

Measured against the default tolerance of 2.

| Pair | Distance | Margin |
| --- | --- | --- |
| Marker `0x2D` vs status green `0x00` | 45 | 22x |
| Marker `0x2D` vs weapon marker `0x5A` | 45 | 22x |
| Marker `0x2D` vs fishing waiting green `0x80` | 83 | 41x |
| Marker `0x2D` vs latency marker `0xA5` | 120 | 60x |
| Marker `0x2D` vs fishing bite green `0xFF` | 210 | 105x |
| In-combat red vs out-of-combat red | 192 | 96x |

`0xD2`, the nibble swap of `0x2D`, is reserved as the natural marker for the next
block added to the strip, continuing the `0xA5` and `0x5A` pairing.

## Rendering rules (producer)

- The block is drawn whenever the addon is loaded and rendering, and is hidden
  only when the status block is hidden. It never expresses a state by being
  absent.
- It is redrawn only when the decoded state actually changes.
- It is driven by `EVENT_PLAYER_COMBAT_STATE` and re-established from
  `IsUnitInCombat("player")` on `EVENT_PLAYER_ACTIVATED`.

## Consumption rules (consumer)

- A change in the decoded state, including a change to unavailable, is announced
  once. A steady state announces nothing.
- The state clears to unavailable when the beacon signal is lost, and also on any
  sample where the block does not decode. The last known value is never held.
  This differs deliberately from the weapon-bar block, which holds its last
  decoded value while the beacon is alive.
- The decoded value is stored but not acted upon. No timing, input, or fishing
  behavior depends on it.

## Compatibility

- An addon that predates this contract draws four blocks. The companion samples
  the B4 point, reads whatever is behind the overlay, fails the marker or
  checksum check, and reports unavailable. This is a required test case.
- A companion that predates this contract samples four points and ignores the
  fifth block entirely. No existing block moves, so every existing signal is
  unaffected.
- The addon manifest version advances from 5 to 6, so the beacon manager offers
  the update to operators running the older addon.

## Enforcement

`tests/beacon.rs` parses the addon source embedded in the companion binary
(`beacon::LUA`, via `include_str!`) and asserts that the addon's `NUM_BLOCKS`,
`COMBAT_MARKER`, `COMBAT_IN_RED`, and `COMBAT_OUT_RED` equal the companion's
constants. A divergence between the two languages fails the build rather than
shipping as a silently dead signal.

`tests/pixelbus.rs` asserts every green appearing at a block center is pairwise
separated by more than the default tolerance, so a later slice cannot introduce a
colliding marker without the suite naming the collision.
