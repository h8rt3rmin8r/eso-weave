# Phase 1 Data Model: Fishing Capture Hardening

This slice adds no persisted data. It adds a seam method, a pure helper, and a
diagnostic transition flag.

## SurfaceSampler seam: prepare

`src/pixelbus/mod.rs`. A new default method on the trait:

```rust
pub trait SurfaceSampler {
    /// Captures a fresh frame if the backend needs to; the reader calls this once
    /// before sampling the block points. The default is a no-op.
    fn prepare(&self) {}

    fn sample(&self, x: u32, y: u32) -> Option<Rgb>;
}
```

`PixelBusReader::sample_and_observe` calls `sampler.prepare()` before the four
`sample` calls. `MockSampler` and the Linux backend inherit the no-op; only the
Windows backend overrides it.

## Pure helper: strip_pixel

`src/pixelbus/mod.rs`. Extracts one pixel from a captured 32-bit BGRA strip:

```rust
pub fn strip_pixel(buffer: &[u8], width: u32, height: u32, x: u32, y: u32) -> Option<Rgb>
```

- Returns `None` when `x >= width`, `y >= height`, or the buffer is too small.
- Otherwise reads the pixel at `(y * width + x) * 4` as `b, g, r` and returns
  `Rgb::new(r, g, b)`.
- Pure and deterministic; unit-tested.

## Windows backend capture state

`src/pixelbus/windows.rs`. `GdiSampler` holds the window handle plus an
interior-mutable captured strip:

| Field | Type | Meaning |
|-------|------|---------|
| `hwnd` | window handle | The resolved game window. |
| `frame` | `RefCell<Option<CapturedStrip>>` | The most recent captured strip. |

`CapturedStrip { width: u32, height: u32, pixels: Vec<u8> }` (BGRA). `prepare`
captures the strip; `sample(x, y)` calls `strip_pixel(&frame.pixels, ..)`.
Single-threaded worker access, so `RefCell` is sufficient (the sampler is created
and used only on the pixel-bus worker thread).

## Reader diagnostic state

`src/pixelbus/mod.rs`, `PixelBusReader`. One new field:

| Field | Type | Meaning |
|-------|------|---------|
| `had_heartbeat` | `bool` | Whether the previous observed frame had a heartbeat, used to log the acquire and lose transitions. Does not affect any emitted event. |

`observe` logs `debug!` "pixel bus heartbeat acquired" on a false-to-true
transition (with the raw B0) and "pixel bus heartbeat lost" on true-to-false, and
the existing per-sample `trace!` gains the decoded fishing signal and the heartbeat
age.

## Spec and changelog

- `docs/ESO-Weave-Specification-v0.2.0.md` section 10.3: the sentence naming GDI
  window-surface capture is updated to describe screen-composited capture.
- `CHANGELOG.md`: a `Fixed` entry and a dated `Decisions` entry for the capture
  mechanism change.
