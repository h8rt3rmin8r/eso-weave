# Research: Life State Safety

No NEEDS CLARIFICATION markers remain. Decisions follow the autopilot policy.

## R1. ESO exposes authoritative life-state primitives

The current ESO API exposes player death events and unit-state queries. The addon
can respond immediately to `EVENT_PLAYER_DEAD` and `EVENT_PLAYER_ALIVE`, then
re-baseline at `EVENT_PLAYER_ACTIVATED`. `IsUnitReincarnating("player")`
distinguishes the transition that a combined dead-or-reincarnating query would
collapse. `IsUnitDead("player")` supplies the remaining dead state.

Decision: compute in precedence order Reincarnating, Dead, Alive. Use one
function for event handlers, activation, initial render, and the one-second
convergence tick. This avoids trusting event order or carrying a second state
machine in Lua.

References:

- UESP ESO API export: <https://esoapi.uesp.net/current/data/i/s/u/IsUnitReincarnating.html>
- UESP ESO API export: <https://esoapi.uesp.net/current/data/i/s/u/IsUnitDead.html>
- UESP ESO API export: <https://esoapi.uesp.net/current/data/_/e/v/_EVENT_PLAYER_ACTIVATED_.html>

## R2. Add one payload block without changing negotiated geometry

S045 made the addon's published physical column capacity authoritative. The
reader locates payloads from a validated layout and already derives extent from
`NUM_BLOCKS`. Adding B21 requires incrementing the shared count from 21 to 22,
adding one named sample point, and leaving layout protocol version 1 intact.

Decision: use the existing red payload, green marker, blue complement pattern.
The marker must occupy the midpoint of a widest remaining gap in the shared green
registry and stay outside the configured tolerance of every incumbent.

## R3. Unknown must fail closed

Heartbeat and payload validity are separate facts. An older addon, corrupt block,
or newly reacquired heartbeat can provide no trustworthy B21 value.

Decision: `Unknown` is the default and signal-loss state, and `LifeState::gates()`
returns true for every value except Alive. A heartbeat never implies Alive.

## R4. Gate each synthesis boundary from one normalized value

The application has four relevant boundaries: physical action interception,
worker-side weave execution, fishing timer output, and auto-potion timer output.
Gating only interception misses already queued work and autonomous controllers.
Gating only the worker swallows the player's physical key without producing the
original or synthesized action.

Decision: routing pushes the same `LifeState` to each subsystem. Input
classification passes non-exempt physical keys through when gated. Weave checks
again before a sequence. Fishing cancels a due autonomous action instead of
retaining its deadline. Auto-potion reports a blocker and emits nothing. This is
distributed enforcement of one authority, not four independently derived rules.

## R5. Do not replay stale work

A weave action queued before death may execute afterward. Fishing deadlines and
low-resource potion conditions can also become due during death.

Decision: weave drops the queued action, fishing cancels the pending deadline and
returns to a safe disabled effective state while preserving the user's request,
and auto-potion re-evaluates current readings only. Returning to Alive does not
invoke a sink. Fresh input or fresh eligible telemetry is required.

## R6. Persist disclosure as a UI preference

Open or closed is a deliberate presentation choice, not derived runtime state.
The constitution permits user settings in `config.json` and prohibits derived
runtime state there.

Decision: add `system_state_expanded` to the existing `ui` settings object with a
default of true. Missing keys migrate additively. The complete header is the
single pointer and keyboard target, and the body is removed from layout while
closed so intrinsic sizing remains truthful.

