# Tasks: Documentation Flow Diagrams

**Input**: Design documents from `/specs/082-docs-flow-diagrams/`

**Prerequisites**: `spec.md`, `plan.md`, `research.md`, `data-model.md`, `contracts/`

## Phase 1: Setup and audit

- [x] T001 Confirm issue #123, parent #119, and Plan 039 traceability in `spec.md`
- [x] T002 Record selected and rejected visual candidates in `research.md`
- [x] T003 Define the four page, asset, alternative, and text-equivalent records in `contracts/diagram-delivery.md`

## Phase 2: User Story 1 - Understand ownership and authorization

**Goal**: Make the separate ownership paths and fail-closed authorization convergence visible.

**Independent Test**: Source and generated policy accept the architecture and authorization diagrams and reject missing references, incomplete alternatives, or lost fail-closed labels.

- [x] T004 [US1] Add failing S082 page-reference, alternative, equivalent, and generated-output tests in `.github/scripts/docs-policy.test.mjs`
- [x] T005 [US1] Add the four-record manifest and pure page/generated validators in `.github/scripts/docs-policy.mjs`
- [x] T006 [US1] Author `docs/src/assets/diagrams/architecture-ownership.svg`
- [x] T007 [US1] Author `docs/src/assets/diagrams/action-authorization.svg`
- [x] T008 [US1] Add the ownership diagram and complete text equivalent to `docs/src/development/architecture.md`
- [x] T009 [US1] Add the authorization diagram and complete text equivalent to `docs/src/concepts/action-authorization.md`

## Phase 3: User Story 2 - Follow safety recovery and frame validation

**Goal**: Show the correctness-bearing closure, synchronization, validation, invalidation, and recovery order.

**Independent Test**: Source and generated policy accept the recovery and validation diagrams and reject horizontal flow, unsafe SVG content, missing accessibility metadata, or missing branch outcomes.

- [x] T010 [US2] Add failing S082 SVG structure, safety, accessibility, direction, and outcome tests
- [x] T011 [US2] Add pure static SVG validation to `.github/scripts/docs-policy.mjs`
- [x] T012 [US2] Author `docs/src/assets/diagrams/safety-recovery.svg`
- [x] T013 [US2] Author `docs/src/assets/diagrams/pixel-bus-validation.svg`
- [x] T014 [US2] Add the recovery diagram and complete text equivalent to `docs/src/development/state-machines.md`
- [x] T015 [US2] Add the frame-validation diagram and complete text equivalent to `docs/src/reference/pixel-bus-protocol.md`

## Phase 4: User Story 3 - Read diagrams in every delivery mode

**Goal**: Keep the four diagrams responsive, theme-safe, local, and equivalent in public and bundled documentation.

**Independent Test**: Built output contains all assets and meaningful image semantics, while CSS and policy mutations detect overflow or remote-renderer regressions.

- [x] T016 [US3] Add failing responsive CSS, remote-runtime, and generated-asset mutation tests
- [x] T017 [US3] Add `.docs-flow-diagram` containment styles to `docs/theme/eso-weave.css`
- [x] T018 [US3] Connect all four sources, generated pages, assets, and CSS to documentation policy
- [x] T019 [US3] Build and inspect narrow and wide light/navy output with images enabled and disabled

## Phase 5: Governance and traceability

- [x] T020 Update S081 completion and S082 progress in `docs/project/build-plans/plan-039.md`
- [x] T021 Add spec 082 and merged S081 evidence to `docs/project/migration-ledger.json`
- [x] T022 Add S082 Added and dated Decisions entries to `CHANGELOG.md`
- [x] T023 Mark the spec implemented and complete post-implementation analysis

## Phase 6: Verification and delivery

- [x] T024 Run every command in `quickstart.md`
- [x] T025 Run UTF-8, LF, mojibake, forbidden-dash, JSON, SVG, and diff-integrity gates
- [ ] T026 Push `codex/s082-docs-flow-diagrams` and open the official PR closing #123
- [ ] T027 Process every first-round review finding and hosted CI result
- [ ] T028 If useful, request at most one authorized second Codex review and process it fully
- [ ] T029 Confirm required checks are green and every review thread is resolved before requesting final merge review

## Dependencies and execution order

- T001 through T003 precede implementation.
- T004, T010, and T016 are failing tests written before validators, assets, pages, or CSS.
- T005 precedes T008 and T009; T011 precedes T012 through T015.
- T017 and T018 follow the four page integrations.
- T020 through T023 follow functional implementation.
- T024 and T025 precede publication.
- T026 through T029 are ordered remote-delivery gates.

## Implementation strategy

Treat the four static vectors, their page references, and their complete text equivalents as one offline documentation contract. Validate the files that ship rather than introducing a generator or browser runtime.
