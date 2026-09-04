# Implementation Plan: Auto-potion Restoration

**Branch**: `codex/043-auto-potion-restoration` | **Date**: 2026-09-03 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/043-auto-potion-restoration/spec.md`

## Summary

Remove the temporary S042 consumer gate, rebuild the controller result as a typed effective state, preserve the session-only request across lifecycle loss, require positive beacon and quickslot evidence, expose every current blocker in the main view, and verify the exact down/up and retry behavior with deterministic tests.

## Technical Context

**Language/Version**: Rust 2021 edition, minimum Rust 1.96

**Primary Dependencies**: egui/eframe UI, tracing, existing platform input backend, existing PixelBus observation pipeline

**Storage**: Existing JSON settings for thresholds and key binding; requested enablement remains session-only

**Testing**: Rust unit and integration tests, policy checks, cargo formatting, Clippy, and rustdoc

**Target Platform**: Windows desktop companion, with existing Linux compilation support retained

**Project Type**: Desktop application with a bundled game addon

**Performance Goals**: One controller evaluation per existing worker iteration, one diagnostic per state transition, and at most one input attempt per retry interval

**Constraints**: Fail closed, no new protocol block, no parallel detection path, no persistent enablement, no foreground game launch, no input without explicit usable-potion classification

**Scale/Scope**: One controller, one routing integration, one main-view status, focused tests, issue #25 only

## Constitution Check

*GATE: Passed before Phase 0 research and re-checked after Phase 1 design.*

- Specification, clarification review, and safety checklists completed before implementation.
- Test-first changes are mandatory for the trigger rule, controller lifecycle, routing, and view state.
- Outside-game, unfocused, signal-loss, suspended, gated, stale, and unknown paths fail closed.
- S041 game/focus facts and S042 quickslot facts remain authoritative.
- The temporary compile-time consumer gate is removed only after executable safety tests are in place.
- The change introduces no new dependency, protocol block, unsafe platform path, or persistent automatic behavior.
- Real-client evidence remains distinct from deterministic evidence and is deferred until a fresh release.
- Files use UTF-8 without BOM, LF endings, and no em or en dash punctuation.

## Project Structure

### Documentation (this feature)

```text
specs/043-auto-potion-restoration/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── auto-potion-state.md
├── checklists/
│   ├── requirements.md
│   └── runtime-safety.md
└── tasks.md
```

### Source Code

```text
src/
├── potion/mod.rs
├── app/mod.rs
├── app/routing.rs
├── app/strings.rs
├── app/ui.rs
└── main.rs
tests/
├── potion.rs
├── app_view_model.rs
└── app_ui_sizing.rs
```

**Structure Decision**: Extend the existing auto-potion controller, routing, normalized application view, and worker integration. Do not add a subsystem or duplicate the S041/S042 observation models.

## Design Sequence

1. Add failing tests for requested versus effective state, every blocker, deterministic trigger cause, signal-loss preservation, state transitions, and exact synthesis.
2. Replace the binary rule result with typed outcomes while retaining a pure evaluation seam.
3. Track beacon availability and change-only effective state in the controller.
4. Route heartbeat and signal loss into that lifecycle state without changing requested enablement.
5. Remove the temporary consumer gate and run the controller through the existing input sink.
6. Add a normalized view value and render the requested setting beside the effective state.
7. Update architecture and release notes for the intentional S039 lifecycle correction.
8. Run the full local gate, inspect the diff, push, publish the pull request, and complete the authorized review protocol.

## Complexity Tracking

No constitution violations require justification.
