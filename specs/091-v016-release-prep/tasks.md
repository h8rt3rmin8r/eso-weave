# Tasks: v0.16.0 Release Preparation

## Phase 1: Evidence and Specification

- [x] T001 Synchronize main, close S090 housekeeping, create issue #163, and
  create the S091 branch.
- [x] T002 Audit all 23 merged pull requests since v0.15.1.
- [x] T003 Select v0.16.0 and create the complete spec-kit package.
- [x] T004 Identify the documentation metadata rollover defect and bounded fix.

## Phase 2: Candidate and Governance

- [x] T005 Add three bounded v0.16.0 Highlights and the S091 detailed record.
- [x] T006 Add field-scoped cargo-release documentation metadata replacements.
- [x] T007 Add documentation-policy contract validation and fixtures.
- [x] T007a Trigger documentation policy for every release.toml change.
- [x] T008 Correct version ownership and embedded-addon release prose.
- [x] T009 Record the dated pinned-artifact decision.
- [x] T010 Archive Plan 040 and establish Plan 041 across indexes and ledger.

## Phase 3: Validation

- [x] T011 Run release-note tests and exact v0.16.0 preview.
- [x] T012 Run documentation policy and complete Rust checks.
- [x] T013 Run whitespace, UTF-8, BOM, punctuation, mojibake, secret, and scope
  checks.
- [x] T014 Run and inspect a cargo-release dry run.
- [x] T015 Complete post-implementation analysis and independent review.

## Phase 4: Hosted Preparation Review

- [x] T016 Commit, push, and open the S091 pull request closing issue #163.
- [x] T017 Move the project item to PR Review and monitor all required checks.
- [ ] T018 Resolve every actionable review finding and confirm merge readiness.
- [ ] T019 Merge the preparation pull request under the operator's explicit
  end-to-end release greenlight and complete branch housekeeping.

## Phase 5: Public Release

- [ ] T020 Preserve the unrelated untracked draft while obtaining clean main.
- [ ] T021 Repeat the v0.16.0 dry run and execute cargo-release.
- [ ] T022 Monitor every tag-workflow job through green completion.
- [ ] T023 Confirm release notes and all five public artifacts.
- [ ] T024 Restore the unrelated draft and close the release milestone.
