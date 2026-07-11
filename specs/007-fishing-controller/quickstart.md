# Quickstart: Fishing Controller

Validates the controller end to end without a real game, device, or wall clock,
using a stub detector, a mock sink, and an explicit clock.

## Prerequisites

- The repository builds (features 002 input engine and 005 pixel bus reader are
  present).
- `cargo` is available; run all commands from the repo root.

## Build and verify (CI parity)

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

The `fishing` tests in `tests/fishing.rs` cover the scenarios below; a green run
is the primary validation.

## Scenario 1: Full cast-reel-recast cycle (US1)

- `set_enabled(true, 0, sink)` -> state Armed, sink recorded one interact (cast).
- `on_event(FishingStarted, 10, sink)` -> Waiting, no new op.
- `on_event(BiteDetected, 20, sink)` -> Reeling, no op yet.
- `tick(20 + reel_delay, sink)` -> Recast, one interact (reel).
- `tick(reel_time + recast_delay, sink)` -> still Recast, one interact (recast).
- `on_event(FishingStarted, later, sink)` -> Waiting for the next bite.
- Assert the interacts were emitted only at cast, reel, and recast, in order.

## Scenario 2: Signal loss disables safely (US2, safety-critical)

- From each active state (Armed, Waiting, Reeling, Recast): `on_event(SignalLost,
  now, sink)` -> Disabled and no interact emitted.
- With a pending reel deadline: `on_event(SignalLost)` before it, then
  `tick(past the deadline)` -> no interact emitted.
- After SignalLost, any further `tick` emits nothing until `set_enabled(true)`.

## Scenario 3: Arm and recast timeouts (US3, US1)

- `set_enabled(true, 0, sink)` -> Armed; `tick(arm_timeout_ms, sink)` with no
  FishingStarted -> Disabled, no further interact.
- In Recast after the recast interact: `tick(arm_timeout_ms later, sink)` with no
  FishingStarted -> Armed and one interact (re-cast).
- `set_enabled(false, now, sink)` from any state -> Disabled, pending deadline
  cleared, no interact.

## Scenario 4: Toggle idempotency

- `set_enabled(true)` twice in a row emits exactly one cast.
- `set_enabled(false)` while Disabled emits nothing.

## Scenario 5: Config round-trip (US4)

- `FishingConfig::store` then `FishingConfig::load` yields an equal config.
- Loading a null section yields the defaults (5000/100/3000, `Key::E`).
- An out-of-range timing or an unparsable interact key falls back to the default
  and pushes a notice.

## Scenario 6: Event mapping

- `map_event(PixelBusEvent::Latency(x))` is `None`; every other reader event maps
  to its matching `DetectorEvent`.

## On real hardware (manual, out of automated scope)

- Wired later by the GUI slice and the worker loop: a `PixelBusDetector` over the
  real sampler feeds `on_event`/`tick` on the worker thread, and a
  `RealFishingSink` drives the input backend so the interact key is synthesized
  in-game.
