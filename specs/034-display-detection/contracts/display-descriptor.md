# Contract: The Display Descriptor

**Feature**: `specs/034-display-detection/` | **Date**: 2026-07-27

This is the contract a future grid-wrap feature will build on, so it is written
for that reader: what the descriptor promises, what it deliberately does not
promise, and what breaks if either is misread. It is a Rust API contract, not a
wire format; nothing here crosses a process boundary.

## Public surface

Exported from `eso_weave::pixelbus`:

| Item | Kind |
| --- | --- |
| `DisplayDescriptor`, `DisplaySource` | the output |
| `MeasuredDisplay` | what a platform probe returns |
| `StoredVideoSettings`, `parse_user_settings` | the file path |
| `Reconciliation`, `StoredPair`, `reconcile` | the observation |
| `DisplayDetector`, `DisplayUpdate` | change detection and read gating |
| `Size`, `Point` | geometry primitives |
| `SurfaceSampler::display` | the seam |

Field shapes are in [data-model.md](../data-model.md). This document states the
guarantees.

## What the descriptor promises

1. **`surface` is always present and always non-zero.** There is no descriptor
   with a zero, absent, or negative extent. A consumer may divide by it.
2. **Every value is in physical device pixels**, the same unit as `block_px`,
   `block_center`, and `capture_dims`. A consumer computing a column count from
   `surface.width / block_px` is comparing like with like, which is the entire
   point of fixing the unit here.
3. **The scale is never applied to any value.** `scale()` is information the
   consumer may use; the geometry is already physical.
4. **`source` is accurate.** `Measured` means the operating system was asked
   about a live window at the moment of resolution. `Configured` means a file
   was read. A consumer that needs the live surface must require `Measured`, and
   the type makes that check possible rather than leaving it to a comment.
5. **A `Configured` descriptor carries `surface` and nothing else.** Its
   `surface_origin`, `display_origin`, `display_size`, and `dpi` are always
   `None`. Do not read absence there as "the display is at the origin".

## What the descriptor does not promise

1. **No window mode.** The descriptor never says fullscreen, borderless, or
   windowed, from either source. From the file the mapping is unverified; from
   measurement it would be a guess, because a surface that exactly covers its
   display is equally consistent with all three. A consumer that believes it
   needs the mode should check whether it actually needs the rectangle.
2. **`display_*` and `dpi` may be absent on a measured descriptor.** X11 never
   supplies `dpi`. Treat absence as unknown, never as a default. Substituting
   `1.0` for an unknown scale reproduces exactly the bug FR-006 exists to
   prevent.
3. **On a multi-head X11 session, `display_size` is the union of all heads**,
   not the head the window is on. A consumer must not treat it as the monitor
   the surface occupies. Windows reports the actual monitor.
4. **Freshness is bounded by the sampling cycle, not instantaneous.** The
   descriptor is at most one pixel-bus poll behind reality.
5. **Nothing is persisted.** There is no descriptor at startup until the first
   resolution.

## The detector's contract

```rust
detector.update(measured, || read_settings())
```

- Returns `Some(DisplayUpdate)` only when the descriptor changed. A caller may
  log unconditionally on `Some` without flooding. The update's reconciliation is
  present when a stored reading was consulted during that change and absent
  otherwise; it is never a change signal in its own right, because it is only
  ever recomputed when the descriptor moves.
- The closure is called at most once per `update`, and **is not called at all**
  when the measurement is unchanged. A caller may therefore make the closure do
  real file input and output without thinking about cadence. This is a
  guarantee, not an optimization, and it is tested by counting invocations.
- The closure is also not called when a previously present measurement
  disappears. Losing the window is not a reason to consult a file.
- `update` never panics, never blocks, and never touches the filesystem itself.

## The seam's contract

`SurfaceSampler::display()` defaults to `None`. An implementation that supplies
it MUST:

- Return `None` rather than a zero or negative surface, including when the
  window is minimized or not currently drawable.
- Report physical pixels, not logical or scaled units.
- Return `None` rather than a partial guess if the window cannot be resolved.
- Perform no blocking work: it is called once per sampling iteration on the
  pixel bus worker thread, which must stay responsive, and it is never called
  from the input hook thread.

Supplying `display_origin` without `display_size`, or either without `dpi`, is
allowed and expected: the fields degrade independently.

## Stability

The descriptor is expected to gain fields as the grid work defines what layout
needs (a usable content rectangle, a per-monitor rectangle on Linux once RandR
is wired). It is not expected to lose the guarantees above. In particular,
`surface` being present and non-zero, and the unit being physical pixels, are
the two properties the wrap math will be written against.
