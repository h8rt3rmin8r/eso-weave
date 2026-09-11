# Implementation Plan: Documentation Figure System

**Branch**: `codex/s088-docs-figure-system` | **Date**: 2026-09-11 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/088-docs-figure-system/spec.md`

## Summary

Unify all meaningful documentation screenshots, diagrams, illustrations, and brand examples behind one dependency-free native dialog, replace mdBook's incomplete checkbox modal at runtime, introduce a shared readable caption hierarchy, and extend repository policy plus the existing hidden-browser smoke to prove interaction, accessibility, geometry, print, and offline parity.

## Technical Context

**Language/Version**: JavaScript on Node.js 24, browser JavaScript, Markdown, CSS

**Primary Dependencies**: Node standard library, mdBook 0.5.4, native HTML `dialog`, host-provided Chrome-compatible browser

**Storage**: Checked-in Markdown and theme assets; generated mdBook output; release-embedded documentation bytes

**Testing**: Node test runner, documentation policy, hidden-browser rendering smoke through the Chrome DevTools Protocol, mdBook build/test, linkcheck, spelling, text hygiene, Cargo repository gate

**Target Platform**: GitHub Pages and bundled loopback documentation on supported Windows and Linux desktop browsers

**Project Type**: Single-crate desktop application with one shared mdBook documentation source

**Performance Goals**: Enhance a documentation page once without layout thrash, reuse one dialog and one browser process, and keep the complete rendering matrix inside the existing documentation CI job

**Constraints**: No third-party viewer or package graph; no network resource; no image-byte changes; exact alternatives and captions; native modal focus and inertness; print remains static; no application behavior change

**Scale/Scope**: 20 meaningful image placements, one decorative wordmark, 13 captions, five representative browser cases, two themes, two viewport widths, one shared theme script and stylesheet

## Constitution Check

*GATE: Passed before research and re-checked after design.*

- **Spec-driven sequence**: PASS. Issues #155 and #156 plus Plan 039 define a bounded combined S088 slice and this packet precedes implementation.
- **Safety-critical surfaces**: PASS. The work changes only documentation theme, policy, browser evidence, canonical documentation, and planning records.
- **Test first**: PASS. Figure inventory, theme contract, caption contract, browser receipt, and interaction mutations will fail before implementation.
- **CI parity**: PASS. The existing documentation gates and directly spawned hidden browser remain the evidence seam. No Rust source is planned, but full Cargo parity will still run before commit under the autopilot protocol.
- **Bounded dependencies**: PASS. A native dialog and existing local theme code avoid GLightbox, npm, remote resources, and browser downloads.
- **Offline parity**: PASS. Pages and release embedding consume the same generated mdBook output and checked-in assets.
- **Text hygiene**: PASS. New text uses UTF-8 without BOM, LF, and standard hyphens.
- **Issue boundary**: PASS. General responsive table and page overflow work remains with #127.

No complexity exception is required.

## Project Structure

```text
specs/088-docs-figure-system/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── contracts/figure-system.md
└── checklists/
    ├── requirements.md
    └── figure-accessibility.md

docs/theme/eso-weave.css
docs/theme/eso-weave.js
docs/project/documentation-figure-system.md
docs/project/build-plans/README.md
docs/project/build-plans/plan-039.md
.github/scripts/docs-policy.mjs
.github/scripts/docs-policy.test.mjs
.github/scripts/docs-render-smoke.mjs
.github/scripts/docs-render-smoke.test.mjs
CHANGELOG.md
```

**Structure Decision**: Keep authored figures valid and readable without JavaScript, enhance only recognized meaningful images at page load, convert mdBook's generated diagram wrapper into the same semantic trigger as raw HTML figures, and share one document-level native dialog. Extend the existing policy and hidden-browser process rather than creating a second test harness.

## Phase 0: Research

1. Inventory raw HTML images, Markdown images, figure classes, caption placements, intrinsic sizes, and decorative exceptions.
2. Inspect mdBook's generated checkbox modal, keyboard handler, CSS, script order, print page, and offline release path.
3. Compare retaining mdBook's checkbox, patching its nested label, adding a third-party viewer, and replacing the runtime interaction with one native dialog.
4. Confirm native dialog modal, focus, Escape, backdrop, sizing, and print behavior against the supported browser seam.

Output: [research.md](research.md)

## Phase 1: Design

1. Define the exact figure inventory and classification contract.
2. Define idempotent trigger conversion for raw HTML and generated Markdown figures.
3. Define one native dialog lifecycle, accessible naming, caption description, focus restoration, and intrinsic sizing contract.
4. Define the shared caption hierarchy and component-specific surface colors.
5. Extend the existing rendering receipt with a 20-cell figure matrix, trusted keyboard and pointer journeys, sequential focus, 200 percent scale, print, and blocked-script evidence.

Outputs: [data-model.md](data-model.md), [contracts/figure-system.md](contracts/figure-system.md), [quickstart.md](quickstart.md)

## Phase 2: Implementation

1. Add failing-first source inventory, runtime theme, caption CSS, print, observation, and receipt tests.
2. Add the idempotent local figure enhancer and one shared native dialog.
3. Replace generated diagram checkbox interaction at runtime without altering source diagrams or asset bytes.
4. Add the persistent trigger affordance, modal containment, intrinsic sizing, and shared caption hierarchy.
5. Extend documentation policy and the hidden-browser smoke for all specified cases and interactions.
6. Publish the figure-system contract and update S087/S088 chronology in Plan 039 and the current-plan index.

## Phase 3: Review and Delivery

1. Run code, security, documentation, accessibility, and scope review.
2. Commit, push, and open one pull request that closes issues #155 and #156.
3. Address every first-round CI, Codex, security, and reviewer finding.
4. Request and address at most one authorized second Codex review round.
5. Stop at maintainer review after every conversation is resolved and every required check is green.
