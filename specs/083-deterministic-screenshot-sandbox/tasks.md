# Tasks: Deterministic Screenshot Sandbox

**Input**: Design documents from `/specs/083-deterministic-screenshot-sandbox/`

**Prerequisites**: `spec.md`, `plan.md`, `research.md`, `data-model.md`, `contracts/`

## Phase 1: Setup and authority

- [x] T001 Confirm issue #124, parent #119, dependency #125, and Plan 039 traceability
- [x] T002 Record target-placement, renderer, fixture, output, and determinism decisions in `research.md`
- [x] T003 Define the invocation, catalog, variant, isolation, filesystem, and completion contracts

## Phase 2: User Story 1 - Reproduce representative states

**Goal**: Generate the complete deterministic application-state matrix through one command.

**Independent Test**: Generate into two target directories and compare manifests, dimensions, and same-environment hashes.

- [x] T004 [US1] Add the no-harness target and observe the target fail before the support implementation is green
- [x] T005 [US1] Add the exact scene, theme, viewport, variant, and receipt types
- [x] T006 [US1] Add explicit output parsing, repository containment, and validation-only execution
- [x] T007 [US1] Add the seven isolated scene fixture builders and pre-render view assertions
- [x] T008 [US1] Add bundled-font headless rendering and exact PNG output
- [x] T009 [US1] Add canonical manifest publication after complete success

## Phase 3: User Story 2 - Trust the isolation boundary

**Goal**: Prove capture cannot access production input, screen, addon, configuration, or startup capabilities.

**Independent Test**: Run target self-checks and the production release build inspection.

- [x] T010 [US2] Add forbidden-import and forbidden-call isolation checks
- [x] T011 [US2] Assert no input action is emitted by every rendered fixture
- [x] T012 [US2] Test path escape, repository-root, non-directory, and symlink rejection
- [x] T013 [US2] Test Cargo target placement and absence of production features or CLI routes
- [x] T014 [US2] Verify the release build contains no capture entry point

## Phase 4: User Story 3 - Select stable variants

**Goal**: Give documentation authors stable names, values, dimensions, and metadata.

**Independent Test**: Validate all 28 ordered receipts and inspect their pre-render state expectations.

- [x] T015 [US3] Add exact catalog completeness, ordering, ID, title, and filename tests
- [x] T016 [US3] Add theme and viewport matrix cardinality and dimension tests
- [x] T017 [US3] Add deterministic data and personal-data exclusion checks
- [x] T018 [US3] Add representative same-process image reproducibility evidence

## Phase 5: Governance and documentation

- [x] T019 Document validation and capture commands in `docs/src/development/test-strategy.md`
- [x] T020 Update S082 completion and S083 progress in `docs/project/build-plans/plan-039.md`
- [x] T021 Add spec 083 and merged S082 evidence to `docs/project/migration-ledger.json`
- [x] T022 Add S083 Added and dated Decisions entries to `CHANGELOG.md`
- [x] T023 Mark the spec implemented and complete post-implementation analysis

## Phase 6: Verification and delivery

- [x] T024 Run every command in `quickstart.md`
- [x] T025 Run UTF-8, LF, mojibake, forbidden-dash, JSON, and diff-integrity gates
- [ ] T026 Push `codex/s083-deterministic-screenshot-sandbox` and open the official PR closing #124
- [ ] T027 Process every first-round review finding and hosted CI result
- [ ] T028 If useful, request at most one authorized second Codex review and process it fully
- [ ] T029 Confirm required checks are green and every review thread is resolved before requesting final merge review

## Dependencies and execution order

- T001 through T003 precede implementation.
- T004 must fail before T005 through T009 supply the implementation.
- T005 and T006 precede fixture and renderer work.
- T007 precedes T008; T008 precedes the complete manifest in T009.
- T010 through T014 complete the isolation boundary after the executable path exists.
- T015 through T018 validate the completed matrix.
- T019 through T023 follow functional implementation.
- T024 and T025 precede publication.
- T026 through T029 are ordered remote-delivery gates.

## Implementation strategy

Keep capture as a development test executable, exercise the real frame seam, and validate model truth before producing pixels. Generate evidence into `target/`; do not check screenshot content into S083.
