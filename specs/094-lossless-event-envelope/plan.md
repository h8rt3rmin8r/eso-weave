# Implementation Plan: Lossless Subscribed-Event Envelope

**Branch**: `codex/s094-lossless-event-envelope` | **Date**: 2026-09-15 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/094-lossless-event-envelope/spec.md`

## Summary

Introduce capture schema v2 with an ordered tagged raw source stream, retain
normalization-dependent API observations, declare all loss, preserve schema-v1
SavedVariables and store records, and keep the existing normalized metric and
recommendation surface stable. This is the first bounded tranche of issue #186.

## Technical Context

**Language/Version**: Rust 2021 on pinned stable toolchain; Lua 5.1 addon runtime

**Primary Dependencies**: serde, serde_json, rusqlite, sha2, mlua test harness

**Storage**: ESO SavedVariables input and user-owned SQLite encounter store

**Testing**: cargo test, Lua 5.1 integration harness, docs-policy Node tests

**Target Platform**: Windows 10/11 x64 and Linux x64; ESO Live 101050 and PTS 101051

**Project Type**: Single Rust desktop crate plus managed Lua addon

**Performance Goals**: Capture remains bounded to 100,000 raw observations and
32 MiB estimated encounter data within the 128 MiB shared SavedVariables limit

**Constraints**: One-shot explicit authority, local-only, no automation or
upload, non-executing hostile import, deterministic canonical bytes, legacy
hash preservation, terminal reserve

**Scale/Scope**: 11 selected callback sources, normalization-dependent API reads,
v1/v2 capture dispatch, and v1-to-v2 store migration

## Constitution Check

*GATE: Must pass before implementation and be re-checked after design.*

- Principle I: PASS after the full S094 spec-kit sequence and actionable child
  issue are complete.
- Principle II: PASS after constitution 4.0.0 defines selected raw local capture
  while retaining explicit authority, hard bounds, declared loss, isolation, and
  no automation.
- Principle III: PASS. Contract and migration tests are written and observed red
  before production changes.
- Principle IV: PASS. The complete cargo, documentation, policy, text, encoding,
  and diff gates run before commit and again before publication when needed.
- Principle V: PASS. No new event families, transport, capture mode, upload,
  gameplay action, or third addon is introduced.
- Text hygiene: PASS. New text is UTF-8 without BOM and uses no en or em dashes.

The v3.0.0 constitution conflicts with issue #186. Governance requires the
planned major amendment before implementation. This is a product boundary
change, not a complexity waiver.

## Project Structure

### Documentation (this feature)

```text
specs/094-lossless-event-envelope/
├── analysis.md
├── checklists/
├── contracts/
├── data-model.md
├── plan.md
├── quickstart.md
├── research.md
├── spec.md
└── tasks.md
```

### Source Code (repository root)

```text
addon/EsoWeaveData/Encounter.lua
src/encounter/{mod.rs,model.rs,store.rs,validate.rs}
src/saved_variables.rs
tests/{encounter_addon.rs,encounter_import.rs,encounter_metrics.rs,
       encounter_recommendations.rs,data_addon.rs}
tests/fixtures/encounter/
docs/src/{features/encounter-capture.md,getting-started/responsible-use.md}
docs/project/{encounter-model.json,build-plans/plan-042.md}
.github/scripts/docs-policy.mjs
```

**Structure Decision**: Extend the existing encounter module, restricted parser,
immutable store, and production Lua harness. Do not add a parallel capture stack.

## Complexity Tracking

No post-amendment constitution violations remain. The store schema migration is
necessary because schema v1 database constraints reject capture v2 and rewriting
legacy canonical bytes would violate user-owned immutable history.

## Delivery Sequence

1. Amend governance and record the S094 boundary.
2. Add red contract tests for tagged raw values, all selected sources, loss, v1
   compatibility, and store migration.
3. Implement the v2 Lua raw source stream and non-destructive legacy handling.
4. Implement Rust v1/v2 models, validation, canonicalization, and store migration.
5. Prove metric and recommendation compatibility, then refresh canonical docs.
6. Run local multi-perspective review and the complete merge gate.
7. Push, open the official PR, process first-round reviews, request at most one
   second Codex review, and stop for the operator merge ritual.

