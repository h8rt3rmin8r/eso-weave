# Implementation Plan: Persistent Auto Potion Request

**Branch**: `codex/s097-auto-potion-persistence` | **Date**: 2026-09-15 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/097-auto-potion-persistence/spec.md`

## Summary

Persist the operator-owned Auto Potion request in version 4 of `state.json`,
restore it through the existing fail-closed controller, make UI and F3 changes
use the existing coalesced session save authority, and remove obsolete
session-only claims from canonical documentation. No input, trigger, telemetry,
configuration, protocol, or addon behavior changes.

## Technical Context

**Language/Version**: Rust 1.96, edition 2021; Markdown and JSON documentation

**Primary Dependencies**: serde, serde_json, existing application model and Auto Potion controller

**Storage**: Local pretty-printed `state.json`, current schema version 4

**Testing**: Rust unit and integration tests through `cargo test --all --locked`; documentation policy, mdBook, render-smoke, spelling, encoding, and diff gates

**Target Platform**: Windows 10/11 x64 and Linux x64

**Project Type**: Single-crate desktop application

**Performance Goals**: No additional polling, worker, input, or startup I/O; one additive Boolean in the existing state write

**Constraints**: Missing and legacy data default off; malformed documents fail as a whole; restore never ticks or relaxes controller gates; UI and F3 converge on one request

**Scale/Scope**: One session-state field, three model seams, focused tests, canonical documentation, changelog, and build-plan rollover

## Constitution Check

*GATE: Passed before research and re-checked after design.*

- **Principle I, Spec-Driven Development**: PASS. Issue #172, this complete
  spec-kit package, Plan 043, and canonical documentation form the authority
  chain. No implementation begins before tasks and analysis.
- **Principle II, Safety-Critical Surfaces**: PASS. The controller remains gated
  and dormant by default, restoration does not tick, and the complete existing
  input, addon, fishing, and encounter safety suites remain mandatory.
- **Principle III, Test-First**: PASS. State migration, model restore, toggle
  persistence, and startup safety tests precede implementation.
- **Principle IV, CI Parity**: PASS. Full fmt, strict Clippy, and locked tests run
  before each source commit.
- **Principle V, Bounded Scope**: PASS. No addon, Pixel Bus, game API, transport,
  telemetry, upload, or gameplay-action authority changes.
- **Configuration and text hygiene**: PASS. The request stays out of
  `config.json`, `state.json` remains UTF-8/LF, and all changed prose avoids
  forbidden dash characters.

## Design

### State schema

Add `auto_potion: bool` to `SessionState` with `#[serde(default)]`, advance
`CURRENT_STATE_VERSION` from 3 to 4, and set the explicit default to `false`.
Serde's strict Boolean deserialization preserves the current whole-document
invalid fallback for strings, numbers, arrays, objects, and null.

### Model restoration and persistence

`restore_session` applies both Boolean values through
`AutoPotionController::set_enabled` after restoring the other session facts. It
does not touch the scheduler or call `tick`. `current_session_state` reads the
controller's authoritative request alongside suspension and Fishing.

`UiIntent::SetAutoPotion` continues to call `set_enabled` and additionally marks
the existing session save scheduler. The F3 path already maps to that same
intent, so no new routing authority is needed. Normal close already calls
`flush_session_now`, which will serialize the latest request after
`current_session_state` is extended.

### Documentation and planning

Archive completed Plan 042 and open Plan 043 for the post-data-addon P1 backlog,
with S097 as the current focused slice. Update direct canonical claims in Auto
Potion, settings, configuration, architecture, and relevant startup or
troubleshooting prose. Record the explicit reversal of S039 R7 and S043 FR-002
in the changelog without rewriting historical specs.

## Project Structure

### Documentation (this feature)

```text
specs/097-auto-potion-persistence/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── issue-172.md
├── contracts/
│   ├── session-state-v4.md
│   └── startup-restore.md
├── checklists/
│   ├── requirements.md
│   └── safety-and-persistence.md
└── tasks.md
```

### Source and repository integration

```text
src/config/state.rs
src/config/mod.rs
src/app/mod.rs
src/main.rs
tests/app_session_state.rs
docs/src/features/auto-potion.md
docs/src/reference/configuration.md
docs/src/reference/settings.md
docs/src/development/architecture.md
docs/src/getting-started/
docs/project/build-plans/
docs/archive/build-plans/
CHANGELOG.md
CLAUDE.md
.specify/feature.json
```

**Structure Decision**: Extend the existing single-crate session-state and
application-model seams. A new module, store, worker, service, or controller
would duplicate established authority and is rejected.

## Verification Strategy

1. Add state-model tests for schema 4, true and false round trips, every legacy
   version default, and malformed request fallback.
2. Add application tests for enabled and disabled restore, dormant startup,
   UI and F3 persistence convergence, settled saves, and immediate close saves.
3. Run focused state and model tests during red-green-refactor.
4. Run full CI parity, release build, documentation gates, plan/spec checks,
   text encoding and forbidden-dash scans, and `git diff --check`.
5. Review the complete diff for hidden input authority, partial state
   application, documentation contradictions, and schema drift.

## Complexity Tracking

No constitution violations or complexity exceptions are required.
