# Contract: Pure Window-Sizing Helpers

The sizing model is a set of pure, deterministic functions in `src/app/mod.rs`.
These are the seams the tests exercise; the egui frame closure only calls them.

## `measurement_stable(prev, current, epsilon) -> bool`

- Returns `false` when `prev` is `None` (the first measurement is never stable).
- Returns `true` when `prev` is `Some((px, py))` and `|px - current.0| <= epsilon`
  and `|py - current.1| <= epsilon`.
- Epsilon is a small tolerance (about 0.5 points).

## `content_min_size(measured, boot_floor, stable) -> (f32, f32)`

- When `stable == false`: returns `boot_floor` per dimension.
- When `stable == true`: returns `measured` per dimension (may be smaller than the
  boot floor; measured wins).
- Pure and per-dimension independent.

Worked cases (`boot_floor = (480, 420)`):

| measured | stable | result |
| --- | --- | --- |
| (300, 340) | false | (480, 420) |
| (300, 340) | true  | (300, 340) |
| (600, 700) | true  | (600, 700) |
| (600, 340) | true  | (600, 340) |

## `open_log_reserve(row_height) -> f32`

- Returns `log_min_height(row_height) + row_height`.
- Strictly greater than `log_min_height(row_height)` by exactly one row, so at the
  enforced minimum open-window height the log pane's max exceeds its min.

## `split_log_height(prev_window_h, window_h, log_h, content_h, log_min) -> f32`

- Let `d = window_h - prev_window_h` and `fl = log_h / prev_window_h` (guard
  `prev_window_h > 0`; when it is not, return `log_h` clamped).
- `target = log_h + fl * d`.
- `hi = max(window_h - content_h, log_min)`.
- Returns `clamp(target, log_min, hi)`.
- The central pane is the remainder `window_h - result`, so rounding settles into
  central (never below `content_h` because of the `hi` clamp).

Invariants (tested):

1. **Measured supersedes boot floor once stable.** For any `measured` and
   `boot_floor`, `content_min_size(measured, boot_floor, true) == measured`, and
   `content_min_size(measured, boot_floor, false) == boot_floor` (per dimension).
2. **No permanent latch.** Feeding a later smaller stable measurement yields the
   smaller minimum (the running-max behavior is gone).
3. **Resizable at the minimum.** At `window_h = content_h + open_log_reserve(row)`,
   `clamp_log_height` (or the log range) gives `max = window_h - content_h =
   open_log_reserve(row) = log_min + row > log_min = min`.
4. **Compressible window.** For any `window_h >= content_h + log_min_height(row)`,
   the log range is valid (`max >= min`), so the window can shrink to
   `content_h + log_min` with the log at six lines.
5. **Proportional share.** For `content_h` small enough not to clamp,
   `split_log_height` moves the log by `fl * d`: doubling a window that is 30
   percent log raises the log by ~30 percent of the added height, and the central
   pane takes the rest.
6. **Clamps.** `split_log_height` never returns below `log_min` nor above
   `max(window_h - content_h, log_min)`.

## Unchanged helpers

`log_min_height(row)` and `clamp_log_height(height, window, row, content_h)` keep
their current signatures and behavior; `clamp_log_height` is now always fed the
true measured content height.
