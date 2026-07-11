# Feature Specification: Installer and First-Run Experience

**Feature Branch**: `011-installer-first-run`

**Created**: 2026-07-11

**Status**: Draft

**Input**: User description: "Windows installer and first-run experience fix. The
current installer runs but gives zero visible feedback, creates only a nested
Start Menu shortcut with no desktop shortcut and no launch option, and the
application flashes a console window on launch while any startup failure is
invisible. Deliver a guided install experience that visibly confirms completion,
offers to launch the application, and leaves the application easy to find; make
the application open cleanly with no stray console window; and ensure a first-run
failure is surfaced to the user rather than silent. Update user-facing docs so
people know where the application and its logs live."

## Clarifications

### Session 2026-07-11

Resolved under the Build-Phase Autopilot Protocol from the master specification
(section 13, which requires the Windows MSI to provide standard install,
uninstall, upgrade-in-place, a Start Menu entry, and the application icon, and to
never write to game or Documents directories), the approved slice plan, and the
constitution. No options were escalated.

- Q: What triggered this slice? -> A: A user installed the released Windows
  package, approved the elevation prompt, and saw nothing happen: the installer
  showed no wizard and no completion confirmation, nothing landed on the desktop,
  and the application did not open. The install had in fact succeeded silently.
  This slice closes that visible-feedback and discoverability gap and hardens the
  application's first launch.
- Q: Does adding a desktop shortcut and a launch option violate the section 13
  constraint that the MSI never writes to game or Documents directories? -> A:
  No. A desktop shortcut is written to the user's Desktop, and the launch option
  starts the already-installed application; neither writes to the game
  installation or the Documents known folder. Upgrade-in-place, uninstall, the
  Start Menu entry, and the icon requirement from section 13 are preserved.
- Q: Should the launch option run automatically, or be user-chosen? -> A: It MUST
  be a user choice presented at the end of the install, defaulting to enabled but
  never launching without the user leaving it selected. Silent or unattended
  installs MUST NOT launch the application.
- Q: What counts as a "surfaced" first-run failure? -> A: If the application
  fails during startup, the user MUST see a visible message identifying that the
  application failed to start, and the failure detail MUST also be recorded to the
  application log. A failure MUST NOT result in nothing visible happening.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Install visibly completes (Priority: P1)

A person runs the downloaded Windows installer and follows a guided sequence
(welcome, license, install location, progress, completion) that ends with an
unmistakable confirmation that the application is installed.

**Why this priority**: The reported failure is that the install appeared to do
nothing. A visible, guided flow that confirms completion is the single most
important fix; without it the product looks broken on first contact.

**Independent Test**: Run the installer on a clean Windows machine and confirm
each step is shown and that a completion screen states the install succeeded.

**Acceptance Scenarios**:

1. **Given** the downloaded installer, **When** the user runs it and approves the
   elevation prompt, **Then** a guided wizard is shown with a welcome step, a
   license step, an install-location step, a progress indicator, and a completion
   step.
2. **Given** the wizard reaches the end, **When** the install has succeeded,
   **Then** the completion step clearly states that the application is installed.
3. **Given** the user cancels partway, **When** they confirm cancellation,
   **Then** the machine is left without a partial install and the user is told the
   install did not complete.

### User Story 2 - The application is easy to find and start (Priority: P1)

After installing, the user can immediately start the application, either from the
completion step of the installer or from a shortcut they can find without
hunting.

**Why this priority**: A silent, discoverable-only-by-search install is why the
user thought nothing happened. Making the app trivially launchable is core to the
fix and independently valuable.

**Independent Test**: Complete an install, use the completion-step launch option,
and separately confirm a desktop shortcut and a Start Menu entry both start the
application.

**Acceptance Scenarios**:

1. **Given** the completion step, **When** the user leaves the launch option
   selected and finishes, **Then** the application starts.
2. **Given** a completed install, **When** the user looks on the desktop, **Then**
   a shortcut that starts the application is present.
3. **Given** a completed install, **When** the user opens the Start Menu, **Then**
   an entry that starts the application is present.
4. **Given** an unattended or silent install, **When** it finishes, **Then** the
   application is not launched automatically.

### User Story 3 - The application opens cleanly (Priority: P2)

When the application starts, the user sees the application window and nothing
else; no stray console or command window appears alongside it.

**Why this priority**: A console flash makes a finished product feel unpolished
and, worse, makes users unsure whether the app is a legitimate GUI program. It is
high-value polish but secondary to the install being visible at all.

**Independent Test**: Launch the installed application and confirm only the
application window appears, with no console window before, during, or after.

**Acceptance Scenarios**:

1. **Given** the installed application, **When** the user starts it from any
   shortcut, **Then** only the application window is shown and no console window
   appears.

### User Story 4 - First-run failures are never silent (Priority: P2)

If the application fails while starting up, the user is shown a clear message that
it failed, and the failure is recorded for later diagnosis, instead of the
program simply vanishing.

**Why this priority**: Once the console window is hidden, a startup crash would
otherwise be completely invisible, recreating the exact "nothing happened"
complaint at the application layer. This guardrail keeps the fix durable.

**Independent Test**: Force a startup failure and confirm a visible message
appears and a corresponding entry is written to the application log.

**Acceptance Scenarios**:

1. **Given** a condition that makes startup fail, **When** the user launches the
   application, **Then** a visible message states that the application failed to
   start.
2. **Given** the same failure, **When** the user inspects the application log,
   **Then** the log contains an entry describing the failure.

### User Story 5 - Users can find the application and its logs (Priority: P3)

User-facing documentation tells people where the installed shortcuts are and
where the application writes its logs, so they can start it and self-diagnose.

**Why this priority**: Documentation reduces support burden and complements the
in-product fixes, but the product must first behave correctly; hence lowest
priority in this slice.

**Independent Test**: Read the updated docs and confirm they state the shortcut
locations and the log directory, matching the actual installed behavior.

**Acceptance Scenarios**:

1. **Given** the updated documentation, **When** a user reads the install
   section, **Then** it states where the Start Menu and desktop shortcuts are and
   where log files are written.

### Edge Cases

- What happens when the user runs the installer without accepting the license?
  The install MUST NOT proceed.
- What happens on an upgrade over a prior version? Upgrade-in-place MUST be
  preserved, the completion step MUST still be shown, and existing shortcuts MUST
  remain valid.
- What happens when the desktop or Start Menu location is redirected or
  unavailable? The install MUST still complete and report success; a shortcut that
  cannot be created MUST NOT fail the whole install.
- What happens when the launch option is selected but the application cannot
  start? The startup-failure message (User Story 4) MUST be shown.
- What happens for a silent or unattended install? No wizard interaction is
  required and the application MUST NOT auto-launch.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The Windows installer MUST present a guided wizard with, at minimum,
  a welcome step, a license step, an install-location step, a progress indicator,
  and a completion step.
- **FR-002**: The completion step MUST clearly confirm that the application was
  installed successfully.
- **FR-003**: The license step MUST require the user to accept the license before
  the install can proceed.
- **FR-004**: The completion step MUST offer a user-controlled option to launch
  the application, defaulting to selected, and MUST start the application only if
  that option remains selected when the user finishes.
- **FR-005**: A silent or unattended install MUST NOT launch the application.
- **FR-006**: The install MUST create a desktop shortcut that starts the
  application.
- **FR-007**: The install MUST continue to create a Start Menu entry that starts
  the application (preserving the section 13 requirement).
- **FR-008**: The installer MUST preserve standard install, uninstall, and
  upgrade-in-place behavior and MUST NOT write to game or Documents directories.
- **FR-009**: Failure to create an optional shortcut MUST NOT cause the overall
  install to fail.
- **FR-010**: When started from any shortcut or the launch option, the application
  MUST display only its own window, with no console or command window shown.
- **FR-011**: If the application fails during startup, it MUST show the user a
  visible message indicating the failure.
- **FR-012**: A startup failure MUST also be recorded to the application log.
- **FR-013**: User-facing documentation MUST state the desktop and Start Menu
  shortcut locations and the application log directory, consistent with actual
  behavior.

### Key Entities

- **Install experience**: The guided sequence a user moves through, its steps, and
  the completion confirmation and launch option.
- **Shortcuts**: The desktop and Start Menu entries that start the application.
- **Startup failure notice**: The visible message plus the log entry produced when
  the application cannot start.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A first-time user running the installer sees a completion
  confirmation in 100% of successful installs (no silent completion).
- **SC-002**: After install, a user can start the application within 10 seconds
  without searching, using either the completion-step launch option or a desktop
  shortcut.
- **SC-003**: In 100% of launches from a shortcut, no console or command window is
  visible.
- **SC-004**: In 100% of forced startup-failure trials, the user sees a visible
  failure message and a matching log entry exists.
- **SC-005**: Silent or unattended installs launch the application in 0% of runs.
- **SC-006**: A user following the updated documentation can locate a working
  shortcut and the log directory on the first attempt.

## Assumptions

- The target is the Windows MSI produced by the existing pinned release pipeline;
  Linux packaging (`.deb`, AppImage) is out of scope for this slice.
- The Windows package remains a per-machine install requiring elevation, as today.
- The application already has a logging facility with a known log directory; this
  slice records startup failures into it rather than introducing new logging
  infrastructure.
- The license shown in the installer is the repository's existing license.
- "Visible failure message" means a native operating-system dialog appropriate for
  a program running without a console; the exact presentation is an implementation
  detail resolved during planning.
- Version continues to be single-sourced from the project manifest per section 13.
