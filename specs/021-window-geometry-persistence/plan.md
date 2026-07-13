# Implementation Plan: Window Geometry Persistence

**Branch**: `021-window-geometry-persistence` | **Date**: 2026-07-13 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/021-window-geometry-persistence/spec.md`

## Summary

Record the application window's outer position, inner size, and maximized state as
they change while running, and restore them at launch so the window reopens where
and how the user last left it, including on the same monitor of a multi-monitor
desktop. Geometry is automatically captured runtime state, so it is persisted with
the existing session state in `state.json` (not the user settings in
`config.json`), advancing the session-state schema version with an additive
forward migration. All correctness logic (a pure geometry-sanitize helper and the
migration) is unit-tested; the egui layer only reads viewport info and raises an
intent, and `main.rs` only feeds sanitized values into the viewport builder.

## Technical Context

**Language/Version**: Rust 1.96 (edition 2021)

**Primary Dependencies**: eframe/egui 0.35 (viewport info and `ViewportBuilder`);
serde/serde_json (state persistence); windows-sys 0.59 on Windows (virtual-screen
metrics via `GetSystemMetrics`, system DPI via `GetDpiForSystem`).

**Storage**: `state.json` under the per-user config directory, the existing
session-state store (UTF-8 without BOM, LF, trailing newline).

**Testing**: `cargo test` unit tests for the pure sanitize helper and the
migration; the GUI capture/restore path is validated manually per `quickstart.md`
(a native window cannot be exercised headlessly).

**Target Platform**: Windows 10/11 x64 and Linux x64 desktop.

**Project Type**: Single-crate desktop application.

**Performance Goals**: No per-frame allocation or disk churn; geometry writes are
coalesced through the existing settle-delayed scheduler (one write after motion
settles), plus one forced write on close.

**Constraints**: No new networked or heavyweight dependency; geometry restore uses
only egui point-space values plus platform-supplied screen bounds; text hygiene
(UTF-8 no BOM, LF, no em/en dashes) holds.

**Scale/Scope**: One window, one small optional state section, one pure helper,
one platform accessor per OS.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **I. Spec-Driven Development**: This slice runs the full spec-kit sequence and
  traces to the master specification section 10 (GUI) and section 11 (config and
  state persistence). PASS.
- **II. Safety-Critical Surfaces**: No safety-critical surface is touched. Input
  suppression, injected-input recursion, the hook thread, beacon uninstall, and
  fishing degradation are unchanged. The existing suspend and fishing restore
  behavior is preserved (FR-009). PASS.
- **III. Test-First With Explicit Seams**: The correctness surface (geometry
  sanitize/clamp and the migration) is a pure function tested with mock inputs;
  the platform screen-bounds accessor is the seam, and the pure helper receives
  its numbers as parameters so no test needs a real display. Red-green-refactor.
  PASS.
- **IV. CI Parity Before Every Commit**: fmt, clippy (-D warnings), and
  `cargo test --all --locked` run in the foreground before commit. PASS.
- **V. Bounded Scope**: No game memory, no packets, nothing inside the game. Pure
  desktop window-manager interaction. PASS.
- **Config vs state separation**: Geometry is runtime state, so it lives in
  `state.json`, never in `config.json`. This directly honors the constitution's
  "configuration stores user settings only" constraint. PASS.

No violations. Complexity Tracking is empty.

## Project Structure

### Documentation (this feature)

```text
specs/021-window-geometry-persistence/
|-- plan.md              # This file
|-- spec.md              # Feature specification
|-- research.md          # Phase 0 decisions
|-- data-model.md        # Phase 1 data model
|-- quickstart.md        # Phase 1 manual validation
|-- contracts/
|   `-- geometry-helpers.md   # Pure-helper behavioral contract
|-- checklists/
|   |-- requirements.md  # Spec quality checklist
|   `-- persistence.md   # Requirements-quality checklist
`-- tasks.md             # Created by /speckit-tasks
```

### Source Code (repository root)

```text
src/
|-- config/
|   `-- state.rs         # WindowGeometry type, SessionState.window field,
|                        #   CURRENT_STATE_VERSION 2 -> 3, sanitize_geometry
|                        #   pure helper, and their unit tests
|-- platform/
|   |-- mod.rs           # virtual_screen_bounds_points() seam (points)
|   |-- windows.rs       # GetSystemMetrics + GetDpiForSystem implementation
|   `-- linux.rs         # returns None (WM owns placement)
|-- app/
|   `-- mod.rs           # AppModel.window field, UiIntent::SetWindowGeometry,
|                        #   current_session_state includes window,
|                        #   restore_session seeds window, flush_session_now
|-- app/ui.rs            # per-frame geometry capture (maximized-preserving),
|                        #   close/exit force-flush; seeds last geometry
`-- main.rs              # read restored geometry from session, sanitize,
                         #   feed ViewportBuilder (position/size/maximized)
```

**Structure Decision**: Single crate is retained (constitution constraint). The
change is additive across the existing config/state, platform, and app modules;
no new crate, module tree, or workspace is introduced. `Cargo.toml` gains one
windows-sys feature (`Win32_UI_HiDpi`) for `GetDpiForSystem`; windows-sys is not
a pinned artifact, and the addition is recorded as a dated decision in the
changelog alongside the feature's Added entry.

## Key Decisions (autopilot decision log)

- **Storage location: `state.json` (session state), not `config.json`.**
  Alternatives: config.json `ui` section (where `log_panel_height` lives) or
  eframe's ron `persist_window`. Chosen state.json because window geometry is
  automatically captured runtime state, which the constitution explicitly keeps
  out of the settings file; eframe ron persistence was rejected because it would
  introduce a second, differently located persistence system and the persistence
  feature is intentionally disabled. Architecture-affecting: recorded here and in
  CHANGELOG Decisions.
- **Coordinate space: egui points, stored as integers.** Capture the outer-rect
  top-left (position) and inner-rect size in egui points, rounded to integers
  (sub-pixel window geometry is meaningless and integers keep `SessionState` both
  `Copy` and `Eq`). Restore feeds the same points into `ViewportBuilder`, so the
  round trip is DPI-consistent by construction.
- **Off-screen recovery via point-space virtual-screen bounds.** The Windows
  accessor reads the physical virtual-screen rectangle (`GetSystemMetrics`) and
  divides by the system DPI scale (`GetDpiForSystem`/96) to yield point-space
  bounds, which the pure `sanitize_geometry` helper uses to require a visible
  margin; if the recorded window does not overlap the desktop by that margin, the
  position is dropped and the window opens centered at the restored size. Linux
  returns no bounds (Wayland cannot report or set absolute position; X11 is left
  to the window manager), so only size and maximized restore there.
- **Maximized-preserving capture.** While the window is maximized, the last normal
  position and size are retained and only the maximized flag is set, so
  unmaximizing returns to the prior normal geometry rather than the maximized
  rectangle. Windows snap (half-screen) is a normal move/resize, captured as
  normal geometry.
- **Write cadence: reuse the settle-delayed scheduler plus a forced flush on
  close.** Geometry changes mark the session store dirty and coalesce into one
  settle-write after motion stops; on window close or the Exit menu, a forced
  session write captures a change made in the final moments before exit
  (FR-008).

## Phasing

- Phase 0 (research.md): confirm the egui viewport fields, the winit
  point-space contract, the Windows metrics, and the Wayland limitation.
- Phase 1 (data-model.md, contracts/, quickstart.md): define `WindowGeometry`,
  the `SessionState.window` field and migration, the `sanitize_geometry`
  contract, and the manual validation script.
- Phase 2 (/speckit-tasks): test-first task list.

## Complexity Tracking

No constitution violations; no entries.
