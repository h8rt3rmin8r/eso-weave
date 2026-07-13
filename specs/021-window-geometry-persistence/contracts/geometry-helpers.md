# Contract: Geometry Pure Helpers

This feature adds no network or IPC surface. Its testable contract is a pure
function plus a platform accessor seam. Signatures are indicative; names may be
refined during implementation as long as the behavior below holds and is tested.

## sanitize_geometry (pure, unit-tested)

```rust
pub struct RestoreBounds {
    pub min_w: u32,
    pub min_h: u32,
    pub max_w: u32,
    pub max_h: u32,
    pub virtual_screen: Option<(i32, i32, i32, i32)>, // point-space (x, y, w, h)
}

pub struct GeometryRestore {
    pub width: u32,
    pub height: u32,
    pub position: Option<(i32, i32)>,
    pub maximized: bool,
}

pub fn sanitize_geometry(geo: WindowGeometry, bounds: RestoreBounds) -> GeometryRestore;
```

Behavioral contract:

1. **Size clamping**: `width`/`height` are clamped into `[min_w, max_w]` /
   `[min_h, max_h]`. A recorded size of zero (or below `min`) becomes the minimum;
   a size above `max` becomes the maximum. The returned size is always usable.
2. **Position visibility (bounds present)**: when `virtual_screen` is
   `Some((vx, vy, vw, vh))`, the recorded window rectangle
   `(x, y, clamped_w, clamped_h)` must overlap the desktop rectangle by at least a
   visible margin (a minimum horizontal strip and a title-bar-height vertical
   strip). If it does, `position = Some((x, y))`; otherwise `position = None`
   (open centered at the restored size).
3. **Position trust (no bounds)**: when `virtual_screen` is `None`,
   `position = Some((x, y))` (the window manager is trusted to place it).
4. **Maximized passthrough**: `maximized` is returned unchanged.
5. **Purity**: no I/O, no platform calls, no clock; deterministic in its inputs.

Required unit tests (red first):

- In-range geometry with bounds returns the same size and `Some(position)`.
- Zero or sub-minimum size clamps up to the minimum.
- Oversized size clamps down to the maximum.
- A position fully outside the virtual screen returns `position = None`, size
  preserved.
- A position partially overlapping by more than the margin keeps `Some(position)`.
- `virtual_screen: None` always returns `Some(position)`.
- `maximized: true` is preserved regardless of position outcome.

## virtual_screen_bounds_points (platform seam)

```rust
// src/platform/mod.rs
pub fn virtual_screen_bounds_points() -> Option<(i32, i32, i32, i32)>;
```

- **Windows**: reads `GetSystemMetrics(SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN,
  SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN)` and divides each by
  `GetDpiForSystem() as f32 / 96.0` to convert physical pixels to points, returning
  `Some((x, y, w, h))`. Returns `None` only if the metrics are non-positive.
- **Linux**: returns `None` (position placement is left to the window manager;
  Wayland cannot report absolute position).

This accessor is the seam: it is the only part that touches the OS, and its output
is passed into `sanitize_geometry`, which is what tests exercise.

## Session migration (unit-tested)

- A `state.json` payload without a `window` key deserializes to `window: None`.
- A round trip of a `SessionState` carrying a `WindowGeometry` preserves every
  field.
- `CURRENT_STATE_VERSION` is 3.
