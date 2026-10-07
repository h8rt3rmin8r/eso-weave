# Tasks: S120 Combat Recovery and API Compatibility

**Input**: spec.md, plan.md, research.md, data-model.md, contracts/recovery.md.

## Phase 1: Specification and Design

- [x] T001 Complete specification, clarification decisions, and requirement/domain checklists in `specs/120-combat-recovery-and-api-compatibility/` (all FR/SC).
- [x] T002 Record source/control research, contracts, model, quickstart and chronological `docs/archive/build-plans/plan-048.md` (FR-001,005,011,014).
- [x] T003 Reconcile stale constitution V wording outside analysis, then run read-only speckit-analyze across spec/plan/tasks before implementation (SC-005).

## Phase 2: User Story 1 - Weaving Recovery

- [x] T004 [US1] Add failing four-slot Lua fixtures in `tests/beacon.rs` for mixed keyboard/controller, genuine desktop conflicts, controller-only and unavailable classifier (FR-001,004).
- [x] T005 [US1] Repair desktop control discovery in `addon/PixelBeacon/PixelBeacon.lua` (FR-001,002,004).
- [x] T006 [US1] Exercise decoded evidence through input admission and `RealSink` synthesis, with latency/bar timings, equivalent snapshots, true binding changes and gate/cleanup regressions in `tests/beacon.rs`, `tests/input_engine.rs`, `tests/weave_engine.rs` (FR-002-004, SC-001).

## Phase 3: User Story 2 - API Compatibility

- [x] T007 [US2] Add failing version-source and presentation fixtures for merge messages, numeric API, freshness/channel failures, offline and repeated starts in `src/beacon/api_check.rs` and `tests/app_view_model.rs` (FR-005-009, SC-002).
- [x] T008 [US2] Implement bounded pinned channel evidence in `src/beacon/api_check.rs` and select Live/PTS in `src/main.rs` (FR-005,006,009,011).
- [x] T009 [US2] Reconcile reviewed manifests/defaults and prevent observed future APIs from silently expanding managed declarations in `src/beacon/mod.rs`, data-addon lifecycle code and addon manifests; cover ownership and neighbors in existing addon tests (FR-010-011).
- [x] T010 [US2] Persist additive evidence in configuration and show actionable supported/incompatible/unknown compatibility in existing `src/app/mod.rs` and `src/app/ui.rs` addon details (FR-006-009, SC-002).

## Phase 4: User Story 3 - Stationary HUD

- [x] T011 [US3] Add failing headless row-absence and captured transition-log fixtures in `tests/app_ui_sizing.rs` and `tests/app_view_model.rs` (FR-012-013, SC-003-004).
- [x] T012 [US3] Delete the transient row in `src/app/ui.rs`; add bounded transition logging in `src/app/mod.rs` while preserving retention/gating (FR-012-013).

## Phase 5: Documentation and Delivery

- [x] T013 Update affected `docs/src/` guidance, `CHANGELOG.md`, research/evidence receipts, and task outcomes with observed versus unobserved status (FR-014, SC-005).
- [x] T014 Run fmt/clippy/all locked tests, documentation and text hygiene checks; commit, authorized push/official PR, handle every review and at most one additional Codex round, then green-CI owner handoff (SC-005). Implementation CI is green; see `evidence.md`. Final receipt commit checks must also be green before the owner handoff.

## Dependencies and Execution

T001 -> T002 -> T003 is the blocking design gate. Each story's failing tests precede its implementation. T006 follows T005; T008-T010 follow T007; T012 follows T011. T013 follows all story implementations and T014 follows documentation. All three stories are required for slice completion. No parallel implementation is scheduled; research agents are read-only and finished.
