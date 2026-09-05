# Implementation Plan: World Transition State

**Branch**: `codex/049-world-transition-state` | **Date**: 2026-09-05 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/049-world-transition-state/spec.md`

## Summary

Close #56 by publishing Unknown, Transitioning, and Active in a new PixelBeacon
B22 block, making Active follow a complete activation baseline, decoding and
routing the state into shared game observations, and displaying it in System and State.

## Technical Context

**Language/Version**: Rust 1.89, Lua 5.1-compatible ESO addon code, egui 0.36

**Primary Dependencies**: Existing eframe/egui, PixelBeacon, game observation model

**Storage**: None; world state is runtime-only

**Testing**: Rust integration tests, embedded-addon contract tests, headless view tests

**Target Platform**: Windows 10/11 x64 and Linux x64 companion, ESO addon API 101050+

**Performance Goals**: No new timer, thread, OS query, or hook-thread work

**Constraints**: Fail to Unknown; event-owned lifecycle; Active after complete
baseline; no synthesis-gate change; UTF-8 without BOM and LF

**Scale/Scope**: One addon block, one typed reader event, one shared observation
field, one System and State row, one issue

## Constitution Check

*GATE: Passed before research, after design, and after implementation.*

- **Spec-first traceability**: PASS. Issue #56 maps to build plan 019 and a
  complete S049 specification, checklists, research, model, contract, and tasks.
- **Safety invariants**: PASS. Existing input, fishing, and addon-deletion safety
  paths are unchanged; invalid evidence becomes Unknown.
- **Test-first delivery**: PASS by plan. Failing wire, lifecycle, routing, and view
  tests precede product changes.
- **CI parity**: PASS. Format, all-target/all-feature lint, and the complete
  locked test suite pass.
- **Bounded scope**: PASS. State crosses only the permitted screen-signal contract.
- **Configuration discipline**: PASS. No runtime state is persisted.
- **Text hygiene**: PASS by specification inspection, with automated final audit required.

Post-implementation re-check: PASS. The delivered B22 contract, lifecycle
ordering, companion fallback, shared routing, and System and State presentation
match this plan without expanding into #59 synthesis gating.

## Project Structure

### Documentation

```text
docs/plans/plan-019.md
specs/049-world-transition-state/
|-- spec.md
|-- plan.md
|-- research.md
|-- data-model.md
|-- quickstart.md
|-- analysis.md
|-- checklists/
|   |-- requirements.md
|   `-- lifecycle.md
|-- contracts/
|   `-- world-transition-state.md
`-- tasks.md
```

### Repository changes

```text
addon/PixelBeacon/{PixelBeacon.lua,PixelBeacon.txt}
src/pixelbus/mod.rs
src/game/mod.rs
src/app/{mod.rs,routing.rs,strings.rs,ui.rs}
tests/{beacon.rs,pixelbus.rs,game_state.rs,app_view_model.rs}
docs/ESO-Weave-Specification.md
README.md
CHANGELOG.md
```

**Structure Decision**: Define the wire enum in `pixelbus`, store the latest
normalized value in the existing shared `GameState`, and derive UI presentation
in `app`. Do not add a controller or a second synchronization primitive.

## Complexity Tracking

No constitution violations require justification.

## Key Decisions

1. A dedicated B22 avoids conflating world readiness with heartbeat or life.
2. Negotiated-header version 2 positively identifies the 23-cell payload; version
   1 stays geometry-readable with 22 cells and never samples B22.
3. `0xCC` is the strongest remaining marker placement under the current registry.
4. A named complete baseline function makes the Active ordering durable.
5. Synthesis consumption remains in #59 so S049 closes one atomic observable.
