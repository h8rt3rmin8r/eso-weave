# Feature Specification: Application Interface Sizing Correctness

**Feature Branch**: `030-ui-sizing-correctness`

**Created**: 2026-07-25

**Status**: Draft

**Input**: GitHub issues #12 (window shrink is ratcheted), #13 (live log pane can
be dragged over the Skills controls), and #14 (settings modal does not grow with
the window), all filed against v0.8.0. Build plan `docs/plans/plan-009.md`,
slice 030. Master specification section 11.

## Overview

The application window enforces a minimum size so its controls are never
clipped, it can show a resizable live-log pane docked at the bottom, and it
opens a settings modal sized from the current window. In v0.8.0 all three of
those behaviors are wrong at once: the window can only be shrunk a sliver per
drag gesture, the log pane can be dragged over the interactive controls, and the
settings modal is stuck at roughly half the height it should have.

This is the fourth consecutive release carrying a window-sizing defect, and the
first report is filed as a zero-tolerance regression rather than a request for
another point patch. The feature therefore has two obligations. The first is to
correct the three behaviors. The second, and the reason this is a slice rather
than a patch, is to make the sizing behavior verifiable automatically: every
sizing defect so far has shipped with a fully green test suite, because the
tested part (the sizing arithmetic) has always been correct and the broken part
(how that arithmetic is connected to the rendered window) has never been tested
at all.

All behavior here is verifiable at the desk. The game is not required.

## Clarifications

### Session 2026-07-25

Answered under the build-phase autopilot decision policy from the constitution,
the master specification, build plan 009, the three issue reports, and the
v0.8.0 code. None were escalated.

- Q: What exactly is the boundary the log pane may not cross: the bottom of the
  last interactive control, or the bottom of the whole central content? -> A: The
  bottom of the whole central content, including the trailing padding of the last
  row. Issue #13 states the central panel must retain "at least its full
  intrinsic content height", and the wider boundary is both the conservative
  choice and the one that can be asserted without enumerating widgets.
- Q: How does full-width chrome (a separator that spans the available width)
  contribute to the intrinsic extent? -> A: Height only, never width. Any element
  that expands to fill the available width contributes nothing to the intrinsic
  width, which is what makes the extent independent of the window. (Corrected
  during implementation: this answer originally assumed the menu bar was
  content-sized and would contribute its buttons' width. Measurement showed the
  menu bar also spans the available width, so it is full-width chrome too. Its
  buttons need far less width than the grids below it, so excluding it clips
  nothing.)
- Q: What happens when the intrinsic minimum is larger than the display work area
  (a small display at a high scale factor)? -> A: The enforced minimum is capped
  at the work area, so the window always stays positionable and resizable. In that
  case content may be clipped; making the central content scrollable is
  deliberately out of scope for this feature, because a filling scroll area would
  reintroduce the window-tracking measurement this feature exists to remove. The
  limitation is recorded as an edge case.
- Q: What is the denominator for "at least half of the settings body visible"?
  -> A: The total laid-out height of the settings content at the modal's inner
  width, measured on the same frame. Half of that height must be visible without
  scrolling.
- Q: When the intrinsic content changes size, does the window follow? -> A: It
  grows but never shrinks. If the content grows beyond the current window the
  window grows to fit, so nothing is clipped; if the content shrinks the window
  keeps the size the user chose and only the enforced minimum drops.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - The window shrinks freely in one drag (Priority: P1)

As a user whose last session left the window very large, I can grab an edge or
corner once and drag the window all the way down to the size the controls
actually need, without letting go and grabbing again a dozen times.

**Why this priority**: This is the reported blocker and the root of the family.
The enforced minimum is currently derived from the window's own current size, so
it re-pins itself every gesture; nothing else in the sizing model can be trusted
while that is true.

**Independent Test**: Persist an oversized window on both axes, restart, and
shrink each axis in a single continuous drag. The window reaches the content
minimum without the drag locking part way.

**Acceptance Scenarios**:

1. **Given** a window restored much larger than its content on both axes,
   **When** the user drags one edge inward without releasing, **Then** the window
   shrinks continuously until the controls exactly fit, with no intermediate lock.
2. **Given** the same window, **When** the user drags a corner inward without
   releasing, **Then** both width and height reach their content minimum in that
   one gesture.
3. **Given** the window is at its content minimum, **When** the user drags
   further inward, **Then** the window stops and no control is clipped.
4. **Given** the content becomes shorter or narrower (a transient control row
   disappears), **When** the layout settles, **Then** the enforced minimum
   follows the content down rather than staying at the earlier larger value.
5. **Given** the very first frames after launch, **When** the content has not yet
   been measured, **Then** a safe starting minimum applies and nothing is clipped.

---

### User Story 2 - The log pane never covers a control (Priority: P1)

As a user with the live log open, I can drag the log's top edge upward as hard
and as fast as I like and it always stops before the Skills controls; nothing
interactive is ever hidden underneath it.

**Why this priority**: A pane covering interactive controls makes them
unreachable, which is a functional failure rather than a cosmetic one. It was
reported once before (issue #5), declared fixed twice, and has returned; it must
now hold under every combination of gestures.

**Independent Test**: Open the live log and drag its splitter upward past the
boundary at several window sizes, including immediately after resizing the
window. The pane stops at the boundary every time and stays there.

**Acceptance Scenarios**:

1. **Given** the log is open, **When** the user drags the splitter upward past
   the controls, **Then** the pane stops at the bottom of the controls and no
   control is covered at any point during the gesture.
2. **Given** the log is open at the smallest allowed window size, **When** the
   user drags the splitter upward, **Then** the pane still stops before the
   controls and retains a small range of movement.
3. **Given** the user has just enlarged or shrunk the window, **When** the
   splitter is dragged immediately afterward, **Then** the boundary is already
   correct for the new window size.
4. **Given** the user dragged the splitter past the boundary, **When** the
   session ends and the app is restarted, **Then** the restored pane height is
   within the boundary and no control is covered on the first frame.
5. **Given** the log is open, **When** the window is shrunk toward its minimum,
   **Then** the pane compresses toward its readable six-line floor and the
   controls stay fully visible.

---

### User Story 3 - The settings modal grows with the window (Priority: P2)

As a user on a large display, opening Settings gives me a large panel that shows
most of the settings at once, instead of a small box that scrolls after two
options while empty space surrounds it.

**Why this priority**: The modal is usable today, just needlessly cramped, so it
ranks below the two defects that make controls unreachable. It shares the same
sizing surface and is corrected in the same pass.

**Independent Test**: Open Settings at a small, a medium, and a very large
window and confirm the panel grows on both axes each time, with progressively
smaller gains, up to its maximum, and that the body shows far more content at
the large window than at the small one.

**Acceptance Scenarios**:

1. **Given** a large window, **When** the user opens Settings, **Then** the modal
   is at or near its maximum size and the body shows a large portion of the
   settings without scrolling.
2. **Given** the modal is open, **When** the user enlarges the window, **Then**
   the modal grows on both axes, by a progressively smaller share of each
   addition, and stops growing at its maximum.
3. **Given** the smallest allowed window, **When** the user opens Settings,
   **Then** the modal fits inside the window with a visible margin on every edge
   and remains fully usable.
4. **Given** any window size, **When** the modal is displayed, **Then** its
   rendered size matches the size the growth rule calls for at that window.

---

### User Story 4 - Sizing regressions cannot ship green (Priority: P1)

As the operator accepting a release, I can trust that a passing test run actually
means the window sizing works, because the checks exercise the rendered window
rather than only the arithmetic behind it.

**Why this priority**: Four releases have now shipped a sizing defect with a
fully green suite. Without this story the other three are one refactor away from
regressing again, exactly as issue #13 did after two separate fixes.

**Independent Test**: Reintroduce any one of the three defects deliberately and
confirm the automated checks turn red without any manual step.

**Acceptance Scenarios**:

1. **Given** the automated checks, **When** the window minimum is made to depend
   on the current window size again, **Then** the checks fail.
2. **Given** the automated checks, **When** the log pane boundary is removed or
   loosened, **Then** the checks fail.
3. **Given** the automated checks, **When** the modal stops being given its
   computed size, **Then** the checks fail.
4. **Given** a fresh checkout, **When** the standard verification commands are
   run, **Then** these checks run automatically with no game, no display, and no
   manual interaction.

---

### Edge Cases

- A session persisted a window larger than the display now in use: the restored
  window is brought within the display and still shrinks freely.
- The display scale or the theme changes while the app is running: the enforced
  minimum is recomputed from the new layout instead of staying at the old value.
- A transient control row appears or disappears (the uninstall confirmation row):
  the minimum grows for it and shrinks again afterward.
- The log is toggled on at the minimum window height: the window grows by exactly
  what the pane needs, and toggling it off returns the window to its former
  height with no residual band.
- A persisted log height from an older version exceeds the boundary for the
  current window: it is brought inside the boundary before the first frame is
  shown.
- The window is shorter than the modal's minimum height: the modal still fits
  inside the window rather than overflowing it.
- The user drags a window edge faster than frames are produced: no gesture is
  clamped by a minimum computed for a stale layout.
- The intrinsic content is taller or wider than the display work area (a small
  display at a high scale factor): the enforced minimum is capped at the work
  area so the window stays positionable and resizable. Content may be clipped in
  that case; a scrollable central area is out of scope for this feature and is
  the follow-up if the case is reached in practice.
- The intrinsic content grows while the window is at the user's chosen size: the
  window grows to fit rather than clipping, and it does not shrink back when the
  content shrinks again.

## Requirements *(mandatory)*

### Functional Requirements

**Window minimum (issue #12)**

- **FR-001**: The enforced minimum window size MUST be intrinsic, meaning it is
  determined only by the laid-out controls and the active theme. It MUST NOT be a
  function of the current window size on either axis. (Corrected during
  implementation: this originally listed the display scale as an input. The layout
  is expressed in points and the scale is applied at the platform boundary, so the
  extent is scale-invariant by construction; a minimum that moved with the scale
  would mean the measurement had leaked into pixel space.)
- **FR-002**: Until the content has been measured, a fixed boot minimum MUST
  apply so no control is clipped during the first frames.
- **FR-003**: A single continuous resize gesture MUST be able to reduce the window
  to the intrinsic minimum on each axis. No gesture may be stopped by a minimum
  that a later gesture would relax.
- **FR-004**: The enforced minimum MUST be able to decrease when the content
  becomes smaller; it MUST NOT latch at the largest value seen.
- **FR-005**: The enforced minimum MUST be recomputed when, and only when, the
  intrinsic content changes (a control row appears or disappears, or the theme
  changes). A display scale change MUST leave it unchanged, per FR-001.
- **FR-006**: At the enforced minimum, no control may be clipped, on either axis,
  with the log closed or open.
- **FR-007**: Elements that expand to fill the available width MUST contribute
  their height to the intrinsic extent and nothing to its width.
- **FR-008**: The enforced minimum MUST be capped at the display work area, so the
  window always remains positionable and resizable on small displays.
- **FR-009**: When the intrinsic content grows beyond the current window, the
  window MUST grow to fit it. When the content shrinks, the window MUST keep its
  current size and only the enforced minimum drops.

**Live log pane (issue #13)**

- **FR-010**: On every rendered frame, the top edge of the live log pane MUST be
  at or below the bottom edge of the whole central content, including the trailing
  padding of its last row. This MUST hold during a splitter drag, during a window
  resize, and during both at once.
- **FR-011**: A pane height produced by a drag MUST be brought within the
  boundary before it is displayed on the next frame and before it is persisted.
- **FR-012**: A persisted pane height that is outside the boundary for the
  current window MUST be brought inside it before the first frame is shown.
- **FR-013**: The pane MUST retain a readable minimum of six lines of log text,
  and MUST retain a usable range of movement even at the enforced minimum open
  window height.

**Settings modal (issue #14)**

- **FR-014**: The modal's rendered size MUST equal the size its growth rule calls
  for at the current window size, on both axes, within a hairline tolerance.
- **FR-015**: The modal MUST grow on both axes as the window grows, by a
  progressively smaller share of each addition, and MUST stop at its configured
  maximum.
- **FR-016**: The modal MUST always fit inside the window, at every window size
  down to the enforced minimum.
- **FR-017**: At the modal's maximum size, at least half of the settings body MUST
  be visible without scrolling, where the settings body is its total laid-out
  height at the modal's inner width on the same frame. If enforcing FR-014 alone
  does not achieve this, the configured maximum MUST be raised until it does, and
  the new value recorded as a decision.

**Verification (the anti-regression obligation)**

- **FR-018**: The sizing behavior MUST be covered by automated checks that
  exercise the rendered window and assert rendered positions and sizes, not only
  the arithmetic that feeds them.
- **FR-019**: Those checks MUST include a simulated multi-step resize gesture,
  rendered as consecutive frames at shrinking window sizes, that fails if the
  enforced minimum rises at any step or differs between steps while the intrinsic
  content is unchanged; and a simulated splitter drag past the boundary that fails
  if any frame violates FR-010.
- **FR-020**: The checks MUST run without a display, without the game, and
  without manual interaction, as part of the project's standard verification
  commands.
- **FR-021**: Reintroducing any one of the three defects MUST turn the automated
  checks red.

**Documentation**

- **FR-022**: The master specification's user interface sections MUST be
  corrected to describe the sizing model actually implemented: the boot minimum
  and the intrinsic content minimum that supersedes it, the log pane's
  never-overlap boundary, and the modal's growth behavior.

### Key Entities

- **Intrinsic content extent**: the width and height the laid-out controls
  require, independent of the window they are shown in. Source of the enforced
  minimum.
- **Boot minimum**: the fixed fallback extent that applies before the content has
  been measured.
- **Log pane boundary**: the lowest position the log pane's top edge may take for
  a given window, derived from the intrinsic content extent.
- **Modal target extent**: the width and height the growth rule calls for at the
  current window size, bounded by a configured minimum, maximum, and a maximum
  fraction of the window.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: From a window persisted at 1600 by 1200 points, a user reaches the
  content minimum on each axis in exactly one continuous drag gesture per axis
  (currently many gestures per axis).
- **SC-002**: Across a matrix of splitter drags past the boundary, run at the
  minimum window size, at a mid size, and immediately after both enlarging and
  shrinking the window, the number of frames in which the log pane covers any
  interactive control is zero.
- **SC-003**: At a window 1600 points tall, the settings modal renders at its
  configured maximum height within one point, and at every window size from the
  enforced minimum up to 2160 points tall the rendered modal height and width
  each match the growth rule within one point.
- **SC-004**: At the modal's maximum size, the visible portion of the settings
  body is at least half of that body's total laid-out height at the same width.
- **SC-005**: Deliberately reintroducing any one of the three defects turns the
  project's standard verification commands red, with no manual step required.
- **SC-006**: The full desk validation, including the reproduction steps from all
  three issues, is completed without launching the game.

## Assumptions

- The boot minimum stays at its current 480 by 420 points; it is a startup
  fallback, not the enforced minimum, and no report asks for it to change.
- The log pane keeps its six-line readable minimum and the extra drag room
  reserved at the enforced minimum open window height, both established by the
  previous slice and not reported as defective.
- The wider minimum window width applied while the log is open is retained; it
  was an explicit request in the original log pane report.
- The modal's growth curve shape (sub-linear growth with a floor, a ceiling, and
  a maximum fraction of the window) is correct as specified and is retained; only
  its enforcement is at fault, with the single exception allowed by FR-017.
- Window geometry persistence, the height-neutral log toggle, and the
  proportional sharing of added height between the panes are behaviors from the
  previous slice that are not reported as defective and are preserved.
- Verification runs on the developer desk and in the project's automated
  pipeline, on both supported platforms, without a game client.
- The PixelBeacon addon, the pixel bus, and the input, weave, and fishing engines
  are untouched by this feature.
