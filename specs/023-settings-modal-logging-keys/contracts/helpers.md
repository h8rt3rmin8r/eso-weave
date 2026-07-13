# Contract: Pure Helpers and Log Linkage

This slice adds no network or IPC surface. Its unit-tested contract is two pure
functions and the log-linkage behavior in the view-model.

## Key::display_name (unit-tested)

```rust
pub fn display_name(self) -> &'static str;
```

- Every key returns a non-empty, human-readable name with no underscore.
- Digit keys map to "Number 1".."Number 5"; Space maps to "Space"; F1/F2 map to
  "F1"/"F2"; letter keys map to their uppercase letter.
- `as_str` and `parse` are unchanged (a round trip of every key still holds).

Required tests (in `src/input/key.rs`):

- `display_name` is non-empty and underscore-free for every variant.
- Spot-check the mapping: Digit1 -> "Number 1", Space -> "Space", F2 -> "F2".

## modal_extent (unit-tested)

```rust
pub fn modal_extent(window: f32, min_px: f32, max_px: f32, max_frac: f32) -> f32;
```

Behavioral contract:

1. Monotonic: a larger `window` never yields a smaller result.
2. Fraction decreases: for two windows `w1 < w2` (both large enough to be past the
   minimum and below the cap), `result(w2)/w2 <= result(w1)/w1`.
3. Bounded above: the result never exceeds `max_px` and never exceeds
   `max_frac * window`.
4. Bounded below and fits: at a very small window the result equals
   `max_frac * window` (fits inside the window) rather than `min_px`.
5. Pure and deterministic.

Required tests (in `tests/app_view_model.rs`):

- Small window returns about `max_frac * window` (fits the window).
- A mid window returns more pixels than a small window but a smaller fraction.
- A very large window is capped at `max_px`.

## Log linkage (unit-tested in the model)

- Applying `SetLogFilter(level)` sets the panel `log_filter` to `level` and makes
  the settings log level (`settings_form().logging.level`) equal `level`.
- Applying settings with a new `logging.level` makes `view().log_filter` equal that
  level.
- Toggling the log panel does not change the log level.

Required test (in `tests/app_view_model.rs`):

- Apply `SetLogFilter(Debug)`; assert `view().log_filter == Debug` and
  `settings_form().logging.level == Debug`.
- Apply settings with `logging.level = Warn`; assert `view().log_filter == Warn`.
- Toggle the log panel open/closed; assert `settings_form().logging.level` is
  unchanged.

## Presentation (validated observationally)

- The settings modal is sized from the current window each frame (width and the
  scroll-area max height) via `modal_extent`.
- The Settings saved toast is green and legible in both themes.
- The settings dropdowns keep a constant resting width.
