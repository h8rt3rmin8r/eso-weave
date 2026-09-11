# Visual and Accessibility Checklist: Documentation Flow Diagrams

**Purpose**: Define the design evidence required before S082 implementation.

**Created**: 2026-09-11

**Feature**: [spec.md](../spec.md)

## Purpose and Scope

- [x] Each selected diagram clarifies a named complex relationship.
- [x] Rejected candidates have a recorded prose-or-table rationale.
- [x] Decorative and duplicative graphics are excluded.
- [x] Every flow uses top-down progression.

## Accessible Meaning

- [x] Each SVG requires an internal title and description.
- [x] Each Markdown reference requires meaningful alternative text.
- [x] Each page requires a complete adjacent text equivalent.
- [x] Labels and outcomes never rely on color alone.

## Responsive and Offline Delivery

- [x] The asset canvas is bounded for narrow and wide layouts.
- [x] The SVG surface is legible against both documentation themes.
- [x] Script, animation, remote resources, and external fonts are prohibited.
- [x] Generated-output and local-link evidence is required.

## Text Integrity

- [x] UTF-8 without BOM and LF are required.
- [x] Mojibake and forbidden dash characters are rejected.
- [x] Plain terminology replaces unexplained abbreviations.
- [x] Adjacent prose stays canonical when images are unavailable.
