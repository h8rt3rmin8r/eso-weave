# Data Model: Fishing Interaction Detection Rewrite

No persisted data changes. The model is the addon's in-memory fishing state
machine and the controller's log map.

## Addon fishing state machine

State variable `fishingState`, unchanged values: `"idle"`, `"waiting"`,
`"bite"`. Rendered to block B1 exactly as today: waiting = #0080FF, bite =
#00FF00, idle = hidden. Colors, positions, and geometry are contract-frozen
(spec FR-007).

Inputs, sampled or received:

- `interacting`: `GetInteractionType() == INTERACTION_FISH`, sampled by the
  100 ms fishing tick.
- `reelPrompt`: first return of `GetGameCameraInteractableActionInfo()`
  equals `GetString(SI_GAMECAMERAACTIONTYPE17)`, sampled by the same tick
  only while `interacting` is true.
- `baitConsumed`: `EVENT_INVENTORY_SINGLE_SLOT_UPDATE` with `isNewItem`
  false, `stackCountChange == -1`, `itemSoundCategory ==
  ITEM_SOUND_CATEGORY_LURE`, no menu open, and `fishingState ~= "idle"`.
- `itemGained`: `EVENT_INVENTORY_SINGLE_SLOT_UPDATE` with `isNewItem` true.
- `biteTimeout`: the existing 5000 ms one-shot safety timer armed on bite.
- `chatterEnd`: `EVENT_CHATTER_END` (cleanup only, not load-bearing).

Transitions (evaluated in this order; the tick is authoritative):

| From | Trigger | To | Notes |
| --- | --- | --- | --- |
| idle | tick: `interacting` true | waiting | render within one tick (FR-001, FR-006) |
| waiting | tick: `reelPrompt` true | bite | primary bite signal (FR-003); arms safety timer |
| waiting | `baitConsumed` | bite | secondary bite signal (FR-004); arms safety timer |
| bite | tick: `interacting` true, `reelPrompt` whatever | bite | poll never demotes a bite (FR-005) |
| bite | `itemGained` | waiting | catch resolved; timer cleared |
| bite | `biteTimeout` | waiting | safety net unchanged |
| waiting or bite | tick: `interacting` false | idle | interaction ended (FR-006); timer cleared |
| any | `chatterEnd` | idle | redundant cleanup; timer cleared |

Removed input: `EVENT_CLIENT_INTERACT_RESULT` (FR-002). Removed constant:
the unused `ADDON_VERSION` local (FR-008).

## Controller log map (Rust, target `eso_weave::fishing`, level DEBUG)

No state, field, or event changes; one log line per existing transition:

| Site | Line content |
| --- | --- |
| `cast()` | cast keypress sent; armed with the arm deadline in ms |
| `on_event(FishingStarted)` from Armed/Recast | cast detected; waiting for bite |
| `on_event(BiteDetected)` from Waiting | bite detected; reeling after reel delay |
| `tick()` reel deadline | reel keypress sent; recast after recast delay |
| `tick()` recast deadline | recasting |
| `on_event(FishingStopped)` from Waiting | cast ended without bite; recasting |
| `disable(reason)` | fishing disabled with the `StopReason` variant |
| `set_enabled(true)` | fishing enabled |

Constraint: log-only; every existing test in `tests/fishing.rs` and
`tests/pixelbus.rs` passes without modification (FR-010).

## Manifest

`addon/PixelBeacon/PixelBeacon.txt`: `## Version: 3` and `## AddOnVersion: 3`
advance to 4 (FR-008). The managed-marker line is untouched (FR-012).
