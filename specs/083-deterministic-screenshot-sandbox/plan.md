# Implementation Plan: Deterministic Screenshot Sandbox

**Branch**: `codex/s083-deterministic-screenshot-sandbox` | **Date**: 2026-09-11 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/083-deterministic-screenshot-sandbox/spec.md`

## Summary

Add a development-only Cargo test executable that builds seven deterministic application fixtures, validates each projected view, and renders all dark/light and narrow/wide variants through the real headless frame seam. Require explicit repository-local output authority, emit a canonical manifest, and structurally exclude capture from production.

## Technical Context

**Language/Version**: Rust 1.96, Edition 2021, JSON, Markdown

**Primary Dependencies**: Existing eframe/egui and image crates; dependency-free `egui_kittest` frame driving; test-only matching `egui-wgpu` 0.36.1 and `pollster` 0.4 for offscreen rendering

**Storage**: Caller-authorized repository-local PNG and JSON output only

**Testing**: Custom no-harness test target, existing integration tests, full Cargo merge gate

**Target Platform**: Headless or hidden Windows development process, with compile parity on Linux CI

**Performance Goals**: Validate-only mode initializes no GPU; one explicit run renders 28 documentation images

**Constraints**: No native window, physical input hook, synthesis, desktop capture, addon lifecycle mutation, user directory, background worker, or production entry point

**Scale/Scope**: Seven scenes, four variants each, one manifest, one maintainer guide section

## Constitution Check

*GATE: Passed before research and re-checked after design.*

- **Spec-driven sequence**: PASS. Issue #124, Plan 039, spec, clarification, checklists, research, design, tasks, and analysis form the authority chain.
- **Safety-critical surfaces**: PASS. The target constructs no platform hook or synthesis backend, calls no addon lifecycle mutation, and asserts its action channel remains empty.
- **Test first**: PASS. The custom target failed before the support implementation was green, first at the published renderer dependency boundary and then at fixture type and state contracts.
- **CI parity**: PASS. Rust and manifest changes require the complete foreground Cargo merge gate.
- **Bounded scope**: PASS. Capture is an integration-test executable and adds no fourth addon bridge, process access, packet access, upload, or production mode.
- **Text hygiene**: PASS. New Rust, JSON, and Markdown use UTF-8 without BOM, LF, and standard hyphens.

No complexity exception is required.

## Project Structure

### Documentation for this feature

```text
specs/083-deterministic-screenshot-sandbox/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── contracts/
│   └── capture-sandbox.md
└── checklists/
    ├── requirements.md
    └── capture-safety.md
```

### Repository surfaces

```text
Cargo.toml                                  # explicit non-production test target and renderer feature
Cargo.lock                                  # locked development renderer graph
tests/documentation_capture.rs              # no-harness entry point and contract checks
tests/support/documentation_capture.rs      # scene catalog, fixtures, render and receipt logic
docs/src/development/test-strategy.md        # isolation and maintainer command
docs/project/build-plans/plan-039.md         # chronological slice status
docs/project/migration-ledger.json           # active spec evidence
CHANGELOG.md                                 # user-facing record and architecture decision
```

**Structure Decision**: Keep all fixture and renderer code under `tests/`. Production library and binary code remain unchanged, while the target can reuse public model and rendering seams.

## Phase 0: Research

1. Audit the current `frame_ui` and bundled-font harness.
2. Compare production mode, Cargo feature, and test-target isolation.
3. Inspect the pinned egui test renderer, reject its unresolvable WGPU feature, and retain the same headless design against matching egui-wgpu directly.
4. Define scene values, output containment, privacy, and same-environment determinism.

Output: [research.md](research.md)

## Phase 1: Design

1. Define the seven-scene ordered catalog and pre-render state assertions.
2. Define the two themes, two viewports, filename grammar, and manifest.
3. Define validation without a capture marker and explicit `--capture-to` rendering behavior.
4. Define production, input, screen, addon, path, and privacy isolation tests.

Outputs: [data-model.md](data-model.md), [capture sandbox contract](contracts/capture-sandbox.md), and [quickstart.md](quickstart.md)

## Phase 2: Tasks and Analysis

Generate story-ordered tasks with a failing test target before the support implementation is green. Analyze every requirement, constitution boundary, and output mutation before implementation.

## Implementation Strategy

1. Add the no-harness target and observe its failing dependency and implementation contracts.
2. Add typed scene, theme, viewport, and receipt contracts plus validation-only execution.
3. Add isolated fixture builders and scene-specific `AppView` assertions.
4. Add predictable headless rendering, PNG output, empty-action assertions, and canonical manifest publication.
5. Add path-containment and static production-isolation checks.
6. Document the command and update project governance evidence.
7. Run capture reproducibility evidence and the full merge gate.

## Complexity Tracking

The renderer adds compile weight only to development targets. This is accepted because using the pinned egui renderer avoids implementing an incomplete parallel painter and keeps production dependencies and startup unchanged.
