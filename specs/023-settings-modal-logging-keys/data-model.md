# Phase 1 Data Model: Settings Modal, Logging, and Keys

This slice adds no persisted data. It adds pure helpers and a view-model behavior
change.

## Key display name

`src/input/key.rs`. A new method on `Key`:

```rust
pub fn display_name(self) -> &'static str
```

| Key | display_name |
|-----|--------------|
| Digit1..Digit5 | "Number 1".."Number 5" |
| E, R, X, Q | "E", "R", "X", "Q" |
| Space | "Space" |
| F1, F2 | "F1", "F2" |

Display-only; `as_str` (canonical storage) and `parse` are unchanged. The GUI
selectable list `KEYS` gains `Key::F2` (length 11 -> 12).

## Modal extent helper

`src/app/mod.rs`, a pure function beside `clamp_log_height`:

```rust
pub fn modal_extent(window: f32, min_px: f32, max_px: f32, max_frac: f32) -> f32
```

Contract:

- Returns `min_px` at the smallest window and grows sub-linearly with the window,
  so the occupied fraction decreases as the window enlarges.
- Clamped to `[min_px, max_px]`.
- Never exceeds `max_frac * window` (so it fits even a very small window).

Applied each frame to the modal width (from `content_rect().width()`) and to the
scroll-area max height (from `content_rect().height()`, minus a header allowance),
with axis-specific `min_px`/`max_px`.

## Log-level linkage (view-model behavior)

`src/app/mod.rs`. No new fields; existing state is made to track.

- `UiIntent::SetLogFilter(level)` now: sets `log_filter = level`; sets
  `settings.logging.level = level`; calls `log.set_level(level)`; marks the config
  store dirty (persists through the coalesced save path).
- `reload_from_settings` sets `log_filter = settings.logging.level`, so applying a
  settings Log level updates the panel dropdown.
- `ToggleLogPanel` is unchanged (panel visibility never changes verbosity).

## Toast styling

`src/app/widgets.rs`, `Toast::show`. Renders with a green success fill
(`palette.ok`) and a contrasting text color (`palette.base`) at a heavier weight.
No API change; the single existing toast (Settings saved) becomes the success
style.

## Dropdown widths

`src/app/ui.rs`. The theme, beacon environment, log level, and keybinding
`ComboBox`es are routed through the existing `combo(..)` fixed-width helper.
