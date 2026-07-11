# Tasks: Graphical User Interface

**Feature**: `specs/009-gui` | **Branch**: `009-gui`

Test-first per constitution Principle III, applied to a GUI by splitting a
testable view-model from the egui rendering. Every correctness-bearing piece
(derivations, intents, settings mapping, routing, log view) has a test authored
before its implementation, exercised against the project's in-memory subsystem
doubles. The egui rendering (`app::ui`) and `main.rs` windowing/threading are not
unit-tested; they are validated with the `quickstart.md` manual checklist. Paths
are repository-relative.

## Phase 1: Setup

- [x] T001 Add `eframe = { version = "0.35", default-features = false, features = ["glow", "default_fonts", "x11", "wayland"] }` to `Cargo.toml`. Declare `pub mod app;` in `src/lib.rs` and create compiling stub files `src/app/mod.rs`, `src/app/beacon_light.rs`, `src/app/routing.rs`, `src/app/log_view.rs`, `src/app/settings_form.rs`, and `src/app/ui.rs`, warning-free.

## Phase 2: Foundational (config additions)

- [x] T002 Add a `Theme` enum (`Dark` default, `Light`) to `src/config/mod.rs`, and the additive opaque `pixelbus` and `ui` sections to `Settings` (field, `RawSettings`, default, known-keys), mirroring the existing additive sections.
- [x] T003 [P] Write failing tests in `tests/app_settings.rs`: `PixelBusPrefs` and `UiPrefs` round-trip through their sections and default on absence; an invalid tolerance/interval or theme falls back with a notice.
- [x] T004 Implement `PixelBusPrefs` (map to `ReaderConfig`) in `src/pixelbus/mod.rs` and `UiPrefs`/`Theme` serde in `src/config/mod.rs` with fallback-with-notice loaders. Run the config-section tests to green.

## Phase 3: User Story 2 - Beacon light and status derivation (P1)

- [x] T005 [P] [US2] Write failing tests in `tests/app_view_model.rs`: `beacon_light` returns the defined color and tooltip for each `BeaconCondition`; `uninstall_enabled` is true only for the two installed conditions; `app_state_label` and `fishing_label` return the defined indicator/button pairs.
- [x] T006 [US2] Implement `BeaconCondition`, `beacon_light`, `uninstall_enabled` in `src/app/beacon_light.rs`, and `app_state_label`, `fishing_label` in `src/app/mod.rs` (FR-003 to FR-006). Run these tests to green.

## Phase 4: User Story 6 - Reader-event routing (P2, safety-relevant)

- [x] T007 [P] [US6] Write failing tests in `tests/app_view_model.rs`: `route_reader_event` sets `WeaveEngine` latency on `Latency`, clears it and disables the `FishingController` on `SignalLost`, forwards `FishingStarted`/`BiteDetected`/`FishingStopped` to the controller, and treats `Heartbeat` as a no-op, using a real `WeaveEngine`, `FishingController`, and `MockFishingSink`.
- [x] T008 [US6] Implement `route_reader_event` in `src/app/routing.rs` reusing `fishing::map_event` (FR-014). Run the routing tests to green.

## Phase 5: User Story 3 - Skill rows (P1)

- [x] T009 [P] [US3] Write failing tests in `tests/app_view_model.rs`: `skill_rows` produces one row per slot with the correct label (including "Ultimate (R)" and "Synergy (X)"), active, weave type, and overrides; applying a `SkillEdit` (active, weave type, set/clear override) updates the corresponding `WeaveConfig` slot.
- [x] T010 [US3] Implement `SkillRow`, `skill_rows`, `SkillEdit`, and the edit-apply mapping in `src/app/mod.rs` (FR-007, FR-008). Run the skills tests to green.

## Phase 6: User Story 4 - Live log view (P2)

- [x] T011 [P] [US4] Write failing tests in `tests/app_log_view.rs`: `level_color` returns the defined color per level; `build_log_view` filters events at or above the minimum level and formats rows in order; `autoscroll` reflects the at-bottom flag.
- [x] T012 [US4] Implement `LogRow`, `LogColor`, `level_color`, `build_log_view`, and `autoscroll` in `src/app/log_view.rs` (FR-009 to FR-011). Run the log-view tests to green.

## Phase 7: User Story 5 - Settings form (P2)

- [x] T013 [P] [US5] Write failing tests in `tests/app_settings.rs`: `SettingsForm::load(&settings).apply(&mut s2)` reproduces every section-10.3 category (keybindings, timing, per-slot overrides, latency, fishing, pixel bus, beacon, logging, theme, always-on-top); an invalid field falls back with a notice.
- [x] T014 [US5] Implement `SettingsForm` load/apply in `src/app/settings_form.rs`, reusing each subsystem's existing load/store plus the new `pixelbus`/`ui` mappings (FR-012, FR-013). Run the settings tests to green.

## Phase 8: User Story 1 - AppModel and intents (P1)

- [x] T015 [P] [US1] Write failing tests in `tests/app_view_model.rs`: constructing an `AppModel` over in-memory subsystems, `apply_intent(ToggleSuspend)` flips `InputEngine` suspend; `SetFishing(true)` enables the fishing controller; `InstallBeacon`/`UninstallBeacon` against a tempdir AddOns root install and marker-gated-remove; `EditSkill` updates the weave config; `ApplySettings` saves; the derived `view()` reflects the resulting state.
- [x] T016 [US1] Implement `AppModel` (shared handles), `UiIntent`, `apply_intent`, and `view()` in `src/app/mod.rs` (FR-001, FR-003 to FR-008, FR-012 to FR-014). Run the AppModel tests to green.

## Phase 9: Rendering and entry point (manual-validated)

- [x] T017 Implement the egui rendering in `src/app/ui.rs`: the menu bar (File: settings, exit; View: Live Log toggle), the status region (indicators plus Suspend/Resume, Go Fish/Stop, beacon light with tooltip, Install, Uninstall with a confirmation modal), the skills region (one row per slot), the bottom log panel (colorized, pause-scroll, level filter), and the settings window (all categories). Reads the view-model and raises intents only. Clippy-clean.
- [x] T018 Implement `main.rs` as the eframe entry point: resolve directories, load `Settings`, init logging, build the subsystems as shared handles, spawn the input backend thread, the weave worker (drains the action receiver through a `RealSink`), and the pixel-bus worker (samples the reader and calls `route_reader_event` plus fishing ticks on the interval), apply theme and always-on-top, and run the eframe app. Compiles clean on the host and type-checks on the Linux target.

## Phase 10: Polish and cross-cutting

- [x] T019 [P] Add module and item documentation across `src/app/*.rs`.
- [x] T020 Update `CHANGELOG.md` `[Unreleased]` with an Added line for the GUI and a dated Decisions entry for adding the `eframe`/`egui` (glow backend) dependency, the view-model/rendering split for testability, the thread model (input hook thread preserved, eframe on the main thread), and the additive `pixelbus` and `ui` settings sections.
- [x] T021 Run CI parity: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked`, plus `cargo check --target x86_64-unknown-linux-gnu`. Then run the `quickstart.md` manual validation checklist is deferred to the operator's hardware; note in the pre-push summary that automated verification excludes the live window.

## Dependencies and order

- Setup (T001) then config additions (T002 to T004). The pure derivations (US2,
  T005 to T006), routing (US6, T007 to T008), skill rows (US3, T009 to T010), log
  view (US4, T011 to T012), and settings form (US5, T013 to T014) are independent
  of each other and build on the config additions. The AppModel (US1, T015 to T016)
  composes the derivations and intents. Rendering and entry point (T017 to T018)
  depend on the whole view-model. Polish (T019 to T021) last; T021 is the gate.

## Parallel opportunities

- The test-authoring tasks (T003, T005, T007, T009, T011, T013, T015) span three
  test files; those in different files can be written in parallel, those sharing a
  file land sequentially.
- T017 (egui rendering) and the view-model tests are independent once the
  view-model exists.

## MVP scope

The view-model (US2 beacon light and labels, US3 skills, US1 AppModel intents) plus
the settings round-trip (US5) and the reader routing (US6) are the testable core
that proves the integration is correct. The log view (US4), the egui rendering, and
the entry point complete the window; the rendering is validated manually.
