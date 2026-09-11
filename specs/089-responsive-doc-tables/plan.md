# Implementation Plan: Responsive Documentation Tables

**Branch**: `codex/s089-responsive-doc-tables` | **Date**: 2026-09-11 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/089-responsive-doc-tables/spec.md`

## Summary

Audit all 54 published Markdown tables, progressively enhance mdBook's existing table wrappers only when their live geometry overflows, give each overflowing table a visible instruction plus a keyboard-focusable named region, preserve semantic table structure and script-free containment, and extend source policy plus the existing hidden-browser harness with a 20-cell responsive table matrix and trusted keyboard, resize, zoom, print, and blocked-script evidence.

## Technical Context

**Language/Version**: JavaScript on Node.js 24, browser JavaScript, Markdown, CSS

**Primary Dependencies**: Node standard library, mdBook 0.5.4, host-provided Chrome-compatible browser, native `ResizeObserver`

**Storage**: Checked-in Markdown, theme assets, policy code, spec packet, and generated mdBook output

**Testing**: Node test runner, documentation policy, hidden-browser smoke through the Chrome DevTools Protocol, mdBook build/test and linkcheck, spelling, text hygiene, Cargo repository parity

**Target Platform**: GitHub Pages and bundled loopback documentation on supported Windows and Linux desktop browsers

**Project Type**: Single-crate desktop application with one shared mdBook documentation source

**Performance Goals**: Enhance 54 tables once per page, batch geometry refreshes by animation frame, use one observer and one browser process, and keep the expanded documentation suite within the existing CI job

**Constraints**: No third-party package or remote resource; no semantic table replacement; no indiscriminate character breaking or reduced type; no page-level horizontal overflow; fitting tables add no focus stop; print and script-free output remain usable; no application behavior change

**Scale/Scope**: 54 tables across 26 pages, five named dense pages, two themes, two primary viewport widths, one shared theme script and stylesheet

## Constitution Check

*GATE: Passed before research and re-checked after design.*

- **Spec-driven sequence**: PASS. Issue #127 and Plan 039 define S089, and the complete specify, clarify, checklist, plan, tasks, and analyze sequence precedes implementation.
- **Safety-critical surfaces**: PASS. The slice changes only documentation theme, policy, browser evidence, canonical maintainer guidance, and planning records.
- **Test first**: PASS. Source inventory, runtime accessibility, CSS, generated semantics, browser receipt, keyboard, zoom, print, and blocked-script mutations will fail before implementation.
- **CI parity**: PASS. Documentation-only commits do not require Cargo by constitution, but autopilot's full foreground Cargo parity will still run before the initial commit.
- **Bounded dependencies**: PASS. Existing mdBook wrappers, checked-in theme resources, native browser observation, and ResizeObserver avoid new packages or downloads.
- **Offline parity**: PASS. Public and bundled documentation consume the same generated output and local theme assets.
- **Text hygiene**: PASS. New text uses UTF-8 without BOM, LF, and standard hyphens.
- **Issue lifecycle**: PASS. Issue #127 owns implementation. Epic #119 remains separate and closes only after its final gate is satisfied.

No complexity exception is required.

## Project Structure

```text
specs/089-responsive-doc-tables/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── contracts/responsive-tables.md
└── checklists/
    ├── requirements.md
    └── table-accessibility.md

docs/theme/eso-weave.css
docs/theme/eso-weave.js
docs/project/documentation-table-system.md
docs/project/build-plans/README.md
docs/project/build-plans/plan-039.md
.github/scripts/docs-policy.mjs
.github/scripts/docs-policy.test.mjs
.github/scripts/docs-render-smoke.mjs
.github/scripts/docs-render-smoke.test.mjs
CHANGELOG.md
```

**Structure Decision**: Preserve every Markdown table and mdBook's generated `.table-wrapper`. The local theme adds a sibling hint and only makes the existing wrapper a named, focusable region when measured overflow exists. Column-count and named-page classes set readable minimum widths for dense comparison tables, while compact two-column tables remain naturally sized. Policy owns the exact source inventory, and the existing browser process owns generated geometry and interaction evidence.

## Phase 0: Research

1. Inventory every Markdown table by page, source line, header signature, columns, rows, and longest source row.
2. Inspect mdBook's generated `.table-wrapper`, table semantics, base overflow behavior, theme order, print rules, and bundled site path.
3. Compare semantic table preservation with card conversion, content duplication, source HTML rewrites, and one shared progressive enhancement.
4. Define stable dense-table width tiers from column count and explicit named-page profiles without changing cell meaning.
5. Define overflow activation from measured geometry, unique naming from visible context, visible instruction relationships, and refresh behavior after fonts, resize, and zoom.

Output: [research.md](research.md)

## Phase 1: Design

1. Define the exact 54-table inventory and five named-page evidence contract.
2. Define idempotent table shell, region, hint, classification, and geometry-state models.
3. Define fitting-to-overflowing transitions, focusability, scroll position, and print or script-failure behavior.
4. Extend the rendering receipt with 20 named-page observations, compact-table evidence, trusted keyboard scrolling, resize, true page zoom, print, and blocked-script states.
5. Define mutation tests that reject inventory drift, lost semantics, redundant focus, missing cue or naming, page overflow, break-all, and screen-only print leakage.

Outputs: [data-model.md](data-model.md), [contracts/responsive-tables.md](contracts/responsive-tables.md), [quickstart.md](quickstart.md)

## Phase 2: Implementation

1. Add failing-first source inventory, generated table, JavaScript, CSS, and browser receipt tests.
2. Implement idempotent enhancement around existing mdBook wrappers with stable hints and live overflow state.
3. Add compact, dense, named-page, focus, cue, scrollbar, page containment, reduced-motion, and print presentation.
4. Add exact inventory and generated-site validation to documentation policy.
5. Add the 20-cell table matrix and special interaction states to the existing hidden-browser smoke.
6. Publish the maintainer table contract and update Plan 039, the active-plan index, and changelog chronology.

## Phase 3: Review and Delivery

1. Run local correctness, security, documentation, accessibility, regression, and scope review with findings filtered at 80 percent confidence.
2. Commit, push, and open one pull request that closes issue #127.
3. Address every first-round CI, Codex, security, and reviewer finding and resolve completed threads.
4. Request and address at most one authorized second Codex review round.
5. Stop at maintainer review after every conversation is resolved and every required check is green.

## Decision Log

- **2026-09-11, preserve semantic tables in labeled overflow regions**: Dense information is genuinely tabular. Converting it to cards or duplicated mobile markup would weaken comparison and accessibility relationships.
- **2026-09-11, enhance mdBook's existing wrapper**: Reusing the generated local containment boundary avoids 54 fragile source rewrites and preserves script-free overflow behavior.
- **2026-09-11, activate interaction from measured overflow**: Current geometry, rather than viewport width alone, determines whether focus, region naming, and instructions are necessary.
- **2026-09-11, use explicit width tiers and named-page profiles**: A small documented width policy prevents column collapse while retaining natural wrapping and avoiding arbitrary per-cell styling.
- **2026-09-11, adapt the checklist setup around a tooling defect**: The prerequisite script requires `plan.md` even though the constitution places checklist before plan. S089 used the resolved feature paths and checklist template directly to preserve the mandated order.
