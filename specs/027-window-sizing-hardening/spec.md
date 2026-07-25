# Feature Specification: UI Window-Sizing and Layout Hardening

**Feature Branch**: `027-window-sizing-hardening`

**Created**: 2026-07-24

**Status**: Draft

**Input**: User description: "UI window-sizing and layout hardening. Bundle four
user-facing UI defects/enhancements (GitHub issues #4, #5, #6, #7) so the
application window always shows every interactive control, the live-log viewer
behaves correctly, the save confirmation is not noisy, and the controls are less
tall."

## Clarifications

### Session 2026-07-24

The ambiguity scan found no critical ambiguities: scope, requirements, success
criteria, entities, and edge cases are all clear, and the three headline
decisions (all four issues in scope, dynamic content-extent minimum, release as
v0.7.0) are already pinned. Three minor design points are resolved here under
autopilot so planning and tasks inherit them unambiguously.

- Q: On disabling the live log viewer, does the window restore its exact
  pre-open height or shrink only to the controls-only minimum? → A: It shrinks
  back by the same amount it grew on open (restoring the pre-open height), so
  toggling the viewer on then off is height-neutral.
- Q: The log pane's "at least six lines" is measured at what text size? → A: At
  the log pane's own text style size (the size the log lines are actually drawn
  at).
- Q: How much wider is the enforced minimum window width while the log viewer is
  open, relative to the base minimum? → A: A fixed increment above the base
  content-derived minimum width, chosen during planning so typical log lines wrap
  less; the exact figure is a plan-level design detail, not a spec requirement.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Every control stays visible at the smallest window (Priority: P1)

A user drags the application window as small as it will go, or launches with a
previously tiny saved size, and still sees every interactive control in full.
Today, at the smallest allowed height the bottom row of the Skills area is
clipped, and at the smallest allowed width the Pixel Beacon row (Install /
Update / Uninstall) is cut off. The smallest allowed window must always fit the
full content, and that floor must keep fitting the content as rows are added in
future without a person re-tuning a number by hand.

**Why this priority**: A clipped control is an unusable control. This is the most
severe of the four defects because it hides functionality the user cannot reach,
and it silently regresses every time the layout grows.

**Independent Test**: Shrink the window to its minimum in both dimensions and
confirm no control is clipped, with the live log viewer off. Delivers value on
its own: the app is fully operable at any size it allows.

**Acceptance Scenarios**:

1. **Given** the live log viewer is off, **When** the user drags the window to
   its minimum height, **Then** every interactive field, including the bottom
   Skills row, is fully visible with no vertical overflow.
2. **Given** the live log viewer is off, **When** the user drags the window to
   its minimum width, **Then** the Pixel Beacon row (Install / Update /
   Uninstall) is fully visible and not clipped horizontally.
3. **Given** a future release adds another row of controls, **When** the app
   computes its minimum size, **Then** the floor grows to fit the new content
   without a hand-edited constant.
4. **Given** a saved window size smaller than the current content minimum,
   **When** the app restores that geometry on launch, **Then** the window opens
   no smaller than the content minimum.

---

### User Story 2 - The live log viewer never covers the controls (Priority: P2)

A user turns on the live log viewer to watch activity. The log pane must appear
in its own space below the controls, tall enough to read, without covering the
Skills area, and the user must not be able to drag its top edge up over the
controls.

**Why this priority**: The log viewer is an opt-in diagnostic surface. When it
covers the Skills area it makes the primary controls unreachable while it is
open, which is a serious usability failure, but it only affects users who enable
the viewer, so it ranks below the always-present clipping in P1.

**Independent Test**: Enable the log viewer from the default window size and
confirm the window grows to host it, the pane shows at least six lines, lines
wrap less because the window is wider, and the resize bar cannot be dragged into
the controls.

**Acceptance Scenarios**:

1. **Given** the app is at its default size with the log viewer off, **When** the
   user enables the live log viewer, **Then** the overall window grows by at
   least the log pane's minimum height and no existing control is covered.
2. **Given** the log viewer is on at its minimum pane height, **When** the user
   reads the pane, **Then** at least six lines of log text are visible.
3. **Given** the log viewer is on, **When** the user compares window width to the
   viewer-off state, **Then** the minimum window width is wider so log lines wrap
   less.
4. **Given** the log viewer is on, **When** the user drags the resize bar above
   the pane upward as far as it will go, **Then** the pane top stops before the
   Skills area and never crosses into or above it.
5. **Given** the log viewer is on, **When** the user turns it off, **Then** the
   window returns to a size appropriate for the controls alone.

---

### User Story 3 - The save confirmation only appears for real changes (Priority: P3)

A user moves the window, resizes it, or drags the log-pane divider. These are
layout adjustments, not settings changes, and must not raise the "Settings saved"
confirmation, yet the window position and log height must still be remembered.
The confirmation appears only when the user actually changes a setting (a toggle
or a form field).

**Why this priority**: This is a noise/annoyance defect, not a loss of
functionality. Persistence already works correctly; only the confirmation is
mis-triggered. It ranks below the two visibility defects.

**Independent Test**: Move and resize the window and drag the log divider, and
confirm no "Settings saved" confirmation appears; then toggle a setting and
confirm it does appear; then relaunch and confirm the window position and log
height were still remembered.

**Acceptance Scenarios**:

1. **Given** the app is running, **When** the user moves or resizes the window,
   **Then** no "Settings saved" confirmation appears.
2. **Given** the log viewer is open, **When** the user drags the log-pane
   divider, **Then** no "Settings saved" confirmation appears.
3. **Given** the app is running, **When** the user changes a setting (a toggle or
   a form field), **Then** the "Settings saved" confirmation appears.
4. **Given** the user has moved the window and resized the log pane without
   changing settings, **When** the user relaunches the app, **Then** the window
   position and log height are restored as last left.

---

### User Story 4 - Controls take less vertical space (Priority: P4)

A user finds the controls taller than they need to be, which makes the window
feel heavy and adds to the vertical space the window must reserve. The buttons,
toggle controls, and dropdown menus are made shorter (by up to about a fifth)
consistently across the app, with their text still fully legible.

**Why this priority**: This is a polish enhancement, not a defect. It improves
feel and reduces the minimum height pressure that P1 addresses, but nothing is
broken without it, so it ranks last.

**Independent Test**: Compare control heights before and after and confirm the
reduction is applied consistently to buttons, toggles, and dropdowns, with no
clipped or overflowing text in either light or dark theme.

**Acceptance Scenarios**:

1. **Given** the app is displayed, **When** the user views the buttons, toggle
   controls, and dropdown menus, **Then** each is shorter than before by a
   consistent amount of up to about 20 percent.
2. **Given** the reduced control heights, **When** the user reads any control's
   label, **Then** the text is fully legible and never clipped or overflowed, in
   both light and dark themes.

---

### Edge Cases

- What happens when the display is very small (smaller than the content
  minimum)? The window must still not shrink below the content minimum; the user
  relies on the operating system to scroll or reposition, and the app never
  reports a size smaller than its content needs.
- What happens when the log viewer is enabled while the window is already at the
  screen's maximum height and cannot grow further? The pane must still open
  without covering the controls; if growth is impossible, the controls remain
  the priority and the pane takes the remaining space down to its readable
  minimum.
- What happens when a saved log height, from a previous version or an edited
  file, is larger or smaller than the allowed range? It is clamped into the
  valid range on load.
- What happens when the user changes a setting and moves the window in the same
  brief moment? The save confirmation still appears, because a real settings
  change occurred in that batch.
- Does the minimum-size floor account for the shorter controls from User Story 4?
  Yes; because the floor is derived from the actual laid-out content, shorter
  controls produce a correspondingly smaller floor automatically.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The application MUST prevent the window from being sized smaller
  than the extent needed to display every interactive control without clipping,
  in both width and height, while the live log viewer is off.
- **FR-002**: The minimum-size floor MUST be derived from the actual laid-out
  content rather than a fixed hand-tuned value, so that adding controls in future
  raises the floor automatically.
- **FR-003**: The application MUST never restore a saved window geometry smaller
  than the current content minimum; a smaller saved size is raised to the
  minimum on launch.
- **FR-004**: Enabling the live log viewer MUST increase the overall window
  height by at least the log pane's minimum height so that no existing control is
  covered.
- **FR-005**: The live log pane MUST present at least six lines of log text at
  its minimum height.
- **FR-006**: While the live log viewer is open, the application MUST enforce a
  minimum window width wider than the base minimum so that log lines wrap less.
- **FR-007**: The application MUST hard-limit the log-pane resize so its top edge
  can never move into or above the Skills area, regardless of how far the user
  drags the resize bar.
- **FR-008**: Disabling the live log viewer MUST shrink the window height back by
  the same amount enabling it grew the window, so toggling the viewer on then off
  leaves the window at its pre-open height (never smaller than the content
  minimum).
- **FR-009**: The application MUST show the "Settings saved" confirmation only
  when a meaningful settings change (a toggle or a form-field edit) is persisted.
- **FR-010**: Window moves, window resizes, and log-pane resizes MUST continue to
  be persisted (window geometry and log height) but MUST NOT raise the "Settings
  saved" confirmation.
- **FR-011**: The application MUST reduce the height of buttons, toggle controls,
  and dropdown menus by a consistent amount of up to about 20 percent, applied
  through the shared control style rather than per control.
- **FR-012**: The reduced control heights MUST keep every control's text fully
  legible with no clipping or overflow in both light and dark themes; if a 20
  percent reduction would clip text, the largest reduction that keeps text
  legible MUST be used and the chosen figure recorded.
- **FR-013**: All persistence behavior (config and session state) MUST remain
  unchanged in what is written; only the confirmation trigger and the size floors
  change.

### Key Entities

- **Window geometry**: The window's position, size, and maximized state, saved as
  session state and restored on launch. Subject to the content-minimum floor.
- **Log pane height**: A layout preference for the live log viewer's height,
  saved and restored, clamped into a valid range bounded below by the six-line
  minimum and above by the Skills-area hard limit.
- **Content minimum extent**: The smallest width and height needed to show every
  interactive control without clipping, derived from the laid-out content and
  consumed by the window minimum size and the log-pane upper limit.
- **Save confirmation trigger**: The signal that distinguishes a meaningful
  settings change (which shows the confirmation) from a layout-only change (which
  does not).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: At the minimum window height with the log viewer off, 100 percent
  of interactive controls are fully visible (zero clipped rows), verified in both
  themes.
- **SC-002**: At the minimum window width, the Pixel Beacon row is fully visible
  (zero horizontal clipping).
- **SC-003**: Enabling the live log viewer covers zero existing controls, and the
  pane shows at least six lines of log text at its minimum height.
- **SC-004**: The log-pane resize bar cannot be positioned such that any Skills
  control is covered, in 100 percent of drag attempts.
- **SC-005**: Moving or resizing the window and dragging the log divider produce
  zero "Settings saved" confirmations, while a genuine settings change produces
  exactly one; window position and log height still survive a relaunch.
- **SC-006**: Buttons, toggles, and dropdowns are shorter by a consistent figure
  of up to 20 percent, with zero instances of clipped or overflowed control text.
- **SC-007**: A future added control row raises the minimum-size floor with no
  code change to a size constant (the floor tracks content).

## Assumptions

- The four issues are treated as one cohesive slice because they share the same
  window-sizing surfaces and interact: the shorter controls from User Story 4
  change the content extent that User Story 1 measures, and the content extent
  from User Story 1 bounds the log-pane limit in User Story 2.
- "At least six lines" refers to lines of the log text at the log pane's own text
  size.
- The exact percentage for the control-height reduction is a target of about 20
  percent; the final figure is whatever keeps all control text legible, recorded
  during implementation.
- No change is made to what is persisted; window geometry and log height continue
  to be saved exactly as today. Only the confirmation trigger and the size floors
  change.
- All behavior in this feature is verifiable at the desk without the running
  game; no in-game validation is required for this slice.
- The work is bounded to the graphical interface and its saved layout state; it
  touches no fishing, weave, input, pixel-bus, or PixelBeacon addon behavior.
