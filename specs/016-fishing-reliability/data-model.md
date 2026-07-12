# Phase 1 Data Model: Fishing Reliability and Status Collaboration

This feature adds one new value type (a stop reason), one new field on the
fishing controller, one pure selection function, and metadata changes to the
addon manifest. It does not add persisted config fields beyond the existing
fishing timings.

## FishingState (existing, unchanged)

The fishing routine phase. Values: `Disabled`, `Armed`, `Waiting`, `Reeling`,
`Recast`. Transitions are unchanged by this feature; only the cadence at which
`tick()` runs and the clock the deadlines are judged against change.

## StopReason (new)

The reason the controller last returned to `Disabled`. Recorded at the single
place the controller disables and cleared when a new cast starts.

- **UserStop**: the player turned fishing off.
- **NoCastDetected**: the arm timeout fired without a cast confirmation.
- **SignalLost**: the beacon signal was lost while a session was active.
- (absent): the controller has not run since startup, or a fresh start cleared
  the prior reason.

Held as an optional value on the controller (for example
`Option<StopReason>`), set inside `disable()` (which gains the reason as a
parameter or a caller-set field) and cleared in `cast()`.

## FishingConfig (existing, default values changed)

User-configurable fishing timings. Field set is unchanged; only defaults move.

| Field | Old default | New default | Notes |
| --- | --- | --- | --- |
| `arm_timeout_ms` | 5000 | 8000 | More margin for cast confirmation |
| `reel_delay_ms` | 100 | 100 | Unchanged; keeps reel in the game window |
| `recast_delay_ms` | 3000 | 3000 | Unchanged |
| `interact_key` | E | E | Unchanged |

Defaults are provisional pending in-game validation and remain editable in
Settings.

## Poll cadence selection (new pure function)

A pure function that selects the worker sleep interval from the fishing state.

- Input: whether fishing is active (fishing state is not `Disabled`) and the
  reader configuration (which carries `interval_fishing_ms` and
  `interval_idle_ms`).
- Output: the interval in milliseconds: `interval_fishing_ms` when active, else
  `interval_idle_ms`.
- No side effects; unit-tested directly.

## Fishing status view-model (existing types, derivation changed)

The `FishingLabel` (indicator plus button label) and `StatusLine` (title, state
text, role, tooltip) are derived each frame from the fishing state and, when
`Disabled`, the `StopReason`.

- `Armed` -> indicator "Casting"
- `Waiting` -> indicator "Fishing (waiting for a bite)"
- `Reeling` -> indicator "Reeling in"
- `Recast` -> indicator "Recasting"
- `Disabled` with no reason or `UserStop` -> indicator "Idle"
- `Disabled` with `NoCastDetected` -> indicator "Idle (no cast detected)"
- `Disabled` with `SignalLost` -> indicator "Idle (signal lost)"

The button label remains "Go Fish" when idle and "Stop Fishing" when active. The
status role stays Active for working states and Muted for idle; a stopped-with-
reason idle may use a Warning role to draw the eye (decision deferred to the
implementation, both are acceptable). Derivation is a pure function; the exact
strings live in `strings.rs`.

## Addon manifest metadata (changed)

The embedded `PixelBeacon.txt` manifest fields that change:

| Field | Old | New |
| --- | --- | --- |
| `## Version` | 2 | 3 |
| `## AddOnVersion` | 2 | 3 |
| `## APIVersion` | 101044 | 101050 101054 |

Unchanged and load-bearing: `## X-ESO-Weave-Managed: true` (the managed marker
that gates safe uninstall) and the file list line. `embedded_version()` will
parse `## Version` as 3.
