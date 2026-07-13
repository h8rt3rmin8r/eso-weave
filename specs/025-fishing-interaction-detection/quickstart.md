# Quickstart: Validating the Fishing Interaction Detection Rewrite

## Prerequisites

- ESO Weave built from this feature (`cargo build --release`) or the packaged
  build carrying addon manifest version 4.
- The Elder Scrolls Online live client, windowed or borderless, on the
  Windows machine running ESO Weave.
- A character near any fishing hole with at least one bait type in the
  backpack.

## Automated verification (no game required)

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

Expected: all green with no modification to existing fishing or pixelbus
tests. The controller logging is additionally visible by running any fishing
controller test with `RUST_LOG` style capture, but the suite passing
unchanged is the gate (FR-010, SC-005).

## Addon update

1. Launch ESO Weave; the addon panel reports the installed PixelBeacon as
   out of date (installed 3, embedded 4).
2. Activate Update. Expected: uninstall verifies the managed marker, then
   install writes version 4; the panel reports current.
3. In the game, run `/reloadui` (or relog).

## In-game validation protocol

Set the application log level to debug or trace before starting.

1. **Preconditions**: bait selected (interact wheel on the hole shows the
   bait name), beacon strip visible top-left, game window focused, heartbeat
   row shows Pixel Beacon detected.
2. **Cast detection (SC-001)**: stand at the hole, press the fishing toggle
   (default F2). Expected within ~200 ms of the line hitting the water: the
   status advances Casting to Fishing; the log shows, in order,
   `eso_weave::fishing` lines for the cast keypress and armed state, then
   the cast-detected transition to waiting. The arm timeout must not fire.
3. **Bite and reel (SC-002)**: wait for the bite. Expected: B1 turns green,
   the log shows the bite transition to Reeling, the reel keypress line, and
   the recast line; the application recasts and the cycle repeats. Observe
   at least 3 consecutive full cycles.
4. **False-bite guard (SC-004)**: during a waiting phase, drink a potion or
   consume any non-bait stack. Expected: no bite transition in the log and
   no reel keypress.
5. **No-bait edge case**: clear the bait selection, toggle fishing. Expected:
   the game never starts the interaction, B1 stays hidden, and the log shows
   the arm timeout disable with stop reason NoCastDetected, now explicitly
   narrated.
6. **Interruption edge case**: mid-wait, open the inventory. Expected: the
   game ends the interaction; B1 hides within one tick; the controller sees
   FishingStopped and recasts (or, if the menu stays open so the recast finds
   no hole, the next arm timeout disables with NoCastDetected in the log).
7. **Stop (safety)**: toggle fishing off mid-cycle. Expected: immediate stop,
   log shows the disable with stop reason UserStop, no further keypresses.

## Diagnosing a failure (SC-003)

Any failed session must now be explicable from
`%APPDATA%\ESO-Weave\logs\<month>.log` alone: the fishing lines narrate every
transition and the final stop reason, and the existing pixel-bus trace shows
each block's raw bytes and decoded state alongside. A session with heartbeat
present but no cast-detected line points at the addon tick (check `/reloadui`
was run and the addon is version 4); a session with no heartbeat points at
capture (slice 024 territory).
