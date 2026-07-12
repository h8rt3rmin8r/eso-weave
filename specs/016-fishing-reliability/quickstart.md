# Quickstart: Validating Fishing Reliability and Status Collaboration

This guide validates the feature end to end. Automated checks prove the pure
logic and the manifest; the in-game section is the owed manual validation that
only a live game session can provide.

## Prerequisites

- The repository builds: `cargo build`.
- For the in-game section: a live game client (current live update), the app
  installed, and the PixelBeacon addon installed by the app.

## Automated validation

Run the full gate in the foreground and watch it to completion:

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

Expected: all pass. The new and updated tests cover:

- Poll cadence: the cadence helper returns the fishing interval when fishing is
  active and the idle interval when not.
- Stop reason: after an arm timeout the controller records NoCastDetected; after
  SignalLost it records SignalLost; after a user stop it records UserStop; a new
  cast clears the reason.
- Status derivation: each state and each stop reason maps to the plain-language
  indicator text in the status contract, with no internal state names.
- Timeout defaults: FishingConfig default arm timeout is 8000 ms; reel and recast
  defaults are unchanged.
- Manifest: the embedded manifest parses to version 3, declares an APIVersion at
  least the targeted live value, and still contains the managed-marker line.
- Existing beacon tests (managed-marker gating, version compare) still pass.

## In-game validation (owed manual test)

1. Run the app and let it install or refresh the PixelBeacon addon. Reload the
   game UI (/reloadui) or relog.
2. Open the in-game AddOns list and confirm PixelBeacon is NOT flagged out of
   date (SC-003).
3. Stand at a fishing hole with the interact prompt visible. Do not cast
   manually. Press the fishing hotkey.
4. Confirm the app status advances from Casting to Fishing (waiting for a bite)
   rather than reverting to Idle within a few seconds (SC-001).
5. When a fish bites, confirm the catch is reeled in and collected (SC-002) and
   that the routine then recasts on its own.
6. Turn fishing off and confirm the status reads Idle.
7. Provoke a stop path: press the hotkey while not aimed at a hole and confirm
   the status reads Idle (no cast detected) after the arm window (SC-004).

Report results back for confirmation, including whether the out-of-date flag is
gone and whether catches reel in reliably across at least ten casts.
