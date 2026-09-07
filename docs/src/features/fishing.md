# Fishing

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

## Status meanings

| Status | Meaning |
| --- | --- |
| Casting | The cast was sent and awaits PixelBeacon confirmation |
| Fishing (waiting for a bite) | A cast is active |
| Reeling In | A bite was observed |
| Recasting | The catch resolved and another cast is pending |
| Idle | Fishing is off |
| Idle (no cast detected) | PixelBeacon never confirmed the cast |
| Idle (signal lost) | The beacon heartbeat disappeared |
| Idle (game not active) | ESO exited |

## State and safety behavior

The controller consumes `Heartbeat`, `FishingStarted`, `BiteDetected`,
`FishingStopped`, and `SignalLost` events and advances only on events and clock
ticks. It never blocks a worker and never sends input after losing the beacon.

After start, the controller sends the interact key once and waits up to
`arm_timeout_ms` (8000 ms by default) for a cast. A bite schedules the reel after
`reel_delay_ms` (100 ms), then the next cast after `recast_delay_ms` (3000 ms).
All three values and the interact key are configurable.

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
