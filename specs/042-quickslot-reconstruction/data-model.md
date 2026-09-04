# Data Model: Quickslot Observation Reconstruction

**Feature**: [spec.md](spec.md) | **Date**: 2026-09-03

## QuickslotState

| Field | Type | Rule |
| --- | --- | --- |
| classification | `QuickslotClassification` | Sole authority for what is selected |
| cooldown | `SlotCooldown` | Independent Ready, Remaining, or Unknown fact |
| item_id | `Option<u32>` | Complete identity only, never a safety input |

## QuickslotClassification

- `Unavailable(QuickslotUnavailableReason)`
- `Empty`
- `NonPotion(QuickslotNonPotionKind)`
- `Potion(QuickslotPotionAvailability)`

## QuickslotUnavailableReason

- `NoSignal`: no fresh beacon observation
- `LegacyProtocol`: B16 to B19 exist but B20 does not
- `CorruptProtocol`: B20 marker, checksum, or code is invalid
- `UnsupportedApi`: required ESO constants or calls are absent
- `InvalidSelection`: selected slot is nil or invalid
- `InconsistentFacts`: separately read primitives cannot form a coherent state

## QuickslotNonPotionKind

- `Item`
- `Collectible`
- `QuestItem`
- `Emote`
- `QuickChat`
- `Other`

## QuickslotPotionAvailability

- `Depleted`: stack count is zero
- `Blocked`: positive stack but the slot is not usable
- `Usable`: positive stack and the slot is usable

## Invariants

1. Only an explicit `Potion` classification makes `is_potion()` true.
2. Only `Potion(Usable)` makes `is_usable_potion()` true.
3. Cooldown and item identity cannot change either predicate.
4. Non-potion and unavailable states carry no item identity in the normalized model.
5. Identity is all three bytes or absent.
6. Any loss of freshness clears to `Unavailable(NoSignal)`.
7. S042's application-level consumer gate remains false for every state.

## View model

The main view presents three fields:

- Quickslot: selected classification and bounded reason or kind
- Quickslot availability: Usable, Blocked, Depleted, or Not applicable
- Quickslot cooldown: Ready, remaining duration, or unavailable

The raw numeric identity leaves the main view. It remains in diagnostics because
it is useful for proving a swap but provides little user-facing meaning.
