# Implementation Plan: Deterministic SQLite Catalog Compiler

**Branch**: `codex/s070-catalog-compiler` | **Date**: 2026-09-09 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/070-catalog-compiler/spec.md`

## Summary

Complete issue #114 with a deliberate Rust compiler command, a normalized and
constraint-backed SQLite schema, deterministic semantic and artifact evidence,
atomic last-known-good publication, a typed read-only application seam, a
minimal rights-compatible baseline, and predictable Windows and Linux package
locations. Retain the single Cargo package and keep collection, icon
acquisition, encounter storage, and user updates outside this slice.

## Technical Context

**Language/Version**: Rust 1.96, SQL, JSON, Markdown, shell and PowerShell package
manifests

**Primary Dependencies**: `rusqlite` 0.40.2 with bundled SQLite, `sha2` 0.10,
`tempfile` 3, existing Serde and thiserror dependencies

**Storage**: Immutable SQLite schema version 1, normalized JSON compiler inputs,
stable JSON reports, and one repository-owned minimal baseline catalog

**Testing**: Rust integration tests, package-policy tests, full Cargo merge gate,
documentation policy, mdBook, linkcheck, spelling, and text hygiene

**Target Platform**: Windows 10/11 x64 and Linux x64

**Project Type**: Single-package desktop application plus dedicated maintainer
compiler binary

**Performance Goals**: Linear bounded ingestion up to the provisional 64 MiB and
500,000-record input limits, with deterministic ordering and no application
startup network or write work

**Constraints**: Read-only runtime; no `build.rs` generation; no Lua or source
execution; no third-party art; no user encounter data; live/PTS isolation;
UTF-8 without BOM; LF; no forbidden dash characters

**Scale/Scope**: One schema version, one baseline fixture, twelve semantic
tables, five diff surfaces, four package layouts, and typed release/entity
lookups

## Constitution Check

*GATE: Passed before research and design. Re-check required after implementation.*

- **Spec-first traceability**: PASS. Issue #114 maps to S070 and active Plan 038.
- **Safety invariants**: PASS. Input, action, Pixel Bus, and beacon behavior are
  unchanged.
- **Test-first delivery**: PASS planned. Compiler, runtime, and package tests are
  added before their implementation.
- **CI parity**: PASS planned. Every Rust commit follows fmt, clippy, and locked
  full tests.
- **Bounded scope**: PASS. The tool consumes declarative local inputs and never
  enters the game, reads packets, or discovers data itself.
- **Single-package discipline**: PASS. A second binary reuses the existing
  package without creating a workspace.
- **Configuration discipline**: PASS. Catalog state is immutable package data,
  not user settings or session state.
- **Pinned artifacts**: PASS planned. Packaging and release workflow changes
  receive a dated changelog decision and policy tests.
- **Text hygiene**: PASS by design with final automated audit.

## Project Structure

```text
specs/070-catalog-compiler/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── contracts/
│   └── catalog-bundle.schema.json
├── checklists/
│   ├── requirements.md
│   └── catalog-integrity.md
├── fixtures/
│   ├── minimal-live.json
│   ├── minimal-live-changed.json
│   └── minimal-pts.json
└── tasks.md
```

```text
src/
├── catalog/
│   ├── compiler.rs
│   ├── model.rs
│   ├── schema.rs
│   └── mod.rs
├── bin/
│   └── catalog-compiler.rs
├── app/{mod.rs,strings.rs,ui.rs}
├── lib.rs
└── main.rs
tests/
├── catalog_compiler.rs
├── catalog_runtime.rs
└── catalog_packaging.rs
assets/catalog/
├── baseline.json
└── catalog.sqlite
wix/main.wxs
packaging/appimage/AppDir/
.github/workflows/release.yml
docs/src/development/
docs/project/build-plans/
CHANGELOG.md
```

**Structure Decision**: Add a library-owned catalog boundary and a thin second
binary inside the existing package. Keep SQL, validation, canonical hashing,
publication, and typed reads out of UI code. Commit the minimal database as a
generated artifact beside its reviewable JSON source.

## Delivery Sequence

1. Complete specify, clarify, integrity checklist, research, model, contract,
   quickstart, tasks, and pre-implementation analysis.
2. Move issue #114 from Specced to In progress with Slice S070.
3. Add failing model, compiler, read-only runtime, and packaging tests.
4. Add dependencies, schema, validated input model, compiler, reports, diff,
   rollback, and CLI.
5. Add the typed runtime seam, predictable locator, warning log, and visible
   Catalog status line.
6. Compile and audit the rights-compatible baseline from its JSON authority.
7. Update every package layout, pinned release logic, canonical documentation,
   Plan 038 records, and changelog decision.
8. Complete the integrity checklist, post-implementation analysis, and every
   proportional local merge gate.
9. Commit, push the authorized branch, publish the official PR, resolve all CI
   and review findings, use at most one additional `@Codex` round, and stop for
   the operator merge ritual.

## Design Decisions

1. Use `rusqlite` with bundled SQLite. It removes undeclared system-library
   variance across supported packages and keeps the runtime API identical on
   Windows and Linux.
2. Keep one Cargo package with an explicit `catalog-compiler` binary. A workspace
   promotion would add lifecycle and release complexity for no current isolation
   benefit.
3. Use a normalized entity/attribute/relation core rather than speculative empty
   tables for every future ESO concept. The constrained entity-kind vocabulary
   covers all approved S068/S069 concepts and allows evidence-backed specialized
   tables through forward migrations.
4. Make the semantic hash a canonical projection of all semantic database rows.
   Treat the file hash as artifact evidence, not the cross-SQLite semantic
   authority.
5. Build into a sibling temporary file, validate after a read-only reopen, copy
   the prior file to a stable rollback path, write the rollback manifest, then
   atomically persist the candidate.
6. Store the catalog beside the executable on Windows and portable Linux, under
   `/usr/share/eso-weave/catalog` for Debian, and under AppDir
   `/usr/share/eso-weave/catalog` for AppImage. The locator checks only these
   deterministic paths plus the debug checkout path.
7. Ship a minimal synthetic baseline containing no game art or user-collected
   prose. Unknown coverage is accurate and keeps later collectors optional.

## Complexity Tracking

No constitution violations require justification.
