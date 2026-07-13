# Phase 0 Research: Settings Modal, Logging, and Keys

## Why is F2 missing and how is it fixed?

`Key::F2` exists in `src/input/key.rs` and round-trips through `as_str`/`parse`,
but the GUI selectable list `const KEYS` in `src/app/ui.rs` is an 11-element array
that omits it. The keybinding dropdowns iterate `KEYS`, so F2 never appears and a
binding set to F2 (the Toggle Fishing default) shows no matching option. Adding
`Key::F2` to the array (length 12) makes it selectable and displayable.

## How are the two log controls currently split, and how are they linked?

The live-log panel dropdown raises `SetLogFilter(level)`, which sets only
`AppModel.log_filter`, a display filter over already-captured events; it does not
change capture or persist. The settings Log level sets `draft.logging.level`, which
on apply flows through `reload_from_settings` to `log.set_level(..)` (the capture
threshold) and persists. They are seeded from the same value at construction and
then diverge.

To link them, `SetLogFilter` is changed to also set `settings.logging.level`, call
`log.set_level(level)`, and mark the config store dirty (so it persists through the
existing coalesced save path), keeping `log_filter` mirrored. `reload_from_settings`
sets `log_filter = settings.logging.level` so applying a settings Log level updates
the panel dropdown. With the display threshold equal to the capture level, the
panel shows everything captured. `ToggleLogPanel` only toggles `log_panel_open`, so
hiding the panel does not touch the level (satisfying the exclusion requirement).

## How should the modal scale with the window?

The modal reads `ctx.content_rect()` each frame (so it already tracks resizes), but
width is `min(width*0.9, 720)` and the body height is `height*0.78`: different
fractions, a low width cap, and the height not following the width rule. The fix is
one pure helper `modal_extent(window, min_px, max_px, max_frac)` that returns
`min_px` at the smallest window, grows at a fraction of window growth (sub-linear,
so the occupied fraction decreases as the window enlarges), clamps to
`[min_px, max_px]`, and finally caps at `max_frac` of the window (so it never
exceeds the window even when very small). It is applied to the modal width and to
the scroll-area max height (minus a header allowance) each frame, with axis-specific
caps chosen to look right from the minimum window up to a QHD ultrawide display.

## How is the toast made green and legible?

`widgets::Toast::show` currently fills with `palette.elevated` and draws
`palette.text`. Filling with the brand success color `palette.ok` and drawing the
text in `palette.base` (the near-black or near-white base surface, which contrasts
with the vivid green in each theme) at a heavier weight yields a clearly green,
legible confirmation in both themes, rather than a mid-green text that would fail
contrast on a dark panel.

## Alternatives considered

- **A single log-level field (remove `log_filter`)**: rejected as a larger change;
  mirroring keeps the existing view/plumbing and the display-filter path intact
  while making them track.
- **Green text or a thin green border only**: rejected as too subtle for "capture
  attention"; a green fill is unmistakable, and pairing it with a contrasting base
  text color keeps it legible.
- **A pure percentage modal (e.g. always 90%)**: rejected; it either overflows a
  small window or grows unbounded on an ultrawide display. The sub-linear helper
  with a cap gives increasing pixels but a decreasing fraction.
