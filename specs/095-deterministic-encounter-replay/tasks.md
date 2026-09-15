# Tasks: Deterministic Encounter Replay

## Phase 1: Specification and Design

- [x] T001 Create issue #200, attach it to #186, and set Project 2 to S095/In progress
- [x] T002 Complete spec, clarification, security checklist, research, data model, contracts, and plan
- [x] T003 Audit replay architecture, security boundary, and complete subscription universe
- [x] T004 Analyze requirement, contract, and task traceability before implementation

## Phase 2: Red Contract Tests

- [x] T005 [US1] Add production-Lua profile and differential replay tests in `tests/encounter_addon.rs`
- [x] T006 [US1] Add exact projection mutation and import rejection tests in `tests/encounter_addon.rs`
- [x] T007 [US2] Add loss, malformed correlation, unsupported profile, legacy v1/v2, and diagnostic-canary tests
- [x] T008 [US1] Run focused tests and record the expected red evidence

## Phase 3: Replay Foundation

- [x] T009 [US1] Add bounded normalization profile models in `src/encounter/model.rs`
- [x] T010 [US1] Emit addon-v3 runtime profile metadata in `addon/EsoWeaveData/Encounter.lua`
- [x] T011 [US1] Separate structural and replay enforcement without recursive canonicalization
- [x] T012 [US1] Implement pure deterministic source replay in `src/encounter/replay.rs`
- [x] T013 [US1] Enforce complete-current verification before import and canonical storage

## Phase 4: Compatibility and Hardening

- [x] T014 [US2] Preserve v1 and pre-profile-v2 compatibility with explicit unavailable assessment
- [x] T015 [US2] Preserve partial degraded evidence with indeterminate assessment
- [x] T016 [US2] Add checked numeric, bounded state, malformed batch, nil-edge, unknown-value, and ceiling coverage
- [x] T017 [US2] Confirm metrics, recommendations, immutable storage, and legacy hashes remain stable

## Phase 5: Subscription Decisions and Documentation

- [x] T018 [US3] Complete included and excluded Live/PTS decision matrix
- [x] T019 [US3] Update `docs/project/encounter-model.json` and contract tests
- [x] T020 [US3] Update encounter manual, architecture, testing strategy, governance guidance, and changelog
- [x] T021 [US3] Update plan 042 and build-plan index chronologically with issue #200 traceability

## Phase 6: Review and Publication

- [x] T022 Run independent code, security, and domain reviews and resolve all findings
- [x] T023 Run full local merge gates plus UTF-8/BOM/mojibake and diff checks
- [ ] T024 Commit with traceability trailer, push, and open the official S095 PR
- [ ] T025 Move issue and PR project items to PR review and wait for hosted CI/reviews
- [ ] T026 Resolve every first-round review and request at most one second `@Codex` review
- [ ] T027 Resolve second-round findings, verify green CI and zero open threads, then request operator merge ritual

## Dependencies and Test-First Evidence

T005-T008 precede T009-T013. Replay foundation precedes compatibility hardening.
Code and contract behavior precede documentation. Publication follows local
review and the complete merge gate. Focused red and green commands are recorded
in `analysis.md`.
