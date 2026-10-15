# Tasks: S131 Native v15 Format Coverage

Scope: #296 and #297, all three added profiles. No partial MVP handoff.

## Phase 1: Setup and research

- [x] T001 Establish exact branch, issue mapping and chronological Plan057 in `docs/project/build-plans/plan-057.md` (FR-010).
- [x] T002 Pin producer and official-uploader evidence plus ordered grammar in `specs/131-native-v15-format-coverage/contracts/native-v15-grammar.md` (FR-001, SC-001).
- [x] T003 Complete design, model and validation guide in `specs/131-native-v15-format-coverage/plan.md`, `data-model.md`, `quickstart.md` (FR-001/006/007).
- [x] T004 Run blocking cross-artifact analysis and record PASS before product edits in `specs/131-native-v15-format-coverage/analysis.md` (FR-010).

## Phase 2: US1 normal/inline (P1)

Independent test: producer-derived synthetic inline records preserve raw data,
references, optional tails and source quantities at arbitrary partitions.

- [x] T005 [US1] Add failing synthetic inline event, cast, effect, defaults and scalar-array tests in `tests/encounter_formats.rs` (FR-002/005, SC-002).
- [x] T006 [US1] Implement bounded semantic decoding and shared schema-aware validation in `src/encounter/native/grammar.rs`, `src/encounter/native.rs` (FR-002/005/008).

## Phase 3: US2 verbose profiles (P1)

Independent test: exact ordered verbose labels, flat repeated states, optional
fields and nested equipment arrays produce equivalent supported facts.

- [x] T007 [US2] Add failing verbose separate/inline, ordered-label, literal-string, alias and malformed tests in `tests/encounter_formats.rs` (FR-003/005, SC-002).
- [x] T008 [US2] Implement verbose semantic fields, bounded nested arrays and mixed-profile transitions in `src/encounter/native/grammar.rs` (FR-003/005).

## Phase 4: US3 history and consumers (P1)

Independent test: old canonical evidence remains identical; new profiles pass
the same restart, store, automatic-worker and authenticated read boundaries.

- [x] T009 [US3] Add partition/checkpoint, limits/recovery and schema 1/2 identity regressions in `tests/encounter_formats.rs` and `tests/encounter_incremental.rs` (FR-004/005/006, SC-002/003).
- [x] T010 [US3] Integrate all feed, context, canonical, checkpoint and projection seams in `src/encounter/native.rs` and `src/encounter/native/incremental.rs` (FR-004/006/008).
- [x] T011 [US3] Extend store/runtime/API test scenarios for new profiles, atomic failure/restart, cancellation, collisions and unavailable facts in `tests/encounter_ingest.rs`, `tests/native_maintenance.rs` and `tests/local_service.rs` (FR-006/007/008, SC-003/004).

## Phase 5: Documentation and delivery

- [x] T012 Reconcile capability/grammar/guide/API/MCP claims in `docs/src/`, current contracts and dated `CHANGELOG.md` (FR-009, SC-005).
- [x] T013 Run full Rust/docs/release and text-integrity gates; record actual results in `specs/131-native-v15-format-coverage/verification.md` (FR-010, SC-006).
- [x] T014 Commit, push, publish/attach PR, resolve every finding within two Codex rounds, verify hosted implementation gates and publish the final delivery-head receipt in PR #299 before owner merge; record receipts in `specs/131-native-v15-format-coverage/verification.md` (FR-010, SC-006).

## Dependencies and execution

T001-004 precede all product work. T005 precedes T006; T007 precedes T008.
T009 precedes T010; T011 starts with failing integration cases before any
necessary fix. T012 follows implemented behavior, then T013 and T014.
Tasks share parser/test files and therefore run sequentially. Independent gate
reads may run together; no parallel file mutation is needed.

## Acceptance coverage

Issue #296 AC1: T002-004; AC2: T005-006/009-010; AC3: T005/009-010;
AC4: T009-011; AC5: T011; AC6: T010-012; AC7: T013-014.
Issue #297 AC1: T002-004/007-008; AC2: T007-010; AC3: T009-011;
AC4: T009-011; AC5: T011; AC6: T010-012; AC7: T012; AC8: T013-014.
