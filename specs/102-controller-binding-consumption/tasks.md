# Tasks: Controller Binding Consumption

**Input**: Design documents from `/specs/102-controller-binding-consumption/`

## Phase 1: Specification and contract

- [x] T001 Complete specification, checklist, research, plan, data model, execution contract, and quickstart
- [x] T002 Reconcile issue #208, parent #188, S100 evidence, S101 execution, controller gates, settings, and documentation
- [x] T003 Run the spec-kit analysis gate and resolve every blocking inconsistency

## Phase 2: Foundational tests

- [x] T004 [US1] Add failing Fishing native binding, scheduled invalidation, chord ownership, and rejection tests
- [x] T005 [US2] Add failing Auto Potion native binding, retry accounting, chord ownership, and rejection tests
- [x] T006 [US1] [US2] Add failing binding replacement and signal-loss pre-lock routing tests
- [x] T007 [US3] Add failing legacy persistence, settings presentation, and toggle regression tests

## Phase 3: Shared native authority

- [x] T008 Generalize Fishing gates into shared autonomous gates with binding snapshot and physical modifier authority
- [x] T009 Invalidate both combat and autonomous generations before coherent binding replacement
- [x] T010 Implement complete native action chord execution with canonical ownership and cleanup

## Phase 4: Controller consumption

- [x] T011 [US1] Move Fishing to native Interact attempts and truthful stopped presentation
- [x] T012 [US2] Move Auto Potion to native Quickslot attempts and success-based retry accounting
- [x] T013 [US1] [US2] Route real sinks and binding state through worker lifecycle and signal loss

## Phase 5: Configuration and presentation migration

- [x] T014 [US3] Remove duplicate typed and persisted gameplay keys while accepting legacy JSON
- [x] T015 [US3] Replace editable controls with read-only live binding status and remediation
- [x] T016 [US3] Update tests and deterministic documentation captures for the migrated layout

## Phase 6: Documentation and completion

- [x] T017 Update input safety, Fishing, Auto Potion, settings, troubleshooting, Plan 043 chronology/index, and `CHANGELOG.md`
- [x] T018 Run focused tests, formatting, strict linting, complete locked tests, release build, docs, trust, spelling, links, encoding, and dash scans
- [x] T019 Review the complete diff for issue #208 scope, safety regression, migration integrity, and parent #188 completion
- [x] T020 Commit, push, publish the official PR, and move project status to PR review
- [ ] T021 Address every review item, request exactly one second `@Codex review` round, and reach green CI with no unresolved thread

## Dependencies and execution order

- T003 blocks implementation.
- T004 through T007 establish failing tests before T008.
- T008 through T010 establish authority before T011 through T013.
- T011 through T013 establish runtime behavior before T014 through T016 remove old authority.
- T017 through T021 complete delivery.
