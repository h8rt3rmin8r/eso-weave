# Feature Specification: ESO API Version Check Automation

**Feature Branch**: `018-eso-api-version-check`

**Created**: 2026-07-12

**Status**: Draft

**Input**: User description: "ESO API Version Check automation. Build a background automation that keeps the PixelBeacon companion addon manifest API version current. On app startup, off the GUI thread, the app issues a web request to a stable public source and parses the current official ESO client API version. It stores the last known API version and reuses it. When a newer version is discovered it updates the stored value. This feeds the addon install and uninstall logic and keeps the on-disk manifest current: if the addon is installed, edit the in-place manifest APIVersion line when it differs (only when the managed marker is present); if the addon is not installed, discover and store the version to use when the user later runs the install. A global default API version constant is compiled in so the app always has a version to fill the required APIVersion field even with no network and no stored value. Resolution order is fresh fetch, then stored value, then compiled default. The automation never blocks startup and never panics on network failure. Feature directory specs/018-eso-api-version-check."

## Clarifications

### Session 2026-07-12

- Q: The manifest API version field may list more than one value (a live value
  plus a maintainer-added future or PTS value). When the app rewrites the field,
  what does it write? → A: Set the resolved live value as the primary token and
  preserve any existing tokens on the line that are newer than the primary; drop
  any tokens older than the primary. This advances the live value, keeps a
  deliberately added forward-compatible value, and never lists stale versions.
- Q: Is the last known API version stored with user settings or with session
  state? → A: With session state, because it is derived runtime data rather than a
  user preference.
- Q: Does the app ever lower the on-disk API version to an older value? → A: No.
  The app never downgrades; it only advances the primary value or leaves it
  unchanged.
- Q: What does the web request actually fetch, given that the exact numeric API
  version is only published behind bot challenges that a plain client cannot
  pass? → A: The request fetches the current live game client version string from
  a bot-friendly source (the official esoui/esoui GitHub live branch), which
  changes exactly when the client API changes and is used as the bump-detection
  signal. The numeric API version written into the manifest is resolved from the
  stored last known value, then the compiled default; the network fetch detects
  that a bump occurred rather than supplying the number directly.
- Q: What happens when a brand-new game version is detected that this build of the
  app does not yet have a numeric API version for? → A: The app keeps writing the
  last known good numeric value (never a guessed or wrong one), records the newly
  seen game version, and surfaces a notice that the app should be updated to carry
  the new addon API version.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Installed addon stays API-current across patches (Priority: P1)

A player has the PixelBeacon addon installed. The next time the player launches
ESO Weave, the app ensures the on-disk addon manifest declares the best known
API version, so ESO loads the addon instead of flagging it Out of Date and
fishing keeps working without the player editing any files. When the app detects
that the game has shipped a newer client version than this build knows about, it
says so plainly so the player knows to update ESO Weave.

**Why this priority**: This is the whole point of the feature. A stale API
version is the single most common reason the addon fails to load, which silently
breaks fishing. Automating this upkeep removes the manual patch-day chore that
today is the only thing standing between a game patch and broken fishing.

**Independent Test**: With the addon installed, set the resolved best known API
version newer than the one on disk and start the app; confirm the on-disk manifest
API version line is rewritten to the new value while every other line, including
the managed marker, is preserved.

**Acceptance Scenarios**:

1. **Given** the addon is installed with the managed marker and the resolved best
   known API version is newer than the manifest value, **When** the app starts,
   **Then** the manifest API version line is updated to the resolved value and all
   other manifest content is unchanged.
2. **Given** the addon is installed and the resolved best known API version matches
   the manifest value, **When** the app starts, **Then** the manifest is left byte
   for byte unchanged.
3. **Given** a manifest at the addon location that does not carry the managed
   marker, **When** a newer API version would be written, **Then** the app does not
   write to that manifest.
4. **Given** the addon is installed and the version check detects a game client
   version newer than this build knows about, **When** the app starts, **Then** the
   app records the newly seen game version and surfaces a notice to update ESO
   Weave, and still leaves the manifest at the last known good API version.

### User Story 2 - Correct version is ready before install (Priority: P2)

A player has not yet installed the addon. ESO Weave still runs the background
version check and keeps its stored state current, so that whenever the player
later runs the install, the addon is written with the best known API version and
is not Out of Date the first time it loads.

**Why this priority**: A first install that is immediately Out of Date reproduces
the exact failure this feature exists to prevent. Keeping stored state current
ahead of install closes that gap, but it depends on the discovery and storage that
Story 1 already establishes, so it is second.

**Independent Test**: With the addon not installed, start the app so the check
runs and stored state updates, then run the install and confirm the installed
manifest carries the resolved best known version.

**Acceptance Scenarios**:

1. **Given** the addon is not installed and the app has a stored API version,
   **When** the player runs the install, **Then** the installed manifest declares
   the stored version.
2. **Given** the addon is not installed and no version has ever been stored,
   **When** the player runs the install, **Then** the installed manifest declares
   the compiled default version.

### User Story 3 - A version is always available (Priority: P3)

A player launches ESO Weave with no internet connection and no prior stored
version. The app still functions and any manifest it writes carries a sensible
compiled-in default API version rather than an empty or invalid field.

**Why this priority**: It is a safety net for the offline first-run case. It
matters for correctness but is the least common path, and the compiled default is
a static fallback rather than the feature's main behavior.

**Independent Test**: With no network and no stored value, render a manifest and
confirm the API version field is populated with the compiled default.

**Acceptance Scenarios**:

1. **Given** no network access and no stored version, **When** the app renders a
   manifest, **Then** the API version field is the compiled default value.
2. **Given** the version source is unreachable, **When** the app starts, **Then**
   startup completes normally with no error surfaced to the player and no crash.

### Edge Cases

- The version source is reachable but returns unexpected or malformed content:
  the parse yields no result and the app proceeds with the stored value, then the
  default, without error.
- The resolved best known version is older than the value already on disk: the app
  does not downgrade the manifest.
- The resolved best known version equals the on-disk value: no manifest write and
  no redundant disk churn.
- The addon directory cannot be resolved (game not installed, path not set): the
  check still updates stored state for later use and surfaces no error.
- The manifest file exists but is unreadable: the app does not delete or replace
  it and treats it as unmanaged for write purposes.
- The network call is slow: it runs off the GUI thread and never delays the window
  appearing or responding.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The app MUST attempt, on startup and off the interface thread, to
  fetch the current live ESO game client version string from a stable bot-friendly
  public source, as the signal that a client API change has occurred.
- **FR-002**: The app MUST parse the fetched game version reliably and treat any
  unparseable or challenged response as no result.
- **FR-003**: The app MUST persist, as derived runtime state, both the last known
  numeric API version and the last seen game version, and MUST update the stored
  game version whenever a newer one is discovered.
- **FR-004**: The app MUST carry a compiled-in default numeric API version so a
  manifest can always be produced with no network access and no stored value.
- **FR-005**: When a numeric API version is required to render or update a manifest,
  the app MUST resolve it in the order: stored last known value, then compiled
  default. The network fetch detects a bump; it does not supply the numeric value.
- **FR-005a**: When the fetched game version is newer than the version this build
  and stored state know about, the app MUST record the newly seen game version and
  MUST surface a plain-language notice that ESO Weave should be updated to carry
  the new addon API version. The app MUST NOT guess a numeric API version for an
  unknown game version.
- **FR-006**: When the addon is installed, the app MUST update the on-disk manifest
  API version field to the resolved value whenever it differs, and MUST leave the
  manifest unchanged when it already matches.
- **FR-007**: The app MUST NOT write to a manifest that does not carry the managed
  marker; the managed marker gate governs every manifest write, not only uninstall.
- **FR-008**: A manifest update MUST change only the API version field and MUST
  preserve every other line of the manifest, including the managed marker.
- **FR-008a**: When rewriting the API version field, the app MUST set the resolved
  live value as the primary token, MUST preserve any existing tokens on the field
  that are newer than the primary, and MUST drop any tokens older than the primary.
- **FR-009**: When the addon is not installed, the app MUST still run the check and
  update stored state so a later install uses the resolved best known value.
- **FR-010**: The install path MUST write the manifest with the resolved API
  version.
- **FR-011**: The app MUST NOT downgrade an on-disk manifest to an older API
  version than it already declares.
- **FR-012**: The startup version check MUST never block the interface from
  appearing or responding, and MUST never crash the app on network or parse
  failure.

### Key Entities *(include if feature involves data)*

- **Numeric API version**: The official ESO client API version number that an
  addon manifest must declare to load without an Out of Date flag.
- **Game version signal**: The live game client version string fetched over the
  network; it changes when the client API changes and is used to detect bumps.
- **Last known numeric API version**: The most recent numeric value, remembered
  between runs as derived runtime state (not user settings).
- **Last seen game version**: The most recent game version string observed,
  remembered between runs as derived runtime state.
- **Compiled default API version**: A static numeric fallback built into the app.
- **Manifest API version field**: The API version declaration inside the on-disk
  PixelBeacon manifest that this feature keeps current.
- **Managed marker**: The manifest line that identifies the manifest as owned by
  ESO Weave and gates every write and delete.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: After an ESO patch that raises the API version and a corresponding
  ESO Weave update, a player who launches the app once has an addon that ESO loads
  without an Out of Date flag, with zero manual file edits; between the patch and
  that update, the app detects the bump and tells the player to update.
- **SC-002**: A first install performed while online produces an addon that is not
  Out of Date on its first load.
- **SC-003**: The app starts and reaches a responsive window with no network
  access, and any manifest it writes has a valid, populated API version field.
- **SC-004**: A manifest that ESO Weave does not own is never modified by the
  version check.
- **SC-005**: The interface appears and responds without waiting on the version
  check, regardless of network latency.

## Assumptions

- The exact numeric API version is only published behind bot challenges that a
  plain client cannot pass; a bot-friendly source (the official esoui/esoui GitHub
  live branch) reliably reports the live game version string over HTTPS and is used
  as the bump-detection signal. The specific endpoint is fixed during planning.
- The last known numeric API version and last seen game version are derived runtime
  state, stored with session state, not user settings, per the project's
  configuration separation.
- The compiled default numeric API version is seeded to the current live value at
  build time and refreshed as part of normal release upkeep; the detection signal
  tells maintainers and players when that refresh is due.
- The addon manifest already carries the managed marker used by the existing
  install and uninstall logic; this feature reuses that marker as its write gate.
- The version check runs once per startup; continuous polling is out of scope.
- Only the live client API version is tracked; PTS or future-version handling
  beyond preserving values already present is out of scope.
