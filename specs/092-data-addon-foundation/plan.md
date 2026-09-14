# Implementation Plan: Persistent Data Addon Foundation

**Branch**: `codex/s092-data-addon-foundation` | **Date**: 2026-09-13 | **Spec**: [spec.md](spec.md)

**Input**: Issues #184, #187, and #189 under coordinating epic #182

## Summary

Replace the development-only Collector and Encounter packages with one managed
`EsoWeaveData` addon containing a bootstrap plus isolated Catalog and Encounter
modules. Preserve their current behavior while moving both SavedVariables
payloads under one namespaced root and replacing destructive desktop deletion
with module-local in-game clearing. In parallel, publish an evidence-backed
provisional native encounter-log ingestion decision with separate operator
verification, and publish a no-go decision for real-time desktop-to-addon
commands.

## Technical Context

**Language/Version**: Rust 1.89.0 (edition 2021), ESO Lua API 101050 and 101051

**Primary Dependencies**: Existing `serde`, `serde_json`, `sha2`, `rusqlite`,
`mlua`, `tracing`, and `eframe` dependencies; no new runtime dependency

**Storage**: One bounded `SavedVariables/EsoWeaveData.lua` envelope with
`EsoWeaveDataSaved.catalog` and `EsoWeaveDataSaved.encounter`; existing catalog
and encounter SQLite stores remain unchanged

**Testing**: Rust unit and integration tests, `mlua` addon harness tests,
documentation policy checks, strict Clippy, and full locked Cargo suite

**Target Platform**: Windows 10/11 x64 and Linux x64, including ESO through
Proton for later field verification

**Project Type**: Single-crate cross-platform desktop application with embedded
ESO addon sources

**Performance Goals**: Idle modules register no high-frequency update or
capture callbacks; package consolidation adds no PixelBus work; shared-file
parsing remains bounded

**Constraints**: No process-memory or packet access, no bulk PixelBus data, no
command transport implementation, no custom bindings, no legacy migration, no
desktop rewrite or deletion of shared SavedVariables, and no weakening of input
or managed-addon safety tests

**Scale/Scope**: Three source issues, one addon package replacing two
development packages, two isolated module payloads, three design contracts, and
one separate field-verification issue

## Constitution Check

*GATE: Passed before Phase 0 research and re-checked after Phase 1 design.*

- **Spec-driven development**: PASS. S092 contains the full spec-kit artifact
  chain and maps issues #184, #187, and #189 to independently testable stories.
- **Safety-critical surfaces**: PASS after the explicitly approved constitution
  3.0.0 amendment. One marker-gated data-addon subtree replaces two package
  boundaries without weakening module isolation, encounter privacy, catalog
  combat prohibition, PixelBeacon protection, or input safety.
- **Test-first development**: PASS. Package, parser, Lua isolation, shared-state
  preservation, and static prohibition tests precede implementation changes.
- **CI parity**: PASS by plan. Formatting, strict Clippy, and the full locked
  suite run before every Rust commit.
- **Bounded scope**: PASS. The product contains exactly PixelBeacon and one data
  addon. S092 adds no in-game capability and no transport implementation.
- **Configuration and text hygiene**: PASS. No new user configuration is added;
  every edited text file is UTF-8 without BOM, LF-only, and contains no Unicode
  dash punctuation.
- **Pinned artifacts**: PASS. No workflow or package definition changes are
  required. The maintainer release procedure changes only to describe the new
  embedded addon inventory and receives a dated changelog decision.

Post-design re-check: PASS. The package contract requires exact file inventory,
marker ownership, rollback, module-local clearing, a 128 MiB outer read limit,
and preservation of both narrower module validators. Both transport contracts
are decision records and authorize no new runtime bridge.

## Project Structure

### Documentation (this feature)

```text
specs/092-data-addon-foundation/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── checklists/
│   ├── requirements.md
│   ├── addon-boundary.md
│   └── research-evidence.md
├── contracts/
│   ├── addon-package.md
│   ├── command-transport-decision.md
│   └── ingestion-transport-decision.md
└── tasks.md
```

### Source Code (repository root)

```text
addon/
├── PixelBeacon/                 # unchanged
└── EsoWeaveData/
    ├── EsoWeaveData.txt
    ├── EsoWeaveData.lua
    ├── Catalog.lua
    └── Encounter.lua

src/
├── data_addon.rs                # package lifecycle and embedded inventory
├── collector/                   # catalog domain, restricted parsing, import
├── encounter/                   # encounter domain, import, storage, metrics
├── catalog_update/              # data-addon lifecycle orchestration
└── app/                         # existing catalog and encounter actions

tests/
├── data_addon.rs
├── collector_addon.rs
├── collector_import.rs
├── encounter_addon.rs
├── encounter_import.rs
└── fixtures/
```

**Structure Decision**: Keep catalog and encounter Rust domain modules stable so
downstream SQLite and calculation code does not churn. Add one package-level
lifecycle authority and consolidate only the addon deployment, Lua bootstrap,
module SavedVariables projection, and product-facing paths.

## Ordered Delivery

1. Amend governance and freeze the S092 contracts.
2. Add failing package-inventory, lifecycle, parser, and module-isolation tests.
3. Create `EsoWeaveData`, adapt parsers and consumers, and remove obsolete
   package artifacts.
4. Remove destructive desktop capture deletion and add module-local clear.
5. Update canonical and maintainer documentation.
6. Publish the ingestion and command decisions plus field-verification owner.
7. Run analysis, CI parity, local review, and remote review.

## Complexity Tracking

No unresolved constitution violation remains. The major constitution bump is an
explicitly approved governance amendment, not an exception to a current rule.
