---

description: "Task list for slice 013: GUI ergonomics, information design, and auto-save"
---

# Tasks: GUI Ergonomics, Information Design, and Auto-Save

**Input**: Design documents from `specs/013-gui-ergonomics-autosave/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, quickstart.md

**Tests**: Included. The constitution requires test-first discipline and never
weakening the safety-critical surfaces, so correctness logic ships with unit tests.
The egui render layer stays thin and is validated by `quickstart.md`, not unit tests.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1..US5)
- Exact file paths are included in each task

## Path Conventions

Single crate: `src/` and `tests/` at repository root.

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Scaffolding all stories build on

- [ ] T001 [P] Set an explicit initial and minimum inner window size in `src/main.rs` via `ViewportBuilder::with_inner_size` and `with_min_inner_size`
- [ ] T002 Create the presentation-helper module `src/app/widgets.rs` and declare it in `src/app/mod.rs`
- [ ] T003 [P] Create the centralized UI strings module `src/app/strings.rs` (labels, tooltips, help text) and declare it in `src/app/mod.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared widgets, typography, and string coverage used across stories

**CRITICAL**: No user story work begins until this phase is complete

- [ ] T004 [P] Wire the bundled `Inter-Medium` and `Inter-SemiBold` weights into `theme::install_fonts` and add a heading text style in `src/app/theme.rs`
- [ ] T005 [P] Add a `StatusRole` to palette-color mapping helper in `src/app/theme.rs` (Healthy, Warning, Active, Muted, Error)
- [ ] T006 Implement the reusable colorized `toggle_switch` widget in `src/app/widgets.rs` (animated knob, gold/teal on, muted off, pointer cursor, hover text)
- [ ] T007 [P] Implement `heading` and `muted_help` (small inline help) render helpers in `src/app/widgets.rs`
- [ ] T008 Populate `src/app/strings.rs` with every user-facing label, tooltip, and help string (no underscores), grouped by control and section
- [ ] T009 [P] Add `tests/app_strings.rs`: assert no user-facing label contains an underscore and every registered control, section title, and Skills column has a non-empty tooltip

**Checkpoint**: Shared widgets, headings, palette-role colors, and strings ready

---

## Phase 3: User Story 1 - Controls that read as what they are (Priority: P1) MVP

**Goal**: Toggles, headings, labeled Skills columns, inherited-default display, and a
renamed, colorized, full-width status region.

**Independent Test**: Launch and confirm toggles, headings, labeled columns,
inherited-default (not zero), and the renamed colorized status region, with no
dependence on persistence, settings, or the log work.

### Tests for User Story 1

- [ ] T010 [P] [US1] Add `tests/app_view_model.rs` cases for `StatusLine` derivations: title, normalized state text, and `StatusRole` for running/suspended, each `FishingState`, and each `BeaconCondition`
- [ ] T011 [P] [US1] Add `tests/app_view_model.rs` cases for the skill row `effective_delay` and `is_override`: override off yields the global default for the row's weave type; override on yields the edited value and targets the delay matching the weave type (light/heavy/bash)

### Implementation for User Story 1

- [ ] T012 [US1] Add the `StatusRole` enum and `StatusLine` derivations (title, state text, role, tooltip) in `src/app/mod.rs`, sourced from the existing suspend flag, `FishingState`, and `BeaconCondition`
- [ ] T013 [US1] Extend the skill row view in `src/app/mod.rs` with `effective_delay` and `is_override`, and make `SkillEdit` for override target the delay matching the row's weave type
- [ ] T014 [US1] Render the top region in `src/app/ui.rs` as a full-width grid: titles Status, Fishing, Pixel Beacon (Addon) first, then a color-coded state field spanning the Skills width
- [ ] T015 [US1] Replace the suspend/resume and fishing action buttons in `src/app/ui.rs` with `toggle_switch` controls
- [ ] T016 [US1] Add a Skills header row (Skill, Enabled, Weave, Override, Delay) in `src/app/ui.rs`; replace the active and override checkboxes with `toggle_switch`; render the inherited default muted/read-only when override is off
- [ ] T017 [US1] Convert section labels (Status, Skills) to `heading` and present File/View via `egui::menu::bar` with pointer-hand hover cursors in `src/app/ui.rs`

**Checkpoint**: The main window reads as considered software, independent of the rest

---

## Phase 4: User Story 2 - Nothing is ever lost (Priority: P1)

**Goal**: Auto-save everywhere (no Apply), coalesced writes, session state in a
separate file restored on launch, and a save toast.

**Independent Test**: Change values on the main window and settings, relaunch, and
confirm restore; confirm no Save/Apply control and a single coalesced toast per
settle; confirm session state is in `state.json`, not `config.json`.

### Tests for User Story 2

- [ ] T018 [P] [US2] Add `tests/app_session_state.rs`: `SessionState` JSON round-trip; safe-default fallback on missing/unreadable/invalid; fishing persists as an on/off intent only (no sub-states)
- [ ] T019 [P] [US2] Add `tests/app_settings.rs` cases for the `SaveScheduler::should_flush` predicate: dirty plus settled elapsed flushes once; repeated changes within the window coalesce to a single flush
- [ ] T020 [US2] Add a safety test asserting that restoring a non-suspended or fishing-on session state performs no input synthesis or suppression while the game window is unfocused (focus-scoped invariant)

### Implementation for User Story 2

- [ ] T021 [P] [US2] Implement `src/config/state.rs`: `SessionState { schema_version, suspended, fishing }`, load/store to a separate `state.json` in the platform config dir, with tolerant defaults and notices
- [ ] T022 [US2] Add the `SaveScheduler` (dirty flag, last-change instant, settle duration, pure `should_flush`) in `src/app/mod.rs`
- [ ] T023 [US2] Route every value-changing intent in `src/app/mod.rs` through the scheduler so config and session writes are coalesced at the single `config::save` choke point plus the state store
- [ ] T024 [US2] Restore session state on startup in `src/app/mod.rs` / `src/main.rs`, applying suspend and re-arming fishing as an on/off intent, preserving the focus-scoped invariant
- [ ] T025 [US2] Replace the draft-and-Apply flow in `src/app/settings_form.rs` with live auto-apply that writes through to `Settings` on each change
- [ ] T026 [US2] Remove the Save/Apply/Close save controls from `src/app/ui.rs`; add the bottom-right save toast via a `toast` helper in `src/app/widgets.rs`, coalesced with the scheduler

**Checkpoint**: Every change persists automatically; nothing is lost across restarts

---

## Phase 5: User Story 3 - A focused, organized settings surface (Priority: P2)

**Goal**: Full-frame settings modal over a dimmed backdrop, clustered groups, no
underscores, inline help per option, and the previously hidden beacon settings.

**Independent Test**: Open Settings and confirm the modal, backdrop, and dismissal
paths; clustered groups; no underscores; inline help; beacon settings present.

### Tests for User Story 3

- [ ] T027 [P] [US3] Add `tests/app_settings.rs` cases: the settings cluster map assigns every setting to a cluster; every setting has a non-empty help string; the beacon path-override and environment settings are present in the form

### Implementation for User Story 3

- [ ] T028 [US3] Render Settings as an `egui::Modal` in `src/app/ui.rs` sized to about 90 percent of `ctx.screen_rect()`, with a dimmed backdrop and close on outside click, Escape, and an explicit close control
- [ ] T029 [US3] Reorganize settings into labeled clustered group boxes (Appearance; Combat timing; Fishing; Pixel Beacon and bus; Logging; Keybindings) with headings in `src/app/ui.rs`
- [ ] T030 [US3] Render each option with human-readable labels from `src/app/strings.rs` (no underscores) and a `muted_help` inline help line
- [ ] T031 [US3] Surface the beacon path-override and environment settings in `src/app/settings_form.rs` and the Pixel Beacon cluster in `src/app/ui.rs`

**Checkpoint**: Settings is a clean, organized modal; changes already auto-save

---

## Phase 6: User Story 4 - A resizable, terminal-style live log (Priority: P2)

**Goal**: Darker monospace terminal log in a resizable bottom panel with a persisted,
clamped height.

**Independent Test**: Open the log, confirm darker background and monospace with
per-level colors, drag between min and max, and confirm height restored after a
restart.

### Tests for User Story 4

- [ ] T032 [P] [US4] Add `tests/config.rs` cases: `ui.log_panel_height` round-trips and defaults safely when absent (back-compat)
- [ ] T033 [P] [US4] Add a pure log-height clamp test (min about a tenth of window, max at the interactive-area bottom) in `tests/app_log_view.rs`

### Implementation for User Story 4

- [ ] T034 [US4] Add `log_panel_height` to the `ui` config section in `src/app/settings_form.rs` and `src/config/mod.rs` (additive, serde default)
- [ ] T035 [US4] Add the pure log-height clamp helper in `src/app/mod.rs` (or `log_view.rs`) and terminal-styling derivation (monospace, `extreme_bg_color`) in `src/app/log_view.rs`
- [ ] T036 [US4] Move the live log into a resizable `egui::TopBottomPanel::bottom` in `src/app/ui.rs`, apply the darker background and monospace rows, and persist/restore the clamped height through the scheduler

**Checkpoint**: The log is terminal-styled and resizable, height remembered

---

## Phase 7: User Story 5 - Guidance everywhere (Priority: P3)

**Goal**: Hover tooltips on every control, section title, and Skills column header,
consistent across the app.

**Independent Test**: Hover controls, titles, and column headers across the main
window and settings and confirm concise, consistent tooltips.

### Tests for User Story 5

- [ ] T037 [P] [US5] Extend `tests/app_strings.rs` to assert tooltip coverage for every interactive control, section title, and Skills column header registered in `src/app/strings.rs`

### Implementation for User Story 5

- [ ] T038 [US5] Attach `on_hover_text` from `src/app/strings.rs` to every interactive control, section title, and Skills column header across `src/app/ui.rs`

**Checkpoint**: Every control and label is explained on hover

---

## Phase 8: Polish and Cross-Cutting Concerns

- [ ] T039 Update `CHANGELOG.md` `[Unreleased]` with an Added line for slice 013 and a dated Decisions entry for the separate session-state file
- [ ] T040 Run `src/../specs/013-gui-ergonomics-autosave/quickstart.md` manual validation and record results
- [ ] T041 Run the CI-parity gate in the foreground: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked`

---

## Dependencies and Execution Order

### Phase Dependencies

- Setup (Phase 1): no dependencies
- Foundational (Phase 2): depends on Setup; blocks all user stories
- User Stories (Phase 3+): depend on Foundational
  - US1 is the MVP and is fully independent
  - US2 adds persistence; US1 edits persist once US2 lands but US1 is testable without it
  - US3 depends on US2 (auto-save) to drop the Apply button; independently testable for modal/layout
  - US4 depends on US2 (scheduler) to persist height; independently testable for styling/resize
  - US5 layers tooltips on the controls from US1/US3/US4
- Polish (Phase 8): after the desired stories

### Within Each User Story

- Tests are written first and must fail before implementation
- View-model derivations before their egui rendering
- The safety test (T020) must pass and is never weakened

### Parallel Opportunities

- T001, T003 in Setup; T004, T005, T007, T009 in Foundational
- Test tasks marked [P] within a story run in parallel before that story's implementation
- US3, US4, and US5 can proceed in parallel after US2, subject to shared-file coordination in `ui.rs`

---

## Implementation Strategy

### MVP First (User Story 1)

1. Phase 1 Setup, Phase 2 Foundational
2. Phase 3 User Story 1
3. Validate the main window presentation independently

### Incremental Delivery

1. Setup + Foundational
2. US1 (presentation MVP) -> validate
3. US2 (auto-save + session) -> validate
4. US3 (settings modal), US4 (resizable log), US5 (tooltips) -> validate each
5. Polish, CHANGELOG, CI parity

## Notes

- [P] tasks touch different files with no incomplete dependencies
- The egui layer stays thin; correctness logic lives in the tested view-model and
  subsystem modules
- Commit after each logical group; run the CI-parity gate before every commit that
  changes Rust sources
- Halt once before push with the decision-log breakdown
