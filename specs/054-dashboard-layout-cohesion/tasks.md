# Tasks: Dashboard Layout Cohesion

**Input**: S054 design documents in `specs/054-dashboard-layout-cohesion/`
**Tests**: Required by the constitution and issue contracts

## Phase 1: Spec-kit setup and analysis

- [x] T001 Sync main, create `codex/s054-dashboard-layout-cohesion`, and bind #72-#75
- [x] T002 Complete specification and requirements checklist
- [x] T003 Resolve autopilot clarifications in research, model, contract, and quickstart
- [x] T004 Add chronological build plan 024 and run blocking cross-artifact analysis

## Phase 2: Red tests

- [x] T005 [US1] Add pure expanded/collapsed breakpoint tests in `src/app/mod.rs`
- [x] T006 [US1] Add rendered equal width, height, and symmetric growth tests in `tests/app_ui_sizing.rs`
- [x] T007 [US1] Add persisted and runtime collapse/resize/log transition tests in `tests/app_ui_sizing.rs`
- [x] T008 [US2] Add control-origin, equal-button, horizontal-group, and action tests in `tests/app_ui_sizing.rs`
- [x] T009 [US3] Add flexible-value, keyboard-detail, and three/four-resource gap tests in `tests/app_ui_sizing.rs`
- [x] T010 [US4] Add exact field-label registry and superseded-copy tests in `tests/app_strings.rs`

## Phase 3: User Story 1, paired dashboard geometry

- [x] T011 [US1] Implement collapse-aware effective layout in `src/app/mod.rs`
- [x] T012 [US1] Replace capped asymmetric width allocation in `src/app/ui.rs`
- [x] T013 [US1] Coordinate expanded card height from Live HUD in `src/app/ui.rs`
- [x] T014 [US1] Integrate collapse transitions with log and intrinsic sizing protection

## Phase 4: User Story 2, stable interactions

- [x] T015 [US2] Add the shared dashboard row allocator in `src/app/ui.rs`
- [x] T016 [US2] Reserve one fixed two-button interaction column
- [x] T017 [US2] Render equal lifecycle buttons horizontally at the shared origin
- [x] T018 [US2] Give every toggle a meaningful accessible name and preserve intents

## Phase 5: User Story 3, complete Live HUD information

- [x] T019 [US3] Let dynamic values consume the flexible row region
- [x] T020 [US3] Expose constrained full text on hover, focus, and accessibility nodes
- [x] T021 [US3] Render one gap after a count-agnostic resource group
- [x] T022 [US3] Verify production three-meter and synthetic four-meter geometry

## Phase 6: User Story 4, concise title case

- [x] T023 [US4] Apply the seven exact dashboard label migrations in `src/app/strings.rs`
- [x] T024 [US4] Add and audit the bounded field/settings label registry
- [x] T025 [US4] Update view-model, rendered, and accessibility copy expectations

## Phase 7: Documentation and local validation

- [x] T026 Update master specification, README, changelog, and plan 024
- [x] T027 Run focused model, sizing, resource, system-state, and strings suites
- [x] T028 Run fmt, clippy with warnings denied, and all locked tests
- [x] T029 Run diff, forbidden-text, UTF-8-no-BOM, LF, generated-file, and secret audits
- [x] T030 Complete layout checklist and post-implementation analysis

## Phase 8: Delivery and hosted review

- [x] T031 Commit with the S054 conventional subject and required coauthor trailer
- [x] T032 Push and publish a pull request with separate `Closes` lines for #72-#75
- [ ] T033 Resolve hosted CI and every first-round review comment
- [ ] T034 Request exactly one second `@Codex` review and resolve every result
- [ ] T035 Confirm all checks and threads are green, then request maintainer merge ritual

## Dependencies and Execution Order

- Phase 1 is the blocking spec gate.
- Phase 2 tests must fail for the intended reason before implementation.
- Geometry precedes row controls because it establishes actual allocations.
- Copy and spacing may proceed after the shared row seam exists.
- Documentation and all local validation precede commit and publication.
- The second review request occurs only after first-round CI and feedback settle.
