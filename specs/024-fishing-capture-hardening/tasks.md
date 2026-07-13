---
description: "Task list for fishing signal diagnosis and capture hardening (slice 024)"
---

# Tasks: Fishing Signal Diagnosis and Capture Hardening

**Input**: Design documents from `specs/024-fishing-capture-hardening/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/capture.md

**Tests**: The pure `strip_pixel` helper is unit-tested (test-first); the existing
decoder/reader tests must keep passing. The OS capture backend and the end-to-end
fix are verified in-game per quickstart.md (a real accelerated surface cannot be
captured headlessly).

## Phase 1: Pixel-extraction helper (test-first)

- [x] T001 In `src/pixelbus/mod.rs` add unit tests for `strip_pixel(buffer, width,
  height, x, y)`: a crafted 2x2 BGRA buffer returns the expected `Rgb` per point
  (channel order: magenta stored as bytes FF 00 FF); out-of-range x or y returns
  `None`; a truncated buffer returns `None` (no panic). (Write first; fails until
  T002.)
- [x] T002 In `src/pixelbus/mod.rs` add `pub fn strip_pixel(buffer: &[u8], width:
  u32, height: u32, x: u32, y: u32) -> Option<Rgb>` per the contract (bounds check,
  BGRA to RGB). Make T001 pass.

## Phase 2: Seam and reader wiring

- [x] T003 In `src/pixelbus/mod.rs` add a default no-op `fn prepare(&self) {}` to
  the `SurfaceSampler` trait, and call `sampler.prepare()` at the start of
  `PixelBusReader::sample_and_observe` before the four `sample` calls. Confirm the
  mock and existing reader tests are unaffected.

## Phase 3: Diagnostics (log-only)

- [x] T004 In `src/pixelbus/mod.rs` add a `had_heartbeat: bool` field to
  `PixelBusReader` (init false), and in `observe` log `debug!` "pixel bus heartbeat
  acquired" (with B0) on a false-to-true transition and "pixel bus heartbeat lost"
  on a true-to-false transition; do not change any emitted event.
- [x] T005 In `src/pixelbus/mod.rs` extend the existing per-sample `trace!` line in
  `observe` with the decoded fishing signal (`fishing_signal(b1)`) and the heartbeat
  age (`now_ms - last_heartbeat_ms`), keeping the raw block bytes.

## Phase 4: Windows screen-composited capture

- [x] T006 In `src/pixelbus/windows.rs` rewrite `GdiSampler` to hold the window
  handle plus a `RefCell<Option<CapturedStrip>>`. Implement `prepare`: resolve the
  strip screen origin via `GetClientRect` + `ClientToScreen`, `BitBlt` a small strip
  (covering the four block points, e.g. 64x16) from `GetDC(NULL)` (SRCCOPY |
  CAPTUREBLT) into a memory bitmap, read it with `GetDIBits` (32 bpp top-down
  BI_RGB) into a `Vec<u8>`, release all GDI handles, and store the strip. Implement
  `sample(x, y)` to return `strip_pixel(&strip.pixels, strip.width, strip.height, x,
  y)`. Add the needed windows-sys GDI imports (already-enabled features).

## Phase 5: Spec and changelog

- [x] T007 In `docs/ESO-Weave-Specification-v0.2.0.md` section 10.3 update the
  sentence naming GDI window-surface capture to describe screen-composited capture
  (reads the desktop framebuffer, so DirectX content is captured).
- [x] T008 Update `CHANGELOG.md` `[Unreleased]` with a `Fixed` entry (fishing
  reads the accelerated surface; the app no longer sees black pixels) and a dated
  `Decisions` entry recording the capture-mechanism change from window-DC GetPixel
  to a screen-composited BitBlt.

## Phase 6: Verification

- [x] T009 Run CI parity in the foreground: `cargo fmt --all -- --check`,
  `cargo clippy --all-targets --all-features -- -D warnings`,
  `cargo test --all --locked`. Fix any findings.
- [ ] T010 In-game validation per `quickstart.md`: rule out the preconditions
  (bait, addon current/loaded, strip visible, focus), then confirm via the logs that
  the heartbeat is acquired and, on a real cast, the fishing signal becomes Waiting
  and the UI advances Casting to Fishing to Reeling; if the heartbeat is present but
  the fishing block never turns blue, record that evidence for the addon follow-up.

---

## Dependencies & Execution Order

- T001 before T002 (test-first); T002 before T006 (capture uses the helper).
- T003 before T006 (the backend overrides `prepare`).
- T004/T005 are independent diagnostics (parallel-safe).
- Phase 6 last; T009 gates the commit; T010 is the in-game verification owed to the
  operator.

## Notes

- The capture stays behind the `SurfaceSampler` seam; the pure decode/observe logic
  and its tests are untouched.
- Safety-critical surfaces (input engine, beacon managed-marker uninstall) and the
  degrade-on-signal-loss behavior are not changed; the addon and keypress paths are
  deliberately left unchanged to keep the fix isolable.
- Commit as `feat(024): fishing signal diagnosis and capture hardening` after T009;
  halt before push per the autopilot protocol. (Because pinned artifacts are not
  touched and this is a source + docs change, the single pre-push halt applies.)
