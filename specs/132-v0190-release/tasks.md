# Tasks: S132 v0.19.0 Release Candidate

**Input**: [spec.md](spec.md), [plan.md](plan.md), research, data model and candidate contract.
**Tests**: Existing gates supply failure/pass evidence; no new runtime implementation or mirror tests.

## Phase 1: Setup

- [x] T001 Establish #301/#302, baseline and branch; persist S132 in `.specify/feature.json`.
- [x] T002 Complete specify, clarify, release checklist and design artifacts under `specs/132-v0190-release/`.

## Phase 2: Foundational Gate

- [x] T003 Run blocking read-only analysis of `specs/132-v0190-release/{spec,plan,tasks}.md` and record a passing receipt in `analysis.md` before implementation (FR-001 through FR-008).

## Phase 3: User Story 1 - Complete Candidate (P1)

Independent criterion: release identity and notes agree; only three intended dependency packages change.

- [x] T004 [US1] Record existing missing-Highlights rejection from `scripts/release-notes.sh` before editing `CHANGELOG.md` (FR-003/FR-006).
- [x] T005 [US1] Incorporate #284's exact dependency changes in `Cargo.toml` and `Cargo.lock` (FR-001).
- [x] T006 [US1] Add Highlights, dependency/preparation records and dated pinned-guide decision to `CHANGELOG.md` (FR-003/FR-004).
- [x] T007 [US1] Validate preparatory gates and commit the clean pre-rollover tree, recording outcomes in `verification.md` (FR-006).
- [x] T008 [US1] Inspect dry run and execute configured rollover; audit `Cargo.toml`, `Cargo.lock`, `CHANGELOG.md`, `README.md`, `docs/src/README.md` and `specs/073-reviewed-catalog-pipeline/fixtures/capture-request.json` (FR-002/FR-003).

## Phase 4: User Story 2 - Current Guidance (P2)

Independent criterion: release guidance describes the implemented package/profile scope; current lifecycle records agree.

- [x] T009 [P] [US2] Reconcile package wording in `docs/project/releasing.md` and `docs/src/development/release-and-packaging.md`, plus current profile context in `CLAUDE.md` (FR-004).
- [x] T010 [P] [US2] Create `docs/project/build-plans/plan-058.md`; update its index and `docs/project/migration-ledger.{json,md}` while preserving Plan057's archived record (FR-005).

## Phase 5: User Story 3 - Reviewed Owner Handoff (P3)

Independent criterion: official open PR, successful final-head checks and no unresolved findings, with publication deferred.

- [x] T011 [US3] Run required local candidate gates and record actual evidence in `specs/132-v0190-release/verification.md` (FR-006).
- [x] T012 [US3] Push `codex/s132-v0190-release`, publish and attach the official PR closing #302, relating to #301/#284 (FR-007).
- [x] T013 [US3] Complete up to two Codex reviews, correct findings, answer and resolve all completed threads on the PR; record dispositions in `verification.md` (FR-007).
- [x] T014 [US3] Confirm required successful checks on the final PR head and owner merge/publication boundaries in `verification.md` (FR-007/FR-008).

## Phase 6: Polish

- [x] T015 Audit changed text for UTF-8/no BOM/LF, punctuation, broken references and `git diff --check`; update `spec.md` and this task receipt to delivered-for-review state (FR-005/FR-006).
- [x] T016 Synchronize #302/#301 acceptance progress and Project stage, and deliver the open PR for owner review, preserving later #284 retirement and publication work (FR-007/FR-008).

## Dependencies and Execution Order

T001-T003 precede implementation. T004 precedes T006. T005/T006/T009/T010 precede T007; T007 precedes T008. T008 precedes final candidate T011-T016. T009 and T010 can proceed independently after the analysis gate; each uses separate files. Review corrections repeat only affected gates, and final CI always covers the delivered head.

## Implementation Strategy

Deliver all three stories atomically in one candidate PR. US1 is the minimal identity/content outcome; US2 makes the accompanying guidance consistent; US3 completes review and automated evidence. No partial subset is called complete. No merge, tag or release publication is a task in this slice.
