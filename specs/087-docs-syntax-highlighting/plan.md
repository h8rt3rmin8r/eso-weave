# Implementation Plan: Documentation Syntax Highlighting

**Branch**: `codex/s087-docs-syntax-highlighting` | **Date**: 2026-09-11 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/087-docs-syntax-highlighting/spec.md`

## Summary

Replace inaccurate fence aliases with a complete audited classification, extend mdBook's bundled Highlight.js runtime with bounded command and PowerShell grammars, apply project-owned AA token colors, and expand the existing hidden-browser smoke to prove actual runtime tokenization, semantics, copy identity, and containment.

## Technical Context

**Language/Version**: JavaScript on Node.js 24, Markdown, CSS

**Primary Dependencies**: Node standard library, mdBook 0.5.4, bundled Highlight.js 10.1.1, host-provided Chrome-compatible browser

**Storage**: Checked-in Markdown, theme JavaScript and CSS; generated mdBook output; release-embedded documentation bytes

**Testing**: Node test runner, documentation policy, hidden-browser rendering smoke, mdBook build/test, linkcheck, spelling, text hygiene, Cargo repository gate

**Target Platform**: GitHub Pages and bundled loopback documentation on supported desktop platforms

**Project Type**: Single-crate desktop application with one shared mdBook documentation source

**Performance Goals**: Reuse one browser process and complete the bounded syntax matrix inside the existing documentation CI job

**Constraints**: No new runtime or package graph; no source prompts; all five mdBook themes; exact copy text; no page-level overflow; no application behavior change

**Scale/Scope**: 23 source fences, 12 metadata corrections, 2 local grammars, 5 theme classes, 4 runtime cases, 2 viewport widths, one existing policy and browser-smoke path

## Constitution Check

*GATE: Passed before research and re-checked after design.*

- **Spec-driven sequence**: PASS. Issue #157 and this S087 packet own the repository-backed behavior and evidence.
- **Safety-critical surfaces**: PASS. Changes are limited to documentation source, generated-output policy, theme assets, release manifest assertions, and evidence.
- **Test first**: PASS. Fence, generated-contract, runtime-receipt, and contrast mutations will demonstrate the intended red state before implementation.
- **CI parity**: PASS. The browser smoke reuses the generated site and host browser already required in documentation CI.
- **Bounded dependencies**: PASS. The existing bundled Highlight.js runtime is extended locally without npm, downloads, or a second runtime.
- **Offline parity**: PASS. Pages and release embedding consume the same mdBook output, with highlight assets added to the release manifest contract.
- **Text hygiene**: PASS. New text uses UTF-8 without BOM, LF, and standard hyphens.

No complexity exception is required.

## Project Structure

```text
specs/087-docs-syntax-highlighting/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── contracts/syntax-highlighting.md
└── checklists/
    ├── requirements.md
    └── syntax-highlighting.md

docs/src/**/*.md
docs/theme/eso-weave.css
docs/theme/eso-weave.js
docs/project/syntax-highlighting.md
.github/scripts/docs-policy.mjs
.github/scripts/docs-policy.test.mjs
.github/scripts/docs-render-smoke.mjs
.github/scripts/docs-render-smoke.test.mjs
tests/documentation.rs
```

**Structure Decision**: Keep Markdown identifiers as the source semantics, use one local post-`book.js` extension of the bundled highlighter, integrate deterministic inventory checks into documentation policy, and extend the existing browser process rather than duplicate browser infrastructure.

## Phase 0: Research

1. Inventory every source fence, including nested list blocks, and classify executable versus deliberately plain content.
2. Inspect mdBook's generated classes, runtime load order, registered grammars, theme assets, and release embedding.
3. Execute representative blocks through the bundled runtime and measure current theme contrast.
4. Compare metadata-only, second-runtime, template override, and bounded local-extension approaches.

Output: [research.md](research.md)

## Phase 1: Design

1. Define canonical fence, exact plain-exception, and generated-language contracts.
2. Define command and PowerShell token roles and an idempotent post-load registration boundary.
3. Define five-theme token palettes and AA checks.
4. Extend the existing rendering receipt with a 40-cell syntax matrix.
5. Preserve the issue #127 boundary for general responsive tables and overflow.

Outputs: [data-model.md](data-model.md), [contracts/syntax-highlighting.md](contracts/syntax-highlighting.md), [quickstart.md](quickstart.md)

## Phase 2: Implementation

1. Add failing-first source inventory, plain allowlist, generated-class, asset, contrast, observation, and receipt tests.
2. Correct 12 executable fences to `bash` while preserving 3 PowerShell and 8 exact plain blocks.
3. Register and apply bounded command and PowerShell grammars through the existing local theme script.
4. Add project-owned AA token colors for all supported themes.
5. Extend policy, hidden-browser smoke, release embedding assertions, canonical documentation, and chronology.

## Phase 3: Review and Delivery

1. Run independent code, security, documentation, accessibility, and scope review.
2. Commit, push, and open a pull request that closes issue #157.
3. Address every first-round CI, Codex, security, and reviewer finding.
4. Request and address at most one authorized second Codex review round.
5. Stop at maintainer review after every conversation is resolved and every required check is green.
