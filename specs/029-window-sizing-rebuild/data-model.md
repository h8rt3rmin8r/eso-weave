# Data Model: Window Sizing Model Rebuild

No persisted schema changes. This slice changes in-memory UI sizing state and the
signatures of pure sizing helpers. The persisted `log_panel_height` (`UiPrefs`) is
unchanged in shape.

## In-memory state (`EsoWeaveApp`, `src/app/ui.rs`)

| Field | Type | Change | Notes |
| --- | --- | --- | --- |
| `content_min` | `egui::Vec2` | REMOVED / REPLACED | Was a running max seeded at the boot floor; replaced by `content_extent`. |
| `content_extent` | `egui::Vec2` | NEW | The current enforced content minimum: boot floor until stable, then the measured extent (can shrink). |
| `prev_measured` | `Option<egui::Vec2>` | NEW | Last frame's measured content extent, for the two-frame stability gate. |
| `prev_window_h` | `Option<f32>` | NEW | Last frame's window inner height, to detect a window-height delta for the proportional split. |
| `log_height` | `f32` | unchanged | The live log pane height; seeded from persisted `log_panel_height`. |
| `log_panel_open` | `bool` | unchanged | Whether the log pane is shown. |
| `last_min_sent` | `Option<egui::Vec2>` | unchanged | Dedup of the `MinInnerSize` viewport command. |

## Pure sizing helpers (`src/app/mod.rs`)

| Function | Signature | Change |
| --- | --- | --- |
| `content_min_size` | `(measured: (f32,f32), boot_floor: (f32,f32), stable: bool) -> (f32,f32)` | CHANGED: gains `stable`; returns boot floor per dimension until stable, then measured. |
| `measurement_stable` | `(prev: Option<(f32,f32)>, current: (f32,f32), epsilon: f32) -> bool` | NEW: true when `prev` is `Some` and both dimensions are within `epsilon` of `current`. |
| `split_log_height` | `(prev_window_h: f32, window_h: f32, log_h: f32, content_h: f32, log_min: f32) -> f32` | NEW: proportional split of the height delta; clamps to `[log_min, max(window_h - content_h, log_min)]`. |
| `open_log_reserve` | `(row_height: f32) -> f32` | NEW: `log_min_height(row_height) + row_height` (six-line minimum plus one row of drag room). |
| `log_min_height` | `(row_height: f32) -> f32` | unchanged. |
| `clamp_log_height` | `(height, window_height, row_height, content_min_height) -> f32` | unchanged (now fed the true content height). |

## Derivation rules

- Enforced minimum (closed): `(content_extent.x, content_extent.y)`.
- Enforced minimum (open): `(content_extent.x + LOG_WIDTH_BONUS,
  content_extent.y + open_log_reserve(row_h))`.
- Log pane range each frame: `min = log_min_height(row_h)`,
  `max = max(window_h - content_extent.y, min)`.
- Proportional split (log open, window-height changed):
  `log_h' = clamp(log_h + (log_h / prev_window_h) * (window_h - prev_window_h),
  log_min, max(window_h - content_h, log_min))`; central `= window_h - log_h'`.
- Open/close window delta: open `+= clamp(log_h, log_min, ...)`; close
  `-= realized_log_h`.

## Validation rules

- `content_extent` is only updated to the measured extent once `measurement_stable`
  holds; before that it stays at the boot floor.
- Every derived height is clamped so the log never drops below `log_min_height` and
  the central pane never drops below `content_extent.y` (controls never clipped or
  covered).

## Contract cross-reference

Exact invariants and worked cases are in [contracts/sizing.md](contracts/sizing.md).
