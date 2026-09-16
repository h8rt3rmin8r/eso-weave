# Tasks: Local Service Lifecycle

**Input**: Design documents from `/specs/105-local-service-lifecycle/`

## Phase 1: Spec-Kit Foundation

- [x] T001 Create the S105 spec-kit workspace from issue #176 and Plan 045.
- [x] T002 Resolve lifecycle, persistence, and slice-boundary clarifications under autopilot.
- [x] T003 Complete specification and domain quality checklists.
- [x] T004 Produce research, data model, lifecycle contract, quickstart, and implementation plan.

## Phase 2: Blocking Analysis and Test Seams

- [x] T005 Run the spec-kit analysis gate and resolve every blocking finding.
- [x] T006 Add exact S104-selected server dependencies and minimal required features in `Cargo.toml` and `Cargo.lock`.
- [x] T007 [P] Add failing config persistence and credential-validation tests in `tests/config.rs` and `tests/local_service.rs`.
- [x] T008 [P] Add failing controller, request-boundary, discovery, collision, rapid-toggle, and exit tests in `tests/local_service.rs`.
- [x] T009 [P] Add failing settings-form and headless UI coverage in `tests/app_settings.rs` and `tests/app_ui_sizing.rs`.

## Phase 3: User Story 1 - Enable One Coherent Service

- [x] T010 [US1] Add `LocalServicePrefs` parsing, serialization, defaulting, and secret generation in `src/local_service.rs` and `src/config/mod.rs`.
- [x] T011 [US1] Implement lifecycle status, commands, controller, and dedicated runtime owner in `src/local_service.rs`.
- [x] T012 [US1] Build the authenticated Axum router and minimal stateless RMCP handler in `src/local_service.rs`.
- [x] T013 [US1] Implement atomic discovery publication and ownership-checked cleanup in `src/local_service.rs`.

## Phase 4: User Story 2 - Truthful UI and Failures

- [x] T014 [US2] Integrate enablement and status with `AppModel` and settings auto-save in `src/app/mod.rs` and `src/app/settings_form.rs`.
- [x] T015 [US2] Add warning, toggle, status, endpoint, failure, and explicit credential-copy controls in `src/app/strings.rs` and `src/app/ui.rs`.
- [x] T016 [US2] Add transition-only secret-free lifecycle diagnostics.

## Phase 5: User Story 3 - Stop and Recover

- [x] T017 [US3] Implement cancellation, latest-intent convergence, failure retry, owned cleanup, and bounded controller drop.
- [x] T018 [US3] Wire application exit through the controller shutdown path and verify port reuse.
- [x] T019 [US3] Complete rapid-toggle, partial-start, re-enable, and exit integration tests.

## Phase 6: Project Integration and Delivery

- [x] T020 Export the lifecycle module and complete focused config, service, model, and UI tests.
- [x] T021 Update Plan 045 current-slice tracking and record the dependency/lifecycle decision in `CHANGELOG.md`.
- [x] T022 Mark the S105 spec, analysis, and task packet implemented after verification.
- [x] T023 Run formatting, clippy, full locked tests, docs, links, spelling, encoding, mojibake, dependency, and diff review gates.
- [x] T024 Commit as `feat(105): add local service lifecycle`, push, and open the official PR closing issue #176.
- [ ] T025 Resolve all first-round CI and review feedback, request no more than one authorized second Codex round, and stop for the operator merge ritual.

## Dependencies and Execution Order

- Phase 1 is complete before code changes.
- T005 blocks implementation.
- T006 blocks compilation; T007 through T009 establish red tests before T010 through T018.
- UI integration depends on the controller and persisted preference model.
- Delivery begins only after every requirement, test, and analysis item is complete.

## Implementation Strategy

1. Prove default-off persistence and the secret boundary.
2. Prove one real ephemeral listener with authenticated MCP initialization.
3. Prove every failure leaves no half-service or foreign discovery mutation.
4. Integrate the non-blocking control and status into the existing auto-save settings flow.
5. Verify shutdown and recovery before project tracking and publication.
