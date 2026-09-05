# Tasks: Concise Release Notes

**Input**: Design documents from `specs/047-concise-release-notes/`

**Tests**: Required for every extraction, budget, input-validation, and output-composition rule.

## Phase 1: Spec-kit setup

- [x] T001 Synchronize `main` and create `codex/047-concise-release-notes`
- [x] T002 Create issue #51 as the atomic implementation outcome
- [x] T003 Complete the S047 specification and requirements checklist
- [x] T004 Resolve clarification choices through the autopilot decision policy
- [x] T005 Complete research, model, contract, quickstart, and release-note safety checklist

## Phase 2: Test-first contract

- [x] T006 [US1] Write failing valid-output and detailed-subsection exclusion tests in `scripts/release-notes.test.sh`
- [x] T007 [US2] Write failing missing, empty, budget, invalid-input, and CRLF tests in `scripts/release-notes.test.sh`
- [x] T008 Run the contract suite and record the expected failure because `scripts/release-notes.sh` does not exist

## Phase 3: Concise release-note generator

- [x] T009 [US1] Implement exact version and Highlights extraction in `scripts/release-notes.sh`
- [x] T010 [US1] Enforce the six-item and 120-word budget without truncation
- [x] T011 [US1] Append the immutable tag-specific full-changelog link
- [x] T012 [US2] Add actionable fail-closed validation for every invalid input and changelog state
- [x] T013 Run the contract suite to green and inspect the exact generated Markdown

## Phase 4: Pipeline and authoring integration

- [x] T014 [US2] Run the contract suite in Linux pull-request CI through `.github/workflows/ci.yml`
- [x] T015 [US2] Preserve complete changelog validation and add concise-note validation in `.github/workflows/release.yml`
- [x] T016 [US1] Generate `RELEASE_NOTES.md` with the new script in the release publication job
- [x] T017 [US2] Document the authoring budget and local preview in `docs/releasing.md`

## Phase 5: v0.12.0 preparation and repository integration

- [x] T018 Draft the compliant Unreleased Highlights only after the generator contract is implemented
- [x] T019 Add the dated pinned-artifact decision and S047 change entry to `CHANGELOG.md`
- [x] T020 Add chronological build plan `docs/plans/plan-017.md` and update `docs/plans/README.md`
- [x] T021 Complete `specs/047-concise-release-notes/analysis.md` and resolve every finding

## Phase 6: Validation and review

- [x] T022 Run Bash contract tests and preview Unreleased notes
- [x] T023 Run YAML, whitespace, UTF-8, BOM, forbidden-dash, and mojibake checks
- [x] T024 Run `cargo fmt --all -- --check`
- [x] T025 Run `cargo clippy --all-targets --all-features -- -D warnings`
- [x] T026 Run `cargo test --all --locked`
- [x] T027 Inspect the final diff for scope, secrets, generated files, and unrelated changes
- [ ] T028 Commit as `feat(047): generate concise release highlights` with attribution
- [ ] T029 Push the feature branch and open a pull request with `Closes #51`
- [ ] T030 Move issue #51 to PR review and verify hosted checks
- [ ] T031 Address every first-round review comment and CI failure
- [ ] T032 Trigger at most one `@Codex review` second round and address every resulting comment
- [ ] T033 Confirm all checks are green and every review thread is resolved before requesting the final merge ritual

## Dependencies and execution order

- The spec-kit analysis gate precedes implementation.
- Tests T006 and T007 must exist and fail for the missing generator before T009 begins.
- Generator implementation and green contract tests precede the v0.12.0 Highlights draft.
- The release workflow preserves its full changelog gate before adding the presentation gate.
- S047 changes no release version and creates no tag.
