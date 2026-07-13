# Implementation Plan: Fishing Signal Diagnosis and Capture Hardening

**Branch**: `024-fishing-capture-hardening` | **Date**: 2026-07-13 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/024-fishing-capture-hardening/spec.md`

## Summary

The fishing failure (Casting reverts to Idle, no cast detected) traces to the
Windows surface sampler reading the wrong buffer: `GdiSampler::sample` calls
`GetPixel` on the game window device context, which on a DirectX surface reads the
GDI front buffer, not the composited accelerated content, so the beacon heartbeat
is never read. Harden the Windows capture to read the composited desktop
framebuffer (a screen `BitBlt`, the same mechanism as the proven CopyFromScreen
workaround), behind the existing `SurfaceSampler` seam, capturing the small beacon
strip once per sample batch. Add log-only diagnostics that make the raw block
bytes, decoded signal, and heartbeat acquire/lose transitions unambiguous, so a
single in-game session either confirms the fix or pinpoints an addon problem.
Reconcile the master spec's capture language and record a dated decision. The pure
decode and observe logic and their tests are untouched; a new pure pixel-extraction
helper is unit-tested; the end-to-end fix is verified by an in-game protocol in the
quickstart.

## Technical Context

**Language/Version**: Rust 1.96 (edition 2021)

**Primary Dependencies**: windows-sys 0.59 GDI (already a dependency:
`GetDC`, `CreateCompatibleDC`, `CreateCompatibleBitmap`, `SelectObject`, `BitBlt`,
`GetDIBits`, `DeleteObject`, `DeleteDC`, `ReleaseDC`) plus `GetClientRect` and
`ClientToScreen`; `tracing` for the diagnostics.

**Storage**: none. No config or persisted change.

**Testing**: `cargo test` for the existing decoder/reader tests (unchanged) and a
new pure pixel-extraction helper; the OS capture backend is exercised in-game per
the quickstart (a real accelerated surface cannot be captured headlessly).

**Target Platform**: Windows 10/11 x64 (the capture change) and Linux x64
(unchanged).

**Performance Goals**: One small screen capture (a tiny strip covering the four
block points) per sample batch, at the existing cadence (100 ms fishing, 1000 ms
idle); negligible.

**Constraints**: Outside the game (reads on-screen pixels only); safety-critical
surfaces untouched; the addon and keypress paths deliberately unchanged; text
hygiene holds.

**Scale/Scope**: One Windows backend rewrite behind the seam, one default seam
method, one reader call plus enhanced logs, one pure helper with tests, one spec
paragraph, one changelog decision.

## Constitution Check

- **I. Spec-Driven Development**: Full spec-kit sequence; traces to master spec
  section 8 (fishing) and 10 (pixel bus). PASS.
- **II. Safety-Critical Surfaces**: Fishing still degrades to disabled on
  SignalLost (the reader's signal-loss path is unchanged); the input engine and the
  beacon managed-marker uninstall are untouched. PASS.
- **III. Test-First With Explicit Seams**: The capture sits behind the existing
  `SurfaceSampler` seam; the pure decode/observe logic and its tests are untouched,
  and the new pixel-extraction helper is unit-tested first. The OS backend itself
  is not headlessly testable and is verified in-game (an explicit, documented
  exception consistent with prior sampling backends). PASS.
- **IV. CI Parity Before Every Commit**: fmt, clippy (-D warnings), test --all
  --locked in the foreground before commit. PASS.
- **V. Bounded Scope: Outside The Game**: The capture only reads on-screen pixels,
  the existing screen-signal contract; no game memory, no packets. PASS.

No violations. Complexity Tracking is empty.

## Project Structure

### Documentation (this feature)

```text
specs/024-fishing-capture-hardening/
|-- plan.md              # This file
|-- spec.md              # Feature specification
|-- research.md          # Phase 0 root-cause and mechanism decisions
|-- data-model.md        # Phase 1 (seam method, helper, no persisted data)
|-- quickstart.md        # Phase 1 in-game validation protocol
|-- contracts/
|   `-- capture.md       # Seam and pixel-extraction behavioral contract
|-- checklists/
|   |-- requirements.md
|   `-- capture-diagnostics.md
`-- tasks.md             # Created next
```

### Source Code (repository root)

```text
src/
|-- pixelbus/
|   |-- mod.rs           # SurfaceSampler::prepare default (no-op); reader calls
|   |                    #   prepare() before sampling; pure strip_pixel helper
|   |                    #   (+ tests); enhanced TRACE + heartbeat acquire/lose
|   |                    #   DEBUG diagnostics in observe/reader
|   |-- windows.rs       # Screen-composited BitBlt of the beacon strip behind
|   |                    #   the seam; sample reads from the captured strip
|   `-- linux.rs         # unchanged (default prepare is a no-op)
docs/
`-- ESO-Weave-Specification-v0.2.0.md   # section 10.3 capture language
CHANGELOG.md             # dated decision + Fixed entry
```

**Structure Decision**: Single crate retained; the change is confined to the
pixelbus module and docs. No new dependency (all GDI calls are in the already
enabled windows-sys features).

## Key Decisions (autopilot decision log)

- **Capture from the composited screen, not the window DC.** `GetPixel` on the
  window DC reads the GDI front buffer, which does not contain DirectX-rendered
  content, so it returns black or stale pixels; the beacon heartbeat is never read.
  The Windows backend instead captures the small beacon strip from the desktop
  device context via `BitBlt` (with `CAPTUREBLT`) into a memory bitmap and reads the
  block points from it. This is the same mechanism as the project's proven
  CopyFromScreen workaround, which captures accelerated content where a window-DC
  read or `PrintWindow` returns black. Architecture-affecting (it changes the
  spec-named capture mechanism); recorded here, in the spec, and in the changelog.
- **Capture once per batch via a `prepare` seam method.** `SurfaceSampler` gains a
  default no-op `prepare(&self)`; the reader calls it once before the four
  `sample` calls; the Windows backend overrides it to capture a fresh strip into
  interior-mutable storage, and `sample(x, y)` reads from that strip. The mock and
  the Linux backend use the default no-op, so the pure decode/observe tests are
  unchanged. Chosen over capturing per pixel (four captures per frame) and over
  changing the seam signature (which would ripple through every test).
- **Pixel extraction as a pure helper.** `strip_pixel(buffer, width, height, x, y)`
  bounds-checks and decodes a 32-bit BGRA pixel to `Rgb`; it is unit-tested, so the
  only testable part of the capture path has coverage while the OS calls stay in the
  thin backend.
- **Log-only diagnostics.** Extend the per-sample TRACE line with the decoded
  fishing signal and the heartbeat age, and add DEBUG logs when the heartbeat is
  first acquired and when it is lost (a new `had_heartbeat` transition flag on the
  reader, which does not change any emitted event). This makes never-present
  heartbeat (capture problem) versus present-heartbeat-but-never-blue (addon
  problem) obvious in one reading. No new GUI surface, per the operator's choice.
- **Isolate the fix.** The addon interaction-detection contract
  (`EVENT_CLIENT_INTERACT_RESULT`/`INTERACTION_FISH`) and the keypress scan-code
  synthesis are deliberately left unchanged this slice, so if the symptom persists
  the logs cleanly point at the next suspect rather than confounding two changes.

## Phasing

- Phase 0 (research.md): confirm the root cause, the working capture mechanism, and
  the BGRA pixel format.
- Phase 1 (data-model.md, contracts/, quickstart.md): the seam method, the pure
  helper contract, the diagnostics, and the in-game validation protocol.
- Phase 2 (tasks.md): test-first task list.

## Complexity Tracking

No constitution violations; no entries.
