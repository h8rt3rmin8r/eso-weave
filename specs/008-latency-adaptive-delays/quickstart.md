# Quickstart: Latency-Adaptive Delays

Validates the effective-delay computation and its gating without a real reader,
latency source, or clock, using crafted latency values and configurations.

## Prerequisites

- The repository builds (feature 003 weave engine and feature 005 pixel bus reader
  are present).
- `cargo` is available; run all commands from the repo root.

## Build and verify (CI parity)

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

The `weave_latency` tests in `tests/weave_latency.rs` cover the scenarios below,
and the existing `tests/weave_sequence.rs` and `tests/weave_engine.rs` staying
green demonstrate the no-regression property.

## Scenario 1: Scaling when enabled (US1)

- `cfg = { enabled: true, k: 0.25 }`, base `d_weave = 50`, latency `120`.
- `effective_delay(50, Some(120), &cfg) == 50 + round(0.25 * 120) == 80`.
- Build a light-attack sequence for a slot with that base and confirm its
  `Wait(d_weave)` step is `Wait(80)`.
- Build a bash-attack sequence and confirm both `d_weave` and `d_bash` waits are
  scaled and the other steps are unchanged.
- Build a heavy-attack sequence and confirm its `d_heavy` wait is unchanged.

## Scenario 2: Off by default and no-data (US2, no-regression)

- Default `LatencyConfig` (feature off): for every weave type,
  `sequence_for_adapted(slot, timing, Some(120), &default)` equals
  `sequence_for(slot, timing)`.
- Enabled but `latency == None`: sequences equal the base sequences.
- After `set_latency(None)` (signal lost), `handle` produces base-delay sequences.

## Scenario 3: Clamp bounds (US3)

- A latency and `k` whose `round(k * latency)` exceeds 300 yields
  `effective_delay == base + 300` (cap inclusive at exactly 300).
- `k = 0.0`, or a latency small enough that `round(k * latency) == 0`, yields
  `effective_delay == base` (never below base).

## Scenario 4: Config round-trip (US4)

- Store a `{ enabled: true, k: 0.5 }` config and reload it; the values match.
- Loading an absent `latency` section yields the defaults (off, `k = 0.25`).
- Loading a `k` that is non-finite or outside `[0.0, 4.0]` falls back to 0.25 and
  pushes a notice.

## Scenario 5: Intake effect on handle

- With the feature enabled, `set_latency(Some(200))` then a handled weave produces
  scaled `d_weave`/`d_bash` waits; `set_latency(None)` then a handled weave produces
  base waits.

## On real hardware (manual, out of automated scope)

- Wired later by the GUI slice and the worker loop: reader `Latency` events call
  `set_latency(Some(ms))` and `SignalLost` calls `set_latency(None)`, and the GUI
  exposes the enabled flag and `k`.
