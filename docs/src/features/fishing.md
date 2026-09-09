# Fishing

The Interact Key is also the use key or action key, and its shipped default is E.

Fishing can cast, wait for a bite, reel in, and recast while the player remains
at a fishing hole. The hotkey performs the first cast. Do not cast manually
before starting the routine.

## Before starting

- Select bait in ESO. Without bait, the cast is not confirmed and fishing stops
  after the arm timeout.
- Install the current PixelBeacon from System and State and enable it in ESO.
- If ESO marks the addon out of date, update it and use `/reloadui` or relog.
- Keep the PixelBeacon overlay visible and the ESO window focused.
- Face a fishing hole until the interact prompt appears.

Press `F2`, or use the Fishing toggle, to cast and start. Press it again to stop.
If ESO Weave is suspended, enabling Fishing retains the request but sends no
cast. Resume also sends nothing. Face the fishing hole and make a fresh manual
cast, or turn Fishing off and on after resuming, to recover deliberately.

On Linux, the default generated `E` interact key is included in the virtual
device capability set before interception begins.

## Status meanings

| Status | Meaning |
| --- | --- |
| Casting | The cast was sent and awaits PixelBeacon confirmation |
| Fishing (waiting for a bite) | A cast is active |
| Reeling in | A bite was observed |
| Recasting | The catch resolved and another cast is pending |
| Idle | Fishing is off |
| Idle (no cast detected) | PixelBeacon never confirmed the cast |
| Idle (signal lost) | The beacon heartbeat disappeared |
| Idle (game not active) | ESO exited |
| Idle (game unfocused) | ESO lost keyboard focus |
| Idle (suspended) | ESO Weave is suspended; the request is retained but no input is pending |
| Idle (player unavailable) | Alive is not authoritatively observed |
| Idle (world unavailable) | World State is loading or Unknown |
| Idle (travel pending) | A recall or jump is pending or cannot be ruled out |
| Idle (settings changed) | Fishing settings changed; start Fishing explicitly to use them |

The request is retained through game inactivity, focus loss, and life, world, or
travel safety cancellation, but no generated input is retained for replay. After
game inactivity or focus loss, returning to an active, focused, and otherwise safe
game automatically starts a fresh cast. After a life, world, or travel safety
cancellation, face the fishing hole and make a fresh manual cast (or toggle
Fishing off and on) once safe evidence returns. Signal loss clears the request;
restore the heartbeat and start again. **Idle (no cast detected)** also clears the
request and requires a new start after checking bait and alignment.

Entering suspension cancels any cast-confirmation, reel, recast, or timeout
deadline and retains the request. Leaving suspension never emits or restores the
cancelled action. A fresh manual cast or an off-on Fishing toggle is required
after the other safety gates permit output.

Dead, Unknown, and all three Recovering paths cancel the current Fishing
deadline without replay. Life becoming Alive emits no Fishing input and does not
restore the cancelled timer. The reader requires a later fresh cast observation
after death recovery.

## State and safety behavior

The controller consumes `Heartbeat`, `FishingStarted`, `BiteDetected`,
`FishingStopped`, and `SignalLost` events and advances only on events and clock
ticks. It never blocks a worker and never sends input after losing the beacon.

After start, the controller sends the interact key once and waits up to
`arm_timeout_ms` (8000 ms by default) for a cast. A bite schedules the reel after
`reel_delay_ms` (100 ms), then the next cast after `recast_delay_ms` (3000 ms).
All three values and the Interact Key are configurable in Settings and apply to
the running controller. A changed Fishing configuration stops requested or
active work without emitting another interact, clears its pending deadline, and
shows **Idle (settings changed)**. Start Fishing explicitly to use the new
configuration. An identical settings application preserves the current state.

If a native game menu opens, autonomous reel and recast actions are deferred and
retried. The state cannot advance past an interact that ESO did not receive. The
operator-initiated first cast is not deferred because it directly follows the
operator's keypress.

PixelBeacon recognizes an active cast from `GetInteractionType()` and recognizes
the bite only from bait consumption while a cast is active. The standing reel-in
prompt is not a bite signal, and `EVENT_CLIENT_INTERACT_RESULT` is an error-alert
channel rather than a successful-cast signal.

If fishing returns to Idle within a few seconds, confirm bait, addon status,
overlay visibility, ESO focus, and that the routine was started while aimed at
the fishing hole.

For a state-by-state recovery path, see
[Fishing returns to Idle](../getting-started/troubleshooting.md#fishing-returns-to-idle).
