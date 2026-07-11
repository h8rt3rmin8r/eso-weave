# Implementation Plan: Weave Engine

**Branch**: `003-weave-engine` | **Date**: 2026-07-11 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/003-weave-engine/spec.md`

## Summary

Add a `weave` module. Its heart is a pure function `sequence_for(slot, timing)`
that turns a slot and the timing configuration into an ordered list of weave steps
(emit a mouse or key operation, or wait a duration). A `WeaveEngine` maps a
handed-off action to its slot, enforces global cooldown against an injected clock,
and runs the slot's steps through a `WeaveSink` seam. The pure sequence builder and
the cooldown logic are fully unit-testable with a mock sink and a virtual clock; no
real input or real waiting is needed to verify correctness. Two thin integrations
extend the S002 Input Engine: mouse synthesis (primary and secondary buttons) and
per-action activity so inactive slots pass through.

## Technical Context

**Language/Version**: Rust 1.96.0, edition 2021 (unchanged).

**Primary Dependencies**: None new for the core. Mouse synthesis reuses the
existing target-specific `windows-sys` (SendInput mouse events) and `evdev`
(uinput BTN_LEFT/BTN_RIGHT) dependencies.

**Storage**: Slot and timing configuration persist through the S001 Config Store
as additive `skills` and `timing` settings sections (backward compatible, no schema
bump), consistent with the S002 `bindings` section.

**Testing**: `cargo test`. The `sequence_for` builder and cooldown gating are
covered by unit tests with a `MockSink` (records ordered steps) and a virtual
clock. The real sink and real waiting are thin.

**Target Platform**: Windows 10 and 11 x64, Linux x64.

**Project Type**: Single desktop-application crate (unchanged).

**Performance Goals**: Sequence building is O(steps). Execution runs on the worker
thread and never on the interception path.

**Constraints**: All timed work runs on the worker (constitution and spec section
6.2). Cooldown gating uses a monotonic clock seam. Configuration is settings-only
and text-hygienic.

**Scale/Scope**: Seven slots, four weave types, four global timing values.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

- **I. Spec-Driven Development**: PASS. Derived from `spec.md` (master spec section
  7.1 to 7.3), bounded by `docs/plans/plan-001.md`.
- **II. Safety-Critical Surfaces**: PASS. The engine adds no interception-path
  work; all timed sequence execution runs on the worker. The one input-layer
  change (per-action activity consulted in `classify`) preserves the existing
  safety tests and adds a test for inactive pass-through. No blocking is added to
  the hook path.
- **III. Test-First With Explicit Seams**: PASS. `WeaveSink` and the clock are the
  seams; `sequence_for` is a pure function. Tests precede implementation.
- **IV. CI Parity Before Every Commit**: PASS on the host; the mouse-synthesis
  additions to the Linux backend are type-checked with the linux target as in
  S002.
- **V. Bounded Scope: Outside The Game**: PASS. Only OS-level input synthesis while
  focused; no memory, network, or in-game access.
- **Platform and Text Hygiene Constraints**: PASS. Additive settings-only sections.

No violations. Complexity Tracking is empty.

## Project Structure

### Documentation (this feature)

```text
specs/003-weave-engine/
├── plan.md, research.md, data-model.md, quickstart.md
├── contracts/
│   ├── weave-engine.md    # WeaveSink seam, sequence_for, WeaveEngine
│   └── weave-settings.md   # additive skills and timing sections
├── checklists/{requirements.md, weave-engine.md}
├── spec.md
└── tasks.md
```

### Source Code (repository root)

```text
src/weave/
├── mod.rs        # WeaveEngine (action -> slot, cooldown gating), WeaveSink trait, RealSink, execution
├── types.rs      # WeaveType, SkillSlot, WeaveConfig, TimingConfig, WeaveStep, InputOp
└── sequence.rs   # sequence_for(slot, timing) -> Vec<WeaveStep>, the four weave sequences
src/input/mod.rs  # add MouseButton, per-action activity (set_action_active, classify consults it)
src/input/{windows.rs, linux.rs, mock.rs}  # add mouse synthesis
tests/
├── weave_sequence.rs  # sequence correctness and per-slot overrides (pure)
└── weave_engine.rs    # cooldown gating and action mapping with MockSink and virtual clock
```

The `config` module gains additive `skills` and `timing` sections on `Settings`.

**Structure Decision**: The correctness-critical logic is the pure
`sequence_for` builder and the cooldown gate, both platform-agnostic and tested
with a mock sink and a virtual clock. The `RealSink` (real synthesis plus real
sleep) and the OS mouse-synthesis code are thin adapters, mirroring the S002
pattern of a maximal testable core and a minimal untestable OS surface.

## Complexity Tracking

No constitution violations. No entries.
