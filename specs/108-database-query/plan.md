# Implementation Plan: Read-Only Database Queries

**Branch**: `codex/s108-database-query` | **Date**: 2026-09-17 | **Spec**: `specs/108-database-query/spec.md`

**Input**: Feature specification from `specs/108-database-query/spec.md`

## Summary

Add one cloneable database query service that owns the fixed catalog and encounter path registry, a global two-permit admission gate, request validation, defended short-lived SQLite connections, typed materialization, and canonical errors. Install thin HTTP inventory/query adapters and extend the existing MCP handler with one database resource and one query tool. Synchronous SQLite execution runs on Tokio blocking workers, while generation cancellation reaches the SQLite progress handler. Tests prove transport parity, data integrity, exact bounds, catalog replacement, and bounded shutdown.

## Technical Context

**Language/Version**: Rust 1.96, edition 2021

**Primary Dependencies**: Rusqlite 0.40.2 with bundled SQLite plus `hooks` and `limits`, Base64 0.22, RMCP 3.4.0, Axum 0.8.9, Tokio 1.53.1, tokio-util 0.7.16, serde, serde_json

**Storage**: Read-only short-lived connections to the current catalog and optional encounter SQLite databases

**Testing**: Rust unit and integration tests with temporary SQLite fixtures and the official RMCP Streamable HTTP client

**Target Platform**: Windows 10 and 11 x64, Linux x64

**Project Type**: Single-crate desktop application with an embedded local HTTP and MCP service

**Performance Goals**: At most two external queries execute concurrently; runtime work leaves the current-thread service executor; results release SQLite resources before client serialization

**Constraints**: Loopback-only authenticated transport, no writes, no arbitrary paths, 2-second execution, 1,000 rows, 1 MiB result, 16 KiB SQL, 64 parameters, 64 KiB request

**Scale/Scope**: Two fixed databases, one inventory resource, one HTTP query route, one MCP query tool, and one shared executor

## Constitution Check

*GATE: Passed before research and passed again after design.*

| Principle or constraint | Result | Evidence |
| --- | --- | --- |
| I. Spec-driven development | PASS | Issue #179, Plan 045, and the complete S108 specify, clarify, checklist, plan, task, and analysis packet precede implementation. |
| II. Safety-critical surfaces | PASS | S108 is read-only and leaves input, addon, shared-data, encounter capture, and fishing behavior unchanged. Full locked tests remain mandatory. |
| III. Test-first seams | PASS | Shared-service tests and real-adapter parity tests fail before implementation. Temporary path registries isolate SQLite fixtures. |
| IV. CI parity before commit | PASS | Format, strict clippy, and full locked tests run before every Rust commit. |
| V. Bounded desktop scope | PASS | The feature queries existing local application databases only. It adds no addon, input, upload, remote, or action behavior. |
| Platform and configuration | PASS | No configuration schema changes. The query service uses portable rusqlite APIs and the existing cross-platform lifecycle. |
| Text hygiene | PASS | New text uses UTF-8 without BOM, LF endings, and no forbidden dash characters. |
| Autopilot and publication | PASS | The user explicitly authorized push and PR publication for S108. Hosted CI and review remain gates before operator handoff. |

No constitutional exception or complexity waiver is required.

## Technical Approach

1. Enable only rusqlite `hooks` and `limits`, plus a small base64 encoder dependency. Do not enable load-extension support.
2. Add `database_query` models and a cloneable service with a private dynamic path registry and shared two-permit semaphore.
3. Validate the complete request before opening SQLite. Reject excess concurrency before spawning blocking work.
4. Open a new defended read-only connection per operation. Inventory uses bounded internal metadata queries; external SQL adds fail-closed authorizer, statement-readonly, parameter-shape, progress, and result limits.
5. Materialize a complete bounded result on the blocking worker. Return owned data only after statement and connection destruction.
6. Extend the local controller construction with the shared query service and generation cancellation token.
7. Add HTTP inventory and query handlers, then add the MCP database resource and `query_database` tool over the same service.
8. Update dynamic catalog paths on accepted catalog replacement without exposing them externally.
9. Update capability metadata and tracking while keeping public documentation and final end-to-end setup coverage assigned to S109.

## Project Structure

### Documentation

```text
specs/108-database-query/
|-- analysis.md
|-- checklists/
|   |-- database-query.md
|   `-- requirements.md
|-- contracts/
|   `-- database-query-v1.md
|-- data-model.md
|-- plan.md
|-- quickstart.md
|-- research.md
|-- spec.md
`-- tasks.md
```

### Source and Tests

```text
src/
|-- app/mod.rs
|-- app/ui.rs
|-- database_query.rs
|-- lib.rs
|-- local_service.rs
|-- main.rs
|-- mcp_state.rs
`-- player_state.rs

tests/
|-- database_query.rs
|-- local_service.rs
`-- player_state.rs
```

**Structure Decision**: Keep the single crate. Isolate SQLite authority and transport-neutral models in `database_query.rs`; keep HTTP security and routing in `local_service.rs`; keep MCP framing in `mcp_state.rs`; and let the application model update only the private current catalog path.

## Complexity Tracking

No violations require justification.
