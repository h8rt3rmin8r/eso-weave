# Data Model: Fishing Bite Signal Correction

No persisted data changes; no Rust state changes. The model is the corrected
addon fishing state machine (delta from slice 025's data-model.md).

## Corrected addon fishing state machine

State variable `fishingState` unchanged: `"idle"`, `"waiting"`, `"bite"`.
Rendering unchanged (waiting #0080FF, bite #00FF00, idle hidden).

Inputs after the correction:

- `interacting`: `GetInteractionType() == INTERACTION_FISH`, sampled by the
  100 ms fishing tick. (Unchanged.)
- `baitConsumed`: `EVENT_INVENTORY_SINGLE_SLOT_UPDATE` with `isNewItem`
  false, `stackCountChange == -1`, `itemSoundCategory ==
  ITEM_SOUND_CATEGORY_LURE`, no menu open, `fishingState ~= "idle"`.
  (Unchanged; now the SOLE bite input.)
- `itemGained`, `biteTimeout`, `chatterEnd`: unchanged clearing inputs.

REMOVED input: `reelPrompt` (the `GetGameCameraInteractableActionInfo()`
comparison against `GetString(SI_GAMECAMERAACTIONTYPE17)`). The prompt is
the standing cast prompt and appears nowhere in the addon after this slice.

Transitions:

| From | Trigger | To | Notes |
| --- | --- | --- | --- |
| idle | tick: `interacting` true | waiting | unchanged |
| waiting | `baitConsumed` | bite | the sole bite transition (FR-002) |
| bite | tick (any prompt state) | bite | poll never demotes (unchanged) |
| bite | `itemGained` | waiting | catch resolved; timer cleared |
| bite | `biteTimeout` | waiting | safety net unchanged |
| waiting or bite | tick: `interacting` false | idle | unchanged |
| any | `chatterEnd` | idle | cleanup, unchanged |

Waiting persists indefinitely with no synthetic exit (FR-004).

## Manifest

`addon/PixelBeacon/PixelBeacon.txt`: `## Version: 4` / `## AddOnVersion: 4`
advance to 5. Managed marker untouched (FR-008). `tests/beacon.rs` pin
advances 4 to 5.
