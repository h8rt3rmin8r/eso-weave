# Phase 0 Research: Fishing Reliability and Status Collaboration

## R1: Current live game API version and manifest multi-value form

**Decision**: Set the manifest to `## APIVersion: 101050 101054`. Confirm 101050
is still the live value against the client immediately before finalizing.

**Rationale**: The live game API version is 101050 (game Update 50, released
2026-06-08). The manifest currently declares 101044, which the game flags as out
of date; when a player has not enabled loading of out-of-date addons, the game
does not load the addon at all, so no beacon renders and the app never sees the
fishing signal. The game accepts a space-separated multi-value APIVersion form
(documented as up to two values, used to cover the current release and the next
one) and treats an addon as current when its highest declared value is greater
than or equal to the live value. Declaring 101050 makes the addon current today,
and declaring 101054 as the second value gives roughly four updates (about a year
of quarterly cadence) of runway before the addon is flagged again.

**Alternatives considered**:
- Single current value (101050 only): clears the flag now but the addon goes out
  of date again on the next update. Rejected: no runway, defeats the purpose.
- A single far-future value only: works numerically but discards the exact
  current match and relies less predictably on the parser. Rejected in favor of
  the documented two-value form.
- Many future values beyond two: the documented form is two values; going beyond
  it is unsupported and risks parser surprises for negligible extra benefit.
  Rejected.

**Tradeoff noted**: A larger runway means the addon will not be auto-flagged if a
genuine future API break lands before Update 54. This is acceptable because
PixelBeacon is a minimal, app-managed, unpublished addon that draws colored
blocks and reads a few stable events; the risk of a silent break is low, and
in-game validation is owed regardless.

## R2: Delivering the refreshed manifest to existing installs

**Decision**: Bump `## Version` and `## AddOnVersion` from 2 to 3.

**Rationale**: The beacon manager single-sources the embedded version by parsing
the embedded manifest (`embedded_version()` reads `## Version`). Beacon status
classification compares the on-disk `## Version` against the embedded version; an
APIVersion-only change would leave `## Version` at 2, so an existing on-disk copy
would still classify as current and the app would not rewrite it, and the new
APIVersion would never reach players already on version 2. Bumping to 3 makes the
app classify the on-disk copy as outdated and refresh it, delivering the new
manifest. The managed-marker line is not touched, so marker-gated safe uninstall
is unaffected.

**Alternatives considered**:
- Change only APIVersion: fails to propagate to existing installs. Rejected.

## R3: Adaptive poll cadence

**Decision**: Add a pure helper `poll_interval(fishing_active: bool, cfg:
&ReaderConfig) -> u64` in the `pixelbus` module that returns
`interval_fishing_ms` when active and `interval_idle_ms` otherwise. The worker
loop computes the next sleep from the current fishing state (state is not
Disabled) each iteration.

**Rationale**: The worker loop hardcodes `interval_idle_ms` (1000 ms) and never
reads `interval_fishing_ms` (100 ms), which is dead config exposed in Settings.
At 1 s cadence the reader misses the transient cast and bite signals and ticks
the state machine only once per second, so the reel keypress can land up to a
second late. Selecting the interval from the live fishing state fixes both the
arm-window starvation and the reel latency. A pure helper keeps the selection
unit-testable without threads.

**Alternatives considered**:
- Always poll at the fishing interval: wastes CPU and screen sampling while idle
  (10x the sampling for no benefit). Rejected.
- Event-driven wakeups instead of polling: a larger redesign of the worker;
  out of scope and unnecessary given the existing poll loop. Rejected.

## R4: Clock unification

**Decision**: Create one shared monotonic origin at startup and use it for both
the GUI intent path and the worker `tick()`, so fishing deadlines are stamped and
evaluated on a single timeline.

**Rationale**: Today the worker creates its own `Instant` origin while the GUI
model uses a separate origin; `set_enabled`/`on_event` stamp deadlines on the GUI
clock but `tick()` judges them on the worker clock. The skew is small today but is
a real correctness smell that can misjudge the arm and reel deadlines. A single
shared origin removes the class of bug. Keep the change minimal (share one origin
value, for example via an `Arc<Instant>` or a small shared clock passed to both).

**Alternatives considered**:
- Pass the GUI-produced `now` into the worker per event only: does not fix the
  `tick()` path, which has no GUI-supplied timestamp. Rejected.
- Leave as is: accepted skew is a latent correctness bug on the exact deadlines
  this feature depends on. Rejected.

## R5: Tuned fishing timeout defaults

**Decision**: `arm_timeout_ms` = 8000 (from 5000), `reel_delay_ms` = 100
(unchanged), `recast_delay_ms` = 3000 (unchanged). All remain user-configurable
and are provisional pending in-game validation.

**Rationale**: With 100 ms polling the cast confirmation should arrive within a
second or two, but the addon render and the interact registration can add
latency; 8000 ms gives generous margin so a valid cast is not prematurely
disarmed, while still failing a genuinely bad cast in a reasonable time. The reel
delay stays short so the reel lands inside the game reel window; the recast delay
is unchanged. Exact values are provisional because the true reel window and
render latency are only observable in-game.

**Alternatives considered**:
- Keep arm_timeout at 5000: less margin for render and input latency; borderline
  casts could still disarm. Rejected in favor of more margin.
- Aggressively lower reel_delay to 0: risks reeling before the game registers the
  bite. Rejected; 100 ms is a safe minimum.

## R6: Stop-reason representation and status derivation

**Decision**: Add a stop-reason value to the fishing controller recorded whenever
it returns to Disabled: one of UserStop, NoCastDetected (arm timeout), or
SignalLost. Expose it read-only so the view-model derives the status text. The
reason persists until the next cast/start. Status labels: Casting (Armed),
Fishing (waiting for a bite) (Waiting), Reeling in (Reeling), Recasting (Recast),
Idle (Disabled with no reason or user stop), and Idle with a parenthetical reason
for NoCastDetected and SignalLost. Derivation is a pure function tested in the
view-model layer; strings live in `strings.rs`.

**Rationale**: The current status prints the raw debug state name and reverts to a
bare Idle with no explanation, which the field feedback called out as
uncollaborative. Recording the reason at the single place the controller disables
keeps it authoritative; deriving text purely keeps the egui layer thin and the
logic testable. Persisting until the next start lets the player read why the last
session ended.

**Alternatives considered**:
- Compute the reason in the GUI from state transitions: the GUI does not see the
  arm-timeout transition reliably at 1 s frames and would duplicate controller
  logic. Rejected; record it at the source.
- A transient toast that auto-clears: the player may miss it; persistent status
  is more useful for troubleshooting. Rejected for the status line, though a log
  line may still be emitted.
