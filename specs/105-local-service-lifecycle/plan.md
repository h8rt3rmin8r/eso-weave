# Implementation Plan: Local Service Lifecycle

**Branch**: `codex/s105-local-service-lifecycle` | **Date**: 2026-09-16 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/105-local-service-lifecycle/spec.md`

## Summary

Implement one persisted, off-by-default local-service preference and one background lifecycle controller. The controller owns an authenticated Axum router and minimal stateless RMCP server on `127.0.0.1:18765`, atomically publishes non-secret discovery, reports truthful state to the settings UI, and performs owned bounded shutdown. Canonical HTTP operations, MCP resources, and queries remain in later Plan 045 slices.

## Technical Context

**Language/Version**: Rust 1.96, JSON, Markdown

**Primary Dependencies**: Axum `=0.8.9`, RMCP `=3.4.0`, Tokio `=1.53.1`, Tokio Util `=0.7.16`, getrandom 0.3 already present transitively but promoted for direct cryptographic use

**Storage**: Existing `config.json` plus atomic `local-extension.json` in the per-user configuration directory

**Testing**: Rust unit and integration tests, ephemeral loopback ports, raw HTTP request probes, headless egui settings tests, CI parity

**Target Platform**: Windows 10/11 x64 and Linux x64

**Project Type**: Single-crate synchronous desktop application with one embedded async local service

**Performance Goals**: No GUI-frame or input-hook blocking; status polling is lock-bounded; shutdown finishes within 3 seconds

**Constraints**: Loopback only, fixed production port, one listener, authentication on every route, 64 KiB body limit, no canonical application data in S105

**Scale/Scope**: One settings cluster, one controller, one runtime owner, one discovery file, one minimal MCP handler

## Constitution Check

*GATE: Passed before research and re-checked after design.*

- **Spec-driven sequence**: PASS. Issue #176, S105, Plan 045, ADR 0002, and the S104 service contract form one authority chain.
- **Safety boundary**: PASS. The service is observation-only, loopback-only, opt-in, authenticated, Host/Origin checked, and has no input or automation control path.
- **Test-first discipline**: PASS. Persistence and lifecycle integration tests precede production implementation.
- **Thread ownership**: PASS. Blocking lifecycle ownership stays off the UI and input-hook threads; one current-thread Tokio runtime owns async work.
- **Configuration hygiene**: PASS. Only requested enablement and persistent credential enter settings. Phase, endpoints, process, generation, and failures remain runtime or discovery state.
- **Dependency governance**: PASS. S104 selected exact reviewed versions; S105 adds only required server features and records the architecture-affecting addition.
- **Text hygiene**: PASS. New text is UTF-8 without BOM, LF, and uses no em dash or en dash.

No complexity exception is required.

## Project Structure

```text
specs/105-local-service-lifecycle/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── contracts/
│   └── lifecycle-v1.md
└── checklists/
    ├── requirements.md
    └── local-service-lifecycle.md

src/
├── local_service.rs
├── config/mod.rs
├── app/settings_form.rs
├── app/strings.rs
├── app/mod.rs
├── app/ui.rs
└── lib.rs

tests/
├── local_service.rs
├── config.rs
├── app_settings.rs
└── app_ui_sizing.rs
```

**Structure Decision**: Use one focused `local_service` module rather than a premature directory hierarchy. The lifecycle host is one bounded component today; later adapters may split it when they introduce distinct services.

## Key Decisions

1. **Minimal real MCP surface**: Mount RMCP now with initialization and empty advertised capabilities. This satisfies atomic transport lifecycle while preserving issue #178 ownership of resources and parity.
2. **Coordinator ownership**: A named background thread owns a Tokio current-thread runtime and an unbounded command channel. UI calls enqueue desired changes and poll a shared immutable status snapshot.
3. **Latest intent convergence**: Start, stop, and shutdown commands are drained at transition boundaries. A stop during start cancels before discovery publication; a start during stop begins only after the prior listener is released.
4. **Credential persistence and retrieval**: Store a lowercase hexadecimal 32-byte OS-random token inside the opaque `local_service` settings section. Preserve it on disable, compare bearer bytes without content-dependent early exit, and expose it only through an explicit clipboard copy action.
5. **Discovery ownership**: Write through a same-directory temporary file and atomic replacement. Cleanup parses the record and removes only an exact process and generation match.
6. **Failure recovery**: Failed state remains visible until disable or a new start request. Re-enable creates a new generation and performs a complete fresh start.
7. **No premature API placeholder**: Unknown authenticated `/api/v1` paths return a structured unavailable response. S105 does not pretend canonical capabilities exist before issue #177.

## Phase 0: Research

1. Confirm RMCP 3.4.0 stateless Streamable HTTP construction and empty handler behavior.
2. Map configuration, settings auto-save, UI modal, app exit, logging, and atomic-write seams.
3. Compare lifecycle command-loop options and select the single-owner model.
4. Define test injection for ephemeral ports, start faults, and deterministic time bounds.

Output: [research.md](research.md)

## Phase 1: Design

1. Define persisted preferences and transient lifecycle snapshots.
2. Define owner commands, transition rules, failure vocabulary, and latest-intent behavior.
3. Define authentication, Host, Origin, body-limit, MCP, and fallback router order.
4. Define discovery publication and compare-and-remove ownership.
5. Define settings rendering and exit integration.

Outputs: [data-model.md](data-model.md), [contracts/lifecycle-v1.md](contracts/lifecycle-v1.md), [quickstart.md](quickstart.md)

## Phase 2: Implementation

1. Add exact dependencies and the lifecycle module behind failing tests.
2. Add persisted preference and credential helpers.
3. Add controller, runtime owner, router security, MCP initialization, discovery, and cleanup.
4. Integrate settings form, view-model state, UI copy, and application shutdown.
5. Update Plan 045 tracking and the changelog decision.

## Phase 3: Review and Delivery

1. Run the spec-kit analysis gate and resolve all findings.
2. Run focused lifecycle, config, model, and headless UI tests.
3. Run repository documentation, link, spelling, encoding, mojibake, dependency, and CI-parity gates.
4. Commit as `feat(105): add local service lifecycle`, push the authorized branch, and open an official PR closing issue #176.
5. Resolve every CI and review thread, request at most one authorized second Codex review, then stop for the operator merge ritual.
