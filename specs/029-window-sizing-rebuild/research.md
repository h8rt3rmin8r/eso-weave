# Research: Window Sizing Model Rebuild

Phase 0 findings. Each decision resolves a design question behind issue #8.

## Decision 1: Replace the permanent running max with a stable-measured minimum

- **Decision**: The enforced window minimum is the boot floor per dimension until a
  content measurement is stable, then the measured extent (which may shrink). A
  measurement is stable once two consecutive frames agree within ~0.5pt.
- **Rationale**: The v0.7.0 bug is that `content_min` is a monotone running max
  seeded at the 480x420 boot floor (`src/app/ui.rs:383-390`), so the floor and any
  transient first-frame layout latch permanently. Making measured win once stable
  removes the ~20 percent dead band and lets the minimum track real content
  (adding/removing a control row adjusts it). The two-frame gate prevents a
  transient first layout from setting the minimum, without a heuristic frame count.
- **Alternatives considered**:
  - Keep the running max but reset it on content change. Rejected: needs a change
    signal and still risks latching a bad frame; measured-wins is simpler.
  - Use a fixed frame counter (skip first N frames). Rejected: brittle and not a
    pure function; two-consecutive-equal is layout-driven and unit-testable.

## Decision 2: Compute the log max against the true content height

- **Decision**: The log pane's available maximum is `window_h - content_extent.y`
  where `content_extent.y` is the current measured central height.
- **Rationale**: With the inflated running max removed, the existing
  `clamp_log_height(height, window, row, content_h)` already expresses the right
  ceiling; feeding it the true content height eliminates the phantom reserved band
  and makes the pane resizable at normal window sizes.
- **Alternatives considered**: Reserve a fixed central height. Rejected: that is the
  original bug in another form.

## Decision 3: Proportional split of window growth (DECIDED in the issue)

- **Decision**: On a window-height delta while the log is open, move the log by its
  live fraction of the usable height, `Fl = log_h / prev_window_h`, then clamp to
  `[log_min, max(window_h - content_h, log_min)]`; the central pane takes the
  remainder (`window_h - log_h`), absorbing any rounding.
- **Rationale**: Keeps the visual balance the user set (a 70/30 split stays ~70/30
  as the window grows or shrinks), which the issue explicitly decided. Deriving the
  fraction from live pane heights each resize (not a stored ratio) means it always
  reflects what the user currently sees.
- **egui mechanism**: The log stays an egui resizable bottom panel. On a frame where
  the window height changed, drive the panel to the split-computed height (force it
  for that frame, for example by collapsing the height range to the target); on
  other frames use the full `[log_min, max]` range and read the realized height back
  into `log_h` (the existing readback at `src/app/ui.rs:311-315`). The exact egui
  0.35 call is confirmed at build; the split math is pure and tested independently.
- **Alternatives considered**:
  - Give all growth to the log. Rejected: the issue decided proportional.
  - Store a ratio and reapply. Rejected: drifts from what the user sees; the issue
    asks for a live-derived fraction.

## Decision 4: One extra line of drag room at the enforced minimum open height

- **Decision**: Enforced minimum open-window height = `content_h +
  open_log_reserve(row_h)`, `open_log_reserve = log_min_height(row_h) + row_h`.
- **Rationale**: A bare six-line reserve makes `max == min` at the minimum (the
  pane is frozen, the reported bug) or, if we reserve the full current log height,
  forbids shrinking the window. Reserving six lines plus one row gives the pane a
  one-line range at the minimum (max strictly > min, satisfying the issue's item 5)
  while still letting the window shrink and the log compress toward six lines.
- **Alternatives considered**:
  - Reserve the persisted log height in the minimum. Rejected: couples the window
    minimum to the log size and blocks window shrink when the log is large.
  - Reserve exactly six lines. Rejected: pane frozen at the minimum (the bug).

## Decision 5: Height-neutral open/close by the pane's actual height

- **Decision**: Open grows the window by the log height actually shown (the
  persisted height, clamped); close shrinks by the pane's actual realized height,
  not a fixed `log_min` delta.
- **Rationale**: The v0.7.0 code grows by `log_min` on open and shrinks by `log_min`
  on close (`src/app/ui.rs:354-364`), so a user-enlarged log leaves surplus
  emptiness on close. Using the actual pane height makes the round trip neutral. The
  persisted `log_panel_height` (`UiPrefs`) is the single source of truth so egui's
  panel persistence does not fight the restore.
- **Alternatives considered**: Keep the fixed delta. Rejected: that is the secondary
  defect in the issue.

## Testability note

The four pure helpers (`measurement_stable`, `content_min_size` with the stable
flag, `split_log_height`, `open_log_reserve`) plus the existing `log_min_height`
and `clamp_log_height` cover every new invariant without a live egui context. The
only non-pure part is the thin per-frame glue and the egui panel-height call, which
is validated by building and the manual desk quickstart.
