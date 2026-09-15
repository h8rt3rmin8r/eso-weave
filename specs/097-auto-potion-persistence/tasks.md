# Tasks: Persistent Auto Potion Request

**Input**: Design documents from `/specs/097-auto-potion-persistence/`

**Prerequisites**: `spec.md`, `plan.md`, `research.md`, `data-model.md`,
`contracts/`, and `quickstart.md`

**Tests**: Required by issue #172, the specification, and Constitution Principle III.

## Phase 1: Specification and Clarification

- [x] T001 Create the S097 feature context with the repository spec-kit scripts.
- [x] T002 Snapshot GitHub issue #172 in `specs/097-auto-potion-persistence/issue-172.md`.
- [x] T003 Author prioritized user journeys, acceptance scenarios, requirements, and measurable outcomes in `spec.md`.
- [x] T004 Reconcile the newer issue authority with S039 R7 and S043 FR-002 in `spec.md`.
- [x] T005 Resolve clarification decisions for storage ownership, schema versioning, restore timing, and invalid data.
- [x] T006 Complete `checklists/requirements.md`.
- [x] T007 Complete `checklists/safety-and-persistence.md`.

## Phase 2: Planning and Contracts

- [x] T008 Record alternatives and decisions in `research.md`.
- [x] T009 Define SessionState v4 and controller authority in `data-model.md`.
- [x] T010 Define the versioned file contract in `contracts/session-state-v4.md`.
- [x] T011 Define the no-input restore contract in `contracts/startup-restore.md`.
- [x] T012 Produce the implementation and verification plan in `plan.md`.
- [x] T013 Produce repository and packaged-smoke instructions in `quickstart.md`.

## Phase 3: Spec-Kit Analysis Gate

- [x] T014 Analyze issue, spec, research, model, contracts, plan, checklist, and tasks consistency in `analysis.md`.
- [x] T015 Resolve every analysis finding and confirm no NEEDS CLARIFICATION marker remains.
- [x] T016 Run spec-kit prerequisites with tasks required.

## Phase 4: User Story 1 - Persist the Request (P1)

**Goal**: Enabled and disabled requests survive settled and close-time saves.

**Independent Test**: Toggle, save, load, and restore both values through a fresh model.

- [x] T017 [US1] Add failing state round-trip and current-version tests in `src/config/state.rs` and `tests/app_session_state.rs`.
- [x] T018 [US1] Add failing UI and F3 coalesced-persistence tests in `tests/app_session_state.rs`.
- [x] T019 [US1] Add a failing close-before-settle persistence test in `tests/app_session_state.rs`.
- [x] T020 [US1] Add the defaulted `auto_potion` field and schema version 4 in `src/config/state.rs`.
- [x] T021 [US1] Include the controller request in `AppModel::current_session_state` in `src/app/mod.rs`.
- [x] T022 [US1] Mark `UiIntent::SetAutoPotion` as session-persistent in `src/app/mod.rs`.
- [x] T023 [US1] Make the focused persistence tests green and refactor duplication.

## Phase 5: User Story 2 - Restore Fail-Closed (P1)

**Goal**: A remembered request is restored without becoming independent input authority.

**Independent Test**: Restore true under startup defaults and verify requested-on plus a dormant or blocked effective state and zero input path.

- [x] T024 [US2] Add failing enabled and disabled model-restore tests in `tests/app_session_state.rs`.
- [x] T025 [US2] Add a failing startup-state assertion proving restored enablement remains dormant or blocked in `tests/app_session_state.rs`.
- [x] T026 [US2] Restore both Boolean values through `AutoPotionController::set_enabled` in `src/app/mod.rs` without ticking or scheduling.
- [x] T027 [US2] Update source documentation in `src/app/mod.rs`, `src/config/state.rs`, `src/config/mod.rs`, and `src/main.rs` to match the new authority.
- [x] T028 [US2] Re-run focused Auto Potion and application safety tests to prove unchanged gates.

## Phase 6: User Story 3 - Upgrade Safely (P2)

**Goal**: Legacy state defaults off and malformed state fails as a whole.

**Independent Test**: Load legacy versions and malformed field types, then verify preservation or complete safe fallback with a notice.

- [x] T029 [US3] Add failing legacy-version default and unrelated-field preservation tests in `src/config/state.rs`.
- [x] T030 [US3] Add failing malformed request tests in `tests/app_session_state.rs`.
- [x] T031 [US3] Confirm serde defaulting and strict Boolean decoding satisfy the migration and fallback contract without custom coercion.
- [x] T032 [US3] Validate saved JSON formatting, schema, field presence, LF ending, and no BOM.

## Phase 7: Canonical Documentation and Planning Rollover

- [x] T033 Update `docs/src/features/auto-potion.md` with restart behavior and fail-closed restoration.
- [x] T034 Update `docs/src/reference/configuration.md` and `docs/src/reference/settings.md` to place request authority in `state.json`.
- [x] T035 Update affected startup, troubleshooting, architecture, and test-strategy prose without historical caveats.
- [x] T036 Archive Plan 042, update the archive index, open Plan 043, and update the current build-plan index chronologically.
- [x] T037 Update `CHANGELOG.md` Added and Decisions entries with the explicit S039/S043 deviation.
- [x] T038 Update `CLAUDE.md` spec-kit pointer to the S097 plan.

## Phase 8: Local Validation and Review

- [x] T039 Run focused session-state, application-model, and Auto Potion tests.
- [x] T040 Run `cargo fmt --all -- --check`, strict Clippy, all locked tests, and the release build.
- [x] T041 Run documentation policy, render smoke, mdBook test/build, spelling, and link validation.
- [x] T042 Run spec prerequisites, diff, UTF-8/BOM/mojibake, forbidden-dash, and scope scans.
- [x] T043 Perform independent code, safety, and documentation consistency review and fix every high-confidence finding.

## Phase 9: Publication and Hosted Review

- [x] T044 Commit S097 with changelog evidence and the Codex co-author trailer.
- [ ] T045 Push the authorized branch and publish an official PR that closes #172.
- [ ] T046 Move issue and PR project items to S097 PR review.
- [ ] T047 Wait for CI, CodeQL, dependency, Codex, and security results; address every comment and resolve every thread.
- [ ] T048 Trigger at most one authorized second `@Codex review` round and address its results.
- [ ] T049 Confirm all required checks green, zero unresolved threads, and ask the operator for final review and merge.

## Dependencies

- Phases 1 and 2 precede Phase 3 because analysis covers the complete design.
- Phase 3 blocks implementation.
- User Story 1 establishes the persisted field before User Story 2 restores it.
- User Story 3 validates backward compatibility after the field exists.
- Documentation and plan rollover follow the implemented behavior.
- Publication follows all local gates and review.
