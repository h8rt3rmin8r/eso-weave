# Implementation Plan: Accessible Troubleshooting Decision Tree

**Branch**: `codex/s112-troubleshooting-decision-tree` | **Date**: 2026-09-17 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/112-troubleshooting-decision-tree/spec.md`

## Summary

Add the S111-approved troubleshooting decision tree as one hand-authored, repository-owned SVG beside the canonical shared diagnostic sequence. Extend the established source policy, generated-site policy, finite figure inventory, browser rendering matrix, and direct-SVG layout receipt from four to five diagrams. Preserve all existing troubleshooting prose and the common figure interaction.

## Technical Context

**Language/Version**: SVG 1.1-compatible markup, Markdown, JavaScript on Node.js 24

**Primary Dependencies**: Node standard library, mdBook 0.5.4, host-provided Chrome-compatible browser

**Storage**: Checked-in SVG, Markdown, JSON governance, and generated mdBook output

**Testing**: Node test runner, documentation source and generated policy, direct-SVG layout smoke, 40-cell diagram rendering smoke, shared figure browser evidence, mdBook test/build, linkcheck, spelling, text hygiene

**Target Platform**: GitHub Pages and bundled loopback documentation on Windows and Linux

**Project Type**: Single-crate desktop application with one shared mdBook source

**Performance Goals**: Reuse the existing one-process browser smoke and add only one bounded diagram observation per relevant matrix cell

**Constraints**: No prose authority removal, remote asset, package dependency, diagram generator, second viewer, application change, addon change, or safety bypass

**Scale/Scope**: One SVG, one page placement, two policy modules with tests, two maintained figure records, build-plan chronology, migration ledger, changelog, and the S112 spec packet

## Constitution Check

*GATE: Passed before research and re-checked after design.*

- **I. Spec-driven development**: PASS. Issue #220, the S111 audit, canonical troubleshooting prose, and the full S112 specify, clarify, checklist, plan, tasks, and analysis sequence form one authority chain.
- **II. Safety-critical surfaces**: PASS. S112 changes no runtime, addon, input, automation, or deletion behavior. Existing prose continues to prohibit bypassing focus, signal, suspension, binding, and ownership guards.
- **III. Test first**: PASS. Fixture and receipt mutations establish the missing fifth-diagram red state before production policy, SVG, and browser inventory changes.
- **IV. CI parity**: PASS. Documentation-only changes require the complete Node, mdBook, browser, spelling, hygiene, trust, and hosted gates. Cargo fmt, clippy, and test still run as repository confidence checks because policy and pinned workflow-adjacent behavior are touched only indirectly.
- **V. Bounded scope**: PASS. One offline documentation asset summarizes existing evidence and grants no new authority.
- **Accessibility and offline delivery**: PASS. Meaningful alternative, matching SVG title and description, visible branch words, non-color distinctions, complete prose equivalent, local bytes, no active content, and shared dialog behavior are mandatory.
- **Text hygiene**: PASS. All text remains UTF-8 without BOM, LF-only, and free of en or em dashes.
- **Autopilot and publication**: PASS. The operator explicitly authorized automatic push, official PR publication, all review handling, and at most one second Codex review for S112.

No complexity exception is required.

## Project Structure

```text
specs/112-troubleshooting-decision-tree/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── contracts/decision-tree.md
└── checklists/
    ├── requirements.md
    └── visual-accessibility.md

docs/src/getting-started/troubleshooting.md
docs/src/assets/diagrams/troubleshooting-decision-tree.svg
docs/project/documentation-figure-system.md
docs/project/diagram-rendering-compatibility.md
docs/project/content-coverage.json
docs/project/build-plans/plan-048.md
docs/archive/build-plans/plan-047.md
docs/project/migration-ledger.{json,md}
.github/scripts/docs-policy.mjs
.github/scripts/docs-policy.test.mjs
.github/scripts/docs-render-smoke.mjs
.github/scripts/docs-render-smoke.test.mjs
CHANGELOG.md
```

**Structure Decision**: Extend the existing static SVG registry and browser matrices. Do not create a second manifest or generator. The policy record is the exact machine-enforced asset contract, while the compatibility record names the human-maintained authority and update triggers.

## Phase 0: Research

1. Reconcile the five issue families with the canonical troubleshooting headings and implementation authorities.
2. Compare a five-way hub, flat table, and sequential decision chain against the existing 400-unit layout and S103 clearance contract.
3. Confirm that the current figure system and browser collector need inventory expansion only.
4. Define the source-policy, generated-policy, rendering, layout, inventory, and text-equivalent failure cases.

Output: [research.md](research.md)

## Phase 1: Design

1. Define the root, five decisions, five evidence endpoints, unmatched continuation, ten branch edges, and stable topology identifiers.
2. Define exact prose, source authorities, update triggers, alternative text, SVG title and description, and visible labels.
3. Define a 400 by 1520 top-down canvas with two-column terminal and continuation stages, dedicated orthogonal lanes, and no shared segments.
4. Define matrix changes from 32 to 40 rendering observations and four to five layout observations.

Outputs: [data-model.md](data-model.md), [contracts/decision-tree.md](contracts/decision-tree.md), [quickstart.md](quickstart.md)

## Phase 2: Test-First Policy Expansion

1. Add the fifth diagram to source-policy fixtures and assert missing placement, family labels, text anchors, external content, and geometry failures.
2. Change finite inventory expectations to 21 meaningful placements and five diagrams.
3. Expand rendering receipt fixtures to five diagram IDs and prove a four-diagram receipt fails with 40 expected observations.
4. Expand layout receipt fixtures to five IDs and prove a four-diagram receipt fails.
5. Run focused tests and record the expected red state before adding production records and the asset.

## Phase 3: Figure and Page Implementation

1. Add the accessible semantic SVG with exact node, edge, stage, branch, and edge-label metadata.
2. Place it inside the shared flow section and add an explicit text-equivalent subheading without changing the existing diagnostic sequence or symptom guidance.
3. Add the fifth record to source and browser inventories.
4. Reconcile geometry iteratively against the direct-SVG layout receipt until every route, label, and visible element passes.

## Phase 4: Governance and Evidence

1. Update the figure-system and rendering-compatibility records with the new counts, matrix, authority paths, source symbols, and update triggers.
2. Reconcile the existing DIA-006 content-coverage record with the implemented asset.
3. Archive completed Plan 047 with PR #224, establish Plan 048 for S112, and update both migration-ledger views chronologically.
4. Add the `[Unreleased]` changelog entry and dated decision for the maintained figure inventory expansion.

## Phase 5: Verification and Delivery

1. Run focused Node tests, complete documentation policy, mdBook test/build/linkcheck, and browser smoke.
2. Inspect the source SVG plus normal and expanded generated views in navy and light at wide, narrow, and 200 percent zoom conditions.
3. Run spelling, UTF-8, LF, mojibake, forbidden-dash, diff, trust, cargo fmt, clippy, test, and release build gates.
4. Mark S112 implemented, commit, push, open the official pull request closing #220, and move project tracking to PR review.
5. Process all CI, Codex, security, and reviewer feedback. Request and address no more than the one authorized second Codex round.
6. Stop for the operator's final review and merge ritual only after all conversations resolve and all required checks pass.
