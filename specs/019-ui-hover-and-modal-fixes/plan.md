# Implementation Plan: GUI Hover Reflow and Settings Modal Fixes

**Branch**: `019-ui-hover-and-modal-fixes` | **Date**: 2026-07-12 | **Spec**: [spec.md](spec.md)

## Summary

Two targeted GUI fixes in the egui layer. First, remove the hover-driven window
reflow by making the size-affecting theme inputs identical across widget states.
Second, make the settings modal body fill its width and put its scrollbar at the
right edge by disabling the scroll area's horizontal auto-shrink.

## Technical Context

**Language/Version**: Rust 1.96, egui/eframe 0.35.

**Primary Dependencies**: existing only; no new dependencies.

**Testing**: `cargo fmt`/`clippy`/`test` stay green. The GUI layer carries no
unit-tested logic (per its module doc), so the two fixes are verified
observationally against the running app.

**Constraints**: Presentation only; no behavior, intent, or persisted-state change.

## Constitution Check

- **I. Spec-Driven Development**: PASS. Full spec-kit artifacts precede the change.
- **II. Safety-Critical Surfaces**: PASS. No safety surface is touched.
- **III. Test-First With Explicit Seams**: PASS with the documented exception that
  the egui layer is validated observationally, not by unit tests (its module doc
  already states this); no testable logic is added.
- **IV. CI Parity Before Every Commit**: PASS. fmt, clippy, test run green.
- **V. Bounded Scope**: PASS. Purely local UI presentation.

## Root cause and fix

### Hover reflow (`src/app/theme.rs::apply`)

In egui 0.35 a button's allocated size depends on its `WidgetVisuals` through the
interaction `expansion` and the `bg_stroke` width (which feeds the inner margin).
The theme sets `hovered.bg_stroke` width to `1.2` while every other state uses
`1.0`, and never sets `expansion`. The 0.2px per-side difference reflows the layout
on hover. Fix: set `expansion = 0.0` explicitly on all five `WidgetVisuals` states
(noninteractive, inactive, hovered, active, open) and set `hovered.bg_stroke` width
to `1.0` so the only size-affecting input is identical across states. Hover keeps
its gold border color; only the width changes.

### Settings modal width and scrollbar (`src/app/ui.rs`)

The settings `ScrollArea::vertical()` inherits egui's default `auto_shrink =
[true, true]`, so it shrinks to its content width and places the scrollbar at the
edge of that narrow content. Fix: add `.auto_shrink([false, false])`, matching the
log-panel scroll area that already renders full width. The body then fills the
modal's inner width (already set to `min(0.9*screen, 720)`) and the scrollbar sits
at the far right edge.

## Project Structure

```text
src/app/theme.rs   # expansion = 0.0 on all states; hovered stroke width 1.2 -> 1.0
src/app/ui.rs      # settings ScrollArea .auto_shrink([false, false])
```

## Complexity Tracking

No violations.
