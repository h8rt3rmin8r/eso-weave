# Implementation Plan: Guided Documentation Screenshots

**Branch**: `codex/s084-guided-documentation-screenshots` | **Date**: 2026-09-11 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/084-guided-documentation-screenshots/spec.md`

## Summary

Generate the S083 capture matrix into an ignored repository-local staging directory, curate all seven dark-wide application scenes into stable documentation assets, publish the supplied MSI Properties capture unchanged, and add one locally authored synthetic PixelBeacon overlay illustration. Embed the assets only where they answer a named reader question, add machine-readable provenance and a maintainer update checklist, and extend documentation policy to enforce the inventory and responsive presentation.

## Technical Context

**Language/Version**: Rust 1.96, JavaScript on Node.js 24, Markdown, CSS, JSON, SVG

**Primary Dependencies**: Existing S083 `documentation_capture` target, mdBook 0.5.2, linkcheck 0.7.7, Node standard library

**Storage**: Versioned local PNG and SVG assets plus `docs/project/documentation-screenshots.json`

**Testing**: S083 capture validation and generation, docs-policy unit and integration tests, mdBook build and linkcheck, Cargo merge gate

**Target Platform**: Identical public GitHub Pages and bundled offline documentation; capture generation on the maintained Windows environment

**Performance Goals**: Curated assets remain bounded and documentation builds retain current static-site behavior

**Constraints**: No desktop control, live ESO, personal data collection, lossy optimization, remote assets, production capture mode, or generated input

**Scale/Scope**: Nine assets across five reader-facing pages, one maintenance page, one provenance manifest, and policy coverage

## Constitution Check

*GATE: Passed before research and re-checked after design.*

- **Spec-driven sequence**: PASS. Issue #125, S084 spec, checklists, research, plan, tasks, and analysis own the delivery chain.
- **Safety-critical surfaces**: PASS. The work consumes test-only deterministic output and does not touch runtime input or addon lifecycle code.
- **Test first**: PASS. Policy fixtures will fail for absent or divergent assets before implementation is accepted.
- **CI parity**: PASS. Rust capture validation, documentation policy, mdBook, linkcheck, and the full Cargo merge gate remain authoritative.
- **Bounded scope**: PASS. Table redesign and slice-reference normalization stay in #127 and #126.
- **Text hygiene**: PASS. New text files use UTF-8 without BOM, LF, and standard hyphens.

No complexity exception is required.

## Project Structure

```text
specs/084-guided-documentation-screenshots/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── screenshot-plan.md
├── analysis.md
├── contracts/documentation-screenshots.md
└── checklists/requirements.md

docs/project/documentation-screenshots.json
docs/src/assets/screenshots/*.png
docs/src/assets/illustrations/pixelbeacon-overlay-example.svg
docs/src/development/screenshot-maintenance.md
docs/src/getting-started/installation.md
docs/src/getting-started/first-launch.md
docs/src/features/weaving.md
docs/src/features/auto-potion.md
docs/src/features/pixelbeacon.md
docs/theme/eso-weave.css
.github/scripts/docs-policy.mjs
.github/scripts/docs-policy.test.mjs
scripts/curate-documentation-captures.ps1
```

**Structure Decision**: Published binary assets live below the mdBook source tree, while canonical provenance stays in project documentation so it does not become an unexplained download in the generated site.

## Phase 0: Research

1. Audit all issue-targeted pages and every S083 scene.
2. Select one consistent application variant and define a minimal page-to-asset map.
3. Define supplied-image handling, synthetic-overlay labeling, local paths, and lossless byte identity.
4. Define policy enforcement using only Node standard-library byte parsing and hashing.

Output: [research.md](research.md)

## Phase 1: Design

1. Define the nine-entry screenshot plan and provenance schema.
2. Define image presentation and accessibility contracts.
3. Define capture, copy, verification, and future refresh workflow.
4. Define generated-site and source-tree checks.

Outputs: [data-model.md](data-model.md), [screenshot-plan.md](screenshot-plan.md), [contracts/documentation-screenshots.md](contracts/documentation-screenshots.md), [quickstart.md](quickstart.md)

## Phase 2: Implementation

1. Generate S083 output and verify the capture manifest.
2. Copy the seven selected deterministic PNG files and the supplied PNG; author the synthetic SVG.
3. Create the canonical provenance manifest from actual checked-in bytes.
4. Embed images, captions, and complete adjacent instructions on five pages.
5. Add responsive screenshot styling and the maintainer update guide.
6. Extend policy and tests, then run every repository gate.

## Phase 3: Review and Delivery

1. Commit and push the complete S084 branch.
2. Open a closing PR for #125 and move the project item to PR review.
3. Address every CI, Codex, and security finding.
4. Request at most one second Codex review round.
5. Stop at maintainer review after all threads resolve and checks pass.
