# Feature Specification: Out-Of-Band Display Detection

**Feature Branch**: `034-display-detection`

**Created**: 2026-07-27

**Status**: Draft

**Input**: GitHub issue #3 (detect the active game resolution and physical screen
size out of band to enable pixel-bus grid wrapping). Build plan
`docs/plans/plan-010.md`, slice 034. Master specification section 10.3.

## Overview

The beacon is a strip. Every slice in this build plan has made it one square
wider, and the last one took it to nine. A strip does not scale: the observables
worth publishing run to the hundreds, and a strip that long runs off the side of
the client area long before then. The scalable shape is a grid, and a grid needs
both sides of the bus to agree on how many columns fit in a row. That column
count is bounded by how wide the game's client area actually is.

Which is where the circularity the issue names comes in. The obvious way to tell
the companion the resolution would be to publish it in a square, but the
companion needs the resolution to know where the squares are. The bus cannot be
used to locate the bus. So the resolution has to come from somewhere else
entirely, and this feature is that somewhere else.

It builds nothing the operator will notice. It produces one small descriptor
(how big the render surface is, which physical screen it is on, how that screen
is scaled, and where the surface sits on it), keeps it current as the window
moves and resizes, and hands it to whatever asks. The grid that consumes it is a
separate feature and is not built here. This is the only slice in build plan 010
that changes no addon code at all.

The one genuinely interesting question is what to do about an unknown. The issue
records that the game's stored window-mode setting is an integer whose meaning
nobody has confirmed, and asks for that to be verified before shipping. This
specification takes a different route: rather than treat the unknown as a task
someone has to go and resolve, it treats the design's dependence on that unknown
as the actual defect, and removes the dependence.

## Clarifications

### Session 2026-07-27

Answered under the build-phase autopilot decision policy from the constitution,
build plan 010, GitHub issue #3, and the existing samplers and beacon path
resolution. None were escalated.

- Q: The stored window-mode value is an enum whose integer meaning is
  unconfirmed. Block the slice on confirming it, guess a mapping, or design
  around it? -> A: Design around it, and this is the load-bearing decision of the
  feature. A guessed mapping is worse than no mapping, because it produces a
  confident wrong answer about which of two stored resolutions is live, and on
  the one install we have measurements from, the two disagree (3440x1440 against
  5160x2160). Blocking is no better: the slice's real deliverable does not need
  the mapping at all. So the configuration reader publishes what it can actually
  see, which is both stored resolution pairs and the raw mode integer, and never
  claims to know which pair is live on the strength of that integer alone. When a
  live window exists, the operating system already answers the question the enum
  was being asked, and it answers it authoritatively.
- Q: Then how does the open verification item ever get closed? -> A: By
  observation, as a side effect. When both sources are readable, exactly one
  stored pair will normally match the measured client size, and that match tells
  us what the mode integer meant on that install. Recording that correspondence
  as a diagnostic accumulates the evidence the issue asked someone to go and
  gather, from ordinary use, with nobody performing a procedure. The feature does
  not act on the inference; it only records it.
- Q: Should the configuration file be able to produce a descriptor when no game
  window exists at all? -> A: Yes, but a labelled and partial one. Pre-launch
  layout is one of the issue's stated motives, so the path must be reachable. But
  a descriptor sourced from a file is a statement about what the game was
  configured to do, not about what is on screen, and the two are not
  interchangeable. The descriptor therefore carries its own provenance, and a
  consumer can refuse a configured one where a measured one is required. The
  checklist run narrowed this further: because the mode value is not mapped, the
  file determines the live surface size only when both stored resolution pairs
  agree, so that is the only case in which a configured descriptor is produced,
  and it carries the surface size alone (see FR-022).
- Q: What is authoritative when the two sources disagree? -> A: The operating
  system, without exception and without a tie-break rule. A disagreement is
  recorded as a diagnostic and changes nothing. This is deliberately blunt: the
  configuration file is a snapshot of the settings screen, and it can be stale,
  edited by hand, or describe a monitor the game is no longer on, whereas the
  measured client rectangle is the surface being captured right now.
- Q: How often is the descriptor re-resolved, and on what thread? -> A: On the
  existing sampling cycle, with no new timer and no new thread. The Windows
  sampler already resolves the client origin and the client rectangle on every
  capture, so the additional monitor and scale queries fold into a call sequence
  that runs anyway. Change detection keeps a stationary window from producing
  repeated work or repeated log lines. The configuration file is read far less
  often than that, because reading a file on every capture would be a real cost
  for a value that changes when the operator visits the settings screen.
- Q: Does the descriptor appear in the application window? -> A: No, and this
  deviates from the previous three slices, which each routed their new signal
  into the interface. Those were signals about the character, and an operator
  watching a health readout is confirming the feature works. This is geometry
  that exists for a layout calculation that has not been written. The place to
  design its readout is the feature that consumes it, where the numbers mean
  something to look at. It is logged on change at debug level and exposed for
  programmatic use, which is what "relay" in the issue asks for.
- Q: Which measurement is "the resolution" for wrap purposes? -> A: The client
  size in physical pixels. The strip is anchored to the client top-left and the
  grid has to fit inside the client area, so the client rectangle is the surface
  the layout is bounded by. Monitor size, scale, and the surface's position on
  that monitor are carried alongside because the issue asks for the physical
  screen geometry too and because the pre-launch case has no client rectangle to
  measure, but they are context, not the bound.
- Q: Does the descriptor apply the display scale to its own numbers? -> A: No. Every
  geometry value it carries is in physical device pixels, which is the unit the
  capture path, the block size, and the sample points already work in. The scale
  is reported as a separate value so a consumer can use it, and applying it
  silently would corrupt the one unit everything on this bus already agrees on.
- Q: Does the descriptor name a window mode (fullscreen, borderless, windowed)?
  -> A: No, from either source. From the stored settings it cannot be named,
  because that is the unverified mapping. From measurement it could only be
  guessed, because a surface that exactly covers its monitor is equally
  consistent with exclusive fullscreen, borderless fullscreen, and a maximized
  borderless window, and telling those apart matters to nobody here. The wrap
  layout needs the rectangle, not a name for it, so the descriptor carries the
  rectangle and no name at all. Naming it would be inventing a fact.
- Q: How often is the stored settings file read? -> A: Only when the measured
  descriptor changes, and once when detection first runs with no measurement
  available. A stationary window therefore performs no file reads at all, which
  matters because the sampling cycle is frequent and the settings change when the
  operator visits a settings screen. This makes the file read follow the same
  change-detected discipline as everything else on this cycle rather than needing
  a cadence of its own.
- Q: At what level and how often are the reconciliation diagnostics recorded? ->
  A: Debug, and only when the reconciliation outcome itself changes, not once per
  cycle and not once per file read. An agreement that has already been recorded
  is not news, and a disagreement repeated every cycle would be the log flood the
  previous slice took care to avoid.
- Q: Does detection write anything to disk? -> A: No, and this is stated as a
  requirement rather than left implicit. The feature reads a file inside the
  operator's own game data directory, which is the same neighborhood the addon
  installer writes to under a strict managed-marker rule. Detection has no reason
  to write, create, or touch anything there, so it is forbidden from doing so and
  the prohibition is tested.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - The companion knows how big the render surface is (Priority: P1)

While the game is running, the application can state the size of the game's
render surface in physical pixels, which physical screen it is on, how that
screen is scaled, and where the surface sits on it, without reading a single
pixel of the beacon.

**Why this priority**: This is the feature, and it is the prerequisite the grid
work is waiting on.

**Independent Test**: With the game running, ask the application for the
descriptor and compare each value against the game's own video settings and the
operating system's display settings.

**Acceptance Scenarios**:

1. **Given** the game window exists, **When** the descriptor is resolved,
   **Then** it reports the client size in physical pixels, the monitor's
   position and size, the monitor's scale, and the client origin in screen
   coordinates.
2. **Given** the game window does not exist, **When** the descriptor is
   resolved, **Then** no measured descriptor is produced and nothing fails.
3. **Given** a descriptor was produced, **When** it is inspected, **Then** it
   states where it came from, so a measured descriptor is never mistaken for a
   configured one.

---

### User Story 2 - It stays correct when the window changes (Priority: P1)

The operator moves the game to another monitor, resizes it, or switches between
fullscreen, borderless, and windowed. The descriptor follows, without the
operator restarting anything or telling the application what changed.

**Why this priority**: Equal to the first. A descriptor resolved once at startup
would be silently wrong for the rest of the session, and a wrong column count
puts every square of a future grid in the wrong place, which is a worse failure
than having no descriptor at all.

**Independent Test**: Move and resize the window across monitors of different
sizes and scales and confirm the reported values track. The change-detection and
re-resolution logic is testable at the desk against a scripted sequence of
readings.

**Acceptance Scenarios**:

1. **Given** a resolved descriptor, **When** the window moves to a monitor with
   a different size or scale, **Then** the next resolution reports the new
   monitor's values.
2. **Given** a resolved descriptor, **When** the window is resized, **Then** the
   next resolution reports the new client size.
3. **Given** an unchanged window, **When** the descriptor is resolved
   repeatedly, **Then** nothing is announced and no repeated work is done.
4. **Given** the window becomes undrawable (minimized) and then drawable again,
   **When** the descriptor is resolved, **Then** it degrades to no measurement
   and recovers, without producing a zero-sized or nonsense descriptor in
   between.

---

### User Story 3 - The stored settings are read, and never trusted over the screen (Priority: P2)

The application can read the game's stored video settings from disk, which lets
it describe the configured resolution before the game is even launched, and lets
it notice when what is on screen does not match what is configured.

**Why this priority**: Below the first two because the measured path is
sufficient for the wrap math, and the issue itself says as much. It earns its
place by covering the pre-launch case and by being an independent check on the
measured path.

**Independent Test**: Parse settings files at the desk: a realistic one, one
missing the keys, one with version-suffixed keys, one with an unknown mode
value, one that is truncated mid-line, and one that is not a settings file at
all.

**Acceptance Scenarios**:

1. **Given** a readable settings file, **When** it is parsed, **Then** both
   stored resolution pairs, the raw mode value, the target display index, the
   overscan adjustments, and the interface scale are reported as found.
2. **Given** a key that has acquired a version suffix, **When** the file is
   parsed, **Then** the key is still recognized.
3. **Given** a missing, unreadable, truncated, or unrecognizable file, **When**
   it is parsed, **Then** the result is an absent or partial reading and nothing
   panics.
4. **Given** both a measured descriptor and a parsed settings file, **When** the
   measured client size matches neither stored pair, **Then** the disagreement
   is recorded as a diagnostic and the measured descriptor is used unchanged.
5. **Given** both sources, **When** the measured client size matches exactly one
   stored pair, **Then** the correspondence between that pair and the stored
   mode value is recorded, and nothing behaves differently because of it.
6. **Given** no game window and a settings file whose two stored resolution
   pairs are identical, **When** a descriptor is requested, **Then** a
   descriptor marked as configured is produced carrying that surface size and no
   display geometry.
7. **Given** no game window and a settings file whose two stored pairs differ,
   **When** a descriptor is requested, **Then** no descriptor is produced,
   because which pair is live cannot be known without guessing the mode.

---

### Edge Cases

- **No game window.** The measured path yields nothing. This is the ordinary
  pre-launch state, not an error, and it is exactly when the configured path is
  worth having.
- **The window is minimized or has a zero-sized client area.** Treated as no
  measurement, matching how the capture path already treats an undrawable
  window. A zero-sized descriptor must never be published, because a consumer
  would compute a zero column count from it.
- **The mode value is one nobody has seen.** The mode is reported as
  undetermined. Nothing downstream depends on knowing it.
- **Both stored resolution pairs are identical.** Two consequences, in opposite
  directions. The correspondence observation becomes inconclusive and is not
  recorded, because a match against both pairs says nothing about the mode value.
  But it is also the one configuration in which the stored settings determine the
  live surface size without needing the mode value at all, and so the only one in
  which a configured descriptor can be produced.
- **The settings file exists but the game has never written video settings.**
  Keys are absent; a partial reading is produced rather than a fabricated one.
- **Multiple monitors with different scale factors.** The descriptor reports the
  scale of the monitor the window is on, not a system-wide value, because the
  window's own surface is what the capture and the layout both live in.
- **The operator changes settings in game while the application is running.**
  The measured path picks the change up on the next cycle. The stored settings
  are not written until the game applies them, so a stale configured reading in
  the interim is expected and is the reason it never overrides the measurement.
- **A future game patch renames a key.** That key reads as absent and the rest
  of the file still parses. Degradation is per key, not per file.

## Requirements *(mandatory)*

### Functional Requirements

**The descriptor**

- **FR-001**: The application MUST be able to produce a display descriptor
  carrying the game render surface's size in physical pixels, and, where the
  source can supply them, the origin of that surface in screen coordinates and
  the position, size, and scale factor of the physical display the surface is
  on. The surface size is the only field always present; every other field MUST
  be explicitly absent-capable, because a reading taken from stored settings can
  supply the surface size and nothing else.
- **FR-002**: Every geometry value in the descriptor MUST be expressed in
  physical device pixels, the same unit the existing capture geometry and block
  size use. The scale factor MUST be reported alongside rather than applied to
  those values.
- **FR-003**: The descriptor MUST record which source produced it, so a
  consumer can distinguish a measured descriptor from one derived from stored
  settings.
- **FR-004**: A descriptor MUST NOT be produced with a zero or negative surface
  size. The absence of a usable measurement MUST be represented as absence.

**Measuring the live surface**

- **FR-005**: The application MUST resolve the descriptor from operating system
  queries about the game window, without reading the beacon, the game's process
  memory, or any in-game data path.
- **FR-006**: Measurement MUST be supported on both Windows and Linux. Where a
  platform cannot supply a value (for example a display scale it does not
  expose), that value MUST be reported as unknown and MUST NOT block the rest of
  the descriptor. It MUST NOT be substituted with a default, because an
  unscaled-by-default reading is indistinguishable from a genuinely unscaled
  display and would be a confident wrong answer rather than a missing one.
- **FR-007**: The descriptor MUST be re-resolvable, and MUST be re-resolved
  automatically while the application is sampling, so that a window move,
  resize, monitor change, or display-mode change is reflected without operator
  action.
- **FR-008**: Re-resolution MUST NOT add a new thread or a new timer, and MUST
  NOT perform blocking work on the input hook thread.
- **FR-009**: A descriptor that has not changed MUST NOT be announced again.
  Announcement of a change MUST be at debug level, consistent with the other
  low-rate signals.
- **FR-010**: A window that cannot be resolved, or whose client area is not
  drawable, MUST yield no measurement, and the application MUST recover on a
  later cycle without restart.

**Reading the stored settings**

- **FR-011**: The application MUST be able to parse the game's stored video
  settings from its settings file, reporting the stored fullscreen resolution
  pair, the stored windowed resolution pair, the raw window-mode value, the
  exclusive-fullscreen and maximized-window preferences, the target display
  index, the overscan adjustments, and the interface scale settings.
- **FR-012**: The settings file's location MUST be derived from the existing
  resolution of the game's per-environment data directory rather than a second,
  independently maintained path.
- **FR-013**: Key matching MUST accept a base key with an optional trailing
  version suffix, because the game bumps that suffix when a setting's meaning
  changes and a fixed literal would silently stop matching after a patch.
- **FR-014**: Parsing MUST tolerate a missing file, an unreadable file, an
  unrelated file, truncated or malformed lines, absent keys, unparsable values,
  duplicate keys, and unknown keys. Any of these MUST yield an absent or partial
  reading. Parsing MUST NOT panic and MUST NOT abort on the first bad line.
- **FR-015**: The parser MUST NOT map the raw window-mode value to a named
  window mode. The value is reported as read, and the named mode is reported as
  undetermined, because no verified mapping exists. A stored resolution pair
  MUST NOT be presented as the live one on the strength of that value alone.
  The descriptor itself MUST NOT carry a named window mode from either source.
- **FR-016**: The settings file MUST be read only when the measured descriptor
  changes, and once when detection runs with no measurement available. A
  stationary window MUST produce no settings-file reads.
- **FR-017**: Detection MUST NOT write, create, delete, or modify any file or
  directory. It reads the settings file and nothing else, and it MUST NOT create
  the file or its parent directory when either is absent.

**Reconciling the two**

- **FR-018**: When a measurement is available it MUST be authoritative. A stored
  reading MUST NOT override, adjust, or suppress any measured value.
- **FR-019**: When both are available and the measured surface size matches
  neither stored pair, the disagreement MUST be recorded as a diagnostic, and
  the measured descriptor MUST be used unchanged.
- **FR-020**: When both are available and the measured surface size matches
  exactly one stored pair, the correspondence between that pair and the raw
  window-mode value MUST be recorded as a diagnostic. Nothing in the feature's
  behavior may depend on that inference.
- **FR-021**: Reconciliation diagnostics MUST be recorded at debug level and only
  when the reconciliation outcome changes, not once per cycle.
- **FR-022**: When no measurement is available, a descriptor MAY be derived from
  the stored settings, and MUST be marked as configured rather than measured.
  Because the window-mode value is not mapped (FR-015), the only case in which
  the stored settings determine the live surface size is when both stored
  resolution pairs are identical, and a configured descriptor MUST NOT be
  produced in any other case. A configured descriptor carries the surface size
  only; it MUST NOT carry display position, size, or scale, because the stored
  settings record a display index and not its geometry.

**Boundaries**

- **FR-023**: This feature MUST NOT change the addon, its manifest version, the
  strip layout, the block count, the capture geometry, the sample points, or any
  existing color contract.
- **FR-024**: This feature MUST NOT change any input, weave, fishing, or
  suppression behavior, and MUST NOT weaken, skip, or make conditional any
  existing test.
- **FR-025**: This feature MUST NOT implement the grid wrap layout. It produces
  the descriptor the wrap feature will consume and stops there.
- **FR-026**: Detection MUST be reachable behind a seam that allows the
  descriptor's production, change detection, reconciliation, and parsing to be
  exercised in tests with no game, no window, and no display hardware.
- **FR-027**: The master specification's pixel-bus section MUST document the
  descriptor as the out-of-band input the future grid contract derives from, and
  MUST state that this feature adds no block. The changelog MUST record the
  feature plus dated decisions for the two contracts later work inherits: the
  refusal to map the window-mode value, and the extension of the existing
  sampling seam rather than the addition of a second one.

### Key Entities

- **Display descriptor**: The feature's single output. The render surface size
  and origin, the display's position, size, and scale, and the provenance of the
  reading. All geometry in physical pixels. It carries no window-mode name.
  Absent when no usable reading exists.
- **Stored video settings**: What the game's settings file says about video, as
  found. Every field independently present or absent. Includes a raw window-mode
  value whose meaning is deliberately not interpreted.
- **Reconciliation diagnostic**: A record that the two sources agreed, disagreed,
  or revealed a correspondence. Observational only; nothing branches on it.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: With the game running, the reported surface size matches the
  game's configured live resolution and the reported display geometry matches the
  operating system's display settings, on both a scaled and an unscaled display.
- **SC-002**: Moving the window between monitors, resizing it, and changing
  display mode each produce an updated descriptor within one sampling cycle, with
  no restart and no operator action.
- **SC-003**: A stationary window produces no repeated announcements, no
  settings-file reads, and no growth in the live log at the operator's default
  level.
- **SC-004**: Every malformed-input case for the settings parser (missing file,
  unreadable file, unrelated content, truncated line, absent key, unparsable
  value, duplicate key, unknown key, version-suffixed key, unknown mode value)
  yields an absent or partial reading, verified by test, with no panic.
- **SC-005**: No stored reading ever changes a measured value, verified by test
  over agreeing, disagreeing, and partially-present combinations.
- **SC-006**: A descriptor is never produced with a zero or negative surface
  size, verified by test over zero-sized, negative, and absent inputs.
- **SC-007**: With no window available, a configured descriptor is produced only
  when the two stored resolution pairs agree, and never carries display
  geometry. Verified by test over agreeing pairs, differing pairs, a single
  present pair, and no pairs at all.
- **SC-008**: Detection writes nothing. Verified by test: running detection
  against a directory containing a settings file, and against one containing
  nothing at all, leaves both byte-for-byte and entry-for-entry unchanged.
- **SC-009**: The addon files, the manifest version, and every existing
  pixel-bus contract are byte-for-byte unchanged, and every pre-existing test
  passes unmodified.
- **SC-010**: The full merge gate passes with no test weakened, skipped, or made
  conditional.

## Assumptions

- **The client rectangle is the render surface.** The game renders into its
  client area and the beacon is anchored to that area's top-left, which the
  capture path already relies on. The wrap layout will be bounded by the same
  rectangle.
- **The operating system is a better witness than a settings file.** The file
  records an intent that was saved at some point; the window is the thing being
  captured right now.
- **The settings file format is stable in shape even where key names are not.**
  Lines are simple key-value assignments with quoted values. Individual key names
  may acquire version suffixes or be renamed, which is why matching is prefix
  based and degradation is per key.
- **Nothing consumes the descriptor yet.** As with the previous three slices, the
  observable lands before anything depends on it, so it can be seen to be correct
  first.
- **Pure Wayland sessions without an X surface remain out of scope**, unchanged
  from the existing sampler's stated limitation. They are not made worse by this
  feature.

## Dependencies

- The existing platform surface samplers and their window resolution.
- The existing per-environment game data directory resolution used by the beacon
  installer, which locates the settings file.
- The existing sampling cycle, which carries re-resolution.
- The master specification's pixel-bus section, for documenting the descriptor as
  the input to the future grid contract.
