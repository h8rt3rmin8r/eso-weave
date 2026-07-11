# Tasks: Fishing Controller

**Feature**: `specs/007-fishing-controller` | **Branch**: `007-fishing-controller`

Test-first per constitution Principle III. The safety-critical behavior (fishing
disables on SignalLost rather than blind-firing, and pending interacts are
cancelled on any transition out of the scheduling state) lives in the pure state
machine and is covered by required, non-weakened tests against a stub detector, a
mock sink, and an injected clock, before the code lands. Paths are
repository-relative.

## Phase 1: Setup

- [x] T001 Add a `Key::E` variant to `src/input/key.rs` (with `as_str` `"e"` and `parse` `"e"`), keeping the enum and its round-trip complete. Declare `pub mod fishing;` in `src/lib.rs` and create compiling stub files `src/fishing/mod.rs` and `src/fishing/detector.rs`, warning-free.

## Phase 2: Foundational

- [x] T002 Define the core types in `src/fishing/mod.rs`: `DetectorEvent`, `FishingState`, `FishingConfig` (defaults arm_timeout_ms 5000, reel_delay_ms 100, recast_delay_ms 3000, interact_key `Key::E`), the `FishingSink` trait, `MockFishingSink` (records `(Key, Transition)`), and the `FishingController` struct (starting Disabled) with an internal deadline field.

## Phase 3: User Story 1 - Cast, reel, and recast loop (P1)

- [x] T003 [P] [US1] Write failing tests in `tests/fishing.rs`: with a `MockFishingSink` and an explicit clock, `set_enabled(true)` from Disabled emits one interact (cast) and enters Armed; FishingStarted -> Waiting; BiteDetected -> Reeling with no immediate emit; a tick at reel_delay emits the reel interact and enters Recast; a tick at recast_delay emits the recast interact; FishingStarted from Recast -> Waiting; a tick at the recast arm-timeout with no FishingStarted -> Armed with a re-cast interact.
- [x] T004 [US1] Implement `set_enabled`, `on_event` (FishingStarted, BiteDetected), and `tick` (ReelDue, RecastDue, RecastArmTimeout) plus the interact emission (Down then Up) in `src/fishing/mod.rs` (FR-003, FR-004, FR-006 to FR-008a, FR-012). Run US1 tests to green.

## Phase 4: User Story 2 - Signal loss disables fishing safely (P1, safety-critical)

- [x] T005 [P] [US2] Write failing safety tests in `tests/fishing.rs`: from each active state (Armed, Waiting, Reeling, Recast), `on_event(SignalLost)` enters Disabled and emits no interact; with a pending reel or recast deadline, SignalLost then a tick past the deadline emits nothing; after SignalLost no tick emits anything until `set_enabled(true)` again; no interact is ever emitted while Disabled.
- [x] T006 [US2] Implement the SignalLost transition (any active state -> Disabled, deadline cleared, no emit), the leaving-cancels-deadline rule, and the no-emit-while-Disabled invariant in `src/fishing/mod.rs` (FR-009 to FR-011). Run US2 tests to green.

## Phase 5: User Story 3 - Arm and disarm control (P1)

- [x] T007 [P] [US3] Write failing tests in `tests/fishing.rs`: `set_enabled(true)` then a tick at arm_timeout_ms with no FishingStarted -> Disabled with no further interact; `set_enabled(false)` from each state -> Disabled with any pending deadline cleared and no emit; FishingStopped from Waiting -> Armed with a re-cast interact; toggle idempotency (a second `set_enabled(true)` emits no second cast; `set_enabled(false)` while Disabled emits nothing).
- [x] T008 [US3] Implement the arm-timeout disarm, `set_enabled(false)` disarm, FishingStopped handling, and toggle idempotency in `src/fishing/mod.rs` (FR-004, FR-005, FR-006). Run US3 tests to green.

## Phase 6: User Story 4 - Configurable timing persisted in settings (P2)

- [x] T009 [P] [US4] Write failing tests in `tests/fishing.rs`: `FishingConfig::store` then `load` round-trips a custom config; a null section yields defaults; an out-of-range timing and an unparsable interact key each fall back to default and push a notice.
- [x] T010 [US4] Add the additive opaque `fishing` section to `Settings` in `src/config/mod.rs` (field, `RawSettings`, default, known-keys) and implement `FishingConfig::load`/`store` in `src/fishing/mod.rs` reusing a range-checked-with-notice helper and `Key::parse` (FR-014, FR-015). Run US4 tests to green.

## Phase 7: Detector abstraction and adapter

- [x] T011 [P] Write failing tests in `tests/fishing.rs` for `map_event`: `Latency(_)` maps to `None`; every other `PixelBusEvent` maps to its matching `DetectorEvent`. Add a `StubDetector` returning scripted events and a test that drives the controller through the detector.
- [x] T012 Implement `DetectorEvent` mapping (`map_event`), the `BiteDetector` trait, `StubDetector`, and `PixelBusDetector` (wrapping `PixelBusReader` plus a `SurfaceSampler`, `poll` calls `sample_and_observe` and maps) in `src/fishing/detector.rs` (FR-001, FR-002). Run detector tests to green.

## Phase 8: Real sink

- [x] T013 Implement `RealFishingSink<B: InputBackend>` in `src/fishing/mod.rs` whose `key` calls `backend.synthesize` and logs a warning on failure (never panics, never blocks). Clippy-clean on the host.

## Phase 9: Polish and cross-cutting

- [x] T014 [P] Add module and item documentation across `src/fishing/*.rs`.
- [x] T015 Update `CHANGELOG.md` `[Unreleased]` with an Added line for the Fishing Controller and a dated Decisions entry for the non-blocking event-and-tick state machine, the dedicated input-only `FishingSink` seam (no weave dependency), the additive `fishing` settings section, and the added `Key::E` interact key.
- [x] T016 Run CI parity: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked`, plus `cargo check --target x86_64-unknown-linux-gnu`.

## Dependencies and order

- Setup (T001) then Foundational (T002). US1 (T003 to T004) establishes the core
  loop; US2 (T005 to T006) adds the safety behavior on top; US3 (T007 to T008)
  adds arm/disarm and idempotency. US4 (T009 to T010) is the config surface. The
  detector abstraction and adapter (T011 to T012) depend only on the core types and
  the reader. The real sink (T013) depends on the input backend. Polish (T014 to
  T016) last; T016 is the gate.

## Parallel opportunities

- The test-authoring tasks (T003, T005, T007, T009, T011) share `tests/fishing.rs`
  and land sequentially despite the `[P]` marker.
- Documentation (T014) is independent once the modules exist.

## MVP scope

US1 (the cast-reel-recast loop) plus US2 (the safety-critical signal-loss disable)
over a stub detector and mock sink are the minimum viable increment: the full loop
with its safety guarantee, fully tested without a real game. Arm/disarm control,
config persistence, the detector adapter, and the real sink complete the feature.
