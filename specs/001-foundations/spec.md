# Feature Specification: Foundations (Project Bootstrap, Config Store, Logging)

**Feature Branch**: `001-foundations`

**Created**: 2026-07-11

**Status**: Draft

**Input**: User description: "Foundations: the Rust project bootstrap plus the Config Store and Logging subsystems, drawing scope from master specification sections 5, 11, 12, and 14. Excludes the Input Engine, Weave Engine, Fishing, PixelBeacon, GUI, and any in-game or pixel functionality."

## Clarifications

### Session 2026-07-11

Resolved under the Build-Phase Autopilot Protocol from the master specification
and the constitution (no options were escalated).

- Q: What user settings does the Config Store persist in this foundations slice?
  -> A: The schema version plus logging preferences (active level and whether the
  persisted file sink is enabled). No combat, input, weave, or fishing settings
  exist yet; they arrive with their own slices.
- Q: What is the exact preserved-name scheme for a corrupt settings file?
  -> A: The original file path with a `.invalid` suffix appended. If such a file
  already exists, a numeric discriminator is appended (for example
  `config.json.invalid.2`) so a previously preserved file is never overwritten.
- Q: How do the logging runtime controls relate to persisted settings?
  -> A: Logging initializes its active level and file-sink state from persisted
  settings at startup. Runtime changes take effect immediately and are written to
  disk only on the normal settings save, so the settings file remains
  settings-only.
- Q: How is a notice surfaced in this slice, given the GUI is out of scope?
  -> A: A notice is emitted as a warn-level log event and returned to the caller
  as a typed outcome. Visual presentation is deferred to the GUI slice, which
  consumes the same event and outcome.
- Q: FR-015 names suspend and keystrokes, which belong to the Input Engine. How
  is that scoped here? -> A: In this slice the logging facility only guarantees it
  never emits input contents above debug and exposes a suppression control. The
  suspend-driven use of that control (no keystroke logging while suspended) is
  wired by the later Input Engine slice, which owns suspend state.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Settings persist and survive corruption (Priority: P1)

An operator adjusts application settings, closes the application, and reopens it
later. Their settings are exactly as they left them. If the settings file has
been damaged or hand-edited into an invalid state, the application still opens
with sensible defaults instead of failing, preserves the damaged file so nothing
is lost, and tells the operator what happened.

**Why this priority**: Durable, trustworthy settings are the base contract every
later subsystem depends on. If settings cannot be stored and reloaded safely,
nothing built on top of them can be trusted.

**Independent Test**: Write a settings value, restart, and confirm it reloads
unchanged. Separately, corrupt the settings file, restart, and confirm the
application starts on defaults, keeps the bad file under a preserved name, and
raises a notice.

**Acceptance Scenarios**:

1. **Given** a fresh install with no settings file, **When** the application
   starts, **Then** it loads built-in defaults and, on first save, writes a
   settings file at the platform settings location.
2. **Given** an operator has changed and saved settings, **When** the
   application restarts, **Then** every saved setting reloads with the same
   value.
3. **Given** a settings file that is not valid, **When** the application starts,
   **Then** it falls back to defaults, preserves the original file under a
   distinct preserved name, and surfaces a notice describing the fallback.
4. **Given** a settings file written by an older schema version, **When** the
   application starts, **Then** it migrates the content forward and never
   silently discards recognized settings.
5. **Given** the settings file, **When** it is inspected on disk, **Then** it is
   valid JSON, encoded without a byte order mark, uses line-feed endings, is
   human-readable (pretty-printed), and contains only user settings, never
   runtime or session state.

---

### User Story 2 - Controllable, private diagnostic logging (Priority: P2)

An operator investigating behavior can raise or lower how much detail the
application records, at runtime, without restarting. Recent activity is always
available to view in the application, and the operator can optionally turn on a
persisted log file. Sensitive input contents never appear in ordinary logs.

**Why this priority**: Diagnosability is essential for a tool that automates
input, and the privacy boundary on what may be logged is a stated project
constraint. Later subsystems log through this facility, so it must exist early.

**Independent Test**: Change the active verbosity at runtime and confirm records
appear or disappear accordingly, view the most recent records from the in-memory
buffer, toggle the optional file on and confirm a dated file appears, and
confirm input contents are absent from records at ordinary verbosity.

**Acceptance Scenarios**:

1. **Given** the application is running, **When** the operator selects a
   verbosity level (off, error, warn, info, debug, or trace), **Then** records
   at or above that level are captured and lower levels are not, effective
   immediately.
2. **Given** activity has occurred, **When** the operator views recent activity,
   **Then** the most recent records are available from an always-on in-memory
   buffer up to a bounded capacity, independent of whether a file is enabled.
3. **Given** the optional persisted log is enabled, **When** records are
   produced, **Then** they are appended to a file named for the current month at
   the platform log location, each line carrying a coordinated-universal-time
   timestamp, the level, the source, and the message.
4. **Given** the application is at ordinary verbosity, **When** input-related
   activity is logged, **Then** the specific input contents are not present in
   any record.

---

### User Story 3 - Buildable, CI-clean project skeleton (Priority: P3)

A developer can clone the repository and build it, and the standard quality gate
(format check, lint with warnings denied, and the test suite) passes cleanly on
a single cross-platform crate whose released version comes from one place.

**Why this priority**: The crate skeleton is the ground the other two stories
stand on. It is listed third only because it delivers developer-facing rather
than operator-facing value; in build order it comes first.

**Independent Test**: Run the format check, the lint, and the test suite on a
clean checkout and confirm all pass, and confirm the reported application version
comes from a single declared source.

**Acceptance Scenarios**:

1. **Given** a clean checkout, **When** the standard quality gate runs, **Then**
   the format check, the lint (warnings denied), and the test suite all pass.
2. **Given** the project, **When** its version is reported, **Then** it is single
   sourced and follows semantic versioning.
3. **Given** the source layout, **When** platform-specific behavior is added
   later, **Then** it fits an established per-platform module seam without
   restructuring the crate.

---

### Edge Cases

- What happens when the settings location is not writable? The application
  continues on in-memory defaults and surfaces a notice rather than failing.
- What happens when the preserved-name target for a corrupt settings file
  already exists? A previously preserved file is not clobbered without a
  distinguishable name.
- What happens when the log location is not writable while the file sink is
  enabled? The in-memory buffer remains available and the failure to write the
  file is surfaced, not fatal.
- What happens when an unknown setting key is present in a loaded file? It is not
  silently discarded without a logged warning.
- What happens at a month boundary while the file sink is enabled? Subsequent
  records are written to the file named for the new month.

## Requirements *(mandatory)*

### Functional Requirements

Project skeleton:

- **FR-001**: The project MUST build as a single cross-platform crate whose
  released version is single sourced and follows semantic versioning.
- **FR-002**: The project MUST pass the standard quality gate (format check, lint
  with warnings denied, and the test suite) on a clean checkout.
- **FR-003**: The source layout MUST provide a per-platform module seam so that
  platform-specific backends can be added in later slices without restructuring.

Config Store:

- **FR-004**: The system MUST load user settings on startup from the platform
  settings location, falling back to built-in defaults when no file exists.
- **FR-005**: The system MUST persist user settings to that location on request,
  as valid JSON, encoded without a byte order mark, with line-feed endings, and
  human-readable (pretty-printed).
- **FR-006**: The settings file MUST contain only user settings and MUST NOT
  contain runtime or session state (for example timestamps, host metadata,
  session identifiers, or derived lookup tables).
- **FR-007**: The system MUST carry a top-level schema version and MUST migrate
  content from older schema versions forward on load.
- **FR-008**: The system MUST NOT silently discard recognized or unknown settings
  during load or migration without recording a warning.
- **FR-009**: On encountering an invalid settings file, the system MUST fall back
  to defaults, preserve the original file by appending a `.invalid` suffix to its
  path (appending a numeric discriminator when such a file already exists so a
  prior preserved file is never overwritten), and surface a notice describing the
  fallback.
- **FR-010**: The system MUST continue on defaults with a surfaced notice when the
  settings location cannot be read or written, rather than failing to start.
- **FR-016**: The persisted settings in this slice MUST consist of the schema
  version and logging preferences only (the active level and whether the file
  sink is enabled). No other settings categories exist yet, and their absence
  MUST NOT be treated as an error.

Logging:

- **FR-011**: The system MUST record structured log events and MUST allow the
  active verbosity level (off, error, warn, info, debug, trace) to be selected at
  runtime, taking effect without a restart.
- **FR-012**: The system MUST maintain an always-on in-memory buffer of the most
  recent events up to a bounded capacity, available independently of any file
  output. When the capacity is reached, the oldest events MUST be evicted first
  (ring semantics).
- **FR-013**: The system MUST provide an optional, runtime-toggleable persisted
  log written to a file named for the current month at the platform log location.
- **FR-014**: Each persisted log line MUST include a coordinated-universal-time
  timestamp in ISO-8601 form, the level, the source, and the message, in that
  order.
- **FR-015**: The logging facility MUST NOT emit specific input contents in any
  log event above debug verbosity, and MUST provide a suppression control a caller
  can use to prevent input logging. The suspend-driven use of that control (no
  keystroke logging while suspended) is owned by the later Input Engine slice,
  which holds suspend state; this slice provides only the guarantee and the
  control.
- **FR-017**: Logging MUST initialize its active level and file-sink state from
  the persisted settings at startup. Runtime changes to either MUST take effect
  immediately and MUST be written to disk only as part of the normal settings
  save, so no runtime state leaks into the settings file.
- **FR-018**: A surfaced notice in this slice MUST be emitted as a warn-level log
  event and returned to the caller as a typed outcome. Visual presentation of
  notices is deferred to the GUI slice.

### Key Entities *(include if feature involves data)*

- **Settings**: The persisted user configuration. Carries a schema version and
  user-adjustable settings only. In this slice that is the schema version plus
  logging preferences (active level and file-sink enabled); later slices extend
  it. Has a well-defined default form and a forward-migration path across schema
  versions.
- **Log Event**: A single structured record with a timestamp, a level, a source,
  and a message. Flows to the in-memory buffer always and to the optional file
  when enabled.
- **Log Configuration**: The runtime-adjustable state governing the active level,
  whether the file sink is enabled, and the in-memory buffer capacity. The active
  level and file-sink flag are operator preferences that are initialized from and
  saved to Settings; the buffer capacity is a fixed default (1000) in this slice.
  No other runtime state is written to the settings file.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: On a clean checkout, the standard quality gate (format check, lint
  with warnings denied, and test suite) passes with zero failures and zero
  warnings.
- **SC-002**: A setting changed and saved in one session is present and unchanged
  after a restart in 100 percent of cases.
- **SC-003**: Starting against an invalid settings file results in a running
  application on defaults, the original file preserved, and a notice raised, in
  100 percent of cases, with no crash.
- **SC-004**: Changing the active verbosity level takes effect for subsequent
  events without a restart, verifiable within the same session.
- **SC-005**: At ordinary verbosity, no log event contains specific input
  contents, verifiable by inspecting captured events during input-related
  activity.
- **SC-006**: The most recent events are retrievable from the in-memory buffer
  whether or not the file sink is enabled.

## Assumptions

- Scope is limited to the project skeleton, the Config Store (master
  specification section 11), and the Logging subsystem (section 12). The Input
  Engine, Weave Engine, Fishing, PixelBeacon, and GUI are explicitly out of scope
  and handled in later slices; the live log viewer that consumes the in-memory
  buffer is part of the later GUI slice.
- Platform settings and log locations follow the master specification: settings
  at the platform per-user configuration directory under an eso-weave folder, and
  logs at the platform per-user state or log directory under an eso-weave folder,
  with the documented fallbacks.
- The default in-memory buffer capacity is 1000 events, per the master
  specification. The default active level is info, and the optional file sink is
  off by default; both are reasonable defaults chosen where the specification did
  not fix a value, and both are operator-adjustable.
- The preserved name for a corrupt settings file uses a distinct suffix so a
  damaged file is never confused with the live file.
- The pre-existing operator-owned files (LICENSE, .gitignore, .gitattributes) are
  present and are not regenerated by this feature.
