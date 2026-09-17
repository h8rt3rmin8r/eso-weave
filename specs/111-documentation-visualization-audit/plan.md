# Implementation Plan: Documentation Visualization Audit

**Branch**: `codex/s111-documentation-visualization-audit` | **Date**: 2026-09-17 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/111-documentation-visualization-audit/spec.md`

## Summary

Audit every Markdown destination published by mdBook, record one reproducible page decision and a complete warrant for each serious candidate, consolidate overlaps, and create one atomic GitHub issue for every approved visualization. Preserve the audit as paired machine-readable and human-readable maintainer evidence, then enforce its completeness against `SUMMARY.md` through documentation policy.

## Technical Context

**Language/Version**: Markdown; JSON; Node.js 24 policy tests

**Primary Dependencies**: mdBook `SUMMARY.md`, existing content-coverage and figure manifests, Node built-in test runner, GitHub issues and project metadata

**Storage**: `docs/project/documentation-visualization-audit.json` plus an adjacent Markdown decision record

**Testing**: Node fixture tests, complete documentation policy, mdBook test/build, render smoke, trust policy, typo and text-integrity checks

**Target Platform**: Public GitHub Pages and identical bundled offline documentation

**Project Type**: Documentation governance slice with external GitHub issue handoff

**Performance Goals**: One bounded linear join over the published page and candidate inventories; no runtime or shipped application work

**Constraints**: Exactly 49 current pages, evidence before approval, no graphics implementation, local/offline accessibility, UTF-8 without BOM, LF, no forbidden dashes

**Scale/Scope**: 49 page decisions, a focused set of serious candidates, seven visual-form categories, atomic follow-up issues, and build-plan lifecycle records

## Constitution Check

*GATE: Passed before Phase 0 research and re-checked after Phase 1 design.*

| Principle | Result | Evidence |
| --- | --- | --- |
| I. Spec-driven development | PASS | Issue #170 and the complete S111 spec, clarification, checklists, plan, research, data model, contract, quickstart, tasks, and analysis precede audit implementation. |
| II. Safety-critical surfaces | PASS | S111 changes maintainer evidence and policy only. It adds no runtime, telemetry, input, addon, network, or filesystem authority. |
| III. Test-first with explicit seams | PASS | Fixture failures for incomplete page joins and candidate contracts precede the audit validator and manifest. |
| IV. CI parity before every commit | PASS | Documentation, render, trust, typo, encoding, and diff gates run before publication. Existing Rust gates remain available but application code is untouched. |
| V. Bounded desktop scope | PASS | Approved work is handed off as issues; no browser dependency, remote asset, or divergent documentation path is introduced. |
| Autopilot and publication | PASS | The user explicitly authorized push and official PR publication, all review processing, and at most one second Codex review for S111. |
| Pinned artifact rule | PASS | Existing visual and screenshot assets remain byte-identical. |

No constitutional exception or complexity justification is required.

## Project Structure

### Documentation for this feature

```text
specs/111-documentation-visualization-audit/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── checklists/
│   ├── audit-integrity.md
│   └── requirements.md
├── contracts/
│   └── audit-manifest.md
└── tasks.md
```

### Repository changes

```text
docs/project/documentation-visualization-audit.json
docs/project/documentation-visualization-audit.md
.github/scripts/docs-policy.mjs
.github/scripts/docs-policy.test.mjs
docs/project/build-plans/plan-047.md
docs/project/build-plans/README.md
docs/archive/build-plans/plan-046.md
docs/archive/build-plans/README.md
docs/project/migration-ledger.json
docs/project/migration-ledger.md
CHANGELOG.md
```

**Structure Decision**: Use JSON as the exact inventory and validation authority and Markdown as the reviewer-facing synthesis. This avoids forcing policy to parse a 49-row prose table while keeping decisions readable. Existing page content remains unchanged because issue #170 owns audit and handoff, not implementation.

## Delivery Phases

1. Archive Plan 046, establish Plan 047, and complete the full S111 spec-kit packet.
2. Add failing fixture tests for exact page coverage, candidate warrants, clusters, form consideration, and approved issue linkage.
3. Implement the bounded documentation-policy validator and integrate it into the repository gate.
4. Inventory all 49 published pages and evaluate serious candidates against the warrant.
5. Consolidate overlaps, create atomic GitHub issues for approvals, and finalize machine and human audit records.
6. Run post-implementation analysis and every applicable local validation gate.
7. Push, open the official PR, process all hosted feedback, invoke at most one second Codex review, and stop for operator merge.

## Complexity Tracking

No constitution violation is present.
