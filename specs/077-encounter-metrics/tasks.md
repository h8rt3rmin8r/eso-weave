# Tasks: Versioned Encounter Metrics

## Phase 1: Specification and gates

- [x] T001 Create S077 through the repository spec-kit feature initializer
- [x] T002 Resolve player, effect, persistence, pet, and provenance clarifications
- [x] T003 Complete requirements and metric-integrity checklists
- [x] T004 Author plan, research, data model, contract, and quickstart
- [x] T005 Run and record the blocking pre-implementation analysis

## Phase 2: Tests first

- [x] T006 Add canonical actual-schema encounter fixture
- [x] T007 Add failing deterministic value, event-order, and loss-quality tests
- [x] T008 Add failing zero-duration, empty-share, arithmetic, and player tests
- [x] T009 Add failing catalog, resolution, output, path, and no-clobber tests
- [x] T010 Add failing maintainer CLI contract test

## Phase 3: Core implementation

- [x] T011 Add typed projection, metric, loss, and join-receipt models
- [x] T012 Implement DPS, effective HPS, damage share, uptime, and cast calculations
- [x] T013 Implement loss quality and canonical serialization
- [x] T014 Implement exact-compatible catalog joins and later-resolution receipts
- [x] T015 Implement atomic projection publication with safe path boundaries
- [x] T016 Expose reusable API and `encounter-project` CLI command

## Phase 4: Documentation and governance

- [x] T017 Update canonical encounter metric documentation
- [x] T018 Update build-plan chronology for merged S076 and active S077
- [x] T019 Add S077 changelog Added and dated Decisions entries
- [x] T020 Complete traceability and textual hygiene checks

## Phase 5: Verification and delivery

- [x] T021 Run focused encounter metrics and CLI tests
- [x] T022 Run full formatting, strict Clippy, and locked Rust suite
- [x] T023 Run optimized builds and complete documentation CI parity
- [x] T024 Record post-implementation analysis and mark all implementation tasks complete
- [ ] T025 Commit, push, publish PR, and move project state to PR review
- [ ] T026 Process all CI and bot feedback with at most one second `@Codex` round
- [ ] T027 Stop with green CI and satisfied reviews for the merge ritual
