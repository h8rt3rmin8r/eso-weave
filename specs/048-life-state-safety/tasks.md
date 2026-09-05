# Tasks: Life State Safety

**Input**: Design documents from `specs/048-life-state-safety/`

**Tests**: Required for the wire contract, every synthesis boundary, recovery,
disclosure persistence, accessibility, and geometry.

## Phase 1: Spec-kit setup

- [x] T001 Synchronize `main` and create `codex/048-life-state-safety`
- [x] T002 Bind atomic issues #53, #54, #55, and #58 to S048
- [x] T003 Complete the feature specification and requirements checklist
- [x] T004 Resolve clarification choices through the autopilot decision policy
- [x] T005 Complete research, model, contract, quickstart, and safety checklist
- [x] T006 Add chronological build plan 018 and update the build-plan index

## Phase 2: Test-first contracts

- [x] T007 Write failing B21 decoder, reader transition, signal-loss, geometry,
  marker-registry, and cross-language agreement tests
- [x] T008 Write failing input, queued weave, fishing, and auto-potion gate tests
- [x] T009 Write failing life-state view and dormant-state tests
- [x] T010 Write failing disclosure settings, accessibility, persistence, and
  narrow and wide geometry tests
- [x] T011 Run focused tests and record the expected failures

## Phase 3: Addon and reader

- [x] T012 Add the B21 life block, shared constants, event handlers, activation
  re-baseline, and convergence tick to PixelBeacon
- [x] T013 Advance the manifest and describe the life-state signal
- [x] T014 Add `LifeState`, decoder, sample field, point, reader state, event, and
  signal-loss clearing to the companion
- [x] T015 Route life state into the shared observation model

## Phase 4: Safety consumers

- [x] T016 Add fail-closed life gating to physical input classification while
  preserving toggle exemptions
- [x] T017 Re-check life state in the weave worker before cooldown bookkeeping
- [x] T018 Cancel rather than defer fishing work while non-Alive, preserving the
  request toggle without replay
- [x] T019 Add an explicit life-state auto-potion blocker before resource logic
- [x] T020 Route one life event to all four consumers and verify signal-loss order

## Phase 5: Interface and persistence

- [x] T021 Add truthful Life state presentation to Live HUD and blocker text
- [x] T022 Rename every System and automation reference to System and State
- [x] T023 Add the full-row accessible disclosure with a visible chevron
- [x] T024 Persist `system_state_expanded` in UI preferences with default-open migration
- [x] T025 Recompute intrinsic layout correctly while collapsed at both breakpoints

## Phase 6: Integration and validation

- [x] T026 Add the S048 changelog entry and architecture decision
- [x] T027 Complete `analysis.md` and resolve every spec-kit finding
- [x] T028 Run all focused contract, controller, routing, and rendered-frame tests
- [x] T029 Run UTF-8, BOM, forbidden-dash, mojibake, whitespace, and diff audits
- [x] T030 Run `cargo fmt --all -- --check`
- [x] T031 Run `cargo clippy --all-targets --all-features -- -D warnings`
- [x] T032 Run `cargo test --all --locked`
- [x] T033 Inspect the final diff for scope, secrets, generated files, and unrelated changes
- [ ] T034 Commit as `feat(048): enforce player life-state safety` with attribution
- [ ] T035 Push the feature branch and open a pull request with four closing keywords
- [ ] T036 Move all four issues to PR review and verify hosted checks
- [ ] T037 Address every first-round review comment and CI failure
- [ ] T038 Trigger at most one `@Codex review` second round and address every result
- [ ] T039 Confirm checks green and threads resolved, then request final merge ritual

## Dependencies and execution order

- The spec-kit analysis gate precedes implementation.
- Failing tests T007 through T010 precede product changes.
- The reader and routing signal precede consumer gates and view presentation.
- Only Alive opens the gate; Heartbeat alone never does.
- Recovery never calls a sink and never replays blocked work.
