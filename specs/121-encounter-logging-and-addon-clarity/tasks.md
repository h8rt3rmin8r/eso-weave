# Tasks: Encounter Logging and Addon Clarity

## Phase 1: Setup

- [x] T001 Complete issue research and spec-kit specify/clarify/checklist/plan artifacts in specs/121-encounter-logging-and-addon-clarity/ (FR-001-013).
- [x] T002 Record S121 regrouping and preserve historical reservations in docs/project/build-plans/plan-048.md (FR-013).

## Phase 2: Foundation

- [x] T003 Capture complete old message inventory and state/surface matrix in specs/121-encounter-logging-and-addon-clarity/audit.md and inventory artifacts (FR-001-002).
- [x] T004 Run blocking cross-artifact analyze and record coverage in specs/121-encounter-logging-and-addon-clarity/verification.md before implementation (FR-012-013).

## Phase 3: US1 Setup and addon state

- [x] T005 [US1] Write failing plain setup/current-versus-saved/unknown/foreign/reload tests in tests/app_strings.rs and tests/app_view_model.rs (FR-003-005,012).
- [x] T006 [US1] Repair labels, tooltips, state/recovery messages and addon details/removal in src/app/strings.rs, mod.rs and ui.rs (FR-003-005,008).
- [x] T007 [US1] Explain logging settings, selected environment and catalog collection workflow/worker messages in src/app/ui.rs and catalog-update sources (FR-004-005,009).

## Phase 4: US2 Record, save, import, clear

- [x] T008 [US2] Write failing command/help/status/save/clear/unknown-version message tests in tests/encounter_addon.rs and collector_addon.rs (FR-006,008,012).
- [x] T009 [P] [US2] Repair all addon public messages in addon/EsoWeaveData/Encounter.lua and Catalog.lua; recompute normalized self-checksum (FR-003-004,006,008).
- [x] T010 [US2] Write failing history guidance/result/diagnostic/delete tests in tests/app_encounter_history.rs, retaining real no-command-ingress tests (FR-006-008,012).
- [x] T011 [US2] Repair all history/import/metric/recommendation messages in src/app/ui.rs, app/encounter_history.rs and encounter/history.rs (FR-003-004,006-008).
- [x] T012 [P] [US2] Align complete workflow and every relevant public/bundled guide under docs/src/ with final vocabulary and deletion/recovery contracts (FR-004-009).

## Phase 5: US3 Lineage

- [x] T013 [US3] Write failing lineage placement/stages/manifest/topology/offline coverage in .github/scripts/docs-policy.test.mjs and docs-render-smoke.test.mjs (FR-010-012).
- [x] T014 [US3] Add docs/src/assets/diagrams/encounter-evidence-lineage.svg and full nearby equivalent in docs/src/reference/encounter-data-and-metrics.md (FR-010).
- [x] T015 [US3] Expand docs/project/content-coverage.json and docs policy/smoke inventory; record source authorities/update triggers in docs/project/diagram-rendering-compatibility.md (FR-011).

## Phase 6: Integration and publication

- [x] T016 Reconcile all old/final message dispositions, states and help paths in specs/121-encounter-logging-and-addon-clarity/audit.md (FR-001-009, SC-001-002).
- [x] T017 Run focused/full Rust and documentation/headless rendering checks, including narrow/200-percent zoom and text hygiene; record exact evidence in specs/121-encounter-logging-and-addon-clarity/verification.md (FR-010-013, SC-003-004).
- [x] T018 Update CHANGELOG.md, finalize spec/task status, commit with co-author attribution and publish authorized branch/official PR (FR-013).
- [x] T019 Respond to every review, verify corrections, resolve findings, at most two Codex rounds, require green hosted checks and hand off for owner final review (FR-013, SC-005).

## Dependencies and Parallel Execution

T001-004 precede all code. US1 and desktop US2 share files and execute sequentially. Addon T008-009 is disjoint from desktop and may run in parallel; docs T012-015 is also disjoint with shared vocabulary contract. T016 integrates all results before T017-019. Tests precede corresponding source edits. Each story remains independently fixture-testable. Full slice completes all stories; no scope is dropped for an MVP.
