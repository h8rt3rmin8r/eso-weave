# Tasks: S124 Dependency Security and v0.17.4 Preparation

Input: [spec](spec.md), [plan](plan.md), [research](research.md), [model](data-model.md), [contract](contracts/maintenance-candidate.md).

## Phase 1: Setup

- [x] T001 Establish branch codex/s124-v0174-release, issues #261/#262/#263, exact source PR revisions and specs/124-v0174-security-maintenance/spec.md (FR-005).
- [x] T002 Complete clarification, requirement/security checklists and design artifacts under specs/124-v0174-security-maintenance; update .specify/feature.json and CLAUDE.md managed plan context (FR-005).

## Phase 2: Foundation

- [x] T003 Pass blocking cross-artifact analysis before implementation; record the result in specs/124-v0174-security-maintenance/analysis.md (FR-005).
- [x] T004 Inspect Cargo.lock advisory baseline and run existing focused credential/cursor/MCP/Lua tests before updates; record meaningful gaps and actual baseline in specs/124-v0174-security-maintenance/verification.md (FR-001, FR-003).

## Phase 3: US1 - Dependency security (P1)

Independent check: zero advisory-affected rustls versions and all ten targets accounted for, with retained authority/exact-value behavior.

- [x] T005 [US1] Update scoped direct requirements in Cargo.toml and perform targeted exact-version Cargo.lock resolution, excluding unrelated automated lockfile churn (FR-001, FR-002).
- [x] T006 [US1] Run relevant existing coverage in src/local_service.rs, src/database_query.rs and tests/; add regression tests before narrowly required compatibility code if a demonstrated gap needs it (FR-003).
- [x] T007 [US1] Inspect complete resolved inventory and compatibility outcomes; record selected versions, required transitive changes and actual results in specs/124-v0174-security-maintenance/verification.md (FR-001, FR-002, FR-003; SC-001, SC-002).

## Phase 4: US2 - Scanner pins (P2)

Independent check: official paired pins and unchanged workflow authority pass the existing policy.

- [x] T008 [US2] Adopt official v4.38.2 full commit refs in .github/workflows/codeql.yml and record dated pin provenance in CHANGELOG.md (FR-004).
- [x] T009 [US2] Run existing .github/scripts/trust-policy.mjs and trust-policy.test.mjs; document actual results in specs/124-v0174-security-maintenance/verification.md (FR-004; SC-002).

## Phase 5: US3 - Candidate and chronological planning (P3)

Independent check: coherent v0.17.4 identities, bounded notes, valid archive/active lifecycle and official PR linked to all three issues.

- [x] T010 [US3] Archive Plan049 to docs/archive/build-plans/plan-049.md with chronological S123/publication/PR260 evidence and introduce docs/project/build-plans/plan-050.md; update both indexes, migration-ledger JSON/prose and archived Plan048 reference (FR-007; SC-005).
- [ ] T011 [US3] Add bounded maintenance Highlights and complete history to CHANGELOG.md, preserving #259; run existing preparation Rust/docs/release/trust gates and commit before rollover (FR-006, FR-008).
- [ ] T012 [US3] Inspect and execute governed cargo-release 0.17.4 rollover from the clean preparation commit; validate Cargo.toml, Cargo.lock, CHANGELOG.md, README.md, docs/src/README.md and specs/073-reviewed-catalog-pipeline/fixtures/capture-request.json (FR-006, FR-009; SC-003, SC-006).
- [ ] T013 [US3] Publish the official PR closing #261/#262/#263, attach it, and retain source dependency PRs for post-merge supersession (FR-008, FR-010; SC-005, SC-006).

## Phase 6: Review and handoff

- [ ] T014 Satisfy every CI/review finding and thread on the official PR within at most two requested Codex rounds; record dispositions in specs/124-v0174-security-maintenance/verification.md (FR-008; SC-004).
- [ ] T015 Complete specs/124-v0174-security-maintenance task/evidence receipt, confirm final-head checks, clean branch and no new tag/publication, then request owner final review/merge (FR-008, FR-009, FR-010; SC-004, SC-006).

## Dependencies and execution

T001-T003 precede all implementation. T004 precedes T005/T006; any newly required regression test precedes its source compatibility change. T007 follows resolved updates and focused checks. T008/T009 and T010 operate on distinct paths after analysis, but mutations remain sequential. All implementation precedes T011, which creates the clean preparation commit before T012. T013 follows candidate gates; T014 precedes final T015.

US1 is the security MVP; the accepted slice includes all three stories. Read-only research can run concurrently; Cargo operations share target/lock state and run sequentially in the foreground. Original PR closure is post-owner-merge housekeeping rather than a premature task here.
