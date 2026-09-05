# Tasks: World Transition State

**Input**: Design documents from `specs/049-world-transition-state/`

**Tests**: Required for wire values, lifecycle ordering, signal loss, routing,
process exit, and presentation.

## Phase 1: Spec-kit setup

- [x] T001 Synchronize `main` and create `codex/049-world-transition-state`
- [x] T002 Bind atomic issue #56 to S049
- [x] T003 Complete the specification and requirements checklist
- [x] T004 Resolve clarification choices through the autopilot decision policy
- [x] T005 Complete research, model, contract, quickstart, and lifecycle checklist
- [x] T006 Add chronological build plan 019 and update the build-plan index

## Phase 2: Test-first contracts

- [x] T007 Write failing B22 decoder, point, block-count, geometry, registry, and
  cross-language agreement tests
- [x] T008 Write failing reader transition, invalid-sample, absence, duplicate,
  and signal-loss tests
- [x] T009 Write failing addon lifecycle and complete-baseline ordering tests
- [x] T010 Write failing shared game-state routing and process-exit tests
- [x] T011 Write failing World state view and dormant presentation tests
- [x] T012 Run focused tests and record the expected failures

## Phase 3: Addon and reader

- [x] T013 Add B22 constants, state, block construction, rendering, and payload order
- [x] T014 Register deactivation and activation lifecycle handlers
- [x] T015 Extract the complete player baseline and publish Active only afterward
- [x] T016 Advance the managed addon manifest to version 16
- [x] T017 Add WorldState, decoder, sample field, point, reader state, event, and
  signal-loss clearing to the companion

## Phase 4: Shared model and interface

- [x] T018 Store WorldState in GameObservations and clear it on inactive runtime
- [x] T019 Route reader world-state transitions into the shared game model
- [x] T020 Add WorldStateView, strings, System and State row, and dormant override

## Phase 5: Documentation and validation

- [x] T021 Update master protocol, architecture diagram, README, and addon description
- [x] T022 Add the S049 changelog entry and architecture decision
- [x] T023 Complete pre-implementation `analysis.md` and resolve every finding
- [x] T024 Re-run spec-kit analysis after implementation
- [x] T025 Run all focused contract, lifecycle, routing, and view tests
- [x] T026 Run UTF-8, BOM, forbidden-dash, mojibake, whitespace, and diff audits
- [x] T027 Run `cargo fmt --all -- --check`
- [x] T028 Run `cargo clippy --all-targets --all-features -- -D warnings`
- [x] T029 Run `cargo test --all --locked`
- [x] T030 Inspect the final diff for scope, secrets, generated files, and unrelated changes
- [ ] T031 Commit as `feat(049): publish world transition state` with attribution
- [ ] T032 Push the branch and open a pull request with `Closes #56`
- [ ] T033 Move #56 to PR review and verify hosted checks
- [ ] T034 Address every first-round review comment and CI failure
- [ ] T035 Trigger at most one `@Codex review` second round and address every result
- [ ] T036 Confirm checks green and threads resolved, then request final merge ritual

## Dependencies and execution order

- The spec-kit analysis gate precedes implementation.
- Failing tests T007 through T011 precede product changes.
- Addon publication and reader decoding precede routing and presentation.
- Transitioning is event-driven; Active follows the complete baseline.
- No task changes synthesized-input behavior.
