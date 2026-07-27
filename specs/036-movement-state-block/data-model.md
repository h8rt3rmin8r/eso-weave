# Phase 1 Data Model: PixelBeacon Movement-State Block

**Feature**: [spec.md](spec.md) | **Date**: 2026-07-27

## Entities

### MovementSignal

The decoded movement state, as the companion sees it. A tri-state, mirroring
`CombatSignal`.

| Variant | Meaning | Notes |
| --- | --- | --- |
| `Unknown` | The companion could not read the signal | The `Default`. Not a game state. Produced by an absent block, a failed marker or checksum, an unrecognized code, and signal loss. |
| `OnFoot` | The player is not mounted | A real game state. Must stay distinguishable from `Unknown` (Spec §Key Entities). |
| `Mounted` | The player is mounted | A real game state. |

Derives `Debug, Clone, Copy, PartialEq, Eq, Default` with `Unknown` as
`#[default]`, matching `CombatSignal` exactly.

The sprint axis has no variant. It is reserved in the wire encoding only (see
[contracts/movement-block.md](contracts/movement-block.md)); adding it later adds
variants here without changing the meaning or the color of either existing one.

### MovementView

The normalized interface view, mirroring `CombatView`.

| Field | Type | Meaning |
| --- | --- | --- |
| `detected` | `bool` | `signal != MovementSignal::Unknown` |
| `state` | `&'static str` | `"Mounted"`, `"On foot"`, or `"Not detected"` |
| `role` | `StatusRole` | `Active` when detected, `Muted` otherwise |

The wording and the role mapping are taken from `combat_view` rather than
invented, so the movement, combat, and weapon-bar fields read under one
convention (Plan §D4).

### BlockSamples.movement

One additional `Rgb` field on the existing sample set, populated from
`ReaderConfig::movement_point()`. The struct already carries nine such fields; a
tenth needs no signature change, which is the property slice 031 built the seam
for.

### PixelBusEvent::Movement

`Movement(MovementSignal)`. Emitted only on a decoded change, and with
`MovementSignal::Unknown` when the signal is lost. Mirrors
`PixelBusEvent::Combat`.

## State transitions

The reader holds `movement: MovementSignal`, initialized to `Unknown`. On each
`observe`:

| Current | Sample decodes to | Next | Event |
| --- | --- | --- | --- |
| any | same value | unchanged | none |
| any | different value | the decoded value | `Movement(new)` |
| any non-`Unknown` | fails to decode | `Unknown` | `Movement(Unknown)` |
| any non-`Unknown` | signal lost | `Unknown` | `Movement(Unknown)` |
| `Unknown` | fails to decode or signal lost | `Unknown` | none |

The third row is the deliberate divergence from the weapon block, which holds its
last value while the beacon is alive. Movement follows combat and clears, because
`Unknown` is defined as "the companion could not read the signal" and holding
would make that definition false (Spec §FR-008, and the dated decision recorded
for slice 031 that this slice inherits).

## Validation rules

Applied in order by `decode_movement`, mirroring `decode_combat`:

1. **Marker**: `green` within tolerance of `0x43`. Fail yields `Unknown`.
2. **Checksum**: `red + blue` within tolerance of 255. Fail yields `Unknown`.
3. **Code**: `red` within tolerance of `0x20` yields `OnFoot`; within tolerance
   of `0x60` yields `Mounted`. Anything else, including the reserved sprint codes
   `0xA0` and `0xE0`, yields `Unknown`.

There is no nearest match and no default to `OnFoot`. Every failure path lands on
`Unknown`, which is what makes an addon that draws no tenth block, and any
unrelated screen content behind that point, unreadable as a state rather than
readable as the wrong one.

## Relationships and invariants

- **Independence**: the movement block shares no channel, position, or state with
  any other block. No combination of combat, menu, fishing, or resource states
  can alter a movement decode (Spec §Edge Cases).
- **Grid position**: derived, never restated. The block's center is
  `block_center(9, block_px)`, which resolves through `grid_position(9, COLUMNS)`
  to row 0, column 9.
- **Extent**: `grid_extent` is one row tall whenever `NUM_BLOCKS <= COLUMNS`.
  This slice takes the count from 9 to 10 against a column count of 16, so the
  captured region is unchanged. The invariant is asserted on the dependency, not
  on the number ten (Plan §D5).
- **Cross-language agreement**: `NUM_BLOCKS`, `COLUMNS`, `MOVEMENT_MARKER`,
  `MOVEMENT_ON_FOOT_RED`, and `MOVEMENT_MOUNTED_RED` are each stated once per
  side and proven equal by `tests/beacon.rs` parsing the embedded addon source.
  The reserved sprint codes are companion-only by design and are excluded from
  that check (Research §R3).
