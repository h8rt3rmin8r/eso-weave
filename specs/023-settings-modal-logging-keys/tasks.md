---
description: "Task list for settings modal, logging linkage, and key presentation (slice 023)"
---

# Tasks: Settings Modal, Success Toast, Logging Linkage, and Key Presentation

**Input**: Design documents from `specs/023-settings-modal-logging-keys/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/helpers.md

**Tests**: The pure helpers (modal sizing, key names) and the log linkage are
unit-tested (test-first). The visual behavior (modal scaling, green toast, dropdown
widths) is validated manually per quickstart.md.

## Phase 1: Key display names and F2 (test-first)

- [x] T001 In `src/input/key.rs` add unit tests for a new `display_name`: every
  variant is non-empty and underscore-free; Digit1 -> "Number 1", Space ->
  "Space", F2 -> "F2". (Write first; fails until T002.)
- [x] T002 In `src/input/key.rs` add `pub fn display_name(self) -> &'static str`
  mapping Digit1..Digit5 -> Number 1..Number 5, E/R/X/Q -> uppercase letters,
  Space -> "Space", F1/F2 -> "F1"/"F2". Leave `as_str`/`parse` unchanged.
- [x] T003 In `src/app/ui.rs` add `Key::F2` to the `KEYS` array (length 11 -> 12).

## Phase 2: Modal-extent helper (test-first)

- [x] T004 In `tests/app_view_model.rs` add tests for `modal_extent(window, min_px,
  max_px, max_frac)`: a small window returns about `max_frac * window` (fits); a
  mid window returns more pixels but a smaller fraction than the small window; a
  very large window is capped at `max_px`. (Write first; fails until T005.)
- [x] T005 In `src/app/mod.rs` add `pub fn modal_extent(window, min_px, max_px,
  max_frac) -> f32` beside `clamp_log_height`: grow sub-linearly from `min_px`,
  clamp to `[min_px, max_px]`, then cap at `max_frac * window`. Make T004 pass.

## Phase 3: Log-level linkage (test-first)

- [x] T006 In `tests/app_view_model.rs` add a linkage test: apply
  `SetLogFilter(Debug)` and assert `view().log_filter == Debug` and
  `settings_form().logging.level == Debug`; apply settings with `logging.level =
  Warn` and assert `view().log_filter == Warn`; toggle the log panel and assert
  `settings_form().logging.level` is unchanged. (Write first; fails until T007.)
- [x] T007 In `src/app/mod.rs` change `SetLogFilter(level)` to also set
  `settings.logging.level`, call `self.log.set_level(level)`, and
  `scheduler.mark_config(now)`; and in `reload_from_settings` set `self.log_filter
  = self.settings.logging.level`. Leave `ToggleLogPanel` untouched.

## Phase 4: Settings modal sizing and dropdowns (US4, FR-008)

- [x] T008 [US4] In `src/app/ui.rs::settings_modal` set the modal width to
  `modal_extent(screen.width(), ..)` and the scroll-area max height to
  `modal_extent(screen.height(), ..)` minus a header allowance, using axis-specific
  min/max caps chosen to look right from the minimum window to a QHD ultrawide
  display.
- [x] T009 In `src/app/ui.rs::settings_body` route the theme, beacon environment,
  and log level dropdowns through the shared `combo(..)` helper.

## Phase 5: Keybinding presentation (US1, US2)

- [x] T010 [US1][US2] In `src/app/ui.rs` render the keybinding dropdowns through
  `combo(("bind", action.as_str()), key_display(selected))` and label each option
  with `key.display_name()`.

## Phase 6: Green toast (US5)

- [x] T011 [US5] In `src/app/widgets.rs::Toast::show` fill with `palette.ok` and
  draw the message in `palette.base` at a heavier weight, keeping the fade and
  bottom-right anchor, so the confirmation is clearly green and legible in both
  themes.

## Phase 7: Verification

- [x] T012 Run CI parity in the foreground: `cargo fmt --all -- --check`,
  `cargo clippy --all-targets --all-features -- -D warnings`,
  `cargo test --all --locked`. Fix any findings.
- [x] T013 Update `CHANGELOG.md` `[Unreleased]` with Fixed/Added entries for F2 and
  friendly key names, the log-level linkage, the modal scaling, and the green toast.
- [ ] T014 Walk the manual validation in `quickstart.md` against the running app.

---

## Dependencies & Execution Order

- T001 before T002 (test-first); T002 before T010 (uses display_name).
- T004 before T005 (test-first); T005 before T008 (uses modal_extent).
- T006 before T007 (test-first).
- T003 and T010 both touch keybinding presentation; do T003 first.
- Phase 7 last; T012 gates the commit.

## Notes

- Presentation-only except the log-linkage view-model change; the egui layer stays
  thin.
- Safety-critical surfaces are not touched.
- Commit as `feat(023): settings modal, logging linkage, and key presentation`
  after T012; halt before push per the autopilot protocol.
