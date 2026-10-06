# Implementation Plan: Encounter Logging and Addon Clarity

**Branch**: `codex/s121-encounter-logging-and-addon-clarity` | **Date**: 2026-10-05 | **Spec**: [spec.md](spec.md)
**Input**: S121, issues #249 and #222.

## Summary

Audit and repair every relevant player-facing desktop, generated diagnostic, addon-command and public/bundled-help message. Add the encounter-lineage SVG using the same final vocabulary and retain a source-located state matrix. Preserve existing capture, command and data behavior.

## Technical Context

**Language/Version**: Rust 1.96.0, Lua 5.1 fixtures, mdBook 0.5.4, Node and local SVG.
**Primary Dependencies**: Existing egui/eframe, mlua, serde and docs theme; no new dependency.
**Storage**: Existing SavedVariables/database unchanged; repository audit artifacts only.
**Testing**: Desktop and Lua state fixtures, source inventory, ownership/retention/security suites, docs build/policy/headless browser.
**Target Platform**: Windows 10/11 x64 and Linux x64; offline documentation.
**Project Type**: Single crate, two managed addon packages, static docs.
**Performance Goals**: No new worker, callback, request or hook-thread work.
**Constraints**: UTF-8 without BOM/LF, no long dashes, no field checks or raw disclosure, existing capability preserved.
**Scale/Scope**: Complete #249 and #222. #221/#223 and release remain separate.

## Constitution Check

Pre-research and post-design pass: full spec sequence, two recording modes, module ownership, no desktop game-command ingress, local exact raw retention, test-first fixture discipline, full fmt/clippy/test before Rust commits, mandatory safety/security tests, offline documentation, explicit push/PR authority, two Codex rounds maximum and owner merge. No exception or amendment needed.

## Project Structure

- `specs/121-encounter-logging-and-addon-clarity/`: spec, plan, research, data-model, contracts/messaging.md, quickstart, tasks, checklists, audit and verification.
- `src/app/{strings,mod,ui,encounter_history}.rs`, `src/encounter/history.rs`, visible catalog-update diagnostics.
- `addon/EsoWeaveData/{Catalog,Encounter}.lua`; desktop/history/Lua tests.
- `docs/src/` relevant feature/setup/reference/development pages and assets/diagrams/encounter-evidence-lineage.svg.
- `docs/project/content-coverage.json`, diagram records, Plan048, `CHANGELOG.md` and documentation policy/smoke scripts.

## Implementation Sequence and Decisions

1. Capture complete message/state inventory before editing and reconcile former/final dispositions (FR-001-002).
2. Tests first for setup/state/recovery/current-versus-saved/deletion, Lua status/help/unknown version and diagram contracts (FR-003-008,010-012).
3. Desktop, addon and docs work can proceed in parallel on owned disjoint files after analyze passes; integrate under the shared messaging contract.
4. Keep fixed commands/codes/schema versions. Explain internal codes and make provenance secondary but available. Unknown-version evidence cannot be cleared; same-version malformed state can. Optional import-before-clear guidance is not a new prerequisite.
5. Exact managed byte comparison offers Update Data after copy changes; recalculate Catalog.lua normalized self-checksum.
6. Reuse SVG figure dialog/topology/manifest and extend policy plus desktop/narrow/200-percent zoom coverage (FR-009-011).
7. Align guides/changelog/planning, finish complete matrix and parity, publish PR, resolve all findings, at most one additional @Codex review, then owner final review after green CI (FR-013).

## Complexity Tracking

No new crate, service, command, schema, permission gate or capability restriction. No release or field verification.
