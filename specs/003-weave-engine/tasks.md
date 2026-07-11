# Tasks: Weave Engine

**Feature**: `specs/003-weave-engine` | **Branch**: `003-weave-engine`

Test-first per constitution Principle III. The correctness-critical logic (the
`sequence_for` builder and cooldown gating) is covered by unit tests with a
`MockSink` and a virtual clock before the code lands. Paths are
repository-relative. `[P]` marks tasks on different files with no incomplete
dependency.

## Phase 1: Setup

- [x] T001 Declare `pub mod weave;` in `src/lib.rs` and create compiling stubs `src/weave/mod.rs`, `src/weave/types.rs`, `src/weave/sequence.rs`, each warning-free.

## Phase 2: Foundational (blocking prerequisites)

- [x] T002 Implement the weave types in `src/weave/types.rs`: `WeaveType`, `MouseButton`, `InputOp`, `WeaveStep`, `TimingConfig` (defaults 500/50/1000/125), `SlotOverrides`, `SkillSlot`, and `WeaveConfig` with the section 7.1 default slots.
- [x] T003 Add `MouseButton` handling to the Input Engine: `InputBackend::synthesize_mouse(button, transition)` on the trait and on `MockBackend` (record), and add per-action activity to `InputEngine` (`set_action_active`, and `classify` treating a bound but inactive action as pass-through), in `src/input/mod.rs` and `src/input/mock.rs`.
- [x] T004 Add additive `skills` and `timing` sections to `config::Settings` (serde default, backward compatible) with load helpers that return notices for invalid timing or unknown weave type, in `src/config/mod.rs`.

## Phase 3: User Story 1 - Correct weave sequences (P1)

**Goal**: FR-003 to FR-007. The core value.

**Independent test**: For each weave type, `sequence_for` returns exactly the
specified ordered steps.

- [x] T005 [P] [US1] Write failing tests in `tests/weave_sequence.rs`: for each of the four weave types, assert the exact ordered `Vec<WeaveStep>` from `sequence_for` (primary and secondary as left and right mouse, the slot key, and the correct waits at the correct positions); a per-slot d_weave override changes the relevant wait while another slot uses the global default.
- [x] T006 [US1] Implement `sequence_for(slot, timing) -> Vec<WeaveStep>` in `src/weave/sequence.rs` for all four weave types with per-slot override substitution (FR-003 to FR-007, FR-009). Run `tests/weave_sequence.rs` to green.

## Phase 4: User Story 2 - Cooldown gating (P1)

**Goal**: FR-010, FR-011.

**Independent test**: With a virtual clock, a second action inside the cooldown
window runs no sequence; after the window, the next action runs.

- [x] T007 [P] [US2] Write failing tests in `tests/weave_engine.rs`: a `MockSink` with a virtual clock; two actions inside `global_cooldown` run exactly one sequence; advancing the clock past the window lets the next action run; each skill action maps to its slot and the toggle actions run no sequence.
- [x] T008 [US2] Implement `WeaveEngine` in `src/weave/mod.rs`: the `WeaveSink` trait (`emit`, `wait`, `now_ms`), a `MockSink`, `slot_for_action`, `handle` with start-to-start cooldown gating and toggle exclusion, and step execution through the sink (FR-010 to FR-014). Run `tests/weave_engine.rs` to green.

## Phase 5: User Story 3 - Inactive slots pass through (P2)

**Goal**: FR-002.

**Independent test**: An inactive slot's key is passed through by `classify`;
activating the slot restores interception.

- [x] T009 [P] [US3] Write failing tests in `tests/weave_engine.rs`: with a weave action marked inactive via `set_action_active`, `InputEngine::classify` returns pass and hands off nothing while focused; after marking it active, the same key is suppressed and handed off.
- [x] T010 [US3] Implement `WeaveEngine::apply_activity(input_engine)` in `src/weave/mod.rs` to set each weave action's activity from its slot's active flag (FR-002). Run US3 tests to green.

## Phase 6: User Story 4 - Configurable timing and persistence (P3)

**Goal**: FR-008, FR-009, FR-015, FR-016.

**Independent test**: A per-slot override is applied; slot and timing config
round-trips through settings; a missing or out-of-range value falls back with a
notice.

- [x] T011 [P] [US4] Write failing tests in `tests/weave_engine.rs`: `WeaveConfig` stores to settings and loads back unchanged through a temp config dir; a settings file with an out-of-range timing value falls back to the default and returns a notice; an unknown weave type falls back with a notice.
- [x] T012 [US4] Implement `WeaveConfig::load(settings)`/`store(settings)` in `src/weave/mod.rs` wired to the T004 config helpers (FR-015, FR-016). Run US4 tests to green.

## Phase 7: Real synthesis adapters (thin)

- [x] T013 Implement `RealSink` in `src/weave/mod.rs`: `emit` drives Input Engine key and mouse synthesis, `wait` sleeps the worker thread, `now_ms` reads a monotonic clock.
- [x] T014 Add mouse synthesis to the OS backends: `synthesize_mouse` in `src/input/windows.rs` (SendInput mouse events) and `src/input/linux.rs` (uinput BTN_LEFT and BTN_RIGHT). Windows compiles clippy-clean on the host; Linux is type-checked with the linux target.

## Phase 8: Polish and cross-cutting

- [x] T015 [P] Add module and item documentation across `src/weave/*.rs`.
- [x] T016 Update `CHANGELOG.md` `[Unreleased]`: an Added line for the Weave Engine (no new dependencies; mouse synthesis reuses existing target deps).
- [x] T017 Run CI parity in the foreground: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked`; plus `cargo check --target x86_64-unknown-linux-gnu`.

## Dependencies and order

- Setup (T001) then Foundational (T002 to T004) first.
- US1 (T005, T006) is the pure core; US2 (T007, T008) adds the engine and cooldown;
  US3 (T009, T010) and US4 (T011, T012) build on them.
- Real adapters (T013, T014) depend on the core and the input synthesis additions.
- Polish (T015 to T017) last; T017 is the CI parity gate.

## Parallel opportunities

- The per-story test tasks (T005, T007, T009, T011) can be drafted in parallel;
  T007/T009/T011 share `tests/weave_engine.rs` so land them sequentially.
- T014 Windows and Linux mouse synthesis touch different files and can proceed in
  parallel.

## MVP scope

User Story 1 (correct sequences) plus Setup and Foundational is the minimum viable
increment: the pure sequence builder verified for all four weave types. Cooldown,
inactive pass-through, persistence, and the real adapters follow.
