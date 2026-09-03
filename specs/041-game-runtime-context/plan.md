# Implementation Plan: Game Runtime and Context Truth

**Branch**: `codex/041-game-runtime-context` | **Date**: 2026-09-03 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/041-game-runtime-context/spec.md`

## Summary

Close issues #22 and #23 by extracting game presence from the beacon manager,
adding provider-aware installation and runtime observations, deriving a truthful
Game Context from independent runtime, focus, freshness, and surface evidence,
and clearing downstream observations when the game is inactive. The existing
pixel-bus worker polls and routes lifecycle state once per second, so no new
thread or input path is introduced.

## Technical Context

**Language/Version**: Rust 1.96 edition 2021, plus existing Markdown artifacts

**Primary Dependencies**: existing `windows-sys`, `serde_json`, `dirs`, and
`x11rb`; no new crate

**Storage**: no persisted runtime state; provider manifests, registry metadata,
and process/window observations are read-only

**Testing**: unit and integration tests through `cargo test --all --locked`,
plus headless egui rendered-frame coverage

**Target Platform**: Windows 10/11 x64 and Linux x64 through Steam Proton

**Project Type**: single-crate desktop application with an embedded addon

**Performance Goals**: state convergence within two seconds; one-second
presence cadence; zero repeated normal logs across 20 unchanged polls

**Constraints**: no work on the hook thread, no game memory or network
inspection, no new input synthesis path, no personal paths in normal logs,
UTF-8 without BOM, LF, no em-dashes or en-dashes

**Scale/Scope**: one new domain module with two platform adapters, one moved VDF
parser, the existing worker and routing seams, the main view model and status
grid, focused tests, build plan, master specification, README, and changelog

## Constitution Check

*GATE: Passed before Phase 0 research and re-checked after Phase 1 design.*

| Principle | Status | Evidence |
| --- | --- | --- |
| I. Spec-Driven Development | PASS | S041 has a complete spec-kit artifact set, traces to issues #22 and #23, and runs analyze before implementation. |
| II. Safety-Critical Surfaces | PASS | Runtime and focus become explicit safety-negative gates; existing recursion, focus, hook-thread, fishing, and addon lifecycle tests remain mandatory. |
| III. Test-First With Explicit Seams | PASS | Provider reconciliation, runtime reduction, context projection, and transitions are pure; platform adapters are thin and fixture-driven. |
| IV. CI Parity Before Every Commit | PASS | Fmt, clippy with warnings denied, and the locked full suite run in the foreground before commit. |
| V. Bounded Scope: Outside The Game | PASS | The feature reads provider metadata, process lists, foreground-window metadata, and screen pixels only. It reads no game memory or network traffic. |

**Post-design re-check**: PASS. The new module corrects an ownership boundary
without creating another crate, dependency, thread, or synthesis path.

## Project Structure

### Documentation (this feature)

```text
specs/041-game-runtime-context/
├── checklists/
│   ├── requirements.md
│   └── runtime-safety.md
├── contracts/
│   ├── game-context.md
│   └── game-observation.md
├── data-model.md
├── plan.md
├── quickstart.md
├── research.md
├── spec.md
└── tasks.md
```

### Source Code (repository root)

```text
src/
├── game/
│   ├── linux.rs
│   ├── mod.rs
│   ├── steam.rs
│   └── windows.rs
├── app/
│   ├── mod.rs
│   ├── routing.rs
│   ├── strings.rs
│   └── ui.rs
├── beacon/
│   ├── linux.rs
│   ├── mod.rs
│   └── windows.rs
├── input/
│   └── mod.rs
├── pixelbus/
│   └── mod.rs
├── potion/
│   └── mod.rs
├── weave/
│   └── mod.rs
├── lib.rs
└── main.rs

tests/
├── app_ui_sizing.rs
├── app_view_model.rs
├── beacon.rs
├── game_state.rs
├── input_engine.rs
├── pixelbus.rs
└── potion.rs
```

**Structure Decision**: `src/game/` becomes the platform-neutral ownership
boundary. The beacon manager delegates its compatibility probe. The existing
pixel-bus worker performs the bounded poll and routes transitions in one lock
order. Presentation consumes an immutable snapshot.

## Implementation Outline

1. Add pure installation reconciliation, runtime reduction, observation state,
   context projection, and transition tests.
2. Move the Steam parser and add Windows/Linux discovery, process, and focus
   adapters with sanitized fixtures.
3. Add game-active safety gates and lifecycle clearing under failing tests.
4. Make menu decoding explicit about invalid evidence and route freshness and
   surface observations into the shared state.
5. Wire the one-second presence poll before sampler availability checks.
6. Project installation, runtime, Game Context, and dormant metrics into the
   main view; add the focusable help affordance.
7. Update build plan, architecture specification, README troubleshooting, and
   changelog decision record.
8. Run analyze, implementation completion checks, full CI parity, and field
   validation where the host supports it.

## Key Decisions

- **D1**: Platform-owned provider evidence wins over a generic ESO uninstall
  entry for the same root.
- **D2**: Game presence moves out of the beacon manager into `src/game/`.
- **D3**: The existing pixel-bus worker owns the one-second poll and transition
  routing; no thread is added.
- **D4**: Raw observation axes remain independent and one pure function derives
  Game Context.
- **D5**: Runtime and focus are explicit fail-closed gates for interception,
  fishing, and auto-potion, while basic weaving remains independent of optional
  PixelBeacon availability.
- **D6**: Reader and downstream caches reset together when runtime leaves Active.
- **D7**: Game Context replaces Game Menu because the field includes more than
  operating-system focus.

## Complexity Tracking

No constitution violations require justification.
