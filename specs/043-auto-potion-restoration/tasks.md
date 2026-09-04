# Tasks: Auto-potion Restoration

**Input**: Design documents from `/specs/043-auto-potion-restoration/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Safety behavior is test-first. Each implementation phase begins with a focused failing test.

## Phase 1: Setup and Baseline

**Purpose**: Confirm the inherited branch and specification baseline before behavior changes.

- [x] T001 Run the full inherited repository gate and record the result in `specs/043-auto-potion-restoration/tasks.md`
- [x] T002 Verify S041 game/focus facts and S042 quickslot facts are the only source inputs used by `src/potion/mod.rs`

---

## Phase 2: Foundational Effective State

**Purpose**: Establish one typed controller result before activating synthesis.

- [x] T003 [US2] Add failing effective-state and blocker contract tests in `tests/potion.rs`
- [x] T004 [US2] Implement typed effective state, trigger cause, and ordered pure evaluation in `src/potion/mod.rs`
- [x] T005 [US2] Add change-only controller state tracking and diagnostics in `src/potion/mod.rs`

**Checkpoint**: Every normalized input combination has one truthful, testable outcome and no application input is active yet.

---

## Phase 3: User Story 1 - Trigger a verified potion safely (Priority: P1)

**Goal**: Restore production synthesis only for the complete positive conjunction.

**Independent Test**: A fully eligible fixture emits exactly Down then Up once, while each negative fixture emits nothing and retry suppresses repetition.

- [x] T006 [US1] Add failing exact-input, retry, and deterministic-cause tests in `tests/potion.rs`
- [x] T007 [US1] Adapt controller synthesis to the typed outcome and exact-attempt contract in `src/potion/mod.rs`
- [x] T008 [US1] Remove the temporary S042 consumer gate and always evaluate through the existing sink in `src/main.rs`

**Checkpoint**: Auto-potion is reachable in production but remains fail-closed for every missing precondition.

---

## Phase 4: User Story 3 - Recover across lifecycle changes (Priority: P3)

**Goal**: Preserve user intent while lifecycle facts safely block and recover evaluation.

**Independent Test**: Signal loss, game exit, focus loss, gate, and suspension prevent input without clearing the request; positive facts restore eligibility.

- [x] T009 [US3] Add failing request-preservation and heartbeat-recovery tests in `tests/potion.rs` and `tests/app_view_model.rs`
- [x] T010 [US3] Track beacon availability and preserve requested enablement in `src/potion/mod.rs`
- [x] T011 [US3] Route Heartbeat and SignalLost into the controller lifecycle in `src/app/routing.rs`

**Checkpoint**: Temporary lifecycle changes are effective-state changes, not preference mutations.

---

## Phase 5: User Story 2 - Expose truthful status (Priority: P2)

**Goal**: Show the request and current effective state or blocker in the main view.

**Independent Test**: Every controller state maps to a distinct normalized view value and rendered status while the request switch remains independently controlled.

- [x] T012 [US2] Add failing normalized view-state tests in `tests/app_view_model.rs`
- [x] T013 [US2] Extend the normalized application view and strings in `src/app/mod.rs` and `src/app/strings.rs`
- [x] T014 [US2] Render requested enablement beside effective state and remove verification-pending copy in `src/app/ui.rs`
- [x] T015 [US2] Update sizing fixtures only if the truthful status changes the tested layout in `tests/app_ui_sizing.rs`

**Checkpoint**: The main view names every blocker without duplicating controller logic.

---

## Phase 6: Documentation and Validation

**Purpose**: Align durable guidance, prove policy compliance, and prepare review evidence.

- [x] T016 Document S043 behavior and the S039 lifecycle correction in `docs/ESO-Weave-Specification.md`, `CHANGELOG.md`, and any directly affected architecture comments
- [x] T017 Run `specify check`, placeholder checks, punctuation checks, UTF-8/BOM checks, and the full repository gate
- [x] T018 Review the complete diff for scope, safety, issue linkage, and absence of unrelated changes
- [x] T019 Push `codex/043-auto-potion-restoration`, open the official pull request referencing issue #25 without closing it, and wait for CI and third-party review
- [ ] T020 Address every review comment, optionally request at most one authorized second Codex round, and stop for the user's final review and merge ritual once all checks are green

---

## Dependencies and Execution Order

- Phase 1 validates the baseline before any source change.
- Phase 2 is foundational and blocks all production activation.
- Phase 3 activates the existing sink only after Phase 2 tests pass.
- Phase 4 reuses the typed state from Phase 2 and must complete before UI status is final.
- Phase 5 consumes controller state without duplicating evaluation.
- Phase 6 follows all implementation checkpoints.

## Implementation Strategy

1. Prove the inherited baseline.
2. Build the typed fail-closed state contract test-first.
3. Restore exact synthesis and retry protection test-first.
4. Correct lifecycle semantics and routing test-first.
5. Expose the controller-owned result in the UI test-first.
6. Update durable documentation, run the full gate, publish, and complete reviews.

## Notes

- Inherited baseline on 2026-09-03: format, Clippy, all tests, and rustdoc passed. Rustdoc reported six pre-existing link warnings that the final documentation gate must resolve or explicitly reclassify.
- Source audit confirmed the controller reads S041-provided game/focus state and the normalized S042 `QuickslotState`; no parallel process, focus, item, or cooldown detector is required.
- Final local gate on 2026-09-03: format, warning-denied Clippy, all targets and features, and warning-denied private-item rustdoc passed. The six inherited rustdoc link warnings were corrected as documentation-only cleanup.
- First Codex review on PR #39 identified two valid safety defects: unavailable menu evidence released the shared gate, and applied potion settings did not refresh the live controller. Both were reproduced by failing tests and corrected before the authorized second review round.
- Real-client verification remains deferred until a fresh release exists.
- Issue #25 must remain open after this pull request unless its post-release receipt is completed separately.
- No addon protocol, dependency, or persistent enablement change belongs in this slice.
