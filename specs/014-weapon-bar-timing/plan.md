# Implementation Plan: Weapon-Bar-Aware Adaptive Timing

**Branch**: `014-weapon-bar-timing` | **Date**: 2026-07-11 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/014-weapon-bar-timing/spec.md`

## Summary

Detect the active weapon bar and each bar's weapon class in-game (PixelBeacon), relay
them through a new fourth pixel-bus block, decode them in the pixel-bus reader, and
drive a per-bar timing model in the weave engine with weapon-class default presets and
an auto-timing preference. Surface the detected bar and classes in the GUI and persist
the per-bar timing. Close research item R1 with a dated specification appendix. The
addon relays a small normalized weapon-class code computed from the named
`WEAPONTYPE_*` constants, so the out-of-process reader never needs raw game enum
integers. Correctness logic (decode, class mapping mirror, per-bar selection, presets)
is unit-tested; the egui layer stays thin. The pixel signal and exact preset values
require in-game validation that is an explicit follow-up, not a design blocker.

## Technical Context

**Language/Version**: Rust (2021), plus the PixelBeacon addon in ESO Lua

**Primary Dependencies**: `eframe`/`egui` 0.35, `serde`/`serde_json`; the addon uses
only the ESO add-on API (no libraries)

**Storage**: `config.json` weave section, extended additively with a back timing
profile and an auto-timing flag (serde defaults keep back-compat). Runtime bar and
class state is not persisted.

**Testing**: `cargo test --all --locked` for the reader decode, the class-mapping
mirror, the per-bar timing selection, the presets, routing, and config round-trips.
The addon Lua and the live pixel signal are validated in-game (no headless harness).

**Target Platform**: Windows 10/11 x64 and Linux x64; the addon runs in the ESO client

**Project Type**: single-crate desktop application plus a companion addon

**Performance Goals**: decode adds one sampled point per read; the addon re-renders the
block only on an actual bar or class change (edge-detected)

**Constraints**: bounded scope (only the rendered pixel signal; no game memory or
network); the addon managed-marker line stays intact; thin egui layer; UTF-8 no BOM,
LF, no em-dashes or en-dashes

**Scale/Scope**: one new pixel block, one decode channel, a front and back timing
profile, seven weapon classes, and a small GUI addition

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

- **I. Spec-Driven Development (NON-NEGOTIABLE)**: PASS. Built through the full
  spec-kit sequence under build plan 003; traces to master spec sections 7.3, 9, and
  16 (R1). The `/speckit.analyze` gate runs before implementation.
- **II. Safety-Critical Surfaces Are Sacrosanct (NON-NEGOTIABLE)**: PASS. The addon
  change appends a block and events but preserves the managed-marker line, so the
  Beacon Manager uninstall verification (spec 006) still gates deletion; a test
  asserts the managed marker is present after the manifest change. Signal loss still
  degrades fishing to disabled; the new decode never fires input.
- **III. Test-First With Explicit Seams**: PASS. The decoders, the class-mapping
  mirror, the reader state, the per-bar timing selection, the presets, and routing
  land behind the existing `SurfaceSampler` and `WeaveSink` seams with unit tests.
- **IV. CI Parity Before Every Commit (NON-NEGOTIABLE)**: PASS. fmt, clippy
  `-D warnings`, and `cargo test --all --locked` run in the foreground to completion.
- **V. Bounded Scope: Outside The Game**: PASS. The reader still only samples the
  rendered pixel signal; the addon uses only the public add-on API to read weapon
  state it already has access to, and relays it as pixels. No memory or network.

Pinned artifacts and contracts (each gets a dated `CHANGELOG.md` Decisions entry):

- The pixel-bus contract `specs/004-pixelbeacon-addon/contracts/pixel-bus.md` gains
  the B3 Weapon-bar block.
- The reader contract `specs/005-pixel-bus-reader/contracts/reader.md` gains the
  weapon-bar decoder, signal, and sample point.
- The addon manifest `addon/PixelBeacon/PixelBeacon.txt` bumps `APIVersion` and the
  addon version; the managed marker is unchanged.
- The master specification gains the R1 timing appendix and marks R1 closed.

## Project Structure

### Documentation (this feature)

```text
specs/014-weapon-bar-timing/
├── plan.md, research.md, data-model.md, quickstart.md
├── checklists/{requirements.md, weapon-bar-timing.md}
└── tasks.md   (from /speckit-tasks)
```

Contract edits land in the existing pinned contract files (004, 005) rather than a new
`contracts/` dir, since this slice extends those contracts.

### Source Code (repository root)

```text
addon/PixelBeacon/
├── PixelBeacon.lua   # B3 weapon-bar block: active pair + per-bar weapon-class code,
│                     #   edge-detected, re-baselined on load/death; new event hooks
└── PixelBeacon.txt   # APIVersion and addon-version bump (managed marker unchanged)

src/pixelbus/mod.rs   # ActiveBar, WeaponClass, WeaponBarSignal; decode_weapon_bar;
                      #   B3 sample point; observe() gains b3; WeaponBar event
src/weave/
├── mod.rs            # per-bar timing state + selection in handle(); set_weapon_bar;
│                     #   store/load back profile + auto flag
├── types.rs          # timing_back + auto_timing on WeaveConfig; class presets
└── sequence.rs       # unchanged (already takes a TimingConfig by reference)
src/app/routing.rs    # route WeaponBar event to weave.set_weapon_bar
src/app/mod.rs, ui.rs, strings.rs  # active-bar/weapon status line, per-bar timing view,
                      #   auto-timing toggle (thin, reuses slice 013 helpers)
docs/ESO-Weave-Specification-v0.1.0.md  # R1 appendix; section 16 marks R1 closed
```

### The B3 encoding (design)

One 16x16 block at x=48, sampled at (56, 8). One RGB carries everything:

- green = `0x5A` marker (identifies a weapon-bar sample; distinct from the latency
  marker `0xA5` so the two never alias within tolerance).
- red = `(front_class << 4) | back_class`, each a 0..6 nibble.
- blue = active bar (`0` unknown, `1` front, `2` back).

Weapon-class codes (fixed, mirrored in Lua and Rust): 0 none/unknown, 1 dual wield,
2 two handed, 3 sword and shield, 4 bow, 5 destruction staff, 6 restoration staff.
The reader decodes only when green matches the marker within tolerance; otherwise it
yields no weapon-bar signal (the block is absent or unreadable). The addon re-renders
the block only when the determined bar or a class changes.

**Structure Decision**: Extend the existing single crate and the companion addon.
`ActiveBar`/`WeaponClass`/`WeaponBarSignal` live in `src/pixelbus` (the decoder's
output); the weave engine consumes them for per-bar timing selection. Per-bar means the
timing profile only; the seven skill slots are unchanged.

## Complexity Tracking

No constitution violations require justification. The per-bar model adds a second
timing profile and an auto flag to the weave config (additive, serde-defaulted) plus
runtime bar/class state on the engine; a full per-bar skill-slot model was rejected as
out of scope (the operator asked to split delay settings per bar, not the slots). The
single-RGB B3 encoding was chosen over multiple blocks to keep the pixel strip compact
and the reader to one added sample point.
