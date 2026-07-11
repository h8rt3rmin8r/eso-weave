# Feature Specification: Graphical User Interface

**Feature Branch**: `009-gui`

**Created**: 2026-07-11

**Status**: Draft

**Input**: User description: "GUI per master specification section 10. A single resizable eframe/egui main window that integrates and controls all subsystems (status region with Suspend/Resume, Go Fish/Stop Fishing, a PixelBeacon status light, Install and Uninstall; a skills region with per-slot active, weave type, and delay override; a menu bar with File and View including a Live Log toggle; a colorized live log viewer over the ring buffer with pause-scroll and a level filter; and a settings surface covering all of section 10.3). The window wires the subsystems together, including a worker loop that pumps the pixel bus reader and routes its events. Depends on features 001, 003, 006, and 007. The correctness-bearing logic lives in a testable application view-model separated from the egui rendering. Adds the eframe/egui dependency (glow backend)."

## Clarifications

### Session 2026-07-11

Resolved under the Build-Phase Autopilot Protocol from the master specification
(section 10) and the constitution (no options were escalated).

- Q: How is the GUI made testable given a native window cannot be exercised
  headlessly? -> A: The correctness-bearing logic (display-state derivation from
  subsystem status, UI-intent handling, the settings-to-config mapping for every
  section-10.3 category, and the reader-event routing) lives in a pure application
  view-model that is unit-tested. The egui rendering is a thin layer that only
  reads the view-model and emits intents; it is validated with a documented manual
  checklist since a window cannot be driven in the automated environment.
- Q: What are the exact PixelBeacon status-light states and tooltips? -> A: Green
  when installed and current; red otherwise. The tooltip states the exact
  condition: "installed and current", "installed but outdated", "not installed",
  or "AddOns directory not found". These map to the Beacon Manager's status plus
  the discovery not-found result.
- Q: How does the app state indicator and Suspend/Resume behave? -> A: The
  indicator shows Running or Suspended; the button toggles the input engine's
  suspend state and its label reflects the action (Suspend when running, Resume
  when suspended). Suspend-exempt toggles remain active while suspended, per the
  input engine contract.
- Q: How does Go Fish/Stop Fishing relate to the fishing controller? -> A: The
  button calls the fishing controller's enable/disable intake; the indicator shows
  the controller's state (disabled, or an active fishing state). Both the button
  and the future in-game hotkey drive the same enable/disable operation.
- Q: How is the reader-event routing defined? -> A: A pure routing function maps a
  reader event to subsystem calls: Latency sets the weave engine's current latency;
  SignalLost clears the weave latency and disables the fishing controller;
  FishingStarted, BiteDetected, and FishingStopped go to the fishing controller;
  Heartbeat is observed. The worker loop that samples the reader and drives ticks
  on a timer is thin around this routing and is validated on real hardware.
- Q: How are settings applied and persisted? -> A: Opening settings loads the
  current configuration into an editable form; applying writes the form back into
  the configuration and saves it to the config file (settings-only, additive
  sections, `.invalid` fallback preserved). An invalid field falls back to its
  default with a surfaced notice, consistent with the existing config and subsystem
  loaders.
- Q: What is out of scope for this slice? -> A: The in-game fishing hotkey binding
  (delivered when the input binding surface wires it) and any behavior beyond
  section 10. This slice delivers the window, the view-model, the subsystem wiring,
  and the worker loop.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Control the app from the status region (Priority: P1)

A player opens the application and, from the status region, sees whether it is
running or suspended and whether fishing is active, and toggles each with a single
button, so they can start and stop automation without editing files.

**Why this priority**: The status region is the primary control surface; without
it the application cannot be operated. It is the minimum viable window.

**Independent Test**: Drive the view-model with stubbed subsystems: toggling
Suspend/Resume flips the input engine's suspend state and the indicator label;
toggling Go Fish/Stop Fishing flips the fishing controller's enabled state and the
indicator; the derived labels match the underlying state in every case.

**Acceptance Scenarios**:

1. **Given** the app is running, **When** the Suspend button is activated, **Then**
   the input engine becomes suspended and the indicator shows Suspended with a
   Resume button.
2. **Given** fishing is disabled, **When** Go Fish is activated, **Then** the
   fishing controller becomes enabled and the indicator shows an active fishing
   state with a Stop Fishing button.
3. **Given** the app is suspended, **When** Resume is activated, **Then** the input
   engine resumes and the indicator shows Running.

---

### User Story 2 - Manage the PixelBeacon from the status region (Priority: P1)

A player sees at a glance whether the beacon addon is installed and current via a
status light, reads the exact condition in a tooltip, and installs, updates, or
uninstalls it with a button (uninstall requiring a single confirmation), so the
fishing dependency is managed without leaving the app.

**Why this priority**: Beacon management is a core section-10 control and gates the
fishing feature; the status light is the at-a-glance health signal.

**Independent Test**: With a stubbed Beacon Manager returning each status, the
derived light color and tooltip text match the defined mapping; the Install intent
calls install; the Uninstall intent is disabled when not installed and, when
confirmed, calls uninstall.

**Acceptance Scenarios**:

1. **Given** the beacon is installed and current, **When** the status is shown,
   **Then** the light is green with the "installed and current" tooltip.
2. **Given** the beacon is not installed, **When** the status is shown, **Then**
   the light is red with the "not installed" tooltip and the Uninstall button is
   disabled.
3. **Given** the beacon is installed but outdated, **When** Install is activated,
   **Then** the manager updates it and the status becomes installed and current.
4. **Given** the beacon is installed, **When** Uninstall is activated and the
   confirmation is accepted, **Then** the manager removes it and the status becomes
   not installed.

---

### User Story 3 - Configure skill slots (Priority: P1)

A player configures each of the seven skill slots from the skills region: toggling
active, choosing a weave type from the fixed list, and setting a per-slot delay
override, with the changes reflected in the running configuration.

**Why this priority**: The skills region is how the weave behavior is configured;
it is a primary section-10 surface alongside the status region.

**Independent Test**: The skills view-model exposes one row per slot with the
correct label (including Ultimate and Synergy), and editing active, weave type, or
an override updates the corresponding slot in the weave configuration.

**Acceptance Scenarios**:

1. **Given** the skills region, **When** it is shown, **Then** there is one row per
   slot with its label, active state, weave type, and override control.
2. **Given** a slot, **When** its active checkbox is toggled, **Then** the slot's
   active flag changes in the configuration and its input pass-through follows.
3. **Given** a slot, **When** its weave type is changed to another of the four
   types, **Then** the slot's weave type updates.
4. **Given** a slot, **When** a per-slot delay override is set or cleared, **Then**
   the slot's override updates (a cleared override uses the global default).

---

### User Story 4 - Read the live log (Priority: P2)

A player opens the Live Log panel from the View menu and watches recent events
colorized by level, filters to a minimum level, and scrolls back without the view
jumping, so they can diagnose behavior at runtime.

**Why this priority**: The log viewer is a diagnostic aid, valuable but not
required to operate the core automation, so it ranks below the P1 controls.

**Independent Test**: The log view-model reads a snapshot of the ring buffer,
applies the panel-local level filter, and reports autoscroll state; the color for
each level matches the defined mapping; toggling the View item attaches or detaches
the panel.

**Acceptance Scenarios**:

1. **Given** events in the ring buffer, **When** the Live Log panel is shown,
   **Then** it lists the most recent events colorized by level.
2. **Given** the panel with a level filter set, **When** applied, **Then** only
   events at or above the filter level are shown.
3. **Given** the View menu, **When** the Live Log item is unchecked, **Then** the
   panel is removed and its resources released; re-checking re-attaches it.

---

### User Story 5 - Edit and persist settings (Priority: P2)

A player opens Settings and edits any configurable value (keybindings, delays,
latency adaptation, fishing timings, sampling interval and tolerance, AddOns path
and environment, log level and file logging, theme, always-on-top), applies, and
the choices persist across restarts.

**Why this priority**: Settings make the app tunable and persist the user's setup;
important but layered on top of the operable core.

**Independent Test**: Loading settings populates the form from the configuration;
applying maps the form back into the configuration and saves it; reloading yields
the applied values; an invalid field falls back with a notice.

**Acceptance Scenarios**:

1. **Given** the current configuration, **When** Settings is opened, **Then** the
   form shows every section-10.3 value from the configuration.
2. **Given** edited settings, **When** applied, **Then** the configuration is
   updated and saved, and reopening shows the new values.
3. **Given** a changed theme or always-on-top, **When** applied, **Then** the
   window reflects the change and it persists across restarts.

---

### User Story 6 - Subsystem wiring reflects live state (Priority: P2)

While the app runs, the pixel bus reader's events are routed to the right
subsystems (latency to the weave engine, signal loss to weave and fishing, fishing
events to the fishing controller), so the on-screen state and the automation follow
the live signal.

**Why this priority**: Wiring is what makes the window a control center rather than
a static form, but the routing logic is testable independently of the window.

**Independent Test**: The pure routing function, given each reader event, performs
the defined subsystem calls against stubs; a Latency event sets the weave latency,
a SignalLost clears it and disables fishing, and fishing events reach the
controller.

**Acceptance Scenarios**:

1. **Given** a Latency reader event, **When** routed, **Then** the weave engine's
   current latency is set to that value.
2. **Given** a SignalLost reader event, **When** routed, **Then** the weave
   engine's latency is cleared and the fishing controller is disabled.
3. **Given** a fishing reader event (started, bite, stopped), **When** routed,
   **Then** it is delivered to the fishing controller.

---

### Edge Cases

- What happens when the AddOns directory cannot be resolved? The status light is
  red with the "AddOns directory not found" tooltip, and Install and Uninstall
  surface the not-found condition rather than acting.
- What happens when Uninstall is requested but the beacon is unmanaged? The Beacon
  Manager refuses and the app surfaces the unmanaged condition; nothing is deleted.
- What happens when the config file is corrupt at startup? The app starts on
  defaults, preserves the bad file with a `.invalid` suffix, and surfaces the
  notice (the existing config behavior), so the window always opens.
- What happens when the Live Log panel is closed? Its resources are released and no
  log snapshotting occurs until it is reopened.
- What happens when a settings field is invalid on apply? It falls back to its
  default with a surfaced notice, matching the config and subsystem loaders.
- What happens to suspend-exempt toggles while suspended? They remain active per
  the input engine contract; the status indicator still shows Suspended.
- What happens when the beacon is installed while the game is running? The
  operation succeeds and the app surfaces the reload-required reminder from the
  Beacon Manager.

## Requirements *(mandatory)*

### Functional Requirements

Window and structure:

- **FR-001**: The application MUST present a single resizable window with a status
  region, a skills region, a menu bar, and an optional bottom log panel.
- **FR-002**: The menu bar MUST provide File (settings, exit) and View (a Live Log
  toggle that attaches the log panel when checked and removes it, releasing its
  resources, when unchecked).

Status region:

- **FR-003**: The status region MUST show an application-state indicator (Running
  or Suspended) and a button that toggles the input engine's suspend state, with a
  label reflecting the action.
- **FR-004**: The status region MUST show a fishing-state indicator and a button
  that toggles the fishing controller's enabled state, with a label reflecting the
  action.
- **FR-005**: The status region MUST show a PixelBeacon status light that is green
  only when the beacon is installed and current and red otherwise, with a tooltip
  stating the exact condition (installed and current, installed but outdated, not
  installed, or AddOns directory not found).
- **FR-006**: The status region MUST provide an Install button (install or update
  the beacon) and an Uninstall button that requires a single confirmation and is
  disabled when the beacon is not installed.

Skills region:

- **FR-007**: The skills region MUST show one row per skill slot with its label
  (including "Ultimate (R)" and "Synergy (X)"), an active checkbox, a weave-type
  dropdown limited to the fixed four types, and a per-slot delay override control.
- **FR-008**: Editing a slot's active flag, weave type, or delay override MUST
  update the corresponding slot in the weave configuration.

Live log viewer:

- **FR-009**: The live log panel MUST display the most recent events from the
  in-memory ring buffer, working regardless of whether file logging is enabled.
- **FR-010**: Log events MUST be colorized by level (ERROR red, WARN amber, INFO
  neutral, DEBUG dim, TRACE dimmer).
- **FR-011**: The panel MUST autoscroll while at the bottom and stop autoscrolling
  when the user scrolls up (pause-scroll), and MUST provide a panel-local minimum
  level filter.

Settings:

- **FR-012**: The application MUST provide an in-app settings surface that edits and
  persists all section-10.3 values: keybindings, global and per-slot delays,
  latency adaptation on/off and `k`, fishing timings, pixel bus sampling interval
  and tolerance, AddOns path override and live/pts selection, log level and file
  logging on/off, theme (dark default, light optional), and always-on-top.
- **FR-013**: Applying settings MUST write the form back into the configuration and
  save it to the config file; an invalid field MUST fall back to its default with a
  surfaced notice; theme and always-on-top MUST take effect and persist.

Subsystem wiring:

- **FR-014**: A reader-event routing MUST map each pixel bus reader event to the
  correct subsystem action: Latency sets the weave engine's current latency;
  SignalLost clears the weave latency and disables the fishing controller;
  FishingStarted, BiteDetected, and FishingStopped go to the fishing controller;
  Heartbeat is observed.
- **FR-015**: A worker loop MUST pump the reader and drive fishing ticks on a timer,
  routing events through the routing function; it MUST NOT block the UI thread.

Testability:

- **FR-016**: The display-state derivation, UI-intent handling, the
  settings-to-config mapping, and the reader-event routing MUST be implemented in a
  view-model that is unit-testable against stubbed subsystems and a crafted
  configuration, independent of the egui rendering and any live window.

### Key Entities *(include if data involved)*

- **App View-Model**: The derived display state (app-state label, fishing label,
  beacon light and tooltip, per-slot rows) plus the handlers for UI intents.
- **UI Intent**: A user action (toggle suspend, toggle fishing, install, uninstall,
  edit a slot, open or apply settings, toggle the log panel, set a log filter).
- **Beacon Light State**: Green or red plus the exact condition text, derived from
  the Beacon Manager status and discovery result.
- **Settings Form**: The editable in-memory copy of all section-10.3 values, mapped
  to and from the persisted configuration.
- **Reader Event Route**: The mapping from a reader event to subsystem calls.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Every status-region control (suspend/resume, go fish/stop, install,
  uninstall) drives the correct subsystem call and the indicators reflect the
  resulting state, in 100 percent of view-model tests.
- **SC-002**: The beacon light color and tooltip match the defined mapping for all
  four conditions, in 100 percent of cases.
- **SC-003**: Editing any skill-slot field updates the corresponding configuration
  slot, for all three field types, in 100 percent of cases.
- **SC-004**: The settings form round-trips every section-10.3 value through the
  configuration (load then apply then reload yields the applied values), in 100
  percent of cases, with invalid fields falling back with a notice.
- **SC-005**: The reader-event routing performs the correct subsystem calls for
  every event kind, in 100 percent of cases.
- **SC-006**: The log view-model applies the level filter and reports autoscroll
  state correctly, and assigns the defined color per level, in 100 percent of
  cases.
- **SC-007**: All of the above are verifiable without a live window, through the
  view-model and stubs; the rendering is validated with a documented manual
  checklist.

## Assumptions

- Scope is master specification section 10. The in-game fishing hotkey binding is
  wired when the input binding surface exposes it; everything else beyond section
  10 is out of scope.
- Features 001 (config, logging ring buffer), 003 (weave engine), 006 (beacon
  manager), and 007 (fishing controller) exist and expose the operations the
  view-model calls. Feature 005 (pixel bus reader) provides the events the routing
  consumes.
- The GUI framework is eframe/egui with the glow backend, per the master
  specification's naming; adding it is recorded as a dated decision. The egui
  rendering cannot be exercised in the automated environment, so it is validated
  with a manual checklist while all logic is unit-tested through the view-model.
- Settings follow the project's additive, user-settings-only configuration pattern;
  this slice reuses the existing per-subsystem load/store and the config store's
  corruption fallback.
- The input engine's suspend and the fishing controller's enable/disable are the
  existing operations; the window drives them and reflects their state.
