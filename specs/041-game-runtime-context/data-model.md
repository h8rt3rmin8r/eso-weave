# Data Model: Game Runtime and Context Truth

## InstallationProvider

Values:

- `EsoStore`
- `Steam`
- `Epic`
- `SteamProton`

The provider describes the platform-owned evidence that won reconciliation. It
is not inferred from a default path.

## InstallationCandidate

Fields:

- `provider: InstallationProvider`
- `root: PathBuf`
- `source: CandidateSource`

Validation rules:

- The root is normalized before comparison.
- Expected client and launcher artifacts must exist below the root.
- Personal paths are Debug-only and never included in normal transition logs.
- A generic Windows ESO uninstall entry is an `EsoStore` candidate only after
  stronger platform candidates for the same root are reconciled.

## InstallationState

Values:

- `NotDetected`
- `Detected(InstallationCandidate)`
- `Ambiguous`
- `Unknown`

Transitions:

- Zero validated candidates -> `NotDetected`
- One normalized root with one winning provider -> `Detected`
- More than one distinct validated root, or conflicting strong providers for
  one root -> `Ambiguous`
- Provider source could not be read authoritatively -> `Unknown` only when the
  failure prevents a truthful negative result

## ProcessObservation

Fields:

- `game: Presence`
- `launcher: Presence`
- `focus: FocusObservation`

`Presence` values are `Present`, `Absent`, and `Unknown`.

## GameRuntime

Values:

- `Inactive`
- `LauncherOpen`
- `Active`
- `Unknown`

Reduction order:

1. game Present -> `Active`
2. game Unknown -> `Unknown`
3. launcher Present -> `LauncherOpen`
4. launcher Unknown -> `Unknown`
5. both Absent -> `Inactive`

Installation state does not change that precedence. It is displayed separately.

## FocusObservation

Values:

- `Focused`
- `Unfocused`
- `Unknown`

Focus is meaningful only while runtime is Active. Unknown is safety-negative.

## BeaconFreshness

Values:

- `NeverObserved`
- `Fresh`
- `Lost`

A valid heartbeat changes the value to Fresh. Timeout changes it to Lost. A
runtime exit resets it to NeverObserved.

## SurfaceObservation

Values:

- `Unavailable`
- `Observed(MenuSurface)`

`Observed(MenuSurface::None)` is authoritative gameplay evidence.
`Unavailable` covers absent samples, invalid marker/checksum/code, signal loss,
loading, and an older addon without the block.

## GameObservations

Fields:

- `installation: InstallationState`
- `runtime: GameRuntime`
- `focus: FocusObservation`
- `freshness: BeaconFreshness`
- `surface: SurfaceObservation`

This is shared runtime state. Updates compare the whole relevant axis and emit
normal logs only when it changes.

## GameContext

Values:

- `NotDetected`
- `Unfocused`
- `Gameplay`
- `Surface(MenuSurface)`
- `SignalUnavailable`
- `Unknown`

The reviewed surface labels are System menu, Map, Inventory, Mail, Character,
Guild store, Crown store, Journal, Chat entry, and Other menu.

Projection order:

1. runtime Unknown -> `Unknown`
2. runtime not Active -> `NotDetected`
3. focus Unknown -> `Unknown`
4. focus Unfocused -> `Unfocused`
5. freshness not Fresh -> `SignalUnavailable`
6. surface Unavailable -> `SignalUnavailable`
7. observed None -> `Gameplay`
8. observed named or Other -> `Surface`

## Safety Gates

The input engine carries independent booleans for application suspension,
game-active state, focus, and menu gate. A real key is eligible for interception
only when the application is not suspended, the game is active, the game is
focused, and no authoritative menu gate is set.

Auto-potion carries the same game-active gate in addition to its existing
suspend, menu, signal, quickslot, cooldown, resource, and retry conditions.
Runtime inactivity does not clear requested auto-potion enablement.
