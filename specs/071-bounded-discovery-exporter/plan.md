# Implementation Plan: Bounded ESO Discovery Exporter

**Branch**: `codex/s071-bounded-discovery-exporter` | **Date**: 2026-09-09 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from
`/specs/071-bounded-discovery-exporter/spec.md`

## Summary

Implement issue #115 as a separately managed, manually initiated ESO addon that
enumerates five approved bounded API families across budgeted frames and emits
deterministic checksummed SavedVariables chunks. Add a strict non-executing Rust
parser and importer to the existing catalog compiler that validates hostile
captures and atomically stages S070 catalog-bundle JSON. Include independent
lifecycle tooling, invented live/PTS fixtures, documentation, and project
records. Do not add encounter capture, icon bytes, automated discovery builds,
or the polished application updater.

## Technical Context

**Language/Version**: Rust 1.96, ESO Lua API 101050/101051 contract

**Primary Dependencies**: Existing `serde`, `serde_json`, `sha2`, `tempfile`,
and standard library only

**Storage**: ESO SavedVariables input and deterministic JSON staging output;
existing S070 SQLite compiler remains the publication boundary

**Testing**: Rust integration tests, static addon contract tests, invented live
and PTS fixtures, existing documentation policy and full Cargo merge gate

**Target Platform**: Windows 10/11 x64 and Linux x64 desktop; ESO live and PTS
addon environments

**Project Type**: Single Rust desktop crate plus one embedded ESO addon

**Performance Goals**: At most 64 records and 4 ms of collector work per update
tick; bounded linear import of at most 64 MiB and 500,000 records

**Constraints**: No Lua execution, upload, game memory or packet access,
synthesized input, PixelBeacon coupling, guessed enumeration, PTS promotion,
third-party art bytes, or direct active-catalog publication

**Scale/Scope**: Five iterator categories, 1,024 chunks, 64 KiB per chunk,
64 KiB per string, depth 16, and one staged bundle per capture

## Constitution Check

*GATE: Passed before Phase 0 research and rechecked after Phase 1 design.*

- **Spec-driven development**: PASS. Issue #115, canonical S068/S070 documents,
  this numbered slice, clarification record, checklists, research, data model,
  contracts, plan, tasks, and analyze gate form one authority chain.
- **Safety-critical surfaces**: PASS. PixelBeacon is excluded and its lifecycle
  regression suite remains mandatory. Collector confinement and marker-gated
  removal receive equivalent dedicated tests.
- **Test-first seams**: PASS. Hostile input, lifecycle, and static addon tests
  precede implementation. Invented fixtures replace dependence on a live game.
- **CI parity**: PASS. Formatting, strict Clippy, locked full tests, optimized
  builds, docs, policy, hygiene, and package checks run before commits.
- **Bounded addon boundary**: PASS after governance correction. The initial
  analyze gate found that Constitution 2.0.1 allowed only PixelBeacon despite
  issue #115 and S068 requiring a dedicated collector. Constitution 2.1.0 now
  permits exactly one separately installed, user-initiated, read-only collector
  that calls documented APIs and writes local SavedVariables. The desktop
  neither reads process memory nor intercepts traffic.
- **Platform/config/text constraints**: PASS. No configuration schema change is
  needed. All authored text is UTF-8 without BOM and LF, with forbidden dashes
  excluded.
- **Pinned artifacts**: PASS. No pinned workflow or packaging edit is planned.
  The architecture decision is recorded in `CHANGELOG.md`.

Post-design recheck: PASS. The constrained parser and staging-only output reduce
authority and preserve every gate. No constitution exception is required.

## Project Structure

### Documentation (this feature)

```text
specs/071-bounded-discovery-exporter/
├── analysis.md
├── checklists/
│   ├── collector-safety.md
│   └── requirements.md
├── contracts/
│   └── collector-envelope.schema.json
├── data-model.md
├── fixtures/
│   ├── live.lua
│   └── pts.lua
├── plan.md
├── quickstart.md
├── research.md
├── spec.md
└── tasks.md
```

### Source Code (repository root)

```text
addon/EsoWeaveCollector/
├── EsoWeaveCollector.lua
└── EsoWeaveCollector.txt

src/
├── bin/catalog-compiler.rs
├── catalog/
└── collector/
    ├── import.rs
    ├── lifecycle.rs
    ├── mod.rs
    └── parser.rs

tests/
├── collector_addon.rs
├── collector_import.rs
└── collector_lifecycle.rs
```

Canonical documentation adds a collector guide and updates architecture,
testing, status, source-rights, build-plan, migration-ledger, and changelog
records.

**Structure Decision**: Keep one Cargo crate and embed the separate addon from
`addon/`, matching the existing project layout. The collector module owns all
capture contract and lifecycle code. The catalog compiler exposes staging and
lifecycle commands but active SQLite publication remains in `catalog`.

## Implementation Phases

1. Freeze envelope schema, limits, category/type matrix, fixtures, and tests.
2. Implement restricted SavedVariables lexer/parser with allocation-time limits.
3. Implement typed envelope and record validation plus Adler-32 checks.
4. Map captures deterministically into S070 catalog bundles and publish staging
   atomically with a redacted receipt.
5. Implement independent marker-gated collector lifecycle and CLI commands.
6. Implement frame-budgeted Lua adapters, normalization, chunking, commands,
   combat pause, cancellation, checkpoint, and completion behavior.
7. Update canonical documentation and chronological project records.
8. Run analyze, focused tests, full local gates, PR CI, and at most two Codex
   review rounds.

## Complexity Tracking

No constitution violation or unjustified repository expansion exists. A custom
parser is narrower than adding a Lua runtime or general parser dependency and
is restricted to the exact data grammar the project emits.
