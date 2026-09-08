# Tasks: Linux Input and Copy Parity

## Phase 1: Spec-kit and Evidence

- [x] T001 Synchronize `main`, create `codex/s061-linux-input-copy-parity`, and select issues #93 and #96.
- [x] T002 Explore Linux input, menu evidence, behavior copy, tests, and documentation.
- [x] T003 Create the S061 specification and requirements checklist.
- [x] T004 Resolve clarification through the autopilot decision policy.
- [x] T005 Create Linux capability and runtime-copy domain checklists.
- [x] T006 Create research, data model, contracts, quickstart, and plan.
- [x] T007 Run and pass pre-implementation analysis.

## Phase 2: Linux Input Capability Parity

- [x] T008 [US1] Add failing exhaustive mapping and application-capability tests under Linux configuration.
- [x] T009 [US1] Add failing physical capability union and virtual-device upgrade-policy tests without device access.
- [x] T010 [US1] Add `Key::ALL` and replace duplicate key universes in source and tests.
- [x] T011 [US1] Derive Linux capabilities from `Key::ALL`, add E inbound mapping, and include F3 plus mouse controls.
- [x] T012 [US1] Build or replace the virtual device with the physical capability union before keyboard grab.
- [x] T013 [US1] Forward key events only and surface every forwarded-key emission failure.

## Phase 3: Fail-closed Menu Startup

- [x] T014 [US2] Add failing tests for initial input and Fishing gating and zero generated work before menu evidence.
- [x] T015 [US2] Add failing PixelBus tests for first valid gameplay evidence after an unavailable initial state.
- [x] T016 [US2] Initialize input and Fishing menu gates closed and retain Auto Potion's closed default.
- [x] T017 [US2] Publish first valid gameplay evidence, keep unavailable/lost evidence closed, and correct menu diagnostics.
- [x] T018 [US2] Re-run physical pass-through, toggle exemption, Fishing, Auto Potion, and safety routing regressions.

## Phase 4: Behavior Copy

- [x] T019 [US3] Add failing semantic tests for additive latency, global captured logging, persistence, and obsolete phrase rejection.
- [x] T020 [US3] Correct latency and Live Log shipped strings and logging API documentation.
- [x] T021 [US3] Correct stale menu-evidence source comments, diagnostics, and test descriptions.
- [x] T022 [US3] Update canonical settings, logging, authorization, architecture, coverage, and test-strategy pages.

## Phase 5: Documentation and Lifecycle

- [x] T023 Convert deferred #93/#96 records to covered evidence, retain #95, and update policy fixtures/digest.
- [x] T024 Archive plan 030 with PR #98 evidence and establish active plan 031.
- [x] T025 Update the migration ledger, plan indexes, and `.specify/feature.json` lifecycle state.
- [x] T026 Add S061 changes and the dated safety/platform decision to `CHANGELOG.md`.

## Phase 6: Analyze, Validate, and Review

- [x] T027 Complete both domain checklists and run focused S061 tests.
- [x] T028 Run and pass post-implementation spec-kit analysis.
- [x] T029 Run format, strict Clippy, full locked tests, docs policy, mdBook, spelling, text hygiene, whitespace, and mojibake gates.
- [x] T030 Complete independent code, safety, Linux, and documentation reviews; resolve all findings at 80% confidence or higher.

## Phase 7: Delivery

- [ ] T031 Commit as `fix(061): align Linux input and runtime copy contracts` with attribution.
- [ ] T032 Push and open an official PR with `Closes #93` and `Closes #96`.
- [ ] T033 Move both issues to PR Review and monitor every CI check and review.
- [ ] T034 Resolve every first-round hosted review finding.
- [ ] T035 Trigger exactly one authorized second `@Codex review` and resolve every result.
- [ ] T036 Confirm green checks, no unresolved review threads, and merge readiness before requesting the merge ritual.

## Dependencies

- T007 blocks implementation.
- T008 and T009 precede T010 through T013.
- T014 and T015 precede T016 and T017.
- T019 precedes T020 through T022.
- T023 through T026 require implemented behavior.
- T027 through T030 block publication.
- T032 and the single second review request are pre-authorized by the user.

## Parallel Opportunities

- Linux capability tests and menu-startup tests can be developed independently after analysis.
- Documentation policy work can begin after runtime semantics stabilize.
- Code, safety, Linux, and documentation review passes are independent.
