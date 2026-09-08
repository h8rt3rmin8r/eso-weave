# Tasks: Settings Runtime Parity

## Phase 1: Spec-kit and Evidence

- [x] T001 Synchronize `main`, create `codex/s062-settings-runtime-parity`, and select issue #95.
- [x] T002 Explore settings persistence, runtime ownership, worker timing, UI, tests, and documentation.
- [x] T003 Create the S062 specification and requirements checklist.
- [x] T004 Resolve application and cancellation policy through the autopilot decision rules.
- [x] T005 Create runtime-application and interface-copy domain checklists.
- [x] T006 Create research, data model, contracts, quickstart, and plan.
- [x] T007 Run and pass pre-implementation analysis.

## Phase 2: Fishing Runtime Parity

- [x] T008 [US1] Add failing tests for changed and unchanged Fishing reconfiguration across disabled, requested, and active phases.
- [x] T009 [US1] Add failing AppModel tests proving effective Fishing settings apply before restart and cancel requested work safely.
- [x] T010 [US1] Implement atomic Fishing reconfiguration with no-op equality and settings-change stop reason.
- [x] T011 [US1] Apply the same sanitized Fishing value to persistence and the shared live controller.
- [x] T012 [US1] Re-run safety-gate, cancellation, deadline, session, and no-replay regressions.

## Phase 3: Pixel Bus Runtime Parity

- [x] T013 [US2] Add failing tests for live-subset merge, interval selection, tolerance invalidation, and preserved block/heartbeat values.
- [x] T014 [US2] Add failing worker-update tests for wake, rapid-update coalescing, no-op updates, and disconnect behavior.
- [x] T015 [US2] Add the complete-value reader update port without a new dependency or thread.
- [x] T016 [US2] Replace the worker sleep with update-aware waiting and apply the newest config at one iteration boundary.
- [x] T017 [US2] Close and route stale tolerance-dependent safety evidence before fresh sampling.
- [x] T018 [US2] Preserve startup block geometry, internal heartbeat timeout, lock ordering, and reader reset semantics.

## Phase 4: Settings Interface and Copy

- [x] T019 [US3] Add failing headless UI and string tests for Fishing Interact Key and per-setting timing claims.
- [x] T020 [US3] Add the Fishing Interact Key combo from `Key::ALL` with stable label and ID.
- [x] T021 [US3] Align numeric widget bounds with loader bounds and keep effective persisted values sanitized.
- [x] T022 [US3] Correct blanket live-apply comments and staged Block Size help without broadening the save toast.
- [x] T023 [US3] Update canonical settings, Fishing, PixelBeacon, interface, safety, architecture, and test documentation.

## Phase 5: Documentation and Lifecycle

- [x] T024 Convert deferred #95 coverage to implemented evidence and update policy fixtures and digest.
- [x] T025 Archive plan 031 with PR #99 evidence and establish active plan 032.
- [x] T026 Update the migration ledger, plan indexes, and `.specify/feature.json` lifecycle state.
- [x] T027 Add S062 changes and the dated application-contract decision to `CHANGELOG.md`.

## Phase 6: Analyze, Validate, and Review

- [x] T028 Complete both domain checklists and run focused S062 tests.
- [x] T029 Run and pass post-implementation spec-kit analysis.
- [x] T030 Run format, strict Clippy, full locked tests, docs policy, mdBook, spelling, text hygiene, whitespace, and mojibake gates.
- [x] T031 Complete independent runtime, safety, UI, test, documentation, and spec-kit reviews; resolve all findings at 80% confidence or higher.

## Phase 7: Delivery

- [x] T032 Commit as `fix(062): apply settings runtime contracts` with attribution.
- [x] T033 Push and open an official PR with `Closes #95`.
- [x] T034 Move issue #95 to PR Review and monitor every CI check and review.
- [x] T035 Resolve every first-round hosted review finding.
- [x] T036 Trigger exactly one authorized second `@Codex review` and resolve every result.
- [ ] T037 Confirm green checks, no unresolved review threads, and merge readiness before requesting the merge ritual.

## Dependencies

- T007 blocks implementation.
- T008 and T009 precede T010 and T011.
- T013 and T014 precede T015 through T018.
- T019 precedes T020 through T022.
- T024 through T027 require stable implemented behavior.
- T028 through T031 block publication.
- T033 and the single second review request are pre-authorized by the user.

## Parallel Opportunities

- Fishing controller and reader update tests can be developed independently after analysis.
- UI copy and documentation can proceed after runtime semantics stabilize.
- Runtime, safety, UI, test, documentation, and spec-kit review passes are independent.
