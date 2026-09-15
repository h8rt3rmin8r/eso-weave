# Implementation Plan: Encounter Capture Modes

**Branch**: `codex/s096-encounter-capture-modes` | **Date**: 2026-09-15 | **Spec**: [spec.md](spec.md)
**Input**: Issue #183 and the approved control-model correction under epic #182.

## Summary

Replace the singleton one-shot SavedVariables shape with a bounded, versioned
controller and session spool. The addon exposes exactly single and continuous
modes through explicit in-game commands, retains independently replayable S095
encounter records with shared session identity and ordinal order, and declares
every interruption or hard failure. Rust imports every terminal member only
after whole-spool validation and one atomic SQLite transaction. The desktop
groups history by session and labels controller facts as last-saved evidence;
it never controls capture.

## Technical Context

**Language/Version**: Rust 1.88.0, Lua 5.1-compatible ESO addon runtime, Markdown, JSON
**Primary Dependencies**: serde, serde_json, rusqlite, sha2, egui/eframe, existing bounded SavedVariables parser
**Storage**: `EsoWeaveDataSaved.encounter` plus local SQLite encounter store schema v4
**Testing**: production-Lua harness, Rust unit/integration tests, mdBook policy and render tests
**Target Platform**: Windows 10/11 x64, Linux x64, ESO Live and PTS addon runtimes
**Project Type**: single-crate desktop application with two embedded managed addons
**Performance Goals**: no detailed handlers between encounters; linear validation/import; one batch transaction
**Constraints**: 128 MiB shared-file read cap, 32 MiB aggregate encounter spool, 100,000 event/raw caps, no command ingress, no silent eviction, value-free diagnostics
**Scale/Scope**: two modes, at most 1,024 terminal encounters and 1,024 interruption markers per retained session

## Constitution Check

### Before Phase 0 research

- Principle I: PASS. Issue #183, current manuals, Plan 042, and complete spec-kit
  artifacts form the authority chain.
- Principles II and V: AMENDMENT REQUIRED. Constitution 4.0.0 requires one-shot
  capture. Issue #183 and the operator-approved S096 approach require exactly
  single and continuous modes. S096 will amend only those authority clauses to
  version 5.0.0 while preserving explicit user action, hard bounds, local-only
  storage, declared loss, value-free diagnostics, and no gameplay automation.
- Principle III: PASS. Production-Lua and Rust tests precede implementation for
  each controller, parser, store, and UI behavior.
- Principle IV: PASS. The full foreground cargo gate runs before each Rust commit.
- Platform/configuration: PASS. Selected mode lives in addon-owned session state,
  not desktop configuration; all text remains UTF-8 without BOM and LF.
- Workflow: PASS. The operator authorized push and official PR publication at kickoff.

### After Phase 1 design

- Principle I: PASS. Requirements map to contracts and chronological tasks.
- Principles II and V: PASS after the planned 5.0.0 amendment. No third mode or
  new transport is permitted.
- Principle III: PASS. Tests cover every state transition and batch failure seam.
- Principle IV: PASS. Verification commands remain unchanged and mandatory.
- Principle V: PASS. The outer spool remains inside ESO Weave Data; PixelBeacon,
  native logs, uploads, input, and automation remain excluded.

## Project Structure

### Documentation (this feature)

```text
specs/096-encounter-capture-modes/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── contracts/
│   ├── batch-import-contract.md
│   ├── capture-mode-state-machine.md
│   └── continuous-session-envelope.md
├── checklists/
│   ├── control-boundary.md
│   ├── hostile-input.md
│   ├── recovery-and-loss.md
│   └── requirements.md
└── tasks.md
```

### Source Code

```text
addon/EsoWeaveData/
└── Encounter.lua

src/
├── encounter/
│   ├── history.rs
│   ├── mod.rs
│   ├── model.rs
│   ├── replay.rs
│   ├── store.rs
│   └── validate.rs
└── app/
    ├── encounter_history.rs
    ├── strings.rs
    └── ui.rs

tests/
├── encounter_addon.rs
├── encounter_import.rs
├── encounter_history.rs
├── encounter_metrics.rs
├── app_encounter_history.rs
└── app_view_model.rs
```

**Structure Decision**: Extend the existing addon module, encounter domain, and
history UI. Do not create a third addon, crate, desktop config authority, or
transport subsystem.

## Implementation Phases

1. Amend governance and lock the two-mode, no-ingress contracts.
2. Add failing production-Lua tests for the controller, session spool, bounds,
   mid-combat activation, continuous ordering, and recovery.
3. Implement the addon controller while freezing nested S095 capture schema v2
   and addon-format version 3. Mid-combat records are explicitly partial and
   replay-indeterminate, so normal replay and canonical hashes remain unchanged.
4. Add failing hostile-parser and store tests, then implement outer state-schema
   dispatch, strict session validation, atomic batch import, store schema v4,
   and byte-preserving legacy migration.
5. Add grouped-history and desktop provenance tests, then expose ordinal/mode
   context and count-based import receipts without adding controls.
6. Update canonical documentation, machine model, Plan 042, and changelog; run
   spec analysis, all local gates, independent review, and hosted review.

## Complexity Tracking

| Addition | Why Needed | Simpler Alternative Rejected Because |
| --- | --- | --- |
| Outer controller/session spool | Continuous capture must retain multiple terminal encounters plus interruption and failure truth | Reusing one singleton would overwrite evidence or merge actor/sequence domains |
| Store schema v4 with session snapshots | Growing spools need authoritative ordinal order and append-only interruption history | Timestamp order and a mutable session row can silently rewrite historical meaning |
| Atomic batch import | One malformed or colliding member must leave existing history unchanged | Repeated single-record commits can partially import a rejected spool |
