# Tasks: Roll-Dodge Safety

**Input**: Design documents from `specs/050-roll-dodge-safety/`

**Tests**: Required for wire values, lifecycle, watchdog, protocol compatibility,
routing, hook pass-through, worker and sink cancellation, non-replay, and presentation.

## Phase 1: Spec-kit setup

- [x] T001 Synchronize `main` and create `codex/050-roll-dodge-safety`
- [x] T002 Bind atomic issues #57 and #60 to S050
- [x] T003 Complete specification and requirements checklist
- [x] T004 Resolve the current ESO event contract and watchdog through clarification
- [x] T005 Complete research, model, contract, quickstart, and safety checklist
- [x] T006 Add chronological build plan 020 and update the build-plan index
- [x] T007 Complete pre-implementation analysis and resolve every finding

## Phase 2: Test-first contracts

- [x] T008 Write failing B23 decoder, point, block-count, protocol-version, registry,
  and cross-language agreement tests
- [x] T009 Write failing reader transition, invalid-sample, absence, duplicate, and
  signal-loss tests
- [x] T010 Write failing addon event, lifecycle, activation-baseline, and 1,500 ms
  watchdog contract tests
- [x] T011 Write failing routing, input-classifier pass-through, and toggle-exemption tests
- [x] T012 Write failing worker drop, non-replay, mid-sequence cancellation, and recovery tests
- [x] T013 Write failing roll-dodge view and dormant presentation tests
- [x] T014 Run focused tests and record the expected failures

## Phase 3: Addon and reader

- [x] T015 Add B23 constants, state, block construction, rendering, and payload order
- [x] T016 Register filtered ability 28549 gained/faded observation
- [x] T017 Implement deadline arming, completion cancellation, watchdog recovery,
  and lifecycle invalidation
- [x] T018 Advance layout protocol to version 3, preserving version 1 and version 2 extents
- [x] T019 Advance the managed addon manifest to version 17
- [x] T020 Add RollDodgeState, decoder, sample field, point, reader state, event,
  and signal-loss clearing to the companion

## Phase 4: Generated-weave gate and interface

- [x] T021 Add a reusable atomic gate core and typed roll gate to InputEngine
- [x] T022 Close the roll gate before controller locks and open it after worker synchronization
- [x] T023 Store and recheck roll state in WeaveEngine before cooldown accounting
- [x] T024 Cancel running generated sequences across both life and roll gates
- [x] T025 Wire gate handles in startup and preserve fishing's life-only dependency
- [x] T026 Add RollDodgeView, strings, Live HUD row, and dormant override

## Phase 5: Documentation and validation

- [x] T027 Update master protocol, architecture, README, and addon description
- [x] T028 Add the S050 changelog entry and architecture decision
- [x] T029 Re-run spec-kit analysis after implementation
- [x] T030 Run all focused contract, lifecycle, routing, input, weave, and view tests
- [x] T031 Run UTF-8, BOM, forbidden-dash, mojibake, whitespace, and diff audits
- [x] T032 Run `cargo fmt --all -- --check`
- [x] T033 Run `cargo clippy --all-targets --all-features -- -D warnings`
- [x] T034 Run `cargo test --all --locked`
- [x] T035 Inspect the final diff for scope, secrets, generated files, and unrelated changes
- [ ] T036 Commit as `feat(050): gate generated weaves during roll dodge` with attribution
- [ ] T037 Push the branch and open a pull request with `Closes #57` and `Closes #60`
- [ ] T038 Move #57 and #60 to PR review and verify hosted checks
- [ ] T039 Address every first-round review comment and CI failure
- [ ] T040 Trigger at most one `@Codex review` second round and address every result
- [ ] T041 Confirm checks green and threads resolved, then request final merge ritual

## Dependencies and execution order

- The spec-kit analysis gate precedes implementation.
- Failing tests T008 through T013 precede product changes.
- Addon publication and reader decoding precede routing and presentation.
- Input hook, worker, and real sink each enforce the same state at their race boundary.
- No task changes auto-potion, fishing, sprint, effects, or remapping behavior.
