# Tasks: Fishing Reliability and Status Collaboration

**Feature**: 016-fishing-reliability
**Spec**: [spec.md](spec.md) | **Plan**: [plan.md](plan.md)

Test-first discipline: for each behavior, the failing unit test is written before
the implementation that satisfies it. Safety-critical behaviors keep their tests.
CI parity runs in the foreground before the commit.

## Phase 1: Setup

- [x] T001 Confirm a green baseline before changes: run `cargo test --all --locked` and note it passes, so later failures are attributable to this feature.

## Phase 2: Foundational

No foundational-only tasks. The three user stories touch different surfaces
(worker loop and fishing config for US1, the addon manifest for US2, the fishing
controller and view-model for US3) and are independently implementable and
testable.

## Phase 3: User Story 1 - Reliable hands-free fishing (Priority: P1)

**Goal**: A cast progresses to a catch and reel-in and does not falsely revert to
idle, by polling at the fishing cadence, judging deadlines on one clock, and
giving the arm window adequate margin.

**Independent test**: With the addon loaded, turn on fishing at a hole and observe
the routine advance past casting into waiting for a bite, reel a catch, and
recast.

### Cadence

- [x] T002 [US1] Write a failing unit test for a `poll_interval(fishing_active: bool, cfg: &ReaderConfig) -> u64` helper in `src/pixelbus/mod.rs`: returns `interval_fishing_ms` when active and `interval_idle_ms` when not.
- [x] T003 [US1] Implement the `poll_interval` pure helper in `src/pixelbus/mod.rs` to satisfy T002.
- [x] T004 [US1] Wire the pixel-bus worker loop in `src/main.rs` to compute each iteration's sleep from `poll_interval`, deriving `fishing_active` from the current fishing state (state is not `Disabled`), replacing the hardcoded `interval_idle_ms` sleep.

### Clock unification

- [x] T005 [US1] Write a failing test that a fishing deadline set through the app intent path is evaluated against the same monotonic origin the worker `tick()` uses (exercise via the shared clock abstraction, not two independent `Instant`s).
- [x] T006 [US1] Introduce a single shared monotonic origin created at startup and use it for both the GUI clock (`now_ms` in `src/app/mod.rs`) and the worker `tick()` timeline (`src/main.rs`), so fishing deadlines are stamped and judged on one clock. Keep the change minimal.

### Timeout defaults

- [x] T007 [US1] Write a failing test in `src/fishing/mod.rs` asserting `FishingConfig::default` has `arm_timeout_ms == 8000`, `reel_delay_ms == 100`, and `recast_delay_ms == 3000`.
- [x] T008 [US1] Set `arm_timeout_ms` default to 8000 in `FishingConfig::default` in `src/fishing/mod.rs` (leave reel and recast defaults unchanged) to satisfy T007.

## Phase 4: User Story 2 - Companion addon recognized as current (Priority: P1)

**Goal**: The game stops flagging the addon out of date and loads it, and existing
installs are refreshed.

**Independent test**: On the live client, install or refresh the addon through the
app, reload the UI, and confirm the AddOns list shows it as current.

- [x] T009 [P] [US2] Write failing assertions in `tests/beacon.rs`: the embedded manifest parses to version 3, its `## APIVersion` is present and includes a value at least the targeted live value (101050), and the managed-marker line is present.
- [x] T010 [US2] Edit `addon/PixelBeacon/PixelBeacon.txt`: set `## Version` and `## AddOnVersion` to 3 and `## APIVersion` to `101050 101054` (confirm 101050 is still live first). Do not modify the `## X-ESO-Weave-Managed: true` line.
- [x] T011 [US2] Update any existing beacon test that hardcoded the embedded version as 2 to expect 3, and confirm the managed-marker gating and version-compare tests still pass.

## Phase 5: User Story 3 - Clear fishing status and stop reasons (Priority: P2)

**Goal**: The app shows plain-language status and, when it stops, why.

**Independent test**: Drive the controller through each state and stop path and
confirm the derived status text matches the status contract with no internal
state names.

### Stop reason

- [x] T012 [US3] Write failing unit tests in `src/fishing/mod.rs`: after an arm timeout the controller records `NoCastDetected`; after `SignalLost` it records `SignalLost`; after a user stop it records `UserStop`; and a new cast clears the reason.
- [x] T013 [US3] Add a `StopReason` enum and an `Option<StopReason>` field on the fishing controller; set it in `disable()` (carry the reason from each caller: user stop, arm-timeout, signal lost) and clear it in `cast()`. Preserve the SignalLost-cancels-pending-interact behavior unchanged.

### Status derivation

- [x] T014 [P] [US3] Add the fishing status and label strings to `src/app/strings.rs`: Casting, Fishing (waiting for a bite), Reeling in, Recasting, Idle, Idle (no cast detected), Idle (signal lost), plus updated tooltips.
- [x] T015 [US3] Write failing unit tests for `fishing_label()` and `status_line_fishing()` in `src/app/mod.rs` covering every row of the status contract table (each state and each stop reason maps to its plain-language indicator; no internal state names).
- [x] T016 [US3] Implement the plain-language derivation in `fishing_label()` and `status_line_fishing()` in `src/app/mod.rs`, reading the fishing state and the `StopReason`, to satisfy T015.

## Phase 6: Polish and Cross-Cutting

- [x] T017 Confirm the safety test that SignalLost cancels any pending interact is present and green (in `src/fishing` tests); add it if missing.
- [x] T018 Update `CHANGELOG.md` `[Unreleased]`: an Added line for the fishing reliability and status-collaboration work, and a dated Decisions entry recording the addon `## APIVersion` change to 101050 101054 (closing R4), the `## Version`/`## AddOnVersion` bump to 3, the arm-timeout default change to 8000 ms, and the fishing clock unification.
- [x] T019 Run CI parity in the foreground and watch to completion: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked`.

## Dependencies and Order

- Phase 1 (T001) first.
- User stories are independent and may be done in any order; recommended order is
  US1 (MVP reliability), then US2 (addon), then US3 (status).
- Within each story, the test task precedes its implementation task
  (T002 before T003; T005 before T006; T007 before T008; T009 before T010; T012
  before T013; T015 before T016).
- T004 depends on T003. T011 depends on T010. T016 depends on T013 and T014.
- Phase 6 runs last; T019 is the final gate before commit.

## Parallel Opportunities

- T009 (beacon test) and T014 (strings) are marked [P]: different files, no
  dependency on other incomplete tasks in their story.
- US2 (T009 to T011) can proceed in parallel with US1 and US3 since it touches
  only the manifest and beacon tests.

## MVP Scope

User Story 1 (T002 to T008) plus User Story 2 (T009 to T011) together deliver the
reliability fix: the addon loads and the routine polls and reels correctly. User
Story 3 adds the status collaboration on top.
