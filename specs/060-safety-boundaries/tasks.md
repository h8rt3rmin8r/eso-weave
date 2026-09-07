# Tasks: Safety Boundaries

## Phase 1: Spec-kit and Evidence

- [x] T001 Synchronize `main`, create `codex/s060-safety-boundaries`, and select issues #92 and #94.
- [x] T002 Explore weave, Fishing, PixelBeacon, UI, tests, and documentation in parallel.
- [x] T003 Create the S060 specification and requirements checklist.
- [x] T004 Resolve clarification through the autopilot decision policy.
- [x] T005 Create authorization and ownership domain checklists.
- [x] T006 Create research, data model, contracts, quickstart, and plan.
- [x] T007 Run and pass pre-implementation analysis.

## Phase 2: Shared Authorization Foundation

- [x] T008 [US1] Add failing input tests for shared focus, suspension, menu, and epoch transitions in `tests/input_engine.rs`.
- [x] T009 [US1] Add failing queue-admission and close-reopen tests in `tests/weave_engine.rs` and worker-test seams.
- [x] T010 [US1] Implement typed shared gate projections, monotonic invalidation, and queued action epochs in `src/input/mod.rs`.
- [x] T011 [US1] Route close-before-lock and open-after-sync transitions through `src/main.rs` and `src/app/routing.rs`.
- [x] T012 [US1] Extend `RealSink` admission, wait, and emission checks in `src/weave/mod.rs`.
- [x] T013 [US1] Add the light, heavy, bash, cooldown, and held-release cancellation matrix in `tests/weave_engine.rs`.

## Phase 3: Fishing Suspension Safety

- [x] T014 [US2] Add failing suspended-enable, active-state, deadline, and no-replay tests in `tests/fishing.rs`.
- [x] T015 [US2] Add a suspended stop reason and fail-safe recovery state transitions in `src/fishing/mod.rs`.
- [x] T016 [US2] Make the real Fishing emission boundary gate-aware and prevent state advancement after rejected output.
- [x] T017 [US2] Route suspension into Fishing from UI and hotkey paths in `src/app/mod.rs` and startup/runtime wiring in `src/main.rs`.
- [x] T018 [US2] Extend view-model tests for suspension routing and status presentation in `tests/app_view_model.rs`.
- [x] T019 [US2] Re-run menu, focus, life, world, travel, signal-loss, and physical pass-through regression tests.

## Phase 4: PixelBeacon Ownership Safety

- [x] T020 [US3] Add failing ownership classification and no-mutation target-shape tests in `tests/beacon.rs`.
- [x] T021 [US3] Implement non-following ownership inspection and guarded install/update writers in `src/beacon/mod.rs`.
- [x] T022 [US3] Verify API update, block-size redeploy, and uninstall share the same fail-safe matrix.
- [x] T023 [US3] Add failing distinct-unmanaged state and action-matrix tests in `tests/app_view_model.rs`.
- [x] T024 [US3] Implement `BeaconCondition::Unmanaged`, persistent guidance, and no-action projection in `src/app/beacon_light.rs` and `src/app/mod.rs`.
- [x] T025 [US3] Replace unconditional uninstall-then-install with guarded in-place managed Update.
- [x] T026 [US3] Add stale-intent, target-link, missing-manifest, invalid-manifest, and sibling-preservation tests.

## Phase 5: Documentation and Lifecycle

- [x] T027 Update action authorization, input safety, weaving, Fishing, and PixelBeacon canonical pages.
- [x] T028 Convert S059 deferred records #92 and #94 to covered evidence in `docs/project/content-coverage.json` and update policy fixtures/digest.
- [x] T029 Archive plan 029 with PR #97 evidence and establish active plan 030.
- [x] T030 Update the migration ledger, current/archive plan indexes, and `.specify/feature.json` lifecycle state.
- [x] T031 Add S060 changes and dated architecture decisions to `CHANGELOG.md`.

## Phase 6: Analyze, Validate, and Review

- [x] T032 Run focused S060 tests and complete both domain checklists.
- [x] T033 Run and pass post-implementation spec-kit analysis.
- [x] T034 Run format, strict Clippy, full locked tests, docs policy, mdBook, spelling, text hygiene, and whitespace gates.
- [x] T035 Complete parallel code, safety, platform, and documentation reviews; resolve all findings at 80% confidence or higher.

## Phase 7: Delivery

- [ ] T036 Commit as `fix(060): enforce synthesis and addon ownership gates` with attribution.
- [ ] T037 Push and open an official PR with `Closes #92` and `Closes #94`.
- [ ] T038 Move both issues to PR Review and monitor every CI check and review.
- [ ] T039 Resolve every first-round hosted review finding.
- [ ] T040 Trigger exactly one authorized second `@Codex review` and resolve every result.
- [ ] T041 Confirm green checks, no unresolved review threads, and merge readiness before requesting the merge ritual.

## Dependencies

- T007 blocks implementation.
- T008 and T009 precede T010 through T013.
- T014 precedes T015 through T018.
- T020 and T023 precede T021 through T026.
- T027 through T031 require implemented behavior.
- T032 through T035 block publication.
- T037 is pre-authorized by the user.

## Parallel Opportunities

- Weave and PixelBeacon failing-test tracks may be authored independently after analysis.
- Documentation policy work can begin after runtime behavior stabilizes.
- Code, safety, platform, and documentation review passes are independent.
