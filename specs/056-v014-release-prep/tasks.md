# Tasks: v0.14.0 Release Preparation

## Phase 1: Spec-kit

- [x] T001 Synchronize `main`, create issue #85, and create `codex/s056-v014-release-prep`.
- [x] T002 Create the S056 feature directory and specification from issue #85.
- [x] T003 Resolve clarification through the autopilot decision policy.
- [x] T004 Complete research, data model, release-candidate contract, quickstart,
  and checklists.
- [x] T005 Run pre-implementation cross-artifact analysis.

## Phase 2: Release candidate

- [x] T006 Add two bounded v0.14.0 Highlights bullets to the Unreleased changelog.
- [x] T007 Record S056 release preparation without claiming publication.
- [x] T008 Add chronological build plan 026 and its index entry.

## Phase 3: Validation

- [x] T009 Run the release-note contract suite.
- [x] T010 Preview the exact v0.14.0 notes and verify bullet and word budgets.
- [x] T011 Verify Cargo, lockfile, README, tags, releases, and release machinery
  remain unchanged; confirm #77 is open and no #79 through #84 implementation
  or closing claim enters the diff.
- [x] T012 Run whitespace, UTF-8, BOM, forbidden-dash, mojibake, secret, and
  generated-file checks.
- [x] T013 Run `cargo fmt --all -- --check`, strict all-target Clippy, and the
  complete locked test suite.
- [x] T014 Complete post-implementation cross-artifact analysis and diff audit.

## Phase 4: Independent review and delivery

- [x] T015 Run parallel code, governance, and release-safety reviews and resolve
  findings at 80 percent confidence.
- [x] T016 Commit as `feat(056): prepare v0.14.0 release candidate` with attribution.
- [ ] T017 Push and open a pull request with `Closes #85`.
- [ ] T018 Move #85 to PR review and verify hosted checks.
- [ ] T019 Address every first-round review finding and CI failure.
- [ ] T020 Trigger at most one explicitly authorized second `@Codex review`, then
  address every result.
- [ ] T021 Confirm green checks and resolved reviews before requesting the final
  merge ritual.
