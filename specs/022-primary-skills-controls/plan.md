# Implementation Plan: Primary and Skills Panel Controls

**Branch**: `022-primary-skills-controls` | **Date**: 2026-07-13 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/022-primary-skills-controls/spec.md`

## Summary

Correct five presentation defects in the main window's status section and Skills
grid: add an addon Update control (uninstall then install, greyed when not
installed), move the Weapon Bar row into the status grid so it aligns, give the
Weave dropdown a fixed width via a shared combo helper (also applied to the
live-log level filter), and overhaul the Delay column (header Delay (ms), a
matching greyed read-only field when not overriding, four-digit width,
right-aligned in both states). All logic stays thin in the egui layer; the only
model change is a new Update intent, and the only tested surface is that intent
plus the strings hygiene tests. The visual behavior is validated observationally
against the running app, consistent with prior GUI slices.

## Technical Context

**Language/Version**: Rust 1.96 (edition 2021)

**Primary Dependencies**: eframe/egui 0.35 (ComboBox, TextEdit, Grid).

**Storage**: none new. Skill delay overrides already persist through the existing
config path; this slice does not change persistence.

**Testing**: `cargo test` for the model Update intent and the unchanged strings
hygiene/coverage tests; the visual behavior (alignment, fixed widths, delay field
appearance) is validated manually per quickstart.md.

**Target Platform**: Windows 10/11 x64 and Linux x64 desktop.

**Project Type**: Single-crate desktop application.

**Performance Goals**: No new per-frame cost; the delay edit buffer is a single
small optional held in the GUI struct.

**Constraints**: Presentation-layer only; safety-critical surfaces untouched; text
hygiene (UTF-8 no BOM, LF, no em/en dashes) holds; the model stays the source of
truth for delay values, with the edit buffer active only while a field is focused.

**Scale/Scope**: One new intent, one new tooltip string, one header rename, one
shared combo helper, one delay-field helper, one weapon-bar grid row, one GUI
edit-buffer field.

## Constitution Check

- **I. Spec-Driven Development**: Full spec-kit sequence; traces to master spec
  section 10 (GUI). PASS.
- **II. Safety-Critical Surfaces**: Untouched. Update reuses the existing beacon
  uninstall (managed-marker gated) and install; the gate is not weakened, and the
  input engine is not involved. PASS.
- **III. Test-First With Explicit Seams**: The one correctness-bearing addition
  (the Update intent) is unit-tested at the model; the render layer stays thin and
  is validated observationally, as with prior GUI slices. PASS.
- **IV. CI Parity Before Every Commit**: fmt, clippy (-D warnings), test --all
  --locked run in the foreground before commit. PASS.
- **V. Bounded Scope**: No game memory, no packets, GUI only. PASS.

No violations. Complexity Tracking is empty.

## Project Structure

### Documentation (this feature)

```text
specs/022-primary-skills-controls/
|-- plan.md              # This file
|-- spec.md              # Feature specification
|-- research.md          # Phase 0 decisions
|-- data-model.md        # Phase 1 (intent + edit-buffer state)
|-- quickstart.md        # Phase 1 manual validation
|-- contracts/
|   `-- update-intent.md # Behavioral contract for the Update intent
|-- checklists/
|   |-- requirements.md  # Spec quality checklist
|   `-- ui-controls.md   # Requirements-quality checklist
`-- tasks.md             # Created next
```

### Source Code (repository root)

```text
src/
|-- app/
|   |-- mod.rs           # UiIntent::UpdateBeacon; apply_intent runs
|   |                    #   uninstall_beacon then install_beacon
|   |-- ui.rs            # Update button; weapon-bar grid row; shared fixed
|   |                    #   combo helper (weave + log filter); Delay (ms)
|   |                    #   cell as a right-aligned field for both states;
|   |                    #   per-row delay edit buffer on EsoWeaveApp
|   `-- strings.rs       # SKILL_COLUMNS "Delay" -> "Delay (ms)";
|                        #   BEACON_UPDATE_TOOLTIP added to all_tooltips
tests/
|-- app_view_model.rs    # Update intent reinstalls test
`-- app_strings.rs       # unchanged hygiene/coverage (still passes)
```

**Structure Decision**: Single crate retained; the change is confined to the app
(GUI) module plus one new intent in the view-model. No new module or dependency.

## Key Decisions (autopilot decision log)

- **Update = uninstall then install, reusing existing paths.** A new
  `UiIntent::UpdateBeacon` calls the model's existing `uninstall_beacon` then
  `install_beacon`. Rationale: Install already updates in place, but a dedicated
  Update makes the maintenance action explicit; a clean uninstall-then-install
  guarantees a fresh managed copy. The managed-marker uninstall gate is unchanged,
  so an unmanaged folder is never deleted (install still writes the managed addon
  over it). Enabled state reuses the Uninstall condition (`uninstall_enabled`: a
  folder is present to remove).
- **Delay field: egui `TextEdit` for both states, with a per-row edit buffer.**
  Both override-on and override-off render `egui::TextEdit::singleline` with a
  fixed `desired_width` (four-digit) and `horizontal_align(Align::RIGHT)`;
  override-off is wrapped in `add_enabled(false, ..)` so it is a greyed, read-only,
  same-width box showing the inherited value. A single
  `delay_edit: Option<(u8, String)>` on `EsoWeaveApp` holds the digits for the row
  being edited so the model value does not clobber in-progress typing; it is
  cleared on focus loss, filters to digits, and bounds to four characters.
  Rejected `DragValue`: it cannot right-align its internal text and its box does
  not match a greyed read-only field, so the two states would not look consistent.
- **Shared fixed-width combo helper.** A small `combo(id_salt, selected_text)`
  helper returns an `egui::ComboBox` preconfigured with a single shared
  `COMBO_WIDTH`, so the resting field never changes width with the selection.
  Applied to the weave selector and the live-log level filter now; slice 023
  applies the same helper to the settings dropdowns. Chosen over per-call
  `.width(..)` so all dropdowns share one width and one place to tune it.
- **Weapon Bar moves into the status grid.** The row is rendered as a fourth row
  of the existing `"status"` `egui::Grid` (title cell, then colorized state cell,
  no control cell), so its title and state align with the three rows above.

## Phasing

- Phase 0 (research.md): confirm the egui widgets (TextEdit alignment, disabled
  rendering, ComboBox width) and the reuse of the beacon paths.
- Phase 1 (data-model.md, contracts/, quickstart.md): the Update intent and the
  edit-buffer state, the intent contract, and the manual validation script.
- Phase 2 (tasks.md): test-first task list.

## Complexity Tracking

No constitution violations; no entries.
