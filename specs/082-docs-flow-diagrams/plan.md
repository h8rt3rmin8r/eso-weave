# Implementation Plan: Documentation Flow Diagrams

**Branch**: `codex/s082-docs-flow-diagrams` | **Date**: 2026-09-11 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/082-docs-flow-diagrams/spec.md`

## Summary

Add four compact top-down SVG diagrams to the highest-value architecture, authorization, safety-recovery, and Pixel Bus validation flows. Keep every asset local, static, accessible, responsive, and paired with complete adjacent text while enforcing source and generated behavior through documentation policy.

## Technical Context

**Language/Version**: Markdown, SVG 1.1-compatible XML, CSS, ECMAScript modules on the repository Node.js baseline

**Primary Dependencies**: mdBook, mdbook-linkcheck2, Node.js built-ins

**Storage**: Static repository files only

**Testing**: `node --test`, mdBook test/build/linkcheck, documentation policy, text hygiene

**Target Platform**: GitHub Pages and bundled loopback-served offline documentation

**Project Type**: Documentation site inside a Rust desktop application repository

**Performance Goals**: Four small local vectors and zero page-time renderer or network requests

**Constraints**: UTF-8 without BOM, LF, 320 CSS pixel support, top-down flow, no application code or new dependency

**Scale/Scope**: Four pages, four SVGs, one scoped CSS component, focused policy and governance updates

## Constitution Check

*GATE: Passed before research and re-checked after design.*

- **Spec-driven sequence**: PASS. Issue #123, Plan 039, spec, clarification record, checklists, plan, design artifacts, tasks, and analysis form the authority chain.
- **Safety-critical surfaces**: PASS. The diagrams document safety behavior without changing application, addon, input, capture, or automation behavior.
- **Test first**: PASS. Focused failing policy mutations precede validator, assets, pages, and CSS implementation.
- **CI parity**: PASS. Documentation and text gates remain mandatory. Rust source is unchanged, so the constitution does not require the full Cargo merge gate.
- **Bounded scope**: PASS. Four static local assets serve public and bundled output with no renderer or network request.
- **Text hygiene**: PASS. New Markdown, SVG, CSS, and JavaScript use UTF-8 without BOM, LF, and standard hyphens.

No complexity exception is required.

## Project Structure

### Documentation for this feature

```text
specs/082-docs-flow-diagrams/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── contracts/
│   └── diagram-delivery.md
└── checklists/
    ├── requirements.md
    └── visual-accessibility.md
```

### Repository surfaces

```text
docs/src/assets/diagrams/                       # editable local SVG sources
docs/src/development/architecture.md            # ownership flow
docs/src/concepts/action-authorization.md       # authorization flow
docs/src/development/state-machines.md          # safety recovery flow
docs/src/reference/pixel-bus-protocol.md        # frame validation flow
docs/theme/eso-weave.css                        # scoped responsive frame
.github/scripts/docs-policy.mjs                 # source, SVG, and output contract
.github/scripts/docs-policy.test.mjs            # focused mutations
docs/project/build-plans/plan-039.md             # chronological slice status
docs/project/migration-ledger.json               # active spec evidence
CHANGELOG.md                                     # user-facing record and decision
```

**Structure Decision**: Treat each checked-in SVG as both editable source and delivered asset. Use ordinary Markdown image references and adjacent prose, relying only on mdBook's existing static-file copying.

## Phase 0: Research

1. Audit complex documentation candidates and select only relationships where a diagram improves comprehension.
2. Compare page-time Mermaid, build-time Mermaid, and direct SVG against offline, security, theme, and maintenance constraints.
3. Define accessible metadata, text-equivalent, responsive, and color-independent contracts.
4. Identify pure policy seams for source, SVG, CSS, and generated output.

Output: [research.md](research.md)

## Phase 1: Design

1. Define the four diagram records and exact page-to-asset mappings.
2. Define safe static SVG structure and prohibited content.
3. Define adjacent text-equivalent anchors and generated output requirements.
4. Define local and CI verification commands.

Outputs: [data-model.md](data-model.md), [diagram delivery contract](contracts/diagram-delivery.md), and [quickstart.md](quickstart.md)

## Phase 2: Tasks and Analysis

Generate story-ordered tasks with failing tests before implementation. Analyze traceability, asset safety, accessibility, responsive behavior, offline equivalence, and scope. Resolve every finding before implementation.

## Implementation Strategy

1. Add a four-record diagram manifest and failing policy tests.
2. Implement pure Markdown, SVG, generated-output, and responsive-CSS validators.
3. Author the four compact SVG source assets.
4. Place each diagram and complete text equivalent into its canonical page.
5. Add one scoped responsive wrapper and generated-output assertions.
6. Update Plan 039, migration evidence, and the changelog.
7. Run the quickstart and inspect the built pages before publication.

## Complexity Tracking

No constitutional violation or additional runtime subsystem is introduced.
