# Tasks: Responsive Live HUD Dashboard

**Input**: S046 design documents in `specs/046-live-hud-dashboard/`
**Tests**: Required by the constitution and both issue contracts

## Phase 1: Setup and red tests

- [x] T001 Confirm clean S046 branch and register slice in Project metadata
- [x] T002 Add model tests for beacon signal and resource presentation states
- [x] T003 Add pure breakpoint tests at 879.9 and 880.0 points
- [x] T004 Add rendered-frame tests for stacked and two-column geometry
- [x] T005 Add rendered-frame meter geometry and state-text coverage

## Phase 2: User Story 1, Live HUD

- [x] T006 [US1] Add `Live HUD` strings and compact status-row helper
- [x] T007 [US1] Move context, combat, movement, weapon, and quickslot rows into Live HUD
- [x] T008 [US1] Preserve dormant and unavailable projections in the moved fields

## Phase 3: User Story 2, System and automation

- [x] T009 [US2] Rename application status projection to `ESO Weave: Active/Suspended`
- [x] T010 [US2] Add independent PixelBeacon signal projection
- [x] T011 [US2] Render game, app, beacon, fishing, and auto-potion operational rows
- [x] T012 [US2] Make Install or Update contextually primary and retain safe Uninstall

## Phase 4: User Story 3, resource meters

- [x] T013 [US3] Add typed observed, low, dormant, and unavailable resource presentation
- [x] T014 [US3] Derive Low from the controller's enabled resource watches
- [x] T015 [US3] Add semantic resource palette tokens with contrast tests
- [x] T016 [US3] Implement one accessible, unanimated ResourceMeter widget
- [x] T017 [US3] Render all three resources through the shared component

## Phase 5: User Story 4, responsive integration

- [x] T018 [US4] Add pure narrow/wide breakpoint projection
- [x] T019 [US4] Arrange sections in stacked and two-column modes
- [x] T020 [US4] Record section and meter geometry for rendered-frame assertions
- [x] T021 [US4] Replace the obsolete fixed-height sizing assertion with responsive contracts
- [x] T022 [US4] Re-run log, modal, minimum, and Skills geometry regressions

## Phase 6: Documentation and validation

- [x] T023 Update master specification, README, changelog, and plan-016
- [x] T024 Run focused view-model and rendered-frame suites
- [x] T025 Run fmt, clippy, and locked full tests
- [x] T026 Run forbidden-dash, mojibake, diff, and issue-link audits
- [x] T027 Complete dashboard safety checklist and implementation analysis
- [ ] T028 Commit, push, publish a PR with separate closing references for #28 and #29
- [ ] T029 Resolve CI and every review comment, with at most one requested second round
