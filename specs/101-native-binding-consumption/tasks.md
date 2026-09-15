# Tasks: Native Binding Consumption

**Input**: Design documents from `/specs/101-native-binding-consumption/`

**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`

## Phase 1: Specification and contract

- [x] T001 Complete specification, clarification checklist, research, plan, data model, execution contract, and quickstart in `specs/101-native-binding-consumption/`
- [x] T002 Reconcile issue #207, parent #188, S100 evidence, existing gates, platform seams, configuration, and S102 boundary
- [x] T003 Run the spec-kit analysis gate and resolve every blocking inconsistency

## Phase 2: Foundational tests

- [x] T004 [US1] Add failing exact native chord, complete-plan, ambiguity, and binding-replacement tests in `tests/input_engine.rs`
- [x] T005 [US2] Add failing pre-lock invalidation and signal-loss routing tests in `src/app/routing.rs` and `tests/app_view_model.rs`
- [x] T006 [US3] Add failing chord sequence, modifier ownership, cancellation, and failure cleanup tests in `tests/weave_sequence.rs` and `tests/weave_engine.rs`
- [x] T007 [US3] Add failing toggle-only settings migration and label tests in `tests/input_engine.rs`, `tests/config.rs`, `tests/app_settings.rs`, and `tests/app_view_model.rs`

## Phase 3: Native combat authority

- [x] T008 [US1] Extend native input types with modifiers, physical events, plan requirements, and immutable chord plans in `src/input/native.rs` and `src/input/mod.rs`
- [x] T009 [US1] Store coherent binding evidence, collision state, plan requirements, physical modifier state, and binding invalidation in `src/input/mod.rs`
- [x] T010 [US1] Replace combat key lookup with exact native admission while preserving pass-through lifecycles, repeats, inactive actions, and toggle exemption in `src/input/mod.rs`
- [x] T011 [US2] Route binding replacement and loss through the pre-lock safety boundary in `src/app/routing.rs`

## Phase 4: Native weave execution

- [x] T012 [US1] Remove duplicated slot keys and build all four pure weave sequences from immutable skill, Attack, and Block chords in `src/weave/types.rs`, `src/weave/sequence.rs`, and `src/weave/mod.rs`
- [x] T013 [US3] Implement temporary modifier and held-primary ownership with cancellation-safe cleanup in `src/weave/mod.rs`
- [x] T014 [US2] Pass queued plans and physical modifier authority through the worker in `src/main.rs`

## Phase 5: Platform coverage

- [x] T015 [US1] Expand Windows keyboard/mouse hooks, injected-event rejection, native synthesis, modifiers, buttons, wheel, and exhaustive maps in `src/input/windows.rs`
- [x] T016 [US1] Expand Linux keyboard/pointer selection, descriptor polling, forwarding, union uinput capabilities, modifiers, buttons, wheel, and exhaustive maps in `src/input/linux.rs`
- [x] T017 [US3] Extend the mock and shared backend seams for native controls and modifiers in `src/input/mock.rs` and `src/main.rs`

## Phase 6: Configuration and presentation migration

- [x] T018 [US3] Narrow `BindingTable` persistence and collision handling to F1, F2, and F3 in `src/input/bindings.rs` and `src/input/action.rs`
- [x] T019 [US3] Remove combat rows from settings, remove hardcoded weave key labels, and preserve toggle editing in `src/app/settings_form.rs`, `src/app/ui.rs`, and `src/app/mod.rs`
- [x] T020 [US3] Update affected tests and documentation captures without changing Fishing or Auto Potion gameplay configuration

## Phase 7: Documentation and completion

- [x] T021 Update input safety, weaving, settings, troubleshooting, Plan 043 chronology/index, and `CHANGELOG.md`
- [x] T022 Run focused tests, formatting, strict linting, complete locked tests, release build, docs, trust, spelling, links, encoding, and dash scans
- [x] T023 Review the complete diff for issue #207 scope, mutation prohibition, safety regression, and S102 deferral
- [ ] T024 Commit, push, publish the official PR, move project status to PR review, and monitor hosted CI
- [ ] T025 Address every review item, request exactly one second `@Codex review` round, and reach green CI with no unresolved review thread

## Dependencies and execution order

- T003 blocks implementation.
- T004 through T007 establish required failing tests before T008.
- T008 through T011 establish authority before T012 through T017.
- T012 through T017 establish runtime behavior before configuration authority is removed in T018 through T020.
- T021 through T025 complete delivery after all behavior is green.
