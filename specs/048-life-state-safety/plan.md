# Implementation Plan: Life State Safety

**Branch**: `codex/048-life-state-safety` | **Date**: 2026-09-05 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/048-life-state-safety/spec.md`

## Summary

Close #53, #54, #55, and #58 by publishing authoritative player life state in a
new PixelBeacon B21 block, decoding and routing it as one typed state, enforcing
Alive at every synthesis boundary, presenting the value in Live HUD, and turning
the renamed System and State panel into a persisted accessible disclosure.

## Technical Context

**Language/Version**: Rust 1.89, Lua 5.1-compatible ESO addon code, egui 0.36

**Primary Dependencies**: Existing eframe/egui, serde, PixelBeacon, input traits

**Storage**: Existing `config.json` UI preferences only

**Testing**: Rust unit and integration tests, headless egui rendered-frame tests,
cross-language embedded-addon contract tests

**Target Platform**: Windows 10/11 x64 and Linux x64 companion, ESO addon API 101050+

**Performance Goals**: No new work on the input hook thread beyond one atomic
load; one life-state poll joins the existing one-second addon tick

**Constraints**: Fail closed; no stale replay; no new dependency; UTF-8 without
BOM and LF; no changes to Skills; no world-transition or roll-dodge scope

**Scale/Scope**: One addon block, one typed reader signal, four gate consumers,
one Live HUD row, one persisted disclosure preference

## Constitution Check

*GATE: Passed before research and re-checked after design and implementation.*

- **Spec-first traceability**: PASS. Four atomic issues map to one cohesive S048
  specification, checklists, research, model, contract, tasks, and analysis.
- **Safety invariants**: PASS. The gate adds fail-closed coverage without weakening
  recursion breaking, focus scoping, hook-thread handoff, fishing SignalLost, or
  managed-addon filesystem protections.
- **Test-first delivery**: PASS. Contract and behavior tests precede implementation.
- **CI parity**: PASS. Formatting, Clippy with warnings denied, and the complete
  locked test suite passed locally on 2026-09-05.
- **Bounded scope**: PASS. Detection stays in the addon screen signal and never
  reads game memory or traffic.
- **Configuration discipline**: PASS. Disclosure is a user preference in the UI
  settings object; life state remains runtime-only.
- **Text hygiene**: PASS by inspection, with an automated final audit required.

Post-implementation re-check: PASS. One authoritative enum feeds distributed
enforcement at unavoidable synthesis boundaries. No new global coordinator or
dependency was introduced, and test-first coverage verifies fail-closed recovery.

## Project Structure

### Documentation

```text
docs/plans/plan-018.md
specs/048-life-state-safety/
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
│   └── life-state-and-gate.md
└── tasks.md
```

### Repository changes

```text
addon/PixelBeacon/{PixelBeacon.lua,PixelBeacon.txt}
src/{pixelbus,input,weave,fishing,potion}/
src/app/{mod.rs,routing.rs,settings_form.rs,strings.rs,ui.rs}
tests/{pixelbus.rs,beacon.rs,weave_engine.rs,fishing.rs,potion.rs,app_*.rs}
CHANGELOG.md
```

**Structure Decision**: Keep the normalized life state in `pixelbus`, the existing
home of game-derived observables. Routing distributes that one value to existing
consumer-owned safety gates. UI disclosure state remains a normal UI preference.

## Complexity Tracking

No constitution violations require justification.
