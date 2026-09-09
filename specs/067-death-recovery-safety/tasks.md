# Tasks: Death Recovery Safety

**Input**: Design documents from `specs/067-death-recovery-safety/`

## Phase 1: Specification and governance

- [x] T001 Create the complete S067 spec-kit artifact chain and pre-implementation analysis.
- [x] T002 Archive plan 036, establish plan 037, and update both plan indexes and migration ledger.
- [x] T003 Move issue #109 to In progress and assign Slice S067 in the Delivery Project.

## Phase 2: Failing safety tests

- [x] T004 Add failing Lua contract tests for Reincarnated registration, death-episode recovery, and managed manifest version.
- [x] T005 Add failing B21 decoder and recovery-capture ordering tests in `tests/pixelbus.rs`.
- [x] T006 Add failing death-epoch and queued/running weave cancellation tests in input and weave suites.
- [x] T007 Add failing deadline cancellation tests for every Fishing phase in `tests/fishing.rs`.
- [x] T008 Add the exact expired-retry recovery reproduction and fresh-baseline tests in `tests/potion.rs` and routing tests.
- [x] T009 Add failing recovery-path presentation tests in app view-model suites.

## Phase 3: Addon recovery authority

- [x] T010 Implement the idempotent PixelBeacon death-episode arbiter and path-specific completion rules.
- [x] T011 Register `EVENT_PLAYER_REINCARNATED`, retain bounded query convergence, and rebaseline without immediate Alive.
- [x] T012 Encode diagnostic recovery paths in B21 and advance the managed addon manifest version.

## Phase 4: Companion ordering and authorization

- [x] T013 Replace Reincarnating with typed Recovering paths in pixelbus, controllers, and presentation.
- [x] T014 Add reader sample generation and force a recovered capture to refresh action-driving observations before Alive.
- [x] T015 Add a diagnostic death epoch while preserving existing atomic authorization epochs and pre-lock closure.
- [x] T016 Verify queued and running weave work cannot cross a death close-reopen cycle.

## Phase 5: Autonomous controller recovery

- [x] T017 Confirm Fishing cancels every deadline and requires a fresh recovery observation without replay.
- [x] T018 Start a new auto-potion retry episode on coherent recovery and require fresh recovery-capture inputs.
- [x] T019 Add state/path/epoch/generation diagnostics without personal data.

## Phase 6: Documentation and analysis

- [x] T020 Update canonical safety, observation, state-machine, status, protocol, and testing pages.
- [x] T021 Add the S067 Changed and dated Decisions entries to `CHANGELOG.md`.
- [x] T022 Complete the runtime checklist and post-implementation analyze gate.
- [x] T023 Run documentation policy, links, UTF-8, punctuation, and mojibake checks.

## Phase 7: Validation and delivery

- [x] T024 Run `cargo fmt --all -- --check` in the foreground.
- [x] T025 Run `cargo clippy --all-targets --all-features -- -D warnings` in the foreground.
- [x] T026 Run `cargo test --all --locked` in the foreground.
- [ ] T027 Commit with a `feat(067)` message and Co-Authored-By trailer.
- [ ] T028 Push the authorized branch, open the official PR with `Closes #109`, and update Project stage.
- [ ] T029 Resolve every CI and external review finding, with at most one authorized second `@Codex` round.

## Dependencies and Execution Order

Specification and governance precede tests. Tests fail before implementation.
Addon recovery authority precedes companion reopening rules. Controller recovery
uses the completed capture-order contract. Documentation, analysis, and full
validation precede commit and remote publication. Issue #110 remains outside the
slice and open in Release verification.
