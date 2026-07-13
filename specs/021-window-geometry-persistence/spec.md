# Feature Specification: Window Geometry Persistence

**Feature Branch**: `021-window-geometry-persistence`

**Created**: 2026-07-13

**Status**: Draft

**Input**: User description: "Window geometry persistence (build plan 006, slice 021). Record and restore the application window across sessions so it reopens at the size, position, maximized state, and monitor it last occupied, including on multi-monitor systems."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Reopen where I left it (Priority: P1)

A user positions and sizes the ESO Weave window where they want it, uses the app,
closes it, and later reopens it. The window returns to the same size and position
on the same monitor, so they do not have to move and resize it every session.

**Why this priority**: This is the entire feature. Without it the window always
opens at a fixed default size and default position, and the user must rearrange
it on every launch. The value is delivered the moment size and position restore.

**Independent Test**: Move and resize the window, close the app, reopen it, and
confirm the window returns to the same size and position. Fully testable on its
own and delivers the core value.

**Acceptance Scenarios**:

1. **Given** the window has been moved and resized, **When** the user closes and
   reopens the app, **Then** the window reopens at the same size and position it
   last had.
2. **Given** a first-ever launch with no recorded geometry, **When** the app
   starts, **Then** the window opens at the existing default size and position
   with no error.
3. **Given** the window was resized and then the app was closed immediately,
   **When** the app reopens, **Then** the most recent size is restored (the last
   change is not lost to timing).

---

### User Story 2 - Maximized stays maximized (Priority: P2)

A user maximizes the window (including via the Windows snap or double-click title
bar gestures). On the next launch the window reopens maximized.

**Why this priority**: Maximized and snapped states are common on desktop and are
a distinct state from a specific size and position. It builds on P1 but is
independently valuable.

**Independent Test**: Maximize the window, close and reopen, and confirm it
reopens maximized. Testable independently of the exact-size restore.

**Acceptance Scenarios**:

1. **Given** the window is maximized, **When** the user closes and reopens the
   app, **Then** the window reopens maximized.
2. **Given** the window was maximized and is then restored to a normal size and
   moved, **When** the user closes and reopens, **Then** the window reopens at
   that normal size and position, not maximized.

---

### User Story 3 - Same monitor on a multi-monitor desk (Priority: P2)

A user with more than one monitor places the window on a specific monitor. On the
next launch the window reopens on that same monitor.

**Why this priority**: Explicitly called out by the operator, who runs a
multi-monitor setup. Restoring position by absolute desktop coordinates places
the window back on the correct monitor when the monitor layout is unchanged.

**Independent Test**: Place the window on a secondary monitor, close and reopen,
and confirm it reopens on that secondary monitor.

**Acceptance Scenarios**:

1. **Given** the window is on a secondary monitor, **When** the user closes and
   reopens the app with the same monitor layout, **Then** the window reopens on
   that secondary monitor.

---

### Edge Cases

- A recorded position now lies entirely off every connected monitor (a monitor
  was disconnected or the desktop layout changed): the window MUST open at a
  visible default rather than off-screen where the user cannot reach it.
- A recorded size is degenerate (zero, negative, not a finite number, or larger
  than any reasonable desktop): the window MUST fall back to the default size
  rather than opening unusably small or absurdly large.
- The recorded size is smaller than the window's minimum: it MUST be brought up
  to at least the minimum so the window remains usable.
- An older saved state file that predates this feature (no recorded geometry):
  the app MUST load it without error and open at the default geometry.
- The saved state file is missing or unreadable: the app MUST open at the default
  geometry with no crash, consistent with existing state-load resilience.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST record the window's outer position, inner size, and
  maximized state as they change while the app is running.
- **FR-002**: The system MUST restore the recorded position, size, and maximized
  state when the app launches, so the window reopens as it was last left.
- **FR-003**: The system MUST persist window geometry as automatically captured
  runtime state (session state), separate from user-editable settings, and MUST
  NOT crash or lose other persisted state when doing so.
- **FR-004**: The system MUST reopen the window on the same monitor it last
  occupied when the monitor layout is unchanged.
- **FR-005**: The system MUST detect a recorded position that is no longer visible
  on any connected monitor and fall back to a visible default position instead of
  opening off-screen.
- **FR-006**: The system MUST reject a degenerate or out-of-range recorded size
  and fall back to the default size, and MUST never restore a size below the
  window's minimum.
- **FR-007**: The system MUST load a saved state that predates this feature (with
  no recorded geometry) without error, opening at the default geometry.
- **FR-008**: The system MUST persist the most recent geometry even when the app
  is closed immediately after a change, so a resize just before quitting is not
  lost.
- **FR-009**: Recording and restoring geometry MUST NOT change the existing
  restore behavior of the suspend and fishing intents.
- **FR-010**: Persisted geometry MUST be written in the project's existing on-disk
  text format conventions (UTF-8 without a byte order mark, LF endings, trailing
  newline) and MUST advance the persisted state version with a forward migration
  that treats absent geometry as "no recorded geometry."

### Key Entities *(include if feature involves data)*

- **Window Geometry**: the automatically captured record of where and how large
  the window last was: outer position (horizontal and vertical desktop
  coordinates), inner size (width and height), and whether the window was
  maximized. Absent before the first recording.
- **Session State**: the existing persisted record of automatically captured
  runtime intents (suspended, fishing) and derived caches, to which window
  geometry is added as an optional section.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In 100% of relaunches with an unchanged monitor layout, the window
  reopens at the same size, position, and monitor it last occupied (within the
  precision the desktop reports).
- **SC-002**: A maximized window reopens maximized in 100% of relaunches.
- **SC-003**: When a recorded position is entirely off all connected monitors,
  the window opens fully visible on a connected monitor in 100% of cases (never
  off-screen and unreachable).
- **SC-004**: First-ever launch and launches from a pre-feature saved state open
  at the default geometry with zero errors or warnings surfaced to the user.
- **SC-005**: A resize performed immediately before closing the app is reflected
  on the next launch (no lost final change).

## Assumptions

- Window geometry is runtime state (where the window happened to be), not a
  user-authored preference, so it is stored with session state rather than in the
  user settings file. This matches the existing treatment of the suspend and
  fishing intents.
- Absolute desktop (virtual-screen) coordinates are sufficient to reselect the
  correct monitor when the layout is unchanged; a separate monitor identifier is
  not required for the common case.
- Off-screen recovery is validated against the desktop's virtual-screen bounds on
  the platform that reports them; where those bounds are not readily available,
  size sanity is still enforced and the platform window manager is relied upon to
  keep the window reachable.
- The existing coalesced auto-save mechanism (a settle-delayed write of session
  state) is reused for geometry, with an added forced write on window close to
  cover a change made in the final moments before exit.
- The default geometry and minimum window size already defined by the app remain
  the fallback for every rejected or absent recorded value.
