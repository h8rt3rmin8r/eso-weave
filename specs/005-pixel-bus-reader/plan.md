# Implementation Plan: Pixel Bus Reader

**Branch**: `005-pixel-bus-reader` | **Date**: 2026-07-11 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/005-pixel-bus-reader/spec.md`

## Summary

Add a `pixelbus` module. Pure decoders turn a sampled color into a heartbeat, a
fishing signal, or a validated latency; a `PixelBusReader` state machine turns a
timed sequence of three samples into typed events (heartbeat, signal-loss,
fishing-started, bite-detected, fishing-stopped, latency), tracking the heartbeat
timeout against an injected clock. Surface sampling sits behind a `SurfaceSampler`
seam with a mock for tests and thin GDI (Windows) and X11 (Linux) backends. The
decoders and state machine are fully unit-testable without any real sampling.

## Technical Context

**Language/Version**: Rust 1.96.0, edition 2021 (unchanged).

**Primary Dependencies**: Reuses `windows-sys` (adds the GDI feature for a
single-pixel read) and `x11rb` (already present) for the Linux surface read. No
new crates.

**Storage**: None for the reader; sampling interval, tolerance, and heartbeat
timeout are configuration passed in.

**Testing**: `cargo test`. The decoders and the reader state machine are covered
by unit tests with crafted `Rgb` samples and an injected millisecond clock. The
OS samplers are thin and validated on real hardware.

**Target Platform**: Windows 10 and 11 x64, Linux x64 (X11 or XWayland).

**Project Type**: Single desktop-application crate (unchanged).

**Performance Goals**: Sampling three points per tick at up to 10 Hz; decoding is
trivial arithmetic.

**Constraints**: Loss of heartbeat must promptly raise signal-loss (safety
surface). Decoding is pure; no real sampling on the tested path.

**Scale/Scope**: Three sample points, six event kinds, one state machine.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

- **I. Spec-Driven Development**: PASS. Derived from `spec.md` (master spec
  section 9.3, reader side), bounded by `docs/plans/plan-001.md`.
- **II. Safety-Critical Surfaces**: PASS and central. Signal-loss on heartbeat
  timeout is the safety behavior; it is implemented in the pure state machine and
  covered by required tests so the later fishing controller disables fishing
  rather than acting on stale state.
- **III. Test-First With Explicit Seams**: PASS. `SurfaceSampler` and the injected
  clock are the seams; the decoders and state machine are pure. Tests precede
  implementation.
- **IV. CI Parity Before Every Commit**: PASS on the host; the Linux sampler is
  type-checked with the linux target as in prior slices.
- **V. Bounded Scope: Outside The Game**: PASS. Only reads pixels from the game
  window surface; no memory, network, or gameplay access.
- **Platform and Text Hygiene Constraints**: PASS.

No violations. Complexity Tracking is empty.

## Project Structure

### Documentation (this feature)

```text
specs/005-pixel-bus-reader/
├── plan.md, research.md, data-model.md, quickstart.md
├── contracts/
│   └── reader.md    # decoders, PixelBusEvent, SurfaceSampler seam, reader state machine
├── checklists/{requirements.md, pixel-bus-reader.md}
├── spec.md
└── tasks.md
```

### Source Code (repository root)

```text
src/pixelbus/
├── mod.rs        # Rgb, FishingSignal, PixelBusEvent, decoders, PixelBusReader, ReaderConfig, SurfaceSampler, MockSampler, SamplePoints
├── windows.rs    # #[cfg(windows)] GdiSampler (GetDC + GetPixel on the game window)
└── linux.rs      # #[cfg(target_os = "linux")] X11Sampler (get_image on the game window)
tests/
└── pixelbus.rs   # decoder and state-machine tests with crafted samples and a virtual clock
```

**Structure Decision**: The pure decoders and the `PixelBusReader` state machine
carry all correctness and safety logic and are fully tested with crafted `Rgb`
and an injected clock. The GDI and X11 samplers are thin adapters returning an
`Rgb` per point, mirroring the prior slices' maximal-testable-core pattern.

## Complexity Tracking

No constitution violations. No entries.
