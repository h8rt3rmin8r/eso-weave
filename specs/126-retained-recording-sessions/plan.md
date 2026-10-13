# Implementation Plan: Retained Recording Sessions

**Branch**: codex/s126-retained-recording-sessions | **Date**: 2026-10-13 | **Spec**: [spec.md](spec.md)
**Input**: Owner-approved S126 / issue #273.

## Summary

Preserve current controller capture logic and add an immutable ordered archive.
A new controller envelope accepts existing v1/controller4 data and writes
v2/controller5, without changing terminal capture schema2/addon3. Explicit
resume records the prior stop; import validates all sessions, then appends the
selected environment's records and per-session snapshots in one transaction.
Desktop history gets durable first-import session order, preserving grouping
through deletion and reimport.

## Technical Context

**Language/Version**: Rust 1.96.0 and ESO-compatible Lua.
**Primary Dependencies**: Existing serde, rusqlite, egui and mlua test seams; none added.
**Storage**: User-owned SavedVariables plus encounter SQLite store schema5.
**Testing**: Lua lifecycle through mlua; parser/store/history/UI integration.
**Target Platform**: Windows 10/11 x64 and Linux x64.
**Project Type**: Desktop and managed addon.
**Performance Goals**: No extra event callbacks or blocking UI work; cached archived totals.
**Constraints**: Whole-spool ceilings of 32 sessions, 1024 encounter slots,
100000 events, 100000 raw observations, 1024 interruptions and 32 MiB estimated
capture bytes; existing terminal and failure reserves remain available.
**Scale/Scope**: One active/current session and up to 31 immutable archives.
Identity counters remain exact and monotonic through explicit clearing.

## Constitution Check

Pre-research and post-design checks PASS:
- I: Issue #273, canonical capture documents and complete spec-kit artifacts.
- II/V: Explicit existing warning/authority, exact loss-aware capture, no silent
  eviction, catalog isolation, four-file inventory, no source deletion by desktop.
- III: Tests precede implementation using existing Lua, worker and store seams.
- IV: Full fmt, Clippy and locked tests before source commits.
- Text: UTF-8 without BOM, LF, no long-dash punctuation.
- Owner boundaries: automatic branch/PR publication; at most two external review
  rounds; no merge, tag, release or owner field-verification work.

## Project Structure

- addon/EsoWeaveData/Encounter.lua: retention, admission, resume, discard and migration.
- src/encounter/model.rs, mod.rs, validate.rs: version dispatch and spool validation.
- src/encounter/store.rs: atomic per-session snapshots, durable order and migration.
- src/encounter/history.rs; src/app/ui.rs: selected-environment last-saved summaries.
- tests/encounter_addon.rs, encounter_import.rs, encounter_history.rs,
  app_encounter_history.rs and app_ui_sizing.rs: lifecycle and interface coverage.
- docs/src/features/encounter-capture.md and related reference/workflow documents.
- docs/project/build-plans/plan-052.md and archived Plan051.

## Phases

1. Resolve identity, archive shape, global budgets and snapshot extension research.
2. Produce [data model](data-model.md), [contract](contracts/retention.md) and
   [quickstart](quickstart.md); update agent context.
3. Generate story-ordered tasks, run blocking analyze, then write red tests.
4. Implement retention/import foundations, then new start, resume, discard and UI.
5. Update canonical docs, archive Plan051, record dated architecture decisions.
6. Run local gates; commit, automatically publish official PR; satisfy reviews
   and final-head CI, then request owner merge.

## Decisions and Alternatives

- Immutable terminal snapshots within v2 are preferred over rewriting every callback
  against a new nested current container. Unknown fields and nested archives fail.
- Whole-spool ceilings are retained rather than multiplying per-session limits.
- Persisted sequence IDs and ordered keys are preferred over wall-clock ordering.
- Store schema5 adds one immutable session-order table; lexical IDs and surviving
  rowid order can reorder history after clock changes or deletion.
- Resume adds a UserResumed marker bound to prior_finished_at. Unconditional
  stopped-to-active relaxation would allow terminal metadata rewrites.
- New start archives a valid failed session; failed sessions cannot resume.
- Environment filtering follows complete validation, never hides malformed data.

## Complexity Tracking

No constitution exceptions. One bounded archive and one durable ordering table
are proportional to retention and chronology requirements; no generic queue,
new service, dependency, transport or native parser is introduced.
