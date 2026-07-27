# Data Model: PixelBeacon Resource Blocks

**Feature**: 033-resource-blocks | **Date**: 2026-07-27

In-memory only; nothing persisted.

## ResourceLevel

A decoded resource reading.

| Variant | Meaning |
| --- | --- |
| `Unknown` | The companion could not read it, or the addon could not compute it |
| `Percent(u8)` | A whole percentage, 0 to 100 inclusive |

**Default**: `Unknown`, which is also the value every failure mode produces
(absent block, failed validation, lost signal).

`Unknown` is distinct from `Percent(0)`. Zero means the pool is genuinely empty,
which is a real and important state; unknown means there is no reading. Collapsing
them would make a missing addon look like a dead character.

**Validation**: constructed only by the decoder, which rejects any payload above
100, so an out-of-range percentage cannot be represented.

## ResourceSet

The three resources as one value, so they travel and are stored together.

| Field | Type |
| --- | --- |
| `health` | `ResourceLevel` |
| `stamina` | `ResourceLevel` |
| `magicka` | `ResourceLevel` |

**Default**: all three `Unknown`.

## PixelBusEvent::Resources

Carries a `ResourceSet`, emitted only when the set changes. Emitting the set rather
than three separate events keeps one event per sample at most, which matters
because these change far more often than any other signal. Returns `None` from the
fishing detector mapping.

## BlockSamples

Gains `health`, `stamina`, and `magicka` fields. The derived `Default` means no
existing construction changes, which is the property slice 031 added the struct to
provide and the third slice in a row to benefit from it.

## ResourceView

Display state, one per resource.

| Field | Type | Derivation |
| --- | --- | --- |
| `detected` | `bool` | the level is not `Unknown` |
| `text` | `String` | the percentage with a percent sign, or "Not detected" |
| `role` | `StatusRole` | `Active` when detected, `Muted` otherwise |

## Constants shared byte for byte

| Name | Value |
| --- | --- |
| `NUM_BLOCKS` | `9` |
| `HEALTH_MARKER` | `0x16` |
| `STAMINA_MARKER` | `0x6D` |
| `MAGICKA_MARKER` | `0xBB` |
| `RESOURCE_UNAVAILABLE` | `0xFF` |
