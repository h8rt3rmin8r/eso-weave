# Feature Specification: Primary and Skills Panel Controls

**Feature Branch**: `022-primary-skills-controls`

**Created**: 2026-07-13

**Status**: Draft

**Input**: User description: "Primary and skills panel controls (build plan 006, slice 022): add an addon Update button, align the Weapon Bar line, give the Weave dropdown a fixed width, stop dropdowns from shifting the rows below, and overhaul the Skills Delay column (rename to Delay (ms), matching greyed read-only field when not overriding, four-digit width, right-aligned)."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Update the addon in one click (Priority: P1)

A user who already has the PixelBeacon addon installed wants to refresh it to the
version bundled with the app without manually removing and reinstalling. They
press an Update control and the addon is reinstalled cleanly.

**Why this priority**: The addon can drift out of date; today the only paths are
Install (which the user may not realize also updates) and a manual
Uninstall-then-Install. A dedicated Update control makes the common maintenance
action obvious and safe.

**Independent Test**: With the addon installed, press Update and confirm the
addon ends up installed and current. With the addon not installed, confirm the
Update control is disabled.

**Acceptance Scenarios**:

1. **Given** the addon is installed (current or outdated), **When** the user
   presses Update, **Then** the addon is removed and reinstalled and ends up
   installed and current.
2. **Given** the addon is not installed, **When** the user views the controls,
   **Then** the Update control is disabled (greyed) and cannot be pressed.
3. **Given** the addon folder is present but not managed by ESO Weave, **When**
   the user presses Update, **Then** the unmanaged folder is not deleted (the
   managed-marker safety rule holds) and the reinstall still writes the managed
   addon.

---

### User Story 2 - A tidy, aligned status section (Priority: P2)

A user reading the top status section sees the Status, Fishing, Pixel Beacon, and
Weapon Bar rows with their titles and states lined up in the same columns, rather
than the Weapon Bar line hanging out of alignment.

**Why this priority**: Visual alignment is a small but real polish defect the
operator called out; a misaligned row reads as unfinished.

**Independent Test**: Open the main window and confirm the Weapon Bar title and
state align with the three rows above it.

**Acceptance Scenarios**:

1. **Given** the main window is open, **When** the user views the status section,
   **Then** the Weapon Bar title aligns in the same column as the Status,
   Fishing, and Pixel Beacon titles, and its state aligns with their states.

---

### User Story 3 - Dropdowns and delay fields that do not jump (Priority: P2)

A user changing a skill's weave type or the live-log level, or toggling a delay
override, sees the control stay put: the dropdown keeps a constant width
regardless of the option selected, and the delay field keeps a constant width and
appearance whether or not the override is on, so the rows below never shift.

**Why this priority**: The operator reported that selecting different dropdown
options and toggling overrides shifts the surrounding layout by a few pixels,
which is distracting and reads as a bug.

**Independent Test**: Select each weave option in turn and confirm the field width
does not change; toggle a delay override on and off and confirm the delay cell
keeps the same width and box appearance.

**Acceptance Scenarios**:

1. **Given** a skill's Weave dropdown, **When** the user selects a longer or
   shorter option, **Then** the resting field width does not change and the rows
   below do not shift.
2. **Given** the live-log level dropdown, **When** the user selects a different
   level, **Then** the field width does not change.
3. **Given** a skill row, **When** the user toggles the delay Override on or off,
   **Then** the delay field keeps the same width and box appearance and the row
   does not shift.

---

### User Story 4 - A readable, consistent Delay column (Priority: P2)

A user reviewing skill delays sees a column headed Delay (ms), with each row's
actual current delay shown in a right-aligned field wide enough for four digits.
When a row is not overriding, the field is visibly greyed and read-only but still
shows the real inherited value so the user can decide whether to override it; when
overriding, the same-looking field is editable.

**Why this priority**: The column header did not state its unit, the dormant
values were plain muted text that looked different from the editable field, and
the field was too narrow, so the column read inconsistently.

**Independent Test**: Confirm the header reads Delay (ms); confirm a non-override
row shows its inherited delay in a greyed, right-aligned four-digit field; confirm
an override row shows an editable, right-aligned four-digit field with the same
width.

**Acceptance Scenarios**:

1. **Given** the Skills grid, **When** the user reads the delay column header,
   **Then** it reads Delay (ms).
2. **Given** a row with Override off, **When** the user views the delay cell,
   **Then** the actual inherited delay is shown in a greyed, read-only,
   right-aligned field sized for at least four digits.
3. **Given** a row with Override on, **When** the user edits the delay,
   **Then** the value is editable in a same-width, right-aligned field and the
   edit is applied.

---

### Edge Cases

- Pressing Update when the addon folder is unmanaged must not delete it (the
  managed-marker uninstall gate is never weakened); the reinstall still proceeds.
- Pressing Update when the AddOns directory cannot be resolved surfaces the same
  outcome as Install or Uninstall in that state (a logged notice, no crash) and
  the Update control is disabled anyway.
- Editing a delay: while the user is typing, the value being typed must not be
  overwritten by the model's stored value on the next frame; non-numeric input is
  rejected and the value is bounded to four digits.
- A delay of zero is a valid override value and is preserved.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST provide an Update control for the addon, placed with
  the Install and Uninstall controls.
- **FR-002**: The Update control MUST be disabled when the addon is not installed
  locally and enabled whenever the addon is installed, including when it is
  current.
- **FR-003**: Activating Update MUST remove the installed addon and then install
  it again, leaving it installed and current, and MUST NOT weaken the
  managed-marker rule that prevents deleting an unmanaged folder.
- **FR-004**: The Weapon Bar row MUST align its title and state in the same
  columns as the Status, Fishing, and Pixel Beacon rows.
- **FR-005**: The Weave dropdown MUST keep a constant resting width regardless of
  which option is selected.
- **FR-006**: Dropdowns on the main window (the weave selector and the live-log
  level filter) MUST NOT shift the rows below them when their selection changes;
  a shared fixed-width treatment MUST be used so the same fix can extend to the
  settings dropdowns in a later slice.
- **FR-007**: The Skills delay column header MUST read Delay (ms).
- **FR-008**: Each delay cell MUST display the actual current effective delay (the
  per-slot override when set, otherwise the global default for the row's weave
  type), in both the override-on and override-off states.
- **FR-009**: The delay value MUST render in a text field box in both states: an
  editable field when the override is on, and a visibly greyed, read-only field
  showing the inherited value when the override is off, with the same width and
  appearance so toggling the override does not shift the row.
- **FR-010**: The delay field MUST be wide enough to comfortably show at least
  four digits, and the numeric value MUST be right-aligned in both states.
- **FR-011**: While a delay is being edited, the in-progress value MUST NOT be
  overwritten by the stored value, non-numeric characters MUST be rejected, and
  the value MUST be bounded to four digits; a delay of zero MUST be accepted.
- **FR-012**: All changes are presentation-layer only and MUST NOT alter the
  safety-critical input engine or the beacon managed-marker uninstall behavior.

### Key Entities *(include if data involved)*

- **Update action**: a new user intent that performs an addon uninstall followed
  by an install, reusing the existing beacon paths.
- **Delay edit buffer**: transient GUI state holding the digits a user is typing
  into a delay field for one row, so the model value does not clobber in-progress
  input.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: With the addon installed, one Update press returns it to installed
  and current in 100% of attempts; with the addon not installed, the Update
  control is never pressable.
- **SC-002**: An unmanaged addon folder is never deleted by Update.
- **SC-003**: Selecting any weave option leaves the field width unchanged (0 px of
  row shift), and selecting any log level leaves that field width unchanged.
- **SC-004**: Toggling a delay override on or off leaves the delay cell width and
  box appearance unchanged (0 px of row shift).
- **SC-005**: The delay header reads exactly Delay (ms); every delay field shows
  the real current value, right-aligned, and fits a four-digit value without
  clipping.

## Assumptions

- Update is a convenience over the existing Install-and-Uninstall behavior; Install
  already updates in place, so Update is defined as an explicit uninstall then
  install for a clean reinstall, and is enabled on the same condition as Uninstall
  (a folder is present to remove).
- The delay field uses a text field styled consistently for both states; the
  editable state remains the source of truth while focused via a small per-row
  edit buffer held in the GUI, cleared when focus leaves.
- Fixed dropdown width is a single shared value comfortably fitting the longest
  option in use; exact pixel values are an implementation detail chosen during
  planning.
- Verification of the visual behavior is observational against the running app,
  consistent with prior GUI slices; the model-level Update intent is unit-tested.
