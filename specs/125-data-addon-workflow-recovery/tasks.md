# Tasks: Data Addon Workflow Recovery

**Input**: [spec.md](spec.md), [plan.md](plan.md), research.md, data-model.md, contracts/workflow.md, quickstart.md.

## Phase 1: Setup and analysis

- [x] T001 Create matching S125 branch/spec artifacts and requirement checklists (FR-011).
- [x] T002 Resolve clarification, research, contracts and constitution checks in specs/125-data-addon-workflow-recovery/ (FR-010, FR-011).
- [x] T003 Run blocking Spec Kit analysis and record requirement/task coverage before implementation (FR-011).

## Phase 2: US1 readability foundation

- [x] T004 [US1] Add failing actual contrast and instruction-completeness fixtures in tests/app_ui_sizing.rs, including a wrong-role negative control and raw-emphasis guard (FR-001, FR-002).
- [x] T005 [US1] Replace every scoped surface emphasis in src/app/ui.rs and audit justified exceptions (FR-001).
- [x] T006 [US1] Audit/shorten lifecycle copy in src/app/mod.rs; adaptive shared instruction rows and scrolling details in src/app/ui.rs (FR-002).
- [x] T007 [US1] Validate empty/selected/busy and lifecycle/reload variants across both themes/scales/layouts in tests/app_ui_sizing.rs (FR-001, FR-002).

## Phase 3: US2 visible deliberate recording

- [x] T008 [US2] Add failing blocked-debug/delayed-chat/command state fixtures through registered production commands in tests/encounter_addon.rs and tests/collector_addon.rs (FR-003, FR-004).
- [x] T009 [US2] Implement shared readiness-aware system replies in existing addon/EsoWeaveData/EsoWeaveData.lua and migrate both modules (FR-003, FR-010).
- [x] T010 [US2] Implement validated idempotent start, concise status/help/errors and retained/failed recovery in addon/EsoWeaveData/Encounter.lua (FR-004).
- [x] T011 [US2] Align addon/EsoWeaveData/Catalog.lua help/completion with maintainer desktop controls and run lifecycle/alias/isolation fixtures (FR-003, FR-004, FR-008, FR-010).

## Phase 4: US3 saved-source inspection and recovery

- [x] T012 [US3] Add failing source/diagnostic/read-only/worker fixtures in tests/encounter_history.rs and tests/app_encounter_history.rs (FR-005, FR-006, FR-010).
- [x] T013 [US3] Add controlled source error categories and bounded inspection in src/encounter/mod.rs, validate.rs and history.rs; separate source and store import diagnostics (FR-005, FR-006).
- [x] T014 [US3] Extend src/app/encounter_history.rs worker requests/events with source identity/generation and state-specific empty results (FR-005, FR-006).
- [x] T015 [US3] Wire source inspection/refresh and stale-response rejection into src/app/ui.rs; display last-saved metadata and recovery (FR-005, FR-006).
- [x] T016 [US3] Validate initialized/active/stopped/interrupted/failed/legacy/invalid/denied/missing/mismatched/store/duplicate cases and preserved history/source bytes (FR-005, FR-006, FR-010).

## Phase 5: US4 complete workflow and catalog intent

- [x] T017 [US4] Add failing workflow reachability/copy actions and candidate acknowledgment state tests in tests/app_ui_sizing.rs (FR-007, FR-009).
- [x] T018 [US4] Implement direct Data Details action, ordered copyable commands/results, first-use/recovery disclosures in src/app/ui.rs (FR-007).
- [x] T019 [US4] Separate maintainer catalog collection and eligible Live acknowledgment/install context in src/app/ui.rs (FR-008, FR-009).
- [x] T020 [US4] Preserve identity/uncheck/busy and keyboard service behavior and run full workflow interaction/paint tests (FR-007, FR-009, FR-010).

## Phase 6: Documentation and delivery

- [x] T021 Update docs/src user/development guidance and create specs/125-data-addon-workflow-recovery/audit.md accounting for all scoped emphasis/copy/state surfaces (FR-001, FR-002, FR-007, FR-008).
- [x] T022 Archive completed Plan050, create chronological docs/project/build-plans/plan-051.md and update indexes/ledger and CHANGELOG.md (FR-011).
- [x] T023 Run full foreground Rust/build/docs/repository gates and document red/green evidence in specs/125-data-addon-workflow-recovery/verification.md (FR-010, FR-011).
- [x] T024 Commit, automatically push, publish/attach one official PR closing #265-#271, satisfy external reviews and final-head CI within two Codex rounds (FR-011).
- [x] T025 Verify zero unresolved findings and green final-head checks; hand off for owner merge without merging or release publication (FR-011).

## Dependencies and implementation strategy

T001-T003 block implementation. US1 establishes the text/layout foundation; US2 establishes final command/reply semantics; US3 supplies actual saved-state/recovery; US4 assembles them. Test tasks precede corresponding implementation, and edits to shared files are sequential. Independent read-only gates may run concurrently; no parallel edits to src/app/ui.rs. Deliver all seven acceptance outcomes in one reviewed PR rather than an incomplete MVP.
