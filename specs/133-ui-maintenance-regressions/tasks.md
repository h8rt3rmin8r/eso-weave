# Tasks: S133

## Phase 1 - Specification

- [x] T001 Create spec, clarification decisions, requirements checklist and plan under specs/133-ui-maintenance-regressions; map #306-#310 in Plan059.
- [x] T002 Research source and installed diagnostics read-only; define data model, contracts and quickstart.
- [x] T003 Execute speckit-analyze on spec.md, plan.md and tasks.md; resolve gaps before implementation.

## Phase 2 - US1/US2 log geometry and visibility

- [x] T004 [US1] Add failing real-frame command/overlap regression coverage in tests/app_ui_sizing.rs (FR-001/002).
- [x] T005 [US1] Repair measured bounded growth and small-display accessibility in src/app/ui.rs and existing geometry seams (FR-001/002).
- [x] T006 [US2] Add restart/immediate-close/legacy/stale-form regressions in tests/app_view_model.rs and tests/app_settings.rs (FR-003).
- [x] T007 [US2] Persist visibility and initialize GUI/model consistently in src/app/settings_form.rs, src/app/mod.rs and src/app/ui.rs (FR-003).

## Phase 3 - US3 uninstall

- [x] T008 [US3] Replace obsolete transient-row regression with failing modal layout/dismissal tests in tests/app_ui_sizing.rs; retain data fixture coverage (FR-004/005).
- [x] T009 [US3] Implement titled confirmation modal in src/app/ui.rs (FR-004/005).

## Phase 4 - US4 maintenance

- [x] T010 [US4] Add absent-root and typed repeated-failure/recovery regressions in tests/s129_package.rs and src/data_maintenance/runtime.rs (FR-005/007/008).
- [x] T011 [US4] Repair automatic migration no-op and bounded diagnostic episode tracking in src/beacon/package.rs and src/data_maintenance/runtime.rs (FR-005/007/008).

## Phase 5 - US5 compatibility

- [x] T012 [US5] Add supported/unsupported/unknown/absent/manual installed-manifest status fixtures in tests/app_view_model.rs and UI width coverage (FR-006).
- [x] T013 [US5] Implement concise installed-evidence presentation in src/app/mod.rs, src/beacon/package.rs and src/app/strings.rs (FR-006).

## Phase 6 - Integration and handoff

- [x] T014 Update canonical interface/config/logging/status docs and annotate superseded historical sizing contracts, refresh governed deterministic captures and receipts; add CHANGELOG.md entry (FR-009).
- [x] T015 Run full formatting, Clippy, locked tests and applicable documentation/policy gates; record evidence in verification.md (FR-009).
- [x] T016 Prepare codex/133-ui-maintenance-regressions for authorized publication with separate Closes #306 through Closes #310 references, verification receipt and stop-before-merge review handoff. Hosted CI/review state is recorded in the PR.

## Dependencies and Execution Order

T003 blocks implementation. Each story's test precedes its repair; shared app files change sequentially. T014-T016 follow all five stories. Every requirement and success criterion is covered by its story and final gates.
