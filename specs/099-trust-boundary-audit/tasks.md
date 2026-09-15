# Tasks: Repository Trust-Boundary Audit

**Input**: Design documents from `/specs/099-trust-boundary-audit/`

**Prerequisites**: `spec.md`, `plan.md`, `research.md`, `data-model.md`, `contracts/`, and `quickstart.md`

**Tests**: Required by issue #181, the specification, and Constitution Principle III.

## Phase 1: Specification and Clarification

- [x] T001 Create the S099 feature context with the repository spec-kit scripts.
- [x] T002 Snapshot GitHub issue #181 at the public control-objective level in `issue-181.md`.
- [x] T003 Author prioritized authority, workflow, hosted-enforcement, and disclosure scenarios in `spec.md`.
- [x] T004 Resolve protected-base trust, solo-maintainer review, Action publisher, release digest, and disclosure decisions.
- [x] T005 Complete `checklists/requirements.md` and `checklists/trust-and-authorization.md`.

## Phase 2: Planning and Contracts

- [x] T006 Record alternatives and decisions in `research.md`.
- [x] T007 Define authority, untrusted input, workflow, hosted policy, and finding entities in `data-model.md`.
- [x] T008 Define the agent authority contract in `contracts/authority-boundary.md`.
- [x] T009 Define static and hosted workflow integrity in `contracts/workflow-integrity.md`.
- [x] T010 Produce the implementation and verification plan in `plan.md`.
- [x] T011 Produce local and hosted verification instructions in `quickstart.md`.

## Phase 3: Spec-Kit Analysis Gate

- [x] T012 Analyze issue, spec, research, model, contracts, plan, checklists, and tasks in `analysis.md`.
- [x] T013 Resolve every analysis finding and confirm no unresolved clarification marker remains.
- [x] T014 Run spec-kit prerequisites with tasks required.

## Phase 4: User Story 1 - Keep Authority With the Operator (P1)

**Goal**: Ensure collaboration and proposed branch content cannot become mutation authority.

**Independent Test**: Review representative instruction-shaped inputs and verify guidance and policy keep them inert outside the authorized task.

- [x] T015 [US1] Add failing policy tests for high-risk local skill metadata and required trusted-base guidance.
- [x] T016 [US1] Update `CLAUDE.md`, `CONTRIBUTING.md`, and `docs/project/build-autopilot.md` with the authority and halt contracts.
- [x] T017 [US1] Remove unrelated offensive bypass and evasion skills from the project-local guidance surface.
- [x] T018 [US1] Add canonical contributor guidance in `docs/src/development/repository-trust.md` and the book summary.
- [x] T019 [US1] Make focused agent-boundary tests green.

## Phase 5: User Story 2 - Execute Immutable Workflow Dependencies (P1)

**Goal**: Execute only reviewed Action commits and verified release tooling.

**Independent Test**: Scan every workflow and prove mutable, credential-persisting, privileged, or unverifiable fixtures fail closed.

- [x] T020 [US2] Add failing tests for mutable Action refs, missing comments, checkout credentials, privileged triggers, unexpected writes, secrets, release downloads, and release-tool versions.
- [x] T021 [US2] Implement `.github/scripts/trust-policy.mjs` with pure validators and a repository entry point.
- [x] T022 [US2] Pin all Action dependencies to resolved commits with release comments.
- [x] T023 [US2] Disable checkout credential persistence in every workflow.
- [x] T024 [US2] Pin the AppImage tool digest and exact `cargo-wix` and `cargo-deb` versions in the release workflow.
- [x] T025 [US2] Run the trust-policy tests from the Linux CI job and make the complete repository scan green.

## Phase 6: User Story 3 - Enforce the Integration Contract (P1)

**Goal**: Make pull requests, CI, review resolution, and immutable Actions hosted enforcement rather than convention.

**Independent Test**: Query effective repository settings and compare every value with the workflow-integrity contract.

- [x] T026 [US3] Add `.github/CODEOWNERS` ownership for governance, workflow, release, agent-guidance, and policy surfaces.
- [x] T027 [US3] Configure `main` branch protection with strict required checks, administrator enforcement, linear history, conversation resolution, and destructive-operation blocking.
- [x] T028 [US3] Restrict Action publishers and enable commit-SHA pinning while retaining read-only default workflow tokens.
- [x] T029 [US3] Query and record high-level verification that effective hosted settings match the contract.

## Phase 7: User Story 4 - Preserve Useful Public Evidence (P2)

**Goal**: Document the trust model and outcome without disclosing sensitive finding details.

**Independent Test**: Review all public changes and confirm they contain control-level evidence only.

- [x] T030 [US4] Update Plan 043 and its index chronologically for S099.
- [x] T031 [US4] Update `CHANGELOG.md` with the governance hardening and dated pinned-artifact decisions.
- [x] T032 [US4] Update the managed spec-kit context in `CLAUDE.md`.
- [x] T033 [US4] Review public artifacts for detailed findings, reusable abuse instructions, or secret values.

## Phase 8: Local Validation and Review

- [x] T034 Run focused trust-policy tests and the complete repository trust scan.
- [x] T035 Run `cargo fmt --all -- --check`, strict Clippy, all locked tests, and the release build.
- [x] T036 Run documentation policy, render tests, mdBook test/build, spelling, and link validation.
- [x] T037 Run spec prerequisites, diff, UTF-8/BOM/mojibake, forbidden-dash, and scope scans.
- [x] T038 Review the complete diff for authority confusion, permission expansion, release drift, policy bypass, disclosure errors, and scope drift.

## Phase 9: Publication and Hosted Review

- [x] T039 Commit S099 with changelog evidence and the Codex co-author trailer.
- [ ] T040 Push the authorized branch and publish an official PR that closes #181.
- [ ] T041 Move issue and PR project items to S099 PR review when project tracking is available.
- [ ] T042 Wait for CI, CodeQL, dependency, Codex, and security results; address every comment and resolve every thread.
- [ ] T043 Trigger exactly one authorized second `@Codex review` round and address its results.
- [ ] T044 Confirm all required checks green, zero unresolved threads, and ask the operator for final review and merge.

## Dependencies

- Phases 1 and 2 precede Phase 3 because analysis covers the complete design.
- Phase 3 blocks all implementation changes.
- The authority model precedes removal and guidance changes.
- Policy tests precede workflow hardening.
- Workflow files are pinned before hosted SHA enforcement is enabled.
- Hosted settings and documentation follow local policy validation.
- Publication follows all local and hosted gates.
