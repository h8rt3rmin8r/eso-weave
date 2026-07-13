# Feature Specification: Settings Modal, Success Toast, Logging Linkage, and Key Presentation

**Feature Branch**: `023-settings-modal-logging-keys`

**Created**: 2026-07-13

**Status**: Draft

**Input**: User description: "Settings modal, success toast, logging linkage, and key presentation (build plan 006, slice 023): make the settings modal scale correctly with the window, give the saved toast a green success color, link the live-log verbosity to the settings log level, show friendly key names, and add the missing F2 key to the selectable list."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Bind Toggle Fishing to F2 (Priority: P1)

A user opens settings to review or change keybindings. The Toggle Fishing action
defaults to F2, and the keybindings dropdown lists F2 as a selectable option so
the user can see and rebind it.

**Why this priority**: This is a functional defect, not just polish. F2 is a
default binding but is missing from the selectable list, so the user cannot see or
change the Toggle Fishing key, and any action bound to F2 shows no matching option.

**Independent Test**: Open the keybindings and confirm F2 appears as an option and
the Toggle Fishing binding can be set to it.

**Acceptance Scenarios**:

1. **Given** the keybindings settings, **When** the user opens a key dropdown,
   **Then** F2 appears as a selectable option.
2. **Given** a binding whose current key is F2, **When** the user views it,
   **Then** the dropdown shows F2 as the selected value (not blank or wrong).

---

### User Story 2 - Readable key names (Priority: P2)

A user reading keybindings sees friendly names (Number 1 through Number 5, E, R,
X, Q, Space, F1, F2) rather than raw internal strings like digit1 or space.

**Why this priority**: The raw canonical strings are developer-facing and read as
unfinished; friendly names make the bindings clear.

**Independent Test**: Open the keybindings and confirm each key reads as its
friendly name in both the selected value and the option list.

**Acceptance Scenarios**:

1. **Given** a key dropdown, **When** the user views the options, **Then** each
   option reads as its friendly name (for example Number 1, Space, F1) rather than
   the raw canonical string.
2. **Given** a binding, **When** the user views its selected key, **Then** it shows
   the friendly name; the stored and parsed key values are unchanged.

---

### User Story 3 - Log verbosity that stays in sync (Priority: P2)

A user changes the log verbosity from the live-log panel dropdown or from the
settings Log level; either change updates the other so there is one consistent
verbosity, and it also changes what the app captures. Hiding the live-log panel
does not change the verbosity.

**Why this priority**: The two controls appeared to be the same setting but were
independent, so a change in one silently did not affect the other, which is
confusing and made the panel dropdown look broken.

**Independent Test**: Change the live-log dropdown and confirm the settings Log
level matches; change the settings Log level and confirm the live-log dropdown
matches; hide and show the panel and confirm the verbosity is unchanged.

**Acceptance Scenarios**:

1. **Given** a chosen log verbosity, **When** the user changes the live-log
   dropdown, **Then** the settings Log level shows the same level and the captured
   verbosity changes to match.
2. **Given** a chosen log verbosity, **When** the user changes the settings Log
   level, **Then** the live-log dropdown shows the same level.
3. **Given** any log verbosity, **When** the user hides or shows the live-log
   panel, **Then** the verbosity setting does not change.

---

### User Story 4 - A settings modal that fits the window (Priority: P2)

A user on a small window or a large or ultrawide display opens settings and the
modal is proportionate: it fills most of a small window and grows on a large one
without becoming absurdly large, and it looks the same whether opened before or
after resizing the window. Width and height follow the same sizing rule.

**Why this priority**: The operator reported the modal did not scale with the
window, its height did not follow the same rule as its width, and it looked wrong
on a large ultrawide display.

**Independent Test**: Resize the window small and large, open settings each time,
and confirm the modal is proportionate in both width and height and never exceeds
its sensible maximum.

**Acceptance Scenarios**:

1. **Given** a resized window, **When** the user opens settings, **Then** the modal
   is sized to the current window (not a previous or default size).
2. **Given** the window is enlarged, **When** the user opens settings, **Then** the
   modal grows in absolute size but occupies a progressively smaller fraction of
   the window, up to a maximum size.
3. **Given** the window, **When** the user views the modal, **Then** its height
   follows the same sizing rule as its width.

---

### User Story 5 - A noticeable save confirmation (Priority: P3)

A user changing a setting sees a clearly green Settings saved confirmation that is
easy to notice, in both light and dark themes.

**Why this priority**: The confirmation used flat neutral colors and was easy to
miss, so a user could be unsure whether a change took effect.

**Independent Test**: Change a setting and confirm the Settings saved toast appears
with a green success color, legible in both themes.

**Acceptance Scenarios**:

1. **Given** a setting change, **When** the confirmation appears, **Then** it uses
   a green success color and its text remains legible in both light and dark
   themes.

---

### Edge Cases

- The live-log dropdown and the settings Log level are open or changed nearly
  together: the last change wins and both reflect it; no inconsistent state
  persists.
- A very small window: the modal still fits within the window (never wider or
  taller than the window) rather than overflowing.
- A very large or ultrawide window: the modal stops growing at its maximum size
  rather than filling the whole screen.
- All settings-modal dropdowns keep a constant resting width regardless of the
  option selected, so selecting an option does not shift the layout.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The keybindings key list MUST include F2 so it is selectable and so a
  binding currently set to F2 displays F2 as its selected value.
- **FR-002**: Keybinding dropdowns MUST display friendly key names (Number 1
  through Number 5, E, R, X, Q, Space, F1, F2) for both the selected value and the
  option list, while the stored and parsed key values remain unchanged.
- **FR-003**: The live-log verbosity control and the settings Log level MUST stay
  in sync: changing either updates the other, and the change also updates what the
  app captures.
- **FR-004**: Hiding or showing the live-log panel MUST NOT change the logging
  verbosity setting.
- **FR-005**: The settings modal MUST size both its width and height to the current
  window each time it is shown and as the window resizes.
- **FR-006**: The modal size MUST follow one shared rule for width and height where
  the modal occupies a progressively smaller fraction of the window as the window
  grows, while its absolute size keeps increasing up to a maximum, and never
  exceeds the window.
- **FR-007**: The Settings saved confirmation MUST use a green success color and
  remain legible in both light and dark themes.
- **FR-008**: All settings-modal dropdowns (theme, beacon environment, log level,
  and keybindings) MUST keep a constant resting width regardless of the option
  selected.
- **FR-009**: All changes are presentation-layer only and MUST NOT alter the
  safety-critical input engine or the beacon managed-marker uninstall behavior.
- **FR-010**: The modal-sizing rule, the key display-name mapping, and the
  log-level linkage MUST be covered by unit tests.

### Key Entities *(include if data involved)*

- **Log verbosity**: a single level that governs both what the live-log panel
  offers and what the app captures; previously split into a display filter and a
  capture level that could diverge.
- **Key display name**: a friendly, human-readable label for each key,
  independent of the stored canonical key string.
- **Modal extent rule**: the shared function that maps a window dimension to a
  modal dimension (progressively smaller fraction, increasing pixels, bounded to a
  maximum and to the window).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: F2 is selectable in the keybindings in 100% of cases, and a binding
  set to F2 displays F2.
- **SC-002**: Every key in the keybindings reads as its friendly name; no raw
  canonical string (for example digit1) is shown to the user.
- **SC-003**: Changing either log control makes the other match in 100% of cases,
  and hiding the panel never changes the verbosity.
- **SC-004**: The modal is sized to the current window every time it opens; on an
  enlarged window it is larger in pixels yet a smaller fraction, and it never
  exceeds its maximum or the window bounds.
- **SC-005**: The Settings saved confirmation is visibly green and legible in both
  themes.

## Assumptions

- Linking the two log controls means the live-log dropdown now changes the captured
  log level and persists it, matching the settings Log level; the previous
  display-only filtering is subsumed because the display threshold equals the
  capture level.
- The green success color reuses the existing brand success (ok) color; legibility
  is maintained by pairing it with a contrasting text color and, if needed, a
  heavier weight.
- The maximum modal size and the growth rate are chosen during planning to look
  right from the minimum window up to a QHD ultrawide display.
- The shared fixed-width dropdown treatment introduced in the previous slice is
  reused for the settings dropdowns.
- Visual behavior is validated observationally against the running app; the pure
  helpers (modal sizing, key names) and the linkage are unit-tested.
