# Tasks: Lossless Subscribed-Event Envelope

**Input**: Design documents in `specs/094-lossless-event-envelope/`

## Phase 1: Governance and Traceability

- [x] T001 Amend `.specify/memory/constitution.md` to 4.0.0 and align `CLAUDE.md`, responsible-use guidance, autopilot, and changelog.
- [x] T002 Create the atomic S094 child issue under #186 and set Project 2 Slice and Stage fields.
- [x] T003 Update `docs/project/build-plans/plan-042.md` and its index with the chronological S094 boundary.

## Phase 2: Red Contract Tests

- [x] T004 [US1] Replace privacy-absence assertions with exact positional callback and API-sample retention tests in `tests/encounter_addon.rs`.
- [x] T005 [US2] Add raw record, byte, string, unsupported-value, callback, clock, and terminal-reserve tests in `tests/encounter_addon.rs`.
- [x] T006 [US3] Add capture v1/v2, store migration, mixed-version, hash-preservation, and round-trip tests in `tests/encounter_import.rs`.
- [x] T007 [US3] Add metric, recommendation, history, and data-addon upgrade regression assertions.
- [x] T008 Record the focused failing test evidence in `analysis.md`.

## Phase 3: Raw Capture Authority

- [x] T009 [US1] Implement exact tagged scalar encoding and raw source sequencing in `addon/EsoWeaveData/Encounter.lua`.
- [x] T010 [US1] Retain all selected callback deliveries and normalization-dependent API reads, linking compatibility projections to their primary raw source.
- [x] T011 [US2] Implement whole-observation preflight, explicit raw loss, temporal discontinuity, and terminal reserve.
- [x] T012 [US3] Preserve terminal and interrupted schema-v1 SavedVariables without fabricating v2 raw evidence.

## Phase 4: Hostile Import and Immutable Storage

- [x] T013 [US1] Add v2 raw models and strict validation in `src/encounter/model.rs` and `src/encounter/validate.rs`.
- [x] T014 [US3] Add explicit v1/v2 capture dispatch and canonical format selection in `src/encounter/mod.rs`.
- [x] T015 [US3] Implement transactional store schema v2 migration and mixed-version validation in `src/encounter/store.rs`.
- [x] T016 [US1] Add exhaustive synthetic Lua-to-parser-to-SQLite fixtures and production ceiling checks.

## Phase 5: Compatibility and Documentation

- [x] T017 [US3] Prove unchanged normalized metric, recommendation, history, and CLI behavior.
- [x] T018 Update `docs/project/encounter-model.json` and `.github/scripts/docs-policy.mjs` for v2 raw authority.
- [x] T019 Update the encounter manual, reference, test strategy, changelog, and subscription matrix.
- [x] T020 Complete `analysis.md`, checklists, UTF-8/BOM/mojibake checks, and task status.

## Phase 6: Validation, Review, and Publication

- [x] T021 Run focused tests, formatting, strict clippy, full tests, release build, docs tests, mdBook, docs policy, typos, and diff checks.
- [x] T022 Run local code, security, and domain reviews and resolve every finding.
- [ ] T023 Commit with S094 traceability, push, open the official PR, and set Project 2 Stage to PR review.
- [ ] T024 Process every first-round hosted review comment and hosted check.
- [ ] T025 Request at most one second `@Codex review`, process it fully, and stop when checks are green for the operator merge ritual.

## Dependencies

T001-T003 gate implementation. T004-T008 must be observed failing before
T009-T016. T017-T020 follow the green implementation. T021-T025 are sequential
publication gates.
