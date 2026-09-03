# Tasks: Game Runtime and Context Truth

**Input**: Design documents from `specs/041-game-runtime-context/`

**Tests**: Required. S041 changes safety-critical input gating and follows strict
red-green-refactor discipline.

## Phase 1: Setup

**Purpose**: Establish the slice boundary and preserve a green baseline.

- [x] T001 Run the clean-tree and baseline test preflight from `specs/041-game-runtime-context/quickstart.md`
- [x] T002 Add the S041 module and required existing dependency features in `Cargo.toml` and `src/lib.rs`
- [x] T003 Move the Steam VDF parser from `src/beacon/steam.rs` to `src/game/steam.rs` and update `src/beacon/linux.rs`

---

## Phase 2: Foundational Observation Model

**Purpose**: Create the pure contracts shared by all three user stories.

- [x] T004 Write failing installation reconciliation tests in `tests/game_state.rs`
- [x] T005 Write failing runtime reduction and Game Context precedence tests in `tests/game_state.rs`
- [x] T006 Implement installation, runtime, focus, freshness, surface, and context types in `src/game/mod.rs`
- [x] T007 Implement normalized provider reconciliation and transition-only shared state updates in `src/game/mod.rs`
- [x] T008 [P] Write failing Steam app-manifest parsing tests in `tests/game_state.rs`
- [x] T009 [P] Write failing Epic manifest parsing and artifact-validation tests in `tests/game_state.rs`
- [x] T010 Implement Steam, Epic, and root-validation helpers in `src/game/steam.rs` and `src/game/mod.rs`
- [x] T011 Implement Windows registry, manifest, process, and focus observations in `src/game/windows.rs`
- [x] T012 Implement Linux Steam Proton, `/proc`, and X11 focus observations in `src/game/linux.rs` and `src/platform/linux.rs`
- [x] T013 Delegate the legacy addon lifecycle running probe to `src/game/` from `src/beacon/mod.rs`, `src/beacon/windows.rs`, and `src/beacon/linux.rs`

**Checkpoint**: Pure reducers and both platform adapters compile, and provider
fixtures prove reconciliation without using personal paths.

---

## Phase 3: User Story 1 - See Whether ESO Is Available and Running (Priority: P1)

**Goal**: Report installation and runtime independently with correct launcher and
game precedence.

**Independent Test**: The lifecycle matrix converges to the expected installation
and runtime state, including launcher exit during active play.

- [x] T014 [US1] Write failing monitor change-detection and lifecycle transition tests in `tests/game_state.rs`
- [x] T015 [US1] Implement the one-second monitor and platform probe entry point in `src/game/mod.rs`
- [x] T016 [US1] Wire the monitor into the existing sampling worker before sampler availability checks in `src/main.rs`
- [x] T017 [US1] Add installation and runtime projections to `src/app/mod.rs` and their copy to `src/app/strings.rs`
- [x] T018 [US1] Render the separate Game installation/provider and runtime rows in `src/app/ui.rs`
- [x] T019 [US1] Add lifecycle view-model and rendered-frame coverage in `tests/app_view_model.rs` and `tests/app_ui_sizing.rs`

**Checkpoint**: No-install, installed inactive, launcher open, Active, launcher
exit, game exit, ambiguity, and probe failure are independently testable.

---

## Phase 4: User Story 2 - See Truthful Game Context (Priority: P1)

**Goal**: Replace Game Menu with Game Context and never turn missing evidence
into Gameplay.

**Independent Test**: Every runtime x focus x freshness x surface combination
maps to one reviewed value and only the authoritative no-menu combination maps
to Gameplay.

- [x] T020 [US2] Write failing optional menu-decode and invalid-sample tests in `tests/pixelbus.rs`
- [x] T021 [US2] Change menu decoding and reader state to preserve unavailable separately from valid `MenuSurface::None` in `src/pixelbus/mod.rs`
- [x] T022 [US2] Extend reader routing with shared freshness and surface observations in `src/app/routing.rs`
- [x] T023 [US2] Replace the menu projection with Game Context status roles and reviewed labels in `src/app/mod.rs` and `src/app/strings.rs`
- [x] T024 [US2] Render Game Context with identical hover and keyboard-focus help in `src/app/ui.rs`
- [x] T025 [US2] Add reader, routing, context matrix, tooltip, and rendered-frame coverage in `tests/pixelbus.rs`, `tests/app_view_model.rs`, and `tests/app_ui_sizing.rs`

**Checkpoint**: Gameplay is impossible without Active, Focused, Fresh, and a
valid observed no-menu surface.

---

## Phase 5: User Story 3 - Enter and Recover From Dormancy Safely (Priority: P1)

**Goal**: Clear stale observations and prevent input while the game is inactive,
then recover without restarting ESO Weave.

**Independent Test**: Runtime exit resets the reader and every game-derived
projection, blocks input and auto-potion without resetting its runtime request,
and permits fresh values after restart.

- [x] T026 [US3] Write failing game-inactive input matrix tests in `tests/input_engine.rs`
- [x] T027 [US3] Add the explicit game-active classification gate in `src/input/mod.rs` and update platform-independent test setup
- [x] T028 [US3] Write failing game-inactive auto-potion and request-preservation tests in `tests/potion.rs`
- [x] T029 [US3] Add the game-active auto-potion gate in `src/potion/mod.rs` without changing signal-loss reset policy
- [x] T030 [US3] Write failing reader-reset, weave-clear, and dormant projection tests in `tests/pixelbus.rs`, `tests/weave_engine.rs`, and `tests/app_view_model.rs`
- [x] T031 [US3] Implement reader reset and aggregate observation clearing in `src/pixelbus/mod.rs` and `src/weave/mod.rs`
- [x] T032 [US3] Route Active exit, focus loss, and restart through one lock order, pause active fishing with an explicit reason, preserve fishing and auto-potion request state, and clear transient input gates in `src/main.rs`, `src/fishing/mod.rs`, `src/potion/mod.rs`, and `src/input/mod.rs`
- [x] T033 [US3] Apply the shared dormant presentation to every pre-Skills live metric in `src/app/mod.rs` and `src/app/ui.rs`
- [x] T034 [US3] Complete restart recovery, game-inactive fishing shutdown, and safety regression coverage in `tests/app_view_model.rs`, `tests/input_engine.rs`, `tests/potion.rs`, and `tests/fishing.rs`

**Checkpoint**: Runtime inactivity produces zero input attempts, all live fields
are dormant within the poll bound, and restart republishes unchanged values.

---

## Phase 6: Documentation, Field Receipt, and Validation

**Purpose**: Align the architecture of record and prove the shipped slice.

- [x] T035 Update runtime, input, PixelBeacon, UI, and terminology sections in `docs/ESO-Weave-Specification.md`
- [x] T036 Update setup and troubleshooting guidance in `README.md`
- [x] T037 Record the feature and architecture decisions under `[Unreleased]` in `CHANGELOG.md`
- [x] T038 Perform the sanitized Windows lifecycle and provider receipt from `specs/041-game-runtime-context/quickstart.md`
- [x] T039 Run UTF-8 no-BOM, LF, forbidden-dash, placeholder, and mojibake checks across all S041 files
- [x] T040 Run `cargo fmt --all -- --check` in the foreground
- [x] T041 Run `cargo clippy --all-targets --all-features -- -D warnings` in the foreground
- [x] T042 Run `cargo test --all --locked` in the foreground
- [x] T043 Reconcile every S041 acceptance criterion and mark all tasks complete in `specs/041-game-runtime-context/tasks.md`

---

## Dependencies and Execution Order

- Phase 1 precedes all implementation.
- Phase 2 blocks every user story.
- User Story 1 establishes runtime before User Story 2 projects Game Context.
- User Story 3 consumes User Stories 1 and 2 to enforce dormancy and recovery.
- Documentation and validation run only after all three stories pass.

## Parallel Opportunities

- T008 and T009 are independent parser test sets after the shared types exist.
- Windows adapter T011 and Linux adapter T012 touch separate files after T010.
- Documentation T035 and T036 touch separate files after implementation settles.

## Implementation Strategy

The minimum useful increment is User Story 1 on top of the foundational model,
but the slice is complete only when all three stories close issues #22 and #23.
Each story starts with failing tests, implements the smallest passing contract,
and preserves the prior story's green state.
