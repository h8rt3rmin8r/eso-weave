# Contract: Fishing Status, Cadence, and Addon Manifest

This feature is internal to the desktop app; the user-facing contracts are the
fishing status text the player reads, the poll cadence behavior, and the addon
manifest the game reads. These are the observable surfaces this slice must honor.

## Fishing status contract

Given the fishing controller state and, when idle, the recorded stop reason, the
status indicator MUST read as follows (plain language, no internal state names):

| Controller state | Stop reason | Indicator text |
| --- | --- | --- |
| Armed | n/a | Casting |
| Waiting | n/a | Fishing (waiting for a bite) |
| Reeling | n/a | Reeling in |
| Recast | n/a | Recasting |
| Disabled | none / UserStop | Idle |
| Disabled | NoCastDetected | Idle (no cast detected) |
| Disabled | SignalLost | Idle (signal lost) |

- The toggle button label MUST be "Go Fish" when idle and "Stop Fishing"
  otherwise.
- The stop reason MUST persist until the next start (cast), then clear.
- The derivation MUST be a pure function unit-tested without the egui layer.

## Poll cadence contract

- While fishing is active (state is not Disabled), the worker MUST select the
  fishing interval (`interval_fishing_ms`).
- While fishing is idle, the worker MUST select the idle interval
  (`interval_idle_ms`).
- The selection MUST be a pure, unit-tested function.

## Clock contract

- A fishing deadline MUST be evaluated on the same monotonic clock it was set on.
  No deadline set from the GUI intent path may be judged against a different
  origin than the worker `tick()` uses.

## Addon manifest contract

The embedded manifest MUST satisfy:

- `## APIVersion` declares at least the current live game API version, using the
  supported space-separated multi-value form to also declare a future value.
- `## Version` and `## AddOnVersion` are raised so an existing on-disk install is
  classified as outdated and refreshed.
- `## X-ESO-Weave-Managed: true` is present and unchanged, so uninstall still
  verifies the managed marker before deleting.
- `embedded_version()` parses the new `## Version` without error.

## Safety contract (unchanged, must stay tested)

- No blocking work on the input hook thread.
- Input suppression scoped to the focused game window only.
- On SignalLost the controller disables and cancels any pending interact rather
  than firing it blind.
- Uninstall deletes the addon folder only after verifying the managed-marker
  line.
