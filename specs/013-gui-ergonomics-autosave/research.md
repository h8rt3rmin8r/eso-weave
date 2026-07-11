# Phase 0 Research: GUI Ergonomics, Information Design, and Auto-Save

All items below were resolvable from the egui 0.35 API surface already in use, the
existing codebase, and the constitution. No external clarification remained.

## Decision: Toggle switch widget

- **Decision**: Add a reusable `toggle_switch` helper in `src/app/widgets.rs`
  modeled on egui's canonical toggle-switch example (allocate an interactive rect,
  animate the knob with `ctx.animate_bool_responsive`, paint track and knob from the
  brand palette). On uses the gold or teal accent; off uses the muted role.
- **Rationale**: egui has no built-in switch, but a custom painted switch is a
  small, well-known pattern that keeps rendering in the thin layer while the boolean
  value and its intent stay in the view-model. Colorized on/off satisfies FR-001.
- **Alternatives considered**: keep checkboxes (rejected: the operator explicitly
  wants physical toggles); a third-party widget crate (rejected: unnecessary
  dependency for a small widget).

## Decision: Settings as a modal

- **Decision**: Use `egui::Modal` (available in egui 0.35). Size its content area to
  roughly 90 percent of `ctx.screen_rect()` so it tracks window resizing; egui's
  Modal paints a dimmed backdrop and closes on a click outside its area. Add Escape
  handling and an explicit close control.
- **Rationale**: native Modal gives the dimmed backdrop and outside-click dismissal
  the operator asked for without hand-rolling an overlay, satisfying FR-016 through
  FR-018.
- **Alternatives considered**: a separate `egui::Window` (rejected: no backdrop, not
  modal); the current inline view swap (rejected: it is the jarring behavior being
  removed).

## Decision: Resizable log panel

- **Decision**: Move the live log into `egui::TopBottomPanel::bottom("log")
  .resizable(true)` with `min_height` set to about a tenth of the window height and
  `max_height` bounded so it stops at the bottom of the interactive region. The
  interactive content moves into the `CentralPanel`.
- **Rationale**: a resizable egui panel provides the drag handle and resize cursor
  for free (FR-024) and naturally clamps content. The persisted height is re-clamped
  on load against the current window size (FR-014, edge case).
- **Alternatives considered**: a hand-drawn splitter (rejected: reimplements what the
  panel already provides); a fixed log height (rejected: the operator wants resize).

## Decision: Terminal styling for the log

- **Decision**: Fill the log with `Visuals::extreme_bg_color` (already the darkest
  palette role) and render each row with `egui::RichText::monospace`, keeping the
  per-level colors from `log_view.rs`.
- **Rationale**: reuses existing palette and font facilities; no new asset. Satisfies
  FR-023.
- **Alternatives considered**: bundling a dedicated monospace font (rejected: egui's
  built-in monospace family is sufficient and adds no asset weight).

## Decision: Coalesced (debounced) auto-save

- **Decision**: Replace the draft-and-Apply flow with immediate in-memory application
  plus a dirty flag and a short idle timer (about 400 ms). The single existing
  `config::save` choke point is called when the timer settles or when the settings
  modal closes; the session-state file uses the same coalesced trigger.
- **Rationale**: egui is immediate-mode and `DragValue` fires many changes per drag;
  coalescing yields one settle-write and one toast (FR-013, FR-015, SC-005) instead
  of thrashing the disk. The timer state lives in the view-model, not the render
  layer, so it is testable via a pure "should save now" function over an elapsed
  duration.
- **Alternatives considered**: save on every change (rejected: disk thrash, toast
  spam); save only on close or exit (rejected: loses the "nothing is ever lost"
  guarantee if the process is killed).

## Decision: Session state in a separate file

- **Decision**: Persist the live suspend and fishing intents to a new `state.json`
  in the platform config directory (alongside `config.json`, distinct file), loaded
  at startup and written through the coalesced trigger. On absence, unreadable data,
  or a state the subsystem cannot resume, fall back to safe defaults (not suspended,
  not fishing).
- **Rationale**: the constitution requires the config file to hold user settings
  only, with no session, runtime, or derived state. A separate state file honors the
  operator's persistence request (FR-011) and the constitution simultaneously
  (FR-029). Restoring under the focus-scoped invariant is safe (FR-012).
- **Alternatives considered**: a new `session` section in `config.json` (rejected:
  violates the config-content rule); not persisting session state (rejected: the
  operator explicitly requested it).

## Decision: Persisted log-panel height location

- **Decision**: Add `log_panel_height` to the existing `ui` config section (which
  already holds theme and always-on-top).
- **Rationale**: panel height is a declarative user layout preference, which is
  exactly what the config file is for; it is additive and defaults safely when
  absent (back-compat via serde defaults).
- **Alternatives considered**: the state file (rejected: it is a preference, not
  runtime state).

## Decision: Persisted logging level vs the per-panel log filter (resolves CHK033)

- **Decision**: Keep two distinct concepts. `logging.level` (in config) is the
  captured/persisted verbosity that gates what enters the ring buffer and file.
  The per-panel filter is a transient, non-persisted view filter over already
  captured events; on launch it initializes from `logging.level` (as today) and is
  not written back to disk.
- **Rationale**: they answer different questions (what to capture vs what to show
  now). Persisting only the level avoids a conflict on restore and matches the
  current initialization behavior.
- **Alternatives considered**: persist the panel filter too (rejected: it is a
  transient view control, and persisting it is session state that does not belong in
  config).

## Decision: Heading typography

- **Decision**: Register the already-bundled `Inter-Medium` and `Inter-SemiBold`
  weights in `theme::install_fonts` and render section headings with the heavier
  weight at a larger size via a `heading` helper.
- **Rationale**: the weights ship in `assets/brand/fonts/` but are currently unused;
  wiring them adds no asset and gives headings real visual weight (FR-002).
- **Alternatives considered**: synthesize bold from the regular weight (rejected:
  lower quality than the real weight that is already bundled).

## Decision: Centralized UI strings

- **Decision**: Keep tooltip and inline-help text in one place (the view-model or a
  small `strings` module) and have the render layer read from it, so the same
  concept reads identically everywhere (FR-026) and coverage is unit-testable.
- **Rationale**: satisfies the consistency requirement and lets a test assert that
  every control and column has a non-empty tooltip and that no user-facing label
  contains an underscore (FR-020, FR-025).
- **Alternatives considered**: inline literals at each call site (rejected: drift and
  no coverage test).
