# Contract: Skill-Cooldown Blocks (B10 to B15)

**Feature**: [spec.md](../spec.md) | **Date**: 2026-07-27

The wire contract between the PixelBeacon addon and the ESO Weave companion for
block indices 10 through 15. Both sides implement it independently and
`tests/beacon.rs` proves they agree by parsing the addon source embedded in the
companion binary.

## Positions

| Block | Slot | Grid position | Sample point |
| --- | --- | --- | --- |
| B10 | Skill 1 | column 10, row 0 | `block_center(block_px, 10)` |
| B11 | Skill 2 | column 11, row 0 | `block_center(block_px, 11)` |
| B12 | Skill 3 | column 12, row 0 | `block_center(block_px, 12)` |
| B13 | Skill 4 | column 13, row 0 | `block_center(block_px, 13)` |
| B14 | Skill 5 | column 14, row 0 | `block_center(block_px, 14)` |
| B15 | Ultimate | column 15, row 0 | `block_center(block_px, 15)` |

Introduced in addon version 11. Positions are derived on both sides from the
block index, the configured block size, and the shared column count.

**Synergy has no block.** It is not an action slot and the game exposes no
cooldown for it.

## Encoding

One pixel per block, three channels.

| Channel | Carries | Rule |
| --- | --- | --- |
| Green | Validity mark | Per-block constant, below |
| Red | Quantized remaining time | `min(remaining_ms / 50, 254)`, or `0xFF` |
| Blue | Checksum | `255 - red` |

### Marks

| Block | Mark | Distance to nearest other block-center green |
| --- | --- | --- |
| B10 | `0x0B` | 11 |
| B11 | `0x21` | 11 |
| B12 | `0x4E` | 11 |
| B13 | `0x92` | 18 |
| B14 | `0xC6` | 11 |
| B15 | `0xE8` | 22 |

Minimum separation across the whole registry is 11, which is 5.5 times the
default tolerance of 2. All six are registered in `BLOCK_CENTER_GREENS`, so
`tests/pixelbus.rs` proves the separation and names any future collision.

### Payload

| Red | Meaning |
| --- | --- |
| `0x00` | Ready |
| `0x01` to `0xFE` | `red * 50` milliseconds remaining, from 50 ms to 12700 ms |
| `0xFF` | Unavailable: the slot is empty, or the game reports no cooldown |

Values beyond 12700 ms saturate at `0xFE` rather than wrapping, so a long
cooldown reads as "at least 12.7 seconds" rather than as a small number.

## Decoding

Applied in order per block. Any failure yields unavailable; there is no nearest
match and no default to ready.

1. Green within `tolerance` of that block's mark, else unavailable.
2. `red + blue` within `tolerance` of 255, else unavailable.
3. `red == 0` gives ready; `0x01..=0xFE` gives `red * 50` ms; `0xFF` gives
   unavailable.

`tolerance` is `ReaderConfig::tolerance`, default 2.

## Rendering discipline

- All six blocks are drawn whenever the status block is drawn, and hidden only
  when it is. None is ever hidden to express a state, so absence means exactly
  one thing: the installed addon predates version 11.
- The addon redraws a block only when its value changes, so a steady state
  produces a steady signal.
- Values are read on the existing tick and re-baselined from the game on
  `EVENT_PLAYER_ACTIVATED`, because no event fires for a cooldown already running
  when the world finishes loading.
- Slot indices derive from the game's own named action-bar constants rather than
  hardcoded integers.

## Grid impact

The block count becomes 16 against a column count of 16: the grid fills exactly
one row and the captured region is `(block_px * 16, block_px)`. This is the
single-row maximum. The next block added anywhere in this family wraps onto row
1, and the compile-time assertion in `tests/pixelbus.rs` asserting
`NUM_BLOCKS <= COLUMNS` is deliberately left in force to say so.

## Compatibility

| Case | Behavior |
| --- | --- |
| Addon older than version 11 | No blocks at 10 through 15. Whatever is behind the overlay fails the mark or checksum check. All six report unavailable and no event is emitted. |
| Companion older than the addon | The blocks are drawn but never sampled. No existing signal is disturbed. |
| Beacon alive, a block stops decoding | That slot clears to unavailable on that sample rather than holding. |
| Beacon signal lost | All six clear to unavailable through the existing signal-loss path. |
| A colour valid for one slot appears at another slot's position | Fails that block's marker check and reads unavailable, so a geometry error cannot report one slot's cooldown as another's. |
