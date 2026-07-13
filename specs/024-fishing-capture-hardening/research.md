# Phase 0 Research: Fishing Capture Hardening

## Root cause (established by deep trace)

The UI never leaves Casting because the controller transition `Armed -> Waiting`
fires only on a `FishingStarted` event, which the reader emits only when it decodes
the blue `#0080FF` waiting block on B1 while the magenta `#FF00FF` heartbeat on B0
is present (`src/pixelbus/mod.rs::observe`, gated by `if heartbeat`). The addon
render and the reader decode agree byte-for-byte on colors and coordinates, so the
failure is upstream of decoding. The symptom wording is "no cast detected" (the arm
timeout), not "signal lost"; `SignalLost` only fires once a heartbeat has been seen
at least once, so the wording indicates the heartbeat is never read.

`GdiSampler::sample` (`src/pixelbus/windows.rs`) does `GetDC(hwnd)` then
`GetPixel`. `GetPixel` on a window device context reads that window's GDI surface
(the front buffer of GDI drawing). A DirectX game renders through its own swap
chain, composited by the desktop compositor separately; its pixels are not in the
GDI window surface. So `GetPixel(windowDC)` returns black or stale pixels, B0 is
never magenta, the `if heartbeat` gate is never entered, and every session ends on
the 8 s arm timeout as "no cast detected". This is immune to the three prior fixes
(hotkey routing, poll cadence, shared clock, addon API version), which all patched
infrastructure around this unread signal.

## The working capture mechanism

The project already has direct evidence (the GUI-capture workaround) that reading
the composited desktop framebuffer captures accelerated content where a window read
fails: PowerShell `Graphics.CopyFromScreen` (a `BitBlt` from the screen device
context) captures the OpenGL-rendered ESO Weave window, while `PrintWindow` returns
black. The same reasoning applies to ESO's DirectX content: the composited screen
holds it, the GDI window surface does not. The Windows backend therefore captures
the small beacon strip from the desktop DC with `BitBlt` (using `CAPTUREBLT`) into a
memory bitmap and reads the block points from the captured bitmap (where `GetDIBits`
or `GetPixel` on the memory DC is reliable).

Steps: `GetClientRect(hwnd)` and `ClientToScreen(hwnd, {0,0})` give the strip's
screen origin; `GetDC(NULL)` is the screen DC; `CreateCompatibleDC` +
`CreateCompatibleBitmap` build a memory target; `BitBlt(SRCCOPY | CAPTUREBLT)`
copies the strip; `GetDIBits` (32 bpp, top-down `BI_RGB`) reads it into a buffer;
handles are released. The buffer is BGRA per pixel (`b, g, r, a`), decoded to `Rgb`.

Only a tiny strip is captured (width covering the four block sample points at
x = 8, 24, 40, 56 with 16 px blocks, so 64 px wide by 16 px tall), so the per-sample
cost is negligible at the existing cadence.

## Capturing once per batch behind the seam

The reader samples four points per observe. To avoid four captures per frame and to
keep the `SurfaceSampler` seam and every test unchanged, add a default no-op
`prepare(&self)` to the trait; the reader calls it once before the four `sample`
calls. The Windows backend overrides `prepare` to capture a fresh strip into
interior-mutable storage (`RefCell`, single-threaded worker access), and `sample`
reads the pixel from the stored strip via the pure `strip_pixel` helper. The mock
and Linux backends keep the default no-op, so `observe`, `sample_and_observe`, and
their tests are unchanged.

## Diagnostics

The existing TRACE line already logs `heartbeat`, `b0..b3`, and `now_ms`. Extend it
with the decoded fishing signal and the heartbeat age, and add DEBUG logs on the
heartbeat acquire and lose transitions via a new `had_heartbeat` flag on the reader
(which changes no emitted event). Reading the log:

- Never a "heartbeat acquired" line, and B0 traces as near-black: a capture problem
  (should now be fixed) or the addon not loaded / strip covered.
- "heartbeat acquired" but the fishing signal never becomes Waiting on a real cast:
  a capture-independent addon interaction-detection problem, scoped as a follow-up.

## Alternatives considered

- **`PrintWindow` with `PW_RENDERFULLCONTENT`**: rejected; project evidence shows it
  returns black for accelerated (OpenGL/DirectX) content, which is exactly the case
  here.
- **Shelling out to PowerShell CopyFromScreen per sample**: rejected; an in-process
  `BitBlt` is far cheaper and avoids spawning a process at the sampling cadence.
- **Changing the seam signature to a batch capture**: rejected; a default `prepare`
  method keeps every existing test and the mock unchanged.
- **Multi-monitor virtual-desktop capture**: the common case (single monitor, strip
  visible, window focused) is handled by the screen DC; occluded or edge cases
  degrade to no heartbeat via the existing signal-loss path. A fully robust
  multi-monitor capture is out of scope for this slice and noted as a limitation.
