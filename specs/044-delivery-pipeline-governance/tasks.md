# Tasks: Delivery Pipeline Governance

**Input**: Design documents from `specs/044-delivery-pipeline-governance/`

**Tests**: Required for the new pull request policy and for all external-state invariants.

## Phase 1: Setup

- [x] T001 Verify the synchronized `main` baseline and S044 feature branch
- [x] T002 Complete the S044 specification, checklists, research, model, contracts, and quickstart
- [x] T003 Run spec-kit validation before implementation

## Phase 2: Foundational governance contracts

- [x] T004 [P] Document the atomic issue and implementation-versus-verification lifecycle in `CONTRIBUTING.md`
- [x] T005 [P] Strengthen `.github/pull_request_template.md` with explicit multi-issue closure and exemption guidance
- [x] T006 [P] Update issue forms so new work declares a discrete outcome and verification lifecycle
- [x] T007 Create the `skip: issue-link` repository label with a narrow description

## Phase 3: User Story 1 - Close atomic work predictably

- [x] T008 [US1] Write failing policy tests in `.github/scripts/issue-link-policy.test.mjs`
- [x] T009 [US1] Implement `.github/scripts/issue-link-policy.mjs` until the tests pass
- [x] T010 [US1] Add the read-only `.github/workflows/issue-link.yml` pull request check
- [x] T011 [US1] Validate positive, negative, multiple-reference, and exemption cases locally

## Phase 4: User Story 2 - See the delivery pipeline at a glance

- [x] T012 [US2] Create and link the public `ESO Weave Delivery` GitHub Project
- [x] T013 [US2] Configure Stage and Slice fields without duplicating issue metadata
- [x] T014 [US2] Add every repository issue exactly once
- [x] T015 [US2] Assign truthful Stage and known Slice values
- [x] T016 [US2] Configure and verify delivery table and board views

## Phase 5: User Story 3 - Trust historical progress metadata

- [x] T017 [US3] Export issue, pull request, milestone, tag, and release evidence
- [x] T018 [US3] Audit every issue's milestone chronology
- [x] T019 [US3] Audit every issue's `needs: verification` state
- [x] T020 [US3] Apply evidence-backed GitHub metadata corrections
- [x] T021 [US3] Record the complete audit disposition in `docs/project-governance.md`

## Phase 6: Documentation and repository integration

- [x] T022 Create chronological build plan `docs/plans/plan-014.md`
- [x] T023 Update `docs/plans/README.md` in chronological order
- [x] T024 Add the dated workflow and governance decision to `CHANGELOG.md`
- [x] T025 Re-run constitution and governance-safety checks

## Phase 7: Validation and review

- [x] T026 Run policy tests, spec-kit checks, encoding checks, and repository quality gates
- [x] T027 Inspect the final diff for scope, secrets, generated files, and unrelated changes
- [ ] T028 Commit with the S044 conventional commit and attribution trailer
- [ ] T029 Push the feature branch and open a pull request closing #45, #46, and #47
- [ ] T030 Move S044 Project items to PR review and verify the new linkage check
- [ ] T031 Address every first-round review comment and all CI failures
- [ ] T032 Trigger at most one `@Codex review` second round and address every resulting comment
- [ ] T033 Confirm all checks are green and all review threads are resolved before requesting the final merge ritual

## Dependencies and execution order

- Phases 1 and 2 establish the contracts required by all user stories.
- User Story 1 must land before relying on automatic issue closure.
- User Stories 2 and 3 may be executed in either order after the lifecycle contract is stable.
- Documentation reflects the final external GitHub state, so Phase 6 follows the Project and audit work.
- The pull request must include complete `Closes #45`, `Closes #46`, and `Closes #47` references.
