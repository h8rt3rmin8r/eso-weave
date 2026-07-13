---
description: "Task list for window geometry persistence (slice 021)"
---

# Tasks: Window Geometry Persistence

**Input**: Design documents from `specs/021-window-geometry-persistence/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/geometry-helpers.md

**Tests**: REQUIRED. The constitution mandates test-first for correctness logic.
The pure sanitize helper and the migration are unit-tested; the GUI capture and
restore path is validated manually per quickstart.md (a native window cannot be
exercised headlessly).

**Note on user stories**: US1 (restore size/position), US2 (maximized), and US3
(same monitor) are delivered by one shared capture/restore mechanism, so the
foundational data model and helper implement all three. Story labels below mark
which acceptance scenarios each task serves.

## Phase 1: Setup

- [x] T001 Add the windows-sys feature `Win32_UI_HiDpi` to the `cfg(windows)`
  dependency in `Cargo.toml` (for `GetDpiForSystem`), keeping the existing
  features. No other dependency change.

---

## Phase 2: Foundational (data model, pure helper, migration) - BLOCKS all stories

### Tests first (write, run, watch them fail)

- [x] T002 [P] In `src/config/state.rs` add unit tests for `sanitize_geometry`
  covering: in-range geometry with bounds returns same size and `Some(position)`;
  zero/sub-minimum size clamps up to minimum; oversized size clamps down to
  maximum; a position fully outside the virtual screen returns `position = None`
  with size preserved; a position overlapping by more than the margin keeps
  `Some(position)`; `virtual_screen: None` returns `Some(position)`;
  `maximized: true` preserved regardless of position outcome. (Contract:
  contracts/geometry-helpers.md.)
- [x] T003 [P] In `src/config/state.rs` add migration/round-trip tests: a
  `state.json` payload without a `window` key deserializes with `window: None`;
  a `SessionState` carrying a `WindowGeometry` round-trips every field; assert
  `CURRENT_STATE_VERSION == 3`.

### Implementation

- [x] T004 In `src/config/state.rs` add the `WindowGeometry` struct (`x: i32`,
  `y: i32`, `width: u32`, `height: u32`, `maximized: bool` with `#[serde(default)]`)
  deriving `Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default`.
- [x] T005 In `src/config/state.rs` add `window: Option<WindowGeometry>` to
  `SessionState` with `#[serde(default, skip_serializing_if = "Option::is_none")]`,
  include it in `Default`, and bump `CURRENT_STATE_VERSION` 2 -> 3. Confirm
  `SessionState` still derives `Copy` and `Eq`.
- [x] T006 In `src/config/state.rs` add the `RestoreBounds` and `GeometryRestore`
  types and the pure `sanitize_geometry(geo, bounds) -> GeometryRestore` per the
  contract (size clamping, visible-margin position check with bounds, position
  trust without bounds, maximized passthrough; no I/O). Make T002 pass.

**Checkpoint**: data model and helper complete and unit-tested.

---

## Phase 3: Platform accessor (seam)

- [x] T007 In `src/platform/mod.rs` declare
  `pub fn virtual_screen_bounds_points() -> Option<(i32, i32, i32, i32)>`
  delegating to the per-OS backend.
- [x] T008 [P] In `src/platform/windows.rs` implement it: read
  `GetSystemMetrics(SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN, SM_CXVIRTUALSCREEN,
  SM_CYVIRTUALSCREEN)`, divide by `GetDpiForSystem() as f32 / 96.0`, round to
  point-space `(x, y, w, h)`, and return `None` if width/height are non-positive.
- [x] T009 [P] In `src/platform/linux.rs` implement it as `None` (window-manager
  owns placement; Wayland cannot report position).

---

## Phase 4: Model wiring (US1, US2, US3)

- [x] T010 [US1][US2][US3] In `src/app/mod.rs` add the `window:
  Option<WindowGeometry>` field to `AppModel`, initialize it to `None` in `new`,
  and add `UiIntent::SetWindowGeometry(WindowGeometry)` handling that updates the
  field and calls `scheduler.mark_session(Instant::now())`.
- [x] T011 [US1][US2][US3] In `src/app/mod.rs` include `window: self.window` in
  `current_session_state()`, and seed `self.window = state.window` in
  `restore_session` (without altering suspend/fishing behavior).
- [x] T012 [US1] In `src/app/mod.rs` add `flush_session_now(&mut self) -> bool`
  that writes the current session state to disk immediately (used on close), and
  clears the scheduler's session-dirty flag so a duplicate write does not follow.

---

## Phase 5: GUI capture and restore (US1, US2, US3)

- [x] T013 [US2] In `src/app/ui.rs` add a `last_geometry: Option<WindowGeometry>`
  field to `EsoWeaveApp`, seed it from the restored geometry passed into `new`
  (new parameter), so an unchanged restored window is not immediately re-saved.
- [x] T014 [US1][US2][US3] In `src/app/ui.rs` each frame read the egui viewport
  (`ctx.input(|i| i.viewport())`): take position from `outer_rect.min` and size
  from `inner_rect.size` (rounded to points); when `maximized` is true keep the
  last normal position/size and only set the maximized flag; when not maximized
  and `outer_rect`/`inner_rect` are present, capture the new normal geometry.
  Raise `UiIntent::SetWindowGeometry` only when the computed geometry differs from
  `last_geometry`, and update `last_geometry`.
- [x] T015 [US1] In `src/app/ui.rs`, when the Exit menu item is chosen or
  `ctx.input(|i| i.viewport().close_requested())` is true, capture the final
  geometry (apply a `SetWindowGeometry`) and call `model.flush_session_now()`
  before the window closes, so a resize just before quitting persists (FR-008).
- [x] T016 [US1][US2][US3] In `src/main.rs` read the restored `WindowGeometry`
  from the loaded session before it is consumed (mirroring the existing
  `stored_api_version` capture), pass it into `EsoWeaveApp::new`, run it through
  `sanitize_geometry` with `RestoreBounds` built from `min_inner_size` (480x420),
  the default size (600x720), and `platform::virtual_screen_bounds_points()`, and
  apply the result to the `ViewportBuilder` (`with_inner_size`, `with_position`
  when `Some`, `with_maximized` when true). Preserve the existing icon and
  `min_inner_size` calls.

---

## Phase 6: Polish and verification

- [x] T017 Update `CHANGELOG.md` `[Unreleased]`: an `### Added` line for window
  geometry persistence, and a dated `### Decisions` entry recording (a) geometry
  stored in `state.json` as session state (schema 2 -> 3), and (b) the added
  windows-sys `Win32_UI_HiDpi` feature for system-DPI conversion.
- [x] T018 Run CI parity in the foreground: `cargo fmt --all -- --check`,
  `cargo clippy --all-targets --all-features -- -D warnings`,
  `cargo test --all --locked`. Fix any findings.
- [ ] T019 Walk the manual validation in `quickstart.md` on Windows (default
  launch, move/resize restore, resize-then-immediate-close, maximize, snap,
  secondary monitor, disconnected monitor, state-file hygiene, pre-feature state)
  and confirm each acceptance scenario.

---

## Dependencies & Execution Order

- T001 (setup) before T008 (uses the new HiDpi feature).
- Phase 2 (T002-T006) is foundational and blocks Phases 4-5.
- T007 before T008/T009; T008/T009 before T016.
- Phase 4 (model) before Phase 5 (GUI wiring).
- T013 before T014 (needs the seeded `last_geometry`).
- T016 depends on T006 (helper), T007-T009 (accessor), and T013 (new-arg).
- Phase 6 last; T018 gates the commit.

### Parallel opportunities

- T002 and T003 are parallel (both add tests, distinct concerns).
- T008 and T009 are parallel (different OS files).

## Notes

- Test-first: T002 and T003 are written and observed to fail before T004-T006.
- The GUI layer stays thin: it only reads viewport info and raises an intent; all
  decision logic is in the tested `sanitize_geometry` helper and the model.
- Commit as `feat(021): window geometry persistence` after T018 passes; halt
  before push per the autopilot protocol.
