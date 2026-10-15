# Tasks: S129 Automatic Catalog Maintenance

Input: spec.md, plan.md, research.md, data-model.md, contracts/automatic-data.md and quickstart.md.

## Phase 1: Setup and analysis

- [x] T001 Create matching branch and complete spec/clarify/checklists in specs/129-automatic-catalog-maintenance/ (FR-015).
- [x] T002 Research package/Lua/Catalog seams and create plan/model/contracts/quickstart in specs/129-automatic-catalog-maintenance/ (FR-001-FR-014).
- [x] T003 Reconcile scoped topology authority in .specify/memory/constitution.md and CHANGELOG.md before blocking analysis (FR-001-FR-005, FR-015).
- [x] T004 Run read-only blocking analysis across spec.md/plan.md/tasks.md and record receipt in analysis.md (FR-015).

## Phase 2: US1 Combined package

- [x] T005 [P] [US1] Write failing package/migration/rollback/link/race/config fixtures in tests/s129_package.rs, tests/beacon.rs and tests/data_addon.rs (FR-001-FR-003).
- [x] T006 [US1] Implement exact staged combined lifecycle/recovery/presence in src/beacon/package.rs, src/beacon/mod.rs and legacy facade src/data_addon.rs (FR-001-FR-003, FR-012).
- [x] T007 [US1] Verify every migration row, byte preservation and beacon failure isolation in package/Lua tests (FR-001-FR-003).

## Phase 3: US2 Automatic Lua collection

- [x] T008 [P] [US2] Write failing initialization/combat/loading/refresh/last-good/budget/optout fixtures in tests/collector_addon.rs (FR-004-FR-005).
- [x] T009 [US2] Implement isolated bootstrap/startup config and bounded automatic collector/finalization in addon/PixelBeacon, retaining historical fixtures (FR-003-FR-005, FR-013).
- [x] T010 [US2] Verify actual metadata, native-logging independence, useful source/category paths, no routine chat/commands and complete-generation handoff in Lua harness tests (FR-004-FR-007).

## Phase 4: US3 Accumulation and activation

- [x] T011 [P] [US3] Add red packaged-baseline/independent-snapshot/union/locale/parent/regression/no-op fixtures in tests/catalog_automatic.rs, tests/catalog_update.rs and tests/collector_import.rs (FR-006-FR-007).
- [x] T012 [US3] Implement verified export/deterministic merge/semantic coverage and parent preservation in src/catalog, src/collector/import.rs and src/catalog_pipeline/mod.rs (FR-006-FR-007).
- [x] T013 [US3] Add automatic serialized complete-generation build/verify/open/activate/no-op/recovery and bounded retention in src/catalog_update/mod.rs (FR-008-FR-010).
- [x] T014 [US3] Test cancellation/atomic/receipt/recovery/version/retention failures in tests/catalog_update.rs and map actual categories/sources in specs/129-automatic-catalog-maintenance/coverage.md (FR-006-FR-010).

## Phase 5: US4 Preferences and runtime integration

- [x] T015 [P] [US4] Write red settings/eligibility/unknown/manual-package/disable-generation/persistence-failure fixtures in tests/data_maintenance.rs and tests/config.rs (FR-011-FR-013).
- [x] T016 [US4] Add typed defaults/migration/export to src/config/mod.rs and shared pure eligibility/cancellation model in src/data_maintenance.rs (FR-011-FR-013).
- [x] T017 [US4] Implement bounded automatic root discovery/reconciliation/retry/startup-config and activation events in src/data_maintenance.rs, main.rs and app integration (FR-008-FR-013).
- [x] T018 [US4] Add two direct accessible persisted controls, inactive absent state, coherent activation consumers and unified lifecycle UI in src/app/mod.rs, settings_form.rs and ui.rs (FR-011-FR-014).
- [x] T019 [US4] Exercise pre-existing/later/partial data, restart, missed notifications, disable/addon-removal/shutdown races, actual-root isolation and UI accessibility in tests/data_maintenance.rs and app harness suites (FR-008-FR-014).

## Phase 6: Documentation and delivery

- [x] T020 Update directly affected docs/src guidance, CLAUDE.md, build-autopilot, policy fixtures and create Plan055 with synchronized indexes/ledger/CHANGELOG.md (FR-014-FR-015).
- [x] T021 Run all foreground Rust/release-profile/docs/render/repository gates and text encoding/mojibake checks; record actual red/green and acceptance audit in verification.md (FR-001-FR-015).
- [ ] T022 Commit/push and publish/attach official PR closing #285-#288/#291; complete at most two Codex review rounds, fix/respond/resolve every finding and verify exact final-head CI (FR-015).
- [ ] T023 Deliver owner final review/merge handoff with no merge/tag/release and explicit evidence limitations in verification.md (FR-015).

## Dependencies and implementation strategy

T001-T004 block implementation. Package, Lua and Catalog domain tests/implementation may proceed in parallel after contracts are fixed because files are disjoint; cross-domain integration waits for their actual interfaces. No parallel edits to shared UI/config/mod.rs. Test tasks precede corresponding code; preserve red evidence. Deliver all five issue outcomes in one PR, with no partial MVP or invented closure. Native watcher, complete viewer retirement and final whole-program asset sweep remain subsequent scoped work.
