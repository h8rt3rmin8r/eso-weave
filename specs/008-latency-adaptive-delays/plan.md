# Implementation Plan: Latency-Adaptive Delays

**Branch**: `008-latency-adaptive-delays` | **Date**: 2026-07-11 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/008-latency-adaptive-delays/spec.md`

## Summary

Enhance the weave engine's pure sequence builder to scale `d_weave` and `d_bash`
by server latency. A small `LatencyConfig` (an `enabled` flag defaulting off and
`k` defaulting to 0.25) plus a `current_latency: Option<u16>` on the engine drive
a pure `effective_delay(base, latency, cfg)` function that returns
`base + clamp(round(k * latency), 0, 300)`. The sequence builder gains an adapted
variant that applies `effective_delay` to `d_weave` and `d_bash` only, leaving
`d_heavy` and `global_cooldown` untouched; the existing `sequence_for` delegates
to it with the feature disabled, so current behavior is byte-for-byte unchanged.
The engine exposes `set_latency(Option<u16>)` to intake reader Latency values and
clear on signal loss, and persists `LatencyConfig` in a new additive `latency`
settings section. Everything correctness-bearing is pure and unit-tested with
crafted latency and configurations.

## Technical Context

**Language/Version**: Rust 1.96.0, edition 2021 (unchanged).

**Primary Dependencies**: Extends the weave engine (feature 003). Consumes the
`u16` latency the Pixel Bus Reader (feature 005) emits via `PixelBusEvent::Latency`
(the reader is not called directly here; the worker loop that pumps events is out
of scope). No new crates.

**Storage**: User settings only. A new additive opaque `latency` settings section
(the enabled flag and `k`) is added, mirroring the existing `timing`, `skills`,
`beacon`, and `fishing` sections, so no config `schema_version` bump.

**Testing**: `cargo test`. `effective_delay`, the adapted sequence builder, the
engine's latency intake and its effect on `handle`, and the config load/store are
unit-tested with crafted latency and configurations, independent of any real
reader or clock.

**Target Platform**: Windows 10 and 11 x64, Linux x64 (unchanged; the logic is
platform-agnostic).

**Project Type**: Single desktop-application crate (unchanged).

**Performance Goals**: One multiply, round, and clamp per scaled delay; not
performance sensitive.

**Constraints**: The disabled and no-latency paths MUST reproduce the existing
weave sequences byte for byte (FR-008); only `d_weave` and `d_bash` are ever
scaled, and the effective delay is clamped to `[base, base + 300]`.

**Scale/Scope**: One pure formula, one adapted sequence builder, one engine intake,
one additive settings section, two scaled delays.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

- **I. Spec-Driven Development**: PASS. Derived from `spec.md` (master spec section
  7.4), bounded by `docs/plans/plan-001.md` slice 008.
- **II. Safety-Critical Surfaces**: PASS. This slice touches none of the named
  safety-critical surfaces. Its own key guarantee (no regression to base sequences
  when off or without latency) is covered by required tests that assert the
  disabled and no-latency paths reproduce the current sequences exactly.
- **III. Test-First With Explicit Seams**: PASS. `effective_delay` and the adapted
  builder are pure; the latency value and the config are injected. Tests precede
  implementation.
- **IV. CI Parity Before Every Commit**: PASS on the host; the logic is
  platform-agnostic and the Linux target is type-checked as in prior slices.
- **V. Bounded Scope: Outside The Game**: PASS. Pure timing arithmetic over a
  latency value already produced by the reader; no memory, network, or gameplay
  access.
- **Platform and Text Hygiene Constraints**: PASS. Settings remain user-only and
  additive; new text is UTF-8 without BOM, LF, no em/en dashes.

No violations. Complexity Tracking is empty.

## Project Structure

### Documentation (this feature)

```text
specs/008-latency-adaptive-delays/
├── plan.md, research.md, data-model.md, quickstart.md
├── contracts/
│   └── latency.md   # LatencyConfig, effective_delay, adapted sequence builder, engine intake, persistence
├── checklists/{requirements.md, computation-and-gating.md}
├── spec.md
└── tasks.md
```

### Source Code (repository root)

```text
src/weave/
├── types.rs      # add LatencyConfig (enabled, k) with defaults, and MAX_LATENCY_BONUS_MS
├── sequence.rs   # add effective_delay(base, latency, cfg) and sequence_for_adapted(slot,
│                 # timing, latency, cfg); sequence_for delegates with the feature disabled
└── mod.rs        # WeaveEngine gains latency: LatencyConfig and current_latency: Option<u16>,
│                 # set_latency(Option<u16>), latency_config accessors, handle() uses the
│                 # adapted builder, and load/store persist the additive `latency` section
src/config/mod.rs # additive `latency` settings section
tests/
└── weave_latency.rs  # effective_delay, adapted builder (scaled vs unscaled), gating,
                      # no-regression, intake effect on handle, config round-trip
```

**Structure Decision**: The adaptation lives in the pure `sequence.rs` builder,
where all weave timing already resolves, so the effective delay is computed exactly
where `d_weave` and `d_bash` are consumed and is fully unit-testable. Keeping
`sequence_for` as a thin delegate to `sequence_for_adapted` (feature disabled)
guarantees the no-regression property structurally: existing callers and tests are
unchanged. The `LatencyConfig` is a new additive settings section owned by the
weave module, isolated from the four base delays in `TimingConfig`.

## Complexity Tracking

No constitution violations. No entries.
