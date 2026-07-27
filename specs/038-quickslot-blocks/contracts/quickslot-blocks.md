# Contract: Quickslot-State Blocks B16 to B19

**Feature**: [../spec.md](../spec.md) | **Date**: 2026-07-27

This is the byte-for-byte contract between `addon/PixelBeacon/PixelBeacon.lua`
and `src/pixelbus/mod.rs`. Both sides state every value below exactly once, and
`tests/beacon.rs` proves they agree by parsing the embedded addon source. A
disagreement fails the build rather than reaching a release.

## Grid position

`NUM_BLOCKS` becomes 20. `COLUMNS` stays 16.

| Block | Index | Column | Row | Carries |
| --- | --- | --- | --- | --- |
| B16 | 16 | 0 | 1 | quickslot cooldown, or unavailable |
| B17 | 17 | 1 | 1 | identity, bits 23..16 |
| B18 | 18 | 2 | 1 | identity, bits 15..8 |
| B19 | 19 | 3 | 1 | identity, bits 7..0 |

These are the first four blocks on row 1. Every block from B0 to B15 keeps the
position it had; row 0 is full and unchanged.

Sample point for index `i`, unchanged from the shared rule:

```
col = i % COLUMNS
row = i / COLUMNS          (integer division)
x   = block_px * col + block_px / 2
y   = block_px * row + block_px / 2
```

Capture region: `block_px * 16` wide by `block_px * 2` tall. At the default
square size that is 256 by 32 physical pixels.

## B16, quickslot cooldown

| Channel | Carries |
| --- | --- |
| red | step count, or `255` for unavailable |
| green | `QUICKSLOT_MARKER` = `0x38` |
| blue | `255 - red` (complement checksum) |

The step count uses the slice 037 scheme unchanged and the same three constants:

```
COOLDOWN_STEP_MS   = 50
COOLDOWN_MAX_STEPS = 254
COOLDOWN_UNAVAILABLE = 255

steps = 0                              when the slot is ready
      = clamp(round(remaining_ms / 50), 1, 254)   otherwise
      = 255                            when unavailable
```

`red = 0` decodes as `Ready`. `red` in `1..=254` decodes as
`RemainingMs(red * 50)`, so the encodable range is 50 ms to 12700 ms and a longer
cooldown saturates at the maximum rather than wrapping. `red = 255` decodes as
`Unknown`.

**Unavailable is published, not withheld.** The block is drawn whenever the addon
is loaded and rendering. It covers, indistinguishably and deliberately: an empty
quickslot, a quickslot holding a non-potion item, a potion with no on-use ability,
a slot the game reports no cooldown for, and (on the reader's side) an absent or
undecodable block.

## B17, B18, B19, the identity

| Channel | Carries |
| --- | --- |
| red | one byte of the identity |
| green | that block's marker |
| blue | `255 - red` (complement checksum) |

| Block | Marker | Byte |
| --- | --- | --- |
| B17 | `QUICKSLOT_ID_HI_MARKER` = `0xB0` | `(id >> 16) & 0xFF` |
| B18 | `QUICKSLOT_ID_MID_MARKER` = `0xDD` | `(id >> 8) & 0xFF` |
| B19 | `QUICKSLOT_ID_LO_MARKER` = `0xF3` | `id & 0xFF` |

Most significant byte first, so the blocks read left to right in the order the
number is written.

`id` is reduced modulo `2^24` by the addon before the shift, so every block
always carries a whole byte. An identity beyond 24 bits aliases; it never
produces an unencodable value, because an unencodable value would fail its
checksum and report the whole quickslot as unknown, which claims there is no
potion rather than that its name is not known.

Every byte value `0..=255` is a legal payload, so these blocks carry no in-band
sentinel. Their validity comes entirely from the marker and the checksum. When
B16 carries the unavailable payload, all three carry `0` and are still drawn.

## Decoding

```
decode_quickslot(b16, b17, b18, b19, tolerance) -> QuickslotState

cooldown = b16 -> decode_cooldown(_, QUICKSLOT_MARKER, tolerance)
                  (absent block -> Unknown)

item_id  = None                      when cooldown == Unknown
         = Some(hi<<16 | mid<<8 | lo) when all three blocks decode
         = None                      otherwise
```

A block decodes when its green is within `tolerance` of its marker and
`red + blue` is within `tolerance` of 255.

**Partial identities are never assembled.** If any one of the three blocks fails,
the identity is absent, not a number built from the two that read. The cooldown
is unaffected: the two halves degrade independently, which is what the interface
shows (FR-012).

## Marker separation

The green registry after this feature, sorted:

```
00 0B 16 21 2D 38 43 4E 5A 6D 80 92 A5 B0 BB C6 D2 DD E8 F3 FF
```

Minimum adjacent separation 11, unchanged by this feature, against a default
reader tolerance of 2. All four new marks are added to `BLOCK_CENTER_GREENS`, so
the existing automated separation check proves this rather than the author
asserting it.

## Cross-language agreement

`tests/beacon.rs` pins, by parsing the embedded Lua:

| Lua constant | Expected |
| --- | --- |
| `NUM_BLOCKS` | `pixelbus::NUM_BLOCKS` (20) |
| `COLUMNS` | `pixelbus::COLUMNS` (16) |
| `QUICKSLOT_MARKER` | `0x38` |
| `QUICKSLOT_ID_HI_MARKER` | `0xB0` |
| `QUICKSLOT_ID_MID_MARKER` | `0xDD` |
| `QUICKSLOT_ID_LO_MARKER` | `0xF3` |

`COOLDOWN_STEP_MS`, `COOLDOWN_MAX_STEPS`, and `COOLDOWN_UNAVAILABLE` are already
pinned by the slice 037 checks and are reused under their existing names on both
sides. They are deliberately not duplicated under a quickslot name.

## Addon manifest

`Version` and `AddOnVersion` advance from 11 to 12, so the beacon manager offers
the update. The description names the new signal.
