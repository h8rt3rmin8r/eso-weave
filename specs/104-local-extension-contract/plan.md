# Implementation Plan: Local Extension Stack and Contract

**Branch**: `codex/s104-local-extension-contract` | **Date**: 2026-09-15 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/104-local-extension-contract/spec.md`

## Summary

Select Axum 0.8.9 and the official RMCP 3.4.0 SDK for one authenticated loopback service, then record exact lifecycle, security, canonical state, database query, adapter parity, limits, and compatibility contracts. Deliver architecture and maintainer-reviewable inventories only; production implementation remains in issues #176 through #180.

## Technical Context

**Language/Version**: Rust 1.96 research probes, Markdown, JSON

**Primary Dependencies**: Selected for later implementation: Axum 0.8.9, RMCP 3.4.0, Tokio 1.53.1, Tokio Util 0.7.16, existing rusqlite 0.40.2

**Storage**: Checked-in contract records; future runtime reads existing catalog and encounter SQLite databases

**Testing**: Documentation policy and links, JSON validation, repository formatting, spelling, UTF-8, LF, mojibake, hosted CI

**Target Platform**: Existing Windows and Linux desktop application

**Project Type**: Single-crate synchronous desktop application with a future embedded local service

**Performance Goals**: No UI or PixelBus blocking; future queries bounded to 2 concurrent, 2 seconds, 1,000 rows, and 1 MiB

**Constraints**: Loopback only, opt-in, authenticated, no permissive CORS, bounded 3-second shutdown, no production dependency or runtime behavior in S104

**Scale/Scope**: One listener, two adapters, one state schema, two runtime databases, four shared operations, five dependent issues

## Constitution Check

*GATE: Passed before research and re-checked after design.*

- **Spec-driven sequence**: PASS. Issue #175, S104, active Plan 045, and the complete spec packet form the authority chain.
- **Safety and security boundary**: PASS. Loopback, bearer credentials, Host and Origin validation, non-secret discovery, query defense in depth, and bounded shutdown are explicit.
- **Test-first behavior**: PASS. S104 ships decisions, not runtime behavior. Inventory and document validation precede dependent implementation tests.
- **CI parity**: PASS. Repository-owned documentation and text gates run locally and in hosted CI.
- **Bounded dependencies**: PASS. The selected future stack uses required features only and exact versions. S104 changes no production dependency.
- **Architecture ownership**: PASS. One runtime owner and two thin adapters preserve a single state and query authority.
- **Text hygiene**: PASS. New text uses UTF-8 without BOM, LF, and standard hyphens.

No complexity exception is required.

## Project Structure

```text
specs/104-local-extension-contract/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── contracts/
│   ├── service-lifecycle.md
│   ├── player-state-v1.md
│   └── database-query-v1.md
└── checklists/
    ├── requirements.md
    └── local-extension-contract.md

docs/project/architecture-decisions/0002-local-extension-stack-and-contract.md
docs/project/local-extension-contract.md
docs/project/build-plans/plan-045.md
docs/archive/build-plans/plan-044.md
docs/project/migration-ledger.json
docs/project/migration-ledger.md
CHANGELOG.md
```

**Structure Decision**: Keep the accepted architecture and condensed implementer contract under maintainer documentation. Keep detailed design evidence in the S104 spec packet. Do not publish user-facing extension documentation before the service exists; issue #180 owns shipped user and integrator documentation.

## Phase 0: Research

1. Compare maintained HTTP and MCP components for license, maintenance, current transport, integration, shutdown, and dependency surface.
2. Build disposable release probes for the minimal HTTP host and selected MCP integration.
3. Audit every current game, PixelBus, automation, binding, configuration, catalog, and encounter state source.
4. Evaluate MCP transport security and SQLite read-only enforcement against primary specifications.

Output: [research.md](research.md)

## Phase 1: Design

1. Define one atomic lifecycle, runtime owner, listener, authentication, discovery, and shutdown contract.
2. Define canonical snapshot metadata and inventory every current public observation.
3. Define database inventory, typed query model, defense-in-depth rules, and exact bounds.
4. Map HTTP routes and MCP resources/tools to the same canonical services and error model.

Outputs: [data-model.md](data-model.md), [contracts/](contracts/), [quickstart.md](quickstart.md)

## Phase 2: Implementation

1. Record ADR 0002 and the condensed local extension contract.
2. Archive completed Plan 044 and establish Plan 045 for the extension epic sequence.
3. Reconcile project tracking and dependent issues #176 through #180 against the decision.
4. Record the dated architecture decision in the changelog.

## Phase 3: Review and Delivery

1. Run documentation policy, links, spelling, formatting, encoding, mojibake, and repository merge gates.
2. Commit, push, and open the official pull request closing issue #175.
3. Resolve every first-round CI, Codex, security, and reviewer finding.
4. Request and address at most one authorized second Codex review round.
5. Stop for maintainer review only after all conversations resolve and required checks pass.
