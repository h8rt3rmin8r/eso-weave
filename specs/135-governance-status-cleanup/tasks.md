# Tasks: S135 Governance Status Cleanup

## Phase 1: Setup

- [x] T001 Complete specification, clarification and written-quality checklists in specs/135-governance-status-cleanup/.

## Phase 2: Foundational

- [x] T002 Read live issue/closure evidence and consolidate decisions in specs/135-governance-status-cleanup/research.md.
- [x] T003 Pass read-only cross-artifact analysis, then record its result in specs/135-governance-status-cleanup/analysis.md before source edits.

## Phase 3: User Story 1 - Historical audits

Independent acceptance: the old issue inventory has an explicit date/range and the current-plan pointer reaches the active index.

- [x] T004 [US1] Date and reframe the historical inventory and current-state authority in docs/project/governance.md.
- [x] T005 [P] [US1] Replace the stale Plan049 current-work pointer in docs/project/documentation-visualization-audit.md.

## Phase 4: User Story 2 - Lifecycle versus evidence

Independent acceptance: closed work is not scheduled again and unperformed evidence remains unverified.

- [x] T006 [US2] Reconcile retired follow-up metadata through patch 5.1.1 and its Sync Impact Report in .specify/memory/constitution.md, preserving principles/gates.
- [x] T007 [US2] Correct verification, Ultimate and release summaries in docs/project/migration-ledger.json and docs/project/migration-ledger.md.
- [x] T008 [P] [US2] Add separate lifecycle context to docs/project/encounter-model.json and docs/project/catalog-sources.json without changing technical/evidence fields.
- [x] T009 [US2] Record corrected and deliberately retained surfaces with cited evidence in specs/135-governance-status-cleanup/audit.md.

## Phase 5: Polish and local review preparation

- [x] T010 Register Active Plan061 for S135/#319 in docs/project/build-plans/plan-061.md, README.md and both migration ledgers.
- [x] T011 Update CLAUDE.md plan context and CHANGELOG.md Unreleased scope/metadata decision.
- [x] T012 Run existing documentation/trust/text checks and baseline semantic comparisons; record observed results in specs/135-governance-status-cleanup/verification.md.
- [x] T013 Prepare issue-closing PR materials and the exact-head CI/review handoff procedure in specs/135-governance-status-cleanup/verification.md and quickstart.md.

## Dependencies and execution order

T001 -> T002 -> T003 block all edits. T004/T005 form the historical-audit increment. T006/T007/T008 form the lifecycle increment; T009 records both. T010 follows ledger corrections; T011 follows plan registration. T012 precedes commit; T013 prepares publication. T005 and T008 can proceed independently on separate files after the foundation.

## Implementation strategy

Deliver User Story 1 first, then lifecycle context, then planning consistency and validation. Existing checks cover this reversible documentation change; no production checker or prose-mirroring test is added.

The checkboxes track local implementation and review preparation. Branch push, PR creation, actual hosted CI/review reconciliation and final owner handoff follow these tasks. Their observed results belong in the PR conversation; local task completion never predeclares them. No merge, tag or release is authorized.
