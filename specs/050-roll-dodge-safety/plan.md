# Implementation Plan: Roll-Dodge Safety

**Branch**: `codex/050-roll-dodge-safety` | **Date**: 2026-09-05 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/050-roll-dodge-safety/spec.md`

## Summary

Close #57 and #60 by publishing a bounded Unknown, Inactive, or Active roll-dodge
state in PixelBeacon B23, retaining old protocol extents, and applying the state
at the hook, worker, and running-sequence boundaries without swallowing or
replaying physical player input.

## Technical Context

**Language/Version**: Rust 1.89, Lua 5.1-compatible ESO addon code, egui 0.36

**Primary Dependencies**: Existing eframe/egui, PixelBeacon, InputEngine, WeaveEngine

**Storage**: None; roll-dodge state is runtime-only

**Testing**: Rust integration tests, embedded-addon contract tests, headless view tests

**Target Platform**: Windows 10/11 x64 and Linux x64 companion, ESO addon API 101050+

**Performance Goals**: Filter combat events in native event dispatch; reuse the
existing 100 ms addon tick; add no thread, OS query, allocation, or hook-thread wait

**Constraints**: Unknown fails closed; physical keys pass through; no replay;
completion event beats watchdog; UTF-8 without BOM and LF

**Scale/Scope**: One addon block, one typed reader event, one hook gate, one worker
gate, one Live HUD row, two atomic issues

## Constitution Check

*GATE: Passed before research, after design, and after implementation.*

- **Spec-first traceability**: PASS. Issues #57 and #60 map to build plan 020 and
  a complete S050 specification, checklists, research, model, contract, and tasks.
- **Safety invariants**: PASS by design. Unknown closes generated weave work while
  physical input and toggle hotkeys remain available.
- **Test-first delivery**: PASS by plan. Failing wire, lifecycle, routing, hook,
  worker, sink, and view tests precede product changes.
- **CI parity**: PASS. Format, all-target/all-feature lint, and the complete
  locked test suite pass.
- **Bounded scope**: PASS. State crosses only the permitted screen-signal contract.
- **Configuration discipline**: PASS. No runtime state or watchdog is persisted.
- **Text hygiene**: PASS. The final encoding, punctuation, whitespace, and
  mojibake audit reports no violation.

## Project Structure

### Documentation

```text
docs/plans/plan-020.md
specs/050-roll-dodge-safety/
|-- spec.md
|-- plan.md
|-- research.md
|-- data-model.md
|-- quickstart.md
|-- analysis.md
|-- checklists/
|   |-- requirements.md
|   `-- safety.md
|-- contracts/
|   `-- roll-dodge-state.md
`-- tasks.md
```

### Repository changes

```text
addon/PixelBeacon/{PixelBeacon.lua,PixelBeacon.txt}
src/pixelbus/mod.rs
src/input/mod.rs
src/weave/mod.rs
src/app/{mod.rs,routing.rs,strings.rs,ui.rs}
src/main.rs
tests/{beacon.rs,pixelbus.rs,input_engine.rs,weave_engine.rs,app_view_model.rs,app_ui_sizing.rs}
docs/ESO-Weave-Specification.md
README.md
CHANGELOG.md
```

**Structure Decision**: Define the wire enum in `pixelbus`, store the last value
in WeaveEngine for worker and UI use, and publish the gate through InputEngine.
Use a reusable atomic gate core with semantically typed life and roll handles.

## Complexity Tracking

No constitution violations require justification.

## Key Decisions

1. Ability 28549 gained/faded events are authoritative; no movement inference is added.
2. A fixed 1,500 ms watchdog recovers the documented gained-only sprint rejection.
3. Protocol version 3 prevents older overlays from fabricating B23.
4. Hook, worker, and sink checks close all enqueue and execution races.
5. Roll dodge appears in Live HUD because it is a live player state.
