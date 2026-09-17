# Implementation Plan: Local Extension Documentation and End-to-End Verification

**Branch**: `codex/s109-extension-docs-e2e` | **Date**: 2026-09-17 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/109-extension-docs-e2e/spec.md`

## Summary

Complete Plan 045 with one canonical public guide for the shipped local HTTP and MCP extension, a machine-readable player-state field inventory checked against production, documentation examples and policy coverage, and final production-adapter parity and lifecycle evidence. Reuse the S105 through S108 controller, snapshot, query, raw HTTP, and official RMCP seams. Do not introduce another server, schema, fixture authority, or runtime dependency.

## Technical Context

**Language/Version**: Rust 1.96, edition 2021; Markdown; JSON; Node.js policy tests

**Primary Dependencies**: Existing Axum 0.8.9 server, RMCP 3.4.0 official client and server, Tokio 1.53.1, rusqlite 0.40.2, serde and serde_json, mdBook 0.5.4, mdbook-linkcheck2 0.13.0

**Storage**: Existing config and discovery files plus read-only catalog and encounter SQLite fixtures; one checked-in JSON documentation inventory

**Testing**: Rust integration tests, official RMCP client, raw loopback HTTP, Node.js documentation and trust policy tests, mdBook test/build/link validation

**Target Platform**: Windows 10 and 11 x64 and Linux x64, with tests using loopback and temporary directories

**Project Type**: Single-crate desktop application with embedded documentation

**Performance Goals**: Preserve the existing three-second lifecycle shutdown bound and database query limits; documentation inventory checks complete as ordinary integration tests

**Constraints**: Local-only authenticated read access, no real credentials or user paths in artifacts, no live ESO dependency, UTF-8 without BOM, LF, no forbidden dashes, and exact documentation policy accounting

**Scale/Scope**: One public guide, one 58-entry production-linked field inventory, focused documentation-contract coverage, final cross-transport parity and lifecycle coverage, and Plan 045 closure records

## Constitution Check

*GATE: Passed before Phase 0 research and re-checked after Phase 1 design.*

| Principle | Result | Evidence |
| --- | --- | --- |
| I. Spec-driven development | PASS | Issue #180, Plan 045, and the complete S109 specification, clarification, and checklist packet precede planning and implementation. |
| II. Safety-critical surfaces | PASS | S109 changes documentation and verification only. Existing input, addon, encounter, and automation safety tests remain mandatory. |
| III. Test-first with explicit seams | PASS | Documentation-inventory and example-contract assertions are written before artifacts. Production adapter coverage reuses deterministic snapshot, query, loopback HTTP, and official RMCP seams. |
| IV. CI parity before every commit | PASS | Any Rust test or source change requires format, strict Clippy, and the full locked test suite before commit. Documentation and policy gates also run. |
| V. Bounded desktop scope | PASS | The documented service stays opt-in, loopback-only, authenticated, observation-only, and read-only. No addon, input, telemetry, or remote capability changes. |
| Autopilot and publication | PASS | The user explicitly authorized push and official pull-request publication for S109. Hosted CI and no more than two Codex review rounds remain merge gates. |
| Pinned artifact rule | PASS | The documentation policy script changes only to register the new canonical page and examples. A dated rationale is recorded in `CHANGELOG.md`. |

No constitutional exception or complexity justification is required.

## Project Structure

### Documentation for this feature

```text
specs/109-extension-docs-e2e/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── checklists/
│   ├── extension-contract.md
│   └── requirements.md
├── contracts/
│   └── documented-local-extension-v1.md
└── tasks.md
```

### Repository changes

```text
docs/src/reference/
├── local-api-and-mcp.md
└── local-extension-state-fields.json

docs/src/SUMMARY.md
docs/src/reference/README.md
docs/src/reference/settings.md
docs/src/getting-started/troubleshooting.md
docs/src/getting-started/responsible-use.md
docs/src/development/architecture.md
docs/src/development/test-strategy.md
.github/scripts/docs-policy.mjs
.github/scripts/docs-policy.test.mjs
tests/local_extension_contract.rs
tests/local_service.rs
docs/project/build-plans/plan-045.md
docs/project/build-plans/README.md
docs/project/local-extension-contract.md
CHANGELOG.md
```

**Structure Decision**: Keep one canonical reference chapter and one adjacent machine-readable inventory in the existing mdBook. Add focused contract assertions in a new Rust integration test while extending existing production-adapter tests only where lifecycle or parity evidence is missing. Update the pinned documentation policy because its exact page, table, and code-block inventories intentionally reject unreviewed documentation growth.

## Delivery Phases

1. Write failing documentation-contract tests for the missing guide, exact field inventory, UI copy, operations, bounds, placeholders, and navigation.
2. Add the canonical reference guide and machine-readable inventory, then link existing public and maintainer pages to it.
3. Update documentation policy inventories and tests with a dated changelog decision.
4. Extend production-adapter integration evidence for full HTTP/MCP state and query parity plus lifecycle recovery and shutdown without duplicating existing harnesses.
5. Reconcile Plan 045 and canonical contract records, run analysis, and execute every local merge, documentation, release-build, trust, encoding, and corruption gate.
6. Push, publish the official pull request, process every hosted review comment, trigger at most one second Codex review, and stop for operator merge.

## Complexity Tracking

No constitution violation is present.
