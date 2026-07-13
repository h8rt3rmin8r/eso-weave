# Feature Specification: GUI Hover Reflow and Settings Modal Fixes

**Feature Branch**: `019-ui-hover-and-modal-fixes`

**Created**: 2026-07-12

**Status**: Draft

**Input**: User description: "Fix two GUI defects: buttons grow on hover and shift the whole window up and down; and the settings modal fills only about half its width with the scrollbar floating in the middle instead of at the far right edge."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Buttons do not shift the window on hover (Priority: P1)

A user moves the pointer over a button in the main window. The button changes its
border color to show it is hovered, but it does not change size, and nothing on
the window moves. Moving the pointer away leaves the layout exactly where it was.

**Why this priority**: A window that jumps up and down as the pointer sweeps over
controls looks broken and makes controls harder to click. It is the more visible
of the two defects.

**Independent Test**: Run the app, sweep the pointer across the Install, Uninstall,
and menu buttons, and confirm no vertical movement of any element.

**Acceptance Scenarios**:

1. **Given** the main window is shown, **When** the pointer hovers any button,
   **Then** the button's rendered size is unchanged and no other element moves.
2. **Given** a hovered button, **When** the pointer leaves it, **Then** the layout
   is identical to before the hover.

### User Story 2 - Settings modal fills its width with an edge scrollbar (Priority: P2)

A user opens Settings. The settings content fills the width of the modal, and the
vertical scrollbar sits at the far right edge of the modal, as expected.

**Why this priority**: The half-width content and mid-floating scrollbar look
unfinished. It is less disruptive than the hover reflow, so it is second.

**Independent Test**: Open Settings and confirm the body spans the modal width and
the scrollbar is flush with the right edge.

**Acceptance Scenarios**:

1. **Given** Settings is open, **When** it renders, **Then** the settings body
   fills the modal's inner width.
2. **Given** Settings has more content than fits, **When** the scrollbar appears,
   **Then** it is at the far right edge of the modal, not floating within it.

### Edge Cases

- A very small window: the modal still fills its (clamped) width and the scrollbar
  stays at the right edge.
- Light and dark themes: the hover fix holds in both; only the border color, never
  the size, changes on hover.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Hovering any button MUST NOT change its rendered size, so no layout
  reflow occurs on hover in either theme.
- **FR-002**: Button hover MAY change color or border appearance, but MUST keep the
  size-affecting inputs identical across the resting and hovered states.
- **FR-003**: The settings modal body MUST fill the modal's inner width.
- **FR-004**: The settings modal vertical scrollbar MUST render at the far right
  edge of the modal.
- **FR-005**: These changes MUST NOT alter any behavior, intent, or persisted
  state; they are presentation only.

### Key Entities

Not applicable; presentation-only changes.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Sweeping the pointer across every main-window button produces zero
  vertical layout movement.
- **SC-002**: The settings body fills the modal width and the scrollbar sits at the
  right edge, in both themes.
- **SC-003**: No test regressions; the full suite stays green.

## Assumptions

- The hover reflow is caused by a per-state difference in the size-affecting theme
  inputs (the widget interaction expansion and the hovered border stroke width),
  not by any deliberate grow-on-hover behavior.
- The settings scroll area inherits horizontal auto-shrink, which narrows it to its
  content; disabling auto-shrink makes it fill the modal width, matching the log
  panel scroll area that already renders correctly.
- The GUI layer is validated observationally (it carries no unit-tested logic), per
  the existing convention for `src/app/ui.rs`.
