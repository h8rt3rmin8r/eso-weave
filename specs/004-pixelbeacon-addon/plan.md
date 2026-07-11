# Implementation Plan: PixelBeacon Companion Addon

**Branch**: `004-pixelbeacon-addon` | **Date**: 2026-07-11 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/004-pixelbeacon-addon/spec.md`

## Summary

Author the embedded in-game addon: a manifest `PixelBeacon.txt` and a single
`PixelBeacon.lua` under `addon/PixelBeacon/`. The Lua renders three fixed
color-coded blocks (status heartbeat, fishing state, latency) anchored to the
top-left of the client area at constant physical-pixel geometry, and detects a
bite from bait consumption during an active fishing interaction with the
specified false-positive suppressions. The addon is a minimal shim with no
settings, UI, libraries, or saved variables. This slice is not Rust; the Rust
crate is unchanged. Verification is structural (manifest marker and version,
block and event coverage) plus manual in-game.

## Technical Context

**Language/Version**: Lua for the ESO addon runtime (game-provided API). No Rust
changes in this slice.

**Primary Dependencies**: None. The addon uses only the game's built-in addon API
(`WINDOW_MANAGER`, `EVENT_MANAGER`, inventory and interaction events,
`GetLatency`, `GetUIGlobalScale`). No external libraries.

**Storage**: None. No saved variables.

**Testing**: No automated game harness exists. Verification is a structural check
(manifest contains the managed marker and a version and declares the API version;
the Lua defines the three blocks with the specified colors, positions, and
encoding, and registers the specified events) plus manual validation in the
running client. The existing Rust test suite is confirmed unchanged.

**Target Platform**: The ESO client (Windows and Linux via Proton), loaded from
the AddOns directory by the later Beacon Manager.

**Project Type**: Embedded game addon (Lua), alongside the Rust application.

**Performance Goals**: The latency block updates about once per second; block
rendering is trivial. The addon does the minimum work per frame.

**Constraints**: No IPC, no network, no saved variables (constitution bounded
scope). Blocks are constant in physical pixels regardless of UI scale. Text files
are UTF-8 without BOM, LF, no em or en dashes.

**Scale/Scope**: One manifest, one Lua file, three blocks, a handful of event
handlers.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

- **I. Spec-Driven Development**: PASS. Derived from `spec.md` (master spec
  sections 9.1 to 9.3), bounded by `docs/plans/plan-001.md`.
- **II. Safety-Critical Surfaces**: PASS (scoped). This slice renders the beacon
  the reader depends on; the marker-guarded uninstall and the SignalLost handling
  are later slices (Beacon Manager, Pixel Bus Reader). The managed marker line is
  authored here so the later uninstall guard has something to verify.
- **III. Test-First With Explicit Seams**: PASS with the medium adapted to a Lua
  addon. There is no unit-test harness for the game API; correctness is enforced
  by a structural validation of the manifest and Lua and by manual in-game checks,
  documented in the tasks. The bite-detection logic is written as small, readable
  functions so its conditions are inspectable.
- **IV. CI Parity Before Every Commit**: PASS. This slice adds no Rust; the cargo
  gate is not applicable to Lua files (constitution Principle IV: non-Rust commits
  have no cargo gate) but the existing Rust suite is confirmed still green.
- **V. Bounded Scope: Outside The Game**: The addon is the one sanctioned in-game
  component; it renders the beacon signal only, with no memory, network, or
  gameplay logic beyond the bite signal. PASS.
- **Text Hygiene Constraints**: PASS. UTF-8 without BOM, LF, no dashes.

No violations. Complexity Tracking is empty.

## Project Structure

### Documentation (this feature)

```text
specs/004-pixelbeacon-addon/
├── plan.md, research.md, data-model.md, quickstart.md
├── contracts/
│   └── pixel-bus.md    # block geometry, colors, latency encoding, manifest marker
├── checklists/{requirements.md, pixelbeacon.md}
├── spec.md
└── tasks.md
```

### Source (repository root)

```text
addon/PixelBeacon/
├── PixelBeacon.txt   # manifest: title, Author, APIVersion, version, managed marker, file list
└── PixelBeacon.lua   # addon: block creation, per-state colors, latency encoding, bite detection
```

No Rust source changes. The addon files are the future embedded payload the
Beacon Manager installs.

**Structure Decision**: A single Lua file keeps the shim minimal per section 9.1.
The manifest is the smallest valid ESO addon descriptor plus the managed marker
line and a version. Block geometry, colors, and the latency encoding are the exact
contract values from section 9.3, so a future Pixel Bus Reader can sample them.

## Complexity Tracking

No constitution violations. No entries.
