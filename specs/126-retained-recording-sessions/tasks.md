# Tasks: Retained Recording Sessions

**Slice**: S126 | **Issue**: #273 | **Spec**: [spec.md](spec.md)
Tests precede implementation. All phases are required for the owner-authorized slice.

## Phase 1 - Setup

- [x] T001 Complete spec, clarification and requirements checklists in specs/126-retained-recording-sessions/spec.md.
- [x] T002 Complete research, model, contract and constitution gates in specs/126-retained-recording-sessions/plan.md.
- [x] T003 Archive completed Plan051 and create chronological Plan052 in docs/project/build-plans/plan-052.md.

## Phase 2 - Foundations

- [x] T004 Write red retention, migration and import tests in tests/encounter_addon.rs and tests/encounter_import.rs (FR-002, FR-005, FR-006, FR-008).
- [x] T005 Add envelope/session-list models and strict version dispatch in src/encounter/model.rs and src/encounter/mod.rs.
- [x] T006 Validate all session identities, ordered keys and whole-spool bounds before filtering in src/encounter/validate.rs.
- [x] T007 Add schema5 durable session ordering and schema1-4 migration in src/encounter/store.rs (FR-002, FR-008).

## Phase 3 - US1: Start another session

Independent criterion: two single sessions survive reload and import without deletion.

- [x] T008 [US1] Write same-clock, repeated-start and selection refusal tests in tests/encounter_addon.rs (FR-001, FR-002, FR-004).
- [x] T009 [US1] Implement validated legacy migration and monotonic identity allocation in addon/EsoWeaveData/Encounter.lua.
- [x] T010 [US1] Implement admission-before-rotation and immutable terminal retention in addon/EsoWeaveData/Encounter.lua.
- [x] T011 [US1] Permit stopped next-session selections without relabeling retained provenance in addon/EsoWeaveData/Encounter.lua.

## Phase 4 - US2: Resume continuous recording

Independent criterion: stop/resume preserves identity and prefix; stop/start creates a new identity.

- [x] T012 [US2] Write resume-before/after-import and invalid resume tests in tests/encounter_addon.rs and tests/encounter_history.rs (FR-003, FR-011).
- [x] T013 [US2] Add bound UserResumed interruption and explicit resume admission in addon/EsoWeaveData/Encounter.lua and src/encounter/model.rs.
- [x] T014 [US2] Validate new resume markers and controlled append-only snapshot extension in src/encounter/validate.rs and src/encounter/store.rs.

## Phase 5 - US3: Capacity and discard

Independent criterion: full capacity loses no evidence; confirmed selected discard removes exactly its target.

- [x] T015 [US3] Write global budget, active deletion and catalog-isolation regressions in tests/encounter_addon.rs (FR-005, FR-007).
- [x] T016 [US3] Implement cached archived totals and spool-wide terminal reserves in addon/EsoWeaveData/Encounter.lua.
- [x] T017 [US3] Implement chronological session listing, confirmed discard and counter-preserving clear in addon/EsoWeaveData/Encounter.lua.
- [x] T018 [US3] Report capacity, counts and actionable commands without raw values in addon/EsoWeaveData/Encounter.lua (FR-009).

## Phase 6 - US4: Recovery and import

Independent criterion: supported upgrade preserves payloads; mixed source import/reimport is atomic and environment-specific.

- [x] T019 [US4] Write malformed archive, mixed-environment and collision rollback tests in tests/encounter_import.rs and tests/encounter_history.rs (FR-006, FR-008).
- [x] T020 [US4] Prepare and preflight every matching session snapshot in one transaction in src/encounter/store.rs.
- [x] T021 [US4] Write history order/deletion/reimport and schema4 migration tests in tests/encounter_history.rs.
- [x] T022 [US4] Add selected-environment session summaries and counts in src/encounter/history.rs and src/app/ui.rs (FR-009, FR-010).
- [x] T023 [US4] Cover source refresh and bounded workflow presentation in tests/app_encounter_history.rs and tests/app_ui_sizing.rs.

## Phase 7 - Documentation and delivery

- [x] T024 Update canonical recording/reference/troubleshooting documents under docs/src and dated CHANGELOG.md decisions (FR-012).
- [x] T025 Reconcile build-plan indexes and migration ledger under docs/project (FR-013).
- [x] T026 Run full local Rust, policy, documentation, rendering and text gates; record evidence in specs/126-retained-recording-sessions/verification.md.
- [x] T027 Commit, automatically push and publish official PR; record issue/PR mapping in specs/126-retained-recording-sessions/verification.md.
- [ ] T028 Address every external finding within two review rounds, verify green final-head CI and request owner merge in specs/126-retained-recording-sessions/verification.md.

## Dependencies and execution

Setup -> foundations -> US1 -> US2 -> US3 -> US4 -> delivery. Tests precede each
behavior change; files shared by stories are edited sequentially. Parser/store
test cases and Lua test scenarios can be drafted independently after design.
No parallel code mutation is required. US1 is the first useful increment, but
all four stories and preservation constraints are required for slice completion.

## Requirement coverage

FR-001: T008-T010; FR-002: T004-T007,T009,T021; FR-003: T012-T014;
FR-004: T008,T011; FR-005: T004,T006,T015-T016; FR-006: T004,T009,T019;
FR-007: T015,T017; FR-008: T004,T007,T019-T021; FR-009: T018,T022-T023;
FR-010: T022-T023; FR-011: T012-T016,T026; FR-012: T024;
FR-013: T003,T025-T028. SC-001-SC-007 are covered by the story regression tasks.
