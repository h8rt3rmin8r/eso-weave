# Implementation Plan: Brand Standard Visuals

**Branch**: `codex/s081-brand-standard-visuals` | **Date**: 2026-09-10 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/081-brand-standard-visuals/spec.md`

## Summary

Turn the Brand Standard into a self-contained visual reference. Publish byte-identical copies of the approved banner, badged mark, and glyph, demonstrate correct surface use, place accessible bounded chips beside all 25 palette tokens, and enforce the source and generated-output contract through documentation policy.

## Technical Context

**Language/Version**: Markdown, CSS, ECMAScript modules on the repository Node.js baseline
**Primary Dependencies**: mdBook, mdbook-linkcheck2, Node.js built-ins
**Storage**: Static repository files only
**Testing**: `node --test`, mdBook test/build/linkcheck, documentation policy, typos
**Target Platform**: GitHub Pages and bundled loopback-served offline documentation
**Project Type**: Documentation site inside a Rust desktop application repository
**Performance Goals**: Three local assets, no remote requests, no page-time scripting
**Constraints**: UTF-8 without BOM, LF, standard hyphens, 320 CSS pixel support, no application code
**Scale/Scope**: One page, two SVG copies, one existing PNG copy, scoped CSS, focused policy and governance updates

## Constitution Check

*GATE: Passed before research and re-checked after design.*

- **Spec-driven sequence**: PASS. Issue #122, Plan 039, spec, clarification record, checklists, plan, design artifacts, tasks, and analysis form the authority chain.
- **Safety-critical surfaces**: PASS. No application, input, capture, addon, or automation behavior changes.
- **Test first**: PASS. Focused policy failures precede validator and page implementation.
- **CI parity**: PASS. Documentation, text, and repository CI gates remain mandatory.
- **Bounded scope**: PASS. All content is local and shared by public and bundled output.
- **Text hygiene**: PASS. New prose uses UTF-8 without BOM, LF, and standard hyphens.

No complexity exception is required.

## Project Structure

### Documentation for this feature

```text
specs/081-brand-standard-visuals/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── contracts/
│   └── brand-standard.md
└── checklists/
    ├── requirements.md
    └── visual-accessibility.md
```

### Repository surfaces

```text
assets/eso-weave-banner.png                 # banner authority
assets/brand/eso-weave-mark.svg             # badged mark authority
assets/brand/eso-weave-glyph.svg            # badge-less glyph authority
docs/src/development/brand-standard.md       # visual standard
docs/src/assets/brand/                       # offline published copies
docs/theme/eso-weave.css                     # scoped gallery and swatch presentation
.github/scripts/docs-policy.mjs              # source/output contract
.github/scripts/docs-policy.test.mjs         # focused regressions
docs/project/build-plans/plan-039.md          # chronological slice status
docs/project/migration-ledger.json            # active spec evidence
CHANGELOG.md                                  # user-facing record and decisions
```

**Structure Decision**: Use raw semantic HTML only where Markdown cannot express labeled swatches and surface cards. Keep presentation in the existing stylesheet and validate authored constants instead of adding a preprocessor or JavaScript.

## Phase 0: Research

1. Confirm asset authorities, bytes, current copies, and legacy outputs.
2. Choose accessible swatch semantics that preserve visible table text.
3. Define surface examples, sizing, and clear-space rules from existing mark geometry.
4. Confirm local link paths and generated mdBook output.
5. Identify policy seams for source, bytes, CSS, and generated HTML.

Output: [research.md](research.md)

## Phase 1: Design

1. Define approved-asset, surface-example, and palette-token entities.
2. Specify exact markup, authority mappings, and validator failure classes.
3. Define responsive gallery and bounded chip styles.
4. Define local and CI verification commands.

Outputs: [data-model.md](data-model.md), [brand standard contract](contracts/brand-standard.md), and [quickstart.md](quickstart.md)

## Phase 2: Tasks and Analysis

Generate story-ordered tasks with tests before implementation. Analyze traceability, accessibility, asset identity, offline equivalence, and scope. Resolve every finding before implementation.

## Implementation Strategy

1. Add failing policy tests for asset identity, source markup, swatch completeness, CSS, and generated output.
2. Implement pure Brand Standard validators and connect repository files.
3. Copy authoritative mark and glyph bytes and retain the existing byte-identical banner.
4. Rebuild the page with approved asset cards, surface guidance, reproduction rules, and all 25 swatches.
5. Add scoped responsive styling and generated-output assertions.
6. Update Plan 039, migration evidence, and the changelog.
7. Run the quickstart and complete CI parity before publication.

## Complexity Tracking

No constitutional violation or additional subsystem is introduced.
