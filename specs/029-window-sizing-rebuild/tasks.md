# Tasks: Window Sizing Model Rebuild

**Feature**: [spec.md](spec.md) | **Plan**: [plan.md](plan.md) |
**Contract**: [contracts/sizing.md](contracts/sizing.md)

Test-first per Constitution Principle III: the sizing math is pure and gets a
failing test before its implementation. The egui frame glue is not unit-testable
(no live context); it is validated by building and the manual desk checks in
[quickstart.md](quickstart.md). Paths are repo-relative.

## Phase 1: Setup

- [x] T001 Confirm the baseline merge gate is green before any change: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked` in the foreground.

## Phase 2: Foundational (pure sizing helpers, block all user stories)

- [x] T002 [P] Update the existing `content_min_size` tests and add `measurement_stable` tests in tests/app_window_sizing.rs: `measurement_stable(None, m, eps) == false`; two equal-within-eps frames are stable; a differing frame is not; `content_min_size(measured, boot, false) == boot` and `content_min_size(measured, boot, true) == measured` per dimension (including measured smaller than the floor); a later smaller stable measurement is not latched.
- [x] T003 In src/app/mod.rs change `content_min_size` to `(measured, boot_floor, stable) -> (f32,f32)` (boot floor until stable, then measured per dimension) and add `measurement_stable(prev: Option<(f32,f32)>, current: (f32,f32), epsilon: f32) -> bool`. Make T002 pass.
- [x] T004 [P] Add failing tests in tests/app_window_sizing.rs for `open_log_reserve` (equals `log_min_height + row`, strictly greater than `log_min_height`) and `split_log_height` (shares a delta by the live fraction `log_h/prev_window_h`; clamps to `[log_min, max(window - content, log_min)]`; central takes the remainder; guards `prev_window_h == 0`).
- [x] T005 In src/app/mod.rs add `open_log_reserve(row_height) -> f32` = `log_min_height(row_height) + row_height` and `split_log_height(prev_window_h, window_h, log_h, content_h, log_min) -> f32` per contracts/sizing.md. Make T004 pass.

## Phase 3: User Story 1 - The window minimum hugs the actual content (P1)

**Goal**: With the log closed, the enforced minimum equals the measured content
once stable (no dead band), and can shrink.

**Independent test**: Pure tests T002/T003 plus desk check MV-1.

- [x] T006 [US1] In src/app/ui.rs replace the running-max `content_min` with the stable-measured model: add state `content_extent: egui::Vec2` and `prev_measured: Option<egui::Vec2>`; each frame compute `stable = measurement_stable(prev_measured, measured, 0.5)`, set `content_extent = content_min_size(measured, BOOT_MIN_SIZE, stable)` (no running max), update `prev_measured`; use `content_extent` as the closed-log enforced minimum (`MinInnerSize`). Remove the permanent `.max` accumulation at ui.rs:387-390.

## Phase 4: User Story 2 - The log pane is resizable and has no phantom band (P1)

**Goal**: Log max computed against the true content height; resizable at the
enforced minimum open height; no phantom band.

**Independent test**: Pure resizability test T008 plus desk checks MV-2, MV-5.

- [x] T007 [US2] In src/app/ui.rs compute the log pane range from `content_extent.y` (max via `clamp_log_height(..., content_extent.y)`), and set the enforced open-window minimum height to `content_extent.y + crate::app::open_log_reserve(row_h)` (width stays `content_extent.x + LOG_WIDTH_BONUS`). Feed the true content height, not a running max.
- [x] T008 [P] [US2] Add a pure test in tests/app_window_sizing.rs asserting that at `window_h = content_h + open_log_reserve(row)` the log range has `max > min` (resizable at the minimum), and that for any `window_h >= content_h + log_min_height(row)` the range is valid (compressible window).

## Phase 5: User Story 3 - Window growth is shared with the log (P2)

**Goal**: A window-height change while open is split proportionally.

**Independent test**: Pure split tests T004/T005 plus desk check MV-3.

- [x] T009 [US3] In src/app/ui.rs add state `prev_window_h: Option<f32>`; when the log is open and the window height changed from `prev_window_h`, compute the new log height via `split_log_height(prev_window_h, window_h, log_h, content_extent.y, log_min)` and drive the bottom panel to that height for that frame (force the panel height, e.g. a collapsed height range), while on non-resize frames using the full `[log_min, max]` range and reading the user's drag back into `log_h` (as at ui.rs:311-315). Update `prev_window_h` each frame.

## Phase 6: User Story 4 - Opening and closing the log is height-neutral (P2)

**Goal**: Open/close round trip returns the window to its prior height.

**Independent test**: Desk check MV-4.

- [x] T010 [US4] In src/app/ui.rs rework the toggle grow/shrink (ui.rs:342-366): on open grow the window by the log height actually shown (`clamp_log_height(log_h, ...)`), on close shrink by the pane's actual realized height (the last read-back `log_h`), not a fixed `log_min` delta. Ensure the persisted `log_panel_height` (`UiPrefs`) remains the single source of truth and `default_size` does not fight it on reopen within a session.

## Phase 7: Polish and cross-cutting

- [x] T011 In src/main.rs confirm `MIN_SIZE`/`BOOT_MIN_SIZE` remain the pre-measurement boot floor only and the comment (main.rs:237-243) is still accurate after the model change; adjust wording only if needed.
- [x] T012 [P] Update CHANGELOG.md `[Unreleased]`: Fixed entries for the three defects (closed dead band, frozen/phantom log pane, non-height-neutral close) and a dated Decisions entry (measured supersedes boot floor once stable; two-frame stability gate; proportional split from live pane fractions; one extra line of drag room in the open-window floor; height-neutral close by actual pane height).
- [x] T013 Run the full merge gate in the foreground and confirm green: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked`.
- [~] T014 Desk validation: PARTIAL. `cargo run` smoke launch confirmed the app starts and the new sizing state/egui glue initialize with no panic. The interactive MV-1..MV-5 checks (dragging the splitter, resizing the window, open/close round trip) require a human at the display and are OWED manual validation before the release is cut. The pure sizing math (measured-wins minimum, proportional split, drag-room reserve) is fully covered by tests/app_window_sizing.rs.

## Dependencies

- Phase 2 (T002-T005) blocks all user stories (helpers used by every glue task).
- US1 (T006) depends on `content_min_size`/`measurement_stable` (T003).
- US2 (T007-T008) depends on US1 (`content_extent`) and `open_log_reserve` (T005).
- US3 (T009) depends on US2 (range) and `split_log_height` (T005).
- US4 (T010) depends on US2/US3 state (`log_h`, `content_extent`).
- Polish (T011-T014) last; T013 is the merge gate, T014 the desk sign-off.

## Parallel opportunities

- T002 and T004 (independent test sections) can be written together.
- T008 (pure resizability test) is independent of the T009/T010 glue.
- T012 (changelog) is independent and parallelizable.

## Implementation strategy

MVP is US1 + US2 (the two P1 defects: the dead band and the frozen/phantom log
pane), which are the core of the reported regression. US3 (proportional growth)
and US4 (height-neutral close) complete the model. All sizing math is pure and
tested; the egui glue is validated by build + desk MV checks.
