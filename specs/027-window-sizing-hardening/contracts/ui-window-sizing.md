# Contract: UI Window-Sizing and Layout Helpers

This is the behavioral contract for the pure helpers and the scheduler change
that back slice 027. It is the UI/internal contract (the app exposes no external
API); each clause is directly unit-testable without a live window.

## C1: content_min_size (pure)

Signature (intent):
`content_min_size(measured: Vec2, boot_floor: Vec2) -> Vec2`

- **MUST** return a size no smaller than `boot_floor` in either dimension.
- **MUST** return `measured` where `measured` exceeds `boot_floor`, so the floor
  tracks the actual laid-out content.
- **MUST** be monotone with respect to a running max caller: given a sequence of
  measured sizes, feeding the running maximum never yields a smaller result than a
  previous frame (no shrink from a transient under-measure).
- **MUST NOT** depend on the current window size (it is a content floor, not a
  window readout).

Acceptance: FR-001, FR-002. Tests cover measured-below-floor (returns floor),
measured-above-floor (returns measured), and per-dimension independence
(width above floor, height below floor -> width from measured, height from floor).

## C2: log_min_height (pure)

Signature (intent): `log_min_height(row_height: f32) -> f32`

- **MUST** return at least six times `row_height`.
- **MUST** add the pane frame's vertical inner margins so six lines are actually
  visible inside the frame.
- **MUST** be strictly increasing in `row_height` (a larger font yields a taller
  minimum).

Acceptance: FR-005. Tests cover a representative row height (result >= 6 * row +
margins) and monotonicity (larger row_height -> larger result).

## C3: clamp_log_height (pure, updated)

Signature (unchanged): `clamp_log_height(height: f32, window_height: f32) -> f32`

- **MUST** clamp below at `log_min_height(row_height)` (the six-line minimum),
  replacing the old `(window_height * 0.1).max(48.0)` lower bound.
- **MUST** clamp above at the log-pane maximum
  `window_height - content_min_height` (never allowing the pane top to cross the
  Skills area), replacing the old `window_height * 0.75`.
- **MUST** return a value within `[min, max]` for any finite input, and return
  `min` when `min > max` (degenerate tiny window: pane collapses to the readable
  minimum, central content is not covered).

Acceptance: FR-005, FR-007. Tests cover below-min raised to min, above-max
lowered to max, and the degenerate `min > max` case returning min.

## C4: SaveScheduler notify flag

- A **meaningful mark** (`mark_config`, `mark_session`) **MUST** set the notify
  flag; after settle, `maybe_flush` **MUST** report a meaningful change (toast
  shows).
- A **layout mark** (`mark_config_layout`, `mark_session_layout`, used by
  `SetLogHeight` and `SetWindowGeometry`) **MUST NOT** set the notify flag; after
  settle, `maybe_flush` **MUST** report no meaningful change (no toast), yet the
  corresponding store **MUST** still be written (persistence unchanged).
- A **mixed batch** (a layout mark and a meaningful mark before settle) **MUST**
  report a meaningful change (toast shows), because a real settings change
  occurred.
- `take()` **MUST** clear the notify flag alongside the dirty flags.
- The invariant `notify => (dirty_config || dirty_session)` **MUST** hold.

Acceptance: FR-009, FR-010, FR-013. Tests (in `tests/app_session_state.rs`):
layout-only settle -> no notify but dirty/written; meaningful settle -> notify;
mixed settle -> notify; post-`take` flags cleared.

## C5: Viewport commands (integration, validated manually)

These are not pure and are validated via `quickstart.md`, not unit tests:

- Enabling the log viewer **MUST** raise the window inner height by
  `log_min_height` and **MUST NOT** reduce the central content area
  (FR-004).
- Disabling the log viewer **MUST** lower the window inner height by the same
  amount (FR-008), never below `content_min_size.height`.
- While the log viewer is open, the enforced minimum width **MUST** be
  `content_min_size.width + width_bonus` (FR-006); on close it returns to
  `content_min_size.width`.
- The measured `content_min_size` **MUST** be pushed to the viewport as
  `MinInnerSize` only when it changes (FR-001, FR-003).

## C6: Control-height style (integration, validated manually)

- Buttons, toggle controls, and dropdowns **MUST** be shorter by a single
  consistent figure of up to ~20 percent, set once in the shared style (FR-011).
- No control text is clipped or overflowed at the reduced height in either light
  or dark theme (FR-012). The final figure is recorded in `plan.md` (D1) and the
  CHANGELOG.
