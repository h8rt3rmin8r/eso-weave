# Implementation Plan: BrandBuilder 2.0.1 Density Correction

**Branch**: `codex/s119-brandbuilder-2-0-1-density` | **Date**: 2026-09-22 | **Spec**: [spec.md](spec.md)

**Input**: GitHub issue #241, upstream ShruggieTech issue #239 and PR #240, and the immutable ESO Weave BrandBuilder 2.0.1 kit

## Summary

Replace S117's unconditional touch-sized egui defaults with BrandBuilder 2.0.1's corrected precise-pointer density, repin the kit and recovery provenance, extend the existing validator, and update the brand standard without changing product behavior or identity.

## Technical Context

**Language/Version**: Rust 2021 on pinned toolchain 1.96.0; Node.js for repository policy validation

**Primary Dependencies**: eframe/egui 0.36 and Node standard library, with no new shipped dependency

**Storage**: Versioned JSON adoption record and retained recovery artifact; no runtime persistence change

**Testing**: Rust unit and egui_kittest coverage, Node test runner, mdBook checks, complete locked Cargo gate

**Target Platform**: Windows 10/11 x64 and Linux x64 desktop with keyboard and precise pointer

**Project Type**: Offline-first cross-platform desktop application

**Performance Goals**: No startup, rendering, allocation, or network regression; theme setup remains one-time work

**Constraints**: Exact kit provenance; 28-point fine-pointer comfortable height; 44-point conservative input contract remains documented; no identity or resource-meter change; UTF-8 without BOM; LF-only text; no forbidden dash characters; no merge

**Scale/Scope**: One theme module, its focused tests, one adoption record, one recovery artifact, validator and tests, brand documentation, changelog, and S119 packet

## Constitution Check

*GATE: Passed before research and re-checked after design.*

- **I. Spec-driven development**: PASS. Issue #241 and this complete S119 packet precede product implementation.
- **II. Safety-critical surfaces**: PASS. Input synthesis, addon ownership, encounter capture, persistence, and network behavior are unchanged and their tests remain mandatory.
- **III. Test first**: PASS. Theme and validator regressions are added and observed failing before production values and metadata change.
- **IV. CI parity**: PASS. Fmt, clippy, all locked tests, documentation checks, and locked release build run before commit.
- **V. Bounded scope**: PASS. The existing theme remains the sole style owner and only the applicable native density correction is adopted.
- **Accessibility**: PASS. The upstream split preserves conservative 44-point targets for imprecise input while restoring precise-pointer density; text scaling may still grow controls.
- **Text hygiene**: PASS. Changed text is UTF-8 without BOM, LF-only, free of mojibake, and free of forbidden dash characters.
- **Pinned artifacts**: PASS. Recovery bytes and any changed pinned artifact receive a dated changelog decision.
- **Publication authority**: PASS. Work stops before the first remote push because this kickoff did not explicitly authorize publication.

No complexity exception is required.

## Project Structure

```text
specs/119-brandbuilder-2-0-1-density/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── evidence.md
├── contracts/native-density.md
└── checklists/
    ├── requirements.md
    └── brand-conformance.md

assets/brand/
├── brand-kit-adoption.json
└── recovery/shruggie-brandbuilder-2.0.1.skill

.github/scripts/
├── brand-kit-policy.mjs
└── brand-kit-policy.test.mjs

src/app/theme.rs
docs/src/development/brand-standard.md
CHANGELOG.md
```

**Structure Decision**: Extend the existing S117 ownership boundaries. Do not vendor the generated adapter as a second runtime theme or retain unrelated platform suites.

## Design Decisions

- **D1, treat the upstream correction as a regression fix**: S117's unconditional 44-point allocation is replaced because upstream formally identified it as a visible-target and conservative-target conflation.
- **D2, use the comfortable precise-pointer profile**: ESO Weave has no compact-density preference and is a conventional keyboard and mouse desktop application, so the governed comfortable profile is the correct default.
- **D3, keep one theme owner**: Translate the new generated metrics into `src/app/theme.rs` rather than importing a parallel generated crate.
- **D4, do not invent touch support**: Document the conservative profile but do not add platform capability detection that the application cannot validate within this slice.
- **D5, repin immutable provenance**: Compiler and adapter byte changes require a distinct 2.0.1 package and recovery identity. S119 will not mislabel changed bytes as 2.0.0.
- **D6, preserve product geometry**: Resource meters and intentionally larger local widgets remain product-owned and outside the global default correction.

## Phases

### Phase 1: Red regression evidence

Add assertions for BrandBuilder 2.0.1, egui adapter 1.0.1, 28-point fine-pointer controls, 8-by-2 item spacing, and 8-by-4 button padding. Record the expected failures against S117.

### Phase 2: Runtime density correction

Apply the governed comfortable precise-pointer metrics in the existing theme, preserve palette and widget-state geometry, and confirm product-owned meters are unchanged.

### Phase 3: Immutable kit repin

Adopt the exact 2.0.1 package and recovery identities, refresh consumed artifact hashes only where bytes changed, and update policy validation.

### Phase 4: Documentation and governance

Explain the precise-pointer versus conservative-target split, update recovery guidance, and record the dated provenance decision in the changelog.

### Phase 5: Verification and local delivery

Run focused regressions, full CI parity, documentation, encoding, forbidden-character, and diff audits; complete tasks and commit locally before the required pre-push halt.

## Post-Design Constitution Re-check

PASS. The design changes one presentation boundary, retains every safety gate, adds no dependency or input authority, preserves accessibility semantics, and requires exact immutable provenance before publication.
