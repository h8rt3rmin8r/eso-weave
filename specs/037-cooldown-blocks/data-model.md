# Phase 1 Data Model: PixelBeacon Skill-Cooldown Blocks

**Feature**: [spec.md](spec.md) | **Date**: 2026-07-27

## Entities

### SlotCooldown

One slot's decoded cooldown, mirroring `ResourceLevel`.

| Variant | Meaning |
| --- | --- |
| `Unknown` | The companion could not read the value, or the game reports none. The `Default`. |
| `Ready` | The slot is off cooldown and can be used. |
| `RemainingMs(u16)` | Milliseconds left, quantized to 50 ms steps, saturating at 12700. |

Derives `Debug, Clone, Copy, PartialEq, Eq, Default` with `Unknown` as
`#[default]`.

`Ready` is a distinct variant rather than `RemainingMs(0)` so that "usable now"
cannot be confused with "a duration that happens to round to zero", and so a
consumer can match on readiness without a comparison.

### CooldownSet

The six slots as one value, so they travel and are stored together, mirroring
`ResourceSet`.

| Field | Type | Slot |
| --- | --- | --- |
| `skill_1` .. `skill_5` | `SlotCooldown` | The five normal action slots |
| `ultimate` | `SlotCooldown` | The ultimate slot |

Synergy has no field: it is not an action slot and the game exposes no cooldown
for it (see [research.md](research.md) R1).

### CooldownView

The normalized per-slot interface view, one per skill row, mirroring
`CombatView` and `MovementView`.

| Field | Type | Meaning |
| --- | --- | --- |
| `text` | `String` | `"Ready"`, a duration such as `"1.2s"`, or `"-"` when unknown |
| `role` | `StatusRole` | `Active` when ready, `Warning` while counting down, `Muted` when unknown |

The Synergy row's view is always the unknown case, rendered muted, because that
row has no block behind it.

### BlockSamples

Six additional `Option<Rgb>` fields, populated from six new `ReaderConfig` point
helpers. The struct already carries ten; six more need no signature change, which
is the property slice 031 built the seam for.

### PixelBusEvent::Cooldowns

`Cooldowns(CooldownSet)`. Emitted only when the set changes, and with a fully
unknown set when the signal is lost. Mirrors `PixelBusEvent::Resources`.

## State transitions

The reader holds `cooldowns: CooldownSet`, initialized to all `Unknown`. On each
`observe`:

| Current | Decoded set | Next | Event |
| --- | --- | --- | --- |
| any | equal | unchanged | none |
| any | different in any slot | the decoded set | `Cooldowns(new)` |
| any non-unknown | any slot fails to decode | that slot becomes `Unknown` | `Cooldowns(new)` if the set changed |
| any non-unknown | signal lost | all `Unknown` | `Cooldowns(all unknown)` |
| all `Unknown` | signal lost | all `Unknown` | none |

Clearing on non-decode rather than holding follows the decision recorded for the
combat block and inherited by every block since.

## Validation rules

Applied per block by `decode_cooldown`, mirroring `decode_resource`:

1. **Marker**: green within tolerance of that slot's mark. Fail yields `Unknown`.
2. **Checksum**: `red + blue` within tolerance of 255. Fail yields `Unknown`.
3. **Range**: `red == 0` yields `Ready`; `red` in `1..=254` yields
   `RemainingMs(red * 50)`; `red == 255` yields `Unknown`.

There is no nearest match and no default to `Ready`. Every failure lands on
`Unknown`, which is what makes an addon that draws no cooldown blocks, and any
unrelated screen content behind those points, unreadable as a cooldown.

## Relationships and invariants

- **Independence**: each block carries its own mark, so a colour valid for one
  slot fails the marker check at every other slot's position. This is what makes
  a geometry error off by one block loud rather than silent, and it is asserted
  as SC-007.
- **Grid position**: derived. Block `i` sits at `block_center(block_px, i)` for
  `i` in 10..=15, resolving through `grid_position(i, COLUMNS)` to row 0, columns
  10 through 15.
- **Extent**: `NUM_BLOCKS` becomes 16 against `COLUMNS` of 16, so `grid_rows` is
  1 and the captured region is `(block_px * 16, block_px)`: exactly one full row.
  One more block wraps, and the compile-time assertion guarding that is left in
  force.
- **Cross-language agreement**: `NUM_BLOCKS`, the six marks, the quantization
  step, the maximum, and the unavailable sentinel are each stated once per side
  and proven equal by `tests/beacon.rs` parsing the embedded addon source.
