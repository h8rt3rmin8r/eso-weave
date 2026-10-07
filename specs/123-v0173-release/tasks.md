# Tasks: S123 v0.17.3 Release Preparation

**Input**: [plan](plan.md), [spec](spec.md), [research](research.md), [model](data-model.md), [contract](contracts/release-candidate.md).

## Phase 1: Setup

- [x] T001 Confirm S123 branch, issue #255, portable `.specify/feature.json` and complete design artifacts in specs/123-v0173-release (FR-004,008).

## Phase 2: Foundation

- [x] T002 Complete requirement checklists and blocking cross-artifact analysis for specs/123-v0173-release/spec.md, plan.md and tasks.md (FR-005,006).
- [x] T003 Confirm unused version/tag and installed governed release tool against release.toml and docs/project/releasing.md; record evidence in specs/123-v0173-release/verification.md (FR-001,008; SC-005).

## Phase 3: US1 - Accurate Release Identity (P1)

**Independent test**: Existing release-note rejection/fixtures and documentation identity policy pass after rollover.

- [x] T004 [US1] Capture missing-Highlights rejection using scripts/release-notes.sh, then add bounded S120-S122 outcome Highlights and S123 preparation record to CHANGELOG.md (FR-002; SC-002).
- [x] T005 [US1] Run release-note/documentation baseline gates and commit clean preparatory CHANGELOG.md and specification/planning edits before rollover (FR-006).
- [x] T006 [US1] Inspect cargo-release dry run and execute governed rollover for Cargo.toml, Cargo.lock, CHANGELOG.md, README.md, docs/src/README.md and specs/073-reviewed-catalog-pipeline/fixtures/capture-request.json (FR-001,002,005; SC-001).
- [x] T007 [US1] Validate candidate notes, history preservation and metadata with existing scripts/release-notes.test.sh and .github/scripts/docs-policy.mjs (FR-001,002,006; SC-001,002).

## Phase 4: US2 - Completed and Current Planning (P2)

**Independent test**: Existing documentation policy accepts contiguous Plan048/049 lifecycle with valid paths and merged figure evidence.

- [x] T008 [US2] Move completed Plan048 into docs/archive/build-plans/plan-048.md, preserve reservations and append chronological delivery evidence (FR-003; SC-003).
- [x] T009 [US2] Create preparation-only docs/project/build-plans/plan-049.md; update current/archive indexes and docs/project/migration-ledger.json and migration-ledger.md (FR-003,004; SC-003).
- [x] T010 [US2] Update docs/project/documentation-visualization-audit.md and stale Plan048 path references in specs/112-troubleshooting-decision-tree/plan.md, specs/120-combat-recovery-and-api-compatibility/plan.md and tasks.md, and specs/121-encounter-logging-and-addon-clarity/tasks.md (FR-003; SC-003).

## Phase 5: US3 - Reviewed Candidate (P3)

**Independent test**: Official PR closes #255 on merge, all exact-head required checks pass and every finding is answered/resolved, without tag/publication.

- [x] T011 [US3] Run full Rust, release-build, documentation/rendering and repository hygiene gates; record actual evidence in specs/123-v0173-release/verification.md (FR-005,006; SC-004).
- [x] T012 [US3] Commit/push validated candidate, publish official PR closing #255, attach artifact and move delivery project to PR Review; record PR in specs/123-v0173-release/verification.md (FR-007; SC-005).
- [x] T013 [US3] Satisfy every external finding/review and exact-head CI within two requested Codex rounds; record dispositions in specs/123-v0173-release/verification.md (FR-007; SC-004).

## Phase 6: Polish and Handoff

- [x] T014 Finish specs/123-v0173-release/verification.md and task receipt, confirm final-head checks, clean tree and no new tag/release, then request owner final review and merge (FR-006,008; SC-004,005).

## Dependencies and Execution

T001-T003 precede implementation. US1's T004 and US2's T008-T010 precede the clean preparatory commit T005, then T006-T007. US3 follows both stories. T013 precedes final T014. US1 is the identity MVP; the accepted slice includes all three stories before handoff.

Parallel opportunities: US1 notes and US2 historical reference edits operate on separate files after foundation. Execute mutations sequentially to avoid shared archive/index authority races; research agents already worked read-only in parallel. For US3, independent format and documentation policy checks may run together; Cargo operations share build state and run sequentially.

All 14 tasks have IDs, explicit paths and requirement coverage. No new mirrored test suite or runtime implementation is needed.
