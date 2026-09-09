# Implementation Plan: Death Recovery Safety

**Branch**: `codex/s067-death-recovery-safety` | **Date**: 2026-09-09 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/067-death-recovery-safety/spec.md`

## Summary

Close #109 by replacing immediate post-death Alive publication with an addon-side
death recovery arbiter, extending B21 with diagnostic recovery paths, reopening
the shared life gate only after a coherent recovery capture, invalidating stale
controller work, and starting a new auto-potion retry episode.

## Technical Context

**Language/Version**: Rust 1.96.0, Lua 5.1-compatible ESO addon code, egui 0.36

**Primary Dependencies**: Existing pixelbus, input, weave, fishing, potion,
eframe/egui, and tracing modules; no new dependency

**Storage**: N/A, all death and recovery state is runtime-only

**Testing**: Rust unit and integration tests, embedded Lua source contract tests,
headless view-model tests, documentation policy tests

**Target Platform**: Windows 10/11 x64 and Linux x64 companion, ESO addon API
101050 and 101054

**Project Type**: Single-crate desktop companion plus managed ESO UI addon

**Performance Goals**: Constant-time hook classification; no new thread; recovery
queries join existing lifecycle handlers and one-second convergence tick

**Constraints**: Fail closed; no time-only authorization; no stale replay; no
new payload block; no settings or persistence; UTF-8 without BOM and LF; no
em-dashes or en-dashes

**Scale/Scope**: One addon lifecycle arbiter, one existing wire block, one reader
ordering rule, three controller families, HUD/log diagnostics, issue #109 only

## Constitution Check

*GATE: Passed before research and design. Re-check after implementation.*

- **Spec-first traceability**: PASS. Issue #109 maps to the complete S067
  specification, checklists, research, model, contract, quickstart, tasks, and
  blocking analysis.
- **Safety invariants**: PASS. The design strengthens life-state authorization
  without weakening recursion breaking, focus scoping, hook-thread handoff,
  signal-loss behavior, or managed addon filesystem protections.
- **Test-first delivery**: PASS planned. Focused failing tests precede addon,
  reader, input, fishing, potion, and presentation changes.
- **CI parity**: PASS planned. Formatting, Clippy, and locked tests run in the
  foreground before each buildable commit.
- **Bounded scope**: PASS. ESO evidence remains inside the managed addon and
  crosses only the existing pixel bus.
- **Configuration discipline**: PASS. No session or recovery data is persisted.
- **Text hygiene**: PASS by design, with automated final audit required.

## Project Structure

### Documentation

```text
docs/project/build-plans/plan-037.md
docs/archive/build-plans/plan-036.md
specs/067-death-recovery-safety/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── checklists/
│   ├── requirements.md
│   └── runtime-safety.md
├── contracts/
│   └── death-recovery.md
└── tasks.md
```

### Repository changes

```text
addon/PixelBeacon/{PixelBeacon.lua,PixelBeacon.txt}
src/pixelbus/mod.rs
src/input/mod.rs
src/app/{mod.rs,routing.rs}
src/potion/mod.rs
tests/{beacon.rs,pixelbus.rs,input_engine.rs,weave_engine.rs,fishing.rs,potion.rs,app_routing.rs,app_view_model.rs}
docs/src/{concepts,development,reference}/
CHANGELOG.md
```

**Structure Decision**: Keep recovery authority beside ESO lifecycle evidence in
PixelBeacon, keep decoding and sample ordering in pixelbus, reuse S060 atomic
gates and epochs, and reset autonomous controller state in its current owner.

## Complexity Tracking

No constitution violations require justification.
