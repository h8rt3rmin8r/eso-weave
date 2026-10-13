# Tasks: S128 v0.18.0 Release Preparation

Input: [spec.md](spec.md), [plan.md](plan.md), research/data-model/contracts/quickstart.
Tests: reuse existing gates; no runtime implementation or mirrored metadata test.

## Phase 1: Setup

- [x] T001 Establish S128/#277/branch alignment and spec-kit artifacts in specs/128-v0180-release/ and .specify/feature.json.
- [x] T002 Inspect protected release/config/workflow authorities and hidden tool availability; record baseline and absence of target identity in specs/128-v0180-release/verification.md.

## Phase 2: Foundation

- [x] T003 Finish clarify/checklists/design/tasks and blocking analysis in specs/128-v0180-release/; refresh managed CLAUDE.md pointer.

## Phase 3: User Story 1, Accurate Candidate (P1)

Independent test: identity and release-note gates agree on v0.18.0 with complete preserved history.

- [x] T004 [US1] Record missing-Highlights rejection from scripts/release-notes.sh in target/s128-notes-red.log before notes changes.
- [x] T005 [US1] Add three bounded Highlights and dated S128 preparation decision to CHANGELOG.md; preserve S125-S127 and earlier records.
- [x] T006 [US1] Run preparatory gates and inspect clean tracked tree before local preparation commit; record evidence in specs/128-v0180-release/verification.md.
- [x] T007 [US1] Inspect governed dry run and execute configured local rollover of Cargo.toml, Cargo.lock, CHANGELOG.md, README.md, docs/src/README.md and specs/073-reviewed-catalog-pipeline/fixtures/capture-request.json.
- [x] T008 [US1] Validate six-file version/date projections and unchanged dependencies/history; rerun notes and candidate gates in specs/128-v0180-release/verification.md.

## Phase 4: User Story 2, Completed and Current Plans (P2)

Independent test: generic documentation policy accepts matching plan lifecycle/index/ledger and working references.

- [x] T009 [US2] Archive completed Plan053 at docs/archive/build-plans/plan-053.md with PR #276 merge evidence; repair affected spec references.
- [x] T010 [US2] Create docs/project/build-plans/plan-054.md and update both current/archive README indexes without splitting archive table rows.
- [x] T011 [US2] Synchronize Plan053/054 lifecycle in docs/project/migration-ledger.json and migration-ledger.md; run established documentation policy.

## Phase 5: User Story 3, Official Reviewed Candidate (P3)

Independent test: official PR links #277, all checks pass on final head and every returned thread is resolved within two rounds.

- [x] T012 [US3] Run full existing format/Clippy/locked tests, documentation/render and bundled production build; record actual results in specs/128-v0180-release/verification.md.
- [ ] T013 [US3] Automatically push codex/s128-v0180-release and publish official PR with Closes #277; move issue project stage to PR review.
- [ ] T014 [US3] Address every finding and resolve completed threads within two rounds; verify final-head CI/review state in specs/128-v0180-release/verification.md.

## Phase 6: Polish and Handoff

- [ ] T015 Check UTF-8/no-BOM/LF/punctuation/mojibake and git diff sanity across all changed files; reconcile tasks/receipt and final-head checks in specs/128-v0180-release/verification.md.
- [ ] T016 Deliver the unmerged, untagged official candidate to the owner with issue/PR and actual validation evidence from specs/128-v0180-release/verification.md.

## Dependencies and Execution Order

Setup -> foundation -> US1 notes -> US2 archive/ledger -> clean prep commit -> US1 rollover/identity -> US3 local gates/publication/reviews -> final receipt/head checks -> owner handoff. T006 waits for T009-T011 so cargo-release sees one clean preparation commit. All mutations are sequential; read-only independent fixtures can be batched.

## Parallel Examples

US1: release-note and Debian fixture reads may run together after notes are ready.
US2: index/ledger inspection can be batched; lifecycle mutations remain ordered.
US3: independent local policy reads or remote CI/review reads can be batched; no overlapping Cargo builds or background test runner.

## Implementation Strategy

US1 is the identity MVP, independently validated. Complete the archive before committing preparation; then the governed rollover creates the candidate identity. Add only receipt updates after validation and wait for checks on every final pushed head. Final merge/tag/publication remain owner-controlled later operations.
