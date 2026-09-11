# Implementation Plan: Compact Work-Slice References

**Branch**: `codex/s085-docs-slice-references` | **Date**: 2026-09-11 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/085-docs-slice-references/spec.md`

## Summary

Normalize published work-slice provenance to `S###`, replace long test symbols with behavior-oriented evidence and repository links, document the exact exception for `s069-v1`, and enforce the contract in Markdown and generated HTML.

## Technical Context

**Language/Version**: JavaScript on Node.js 24, Markdown

**Primary Dependencies**: Node standard library, mdBook 0.5.2, linkcheck 0.7.7

**Storage**: Versioned published Markdown; existing unpublished JSON evidence manifest remains unchanged

**Testing**: Node test runner, documentation source and generated-site policy, mdBook build, linkcheck, spelling, text hygiene, Cargo merge gate

**Target Platform**: Identical public GitHub Pages and bundled offline documentation

**Performance Goals**: One linear pass over each published Markdown and generated HTML file

**Constraints**: No runtime changes, no test renames, no content-coverage contract edits, no broad table redesign

**Scale/Scope**: Three main developer pages, one conventions page, one incidental phrase, policy code, and focused tests

## Constitution Check

*GATE: Passed before research and re-checked after design.*

- **Spec-driven sequence**: PASS. Issue #126 and the S085 specification packet own the delivery chain.
- **Safety-critical surfaces**: PASS. The work changes documentation and validation only.
- **Test first**: PASS. Focused policy imports and fixtures will fail before implementation exists.
- **CI parity**: PASS. Documentation, spelling, hygiene, and full merge gates remain authoritative.
- **Bounded scope**: PASS. The general table redesign remains in issue #127.
- **Text hygiene**: PASS. New text uses UTF-8 without BOM, LF, and standard hyphens.

No complexity exception is required.

## Project Structure

```text
specs/085-docs-slice-references/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── contracts/work-slice-references.md
└── checklists/requirements.md

docs/src/development/coverage-matrix.md
docs/src/development/test-strategy.md
docs/src/development/state-machines.md
docs/src/development/repository-conventions.md
docs/src/development/brand-standard.md
.github/scripts/docs-policy.mjs
.github/scripts/docs-policy.test.mjs
```

**Structure Decision**: The existing published developer pages remain the reader authority. Exact test anchors remain in source and unpublished evidence data instead of a new duplicated index.

## Phase 0: Research

1. Inventory canonical, malformed, long, and legitimate non-slice identifiers.
2. Map every long published test symbol to a behavioral summary, relevant source file, and work slice.
3. Identify policy extension points for source and generated HTML.

Output: [research.md](research.md)

## Phase 1: Design

1. Define canonical visible syntax and narrow exclusions.
2. Define evidence navigation without duplicating exact symbols.
3. Define Markdown and HTML policy contracts.
4. Define focused red and green verification cases.

Outputs: [data-model.md](data-model.md), [contracts/work-slice-references.md](contracts/work-slice-references.md), [quickstart.md](quickstart.md)

## Phase 2: Implementation

1. Add failing-first tests for source, generated HTML, and repository integration.
2. Implement pure validators and integrate them into existing source and generated-site gates.
3. Normalize reader-facing documentation and add compact repository links.
4. Run focused and full validation.

## Phase 3: Review and Delivery

1. Complete independent local review and address findings.
2. Commit and push S085.
3. Open a closing pull request for issue #126 and move its project item to PR review.
4. Address every CI, Codex, and security finding.
5. Request at most one second Codex review round.
6. Stop at maintainer review after all threads resolve and checks pass.
