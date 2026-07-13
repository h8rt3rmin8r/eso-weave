# Build Plan 002: Brand and UX Polish

Plan: 002
Status: active
Master specification: `docs/ESO-Weave-Specification-v0.2.0.md`
Constitution: `.specify/memory/constitution.md`

## Purpose

Build plan 001 decomposed the master specification into ten functional slices,
from foundations through packaging and the installer. This plan adds a polish
slice that raises the product's visual quality to a professional bar after the
first end-to-end build shipped (v0.2.0).

It traces to the master specification's GUI layer (theme, colorized log,
dark-default/light), its application-icon requirement, and the installer slice
(011). It does not add runtime behavior or touch any safety-critical surface.

## Slices

### Slice 012: Brand and UX Polish

Scope: establish an "Arcane gold on ink" brand standard (documented in
`docs/brand/ESO-Weave-Brand-v1.md`) with a new woven-caret mark, and apply it
across the application UI (custom egui theme for dark and light, bundled Inter,
aligned skill columns, pointer cursors, on-palette status and log colors), the
runtime and executable icon (window icon plus a `build.rs` exe-icon embed), and
the Windows and Linux installers (proportional license page, branded wizard
artwork, opt-in Custom Setup desktop shortcut). Reworks every asset in `assets/`
and adds a GitHub social-share image. Feature under
`specs/012-brand-ux-polish/`.
