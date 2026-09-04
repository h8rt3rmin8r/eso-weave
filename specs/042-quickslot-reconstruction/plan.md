# Implementation Plan: Quickslot Observation Reconstruction

**Branch**: `codex/042-quickslot-reconstruction` | **Date**: 2026-09-03 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/042-quickslot-reconstruction/spec.md`

## Summary

Replace cooldown-derived potion inference with one explicit B20 quickslot
classification, attach the existing cooldown and optional identity facts, add a
bounded in-client diagnostic receipt, expose truthful view states, broaden update
convergence, and keep auto-potion fail-closed for issue #25.

## Technical Context

**Language/Version**: Rust 2021 edition and ESO Lua API 101050

**Primary Dependencies**: egui/eframe desktop UI, Windows screen capture, ESO addon API

**Storage**: Existing JSON settings only; no new persistence

**Testing**: Rust unit and integration tests, Lua source contract parsing, policy checks

**Target Platform**: Windows desktop companion plus ESO live addon

**Project Type**: Desktop application with bundled game addon

**Performance Goals**: One application event and zero repeated diagnostic lines for unchanged observations

**Constraints**: Fail closed, no localized parsing, no new input authorization, one additional beacon block, no foreground game launch

**Scale/Scope**: One protocol block, one state model, one status-view group, one diagnostic command, issue #24 only

## Constitution Check

*GATE: Passed before Phase 0 research and re-checked after Phase 1 design.*

- Specification and clarifications completed before implementation.
- TDD is mandatory for protocol, view, routing, and safety behavior.
- Outside-game and signal-loss behavior remain explicit and fail closed.
- Lua/Rust constants and geometry remain mechanically cross-checked.
- No new platform dependency or unsafe input path is introduced.
- Real-client evidence is separated honestly from headless automated evidence.
- Files use UTF-8 without BOM, LF endings, and no em/en dash punctuation.

## Project Structure

### Documentation (this feature)

```text
specs/042-quickslot-reconstruction/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── quickslot-observation.md
├── checklists/
│   ├── requirements.md
│   └── runtime-safety.md
└── tasks.md
```

### Source Code

```text
addon/PixelBeacon/
├── PixelBeacon.lua
└── PixelBeacon.txt
src/
├── pixelbus/mod.rs
├── app/mod.rs
├── app/strings.rs
├── app/ui.rs
├── potion/mod.rs
└── weave/mod.rs
tests/
├── beacon.rs
├── pixelbus.rs
├── app_view_model.rs
├── potion.rs
└── weave_engine.rs
```

**Structure Decision**: Extend the existing addon-to-reader pipeline and normalized
view. Do not add a subsystem or third-party dependency.

## Design Sequence

1. Pin the explicit B20 contract and negative compatibility outcomes in tests.
2. Introduce discriminated Rust state and decoding while keeping automation gated.
3. Reconstruct Lua fact collection and classification, then render B20.
4. Add opt-in snapshot and change-only diagnostic commands.
5. Expand event convergence and normalized status presentation.
6. Run the complete local gate, inspect the diff independently, then publish.

## Complexity Tracking

No constitution violations require justification.
