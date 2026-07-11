# Feature Specification: Beacon Manager

**Feature Branch**: `006-beacon-manager`

**Created**: 2026-07-11

**Status**: Draft

**Input**: User description: "Beacon Manager: manage the lifecycle of the embedded PixelBeacon companion addon on the user's machine. Discover the ESO AddOns directory automatically (Windows Documents known-folder API then Elder Scrolls Online\live\AddOns; Linux Proton via libraryfolders.vdf and Steam app id 306130), with a manual path override and selectable live/pts environment. Install (write embedded PixelBeacon.txt manifest plus PixelBeacon.lua into AddOns/PixelBeacon/, install-over-existing is a safe update, manifest carries the managed-marker line and an embedded addon version). Verify installed status (folder exists, marker line present, installed version equals embedded version). Uninstall (delete the PixelBeacon folder only when the managed-marker line is verified present in its manifest; never delete an unmanaged folder; discovery never writes outside the resolved AddOns directory). If the game is running during install or uninstall, surface a reminder that /reloadui or relog is required. Draws from master specification sections 9.4 and 9.5; depends on feature 001 foundations."

## Clarifications

### Session 2026-07-11

Resolved under the Build-Phase Autopilot Protocol from the master specification
(sections 9.4, 9.5) and the constitution (no options were escalated).

- Q: What files are installed, and where do their contents come from? -> A: The
  two addon files (`PixelBeacon.txt` manifest and `PixelBeacon.lua`) authored in
  the prior PixelBeacon slice are embedded in the application binary and written
  verbatim into `AddOns/PixelBeacon/`. There is no download step; the application
  ships the canonical copy.
- Q: What defines a "managed" beacon folder? -> A: The exact marker line
  `## X-ESO-Weave-Managed: true` present anywhere in the installed
  `PixelBeacon.txt` manifest. Uninstall deletes the folder only when this line is
  verified present in the manifest that is actually on disk; any other folder
  (missing manifest, missing marker) is treated as unmanaged and never deleted.
- Q: What are the four installed states the manager reports? -> A: NotInstalled
  (no folder or no manifest), Managed-UpToDate (folder + marker + installed
  version equals embedded version), Managed-VersionMismatch (folder + marker but
  installed version differs from embedded version), and Unmanaged (folder exists
  but the marker line is absent). Install is offered for the first three; the
  managed-only uninstall is offered for the two Managed states; Unmanaged is
  never auto-deleted.
- Q: How is "the game is running" determined, and what happens if it cannot be
  determined? -> A: A best-effort process check for the ESO client. Install and
  uninstall always proceed regardless; the running check only decides whether the
  reload-required reminder is surfaced. If the check is inconclusive the reminder
  is surfaced (fail safe toward reminding).
- Q: What happens when the AddOns directory cannot be resolved? -> A: Discovery
  returns a typed not-found result rather than guessing a literal path; install,
  verify, and uninstall are unavailable until the user supplies a manual override
  path in settings. A resolved path (auto or manual) is confirmed to be an
  existing directory before any write, and all writes are confined to the
  resolved `AddOns/PixelBeacon/` subtree.
- Q: Which manifest field is the authoritative addon version for verify? -> A:
  The `## Version:` line of `PixelBeacon.txt`. The embedded version is
  single-sourced from the embedded manifest, and verify parses the `## Version:`
  value from the on-disk manifest and compares it for equality with the embedded
  value; because install writes the embedded manifest verbatim, a fresh managed
  install is always up to date, and a mismatch means an older application version
  wrote the on-disk copy.
- Q: Are lifecycle operations recorded through logging? -> A: Yes. Discovery
  outcome, install, verify, and uninstall each emit a structured log record
  (success at info; not-found, not-writable, and unmanaged-refused at warn or
  error) through the existing logging sink, so failures and the safety-critical
  uninstall refusal are observable. No file contents are logged.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Install the beacon addon (Priority: P1)

A player who wants the fishing feature installs the PixelBeacon addon from the
application without leaving it or hand-copying files. The manager finds the ESO
AddOns directory automatically, writes the addon's two files into a
`PixelBeacon` subfolder, and confirms the addon is installed and up to date.

**Why this priority**: Installation is the entry point to the entire fishing
vertical; without a correctly installed beacon there is nothing for the reader to
sample. It is the minimum viable slice: discover plus install plus verify.

**Independent Test**: Point discovery at a temporary AddOns directory, run
install, and confirm `PixelBeacon/PixelBeacon.txt` and `PixelBeacon/PixelBeacon.lua`
exist with the expected embedded contents, the marker line, and the embedded
version, and that verify then reports Managed-UpToDate.

**Acceptance Scenarios**:

1. **Given** a resolved AddOns directory with no `PixelBeacon` folder, **When**
   the user installs, **Then** the manager creates `PixelBeacon/` containing the
   embedded manifest and Lua file, the manifest carries the managed-marker line
   and the embedded version, and verify reports Managed-UpToDate.
2. **Given** an existing managed `PixelBeacon` folder at an older version, **When**
   the user installs, **Then** the files are overwritten in place with the
   embedded copies (a safe update) and verify reports Managed-UpToDate.
3. **Given** the AddOns directory cannot be resolved, **When** the user attempts
   to install, **Then** the manager reports the directory was not found and takes
   no action, and installation becomes available once a valid manual override
   path is set.

---

### User Story 2 - Verify installed status (Priority: P1)

A player opens the application and sees at a glance whether the beacon is
installed, whether it is the version this application ships, and whether it is a
copy the application manages, so they know if action is needed.

**Why this priority**: Status is read on every launch and gates the install and
uninstall actions; the lifecycle is meaningless without a trustworthy status
read. It is P1 alongside install because install depends on it.

**Independent Test**: Construct AddOns directories representing each state
(absent, managed-current, managed-old, unmanaged) and confirm the manager reports
NotInstalled, Managed-UpToDate, Managed-VersionMismatch, and Unmanaged
respectively.

**Acceptance Scenarios**:

1. **Given** no `PixelBeacon` folder (or a folder with no manifest), **When**
   status is read, **Then** the manager reports NotInstalled.
2. **Given** a `PixelBeacon` folder whose manifest has the marker line and a
   version equal to the embedded version, **When** status is read, **Then** the
   manager reports Managed-UpToDate.
3. **Given** a `PixelBeacon` folder whose manifest has the marker line but a
   version different from the embedded version, **When** status is read, **Then**
   the manager reports Managed-VersionMismatch.
4. **Given** a `PixelBeacon` folder whose manifest lacks the marker line, **When**
   status is read, **Then** the manager reports Unmanaged.

---

### User Story 3 - Uninstall only what we manage (Priority: P1)

A player removes the beacon addon through the application. The manager deletes the
`PixelBeacon` folder only when it can confirm the folder is one the application
installed, and refuses to delete a folder the user placed there themselves.

**Why this priority**: This is the safety-critical surface of the feature.
Deleting an unmanaged folder would destroy a user's own work; the managed-marker
gate is a non-negotiable guardrail and must be proven by tests.

**Independent Test**: Attempt uninstall against a managed folder and confirm it is
removed; attempt uninstall against a folder without the marker line and confirm it
is left untouched and an unmanaged result is reported.

**Acceptance Scenarios**:

1. **Given** a managed `PixelBeacon` folder (marker line verified present in the
   on-disk manifest), **When** the user uninstalls, **Then** the entire
   `PixelBeacon` folder is removed and verify then reports NotInstalled.
2. **Given** a `PixelBeacon` folder whose manifest lacks the marker line, **When**
   the user uninstalls, **Then** nothing is deleted and the manager reports the
   folder is unmanaged.
3. **Given** a `PixelBeacon` folder with no manifest at all, **When** the user
   uninstalls, **Then** nothing is deleted and the manager reports the folder is
   unmanaged.

---

### User Story 4 - Reload reminder while the game runs (Priority: P2)

A player who installs or uninstalls the beacon while ESO is open is reminded that
the change will not take effect until they run `/reloadui` or relog, so they are
not confused by the beacon appearing to do nothing.

**Why this priority**: It prevents a confusing but non-destructive failure mode.
The lifecycle operations themselves are correct without it, so it ranks below the
core P1 operations.

**Independent Test**: With the running-game check forced to "running", perform an
install and an uninstall and confirm each result carries the reload-required
reminder; with the check forced to "not running", confirm the reminder is absent.

**Acceptance Scenarios**:

1. **Given** the ESO client is detected as running, **When** the user installs or
   uninstalls, **Then** the operation completes and its result includes a reminder
   that `/reloadui` or a relog is required.
2. **Given** the ESO client is not detected as running, **When** the user installs
   or uninstalls, **Then** the operation completes with no reload reminder.
3. **Given** the running state cannot be determined, **When** the user installs or
   uninstalls, **Then** the reminder is surfaced (fail safe toward reminding).

---

### Edge Cases

- What happens when the Documents known folder has been relocated by the user?
  Discovery resolves it through the platform known-folder API rather than assuming
  a literal path, so a relocated Documents folder still resolves correctly.
- What happens when multiple Steam libraries exist on Linux? Discovery parses the
  library list and locates the library that actually contains the ESO app id
  before resolving the compatdata AddOns path.
- What happens when the resolved path exists but is not writable? Install reports
  a failure result and writes nothing partial; status and uninstall behavior are
  unaffected by a failed write.
- What happens when the environment is switched between `live` and `pts`? The
  resolved AddOns directory changes accordingly, and status, install, and
  uninstall all operate against the selected environment's directory.
- What happens when the on-disk manifest is present but malformed (no readable
  version field)? Status treats a marker-bearing folder with an unreadable version
  as Managed-VersionMismatch (managed but not confirmed current), never as
  Unmanaged, so the managed uninstall remains available and no user folder is at
  risk.
- What happens when a manual override path is set to a location outside any AddOns
  directory? The path is used as the AddOns root as given, but all writes remain
  confined to the `PixelBeacon` subfolder beneath it; the manager never writes
  outside the resolved root.

## Requirements *(mandatory)*

### Functional Requirements

Discovery:

- **FR-001**: The manager MUST resolve the ESO AddOns directory without user input
  in the common cases: on Windows via the Documents known-folder platform API then
  `Elder Scrolls Online/<env>/AddOns`; on Linux (Proton) by parsing the Steam
  `libraryfolders.vdf`, locating app id `306130`, and resolving that library's
  `steamapps/compatdata/306130/pfx/drive_c/users/steamuser/Documents/Elder Scrolls Online/<env>/AddOns`.
- **FR-002**: The manager MUST NOT assume a literal Documents path; a relocated
  Documents folder MUST still resolve via the known-folder API.
- **FR-003**: The manager MUST support a selectable game environment, defaulting to
  `live`, with `pts` selectable in settings, and MUST resolve the AddOns directory
  under the selected environment.
- **FR-004**: The manager MUST accept a manual AddOns path override from settings
  that takes precedence over auto-discovery when set.
- **FR-005**: When the AddOns directory cannot be resolved (auto-discovery fails
  and no override is set), the manager MUST return a typed not-found result and
  MUST NOT guess or create a literal path.

Install:

- **FR-006**: Install MUST write the embedded addon files (`PixelBeacon.txt`
  manifest and `PixelBeacon.lua`) into a `PixelBeacon` subfolder of the resolved
  AddOns directory, creating the subfolder if absent.
- **FR-007**: Installing over an existing `PixelBeacon` folder MUST be a safe
  update: the embedded files overwrite the existing ones in place and the result
  is an up-to-date managed install regardless of the prior contents.
- **FR-008**: The installed manifest MUST contain the managed-marker line
  `## X-ESO-Weave-Managed: true` and a `## Version:` line equal to the
  application's embedded addon version.
- **FR-009**: All install writes MUST be confined to the `PixelBeacon` subtree of
  the resolved AddOns directory; the manager MUST NOT write outside that subtree.
- **FR-010**: The manager MUST confirm the resolved AddOns directory is an existing
  directory before performing any write; a failed or incomplete write MUST be
  reported as a failure result.

Verify:

- **FR-011**: The manager MUST report installed status as exactly one of:
  NotInstalled, Managed-UpToDate, Managed-VersionMismatch, or Unmanaged.
- **FR-012**: Managed-UpToDate MUST require all of: the `PixelBeacon` folder
  exists, the manifest contains the managed-marker line, and the manifest's
  `## Version:` value equals the embedded addon version.
- **FR-013**: A `PixelBeacon` folder whose manifest lacks the managed-marker line
  MUST be reported as Unmanaged.
- **FR-014**: Absence of the `PixelBeacon` folder, or a folder with no readable
  manifest, MUST be reported as NotInstalled.

Uninstall:

- **FR-015**: Uninstall MUST delete the `PixelBeacon` folder if and only if the
  managed-marker line is verified present in the manifest that is actually on disk.
- **FR-016**: Uninstall MUST NOT delete a folder whose manifest lacks the
  managed-marker line, or a folder with no manifest; it MUST report the folder as
  unmanaged and leave it untouched.
- **FR-017**: Uninstall MUST NOT delete anything outside the resolved
  `PixelBeacon` folder.

Reload reminder:

- **FR-018**: When the ESO client is detected as running (or the running state
  cannot be determined) during install or uninstall, the operation's result MUST
  include a reminder that a `/reloadui` or relog is required; when it is detected
  as not running, the result MUST NOT include the reminder.
- **FR-019**: The running-game check MUST NOT block or fail the lifecycle
  operation; install and uninstall MUST proceed regardless of the check outcome.

Testability:

- **FR-020**: Discovery, status classification, install, and uninstall MUST be
  testable against an injectable AddOns root (for example a temporary directory)
  and an injectable running-game check, independent of the real platform
  known-folder API and the real ESO process.

Observability:

- **FR-021**: Discovery, install, verify, and uninstall MUST each emit a
  structured log record through the application's logging sink (success at info;
  not-found, not-writable, and the unmanaged-uninstall refusal at warn or error).
  No addon file contents are logged.

### Key Entities *(include if feature involves data)*

- **AddOns Location**: The resolved AddOns directory for the selected environment,
  the selected environment (`live` or `pts`), and whether it came from
  auto-discovery or a manual override; may be unresolved (not found).
- **Beacon Status**: One of NotInstalled, Managed-UpToDate,
  Managed-VersionMismatch, or Unmanaged, derived from the on-disk folder,
  manifest marker line, and installed version.
- **Embedded Addon**: The canonical manifest and Lua file contents shipped in the
  application, plus the embedded addon version (the manifest's `## Version:`
  value) that install writes and verify compares against.
- **Lifecycle Result**: The outcome of an install or uninstall (success or a typed
  failure such as not-found, not-writable, or unmanaged-refused), plus whether the
  reload-required reminder applies.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: On a machine with a standard ESO install, the AddOns directory
  resolves automatically with no user input in 100 percent of the common-case
  configurations described in FR-001 (relocated Documents included).
- **SC-002**: After install, the `PixelBeacon` folder contains exactly the two
  embedded files with the marker line and embedded version, and verify reports
  Managed-UpToDate, in 100 percent of runs against a writable AddOns root.
- **SC-003**: Each of the four status inputs (absent, managed-current,
  managed-old, unmanaged) yields exactly its defined status, for 100 percent of
  cases.
- **SC-004**: Uninstall removes a managed folder and refuses every unmanaged
  folder, with zero deletions of a folder lacking the verified marker line, across
  all test cases.
- **SC-005**: No write or delete ever occurs outside the resolved `PixelBeacon`
  subtree, verifiable across all install and uninstall test cases.
- **SC-006**: The reload-required reminder appears for every install or uninstall
  performed while the game is running (or indeterminate) and for none performed
  while it is not running.

## Assumptions

- Scope is master specification sections 9.4 (discovery) and 9.5 (lifecycle). The
  addon file contents, the pixel-bus protocol, the reader, and the fishing
  controller are out of scope; this slice only places, verifies, and removes the
  addon files.
- The embedded addon files are the canonical `PixelBeacon.txt` and
  `PixelBeacon.lua` produced by feature 004; they are compiled into the
  application. The embedded addon version is single-sourced with those files.
- Feature 001 foundations (config store and logging) exist and provide the
  settings surface for the manual path override and environment selection and the
  logging sink for lifecycle events.
- The Windows Documents known-folder resolution and the Linux Steam library and
  compatdata resolution are thin platform seams validated on real hardware; the
  status classification and the marker-gated uninstall are pure logic and fully
  unit-tested against a temporary AddOns root.
- The running-game detection is best-effort and advisory only; it affects only
  whether the reload reminder is shown, never whether an operation proceeds.
- Actually performing install or uninstall from the graphical interface, and
  surfacing the reminder to the user, are wired in the later GUI slice; this slice
  exposes the manager operations and their typed results.
