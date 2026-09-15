# Contract: Auto Potion Startup Restore

## Inputs

- A validated `SessionState` Boolean request.
- A newly constructed `AutoPotionController` with fail-closed runtime defaults.

## Required behavior

1. The application model applies the Boolean through `set_enabled`.
2. Applying the Boolean does not schedule a save, tick the controller, or call
   an input sink.
3. `false` produces Off.
4. `true` produces the immediate dormant or blocked state supported by current
   startup evidence.
5. Ordinary runtime routing remains solely responsible for current evidence.
6. The existing worker may submit input only when its unchanged trigger
   conjunction later succeeds.

## Interaction behavior

- UI toggle and F3 both resolve to `UiIntent::SetAutoPotion`.
- Applying that intent updates the controller request and marks the shared
  session scheduler.
- Settled and close-time saves read the controller request through
  `current_session_state`.

## Prohibitions

- No persisted effective state, telemetry, retry time, or trigger cause.
- No direct input during restore.
- No controller tick during restore.
- No gate relaxation or fabricated freshness.
- No duplicate request in configuration.
