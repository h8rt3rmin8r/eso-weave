# Data Model: PixelBeacon In-Combat State Block

**Feature**: 031-combat-state-block | **Date**: 2026-07-27

Phase 1 output. Entities are in-memory only; nothing here is persisted, and the
constitution forbids writing runtime state to the config file.

## CombatSignal

The decoded combat state. Three variants, matching FR-002.

| Variant | Meaning | Wire encoding |
| --- | --- | --- |
| `Unknown` | The companion could not read the signal | no valid sample |
| `OutOfCombat` | The game reports the player is not in combat | red `0x20` |
| `InCombat` | The game reports the player is in combat | red `0xE0` |

**Validation**: produced only by `decode_combat`, which returns `Unknown` for any
sample failing the marker, checksum, or state-code check. There is no conversion
from an arbitrary integer, so an out-of-range value cannot be represented.

**Default**: `Unknown`. A reader that has never decoded the block reports
unavailable, never out of combat.

**State transitions**: any variant to any variant. Every transition is announced
once; a repeat of the current variant is not.

```text
                +-----------------+
                |     Unknown     |<--- signal lost, or block does not decode
                +-----------------+
                   ^           |
   block stops     |           |  block decodes
   decoding        |           v
          +-------------+   +-------------+
          | OutOfCombat |<->|  InCombat   |
          +-------------+   +-------------+
                 game combat transition
```

## BlockSamples

The set of raw samples taken from one strip read, replacing the four positional
arguments of `observe`.

| Field | Type | Block |
| --- | --- | --- |
| `status` | `Option<Rgb>` | B0 |
| `fishing` | `Option<Rgb>` | B1 |
| `latency` | `Option<Rgb>` | B2 |
| `weapon` | `Option<Rgb>` | B3 |
| `combat` | `Option<Rgb>` | B4 |

`None` means the surface could not be sampled at that point (the game window is
gone, or the point is outside the captured region). It is distinct from a sample
that was taken but does not decode.

**Default**: every field `None`. This is what makes the struct extensible: a
construction using `..Default::default()` keeps compiling when a later slice adds
a field, which is FR-014.

## PixelBusEvent::Combat

A new variant carrying a `CombatSignal`, emitted only when the decoded state
changes.

**Relationships**: routed by `route_reader_event` to the weave engine's combat
store, exactly as `WeaponBar` is routed to its bar and class store. Unlike the
fishing events, it is not mapped into a detector event; `map_event` returns
`None` for it, so it never reaches the fishing controller.

## CombatView

The derived display state for the interface, mirroring `WeaponBarView`.

| Field | Type | Derivation |
| --- | --- | --- |
| `detected` | `bool` | `signal != CombatSignal::Unknown` |
| `state` | `&'static str` | "In combat", "Out of combat", or "Not detected" |
| `role` | `StatusRole` | `Active` when detected, `Muted` otherwise |

**Validation**: a pure function of one `CombatSignal`, with no other input, so it
is fully testable by enumerating the three variants.

## Reader state

`PixelBusReader` gains one field.

| Field | Type | Default | Cleared when |
| --- | --- | --- | --- |
| `combat` | `CombatSignal` | `Unknown` | signal lost, or the block does not decode |

Note the difference from the neighbouring `weapon: Option<WeaponBarSignal>`
field, which is cleared only on signal loss and otherwise holds its last decoded
value. Combat state does not hold. The divergence is deliberate and is recorded
in the spec's clarification session and in the contract.

## Constants

Shared byte for byte between `addon/PixelBeacon/PixelBeacon.lua` and
`src/pixelbus/mod.rs`, with agreement asserted by test.

| Name | Value | Purpose |
| --- | --- | --- |
| `NUM_BLOCKS` | `5` | Blocks on the strip; drives the drawn width and the captured region |
| `COMBAT_MARKER` | `0x2D` | Green validity marker for B4 |
| `COMBAT_IN_RED` | `0xE0` | Red channel, in combat |
| `COMBAT_OUT_RED` | `0x20` | Red channel, out of combat |

## Block-center green registry

A documented constant in `src/pixelbus/mod.rs` naming every green that appears at
a block center, so a later slice picks a non-colliding marker by adding to a list
the tests check rather than by rediscovering the set.

| Block | Green | Note |
| --- | --- | --- |
| B0 status | `0x00` | part of the magenta status color |
| B1 fishing, waiting | `0x80` | |
| B1 fishing, bite | `0xFF` | |
| B2 latency | `0xA5` | marker |
| B3 weapon | `0x5A` | marker |
| B4 combat | `0x2D` | marker, added by this feature |

**Invariant**: every pair in this list differs by more than the default reader
tolerance. Asserted in `tests/pixelbus.rs`.
