# Feature Specification: Responsive Live HUD Dashboard

**Feature Branch**: `codex/046-live-hud-dashboard`

**Created**: 2026-09-04

**Status**: Implemented

**Input**: GitHub issues #28 and #29, combined as S046

## User Scenarios & Testing

### User Story 1 - Understand live game state in one scan (Priority: P1)

As an operator, I want game observations grouped in a Live HUD so I can read
resources, context, combat, movement, weapon, and quickslot state without
sorting through setup controls.

**Why this priority**: The current unheaded grid gives every datum equal visual
weight and mixes live telemetry with application administration.

**Independent Test**: Render inactive, active, menu-gated, and signal-lost
models and verify the Live HUD contains every game observation with truthful
dormant or unavailable copy.

**Acceptance Scenarios**:

1. **Given** ESO is inactive, **When** the main view renders, **Then** every live
   datum uses a coherent dormant presentation and none says Gameplay or Ready.
2. **Given** ESO is active with a fresh signal, **When** the main view renders,
   **Then** context, character state, quickslot facts, and resources appear in
   the Live HUD without setup controls.
3. **Given** ESO is active but PixelBeacon is unavailable, **When** the main
   view renders, **Then** game activity remains visible while signal-dependent
   values say unavailable rather than zero or gameplay.

---

### User Story 2 - Understand readiness and blockers separately (Priority: P1)

As an operator, I want system and automation state grouped by responsibility so
I can tell whether ESO Weave can act and what needs attention.

**Why this priority**: Installation, live signal, requested automation, and
effective automation are independent facts that currently look interchangeable.

**Independent Test**: Render each installation and runtime condition plus
requested and blocked automation fixtures, then verify the operational section
names both the requested state and effective result.

**Acceptance Scenarios**:

1. **Given** the application is running normally, **When** the operational
   section renders, **Then** it says `ESO Weave: Active`, not an unowned
   `Status: Running` row.
2. **Given** PixelBeacon is installed but has no live signal, **When** the view
   renders, **Then** installation and signal appear as separate labeled facts.
3. **Given** fishing or auto-potion is requested but blocked, **When** the view
   renders, **Then** its switch remains requested and adjacent text names the
   effective blocker.
4. **Given** PixelBeacon is absent, outdated, or current, **When** its actions
   render, **Then** the appropriate install or update action alone is primary
   and uninstall remains secondary behind confirmation.

---

### User Story 3 - Read precise resource meters at any supported size (Priority: P1)

As an operator, I want Health, Stamina, and Magicka rendered as stable labeled
meters so magnitude is glanceable while the exact percentage remains visible.

**Why this priority**: Resource telemetry works, but three plain strings discard
the visual advantage of bounded values and make observed zero resemble absence.

**Independent Test**: Render 0, 1, 50, 99, and 100 percent plus low, dormant,
and unavailable states through the shared component and inspect its geometry,
visible copy, and accessibility metadata.

**Acceptance Scenarios**:

1. **Given** an observed percentage, **When** its meter renders, **Then** fill
   width represents the value and the exact integer percentage remains visible.
2. **Given** an enabled auto-potion watch whose threshold is crossed, **When**
   its meter renders, **Then** the visible state says Low and derives that state
   from the configured threshold.
3. **Given** observed zero, game inactivity, or signal unavailability, **When**
   each renders, **Then** the three presentations remain visibly and
   semantically distinct.
4. **Given** any resource theme, **When** assistive technology inspects the
   meter, **Then** it receives the resource name, current numeric value when
   observed, and the same state text shown visually.

---

### User Story 4 - Preserve the dashboard across window sizes (Priority: P2)

As an operator, I want the dashboard to use two columns when useful and stack
cleanly when narrow without clipping Skills or the live log.

**Why this priority**: A responsive hierarchy cannot be implemented by letting
window-sized containers contaminate intrinsic minimum-size measurement.

**Independent Test**: Drive the real frame through the headless egui harness at
narrow, wide, and high-DPI-equivalent point sizes and assert layout ordering,
stable meter dimensions within each mode, minimum sizing, and log separation.

**Acceptance Scenarios**:

1. **Given** width below the documented breakpoint, **When** a frame renders,
   **Then** Live HUD precedes System and automation in one vertical reading order.
2. **Given** width at or above the breakpoint, **When** a frame renders, **Then**
   the two sections occupy separate side-by-side columns.
3. **Given** rapidly changing state text or resource values, **When** successive
   frames render in one layout mode, **Then** row heights and unrelated controls
   do not move.
4. **Given** either layout with the log open, **When** the window or splitter is
   resized, **Then** the log never covers dashboard or Skills controls.

### Edge Cases

- A resource is exactly 0, 1, its configured low threshold, 99, or 100.
- A resource watch is disabled while its observed value is low.
- The game is active while installation evidence is unknown.
- PixelBeacon is current on disk while its live signal is never observed or lost.
- A long effective blocker is shown at the narrow layout.
- The width moves back and forth across the breakpoint during one resize gesture.
- The destructive addon confirmation row appears in either responsive mode.
- The live log is open at the smallest supported content extent.

## Requirements

### Functional Requirements

- **FR-001**: The pre-Skills region MUST have visible `Live HUD` and
  `System and automation` headings.
- **FR-002**: Live HUD MUST contain game context, resources, combat, movement,
  weapon-bar facts, and selected quickslot facts.
- **FR-003**: System and automation MUST contain ESO installation, ESO runtime,
  ESO Weave active/suspended state, PixelBeacon installation, PixelBeacon live
  signal, fishing, auto-potion, and addon lifecycle actions.
- **FR-004**: PixelBeacon installation and signal MUST be separate view-model
  facts and separately labeled UI rows.
- **FR-005**: Fishing and auto-potion MUST show requested state through their
  controls and effective state or blocker through adjacent text.
- **FR-006**: The application status row MUST identify its subject as ESO Weave
  and use Active or Suspended copy.
- **FR-007**: ESO inactivity MUST replace every live game observation with a
  dormant state; it MUST NOT leave remembered Gameplay, Ready, or numeric data.
- **FR-008**: Active ESO with unavailable telemetry MUST remain distinct from
  inactive ESO and from observed zero.
- **FR-009**: The addon action row MUST make Install primary when absent, Update
  primary when outdated, and neither primary when current; uninstall MUST retain
  its existing managed-marker guard and confirmation.
- **FR-010**: Health, Stamina, and Magicka MUST use one reusable meter component
  with semantic resource themes.
- **FR-011**: Each observed meter MUST expose the exact integer percentage as
  visible text and progress accessibility metadata.
- **FR-012**: Meter fill MUST be deterministic, unanimated, and proportional to
  the validated 0 through 100 value.
- **FR-013**: Observed empty, observed low, dormant, and unavailable meter states
  MUST have distinct visible text or geometry in addition to color.
- **FR-014**: A Low presentation MUST only appear when that resource has an
  enabled auto-potion watch and its observed value is at or below the configured
  threshold.
- **FR-015**: Text, meter track, meter fill, and state boundaries MUST meet the
  documented WCAG 2.2 AA contrast targets in dark and light themes.
- **FR-016**: The dashboard MUST stack below the responsive breakpoint and use
  two columns at or above it, with Live HUD first in both reading orders.
- **FR-017**: Responsive containers MUST NOT make the enforced minimum width
  track the current window width or reintroduce resize ratcheting.
- **FR-018**: Dynamic text and percentages MUST NOT change the reserved size of
  their rows within a layout mode.
- **FR-019**: The existing Skills grid MUST retain its rows, columns, controls,
  tooltips, cooldown values, and intent wiring without functional redesign.
- **FR-020**: The existing live-log non-overlap, settings-modal, persistence, and
  minimum-window contracts MUST continue to pass in both responsive modes.
- **FR-021**: Rendered-frame tests MUST cover narrow and wide layout geometry,
  state text presence, resource boundaries, long blockers, and log containment.
- **FR-022**: Master specification, README, changelog, and build-plan records
  MUST describe the new hierarchy and truthful state semantics.

### Key Entities

- **DashboardLayout**: Narrow stacked or wide two-column presentation selected
  from available point width at one documented breakpoint.
- **ResourcePresentation**: Observed, Low, Dormant, or Unavailable state carrying
  validated value and visible semantics.
- **ResourceMeter**: Shared rendered component for resource name, fill, exact
  value or state, semantic theme, and accessibility metadata.
- **BeaconSignalLine**: View-model projection of runtime and beacon freshness,
  independent from on-disk addon installation.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Every legacy pre-Skills datum is mapped exactly once to Live HUD
  or System and automation, except redundant internal aliases removed by design.
- **SC-002**: Rendered narrow and wide fixtures show both headings in the correct
  order with no clipping or overlap.
- **SC-003**: Boundary resource values produce fill fractions of 0.00, 0.01,
  0.50, 0.99, and 1.00 and preserve exact visible text.
- **SC-004**: Dormant, unavailable, and observed zero produce three distinct
  state values and three distinct accessible descriptions.
- **SC-005**: Changing any percentage from 0 through 100 changes no meter height
  or section width within one layout mode.
- **SC-006**: The complete local merge gate and all pre-existing safety suites
  pass without modifying Skills behavior or input-safety logic.

## Assumptions

- Point width, not physical-pixel width, is the correct responsive input because
  egui already applies platform scale factors before layout.
- The existing typed game runtime and context projections remain authoritative.
- `egui::WidgetInfo` on a progress indicator is the supported accessibility seam.
- Final appearance remains subject to maintainer review on the pull request;
  automated geometry and contrast checks close the implementation evidence.

## Explicit Design Deviation

Earlier rendered-frame tests asserted that the full intrinsic height was
independent of window width. That assumption is incompatible with #28's required
stacked and two-column modes. S046 preserves a window-independent minimum width
and anti-ratchet behavior while allowing the measured content height to change
at one explicit breakpoint. The replacement contract tests both modes and the
transition directly instead of treating responsiveness as a regression.
