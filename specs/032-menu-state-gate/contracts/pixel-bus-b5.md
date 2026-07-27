# Contract: Pixel Bus Block B5, Menu State

**Feature**: 032-menu-state-gate | **Date**: 2026-07-27

The wire contract between the PixelBeacon addon (producer, Lua) and the ESO Weave
pixel-bus reader (consumer, Rust). Both sides implement it independently and their
agreement is asserted by test, following the pattern slice 031 established.

## Geometry

| Property | Value |
| --- | --- |
| Block index | 5 (the sixth block) |
| Block count after this feature | 6 |
| Span, physical pixels | `[BLOCK_PX * 5, BLOCK_PX * 6)` horizontally, `[0, BLOCK_PX)` vertically |
| Sample point | `block_center(block_px, 5)` |

Stated once per side: `local NUM_BLOCKS = 6` in the addon,
`pub const NUM_BLOCKS: u32 = 6` in the companion, with the cross-language agreement
test extended to this block's constants.

## Encoding

| Channel | Meaning |
| --- | --- |
| Green | Validity marker, constant `0xD2` |
| Red | Surface code, `code * 24` |
| Blue | Complement checksum, `255 - red` |

| Code | Red | Blue | Surface |
| --- | --- | --- | --- |
| 0 | 0 | 255 | None, gameplay |
| 1 | 24 | 231 | System menu |
| 2 | 48 | 207 | Map |
| 3 | 72 | 183 | Inventory |
| 4 | 96 | 159 | Mail |
| 5 | 120 | 135 | Character and skills |
| 6 | 144 | 111 | Guild store |
| 7 | 168 | 87 | Crown store |
| 8 | 192 | 63 | Journal |
| 9 | 216 | 39 | Chat entry |
| 10 | 240 | 15 | Other, unenumerated |

**The gate is `code != 0`.** Any code from 1 to 10 means a surface is active.

## Decoding

A sample yields a surface only if all three hold within the reader's tolerance:

1. Green matches `0xD2`.
2. `red + blue` equals `255`.
3. Red is within tolerance of one of the eleven code values.

Any failure yields the unavailable result, which the gate treats as **inactive**.
There is no nearest-match and no default to a surface.

## Separation guarantees

Against the default tolerance of 2.

| Pair | Distance | Margin |
| --- | --- | --- |
| Marker `0xD2` vs nearest other block-center green (`0xA5` latency) | 45 | 22x |
| Marker `0xD2` vs combat marker `0x2D` | 165 | 82x |
| Adjacent surface codes | 24 | 12x |

## Producer rules

- The active/inactive boolean is derived from `IsGameCameraUIModeActive()` ORed
  with `ZO_GetChatSystem():IsTextEntryOpen()`. It is **never** derived from which
  gameplay scene is showing, because that test does not cover chat entry.
- The boolean is decided **before** the label. Only once a surface is known active
  does the addon map the current scene to a code, falling back to code 10. A scene
  name that is wrong or unenumerated therefore degrades to "other", never to
  "none".
- Published on the addon's fast tick, not its one-second tick, so end-to-end
  latency is bounded by two fast intervals.
- Rendered change-detected, and never hidden to express a state.

## Consumer rules

- A decoded change is announced once; a steady state announces nothing.
- The gate clears to inactive on signal loss and on any sample that does not
  decode. Inactive is the safe value: it reproduces the application's behavior
  without this feature.
- While the gate is active, the application starts no new weave sequence and no new
  fishing interaction. Work already in progress completes.
- The gate can only cause a key to pass through. It can never cause a suppression.

## Compatibility

- An addon predating this contract draws five blocks. The companion samples the B5
  point, fails validation, and holds the gate inactive, so interception behaves
  exactly as it does today. Required test case.
- A companion predating this contract samples five points and ignores the sixth
  block. No existing block moves.
- Manifest advances from version 6 to 7.
