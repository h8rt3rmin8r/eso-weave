# Phase 1 Data Model: Window Geometry Persistence

## New type: WindowGeometry

Stored in `src/config/state.rs`. Serialized as a JSON object inside `state.json`.

| Field | Type | Meaning |
|-------|------|---------|
| `x` | `i32` | Outer window left edge, in egui points (desktop virtual coordinates; may be negative on a secondary monitor). |
| `y` | `i32` | Outer window top edge, in egui points. |
| `width` | `u32` | Inner (client) width, in egui points. |
| `height` | `u32` | Inner (client) height, in egui points. |
| `maximized` | `bool` | Whether the window was maximized (defaults to false when absent). |

Derives: `Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default`.
The `maximized` field carries `#[serde(default)]` so an object without it loads as
not maximized.

## Changed type: SessionState

`src/config/state.rs`. One optional field is added; all existing fields and
behavior are unchanged.

| Field | Type | Notes |
|-------|------|-------|
| `schema_version` | `u32` | Bumped: `CURRENT_STATE_VERSION` 2 -> 3. |
| `suspended` | `bool` | Unchanged. |
| `fishing` | `bool` | Unchanged. |
| `api_version` | `ApiVersionCache` | Unchanged. |
| `window` | `Option<WindowGeometry>` | New. `#[serde(default, skip_serializing_if = "Option::is_none")]`. `None` means no recorded geometry (open at default). |

`SessionState` retains `Copy` and `Eq` because every field is `Copy`/`Eq`.

Migration: additive and backward compatible. A pre-feature `state.json` (schema 1
or 2) without a `window` key loads with `window: None`. A malformed file falls
back to `SessionState::default()` (window `None`) with a notice, per the existing
resilience path.

## Restore output: GeometryRestore

The result of sanitizing a recorded `WindowGeometry` for launch. Not serialized;
consumed by `main.rs` to build the viewport.

| Field | Type | Meaning |
|-------|------|---------|
| `width` | `u32` | Size to apply, clamped to the valid range. |
| `height` | `u32` | Size to apply, clamped to the valid range. |
| `position` | `Option<(i32, i32)>` | Position to apply, or `None` to open centered/default (off-screen or no bounds to trust position). |
| `maximized` | `bool` | Whether to open maximized. |

## Restore inputs: RestoreBounds

Parameters passed to the pure sanitize helper so it needs no platform access.

| Field | Type | Meaning |
|-------|------|---------|
| `min_w`, `min_h` | `u32` | Minimum inner size (the app's `min_inner_size`, 480x420). |
| `max_w`, `max_h` | `u32` | Maximum plausible inner size (from the point-space desktop extent, or a generous cap when unknown). |
| `virtual_screen` | `Option<(i32, i32, i32, i32)>` | Point-space desktop rectangle `(x, y, w, h)`; `None` skips the off-screen position check. |

The no-geometry case (session `window` is `None`) keeps the existing default size
(600x720) applied directly in `main.rs`; `sanitize_geometry` runs only when a
recorded geometry exists, and clamps any degenerate size up to the minimum.

## Model state: AppModel.window

`src/app/mod.rs`. A new field `window: Option<WindowGeometry>` holds the latest
captured geometry. It is:

- seeded from the restored session in `restore_session`,
- updated by `UiIntent::SetWindowGeometry(WindowGeometry)` (which marks the
  session store dirty),
- included in `current_session_state()` so it round-trips on every session write.

## Entity relationships

`state.json` (file) contains one `SessionState`, which now optionally contains one
`WindowGeometry`. `AppModel` mirrors that optional geometry in memory. The GUI
layer derives a candidate `WindowGeometry` each frame from egui viewport info and
raises an intent only when it differs from the last captured value; `main.rs`
transforms a restored `WindowGeometry` through `sanitize_geometry` into a
`GeometryRestore` for the viewport builder.
