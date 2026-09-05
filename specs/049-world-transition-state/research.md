# Research: World Transition State

No NEEDS CLARIFICATION markers remain. Decisions follow the autopilot policy.

## ESO lifecycle authority

Decision: use `EVENT_PLAYER_DEACTIVATED` for transition entry and
`EVENT_PLAYER_ACTIVATED` for transition completion.

Rationale: the current live ESO UI source registers deactivation as loading-screen
shown and activation as loading-screen dropped. The same source combines
`IsPlayerActivated()` with camera state before treating the world as active.
Current API exports retain both events, and addon practice uses activation to
reinitialize state after zone changes and reloads.

Sources:

- <https://github.com/esoui/esoui/blob/live/esoui/ingame/scenes/ingamescenemanager.lua>
- <https://github.com/mrheault/eso-api-lua-intellij/blob/master/eso-api.events.lua>
- <https://www.esoui.com/forums/archive/index.php/t-9691.html>

Rejected alternatives:

- `EVENT_PREPARE_FOR_JUMP`: it does not cover every door, reload, or loading path
  and belongs to the earlier TravelPending problem in #59.
- heartbeat disappearance: it is delayed by timeout and the old pixels may remain
  visible during loading.
- polling a player-active query: no periodic inference can express the immediate
  start boundary as directly as the event pair.

## Wire representation

Decision: add B22 with marker `0xCC`, red payloads `0x20` Unknown, `0x80`
Transitioning, and `0xE0` Active, and blue `255 - red`.

Rationale: a dedicated block keeps world lifecycle orthogonal to heartbeat and
life state. `0xCC` occupies the widest useful remaining green-channel gap and
stays six channel values from both `0xC6` and `0xD2`, three times the default
tolerance. Discrete red values match the proven life-state spacing.

Rejected alternatives:

- overload B0 heartbeat: heartbeat means readable addon pixels, not active world.
- overload B21 life: Alive and world Active are different facts during loading.
- omit wire Unknown: the addon must represent its pre-activation state truthfully.

## Protocol generation

Decision: advance the negotiated layout header from version 1 (`0x20`) to version
2 (`0x40`) with addon version 16. Continue decoding version 1 geometry using its
22-cell payload extent, but sample B22 only after positively identifying version 2.

Rationale: a version-15 overlay ends at B21. Sampling the next ordinary screen
pixel as if it were B22 creates a low-probability false positive even with marker
and checksum validation. The header generation is the existing authority for
payload shape, so it is the correct discriminator and preserves all earlier
signals without trusting pixels the older protocol never defined.

## Activation baseline

Decision: extract one `rebaselinePlayerState()` addon function that refreshes
weapon, combat, menu, resources, movement, cooldowns, quickslot, life, and
fishing, then publish Active after it returns.

Rationale: a named boundary is directly testable and prevents a later payload
from being appended below the Active write by accident. An explicit unavailable
payload is still fresh because the current API call produced it.

Rejected alternative: inline the existing activation callback body and append
Active. That preserves behavior today but makes the ordering contract fragile.

## Companion ownership

Decision: store WorldState in `GameObservations` and update it through
`route_game_observation`.

Rationale: this is runtime game context already shared between the worker and UI.
Putting it in WeaveEngine would falsely imply that it is a weave concern, while
adding a second shared handle would duplicate synchronization.

## Consumer boundary

Decision: do not change input, weave, fishing, or potion gates in S049.

Rationale: #56 is the atomic observable issue. #59 separately requires live
verification of the cancellable pre-loading interval and combines that evidence
with this world lifecycle. Consuming only part of the state now would create a
misleading partial travel safety claim.
