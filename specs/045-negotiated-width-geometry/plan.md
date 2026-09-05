# Implementation Plan: Negotiated Width-Aware Pixel Geometry

**Branch**: `codex/045-negotiated-width-geometry` | **Date**: 2026-09-04 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/045-negotiated-width-geometry/spec.md`

## Summary

Replace the fixed shipping geometry with an addon-authoritative, versioned
layout header at three invariant top-left cells. The reader validates that
header before deriving payload positions and capture extent, falls back to the
old 16-column geometry only after detecting a valid legacy heartbeat, and
publishes the live layout state for diagnostics and settings UI.

## Technical Context

**Language/Version**: Rust 1.96, Lua 5.1-compatible ESO addon code

**Primary Dependencies**: standard library, windows-sys GDI, x11rb, egui/eframe

**Storage**: existing JSON configuration only; layout is runtime-derived and is
not persisted

**Testing**: Rust integration tests, pure protocol tests, Lua source contract
tests, cargo fmt, clippy, and full locked test suite

**Target Platform**: Windows 10/11 x64 and Linux x64 through X11/XWayland

**Project Type**: desktop companion plus managed in-game screen-signal addon

**Performance Goals**: one capture per steady-state sample, at most two during
bootstrap or extent growth, no change to the configured 100 ms fast cadence

**Constraints**: fail closed on recognized invalid metadata, preserve legacy
addon reads, never derive competing columns in the companion, no live-game proof
claimed before issue #44

**Scale/Scope**: three header cells, 21 payload cells, 3 to 65535 columns

## Constitution Check

### Pre-research gate

- Principle I passes: the complete spec-kit sequence precedes product edits.
- Principle II passes: input, uninstall, AddOns confinement, and fishing loss
  behavior remain unchanged and retain their tests.
- Principle III passes: geometry, header decoding, and capture scheduling stay
  behind pure functions and `SurfaceSampler` mocks.
- Principle IV will run the full cargo merge gate before every source commit.
- Principle V passes: the only in-game component remains the visible
  PixelBeacon screen-signal addon.
- Text and platform constraints pass: UTF-8 without BOM, LF, no long dashes,
  Windows and Linux behavior share one contract.

### Post-design gate

Passed. The proposed state is runtime-only, test seams remain explicit, and the
protocol failure path removes payload authority instead of weakening any safety
gate. No constitution exception is required.

## Project Structure

```text
addon/PixelBeacon/
├── PixelBeacon.lua
└── PixelBeacon.txt
src/
├── app/
├── beacon/
├── pixelbus/
│   ├── mod.rs
│   ├── windows.rs
│   └── linux.rs
├── weave/
└── main.rs
tests/
├── app_settings.rs
├── beacon.rs
├── pixelbus.rs
└── pixelbus_display.rs
docs/
├── ESO-Weave-Specification.md
└── plans/
```

**Structure Decision**: Extend the existing single crate, sampler seam, managed
addon, and settings surface. A new crate or persistent schema would add no useful
boundary.

## Implementation Sequence

1. Freeze the new header bytes, geometry formulas, failure states, and legacy
   rule with failing tests.
2. Add pure header encode/decode and `BusLayout` geometry.
3. Make `SurfaceSampler::prepare` extent-aware and teach the reader to bootstrap,
   validate, cache, and change-detect layout.
4. Update GDI to capture the requested extent and keep X11's direct point reads
   unchanged.
5. Add PixelBeacon header cells, dynamic width calculation, reflow, resize event,
   periodic convergence, and manifest version 14.
6. Route layout state into the shared weave model and render a truthful live
   settings caption.
7. Supersede fixed-column documentation, update the changelog and plan index,
   then run focused and full gates.

## Complexity Tracking

No constitution violation exists. The three-cell header is the smallest design
that carries a strong magic/version discriminator plus independently marked and
checksummed 16-bit capacity without limiting small-block displays to 255 columns.
