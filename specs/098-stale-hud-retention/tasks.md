# Tasks: Stale HUD Retention

**Input**: Design documents from `/specs/098-stale-hud-retention/`

**Prerequisites**: `spec.md`, `plan.md`, `research.md`, `data-model.md`, `contracts/`, and `quickstart.md`

**Tests**: Required by issue #171, the specification, and Constitution Principle III.

## Phase 1: Specification and Clarification

- [x] T001 Create the S098 feature context with the repository spec-kit scripts.
- [x] T002 Snapshot GitHub issue #171 in `specs/098-stale-hud-retention/issue-171.md`.
- [x] T003 Author prioritized journeys, acceptance scenarios, requirements, outcomes, and boundaries in `spec.md`.
- [x] T004 Resolve control, coherence, cause, clock, expiry, settings-edit, and process-lifetime decisions.
- [x] T005 Complete `checklists/requirements.md` and `checklists/presentation-safety.md`.

## Phase 2: Planning and Contracts

- [x] T006 Record alternatives and decisions in `research.md`.
- [x] T007 Define settings, presentation snapshot, retention state, causes, transitions, and invariants in `data-model.md`.
- [x] T008 Define the presentation state-machine contract in `contracts/presentation-retention.md`.
- [x] T009 Define the persisted bounds and interface contract in `contracts/settings.md`.
- [x] T010 Produce the implementation and verification plan in `plan.md`.
- [x] T011 Produce automated and manual verification instructions in `quickstart.md`.

## Phase 3: Spec-Kit Analysis Gate

- [x] T012 Analyze issue, spec, research, model, contracts, plan, checklists, and tasks in `analysis.md`.
- [x] T013 Resolve every analysis finding and confirm no unresolved clarification marker remains.
- [x] T014 Run spec-kit prerequisites with tasks required.

## Phase 4: User Story 3 - Configure Bounded Retention (P2)

**Goal**: Persist and edit a precise whole-second interval from 0 through 999.

**Independent Test**: Load, edit, save, and reload missing, boundary, and invalid values.

- [x] T015 [US3] Add failing default, bound, exact round-trip, and invalid-value tests in `tests/app_settings.rs`.
- [x] T016 [US3] Add failing settings string and layout tests in `tests/app_strings.rs` and `tests/app_ui_sizing.rs`.
- [x] T017 [US3] Add the validated preference in `src/app/settings_form.rs`.
- [x] T018 [US3] Add setting labels, help, and bounded numeric control in `src/app/strings.rs` and `src/app/ui.rs`.
- [x] T019 [US3] Make focused settings and interface tests green.

## Phase 5: User Story 1 - Retain the Last Coherent HUD (P1)

**Goal**: Preserve one coherent rendered snapshot across every covered loss until one monotonic deadline.

**Independent Test**: Project a coherent view, cause each loss at deterministic times, and compare every retained field plus stale reason and age.

- [x] T020 [US1] Add failing deterministic tests for inactive, runtime-unavailable, unfocused, focus-unavailable, and signal-unavailable causes in `tests/app_view_model.rs`.
- [x] T021 [US1] Add failing no-prior-snapshot, age, cause-change, zero, expiry, and repeated-expiry tests in `tests/app_view_model.rs`.
- [x] T022 [US1] Add display-only presentation, cause, and interval types in `src/app/mod.rs`.
- [x] T023 [US1] Add one process-local retention state and deterministic `view_at` projection in `src/app/mod.rs`.
- [x] T024 [US1] Add the accessible HUD freshness row in `src/app/strings.rs` and `src/app/ui.rs`.
- [x] T025 [US1] Make focused retention and presentation tests green.

## Phase 6: User Story 2 - Recover Fresh Presentation (P1)

**Goal**: Replace stale values immediately when coherent evidence returns.

**Independent Test**: Recover with different values before expiry, pass the old deadline, and begin a later independent loss.

- [x] T026 [US2] Add failing recovery, superseded-deadline, and later-loss tests in `tests/app_view_model.rs`.
- [x] T027 [US2] Implement coherent snapshot replacement and stale-interval cancellation in `src/app/mod.rs`.
- [x] T028 [US2] Make focused recovery tests green.

## Phase 7: User Story 4 - Preserve Immediate Safety Response (P1)

**Goal**: Prove retained views are structurally and behaviorally separate from current action authority.

**Independent Test**: Retain visible fields while underlying routing clears engine observations and closes input, Fishing, and Auto Potion gates.

- [x] T029 [US4] Add failing integration assertions for retained display plus cleared authoritative engine fields in `tests/app_view_model.rs`.
- [x] T030 [US4] Add immediate input and controller gate assertions for runtime, focus, and signal losses in `tests/app_view_model.rs`.
- [x] T031 [US4] Keep controller, input, addon, protocol, and persistence semantics unchanged; limit routing changes to shared loss-timestamp propagation.
- [x] T032 [US4] Re-run weave, input, Fishing, Auto Potion, game-state, and Pixel Bus safety suites.

## Phase 8: Canonical Documentation and Planning

- [x] T033 Update interface and settings documentation with the control, causes, age, recovery, zero, and accessibility behavior.
- [x] T034 Update configuration, architecture, test-strategy, and troubleshooting documentation with process-local retention and immediate action-gate separation.
- [x] T035 Update Plan 043 and its index chronologically for S098 completion and the next work direction.
- [x] T036 Update `CHANGELOG.md` Added and Decisions entries.
- [x] T037 Update `CLAUDE.md` spec-kit pointer to the S098 plan.

## Phase 9: Local Validation and Review

- [x] T038 Run focused settings, application-model, string, sizing, and safety tests.
- [x] T039 Run `cargo fmt --all -- --check`, strict Clippy, all locked tests, and the release build.
- [x] T040 Run documentation policy, render smoke, mdBook test/build, spelling, and link validation.
- [x] T041 Run spec prerequisites, diff, UTF-8/BOM/mojibake, forbidden-dash, and scope scans.
- [x] T042 Review the complete diff for stale-authority leaks, deadline extension, accessibility loss, documentation contradictions, and scope drift.

## Phase 10: Publication and Hosted Review

- [x] T043 Commit S098 with changelog evidence and the Codex co-author trailer.
- [x] T044 Push the authorized branch and publish an official PR that closes #171.
- [x] T045 Move issue and PR project items to S098 PR review.
- [ ] T046 Wait for CI, CodeQL, dependency, Codex, and security results; address every comment and resolve every thread.
- [ ] T047 Trigger at most one authorized second `@Codex review` round and address its results.
- [ ] T048 Confirm all required checks green, zero unresolved threads, and ask the operator for final review and merge.

## Dependencies

- Phases 1 and 2 precede Phase 3 because analysis covers the complete design.
- Phase 3 blocks implementation.
- Settings establish the interval before presentation state uses it.
- Retention establishes the state machine before recovery and safety-separation integration.
- Documentation follows implemented behavior.
- Publication follows all local gates and review.
