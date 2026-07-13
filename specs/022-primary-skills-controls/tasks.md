---
description: "Task list for primary and skills panel controls (slice 022)"
---

# Tasks: Primary and Skills Panel Controls

**Input**: Design documents from `specs/022-primary-skills-controls/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/update-intent.md

**Tests**: The model Update intent is unit-tested (test-first). The visual changes
(alignment, fixed dropdown width, delay field appearance) are validated manually
per quickstart.md; a native window cannot be exercised headlessly.

## Phase 1: Strings

- [x] T001 In `src/app/strings.rs` rename `SKILL_COLUMNS[4].0` from `"Delay"` to
  `"Delay (ms)"`, and add a `BEACON_UPDATE_TOOLTIP` constant. Add
  `BEACON_UPDATE_TOOLTIP` to `all_tooltips()` so the non-empty coverage test
  covers it.

---

## Phase 2: Model Update intent (test-first)

- [x] T002 In `tests/app_view_model.rs` add `update_beacon_intent_reinstalls`:
  install the beacon (assert `InstalledCurrent`), apply `UiIntent::UpdateBeacon`,
  and assert the condition is still `InstalledCurrent` and `uninstall_enabled` is
  true. (Write first; it fails to compile until T003.)
- [x] T003 In `src/app/mod.rs` add `UiIntent::UpdateBeacon` and handle it in
  `apply_intent` by calling `self.uninstall_beacon()` then `self.install_beacon()`
  and returning no notices. Do not weaken the managed-marker uninstall gate.

---

## Phase 3: Status section (US1, US2)

- [x] T004 [US1] In `src/app/ui.rs` add an Update button in the beacon control row
  (with Install and Uninstall), using `add_enabled(view.uninstall_enabled, ..)`
  and `BEACON_UPDATE_TOOLTIP`, that pushes `UiIntent::UpdateBeacon`.
- [x] T005 [US2] In `src/app/ui.rs` move the Weapon Bar line into the `"status"`
  grid as a fourth row: title cell via `label_strong`, then the colorized state
  cell, then `end_row()` (empty control cell), and remove the separate
  `ui.horizontal` block so the title and state align with the rows above.

---

## Phase 4: Shared fixed-width combo helper (US3)

- [x] T006 [US3] In `src/app/ui.rs` add a `COMBO_WIDTH` constant and a `combo(id_salt,
  selected_text) -> egui::ComboBox` helper that presets `.width(COMBO_WIDTH)` and
  `.selected_text(..)`.
- [x] T007 [US3] In `src/app/ui.rs` route the weave selector and the live-log level
  filter through `combo(..)` so their resting width no longer tracks the selection.

---

## Phase 5: Delay column (US4)

- [x] T008 [US4] In `src/app/ui.rs` add a `delay_edit: Option<(u8, String)>` field
  to `EsoWeaveApp`, initialized to `None` in `new`.
- [x] T009 [US4] In `src/app/ui.rs` replace the delay cell so both states render an
  `egui::TextEdit::singleline` with a fixed four-digit `desired_width` and
  `horizontal_align(egui::Align::RIGHT)`: editable when `row.is_override`
  (buffered through `delay_edit`, filtered to at most four digits, parsed to `u32`
  with empty as 0, pushing `EditSkill(.., override_edit_for(weave_type, Some(v)))`
  on change and clearing `delay_edit` on focus loss), and wrapped in
  `add_enabled(false, ..)` showing the inherited value when not overriding. Keep the
  Delay tooltip on both.

---

## Phase 6: Verification

- [x] T010 Run CI parity in the foreground: `cargo fmt --all -- --check`,
  `cargo clippy --all-targets --all-features -- -D warnings`,
  `cargo test --all --locked`. Fix any findings.
- [x] T011 Update `CHANGELOG.md` `[Unreleased]` with a Fixed/Added entry describing
  the Update button, the aligned Weapon Bar, the fixed-width dropdowns, and the
  Delay (ms) column overhaul.
- [ ] T012 Walk the manual validation in `quickstart.md` against the running app
  (Update enable/disable and reinstall, Weapon Bar alignment, weave and log
  dropdown width stability, Delay (ms) header and field appearance/editing).

---

## Dependencies & Execution Order

- T001 (strings) before T004 (Update tooltip) and before the Delay header shows.
- T002 before T003 (test-first).
- T006 before T007 (helper before use).
- T008 before T009 (edit-buffer field before the delay cell uses it).
- Phase 6 last; T010 gates the commit.

## Notes

- Presentation-only except the Update intent; the egui layer stays thin.
- Safety-critical surfaces (input engine, beacon managed-marker uninstall) are not
  touched.
- Commit as `feat(022): primary and skills panel controls` after T010; halt before
  push per the autopilot protocol.
