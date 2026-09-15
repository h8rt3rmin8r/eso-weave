# Implementation Plan: Controller Binding Consumption

**Branch**: `codex/s102-controller-binding-consumption` | **Date**: 2026-09-15 | **Spec**: `specs/102-controller-binding-consumption/spec.md`

## Summary

Complete native gameplay binding authority by moving Fishing Interact and Auto Potion Quickslot synthesis onto one shared autonomous chord executor. Remove their persisted and editable desktop gameplay keys, surface read-only native state, preserve controller-specific lifecycle behavior, and migrate legacy configuration without fallback.

## Technical Context

**Language/Version**: Rust 1.80 workspace policy; Lua 5.1-compatible addon remains unchanged

**Primary Dependencies**: std synchronization, serde/serde_json, existing native input and egui boundaries

**Storage**: User-local JSON settings with schema-compatible ignored legacy fields

**Testing**: cargo test, platform-neutral mock backends, deterministic UI capture harness, mdBook validation

**Target Platform**: Windows 10/11 and Linux desktop

**Project Type**: Cross-platform desktop application with bundled ESO addon

**Performance Goals**: Constant-time binding lookup and bounded synchronous chord emission; no new worker or polling loop

**Constraints**: Read-only ESO authority; no guessed controls; no release of physical modifiers; pre-lock invalidation; no regression to F1/F2/F3 toggles

**Scale/Scope**: Two autonomous actions, four non-valid states, 111 keyboard primaries, seven mouse primaries, four modifiers, two settings rows, one legacy migration

## Constitution Check

*GATE: Passed before research and re-checked after design.*

- **Principle I, Spec-Driven Development**: PASS. S102 is issue #208 and follows specify, clarify/checklist, plan, tasks, analyze, then implement.
- **Principle II, Safety-Critical Surfaces**: PASS. Native evidence starts unavailable, epoch replacement precedes publication, and synthesis validates gates and modifier ownership at the final boundary.
- **Principle III, Test-First**: PASS. Tasks require failing controller, sink, routing, migration, and presentation tests before implementation.
- **Principle IV, CI Parity**: PASS. All local and hosted verification gates remain mandatory.
- **Principle V, Bounded Scope**: PASS. Only Interact and Quickslot consumption, duplicate setting removal, and issue #188 completion are included.
- **Addon constraints**: PASS. PixelBeacon protocol and addon files are unchanged.
- **Pinned artifacts and text hygiene**: PASS. No dependency or pinned process artifact is planned. Text remains UTF-8 without BOM, LF, and free of forbidden dash characters.

## Design

### Shared autonomous authority

Promote the existing Fishing gate generation into an autonomous controller authority shared by Fishing and Auto Potion. It contains cloned safety gates, an atomic epoch, physical modifier state, and the coherent native binding set. Binding replacement increments both combat and autonomous epochs before publishing the new set.

### Complete chord executor

Add one input-layer executor over `InputBackend`. It captures or receives an autonomous epoch, copies only a valid requested action chord, rechecks admission around the lookup, rejects incompatible physical modifiers, presses missing modifiers in canonical order, emits the primary, immediately releases a non-momentary primary, and releases only generated modifiers in reverse order. Failures perform best-effort cleanup and return false.

Fishing retains scheduled authorization through its sink's arm boundary. Auto Potion captures current authorization for each immediate trigger. Controller sinks report success, so Auto Potion consumes retry time only after a primary down and Fishing can explain a rejected action.

### Configuration and presentation migration

Remove `interact_key` and `quickslot_key` from typed configuration and serialized output. Deserialization tolerates extra legacy fields through serde's default unknown-field behavior. Replace editable key combos with read-only formatted states sourced from `InputEngine::native_bindings`, including remediation for unavailable, unbound, conflicting, and unsupported evidence.

### Routing and lifecycle

The existing safety pre-route remains the first observer of binding replacement and signal loss. Normal routing updates controller presentation state only after the lock-free input authority is closed. Signal loss retains existing Fishing disable behavior and Auto Potion beacon block.

## Project Structure

### Specification package

```text
specs/102-controller-binding-consumption/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── controller-chord-execution.md
├── checklists/
│   └── requirements.md
├── analysis.md
└── tasks.md
```

### Repository integration

```text
src/input/mod.rs
src/fishing/mod.rs
src/potion/mod.rs
src/app/routing.rs
src/app/settings_form.rs
src/app/ui.rs
src/app/mod.rs
src/main.rs
tests/fishing.rs
tests/potion.rs
tests/input_engine.rs
tests/app_settings.rs
tests/app_view_model.rs
tests/support/documentation_capture.rs
docs/src/concepts/input-safety.md
docs/src/features/fishing.md
docs/src/features/auto-potion.md
docs/src/reference/settings.md
docs/src/getting-started/troubleshooting.md
docs/project/build-plans/plan-043.md
docs/project/build-plans/README.md
CHANGELOG.md
.specify/feature.json
specs/102-controller-binding-consumption/
```

**Structure Decision**: Extend the existing input authority and controller sink seams. The input module owns portable chord execution, controllers retain policy and lifecycle, and the application reads the same coherent evidence for presentation.

## Verification Strategy

1. Add failing autonomous authority, chord ownership, controller rejection, retry accounting, migration, and presentation tests.
2. Implement the shared executor and route both real controller sinks through it.
3. Remove duplicate fields and controls, add read-only state, and refresh deterministic captures if geometry changes.
4. Update canonical documentation, Plan 043 chronology, and release notes.
5. Run spec-kit analysis, focused tests, full CI parity, release build, docs, trust, spelling, links, encoding, and diff review.
6. Commit, push, publish the official PR, address every hosted result, request exactly one second `@Codex review` round, and stop only when green for the merge ritual.

## Complexity Tracking

No constitution violations or complexity exceptions are required.
