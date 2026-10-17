# Tasks: S134 v0.19.1 Patch Release Candidate

Input: [spec.md](spec.md), [plan.md](plan.md), research, data model and contracts.

## Phase 1: Setup

- [x] T001 Complete #313 housekeeping and S134 issue/specification/clarification/checklists in specs/134-v0191-release/ and .specify/feature.json (FR-004, FR-008).
- [x] T002 Research release authorities and define the plan, data model, contracts/release.md and quickstart.md in specs/134-v0191-release/ (FR-002, FR-003, FR-004).

## Phase 2: Foundation

- [x] T003 Run the read-only consistency analysis of spec.md, plan.md and tasks.md before implementation, and retain its report in specs/134-v0191-release/analysis.md (FR-004).
- [x] T004 Establish Plan060 and synchronize docs/project/build-plans/README.md, migration-ledger.json, migration-ledger.md and CLAUDE.md (FR-008).

## Phase 3: US1 - Understand the patch

Independent test: valid generated notes represent all five S133 outcomes and retain history.

- [x] T005 [US1] Exercise the existing missing-Highlights rejection in scripts/release-notes.sh and record it in specs/134-v0191-release/verification.md (FR-001, FR-004).
- [x] T006 [US1] Add concise five-outcome Highlights and preparation decision in CHANGELOG.md; pass scripts/release-notes.test.sh and the candidate preview (FR-001).

## Phase 4: US2 - Consistent candidate

Independent test: all six identity surfaces agree and existing candidate gates pass.

- [x] T007 [US2] Pass preparation documentation/policy/text gates, commit the prepared CHANGELOG.md and S134 records, and verify a clean tree (FR-002, FR-004).
- [x] T008 [US2] Inspect the configured local cargo-release dry run and execute the v0.19.1 rollover governed by release.toml (FR-002).
- [x] T009 [US2] Audit six-file identity/date agreement, unchanged historical changelog, dependencies and independent addon contracts in Cargo.toml, Cargo.lock and rollover references (FR-002, FR-003).
- [x] T010 [US2] Run fmt, Clippy, all locked tests, production binary build and release-profile tests/documentation.rs; record actual results in verification.md (FR-003, FR-005).
- [x] T011 [US2] Run existing notes/Debian, policy/brand/trust, spelling, mdBook/generated-site and headless render checks; record limits in verification.md (FR-005).

## Phase 5: US3 - Owner release control

Independent test: official PR stays open, closes only #315 on merge, and no target tag/release exists.

- [ ] T012 [US3] Commit the observed S134 evidence, publish the official PR with Closes #315 and relate #314, and reconcile issue/Project states (FR-004, FR-006, FR-008).
- [ ] T013 [US3] Resolve review findings and confirm final-head CI/review state through the PR; keep hosted evidence linked in verification.md (FR-005, FR-006).
- [ ] T014 [US3] Verify the PR is open/unmerged, v0.19.1 tag is absent and latest published release remains v0.19.0; stop before merge/tag/publication (FR-007).

## Phase 6: Delivery polish

- [ ] T015 Finish specs/134-v0191-release/tasks.md and verification.md, validate text and final scope, and hand the PR to the owner (FR-004, FR-007).

## Dependencies and execution strategy

T001-T003 precede implementation. T004-T007 prepare a clean commit before T008.
T009-T011 evaluate the actual rollover before T012-T015 handoff. US1 notes can be
validated independently; US2 depends on their preparation commit; US3 depends on
candidate evidence. Read-only identity inspection and independent policy scans
can be batched, but edits and Cargo commands remain sequential. No parallel
implementation work is needed for this bounded release slice. Deliver the whole
candidate PR; do not deploy a partial notes-only increment.
