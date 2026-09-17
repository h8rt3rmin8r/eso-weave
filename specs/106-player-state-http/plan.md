# Implementation Plan: Canonical Player-State HTTP API

**Branch**: `codex/s106-player-state-http` | **Date**: 2026-09-17 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/106-player-state-http/spec.md`

## Summary

Add a dedicated canonical player-state module, immutable revision publisher, complete schema 1.0.0 projection, and authenticated HTTP capability and player-state routes. The application model captures authoritative raw state in one lock order and publishes semantic changes without using `AppView` or HUD retention. The existing lifecycle host clones one immutable revision per request and overlays its own service generation.

## Technical Context

**Language/Version**: Rust 1.96, JSON, Markdown

**Primary Dependencies**: Existing Serde, serde_json, time, Axum 0.8.9, Tokio 1.53.1, RMCP 3.4.0

**Storage**: No new persistence; current snapshot state is memory-only

**Testing**: Rust unit and integration tests, deterministic model fixtures, raw HTTP probes, CI parity

**Target Platform**: Windows 10/11 x64 and Linux x64

**Project Type**: Single-crate synchronous desktop application with an embedded async loopback service

**Performance Goals**: Constant-time snapshot reference acquisition; no serialization while holding UI, controller, input, or publisher locks

**Constraints**: Complete schema 1.0.0 inventory, immutable revision coherence, truthful absence/freshness, existing authentication and 64 KiB request guards, observation-only

**Scale/Scope**: One canonical publisher, two HTTP GET routes, six stable state domains, one maintained inventory gate

## Constitution Check

*GATE: Passed before research and re-checked after design.*

- **Spec-driven sequence**: PASS. Issue #177, S106, Plan 045, ADR 0002, and the S104/S105 contracts form one authority chain.
- **Safety boundary**: PASS. The API is loopback-only, authenticated, observation-only, and exposes no input, automation control, raw diagnostics, or filesystem paths.
- **Test-first discipline**: PASS. Publisher, inventory, projection, HTTP, loss, recovery, and concurrency tests precede production behavior.
- **Thread ownership**: PASS. Producers capture short value copies in the established worker lock order. Handlers clone an immutable `Arc` and serialize after every lock is released.
- **Configuration hygiene**: PASS. No runtime snapshot, revision, capture time, or source observation enters configuration.
- **Dependency governance**: PASS. Existing dependencies provide serialization, RFC 3339 formatting, and transport behavior; no new dependency is required.
- **Text hygiene**: PASS. New text is UTF-8 without BOM, LF, and contains no em dash or en dash.

No complexity exception is required.

## Project Structure

```text
specs/106-player-state-http/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── contracts/
│   └── http-state-v1.md
└── checklists/
    ├── requirements.md
    └── canonical-player-state.md

src/
├── player_state.rs
├── local_service.rs
├── app/mod.rs
├── main.rs
└── lib.rs

tests/
├── player_state.rs
├── local_service.rs
└── app_view_model.rs
```

**Structure Decision**: Add one reusable `player_state` module because the model and publisher are transport-independent and become the sole state authority for issue #178. Keep HTTP adapter code inside the existing lifecycle host.

## Key Decisions

1. **Canonical state is not `AppView`**: Project from decoded authorities and configuration. Display strings, colors, and retained HUD values remain non-public.
2. **Immutable revision handoff**: Store `Arc<PublishedSnapshot>` behind a short `RwLock`. Publication compares semantic content, increments one monotonic revision on change, and swaps the reference. Requests clone the reference and release the lock before serialization.
3. **One capture lock order**: Capture weave, fishing, and auto-potion under their established worker order, then read game and lock-free input facts. Process-transition mutation is aligned to the same order so the capture cannot mix pre-transition game facts with post-transition player facts.
4. **Typed envelope, explicit observation documents**: Use typed top-level snapshot, capability, revision, and observation metadata structures. Domain payloads use stable JSON object construction because many leaf value types differ, while tests enforce the complete path and shape inventory.
5. **Stable revisions**: Capture time, response age, and service generation do not independently advance semantic revision. Public values, knowledge, freshness, source, or protocol facts do.
6. **Lifecycle-owned generation**: The application publisher has no service lifecycle dependency. The HTTP adapter overlays the active generation into a response document.
7. **No premature parity claim**: Capabilities advertise the HTTP operations shipped in S106 and the future database identifiers, but no MCP player-state resource or query operation.

## Phase 0: Research

1. Map every S104 public and non-public inventory row to current authorities.
2. Identify lock orders and mutation paths that can affect coherence.
3. Confirm existing dependency support for RFC 3339 and response serialization.
4. Select the immutable handoff and revision comparison design.

Output: [research.md](research.md)

## Phase 1: Design

1. Define capability, observation, semantic domain, immutable revision, and response documents.
2. Define bootstrap, revision, capture, and service-generation behavior.
3. Define the complete application-model projection and source classification inventory.
4. Define HTTP methods, errors, cancellation, and route behavior.

Outputs: [data-model.md](data-model.md), [contracts/http-state-v1.md](contracts/http-state-v1.md), [quickstart.md](quickstart.md)

## Phase 2: Implementation

1. Add failing publisher, inventory, canonical projection, and HTTP contract tests.
2. Add the transport-independent player-state model and publisher.
3. Add authoritative application-model projection and publication.
4. Inject the publisher into the lifecycle host and implement both HTTP routes.
5. Update Plan 045 tracking, canonical summary, and changelog.

## Phase 3: Review and Delivery

1. Run the spec-kit analysis gate and resolve all findings.
2. Run focused player-state, lifecycle, model, and concurrency tests.
3. Run documentation, encoding, forbidden-dash, dependency, release build, and CI-parity gates.
4. Commit as `feat(106): add canonical player-state HTTP API`, push the authorized branch, and open an official PR closing issue #177.
5. Resolve every CI and review thread, request at most one authorized second Codex review, then stop for the operator merge ritual.
