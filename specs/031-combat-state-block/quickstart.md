# Quickstart and Validation: PixelBeacon In-Combat State Block

**Feature**: 031-combat-state-block | **Date**: 2026-07-27

How to build, check, and validate this feature. Everything in the desk section
runs without the game. The in-game section is the part that cannot be automated
and is the operator's to run.

## Prerequisites

- The pinned toolchain (`rust-toolchain.toml`, Rust 1.96.0). No new dependency is
  introduced by this feature.
- For the in-game section only: ESO installed, and the application able to
  resolve the AddOns directory.

## Desk validation

The full merge gate, run in the foreground and watched to completion. Never
background these; cargo buffers test output until the run ends, so a backgrounded
run cannot be told apart from a hung one.

```bash
cargo fmt --all -- --check
```

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

```bash
cargo test --all --locked
```

Expected: all three clean, with no test skipped, weakened, or made conditional.

### What the suite proves without the game

| Behavior | Where |
| --- | --- |
| The two combat colors decode to the right states | `tests/pixelbus.rs` |
| A wrong marker, a failed checksum, or an unrecognized code decodes to unavailable | `tests/pixelbus.rs` |
| An arbitrary color at the B4 point never decodes as a state (User Story 2) | `tests/pixelbus.rs` |
| A change is announced once and a steady state announces nothing | `tests/pixelbus.rs` |
| The state clears to unavailable on signal loss and on a non-decoding block | `tests/pixelbus.rs` |
| Every block-center green is pairwise separated beyond the default tolerance | `tests/pixelbus.rs` |
| The addon and the companion agree on the block count and all three color constants | `tests/beacon.rs` |
| The weave engine behaves identically for all three combat states (FR-016) | `tests/weave_engine.rs` |
| The view derives "In combat", "Out of combat", and "Not detected" with the right role | `tests/app_view_model.rs` |

### Targeted runs while iterating

```bash
cargo test --test pixelbus
```

```bash
cargo test --test beacon
```

## In-game validation

Required before this feature can be called validated in the field. It is the only
part that exercises the real game API and the real screen-capture path.

### Setup

1. Build and run the application.
2. Open the beacon controls. With an older addon installed, the manager should
   offer an update, because the manifest advanced from version 5 to 6.
3. Apply the update, then in game run `/reloadui` so the new addon loads.
4. Set the application log level to DEBUG and open the live log pane, so combat
   changes are visible as they are decoded.

### Scenario 1: transitions (SC-001)

1. Stand out of combat. Confirm the interface shows "Out of combat".
2. Enter combat. Confirm the interface changes to "In combat" within about a
   second, and that the live log records the change once.
3. Leave combat and let the combat flag drop. Confirm the interface returns to
   "Out of combat".
4. Repeat for at least ten transitions total.

**Pass**: every transition is reflected, in the right direction, with one log
entry each.

### Scenario 2: no churn while steady (SC-002)

1. Stay in a steady state, in or out of combat, for at least sixty seconds.

**Pass**: zero further combat entries in the log. Latency entries continuing is
expected and unrelated.

### Scenario 3: loading screen re-baseline (SC-003)

1. Enter combat, and while the interface reads "In combat", zone through a door
   or use a wayshrine so a loading screen occurs.
2. After the world loads, compare the interface against the actual state.
3. Repeat starting from out of combat.

**Pass**: the interface agrees with the real state after every load. A stale
value carried across the load is a failure.

### Scenario 4: older addon (SC-004)

1. Close the application. Install or restore the previous addon version, the one
   that draws four blocks.
2. Start the application and let it read the strip.

**Pass**: combat state reads "Not detected" and no combat changes are logged.
Every other signal (heartbeat, fishing, latency, weapon bar) continues to work
exactly as before. A reading of "In combat" or "Out of combat" here is the
failure this scenario exists to catch.

### Scenario 5: existing signals unaffected (SC-005)

1. With the new addon installed, confirm the heartbeat is present, the latency
   readout updates, and the weapon-bar readout still reports the active bar and
   both weapon classes correctly across a bar swap.
2. Run one fishing session end to end.

**Pass**: all four pre-existing signals behave as they did before this feature.

## Notes

- The whole strip widened from four to five block widths. If a non-default block
  size is configured, the capture region follows automatically; no separate
  adjustment is needed, and Scenario 5 is worth repeating once at a small block
  size if one is in use.
- Nothing consumes the combat signal yet, so there is deliberately no scenario
  here that asserts a behavior change from entering combat. If one appears to
  occur, that is a defect against FR-016.
