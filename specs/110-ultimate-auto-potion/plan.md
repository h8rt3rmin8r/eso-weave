# Implementation Plan: Ultimate Auto Potion Resource Watch

**Branch**: `codex/s110-ultimate-auto-potion` | **Date**: 2026-09-17 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/110-ultimate-auto-potion/spec.md`

## Summary

Extend the existing Auto Potion resource disjunction with one independently persisted Ultimate watch. Feed the rule from the current atomic `UltimateTelemetry`, compare current and maximum exactly with widened integer cross multiplication, fail closed on unavailable evidence, append Ultimate to deterministic cause order, and expose the same bounded settings control and documentation as the existing watches. Preserve every existing lifecycle, input, quickslot, cooldown, retry, and synthesis gate.

## Technical Context

**Language/Version**: Rust 1.96, edition 2021; Markdown; JSON

**Primary Dependencies**: Existing serde and serde_json settings path, egui settings UI, Pixel Bus `UltimateTelemetry`, Auto Potion controller and native action executor

**Storage**: Existing opaque `Settings::potion` JSON object with one additive `ultimate` watch

**Testing**: Rust unit and integration tests, existing documentation policy and mdBook checks

**Target Platform**: Windows 10 and 11 x64 and Linux x64

**Project Type**: Single-crate desktop application with embedded documentation

**Performance Goals**: Constant-time four-watch evaluation with no allocation, new timer, thread, sampling work, or input path

**Constraints**: Exact inclusive ratio math, unreadable evidence fails closed, deterministic OR order, UTF-8 without BOM, LF, no forbidden dashes

**Scale/Scope**: One configuration field, one existing telemetry input, one settings row, one trigger enum variant, focused tests, canonical documentation, and build-plan lifecycle records

## Constitution Check

*GATE: Passed before Phase 0 research and re-checked after Phase 1 design.*

| Principle | Result | Evidence |
| --- | --- | --- |
| I. Spec-driven development | PASS | Issue #173, Plan 046, and the complete S110 specification, clarification, requirements checklist, safety checklist, plan, research, data model, contract, quickstart, tasks, and analysis packet precede implementation. |
| II. Safety-critical surfaces | PASS | Ultimate joins only the pure resource predicate. Autonomous synthesis still uses `NativeActionExecutor`, and all focus, lifecycle, input, quickslot, cooldown, and retry gates remain mandatory. |
| III. Test-first with explicit seams | PASS | Failing ratio, migration, mixed-watch, routing, status, and settings tests precede production edits. Existing `PotionInputs`, sink, and Pixel Bus event seams require no platform dependency. |
| IV. CI parity before every commit | PASS | Format, strict Clippy, and the full locked suite run before commit, with focused documentation and text-hygiene gates. |
| V. Bounded desktop scope | PASS | S110 consumes existing local screen evidence and does not change addon protocol, game memory, networking, telemetry, or action vocabulary. |
| Autopilot and publication | PASS | The user explicitly authorized automatic push and official pull-request publication for S110. Hosted CI and no more than two Codex review rounds remain merge gates. |
| Pinned artifact rule | PASS | No pinned artifact changes are planned. |

No constitutional exception or complexity justification is required.

## Project Structure

### Documentation for this feature

```text
specs/110-ultimate-auto-potion/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── checklists/
│   ├── auto-potion-safety.md
│   └── requirements.md
├── contracts/
│   └── ultimate-watch.md
└── tasks.md
```

### Repository changes

```text
src/potion/mod.rs
src/main.rs
src/app/mod.rs
src/app/strings.rs
src/app/ui.rs
tests/potion.rs
tests/app_view_model.rs
tests/app_strings.rs
tests/app_ui_sizing.rs
docs/src/features/auto-potion.md
docs/src/reference/settings.md
docs/src/development/state-machines.md
docs/src/development/test-strategy.md
docs/src/development/coverage-matrix.md
docs/project/build-plans/plan-046.md
docs/project/build-plans/README.md
docs/archive/build-plans/plan-045.md
docs/archive/build-plans/README.md
CHANGELOG.md
```

**Structure Decision**: Keep normalization inside the pure Auto Potion rule rather than changing `ResourceSet`, because Ultimate is an exact current and maximum pair with its own existing telemetry authority. Pass the atomic observation beside `ResourceSet` in `PotionReadings`, sourced from the already current `WeaveEngine` cache. This avoids a duplicate pipeline and keeps presentation retention isolated from action evidence.

## Delivery Phases

1. Archive completed Plan 045, establish Plan 046, and complete the full S110 spec-kit packet.
2. Add failing pure-rule tests for Ultimate equality, boundaries, unavailable evidence, exact non-divisible math, mixed watches, and deterministic causes.
3. Add failing migration, round-trip, routing, status, settings-label, and sizing tests.
4. Extend the configuration and pure controller rule, then wire existing Ultimate telemetry into the worker tick.
5. Add the fourth settings row, status name, diagnostics, documentation, and changelog entry.
6. Run focused gates, complete post-implementation analysis, then execute full CI parity and repository text checks.
7. Push, publish the official pull request, process every hosted review comment, trigger at most one second Codex review, and stop for operator merge.

## Complexity Tracking

No constitution violation is present.
