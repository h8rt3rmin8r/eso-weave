# Phase 0 Research: UI Window-Sizing and Layout Hardening

All research is scoped to egui/eframe 0.35 (the pinned dependency) and the
existing `eso-weave` GUI code. No NEEDS CLARIFICATION markers remained after the
spec/clarify phases; the items below record the technical decisions that back the
plan's design.

## R1: Measuring the laid-out content extent

**Decision**: Inside the `CentralPanel` closure, after `main_view` lays out its
content, read the panel's tight content bounds with `ui.min_rect().size()`. Track
a running maximum of that size across frames (content can settle over the first
few frames as fonts/galleys resolve). Feed the running max, plus the menu bar and
separator already inside the panel, into a pure `content_min_size` helper that
returns the `(width, height)` floor.

**Rationale**: `Ui::min_rect()` is the bounding box egui actually used for the
content, which is exactly the "fits every control" quantity FR-001/FR-002 want.
A running max avoids a transient first-frame under-measure shrinking the floor.
The existing code already reads `ctx.content_rect()` (the available area), which
is the wrong quantity for a fit-to-content floor because it reflects the current
window, not the content need.

**Alternatives considered**:
- `ctx.used_rect()` (union of all panels/areas): includes the bottom log panel
  and any transient areas (toast, menus), so it would couple the floor to the log
  pane. Rejected; we want the central content alone.
- Hand-tuned constant widened by inspection: the status-quo approach that
  regressed when the Weapon Bar row was added. Rejected per FR-002.

## R2: Raising the viewport minimum size at runtime

**Decision**: Use `ctx.send_viewport_cmd(egui::ViewportCommand::MinInnerSize(
egui::vec2(min_w, min_h)))` to raise the window's minimum inner size once the
measured floor is known, and only when it changes (guard against per-frame
spam). The compile-time `MIN_SIZE` in `src/main.rs` stays as the initial
`with_min_inner_size` boot floor used before the first measurement and by
`sanitize_geometry`.

**Rationale**: egui 0.35 exposes `ViewportCommand::MinInnerSize`; the codebase
already drives the viewport this way for `WindowLevel` and `Close`
(`src/app/ui.rs`). Sending only on change keeps it cheap and avoids fighting a
user resize.

**Alternatives considered**:
- Rebuild `NativeOptions` (not possible after launch). Rejected.
- Clamp the window in the app's own resize handler: egui does not expose a
  pre-resize veto; the viewport minimum is the supported mechanism. Rejected.

## R3: Growing/shrinking the window when the log pane toggles

**Decision**: On `ToggleLogPanel(true)`, read the current inner size and send
`ViewportCommand::InnerSize` with height increased by the log pane's minimum
height; on `ToggleLogPanel(false)`, send it decreased by the same amount (never
below the content-min floor). Keep the added amount in one place so open/close
are symmetric (height-neutral).

**Rationale**: FR-004 (grow, never cover controls) and FR-008 (height-neutral
toggle). `InnerSize` is the resize command; combined with the raised
`MinInnerSize` while open, the window cannot end up too small.

**Alternatives considered**:
- Let the bottom panel steal from the central panel (current behavior): the
  documented defect. Rejected.
- Grow by the last-used log height instead of the minimum: risks a large jump;
  the minimum is the guaranteed-safe delta and the user can still drag larger.
  Chosen minimum for predictability.

## R4: Six-line log minimum height

**Decision**: Compute the minimum log-pane height from the log text style's row
height times six, plus the panel frame's inner vertical margins (the code uses
`Margin::same(6)`), in a pure helper `log_min_height(row_height)`. Use the same
helper for the panel `min_size` and inside `clamp_log_height` so the two never
diverge.

**Rationale**: FR-005 ("at least six lines" at the log text size, per the
Clarifications). Row height is available from the egui style's text style; passing
it into a pure function keeps the helper testable without a live context.

**Alternatives considered**:
- Keep `(window_h * 0.1).max(48.0)`: about one line at the default height, the
  documented defect. Rejected.
- A fixed pixel constant: does not track the actual font size. Rejected in favor
  of deriving from row height.

## R5: Hard top-clamp of the log resize bar

**Decision**: Set the bottom panel's `max_size` to
`window_h - content_min_height` (the measured central floor), not `window_h *
0.75`. When the window is too short for both, the central content wins and the
pane collapses toward its six-line minimum.

**Rationale**: FR-007 (the pane top can never cross into the Skills area). Tying
`max_h` to the same measured floor as #4 is what makes the clamp exact rather
than a heuristic fraction.

**Alternatives considered**:
- Keep `window_h * 0.75`: allows the pane to cover most of the window including
  the Skills grid. Rejected; it is the defect.

## R6: Distinguishing meaningful changes from layout writes (toast)

**Decision**: Add a `dirty_notify: bool` to `SaveScheduler`, set by the existing
`mark_config` / `mark_session` (the "meaningful" marks) and left unset by two new
silent variants used only by the layout intents (`SetLogHeight`,
`SetWindowGeometry`). `take()` returns the flag (or `maybe_flush` returns it) so
the toast at the flush site fires only when a meaningful change was in the batch.
Persistence is unchanged: the layout marks still set `dirty_config` /
`dirty_session` and still settle into a write.

**Rationale**: FR-009/FR-010/FR-013. The current `(config, session)` split cannot
separate a log-height write (config) from a real toggle (config), so a dedicated
"notify" signal is the minimal correct change. The scheduler is the single
coalescing decision point and is already unit-tested
(`tests/app_session_state.rs`), so the flag is directly testable.

**Alternatives considered**:
- Inspect the intent variant at the toast site: spreads the layout-vs-settings
  knowledge into the view and does not compose with coalescing (a batch can mix
  both). Rejected.
- Suppress the toast for a fixed time after a resize: fragile timing heuristic.
  Rejected.

## R7: Control-height reduction without clipping text

**Decision**: Reduce `spacing.interact_size.y` (and align `button_padding` and the
combo min height) once in `theme::apply`, targeting a ~20 percent reduction. Bound
the reduction so the interior height is never less than the control font's line
height plus a small padding, guaranteeing text is not clipped. Record the final
figure in this plan and the CHANGELOG.

**Rationale**: FR-011 (consistent, centralized) and FR-012 (legibility over a
fixed percentage). `toggle_switch` already derives its height from
`interact_size.y` (`src/app/widgets.rs`), and buttons/combos read the shared
spacing, so one style change propagates everywhere. Both light and dark themes use
the same style path, so the check covers both.

**Alternatives considered**:
- Per-call height overrides: violates FR-011 (not centralized) and is easy to
  miss on a control. Rejected.
- A hard 20 percent regardless of font: risks clipping at the current body size.
  Rejected in favor of a legibility-bounded reduction.
