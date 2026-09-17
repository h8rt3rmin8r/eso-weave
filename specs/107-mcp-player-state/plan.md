# Implementation Plan: MCP Player-State Resources

**Branch**: `codex/s107-mcp-player-state` | **Date**: 2026-09-17 | **Spec**: `specs/107-mcp-player-state/spec.md`

**Input**: Feature specification from `specs/107-mcp-player-state/spec.md`

## Summary

Expose the S106 canonical capability and player-state documents as exactly two fixed, read-only MCP resources on the existing S105 stateless Streamable HTTP endpoint. A focused adapter will implement RMCP resource discovery and reads by cloning one immutable snapshot reference per request and applying the lifecycle-owned generation exactly as HTTP does. Official RMCP client integration tests will prove protocol interoperability, exact discovery metadata, HTTP parity, deterministic errors, concurrency, and bounded shutdown without introducing another cache or session model.

## Technical Context

**Language/Version**: Rust 1.91.1, edition 2021

**Primary Dependencies**: RMCP 3.4.0, Axum 0.8.9, Tokio 1.53.1, tokio-util 0.7.16, serde, serde_json

**Storage**: In-memory immutable canonical snapshots only; no new persistence

**Testing**: Rust unit and integration tests, including the official RMCP Streamable HTTP client

**Target Platform**: Windows 10 and 11 x64, Linux x64

**Project Type**: Single-crate desktop application with an embedded local HTTP and MCP service

**Performance Goals**: MCP reads clone one shared snapshot reference, hold no producer lock during serialization, and remain bounded by the existing service request and shutdown limits

**Constraints**: Loopback-only transport, bearer authentication, validated Host and Origin, stateless JSON Streamable HTTP, no writes, no remote binding, no new cache, no session persistence

**Scale/Scope**: Two fixed resources, one shared listener, one canonical publisher, and a small set of protocol and parity tests

## Constitution Check

*GATE: Passed before research and passed again after design.*

| Principle or constraint | Result | Evidence |
|---|---|---|
| I. Spec-driven development | PASS | Issue #178, Plan 045, and the complete S107 specify, clarify, checklist, plan, task, and analysis packet precede implementation. |
| II. Safety-critical surfaces | PASS | S107 is read-only and leaves input, addon, shared-data, capture, and fishing behavior unchanged. Full locked tests remain mandatory. |
| III. Test-first seams | PASS | Official RMCP client tests fail before the adapter is implemented. Snapshot publication and lifecycle seams are reused. |
| IV. CI parity before commit | PASS | Format, strict clippy, and full locked tests run before every Rust commit. |
| V. Bounded desktop scope | PASS | The feature exposes existing local state only. It adds no addon, capture, input, upload, action, or remote behavior. |
| Platform and configuration | PASS | No configuration or persistence changes. The existing cross-platform local-service runtime remains authoritative. |
| Text hygiene | PASS | New text uses UTF-8 without BOM, LF endings, and no forbidden dash characters. |
| Autopilot and publication | PASS | The user explicitly authorized push and PR publication for S107. Hosted CI and review remain gates before operator handoff. |

No constitutional exception or complexity waiver is required.

## Technical Approach

1. Add red integration tests that use RMCP's official Streamable HTTP client to initialize, discover, and read the endpoint.
2. Add a focused `mcp_state` module implementing `ServerHandler` resource metadata and reads over the existing `SnapshotPublisher`.
3. Replace the lifecycle-only handler factory with a cloneable adapter carrying the publisher and current service generation.
4. Mark the canonical capability document's MCP player-state flag true and verify HTTP and MCP parity against the same revision.
5. Exercise error, authentication, concurrent publication, client disconnect, restart generation, and shutdown boundaries.
6. Update the current build-plan and contract records while keeping database queries assigned to S108 and public integrator documentation assigned to S109.

## Project Structure

### Documentation

```text
specs/107-mcp-player-state/
├── analysis.md
├── checklists/
│   ├── mcp-player-state.md
│   └── requirements.md
├── contracts/
│   └── mcp-state-v1.md
├── data-model.md
├── plan.md
├── quickstart.md
├── research.md
├── spec.md
└── tasks.md
```

### Source and Tests

```text
src/
├── lib.rs
├── local_service.rs
├── mcp_state.rs
└── player_state.rs

tests/
├── local_service.rs
└── player_state.rs
```

**Structure Decision**: Keep the existing single crate. Isolate MCP protocol translation in `src/mcp_state.rs`; keep transport, security, and lifecycle ownership in `src/local_service.rs`; keep canonical serialization in `src/player_state.rs`; and extend integration coverage in the existing local-service test target.

## Complexity Tracking

No violations require justification.
