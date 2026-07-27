# Contract: Pixel Bus Blocks B6 to B8, Resources

**Feature**: 033-resource-blocks | **Date**: 2026-07-27

Wire contract between the PixelBeacon addon (Lua) and the ESO Weave pixel-bus
reader (Rust). Both sides implement it independently; agreement is asserted by
test, following the pattern established in slice 031.

## Geometry

| Block | Index | Resource | Sample point |
| --- | --- | --- | --- |
| B6 | 6 | Health | `block_center(block_px, 6)` |
| B7 | 7 | Stamina | `block_center(block_px, 7)` |
| B8 | 8 | Magicka | `block_center(block_px, 8)` |

Block count after this feature: **9**. Stated once per side (`local NUM_BLOCKS` in
the addon, `pixelbus::NUM_BLOCKS` in the companion) with the cross-language check
extended to the new markers.

## Encoding

All three blocks share one shape and one decoder.

| Channel | Meaning |
| --- | --- |
| Green | Validity marker, one per resource |
| Red | Payload: the percentage, 0 to 100, or `0xFF` for unavailable |
| Blue | Complement checksum, `255 - red` |

| Resource | Marker |
| --- | --- |
| Health | `0x16` |
| Stamina | `0x6D` |
| Magicka | `0xBB` |

The percentage is of the resource's **current** maximum, computed at publication.

## Decoding

A sample yields a percentage only if all three hold, each within tolerance:

1. Green matches that resource's marker.
2. `red + blue` equals `255`.
3. Red is at most `100 + tolerance`.

The decoded percentage is then `min(red, 100)`.

The allowance above 100 matters more than it looks. A full resource publishes 100,
and full is the normal out-of-combat state; without the allowance any upward
capture drift would push the payload to 101 or 102, fail the range check, and flap
the readout to unavailable exactly when the value is least interesting and most
common. Clamping keeps the error within tolerance (a published 99 read as 101 still
decodes to 100, one off) while making the top of the range as stable as the middle.

Otherwise the result is unavailable. Red at `0xFF` is the addon's explicit
unavailable value; it passes the marker and checksum checks and fails the range
check like any other out-of-range payload, so it needs no special case and cannot
be confused with a clamped 100.

## Guarantees

- **Bounded error.** For any sample perturbed within tolerance, decoding yields
  either a percentage within tolerance of the published one, or unavailable. It
  never yields a different percentage. This is the property that justifies choosing
  a numeric payload over a colour lookup table, whose error is unbounded.
- **Monotonic.** If one published percentage exceeds another, the decoded values
  preserve that ordering.
- **Independent.** Each block decodes on its own; one bad sample does not affect
  the other two.
- **Rejection is safe.** A sample whose payload and checksum both drift the same
  way sums to twice the tolerance and is rejected rather than decoded. That is
  correct: unavailable is always safe, a wrong percentage is not.

## Marker separation

Against the default tolerance of 2. Greens on the strip after this feature:
`0x00`, `0x16`, `0x2D`, `0x5A`, `0x6D`, `0x80`, `0xA5`, `0xBB`, `0xD2`, `0xFF`.

| Marker | Nearest neighbour | Distance | Margin |
| --- | --- | --- | --- |
| `0x16` health | `0x00` | 22 | 11x |
| `0x6D` stamina | `0x80` | 19 | 9.5x |
| `0xBB` magicka | `0xA5` | 22 | 11x |

Ten markers now occupy the channel. A future block has roughly 19 of headroom;
`tests/pixelbus.rs` enforces the separation rule and will name any collision.

## Producer rules

- Published from `GetUnitPower("player", COMBAT_MECHANIC_FLAGS_*)`, driven by
  `EVENT_POWER_UPDATE` with a re-read on the fast tick as a backstop.
- A maximum that is zero or unreadable publishes `0xFF`, never a percentage.
- Rendered change-detected, and never hidden to express a state.

## Consumer rules

- A decoded change is announced once; a steady resource announces nothing.
- Each resource clears to unavailable on signal loss and on any sample that does
  not decode.
- Changes are logged at **trace** level, not debug: unlike every other block, these
  change many times a second and would otherwise bury the live log.
- Values are stored and displayed. Nothing acts on them.

## Compatibility

An addon predating this contract draws six blocks. The companion samples B6 to B8,
fails validation, and reports all three unavailable. Manifest advances 7 to 8.
