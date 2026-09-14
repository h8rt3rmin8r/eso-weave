# Implementation Plan: Data Addon Lifecycle UI

**Branch**: `codex/s093-data-addon-lifecycle-ui` | **Date**: 2026-09-13 | **Spec**: [spec.md](spec.md)

**Input**: Issue #185 under epic #182

## Summary

Add an evidence-scoped data-addon presentation model and first-class lifecycle
row directly beneath PixelBeacon. Reuse the S092 lifecycle authority, retain
reload outcomes, label unsupported evidence as unconfirmed, remove duplicate
lifecycle controls from Catalog Update, and update deterministic documentation.

## Technical Context

**Language/Version**: Rust 1.89.0, edition 2021

**Primary Dependencies**: Existing `eframe`, `serde`, `mlua`, `sha2`, and
application worker infrastructure; no new runtime dependency

**Storage**: Existing settings only; no schema change and no SavedVariables read

**Testing**: Pure view-model tests, lifecycle integration tests, egui sizing and
accessibility tests, deterministic screenshot capture, full locked Cargo suite,
and documentation policy checks

**Target Platform**: Windows 10/11 x64 and Linux x64, including Proton paths

**Performance Goals**: No SavedVariables parse; lifecycle status refresh
completes off the paint path; idle UI adds no continuous filesystem scan

**Constraints**: Preserve data-addon atomic ownership guarantees, no current
session claims from process state, no command transport, no SavedVariables
mutation, and no bulk PixelBus use

**Scale/Scope**: One issue, one main lifecycle row plus evidence subrows, one
shared observation authority, one modal de-duplication, canonical docs, and
affected screenshots

## Constitution Check

*GATE: Passed before implementation and re-checked after design.*

- **Spec-driven development**: PASS. One issue maps to three independently
  testable stories and a complete spec-kit package.
- **Safety-critical surfaces**: PASS. All mutations delegate to the S092
  marker-gated atomic authority; no links, neighboring files, or shared saved
  state are modified.
- **Test-first development**: PASS by plan. Projection, ordering, action,
  retention, and error tests precede production changes.
- **CI parity**: PASS by plan. Formatting, strict Clippy, full locked tests, and
  documentation gates run before publication.
- **Bounded scope**: PASS. S093 adds presentation and lifecycle orchestration
  only and explicitly excludes downstream ingestion and command work.
- **Configuration and text hygiene**: PASS. No setting is added; every edited
  text file is UTF-8 without BOM and checked for mojibake.
- **Pinned artifacts**: PASS. No dependency, workflow, package, or
  SavedVariables schema change is required.

Post-design re-check: PASS. The design separates observation provenance from
display roles, retains reload state, shares one cached lifecycle snapshot, and
does not manufacture a live addon channel.

## Project Structure

### Documentation

```text
specs/093-data-addon-lifecycle-ui/
|-- spec.md
|-- plan.md
|-- research.md
|-- data-model.md
|-- quickstart.md
|-- tasks.md
|-- analysis.md
|-- screenshot-plan.md
|-- contracts/data-addon-lifecycle-status.md
`-- checklists/
```

### Source

```text
src/app/mod.rs
src/app/strings.rs
src/app/ui.rs
src/data_addon.rs
tests/app_view_model.rs
tests/app_ui_sizing.rs
tests/app_strings.rs
tests/catalog_update.rs
tests/data_addon.rs
tests/support/documentation_capture.rs
docs/src/
```

**Structure Decision**: Keep lifecycle mutation in `data_addon`; add pure
observation projection and intent orchestration in `app`; keep egui rendering in
`app/ui`. The Catalog Update dialog consumes the same status and owns only the
capture/build workflow.

## Phase 0: Research

See [research.md](research.md). The selected design uses explicit provenance,
explicit unconfirmed evidence, a retained reload outcome, and no new transport.

## Phase 1: Design

See [data-model.md](data-model.md) and
[contracts/data-addon-lifecycle-status.md](contracts/data-addon-lifecycle-status.md).

## Phase 2: Test-First Implementation

1. Add failing pure projection, action matrix, row ordering, confirmation,
   stale-error, and modal de-duplication tests.
2. Add the observation and presentation types, cached refresh path, and direct
   lifecycle intents.
3. Render the lifecycle row and evidence details beneath PixelBeacon.
4. Remove duplicate modal mutations and update all operator guidance.
5. Refresh deterministic screenshots and run every repository gate.

## Complexity Tracking

No constitutional violation or exceptional complexity is accepted. A cached
observation exists because filesystem inspection does not belong in a per-frame
renderer. Optional stale SavedVariables evidence is deliberately not parsed.
