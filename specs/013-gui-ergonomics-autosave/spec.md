# Feature Specification: GUI Ergonomics, Information Design, and Auto-Save

**Feature Branch**: `013-gui-ergonomics-autosave`

**Created**: 2026-07-11

**Status**: Draft

**Input**: User description: "GUI ergonomics, information design, and auto-save
overhaul for the ESO Weave desktop app (build plan 003, slice 013). Two-state
controls should be real toggles, sections need real headings and labeled columns,
tooltips and inline help everywhere, the status region renamed and colorized and
spread to full width, the live log a resizable terminal-styled panel, settings a
full-frame modal organized into clusters with no underscores and per-option help,
and every change auto-saved (including session state) with no explicit Save."

## Clarifications

### Session 2026-07-11

- Q: Should the live Suspend and Fishing states persist across restarts, or only
  configuration? (decided with the operator) -> A: Persist session state too. The
  app restores the Suspend and Fishing states it was closed in. This is safe
  because input is only synthesized or suppressed while the ESO window is focused,
  so a restored "running" or "armed" state does nothing until the game is focused.
- Q: How should help be presented without cluttering the interface? (decided with
  the operator) -> A: Hover tooltips on controls and labels across the whole app,
  plus small muted inline help text under each option in the settings modal. No
  question-mark help bubbles on the main window.
- Q: When the app closed with fishing mid-cycle, what fishing state is restored?
  (decided under autopilot) -> A: The persisted fishing state is a single on/off
  intent, not a mid-cycle sub-state. On launch, "on" restores as a clean re-arm of
  the fishing controller (equivalent to the user having just enabled it) and "off"
  restores as idle; transient sub-states (waiting, reeling, recast) are never
  persisted or resumed. This keeps restore deterministic and testable and avoids
  resuming an interrupted cycle from a stale point.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Controls that read as what they are (Priority: P1)

A user opens the main window and immediately understands it. Two-state controls
(suspend/resume, fishing on/off, per-skill enabled and override) present as
physical toggle switches with colorized on/off cues rather than identical buttons
or bare checkboxes. Sections carry real headings. The Skills grid has labeled
columns so the user knows what each field means. The top region reads "Status",
"Fishing", and "Pixel Beacon (Addon)", each with its title first and a normalized,
color-coded state field, spread across the full width of the window.

**Why this priority**: "Everything is a boring look-alike button" is the core
complaint. Making controls, headings, and columns legible is the single most
visible improvement and stands alone as a deliverable.

**Independent Test**: Launch the app and confirm every two-state control is a
toggle switch with a distinct on/off appearance, every section has a heading, the
Skills columns are labeled, and the status region uses the renamed titles with a
colorized state field spanning the window width. No dependence on the settings or
log work.

**Acceptance Scenarios**:

1. **Given** the main window is shown, **When** the user views the suspend/resume,
   fishing, per-skill enabled, and per-skill override controls, **Then** each is a
   toggle switch whose on and off states are visually distinct and color-coded, not
   a button or a bare checkbox.
2. **Given** the Skills section, **When** the user scans it, **Then** a header row
   labels each column (Skill, Enabled, Weave, Override, Delay), and every row's
   controls align under those headers.
3. **Given** a skill row with its override off, **When** the user looks at the
   Delay column, **Then** it shows the inherited global default value in a muted,
   read-only style, not a literal zero.
4. **Given** a skill row with its override on, **When** the user edits the Delay,
   **Then** the edited value applies to the delay matching that row's weave type
   (light, heavy, or bash).
5. **Given** the top region, **When** the user reads it, **Then** the three lines
   are titled "Status", "Fishing", and "Pixel Beacon (Addon)" with the title first,
   a state field whose color reflects the state (for example running in the healthy
   color, suspended in the warning color, fishing active in the accent color, idle
   or absent in the muted color, beacon current/outdated/not-installed in the
   healthy/warning/error colors), and the region spans the same width as Skills.

---

### User Story 2 - Nothing is ever lost (Priority: P1)

A user changes anything (a skill toggle, a weave type, a timing value, the theme,
the suspend or fishing state, the log panel height) and it is remembered. There is
no Save or Apply button anywhere. Changes persist to disk automatically and survive
an application restart, including the live suspend and fishing state. A gentle
bottom-right toast confirms when settings are saved and fades away on its own.

**Why this priority**: Inconsistent persistence (main-page edits lost, settings
requiring explicit Apply) is a primary frustration. Universal auto-save is
foundational to trusting the app and is independently valuable.

**Independent Test**: Change a main-page skill setting and a settings value, close
and relaunch the app, and confirm both persisted; confirm no Save/Apply control
exists and a save toast appears and auto-dismisses.

**Acceptance Scenarios**:

1. **Given** any editable control on the main window or in settings, **When** the
   user changes it, **Then** the change is written to persistent storage without
   any explicit save action.
2. **Given** the user has changed skill settings, timings, theme, and the suspend
   and fishing states, **When** the app is closed and relaunched, **Then** all of
   those values are restored to what they were at close.
3. **Given** the user drags a value control continuously, **When** the value
   settles, **Then** exactly one persisted write results (not one per intermediate
   value) and one save confirmation is shown, not a stream of them.
4. **Given** a setting was just saved, **When** the save completes, **Then** a
   subtle confirmation appears at the bottom-right and disappears on its own after a
   brief interval.
5. **Given** the app restores a "running" or "fishing armed" state on launch,
   **When** the game window is not focused, **Then** no input is synthesized or
   suppressed until the game window is focused.

---

### User Story 3 - A focused, organized settings surface (Priority: P2)

A user opens Settings and gets a full-frame modal centered over a dimmed main
window, not a jarring whole-view swap. The options are grouped into labeled
clusters (Appearance; Combat timing; Fishing; Pixel Beacon and bus; Logging;
Keybindings), every label is human-readable with no underscores, and each option
has a short line of help beneath it. Clicking outside the modal, pressing Escape,
or using the close control dismisses it; changes are already saved (User Story 2).

**Why this priority**: The settings screen is called out as messy, colorless,
unstructured, and jarring, but it depends on the auto-save behavior to drop the
Apply button, so it follows the P1 stories.

**Independent Test**: Open Settings and confirm it is a modal over a dimmed
backdrop, closes on outside-click/Escape/close control, presents grouped clusters
with headings, shows no underscored labels, and displays inline help under each
option.

**Acceptance Scenarios**:

1. **Given** the user opens Settings, **When** it appears, **Then** it is a modal
   occupying most of the window over a dimmed backdrop, rather than replacing the
   main view.
2. **Given** the settings modal is open, **When** the user clicks outside it,
   presses Escape, or activates the close control, **Then** the modal closes and the
   main window is shown.
3. **Given** the settings modal, **When** the user reads it, **Then** options are
   organized into labeled clusters with headings, and related settings are grouped
   together rather than stacked in one flat list.
4. **Given** any settings label, **When** the user reads it, **Then** it is
   human-readable words with no underscores.
5. **Given** any settings option, **When** the user looks at it, **Then** a short
   help line describes what it does.
6. **Given** the settings modal, **When** the user looks for the beacon location
   and environment options, **Then** they are present (previously not surfaced).

---

### User Story 4 - A resizable, terminal-style live log (Priority: P2)

A user opens the live log and sees a terminal-like panel: a background darker than
the rest of the app and a monospace font, with the existing per-level colors. A
draggable divider between the interactive area and the log shows a grab affordance
and resize cursor; the user can shrink the log to a small strip or grow it until it
meets the bottom of the interactive area. The chosen height is remembered.

**Why this priority**: The log is functional but visually undifferentiated and
fixed-size. Making it terminal-like and resizable is a clear, self-contained
usability win.

**Independent Test**: Open the log, confirm the darker background and monospace
font with per-level colors, drag the divider to the minimum and maximum extents,
and confirm the height is restored after a restart.

**Acceptance Scenarios**:

1. **Given** the live log is shown, **When** the user views it, **Then** its
   background is darker than the surrounding app and its text is monospace, while
   per-level colors are preserved.
2. **Given** the divider between the interactive area and the log, **When** the user
   hovers it, **Then** it shows a grab affordance and a resize cursor.
3. **Given** the divider, **When** the user drags it, **Then** the log height
   changes between a small minimum (about a tenth of the window) and a maximum that
   stops at the bottom of the interactive area, without overlapping it.
4. **Given** the user set a log height, **When** the app is closed and relaunched,
   **Then** the log panel returns to that height.

---

### User Story 5 - Guidance everywhere (Priority: P3)

A user who is unsure what a control does hovers it and gets a concise tooltip.
Section titles and column headers are likewise explained on hover. In settings,
the inline help complements the tooltips. The wording is consistent across the app.

**Why this priority**: Tooltips greatly aid a dense control panel, but they layer on
top of the structural improvements rather than standing alone, so they are lowest
priority.

**Independent Test**: Hover controls, section titles, and column headers across the
main window and settings modal and confirm each shows a concise, consistent
tooltip.

**Acceptance Scenarios**:

1. **Given** any interactive control on the main window or in settings, **When** the
   user hovers it, **Then** a concise tooltip explains what it does.
2. **Given** a section title or a Skills column header, **When** the user hovers it,
   **Then** a tooltip explains the section or column (for example what "Weave",
   "Override", and "Delay" mean).
3. **Given** the same concept appears in more than one place, **When** its tooltip
   or help text is read, **Then** the wording is consistent.

---

### Edge Cases

- The settings modal must remain usable when the window is resized while it is open;
  it tracks the current window size rather than a fixed pixel size.
- The log resize must clamp so the log can never grow past the interactive area or
  shrink to nothing; the persisted height is re-clamped on load in case the window
  is smaller than when the height was saved.
- Auto-save must not thrash the disk during a continuous drag; writes are coalesced
  so a drag produces a single settle-write.
- A persisted session state that is no longer valid (for example a fishing state the
  subsystem cannot resume) must fall back to a safe default rather than error.
- Restoring a "running" (not suspended) state on launch must not cause any input
  action while the game window is unfocused, preserving the focus-scoped safety
  invariant.
- Removing underscores from labels must not change the persisted configuration keys;
  only the displayed text changes.

## Requirements *(mandatory)*

### Functional Requirements

Controls and information design:

- **FR-001**: Every control that toggles between exactly two states (suspend/resume,
  fishing on/off, per-skill enabled, per-skill override, and each boolean setting)
  MUST present as a toggle switch with visually distinct, color-coded on and off
  states, not a button or a bare checkbox.
- **FR-002**: Section labels (Status, Skills, and each settings cluster) MUST be
  rendered as headings that are visually distinct from body text.
- **FR-003**: The Skills section MUST label its columns (Skill, Enabled, Weave,
  Override, Delay) with a header row, and each row's controls MUST align under those
  headers.
- **FR-004**: When a skill's override is off, the Delay column MUST display the
  inherited global default for that row's weave type in a muted, read-only style,
  not a literal zero.
- **FR-005**: A skill's override value MUST apply to the delay matching that row's
  weave type (light, heavy, or bash).
- **FR-006**: The top status region MUST title its lines "Status", "Fishing", and
  "Pixel Beacon (Addon)", with the title first on each line, followed by a
  normalized state field.
- **FR-007**: The state field of each status line MUST be color-coded to reflect the
  state using the brand status palette (healthy, warning, accent, muted, and error
  roles as appropriate).
- **FR-008**: The status region MUST span the same horizontal extent as the Skills
  section.

Persistence and auto-save:

- **FR-009**: The application MUST persist every user-editable value automatically
  when it changes, with no Save or Apply control anywhere in the UI.
- **FR-010**: Values previously not persisted from the main window (per-skill
  enabled, weave type, and override) MUST be persisted and restored across restarts.
- **FR-011**: The live suspend state and the live fishing on/off intent MUST be
  persisted and restored across restarts. The fishing intent restores as a clean
  re-arm when on and as idle when off; transient fishing sub-states (waiting,
  reeling, recast) are never persisted or resumed.
- **FR-012**: Restoring a non-suspended or fishing-active state on launch MUST NOT
  cause any input synthesis or suppression while the game window is unfocused; the
  focus-scoped input invariant is unchanged.
- **FR-013**: Persisted writes MUST be coalesced so a continuous edit (such as
  dragging a value) results in a single settle-write rather than one write per
  intermediate value.
- **FR-014**: The log panel height MUST be persisted and restored across restarts,
  re-clamped to the valid range on load.
- **FR-015**: A subtle bottom-right confirmation MUST appear when a save completes
  and dismiss itself after a brief interval, coalesced so a single settle produces a
  single confirmation.

Settings modal:

- **FR-016**: Settings MUST be presented as a modal occupying most of the window over
  a dimmed backdrop, rather than replacing the main view.
- **FR-017**: The settings modal MUST close on an outside click, on Escape, and on an
  explicit close control.
- **FR-018**: The settings modal MUST track the current window size (it is sized
  relative to the window, which can be resized).
- **FR-019**: Settings options MUST be organized into labeled clusters with headings
  (Appearance; Combat timing; Fishing; Pixel Beacon and bus; Logging; Keybindings),
  with related options grouped together.
- **FR-020**: No user-facing label anywhere in the UI may contain an underscore;
  labels MUST be human-readable words. Persisted configuration keys are unaffected.
- **FR-021**: Each settings option MUST show a short inline help line describing what
  it does.
- **FR-022**: The settings modal MUST surface the beacon location override and
  environment options that are persisted but currently not shown.

Live log:

- **FR-023**: The live log MUST use a background darker than the surrounding app and a
  monospace font, preserving the existing per-level colors.
- **FR-024**: The live log MUST be resizable via a draggable divider that shows a grab
  affordance and a resize cursor, clamped between a small minimum (about a tenth of
  the window height) and a maximum that stops at the bottom of the interactive area.

Tooltips and guidance:

- **FR-025**: Every interactive control, every section title, and every Skills column
  header MUST provide a concise hover tooltip explaining it.
- **FR-026**: Tooltip and help wording for the same concept MUST be consistent across
  the app, sourced from a single set of strings.

Cross-cutting:

- **FR-027**: The egui rendering layer MUST remain thin; all correctness-bearing
  behavior (state and color derivations, the coalesced-save trigger, session
  persistence, and the label, help, and tooltip strings) MUST live in the tested
  view-model and subsystem modules with unit tests.
- **FR-028**: This slice MUST NOT modify any pinned contract surface or weaken any
  safety-critical invariant; it changes presentation and persistence wiring only.
- **FR-029**: Session state (the live suspend and fishing intents) MUST be persisted
  to a separate state file, never to the configuration file, because the
  configuration file stores declarative user settings only and never session,
  runtime, or derived state. The log-panel height is a user layout preference and is
  stored in the configuration file's UI section.

### Key Entities

- **Toggle switch**: A reusable two-state control with distinct, color-coded on and
  off appearances, used wherever a boolean is edited.
- **Status line**: A titled main-window line (Status, Fishing, Pixel Beacon (Addon))
  with a normalized, color-coded state field derived in the view-model.
- **Skill row**: A labeled row exposing enabled, weave type, override, and the
  effective delay (edited value when overridden, inherited default when not).
- **Settings cluster**: A labeled group of related settings within the modal, each
  option carrying inline help.
- **Session state**: The persisted live suspend and fishing states, restored on
  launch under the focus-scoped input invariant.
- **Log layout**: The persisted, clamped live-log panel height.
- **Save confirmation**: The transient bottom-right notification shown after a
  coalesced save.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Every two-state control on the main window and in settings is a toggle
  switch with distinct on and off appearances; no such control remains a button or a
  bare checkbox.
- **SC-002**: Every section has a heading and the Skills grid has labeled columns; a
  first-time user can name each Skills column from its header alone.
- **SC-003**: With every override off, no skill row shows a literal zero for its
  delay; each shows the value actually in effect.
- **SC-004**: Changing any value on the main window or in settings and then
  relaunching the app restores that value, including the suspend and fishing states;
  no Save or Apply control exists anywhere.
- **SC-005**: A continuous drag of a value control results in exactly one persisted
  write and one save confirmation.
- **SC-006**: Settings opens as a modal over a dimmed backdrop and closes on
  outside-click, Escape, and the close control; it never replaces the main view.
- **SC-007**: No user-facing label contains an underscore, and every settings option
  shows an inline help line.
- **SC-008**: The live log renders darker than the app in a monospace font with
  per-level colors, and can be dragged between the minimum and maximum heights, with
  the height restored after a restart.
- **SC-009**: Every interactive control, section title, and Skills column header
  shows a concise tooltip on hover.
- **SC-010**: The application's existing behavior and its automated verification gate
  remain unaffected except for the intended presentation and persistence changes; no
  safety-critical invariant is weakened.

## Assumptions

- The GUI framework provides a resizable panel with a drag handle and resize cursor,
  a modal with a dimmed backdrop and outside-click dismissal, a monospace font
  family, and per-widget hover tooltips sufficient to meet these requirements without
  changing the underlying view-model.
- The bundled primary typeface already includes the additional weights needed for
  headings; no new font files are added.
- Session-state persistence is acceptable and safe because input is only ever
  synthesized or suppressed while the game window is focused (an existing invariant);
  restoring a running or armed state changes nothing until the game is focused.
- Removing underscores changes only displayed text; persisted configuration keys and
  the on-disk config format are unchanged aside from an additive UI layout setting
  (log-panel height) that defaults safely when absent. Session state is kept in a
  separate state file, not the configuration file, per the constitution's rule that
  the config stores user settings only.
- This slice is presentation, persistence wiring, and information design; it
  introduces no new combat or input behavior, touches no pinned contract, and traces
  to the master specification's GUI layer (section 10).
