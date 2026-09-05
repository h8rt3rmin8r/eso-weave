# Research

## Decision

Use the current LibSprint-style action-slot inference only for keyboard-mode on-foot sprinting:

1. Require active-world, on-foot movement and reject swimming, falling, death, reincarnation, roll dodge, and gamepad mode.
2. Query `GetActiveHotbarCategory()` and treat all ordinary slots on that actual
   active bar reporting `ActionSlotHasNonCostStateFailure` as candidate evidence.
3. Debounce candidate entry for 200 ms, debounce ambiguous slot recovery for 200 ms, and clear immediately on stopped movement or a hard exclusion.
4. Refresh from `EVENT_ACTION_SLOT_STATE_UPDATED` and the existing 100 ms active-world tick.
5. Expire explicit sprint after 1,500 ms without qualifying evidence.
6. Leave mounted gallop unsupported and do not use stamina or speed as independent proof.

## Evidence

- [LibSprint](https://www.esoui.com/downloads/info3827-LibSprint.html) uses movement plus action-slot non-cost failures and explicitly documents that it does not support gamepad mode. Personal Assistant Consume uses it to avoid rejected consumption while sprinting.
- [ESOUI's action bar](https://github.com/esoui/esoui/blob/live/esoui/ingame/actionbar/actionbar.lua)
  uses `GetActiveHotbarCategory()` as the active display authority. Deriving only
  primary or backup from the weapon pair would misread temporary and special bars.
- [API 101050 live testing](https://forums.elderscrollsonline.com/en-gb/discussion/696465/api-request-please-expose-the-existing-player-sprint-state-isplayersprinting-isunitsprinting) found that `ACTION_RESULT_SPRINTING`, item usability, velocity, stamina, and the LibSprint method do not provide a reliable universal or console signal.
- [`GetUnitWorldPosition` API notes](https://forums.elderscrollsonline.com/en/discussion/454530/update-21-api-patch-notes-change-log-pts) confirm coordinates exist, but speed remains an ambiguous proxy and is unnecessary for this bounded keyboard implementation.
- Current project code already reserves B9 `0xA0` for on-foot sprint and `0xE0` for mounted sprint, so no protocol or geometry expansion is needed.

## Clarifications Resolved

- S052 does not claim universal sprint detection. Explicit `Sprinting` is a bounded keyboard-mode result.
- Gamepad mode retains truthful base movement without publishing explicit sprint.
- Mounted gallop remains unsupported because the available public signals do not prove it reliably.
- Unknown movement does not indefinitely suppress auto-potion. Only explicit `Sprinting` blocks it.
- A blocked low-resource condition is not queued. The ordinary current-state rule runs again when movement changes.

## Alternatives Rejected

- Fixed speed thresholds fail under snares, buffs, walls, teleportation, and character-specific movement modifiers.
- Stamina drain fails under skills, poisons, potions, recovery timing, and mounted War Mount behavior.
- `ACTION_RESULT_SPRINTING` exists as a constant but current live testing reports no passive sprint events.
- Private special-move functions and key-state functions are outside the addon contract.
- Desktop sprint-key tracking would require binding synchronization and still would not prove toggle or gamepad state.

## Validation Boundary

Deterministic tests prove state-machine timing, exclusions, encoding, decoding, and auto-potion behavior. Live validation must exercise stationary, run, sprint, sprint into a wall, speed modification, swimming, falling, roll dodge, death, loading, bar swap, keyboard hold and toggle settings, gamepad mode, mounted riding, and mounted gallop.
