# Phase 0 Research: Primary and Skills Panel Controls

## How is the status section currently laid out, and why is the Weapon Bar misaligned?

`src/app/ui.rs::main_view` draws Status, Fishing, and Pixel Beacon as rows of one
`egui::Grid::new("status")` (3 columns, `min_col_width(110.0)`), each via
`status_cells` (title cell + colorized state cell) plus a control cell. The Weapon
Bar line is drawn afterward in a separate `ui.horizontal(..)`, so it does not
participate in the grid's columns and its title does not align. Moving it into the
grid as a fourth row (title cell, state cell, empty control cell) aligns it.

## Why do the dropdowns change width, and how is that fixed?

`egui::ComboBox::from_id_salt(..)` with no `.width(..)` sizes the resting button to
the selected text, so the field width changes with the selection (Light Attack vs
Block Casting; OFF vs TRACE). `ComboBox::width(f32)` fixes the button width so it
no longer tracks content. The slice-019 theme fix already zeroed hover expansion
for all widget states, so the remaining shift is purely this content-driven width.
A single shared width comfortably fits the longest option in use ("Block Casting")
with breathing room.

## How should the Delay field render for both states, right-aligned and four-digit?

The current cell uses `egui::DragValue` when overriding and a muted `egui::Label`
when not, which differ in width and appearance and shift the row on toggle.
`egui::TextEdit::singleline` supports `desired_width(f32)` (fixed four-digit width)
and `horizontal_align(egui::Align::RIGHT)` (right-aligned digits), and renders as a
boxed field. For the override-off state, wrapping the same TextEdit in
`ui.add_enabled(false, ..)` yields a greyed, read-only, same-width box showing the
inherited value. `DragValue` was rejected: it has no internal text alignment and
its box does not match a greyed read-only field.

TextEdit edits a `String`, so a small per-row buffer is needed so the model value
does not overwrite in-progress typing each frame. A single
`delay_edit: Option<(u8, String)>` on `EsoWeaveApp` holds the slot index and the
digits being typed; on change it is filtered to ASCII digits, bounded to four
characters, parsed to a `u32` (empty -> 0, a valid override), and pushed as an
`EditSkill` override; it is cleared when the field loses focus so the field then
reflects the model value again.

## How is Update implemented without weakening safety?

The model already has `install_beacon` and `uninstall_beacon`. `uninstall_beacon`
calls `beacon::uninstall`, which deletes a folder only after verifying the managed
marker; an unmanaged folder is refused (logged), never deleted. A new
`UiIntent::UpdateBeacon` runs `uninstall_beacon` then `install_beacon`. On a
managed install this is a clean reinstall; on an unmanaged folder the uninstall is
refused and the install still writes the managed addon over it. The gate is
untouched. The Update button is enabled on the same condition as Uninstall
(`view.uninstall_enabled`: an installed folder is present), so it is greyed when
the addon is not installed.

## Alternatives considered

- **DragValue with `add_sized`** for the delay field: rejected; it centers rather
  than right-aligns its text and cannot be made to match a greyed read-only box.
- **Per-call `.width(..)`** on each ComboBox: works, but a shared helper keeps one
  width and one tuning point and lets slice 023 reuse it for the settings
  dropdowns.
- **A confirm dialog for Update** (like Uninstall): rejected; Update is an explicit
  maintenance action and the existing Uninstall confirm covers deliberate removal;
  a reinstall does not risk data loss beyond what the managed-marker gate governs.
