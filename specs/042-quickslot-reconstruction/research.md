# Phase 0 Research: Quickslot Observation Reconstruction

**Feature**: [spec.md](spec.md) | **Date**: 2026-09-03

No NEEDS CLARIFICATION markers remain. Decisions follow the autopilot policy.

## R1: Source baseline and current defect

**Decision**: Target ESO UI 12.0.7, API 101050, and model the selected slot from
the game's independent action-bar primitives.

The current API documents `GetCurrentQuickslot`, `GetSlotType`,
`GetSlotBoundId`, `GetSlotItemLink`, `GetSlotItemCount`, `IsSlotUsable`, and
`GetSlotCooldownInfo` with an explicit hotbar category. The game's current
`ActionButton` similarly reads slot type, bound ID, count, usability failures,
and cooldown independently. Its utility-wheel table permits item, collectible,
quest item, emote, and quick-chat action types.

The existing PixelBeacon implementation instead starts from an item link, adds
an on-use-ability predicate, and returns the same cooldown sentinel for every
failure. The Rust reader then defines potion presence as cooldown not Unknown.
That collapse is the proven model defect.

**Sources**:

- [ESO UI source, live branch](https://github.com/esoui/esoui/tree/live)
- [Action button state](https://github.com/esoui/esoui/blob/live/esoui/ingame/actionbar/actionbutton.lua)
- [Assignable utility wheel](https://github.com/esoui/esoui/blob/live/esoui/ingame/utilitywheel/assignableutilitywheel_shared.lua)
- [API documentation](https://github.com/esoui/esoui/blob/live/ESOUIDocumentation.txt)

## R2: Classification order

**Decision**: Read and retain all raw facts, then classify in this order:

1. unsupported API/category constant
2. missing or invalid selected slot
3. `ACTION_TYPE_NOTHING` as Empty
4. supported non-item action types by bounded kind
5. item action with missing link or internally inconsistent facts as Unavailable
6. non-potion item as Non-potion(Item)
7. potion with zero count as Potion(Depleted)
8. positive-stack potion with `IsSlotUsable == false` as Potion(Blocked)
9. positive-stack usable potion as Potion(Usable)

Cooldown and optional identity are attached facts. Neither may change the class.

**Deviation from slice 038**: Remove `GetItemLinkOnUseAbilityInfo` as a gate.
The official action-button logic uses action type, count, state failures, and
cooldown to determine usability. On-use ability metadata is useful in diagnostics
but is not an official slot-usability predicate. Retaining it as an extra gate
adds a false-negative path with no safety benefit.

## R3: Protocol shape

**Decision**: Keep B16 cooldown and B17 to B19 optional item identity unchanged,
and add B20 as a dedicated classification block. `NUM_BLOCKS` becomes 21 while
`COLUMNS` remains 16.

B20 uses green marker `0x76`, chosen from the widest remaining registry gap
between `0x6D` and `0x80`. It remains at least nine values from any neighboring
marker, over four times the default tolerance of two. Red carries a spaced state
code and blue its complement.

One block is sufficient because the state code combines the bounded non-potion
kind or potion availability. Exact raw count remains diagnostic evidence; the
transport needs only depleted versus positive and usable versus blocked.

**Alternatives rejected**:

- Reserved B16 cooldown payloads continue to couple class to cooldown.
- Multiple new blocks for every raw primitive expand issue #26's geometry burden
  without improving the consumer contract.
- Replacing B16 to B19 would discard a stable cooldown and optional-identity wire
  format unnecessarily.

## R4: Old addon and corrupt data

**Decision**: If B20 is absent while the legacy four blocks exist, decode
Unavailable(LegacyProtocol). If B20 is present but fails marker, checksum, or
state-code validation, decode Unavailable(CorruptProtocol). Signal loss clears to
Unavailable(NoSignal).

Legacy cooldown and identity bytes are never promoted to Potion. This is an
intentional fail-closed compatibility path, not silent compatibility.

## R5: Diagnostic receipt

**Decision**: Add `/pbquickslot` for one snapshot and `/pbquickslot watch` to
toggle change-only receipts. Each receipt is bounded to primitive numeric values
and booleans: selected index, category availability, slot type, bound ID, item
link present, item type, count, on-use ability present, usability, cooldown
remaining/duration/global, final code, and identity. No localized item name,
header, or description is printed.

The snapshot command is the field-proof seam. The headless runner cannot launch
or focus-steal ESO, so it must not claim real-client evidence. Issue #24 remains
open until the operator attaches the required matrix receipt.

## R6: Update convergence

**Decision**: React to `EVENT_ACTIVE_QUICKSLOT_CHANGED`,
`EVENT_ACTION_SLOT_UPDATED`, `EVENT_ACTION_SLOT_STATE_UPDATED`,
`EVENT_ACTION_UPDATE_COOLDOWNS`, `EVENT_INVENTORY_SINGLE_SLOT_UPDATE`, and
`EVENT_PLAYER_ACTIVATED`. Keep the existing one-second poll as recovery and
cooldown countdown backstop.

Every source calls the same compute-then-render-if-changed path. Watch diagnostics
compare a bounded snapshot key, so repeated events and polls do not duplicate
output.

## R7: Automation boundary

**Decision**: Expose truthful `is_potion` and `is_usable` queries on the model,
but do not let S042 authorize input. The existing auto-potion rule remains gated
off for the new explicit protocol until issue #25 adopts it with the runtime,
focus, context, freshness, retry, and observability contract.

This is safer than allowing a bug fix in an observation slice to activate a
feature whose end-to-end safety matrix has not yet been completed.
