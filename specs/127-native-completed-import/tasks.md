# Tasks: Completed Native Encounter Log Import

**Slice**: S127 | **Issue**: #272 | **Spec**: [spec.md](spec.md)
All stories are required. Tests precede behavior code; shared files are edited sequentially.

## Phase 1 - Setup

- [x] T001 Complete specification, clarification and quality checklists in specs/127-native-completed-import/spec.md.
- [x] T002 Resolve research, model, contract and constitution gates in specs/127-native-completed-import/plan.md.
- [x] T003 Archive Plan052 and create chronological Plan053 in docs/project/build-plans/plan-053.md.

## Phase 2 - Foundations

- [x] T004 Derive and sanitize representative native fixtures under specs/127-native-completed-import/fixtures and document transformation.
- [x] T005 Write red native grouping, copy identity and hostile-data tests in tests/encounter_native.rs.
- [x] T006 Add source-specific native models and public seams in src/encounter/native.rs and src/encounter/mod.rs.
- [x] T007 Implement bounded tokenizer and supported normal15 record validation in src/encounter/native.rs.
- [x] T008 Implement isolated run/context/completed-combat grouping and source reports in src/encounter/native.rs.

## Phase 3 - US1: Import completed native recordings

Independent criterion: two representative completed fights appear with native provenance beside existing addon history.

- [x] T009 [US1] Write native import/read-only/mixed-history regressions in tests/encounter_native.rs.
- [x] T010 [US1] Add schema6 native tables, exact historical schema5 definitions and transactional migration in src/encounter/store.rs.
- [x] T011 [US1] Implement native canonical preflight/insert/load and mixed listing in src/encounter/store.rs.
- [x] T012 [US1] Add explicit source kind and optional addon mode in src/encounter/model.rs.
- [x] T013 [US1] Add stable native inspection/import service methods in src/encounter/history.rs.
- [x] T014 [US1] Write serialized native worker and explicit source-selection tests in tests/app_encounter_history.rs.
- [x] T015 [US1] Add native worker commands/events in src/app/encounter_history.rs.
- [x] T016 [US1] Write narrow/wide source-selection presentation tests in tests/app_encounter_history.rs; retain the existing tests/app_ui_sizing.rs geometry suite.
- [x] T017 [US1] Add transient native path, Inspect/Import and mixed source labels in src/app/ui.rs.
- [x] T018 [US1] Add native inspection/import validation commands in src/bin/catalog-compiler.rs.

## Phase 4 - US2: Native evidence and metrics

Independent criterion: native duration/counts/observed quantities remain available without a catalog, and unsupported calculations have reasons.

- [x] T019 [US2] Write exact retention, actor-scope, hit/overflow and unavailable-capability tests in tests/encounter_native.rs.
- [x] T020 [US2] Implement native projection with its own algorithm and capability reasons in src/encounter/native.rs.
- [x] T021 [US2] Dispatch native detail before addon catalog gates in src/encounter/history.rs and src/app/encounter_history.rs.
- [x] T022 [US2] Render native factual metrics and unavailable reasons without raw values in src/app/ui.rs.

## Phase 5 - US3: Incomplete and unsupported input

Independent criterion: malformed input rejects atomically; unknown/tail/unfinished data remains explicit and default messages reveal no raw values.

- [x] T023 [US3] Write unsupported version/layout, line/depth/token/budget and incomplete-boundary cases in tests/encounter_native.rs.
- [x] T024 [US3] Complete bounded error/report dispositions and relevant unknown context in src/encounter/native.rs.
- [x] T025 [US3] Write source replacement and kind/path/channel/generation staleness cases in tests/app_encounter_history.rs and tests/app_ui_sizing.rs.
- [x] T026 [US3] Reject stale cross-source results and clear failed native readiness in src/app/ui.rs.

## Phase 6 - US4: Deterministic history

Independent criterion: reimport/copy/append-run, collision rollback, old-store migration, deletion and backup preserve identities and canonical evidence.

- [x] T027 [US4] Write schema1-5 rollback/collision, mixed-source order/reimport/deletion/backup tests in tests/encounter_native.rs and tests/encounter_history.rs.
- [x] T028 [US4] Validate native schema/rows/shared ordering and preserve immutable evidence in src/encounter/store.rs.
- [x] T029 [US4] Extend source-aware deletion/backup and empty/no-op rollback behavior in src/encounter/store.rs.
- [x] T030 [US4] Reconstruct historical fixture schemas explicitly in tests/encounter_addon.rs and tests/encounter_import.rs.

## Phase 7 - Documentation and delivery

- [x] T031 Update native canonical guidance/diagram and approved native policy anchors/removal fixtures in docs/src and .github/scripts/docs-policy*.
- [x] T032 Reconcile chronological plan indexes and migration ledger under docs/project and dated CHANGELOG.md decisions.
- [x] T033 Run focused/private representative-size and all foreground Rust/policy/docs/render/text gates; record audit/verification in specs/127-native-completed-import.
- [x] T034 Commit, automatically push and publish official PR; record issue/PR mapping in specs/127-native-completed-import/verification.md.
- [ ] T035 Address every external finding within two rounds, verify green exact final-head CI, and request owner merge in specs/127-native-completed-import/verification.md.

## Dependencies and execution

Setup -> foundations -> US1 -> US2 -> US3 -> US4 -> delivery.
Parser/context foundations precede storage and service; worker/service precede UI.
Tests are written before the behavior they constrain. US1 is the first useful
increment, but all preservation, capability, error and reimport stories are required.
Read-only native-contract and integration research ran concurrently during plan.
After design, fixture and UI test drafts could be independent; shared implementation
files make sequential code mutation preferable. No parallel code delegation is required.

## Requirement coverage

FR-001:T014-T018; FR-002:T005,T008,T013-T017; FR-003:T005,T007-T008,T023-T024;
FR-004:T004-T008,T019-T020; FR-005:T019-T022; FR-006:T005,T027-T029;
FR-007:T009-T011,T027-T030; FR-008:T005-T008,T013,T023,T033;
FR-009:T023-T026; FR-010:T004-T007,T009,T019,T022-T024;
FR-011:T009-T012,T021,T027-T030; FR-012:T031,T033;
FR-013:T001-T003,T032-T035.
SC-001:T004-T018; SC-002:T027-T030; SC-003:T023-T026;
SC-004:T019-T022; SC-005:T009-T011,T025-T030; SC-006:T033; SC-007:T033-T035.
