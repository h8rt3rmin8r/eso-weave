# Tasks: v0.15.0 Release Preparation

## Phase 1: Spec-kit and Evidence

- [x] T001 Synchronize `main`, create issue #102, and create `codex/s064-v015-release-prep`.
- [x] T002 Create the S064 feature directory and specification from issue #102.
- [x] T003 Resolve version, scope, Highlight mapping, and publication clarification through autopilot.
- [x] T004 Create requirements, release-safety, and delivery checklists.
- [x] T005 Create research, data model, release-candidate contract, quickstart, and implementation plan.
- [x] T006 Run and pass pre-implementation cross-artifact analysis.

## Phase 2: Release Candidate

- [x] T007 [US1] Add four bounded v0.15.0 Highlights bullets to Unreleased.
- [x] T008 [US2] Preserve all detailed S057 through S063 Added, Changed, and Decisions entries and record dependency PRs #89 and #90.
- [x] T009 [US2] Add a detailed S064 Changed entry without claiming publication.
- [x] T010 [US3] Archive plan 033 with PR #101 evidence.
- [x] T011 [US3] Establish active chronological plan 034 and update the lifecycle ledger and both plan indexes.

## Phase 3: Validation and Analysis

- [x] T012 Run the release-note contract suite.
- [x] T013 Preview exact v0.15.0 notes and verify four bullets and the 120-word budget.
- [x] T014 Verify Cargo, lockfile, README, tags, releases, and pinned release machinery remain unchanged.
- [x] T015 Confirm issues #77 and #84 remain open and the diff closes only #102.
- [x] T016 Run documentation policy, whitespace, UTF-8, BOM, forbidden-dash, mojibake, and secret checks.
- [x] T017 Run applicable formatting, strict Clippy, and complete locked tests.
- [x] T018 Complete post-implementation cross-artifact analysis and diff audit.

## Phase 4: Delivery and Hosted Review

- [x] T019 Commit as `feat(064): prepare v0.15.0 release candidate` with attribution.
- [ ] T020 Push and open an official pull request with `Closes #102`.
- [ ] T021 Move issue #102 to PR Review and monitor every CI check and review.
- [ ] T022 Resolve every first-round hosted review finding.
- [ ] T023 Trigger exactly one authorized second `@Codex review` and resolve every result.
- [ ] T024 Confirm green checks, no unresolved review threads, and merge readiness before requesting the merge ritual.

## Dependencies

- T006 blocks candidate implementation.
- T007 through T011 precede validation.
- T012 through T018 block commit and publication.
- T020 and the single second review request are pre-authorized.
- T024 ends with operator review; S064 never merges or publishes the release.
