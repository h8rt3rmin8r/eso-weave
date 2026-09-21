# Implementation Plan: BrandBuilder 2.0 Kit Adoption

**Branch**: `codex/s117-brand-kit-v2-adoption` | **Date**: 2026-09-21 | **Spec**: [spec.md](spec.md)

**Input**: GitHub issue #233, the S117 feature specification, and official package `eso-weave-brand-1.0.0-bb2.0.0`

## Summary

Replace the legacy visual mapping with the official semantic theme in the existing egui layer, add Geist Mono and semantic metadata usage, adopt the generated classic Win32 icon, retain exact BrandBuilder recovery bytes, publish a machine-readable adoption record, correct identity guidance, and enforce conformance with Rust and Node tests.

## Technical Context

**Language/Version**: Rust 2021 on pinned toolchain 1.96.0; Node.js for repository policy validation

**Primary Dependencies**: eframe/egui 0.36, sha2 already present, Node standard library

**Storage**: Versioned JSON adoption record and retained binary assets; no runtime persistence change

**Testing**: Rust unit and egui_kittest suites, Node test runner, mdBook and documentation policy checks, complete locked Cargo gate

**Target Platform**: Windows 10/11 x64 classic desktop and Linux x64 desktop/AppImage/Debian

**Project Type**: Offline-first cross-platform desktop application with bundled documentation and packaging

**Performance Goals**: No runtime network request or material startup regression; theme and font setup remain one-time initialization

**Constraints**: Exact-version kit pin; declared authority precedence; no identity redesign; no new shipped dependency; 44-point interactive target; WCAG AA; UTF-8 without BOM; LF-only text; no em/en dash; no merge

**Scale/Scope**: Central theme and representative metadata seams, one new font, one changed classic icon, adoption/recovery records, brand docs, validator/tests, changelog, and S117 packet

## Constitution Check

*GATE: Passed before Phase 0 research and must be re-checked after implementation.*

- **I. Spec-driven development**: PASS. Issue #233 is the sole tracker and the complete specify, clarify, checklist, plan, tasks, and analysis chain precedes product implementation.
- **II. Safety-critical surfaces**: PASS. Input synthesis, addon ownership, capture authority, persistence, and network behavior are unchanged. Their tests remain mandatory.
- **III. Test first**: PASS. Node integrity tests and Rust semantic/geometry tests are added and observed failing before production theme, font, asset, and record changes.
- **IV. CI parity**: PASS. Fmt, clippy, all locked tests, documentation checks, and locked release build run before publication.
- **V. Bounded scope**: PASS. Applicable kit surfaces are adopted without adding unrelated platform suites or a parallel UI framework.
- **Accessibility**: PASS. Exact contrast pairs, non-color status cues, focus width, and the 44-point target floor are enforced.
- **Text hygiene**: PASS. All touched text remains UTF-8 without BOM, LF-only, free of mojibake, and free of forbidden dash characters.
- **Pinned artifacts**: PASS. The classic ICO change receives a dated `[Unreleased]` decision and all retained bytes are hashed.
- **Project tracking**: PASS. S117 maps to issue #233 and does not replace the reserved S115 documentation work.
- **Publication authority**: PASS. The user explicitly authorized push and official PR creation. Autopilot proceeds through hosted checks and at most one second Codex review round, then stops before merge.

No complexity exception is required.

## Project Structure

```text
specs/117-brand-kit-v2-adoption/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── contracts/brand-kit-adoption-contract.md
└── checklists/
    ├── requirements.md
    └── brand-conformance.md

assets/brand/
├── brand-kit-adoption.json
├── recovery/shruggie-brandbuilder-2.0.0.skill
└── fonts/GeistMono-Regular.ttf

.github/scripts/
├── brand-kit-policy.mjs
└── brand-kit-policy.test.mjs

src/app/theme.rs
src/app/widgets.rs
src/app/ui.rs
tests/app_ui_sizing.rs
assets/icon.ico
assets/brand/README.md
docs/src/development/brand-standard.md
CHANGELOG.md
```

## Design Decisions

- **D1, extend the existing theme boundary**: Keep one application design system and translate kit roles into `src/app/theme.rs`. Do not add the generated adapter crate as a second styling owner.
- **D2, honor authority over generated defects**: Use light values from `brand.json` and Interface Canon where generated egui values conflict, and record each deviation in adoption metadata.
- **D3, semantic names**: Replace legacy gold-specific general UI fields with action, emphasis, on-action, surface, and focus roles. Keep product-domain meter colors clearly separated.
- **D4, accessible target floor**: Enforce 44-point response allocations globally, then repair affected layouts rather than preserving the legacy 22-point interaction size.
- **D5, bounded mono adoption**: Register Geist Mono centrally and apply it to existing helpers and technical metadata seams. Avoid a broad unrelated widget rewrite.
- **D6, selective artifact retention**: Retain recovery bytes and every consumed/applicable artifact. Record but do not vendor unrelated platform suites.
- **D7, dual-layer verification**: Node validates byte and metadata contracts; Rust validates runtime resolution, contrast, font setup, and rendered interaction geometry.

## Phases

### Phase 1: Red conformance tests

Add adoption-schema, hash, semantic token, font, contrast, and 44-point target tests. Capture expected failures against the current legacy theme and absent artifacts.

### Phase 2: Runtime and typography adoption

Implement semantic palette roles, governed state/spacing/radius/focus geometry, minimum targets, Geist Mono registration, and representative technical metadata usage. Repair any responsive regressions exposed by tests.

### Phase 3: Assets, recovery, and records

Add the adoption record, retained recovery distribution, mono font, and classic Win32 ICO. Verify already-current masters and Linux/installer/documentation reference assets. Record the pinned artifact decision.

### Phase 4: Documentation and complete verification

Rewrite the brand standard and asset README around the exact kit contract. Run focused suites, local CI parity, docs, release build, text hygiene, full diff review, and post-implementation analysis.

### Phase 5: Publication and review closure

Commit, push, open the official PR with `Closes #233`, attach it to this task, wait for CI and third-party reviews, address every actionable comment, request one `@Codex` second round if the first round produces review feedback, and stop for the operator's final review and merge ritual.
