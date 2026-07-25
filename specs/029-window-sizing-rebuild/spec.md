# Feature Specification: Window Sizing Model Rebuild

**Feature Branch**: `029-window-sizing-rebuild`

**Created**: 2026-07-25

**Status**: Draft

**Input**: GitHub issue #8 (v0.7.0 window minimum-height clamp and live-log pane
sizing are broken; region clamping needs a ground-up review)

## Overview

The application window enforces a minimum size so its controls are never clipped,
and it can show a resizable live-log pane docked at the bottom. The v0.7.0 release
(the previous window-sizing work) fixed the original clipping but left the window
minimum height permanently inflated and the log pane effectively unusable when
open. This feature rebuilds the sizing model so the window minimum tracks the
actual content, the log pane is genuinely resizable, window growth is shared
sensibly between the main area and the log, and opening or closing the log leaves
the window the same overall height it started at.

The defect is user-visible and blocks a clean release: users see a dead band of
empty space they cannot remove, and a log pane they cannot enlarge. All behavior
is verifiable at the desk without the game.

## Clarifications

### Session 2026-07-25

- Q: What exactly counts as a "stable" content measurement (the gate that lets the
  measured extent supersede the boot floor)? -> A: Two consecutive frames whose
  measured content width and height are each equal within a small tolerance
  (about half a point). Until that holds, the boot floor applies; from then on the
  measured extent sets the minimum per dimension.
- Q: How much drag room does the enforced minimum open-window height reserve for
  the log, so the pane is resizable at that minimum without preventing the window
  from shrinking? -> A: The six-line log minimum plus one additional row height.
  At the enforced minimum the pane can range from six lines up to seven lines
  (maximum strictly greater than minimum), and the window can still shrink with the
  log compressing back toward six lines.
- Q: Against what value is the log pane's available maximum computed, now that the
  old inflated floor is removed? -> A: The current measured central-content height
  (the height the controls actually need this frame), not a running maximum, so no
  phantom reserved band remains.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - The window minimum hugs the actual content (Priority: P1)

As a user shrinking the window with the log closed, I can shrink it down until it
just fits the controls, with no leftover empty band below the last control.

**Why this priority**: This is the most visible defect (a permanent ~20 percent
dead band) and the root of the others. Fixing it makes the window feel correct at
its minimum and is the foundation for the log-pane behavior.

**Independent Test**: With the log closed, shrink the window to its minimum and
confirm the last control sits near the bottom edge with no large empty band, and
that the minimum tracks the real content (for example, it adjusts when a control
row is added or removed) rather than a fixed oversized floor.

**Acceptance Scenarios**:

1. **Given** the log is closed and the content has been laid out, **When** the
   user shrinks the window as far as allowed, **Then** the enforced minimum height
   equals the measured content height (no dead band beyond a hairline).
2. **Given** the very first frames before the content has been measured, **When**
   the window opens, **Then** a safe starting minimum (the boot floor) applies so
   nothing is clipped until the real content extent is known.
3. **Given** the content later becomes shorter (a row is removed), **When** it is
   re-measured, **Then** the enforced minimum can shrink to the new content height
   rather than staying latched at the earlier larger value.

### User Story 2 - The log pane is resizable and has no phantom band (Priority: P1)

As a user with the log open, I can drag the pane larger, and there is no invisible
reserved band of empty space between the controls and the log.

**Why this priority**: The log pane being frozen at its minimum makes the log
feature nearly useless; this is a core part of the reported regression.

**Independent Test**: Open the log, drag its top splitter upward, and confirm the
pane grows; confirm there is no empty gap reserved above the log that no control
occupies.

**Acceptance Scenarios**:

1. **Given** the log is open at a normal window size, **When** the user drags the
   pane's splitter, **Then** the pane resizes between a six-line minimum and the
   space available above it.
2. **Given** the log is open at the enforced minimum open-window height, **When**
   the user inspects the pane, **Then** the pane still has a small range to grow
   (its maximum is strictly greater than its minimum), so it is never fully frozen.
3. **Given** the log is open, **When** the user reads the area between the controls
   and the log, **Then** no reserved empty band is present (the space is either
   control content or the log).

### User Story 3 - Window growth is shared with the log (Priority: P2)

As a user enlarging the window with the log open, the extra height is shared
between the main area and the log in proportion to how much of the window each
already occupies, so the balance I set is preserved.

**Why this priority**: Without this, all extra height goes to the main area and the
log never benefits from a larger window, which is a primary complaint. It builds on
Stories 1 and 2.

**Independent Test**: Set the log to roughly a third of the window, enlarge the
window vertically, and confirm the log grows by roughly its share of the added
height (not zero), keeping the visual balance close to what was set.

**Acceptance Scenarios**:

1. **Given** the log occupies a fraction of the usable height, **When** the window
   grows taller, **Then** the log grows by approximately that same fraction of the
   added height and the main area takes the remainder.
2. **Given** the window shrinks while the log is open, **When** the height is
   removed, **Then** it is taken from both panes in the same proportion, and each
   pane is held to its own minimum (the log never below six lines, the main area
   never below its content).

### User Story 4 - Opening and closing the log is height-neutral (Priority: P2)

As a user toggling the log, opening then closing it returns the window to the same
overall height it had before, even if I resized the log while it was open.

**Why this priority**: A residual empty band after closing a resized log is a real
but lower-severity annoyance than the frozen pane.

**Independent Test**: Note the window height, open the log, enlarge the pane, close
the log, and confirm the window returns to its original height (no leftover empty
band).

**Acceptance Scenarios**:

1. **Given** the log is closed at some window height, **When** the user opens then
   closes it without resizing, **Then** the window returns to its original height.
2. **Given** the user enlarged the log while it was open, **When** the user closes
   it, **Then** the window shrinks by the pane's actual height, returning to its
   original height with no residual band.

### Edge Cases

- What happens on the very first frames before a stable content measurement? The
  boot floor applies until the content extent is measured and stable, then the
  measured extent takes over.
- What happens when the window is too short to hold both the content and a six-line
  log? The log collapses to its six-line minimum and the main area keeps its
  content; the log never covers the controls.
- What happens if a transient first-frame layout measures larger than the settled
  content? The stability gate prevents that transient value from latching the
  minimum permanently.
- What happens to a saved log height across a session? The persisted log height is
  the single source of truth for the pane and is restored consistently.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: With the log closed and content measured, the enforced window minimum
  height MUST equal the measured content height (no dead band beyond a hairline).
- **FR-002**: A safe boot floor MUST apply only until the content extent has been
  measured and is stable; after that, the measured extent MUST set the minimum per
  dimension and MUST NOT be permanently maxed against the boot floor.
- **FR-003**: A content measurement MUST be treated as stable only after it repeats
  across consecutive frames, so a transient first-frame layout does not latch the
  minimum.
- **FR-004**: Once measured and stable, the enforced minimum MUST be able to shrink
  when the content becomes smaller (it is not a permanent running maximum).
- **FR-005**: With the log open, the log pane MUST be resizable between a six-line
  minimum and the height available above it, with no reserved empty band between the
  controls and the log.
- **FR-006**: At the enforced minimum open-window height, the log pane's maximum
  MUST be strictly greater than its minimum, so the pane is never fully frozen,
  while the window MUST still be shrinkable with the log compressing toward its
  six-line minimum.
- **FR-007**: While the log is open, a change in window height MUST be distributed
  between the main area and the log pane in proportion to the fraction of the usable
  height each currently occupies, with each pane held to its own minimum and the
  main area taking any rounding remainder.
- **FR-008**: The proportional split fraction MUST be derived from the live pane
  heights at the time of each resize, not from a separately stored ratio.
- **FR-009**: Opening the log MUST grow the window by the log height actually shown,
  and closing it MUST shrink the window by the pane's actual current height, so an
  open-then-close cycle is height-neutral even when the pane was resized.
- **FR-010**: The persisted log-panel height MUST be the single source of truth for
  the pane height and MUST be restored consistently within and across sessions.
- **FR-011**: The window MUST never clip its controls at any allowed size, and the
  log pane MUST never cover the interactive controls.
- **FR-012**: The sizing computations MUST be implemented as pure, deterministic
  functions that can be unit-tested without a live window, and the new invariants
  MUST be covered by tests.

### Key Entities

- **Content extent**: the measured width and height of the laid-out main content,
  once stable; the basis for the enforced window minimum.
- **Boot floor**: the safe pre-measurement minimum size applied only until the
  content extent is measured and stable.
- **Log pane height**: the persisted, user-adjustable height of the live-log pane;
  the single source of truth for the pane.
- **Usable height**: the window's inner height available to the main area and the
  log pane together, split between them.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: With the log closed at the minimum window size, the empty space below
  the last control is at most a hairline (no perceptible dead band), verified at the
  desk and by the content-minimum unit tests.
- **SC-002**: With the log open, the pane can be dragged from its six-line minimum
  up to the available height, and at the enforced minimum open-window height its
  maximum strictly exceeds its minimum, verified by unit tests and the desk check.
- **SC-003**: Enlarging the window with the log open increases the log height by a
  non-zero amount approximately proportional to the log's current share, verified by
  the proportional-split unit tests and the desk check.
- **SC-004**: Opening then closing the log (including after resizing the pane)
  returns the window to its original height with no residual band, verified at the
  desk.
- **SC-005**: The enforced minimum tracks the content (it changes when a control row
  is added or removed) instead of staying fixed at an oversized floor, verified by
  the content-minimum unit tests.
- **SC-006**: The full merge gate (format check, lint with warnings as errors, and
  the test suite) passes.

## Assumptions

- Stability is defined as two consecutive content measurements that are equal within
  a small tolerance; this is sufficient to avoid latching a transient first frame.
- The enforced minimum open-window height reserves the six-line log minimum plus one
  extra line of drag room, which both satisfies "resizable at the minimum" and keeps
  the window shrinkable (the log compresses toward six lines as the window shrinks).
- The boot floor value and the six-line log minimum are unchanged from the current
  release; only how and when they are applied changes.
- The number and layout of controls is unchanged by this feature; only the sizing
  model changes.

## Out of Scope

- Any change to the pixel-bus, beacon, input engine, or fishing subsystems.
- The settings-modal sizing behavior.
- The other open feature requests (resource blocks, resolution detection) and the
  owed in-game validation from the prior slice.
