# Implementation Plan: Combat Recovery and API Compatibility

**Branch**: `codex/s120-combat-recovery-and-api-compatibility` | **Date**: 2026-10-05 | **Spec**: [spec.md](spec.md)

**Input**: S120, issues #248, #251, and #250.

## Summary

Repair the demonstrated controller/desktop binding conflict, source numeric API evidence independently of client release messages, keep unresolved compatibility visible in existing addon details, and remove the transient HUD row while logging state transitions. Existing equality guards and worker authorization checks remain authoritative.

## Technical Context

**Language/Version**: Rust 1.96.0, edition 2021; ESO Lua with Lua 5.1 fixture runtime.

**Primary Dependencies**: Existing eframe/egui, serde, ureq 3, time, tracing, mlua; no new runtime dependency.

**Storage**: Additive backward-compatible API observation fields in existing session state; no SavedVariables writes or schema changes.

**Testing**: Lua four-slot fixtures, deterministic input/worker seams, version-source and managed-package tests, headless UI and captured logs; full fmt/clippy/test parity.

**Target Platform**: Windows 10/11 x64 and Linux x64 desktop.

**Project Type**: Single desktop crate with two existing managed addon packages.

**Performance Goals**: No network or blocking work on the input hook; bounded startup source reads with timeout and body limits; one log per meaningful HUD transition.

**Constraints**: No live-game or installed-app checks, no guessed bindings/API numbers, ownership-scoped addon writes, preserve cancellation cleanup and existing gates.

**Scale/Scope**: Three issue outcomes in one slice; #249 and diagrams #221-#223 remain separate.

## Constitution Check

Before research, native-binding consumption was checked against completed S100-S102 and canonical docs. Constitution V retained obsolete fishing-only wording; a separate explicit wording amendment before analyze reconciles existing shipped behavior without adding a transport or in-game feature.

After design: full spec sequence, test-first implementation, focus/recursion/worker and ownership tests, CI parity before every Rust commit, local-only data, no raw payload logging, UTF-8 without BOM/LF, owner merge and two-round review cap all retained. No complexity violation remains.

## Project Structure

```text
specs/120-combat-recovery-and-api-compatibility/
  spec.md, plan.md, tasks.md, research.md, data-model.md, quickstart.md
  contracts/recovery.md
  checklists/requirements.md, control-and-compatibility.md
addon/PixelBeacon/PixelBeacon.lua, PixelBeacon.txt
addon/EsoWeaveData/EsoWeaveData.txt
src/beacon/api_check.rs, mod.rs
src/beacon/version_source.rs
src/config/mod.rs
src/app/mod.rs, ui.rs
src/main.rs
tests/beacon.rs, input_engine.rs, real_sink.rs
tests/app_view_model.rs, app_ui_sizing.rs, data_addon.rs
docs/src/, docs/archive/build-plans/plan-048.md
```

## Implementation Sequence and Decisions

1. Write failing mixed keyboard/controller fixtures. Classify controller controls before desktop signature/conflict counting; retain genuine desktop ambiguity, unsupported controls, and controller-only failure. Exercise decoded evidence through admission and timed synthesis, including equivalent snapshots and true cancellation (FR-001-004, SC-001).
2. Write failing source/compatibility tests. Fetch bounded selected-channel history and documentation pinned to its immutable revision. Parse the documented numeric API separately from the newest numeric client release, reject stale/future/malformed/channel-mismatched evidence, and retain unknown on failures. Keep `GameVersionSource` fixture compatibility via a default evidence method; production overrides it (FR-005-006,009,011).
3. Keep package declarations limited to reviewed APIs. Persist observations separately from package support, never use a newer observed API as permission to stamp support. Present supported/incompatible/unknown and update guidance in existing addon details on every start (FR-007-010, SC-002).
4. Write failing UI/log tests, delete only the conditional freshness row, and log loss/cause/expiry/recovery transitions independently of retained values. Preserve presentation metadata and input gates (FR-012-013, SC-003-004).
5. Update canonical copy, changelog, task outcomes and evidence; run full automated parity and text hygiene. Commit, authorized push, official PR, respond to every external review, request at most one additional Codex round, then owner final review after green checks (FR-014, SC-005).

Source evidence is a compatibility diagnostic, not a new input eligibility gate. The exact owner's installed controls/build remain unobserved. Research establishes a matching disabling path and reviewed API 101051, not proof of repaired live behavior.

## Complexity Tracking

No additional crate, daemon, transport, permission system, or constitutional exception.
