# Contract: Game Observation and Runtime Reduction

## Provider discovery

Each platform adapter returns candidate evidence and source health. It does not
select the final provider.

```text
discover_installations() -> DiscoveryObservation

DiscoveryObservation:
  candidates: list<InstallationCandidate>
  source_failures: set<ProviderSource>
```

The platform-neutral reconciler validates, normalizes, de-duplicates, and ranks
candidates. Provider priority applies only to candidates sharing the same root:

```text
Steam or Epic manifest > generic ESO uninstall entry
```

Different validated roots are Ambiguous. Enumeration order is never a tie
breaker.

## Runtime probe

```text
observe_processes() -> ProcessObservation
```

Required process names:

- game: `eso64.exe`, `eso.exe`
- launcher: `Bethesda.net_Launcher.exe`

The reducer follows the order in `data-model.md`. It must be a pure function with
the full game x launcher presence matrix under tests.

## Polling contract

- Cadence: once per 1,000 ms while the application is running.
- Execution: existing pixel-bus worker, before a missing-sampler early return.
- Normal logging: transitions only.
- Hook thread: no discovery, file, registry, process enumeration, or logging
  work.
- Exit from Active: clear downstream observations and reset reader history in
  the same worker iteration.

## Privacy contract

Info and warning logs may contain provider, evidence category, runtime, focus,
and error class. Absolute installation paths are Debug-only.

## Compatibility contract

`beacon::probe_game_running` may remain as a compatibility wrapper during the
slice, but its implementation delegates to the game module. New callers use the
game contract directly.
