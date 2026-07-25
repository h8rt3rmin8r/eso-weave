# Phase 1 Data Model: UI Window-Sizing and Layout Hardening

This slice adds no persisted schema. It introduces in-memory layout quantities
and one scheduler field. Existing persisted stores (config, session state) are
unchanged in schema and in written content.

## In-memory quantities (transient, per-frame)

### ContentMinSize

The measured minimum inner size the window must have to show every interactive
control without clipping.

- **width: f32** (points) - the horizontal floor; must fit the widest row,
  including the Pixel Beacon Install/Update/Uninstall row.
- **height: f32** (points) - the vertical floor; must fit every row including the
  Weapon Bar row and the bottom Skills grid row, plus the menu bar and separator.

**Derivation**: running maximum of the central panel's `min_rect().size()` over
the first stable frames.
**Validity rules**: never below the compile-time boot floor `MIN_SIZE`; monotone
non-decreasing within a session (running max) so a transient under-measure cannot
shrink the window.
**Consumers**: the viewport `MinInnerSize` command (#4), and the log-pane
`max_size` upper bound `height_of_window - ContentMinSize.height` (#5d).

### LogPaneGeometry (derived, not persisted beyond the existing height pref)

- **min_height: f32** (points) - `log_min_height(row_height)` = six log rows plus
  frame margins.
- **max_height: f32** (points) - `window_height - ContentMinSize.height`, floored
  at `min_height`.
- **width_bonus: f32** (points) - fixed increment added to the enforced minimum
  width while the pane is open (#5c).
- **current_height: u32** (points) - the existing persisted UI preference
  `log_panel_height`; unchanged in meaning, now clamped by `clamp_log_height`
  using the new `min_height`.

## Persisted entities (unchanged)

### Config store (user settings only)

- `ui.log_panel_height: u32` - already present; still written the same way. Only
  the clamp bounds change (six-line minimum), not the field or its persistence.
- No new config field is added by this slice.

### Session state store

- `window: WindowGeometry { x, y, width, height, maximized }` - already present;
  still written the same way. On restore, `sanitize_geometry` now floors `width`
  and `height` at the current `ContentMinSize` in addition to the existing
  `MIN_SIZE`.

## SaveScheduler (in-memory, extended)

Existing fields: `dirty_config: bool`, `dirty_session: bool`, `last_change:
Option<Instant>`, `settle: Duration`.

- **dirty_notify: bool** (new) - set true by a meaningful mark (a real settings
  change), left false by a layout-only mark (window geometry, log height).
  Cleared by `take()` alongside the other dirty flags.

**State transitions**:
- `mark_config(now)` / `mark_session(now)`: set the respective dirty flag AND
  `dirty_notify = true` (these are the meaningful marks: toggles, form fields,
  filter changes).
- `mark_config_layout(now)` / `mark_session_layout(now)` (new silent variants):
  set the respective dirty flag, set `last_change`, but do NOT set
  `dirty_notify`. Used only by `SetLogHeight` and `SetWindowGeometry`.
- `take()`: returns the config/session flags (and the notify flag via
  `maybe_flush`), then clears all three.

**Invariant**: `dirty_notify` implies `dirty_config || dirty_session` (a notify is
always accompanied by a real write). A layout-only batch has
`dirty_notify == false` while still being dirty, so it persists silently.

## Relationships

```text
central panel content  --measure-->  ContentMinSize.height/width
                                         |                 |
                                         v                 v
                          viewport MinInnerSize     log max_height (window_h - height)
                                                           ^
log text row height --> log_min_height() --> LogPaneGeometry.min_height --> clamp_log_height

SetLogHeight / SetWindowGeometry --mark_*_layout--> SaveScheduler (dirty, notify=false) --> silent persist
toggles / form fields            --mark_*-------->  SaveScheduler (dirty, notify=true)  --> persist + toast
```
