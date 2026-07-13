# Implementation Plan: Settings Modal, Success Toast, Logging Linkage, and Key Presentation

**Branch**: `023-settings-modal-logging-keys` | **Date**: 2026-07-13 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/023-settings-modal-logging-keys/spec.md`

## Summary

Fix five settings-and-logging defects, mostly presentation. Add `Key::F2` to the
selectable key list (a functional gap) and show friendly key names; link the
live-log verbosity and the settings Log level so either updates the other and the
capture level; give the settings modal one shared width-and-height sizing rule
that grows sub-linearly with the window up to a cap; color the Settings saved
toast green; and route the settings dropdowns through the shared fixed-width combo
helper. The correctness-bearing pieces (the modal-sizing helper, the key
display-name mapping, and the log-level linkage in the view-model) are unit-tested;
the rest is validated observationally against the running app.

## Technical Context

**Language/Version**: Rust 1.96 (edition 2021)

**Primary Dependencies**: eframe/egui 0.35 (Modal, ComboBox, Area/Frame for the
toast).

**Storage**: no new persisted fields. The linkage routes the live-log dropdown
through the existing `logging.level` setting, which already persists.

**Testing**: `cargo test` for the modal-sizing helper, the key display-name
mapping, and the log-level linkage in the model; the visual behavior (modal
scaling, green toast, dropdown widths) is validated manually per quickstart.md.

**Target Platform**: Windows 10/11 x64 and Linux x64 desktop.

**Performance Goals**: No new per-frame cost; the modal-sizing helper is pure
arithmetic.

**Constraints**: Presentation-layer only; safety-critical surfaces untouched; text
hygiene holds; the key display-name mapping is display-only (stored and parsed key
values unchanged).

**Scale/Scope**: One key enum display method, one selectable-key array addition,
one pure sizing helper, one log-linkage change in the view-model, one toast style
change, and the combo-helper applied to four settings dropdowns.

## Constitution Check

- **I. Spec-Driven Development**: Full spec-kit sequence; traces to master spec
  section 10 (GUI) and section 11 (logging preferences). PASS.
- **II. Safety-Critical Surfaces**: Untouched. Input suppression, the hook thread,
  beacon uninstall, and fishing degradation are unchanged; the key change only
  extends the selectable list (the key already existed and round-trips). PASS.
- **III. Test-First With Explicit Seams**: The pure helpers (modal sizing, key
  names) and the log-level linkage are unit-tested first; the render layer stays
  thin and is validated observationally. PASS.
- **IV. CI Parity Before Every Commit**: fmt, clippy (-D warnings), test --all
  --locked in the foreground before commit. PASS.
- **V. Bounded Scope**: GUI and logging preferences only. PASS.

No violations. Complexity Tracking is empty.

## Project Structure

### Documentation (this feature)

```text
specs/023-settings-modal-logging-keys/
|-- plan.md              # This file
|-- spec.md              # Feature specification
|-- research.md          # Phase 0 decisions
|-- data-model.md        # Phase 1 (linkage + helpers, no persisted data)
|-- quickstart.md        # Phase 1 manual validation
|-- contracts/
|   `-- helpers.md       # Pure-helper and linkage behavioral contract
|-- checklists/
|   |-- requirements.md  # Spec quality checklist
|   `-- ui-settings.md   # Requirements-quality checklist
`-- tasks.md             # Created next
```

### Source Code (repository root)

```text
src/
|-- input/key.rs         # Key::display_name friendly names + unit tests
|-- app/
|   |-- mod.rs           # modal_extent pure helper (+ tests); SetLogFilter
|   |                    #   now sets logging.level + capture + persist;
|   |                    #   reload_from_settings syncs log_filter
|   |-- ui.rs            # KEYS gains F2; keybinding + settings dropdowns via
|   |                    #   combo(); key display names; modal width/height via
|   |                    #   modal_extent
|   `-- widgets.rs       # Toast rendered green (success), theme-aware, legible
tests/
`-- app_view_model.rs    # modal_extent tests; log-linkage test
```

**Structure Decision**: Single crate retained; changes are confined to the app and
input modules, no new module or dependency.

## Key Decisions (autopilot decision log)

- **Log linkage: the live-log dropdown drives the real log level.** `SetLogFilter`
  now sets `settings.logging.level`, calls `log.set_level`, marks the config store
  dirty (so it persists), and keeps `log_filter` mirrored; `reload_from_settings`
  sets `log_filter = settings.logging.level` so applying a settings Log level
  updates the panel dropdown. The display filter threshold equals the capture
  level, so the panel simply shows everything captured. Panel visibility
  (`ToggleLogPanel`) is untouched, so hiding the panel never changes verbosity.
- **Modal sizing: one pure sub-linear helper for both axes.** `modal_extent(window,
  min_px, max_px, max_frac)` in the view-model (next to `clamp_log_height`) returns
  `min_px` at the smallest window and grows at a fraction of window growth, clamped
  to `[min_px, max_px]` and never exceeding `max_frac` of the window. Applied to
  both width and height each frame from `ctx.content_rect()`, so the modal tracks
  the window, grows in pixels but shrinks in fraction, and is bounded. Width and
  height use the same rule with their own caps.
- **Green toast without a contrast regression.** The Toast fills with the brand
  success (ok) color and draws its text in a contrasting color (the base surface
  color) at a heavier weight, so it is clearly green and legible in both themes
  rather than relying on a mid-green text on a dark panel.
- **Friendly key names as a display-only mapping.** `Key::display_name` returns
  Number 1 through Number 5, E, R, X, Q, Space, F1, F2; the canonical `as_str`
  (storage) and `parse` are unchanged. The GUI uses `display_name` for the selected
  text and options; `KEYS` gains `Key::F2`.
- **Reuse the shared combo helper.** The `combo(..)` helper from slice 022 is
  applied to the theme, beacon environment, log level, and keybinding dropdowns so
  they stop shifting on selection.

## Phasing

- Phase 0 (research.md): confirm the egui Modal sizing surface, the toast frame,
  and the existing log state split.
- Phase 1 (data-model.md, contracts/, quickstart.md): the linkage, the helper
  contracts, and the manual validation script.
- Phase 2 (tasks.md): test-first task list.

## Complexity Tracking

No constitution violations; no entries.
