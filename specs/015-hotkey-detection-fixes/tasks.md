---

description: "Task list for feature 015: hotkey and weapon-bar detection fixes"
---

# Tasks: Hotkey and Weapon-Bar Detection Fixes

**Input**: Design documents from `/specs/015-hotkey-detection-fixes/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, quickstart.md

**Tests**: REQUIRED. The constitution mandates test-first with explicit seams
(Principle III); the pure routing functions are written test-first.

**Organization**: Tasks are grouped by user story. US1 (F1 suspend) and US2 (F2
fishing) share one toggle-routing mechanism, placed in Foundational; each story
owns its intent-map arm, its test, and its in-game validation. US3 (weapon-bar
detection) is independent.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- Paths are at the repository root single crate.

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Confirm a clean baseline before changes.

- [X] T001 Confirm a clean tree and green baseline: run `cargo test --all --locked` once so any later failure is attributable to this slice.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: The shared toggle-routing mechanism that both US1 and US2 depend on:
partition the action stream, carry toggles to the GUI, and drain them each frame.
No story-specific behavior yet (the intent map returns `None` until US1/US2 add
their arms).

- [X] T002 [P] Write a failing unit test for `Action::is_app_toggle` in `src/input/action.rs` (`#[cfg(test)]`): true for `ToggleSuspend` and `ToggleFishing`, false for every skill/ultimate/synergy action (iterate `Action::ALL`).
- [X] T003 Implement `Action::is_app_toggle(self) -> bool` in `src/input/action.rs` to pass T002.
- [X] T004 Add `AppModel::fishing_on(&self) -> bool` in `src/app/mod.rs` (live fishing on/off, reusing the same check as `current_session_state`) so the GUI can negate a fishing toggle. Include a brief doc comment.
- [X] T005 Add the pure router `app_toggle_intent(action: Action, fishing_on: bool) -> Option<UiIntent>` in `src/app/routing.rs` returning `None` for all actions initially (arms added in US1/US2); export it from the app module as needed.
- [X] T006 In `src/main.rs`, create the app-toggle channel (`std::sync::mpsc::channel`), and in the weave worker forward each drained action where `action.is_app_toggle()` to the toggle sender instead of `WeaveEngine::handle`; keep passing non-toggle actions to `handle`.
- [X] T007 In `src/main.rs`, pass the toggle receiver into `EsoWeaveApp`/`AppModel` construction (thread the `Receiver<Action>` to the GUI; store it on `EsoWeaveApp`).
- [X] T008 In `src/app/ui.rs` `update()`, drain the toggle receiver with `try_recv` each frame and, for each action, apply `app_toggle_intent(action, self.model.fishing_on())` via `self.model.apply_intent(...)` when it yields `Some`. Place the drain alongside the existing intent application so the coalesced auto-save flush still runs.

**Checkpoint**: The mechanism compiles and runs; toggles are carried to the GUI
but still no-op (intent map returns `None`). Existing tests stay green.

---

## Phase 3: User Story 1 - F1 suspends and resumes (Priority: P1)

**Goal**: A focused F1 toggles the same suspend state the Status button toggles,
reflected in the GUI and persisted identically.

**Independent Test**: `app_toggle_intent(ToggleSuspend, _)` maps to
`UiIntent::ToggleSuspend`; applying it flips `InputEngine.suspended` and the
derived Status label.

- [X] T009 [P] [US1] Write a failing unit test in `src/app/routing.rs` asserting `app_toggle_intent(Action::ToggleSuspend, false)` and `(.., true)` both return `Some(UiIntent::ToggleSuspend)`.
- [X] T010 [US1] Add the `ToggleSuspend => Some(UiIntent::ToggleSuspend)` arm to `app_toggle_intent` in `src/app/routing.rs` to pass T009.
- [X] T011 [US1] Add a unit test that applying `UiIntent::ToggleSuspend` through `AppModel::apply_intent` flips `input.is_suspended()` and that `view().suspended`/`app_state` reflects it (parity with the button path), in `src/app/mod.rs` tests.
- [ ] T012 [US1] In-game validation per quickstart: focused F1 toggles Status both directions, held F1 toggles once per press, unfocused F1 does nothing, and a rebound suspend key toggles while F1 no longer does. Record the result in the pre-push breakdown.

**Checkpoint**: F1 has full button parity for suspend.

---

## Phase 4: User Story 2 - F2 enables and disables fishing (Priority: P1)

**Goal**: A focused F2 toggles the same fishing enabled state the Fishing button
toggles, reflected in the GUI and persisted identically.

**Independent Test**: `app_toggle_intent(ToggleFishing, fishing_on)` maps to
`UiIntent::SetFishing(!fishing_on)` for both current states.

- [X] T013 [P] [US2] Write a failing unit test in `src/app/routing.rs` asserting `app_toggle_intent(Action::ToggleFishing, false)` returns `Some(SetFishing(true))` and `(.., true)` returns `Some(SetFishing(false))`.
- [X] T014 [US2] Add the `ToggleFishing => Some(UiIntent::SetFishing(!fishing_on))` arm to `app_toggle_intent` in `src/app/routing.rs` to pass T013.
- [X] T015 [US2] Add a unit test that applying the mapped `SetFishing` intent through `apply_intent` flips the fishing on/off state and `view().fishing_active`, in `src/app/mod.rs` tests.
- [ ] T016 [US2] In-game validation per quickstart: focused F2 toggles Fishing both directions, held F2 toggles once per press, unfocused F2 does nothing. Record the result in the pre-push breakdown.

**Checkpoint**: F2 has full button parity for fishing.

---

## Phase 5: User Story 3 - Weapon bar is visible and diagnosable (Priority: P2)

**Goal**: When a valid B3 block is decoded, the readout shows the bar and
classes; otherwise "Not detected". Reader diagnostics let the operator find the
live cause in-game without a debugger. No weave-timing change.

**Independent Test**: The decode/observe path already renders a detected view for
a valid B3 and "Not detected" for none (existing tests); the new diagnostics emit
the expected tracing events by level.

- [X] T017 [P] [US3] Decode-unchanged guard (test-first, Principle III). Satisfied by the pre-existing tests in `tests/pixelbus.rs` (`decode_weapon_bar_reads_bar_and_classes`, `decode_weapon_bar_requires_marker_within_tolerance`, `weapon_bar_not_decoded_without_heartbeat`, `weapon_bar_event_only_on_change`), which already pin `observe` decode behavior and predate the diagnostics edit. No duplicate test was added; these guard the T018 change and stayed green.
- [X] T018 [US3] Add reader diagnostics in `src/pixelbus/mod.rs` `observe` without changing decode behavior (T017 stays green): a DEBUG `tracing` event on the weapon-bar detected transition (carrying bar, front, back) and on the weapon-bar-cleared transition (on `SignalLost`); a TRACE event with the raw sampled block bytes each observe (including a present heartbeat with a non-decoding B3). Use `target: "eso_weave::pixelbus"`. Do not emit at INFO+ per sample.
- [ ] T019 [US3] In-game validation per quickstart: raise the log level, swap bars, and confirm the log distinguishes "detected" from "heartbeat present but B3 not decoding" from "no heartbeat"; if B3 is absent, reinstall the addon via the app and re-check. Record findings (including whether a stale addon was the cause) in the pre-push breakdown.

**Checkpoint**: Detection is correctly displayed and diagnosable in-game.

---

## Phase 6: Polish & Cross-Cutting Concerns

- [X] T020 Update `CHANGELOG.md` `[Unreleased]`: an Added/Fixed line for the F1/F2 hotkey wiring and the weapon-bar diagnostics, plus dated Decisions entries for D1 (toggle routing to the intent path) and D4 (reader diagnostics levels).
- [X] T021 CI parity in the foreground, watched to completion: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked`.
- [X] T022 Prepare the v0.4.2 patch release: ensure the `CHANGELOG.md` `[Unreleased]` section is complete (Fixed/Added/Decisions). Do NOT hand-bump `Cargo.toml` or the README badge: `cargo release 0.4.2 --execute` is the single source that bumps `Cargo.toml`, rolls `[Unreleased]` into `## [0.4.2]`, and updates the badge via `release.toml`. The tag and release run only under separate explicit authorization after the pre-push halt.

---

## Dependencies & Execution Order

- Phase 1 (T001) before all.
- Phase 2 (T002-T008) is foundational and blocks US1 and US2 (shared mechanism).
  T002/T003 (classifier) and T004/T005 (accessor + router skeleton) can precede
  the wiring (T006-T008). T006-T008 are sequential (same files, `main.rs`/`ui.rs`).
- US1 (T009-T012) and US2 (T013-T016) both depend on Phase 2 and are otherwise
  independent of each other; their intent-map arms touch the same function
  (`app_toggle_intent`) so T010 and T014 are sequential, but their tests
  (T009, T013) are [P].
- US3 (T017-T019) is independent of US1/US2 and can proceed in parallel with them.
- Phase 6 after all implementation; T022 last and non-actioning.

## Parallel Opportunities

- T002 (classifier test) and, after Phase 2 wiring, T009 and T013 (intent-map
  tests) are [P].
- T017 (pixelbus guard test) is [P] with the US1/US2 work (different file); T018 (diagnostics) then depends on T017.

## Implementation Strategy

MVP = US1 + US2 (the two P1 hotkey fixes) on the shared Foundational mechanism;
this alone restores the headline in-game controls. US3 (P2) adds detection
visibility and diagnostics. All three land together in the v0.4.2 patch.
