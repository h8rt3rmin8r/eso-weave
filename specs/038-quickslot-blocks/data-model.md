# Phase 1 Data Model: PixelBeacon Quickslot-State Blocks

**Feature**: [spec.md](spec.md) | **Date**: 2026-07-27

No persisted data. Everything here is runtime state derived from a screen sample
and discarded on exit. The configuration file is untouched (constitution:
"Configuration stores user settings only").

## New types

### `QuickslotState` (`src/pixelbus`)

The whole decoded quickslot, carried as one value and announced as one event.

| Field | Type | Meaning |
| --- | --- | --- |
| `cooldown` | `SlotCooldown` | how long until the slot can be used |
| `item_id` | `Option<u32>` | the slotted item's identity, when known |

`SlotCooldown` is reused unchanged from slice 037: `Unknown`, `Ready`,
`RemainingMs(u16)`. Reusing it rather than defining a parallel type is what makes
the quantization, the saturation rule, and the ready case shared by construction.

**Whether a potion is present is derived, never stored**: it is exactly
`cooldown != SlotCooldown::Unknown`, exposed as `has_potion()`. Storing it as a
third field would admit two states that cannot exist (a potion with no cooldown,
and a cooldown with no potion) and would make the decoder responsible for keeping
two fields consistent forever. See spec Clarifications and FR-007.

**Invariant**: `item_id.is_some()` implies `has_potion()`. The decoder enforces
it by construction; a test pins it.

`new_unknown()` yields `{ cooldown: Unknown, item_id: None }`, which is both the
initial state and the cleared state.

### `PixelBusEvent::Quickslot(QuickslotState)`

One variant carrying the whole state, following `Resources` and `Cooldowns`. A
swap changes the identity and the cooldown in the same sample; four events for
one swap would be four log lines for one thing happening.

### `QuickslotView` (`src/app`)

The interface projection, in the status region.

| Field | Type | Meaning |
| --- | --- | --- |
| `cooldown` | `CooldownView` | the cooldown text and its status role |
| `identity` | `CooldownView` | the identity text and its status role |

Two independently-degrading halves rather than one string, because the partial
state (cooldown decoded, identity not) is reachable whenever exactly one identity
block is disturbed, and collapsing it would discard a correctly read value and
make a one-block disturbance look identical to a missing addon (FR-012).

`CooldownView` is reused rather than a new pair of types; it is already exactly a
display string plus a status role, and the muted-when-unknown treatment is
already what its `Unknown` case renders.

## Extended types

### `BlockSamples`

Four new fields, following the existing one-per-block pattern:
`quickslot_status` (B16), `quickslot_id_hi` (B17), `quickslot_id_mid` (B18),
`quickslot_id_lo` (B19), each `Option<Rgb>`.

### `ReaderConfig`

Four new derived sample points: `quickslot_status_point()`,
`quickslot_id_hi_point()`, `quickslot_id_mid_point()`,
`quickslot_id_lo_point()`, each `block_center(self.block_px, N)` for N in 16..20.
Derived, never stored, so the reader and the addon cannot disagree. These are the
first sample points in the project's history whose `y` is not `block_px / 2`.

### `PixelBusReader`

One new field, `quickslot: QuickslotState`, cleared to `new_unknown()` on signal
loss alongside combat, menu, resources, movement, and cooldowns.

### `WeaveEngine`

`set_quickslot` and `quickslot()`, following `set_cooldowns`/`cooldowns()`
exactly: stored, read by the view model, and acted on by nothing. The existing
test asserting the engine behaves identically for every value of every stored
signal is extended to cover this one, so the inertness is enforced rather than
intended.

### `BLOCK_CENTER_GREENS`

Grows from 17 entries to 21. Adding the four marks here is what makes the
separation check prove them rather than the author asserting them.

### `NUM_BLOCKS`

16 to 20, stated once on each side of the contract.

## Constants

| Name | Value | Side |
| --- | --- | --- |
| `QUICKSLOT_MARKER` | `0x38` | both |
| `QUICKSLOT_ID_HI_MARKER` | `0xB0` | both |
| `QUICKSLOT_ID_MID_MARKER` | `0xDD` | both |
| `QUICKSLOT_ID_LO_MARKER` | `0xF3` | both |
| `COOLDOWN_STEP_MS` | `50` | reused, unchanged |
| `COOLDOWN_MAX_STEPS` | `254` | reused, unchanged |
| `COOLDOWN_UNAVAILABLE` | `255` | reused, unchanged |

The three reused constants are not redefined under a quickslot name. The
cross-language check already pins them, and a second name for the same number is
how two numbers eventually become different.

## Lifecycle

```
addon draws B16..B19  ->  sampler reads 4 points  ->  decode_quickslot
  ->  changed?  ->  PixelBusEvent::Quickslot  ->  WeaveEngine::set_quickslot
  ->  view model  ->  two status rows
```

Signal loss short-circuits to `new_unknown()`. A present-but-undecodable block
clears rather than holds, following the combat block's recorded decision, because
a stale "there is a ready potion" surviving an addon downgrade is exactly the
false reading the consumer one slice away would act on.
