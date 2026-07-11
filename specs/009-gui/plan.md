# Implementation Plan: Graphical User Interface

**Branch**: `009-gui` | **Date**: 2026-07-11 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/009-gui/spec.md`

## Summary

Add an `app` module and an eframe/egui entry point that integrate every
subsystem behind a testable view-model. The correctness-bearing logic (display
derivation, UI-intent handling, the settings-to-config mapping for all of section
10.3, the log view snapshot/filter/color, and the reader-event routing) lives in
pure functions and small structs that are unit-tested against in-memory
subsystems and crafted configuration. The egui rendering (`app::ui`) is a thin
layer that reads the view-model and emits intents; it is validated with a
documented manual checklist, since a native window cannot be exercised in the
automated environment. Two new additive settings sections are added (`pixelbus`
for sampling interval and tolerance, and `ui` for theme and always-on-top) to
complete section 10.3. Adds the `eframe`/`egui` dependency with the glow backend.

## Technical Context

**Language/Version**: Rust 1.96.0, edition 2021 (unchanged).

**Primary Dependencies**: Adds `eframe` 0.35 with `default-features = false` and
features `glow`, `default_fonts`, `x11`, `wayland` (the glow OpenGL backend, no
wgpu). Integrates features 001 (config, logging `LogHandle`), 002 (input engine
suspend and bindings), 003 (weave engine and its config), 006 (beacon manager),
and 007 (fishing controller), plus 005's `PixelBusReader` and `PixelBusEvent` for
the worker routing. No other new crates.

**Storage**: User settings only. Existing per-subsystem sections
(`bindings`, `timing`, `skills`, `latency`, `fishing`, `beacon`) are reused; two
new additive sections (`pixelbus`, `ui`) are added, so no config `schema_version`
bump. The config store's corruption fallback is reused unchanged.

**Testing**: `cargo test`. The view-model derivations, intent handling, settings
mapping, reader-event routing, and log view are unit-tested against in-memory
subsystems (mock input backend, `MockSink`, `MockFishingSink`, a tempdir AddOns
root, a crafted `Settings`) and a captured logging dispatch. The egui rendering
and the real windowing/threading are validated on real hardware via a manual
checklist in `quickstart.md`.

**Target Platform**: Windows 10 and 11 x64, Linux x64 (X11 or XWayland via glow).

**Project Type**: Single desktop-application crate (unchanged).

**Performance Goals**: The UI redraws at the framework's cadence; the view-model
reads cheap snapshots. The worker loop samples the reader at the configured
interval and never blocks the UI thread.

**Constraints**: The view-model and all logic MUST be unit-testable without a live
window (Principle III). The input hook thread's contract (its own message pump, no
blocking work) is preserved: the input backend runs on its own thread as before,
and eframe owns the main thread. SignalLost routing disables fishing (Principle II
safety surface) and clears weave latency.

**Scale/Scope**: One window with four regions, six user stories, ~10 UI intents,
nine settings categories (two new sections), one routing table.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

- **I. Spec-Driven Development**: PASS. Derived from `spec.md` (master spec section
  10), bounded by `docs/plans/plan-001.md` slice 009.
- **II. Safety-Critical Surfaces**: PASS. The window drives the existing safety
  surfaces without weakening them: fishing disables on SignalLost (the routing
  calls the controller's SignalLost path), the beacon uninstall still goes through
  the manager's marker gate (the UI only adds a confirmation), and the input hook
  thread keeps its own message pump and non-blocking contract (eframe runs on a
  separate main thread). These paths remain covered by their existing tests plus
  the routing tests here.
- **III. Test-First With Explicit Seams**: PASS. The view-model (derivations,
  intent handling, settings mapping, routing, log view) is the seam and is
  unit-tested against in-memory subsystems; the egui rendering is thin and excluded
  from the tested surface, validated manually. Tests precede the logic.
- **IV. CI Parity Before Every Commit**: PASS on the host; the glow backend
  compiles on the host and is type-checked on the Linux target. The GUI is not
  exercised by the automated suite.
- **V. Bounded Scope: Outside The Game**: PASS. The window only drives the existing
  subsystems; no new game access.
- **Platform and Text Hygiene Constraints**: PASS. Settings remain user-only and
  additive; the `eframe` dependency addition is recorded as a dated CHANGELOG
  decision; new text is UTF-8 without BOM, LF, no em/en dashes.

No violations. Complexity Tracking is empty (the GUI framework is spec-named).

## Project Structure

### Documentation (this feature)

```text
specs/009-gui/
├── plan.md, research.md, data-model.md, quickstart.md
├── contracts/
│   └── app.md       # view-model, intents, derivations, settings form, routing, log view
├── checklists/{requirements.md, view-model-and-wiring.md}
├── spec.md
└── tasks.md
```

### Source Code (repository root)

```text
src/app/
├── mod.rs         # AppModel (holds shared subsystem handles), UiIntent, apply_intent,
│                  # derived view (app-state label, fishing label, beacon light), SkillRow
├── beacon_light.rs# BeaconCondition + beacon_light() derivation (pure)
├── settings_form.rs # SettingsForm: load from / apply to Settings for all 10.3 categories;
│                  # new UiPrefs (theme, always_on_top) and PixelBusPrefs mapping
├── routing.rs     # route_reader_event(event, weave, fishing, now, sink) (reuses fishing::map_event)
├── log_view.rs    # LogRow, level_color, build_log_view(events, filter) + autoscroll rule
└── ui.rs          # egui rendering (thin, untested): menu bar, status, skills, log panel, settings window
src/config/mod.rs  # additive `pixelbus` and `ui` sections; Theme enum
src/pixelbus/mod.rs# ReaderConfig serde mapping helpers (load/store from the `pixelbus` section)
src/main.rs        # eframe entry: build subsystems, spawn input + weave + pixel-bus worker threads, run
tests/
├── app_view_model.rs  # derivations, intents, beacon light, skills, routing
├── app_settings.rs    # settings form round-trip for every 10.3 category
└── app_log_view.rs     # log snapshot, filter, colors, autoscroll
```

**Structure Decision**: Everything correctness-bearing is a pure function or a
small struct in `app::*` (outside `ui.rs`), tested against the real in-memory
subsystems the project already provides as test doubles. `ui.rs` holds only egui
calls that read the view-model and raise intents, so the untestable surface is
isolated and thin. `main.rs` wires the threads; its windowing and real backends
are validated manually. This mirrors the maximal-testable-core pattern of the
prior slices, applied to a GUI by splitting view-model from rendering.

## Complexity Tracking

No constitution violations. The `eframe` dependency is required by the
spec-named GUI framework and is recorded as a dated CHANGELOG decision; it is not
a constitution violation, so no Complexity Tracking entry is needed.
