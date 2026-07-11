# Implementation Plan: Fishing Controller

**Branch**: `007-fishing-controller` | **Date**: 2026-07-11 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/007-fishing-controller/spec.md`

## Summary

Add a `fishing` module with a pure, event-and-tick-driven state machine. A
`BiteDetector` trait emits the five typed events; a `StubDetector` drives tests
and a `PixelBusDetector` adapts the Pixel Bus Reader's events (dropping Latency)
in production. The `FishingController` consumes detector events plus a clock tick
and drives the Disabled, Armed, Waiting, Reeling, Recast state machine, evaluating
arm_timeout, reel_delay, and recast_delay as deadlines against an injected
millisecond clock (no blocking sleeps). The interact key is emitted through a
`FishingSink` seam (a key press then release), with a mock sink that records
operations and a real sink that drives the input engine's `InputBackend`. On
SignalLost from any active state the controller disables fishing and cancels any
pending interact, so it never blind-fires. Fishing timing and the interact key
persist as an additive `fishing` settings section. A `Key::E` variant is added to
the input engine for the default interact key.

## Technical Context

**Language/Version**: Rust 1.96.0, edition 2021 (unchanged).

**Primary Dependencies**: Reuses the input engine (`Key`, `Transition`,
`InputBackend`) from feature 002 and the Pixel Bus Reader (`PixelBusEvent`,
`PixelBusReader`, `SurfaceSampler`) from feature 005. No new crates. Adds a
`Key::E` variant to `src/input/key.rs` for the default interact key.

**Storage**: User settings only. A new additive opaque `fishing` settings section
(three timing parameters and the interact key) is added, mirroring the existing
`timing`, `skills`, and `beacon` sections, so no config `schema_version` bump.

**Testing**: `cargo test`. The controller state machine, the deadline timing, the
signal-loss safety, the config load/store, and the pure event mapping are unit
tested with a `StubDetector`, a `MockFishingSink`, and an injected clock. The
`PixelBusDetector` polling path is thin over the already-tested reader.

**Target Platform**: Windows 10 and 11 x64, Linux x64 (unchanged; the controller
is platform-agnostic).

**Project Type**: Single desktop-application crate (unchanged).

**Performance Goals**: A handful of state transitions and at most one deadline
check per tick; not performance sensitive.

**Constraints**: Safety-critical. SignalLost disables fishing and cancels pending
interacts; no interact is ever emitted while Disabled or after leaving the state
that scheduled it. All enforced in the pure state machine with required tests.

**Scale/Scope**: Five states, five detector events, three timing parameters, one
interact key, two sink implementations, one detector adapter.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

- **I. Spec-Driven Development**: PASS. Derived from `spec.md` (master spec section
  8), bounded by `docs/plans/plan-001.md` slice 007.
- **II. Safety-Critical Surfaces**: PASS and central. Fishing degrades to Disabled
  on SignalLost rather than firing inputs blind, and pending interacts are
  cancelled on any transition out of the scheduling state. Both are pure logic
  with required, non-weakened tests.
- **III. Test-First With Explicit Seams**: PASS. The `BiteDetector` trait (with a
  stub), the `FishingSink` seam (with a mock), and the injected clock are the
  seams; the controller is pure. Tests precede implementation.
- **IV. CI Parity Before Every Commit**: PASS on the host; the platform-agnostic
  controller needs no target-specific code, and the Linux target is type-checked
  as in prior slices.
- **V. Bounded Scope: Outside The Game**: PASS. The controller only synthesizes the
  interact key through the input engine; no memory, network, or gameplay access.
  The fishing module depends on the input engine and the reader, not the weave
  engine.
- **Platform and Text Hygiene Constraints**: PASS. Settings remain user-only and
  additive; new text is UTF-8 without BOM, LF, no em/en dashes.

No violations. Complexity Tracking is empty.

## Project Structure

### Documentation (this feature)

```text
specs/007-fishing-controller/
├── plan.md, research.md, data-model.md, quickstart.md
├── contracts/
│   └── fishing.md   # DetectorEvent, BiteDetector, FishingConfig, FishingSink, FishingController, PixelBusDetector
├── checklists/{requirements.md, state-machine-safety.md}
├── spec.md
└── tasks.md
```

### Source Code (repository root)

```text
src/fishing/
├── mod.rs        # DetectorEvent, BiteDetector trait, StubDetector, FishingState,
│                 # FishingConfig (+load/store, defaults), FishingSink trait,
│                 # MockFishingSink, RealFishingSink, FishingController
│                 # (new, state, set_enabled, on_event, tick), interact emission
├── detector.rs   # map_event (pure PixelBusEvent -> Option<DetectorEvent>) and
│                 # PixelBusDetector (wraps PixelBusReader + SurfaceSampler)
tests/
└── fishing.rs    # cast-reel-recast cycle, signal-loss safety, arm/recast timeouts,
                  # toggle idempotency, deadline cancellation, config round-trip, map_event
```

Also modified: `src/input/key.rs` (add `Key::E`), `src/config/mod.rs` (additive
`fishing` settings section), `src/lib.rs` (`pub mod fishing;`).

**Structure Decision**: The controller and its timing are pure and fully tested
against a `StubDetector`, a `MockFishingSink`, and an injected clock, mirroring the
maximal-testable-core pattern of the weave engine and the reader. The
`PixelBusDetector` is a thin adapter over the already-tested reader; only its
`map_event` mapping carries logic and is unit-tested directly.

## Complexity Tracking

No constitution violations. No entries.
