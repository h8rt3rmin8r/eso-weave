# Feature Specification: Fishing Reliability and Status Collaboration

**Feature Branch**: `016-fishing-reliability`

**Created**: 2026-07-12

**Status**: Draft

**Input**: User description: "Fishing reliability and status collaboration (build plan 004, slice 016). Fix the fishing feature so a cast at a fishing hole reliably progresses to a catch and reel-in, and make the app communicate what it is doing and why it stopped."

## Clarifications

### Session 2026-07-12

Resolved under autopilot from the master specification, the constitution, and the
existing code seams; none rose to a critical, irreversible, or intent-level
ambiguity requiring an operator decision.

- Q: What provisional values should the tuned fishing timeouts take? -> A: Arm
  timeout raised to 8000 ms for margin, reel delay kept at 100 ms, recast delay
  kept at 3000 ms; all remain configurable and are provisional pending in-game
  validation.
- Q: How long should the stop reason remain visible after the routine returns to
  idle? -> A: It persists in the idle status until the next time fishing is
  started, so the player can read why the last session ended.
- Q: Which future API version should the manifest declare alongside the current
  live value? -> A: The two-value form 101050 101054 (current live plus about
  four updates of runway), with the current live value confirmed against the
  client at implementation time.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Reliable hands-free fishing (Priority: P1)

A player stands at a fishing hole with the in-game interact prompt visible and
turns on the fishing routine. The app casts the line for them, waits for the
bite, reels in the catch, and recasts, repeating for as long as the routine is
on. The routine does not falsely stop within a few seconds of starting, and a
bite that lands is reeled in while the catch is still on the line.

**Why this priority**: This is the reported defect and the core value of the
feature. Without it the fishing routine does not work: the status flips back to
idle within seconds and caught fish are never collected.

**Independent Test**: With the companion addon loaded, stand at a fishing hole,
turn on fishing, and observe that the routine advances past the initial casting
state into actively waiting for a bite, that a bite is reeled in and the catch
collected, and that the routine then recasts on its own.

**Acceptance Scenarios**:

1. **Given** the companion addon is loaded and the player is aimed at a fishing
   hole, **When** the player turns on fishing, **Then** the routine casts and
   advances from casting to waiting for a bite rather than reverting to idle.
2. **Given** the routine is waiting for a bite, **When** a fish bites, **Then**
   the routine reels in while the catch is still on the line and the catch is
   collected.
3. **Given** the routine has just reeled in a catch, **When** the recast delay
   elapses, **Then** the routine casts again without the player pressing
   anything.

---

### User Story 2 - Companion addon recognized as current by the game (Priority: P1)

A player opens the in-game AddOns list and sees the companion addon listed as
current rather than flagged out of date, so the game loads it without the player
having to enable loading of out-of-date addons.

**Why this priority**: If the game flags the addon as out of date and the player
has not enabled out-of-date addons, the game does not load it at all. No signal
is rendered, so the app can never detect a cast or a bite. This is a co-primary
cause of the reported failure and a prerequisite for User Story 1 to work for
most players.

**Independent Test**: On the current live game client, install or refresh the
addon through the app, reload the game UI, and confirm the addon is not flagged
out of date in the AddOns list.

**Acceptance Scenarios**:

1. **Given** the current live game client, **When** the addon is installed by the
   app and the UI is reloaded, **Then** the AddOns list shows the addon as
   current, not out of date.
2. **Given** a player who already had the previous addon version installed,
   **When** the app runs, **Then** the app detects the on-disk copy as outdated
   and refreshes it to the current version.

---

### User Story 3 - Clear fishing status and stop reasons (Priority: P2)

A player watches the app during a fishing session and understands, in plain
language, what the routine is doing at each moment, and when it stops the app
tells them why (they stopped it, no cast was detected, or the beacon signal was
lost), so they can correct the setup without guessing.

**Why this priority**: The feedback noted the app does not collaborate with the
player. Even with the mechanics fixed, an unexplained state is hard to trust and
hard to troubleshoot. This turns a silent failure into an actionable message.

**Independent Test**: Drive the routine through each state and each stop path and
confirm the status text reads as plain language and that a stop names its reason.

**Acceptance Scenarios**:

1. **Given** the routine is running, **When** it is in each of its working
   states, **Then** the status reads in plain language (for example Casting,
   Fishing (waiting for a bite), Reeling in, Recasting) rather than an internal
   state name.
2. **Given** the routine armed a cast but no cast was confirmed within the arm
   window, **When** it returns to idle, **Then** the status explains that no cast
   was detected rather than silently showing idle.
3. **Given** the beacon signal is lost mid-session, **When** the routine stops,
   **Then** the status explains that the signal was lost.
4. **Given** the player turns fishing off, **When** the routine returns to idle,
   **Then** the status reflects that the player stopped it, not an error.

---

### Edge Cases

- The player presses the fishing hotkey while not aimed at a hole: no cast
  confirmation arrives, and after the arm window the routine stops and reports
  that no cast was detected.
- The beacon signal is lost mid-session (the player alt-tabs, disables the addon,
  or obstructs the beacon): the routine stops and reports signal lost, and it
  does not blind-fire the interact key.
- The player rapidly toggles fishing on and off: the routine settles to the last
  requested state without leaving a pending action stranded.
- The player still has the previous addon version installed: the app refreshes
  the addon files, and the player must reload the UI or relog for the game to
  load the new files.
- A game update raises the live API version beyond the addon's first declared
  value: the second declared value keeps the addon current until that version is
  passed.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The companion addon manifest MUST declare an API version that the
  current live game client accepts as current, so the game loads the addon
  without the player enabling out-of-date addons. The manifest MUST use the
  game's supported multi-value form to also declare a future version, extending
  how long the addon stays current across future game updates.
- **FR-002**: The addon package version MUST be raised so that the app classifies
  a previously installed copy as outdated and refreshes it to the current
  version, delivering the updated manifest to existing installs.
- **FR-003**: The addon change MUST preserve the managed-marker line that gates
  safe uninstall, so that uninstall still deletes the addon folder only after
  verifying that marker.
- **FR-004**: While the fishing routine is active, the system MUST sample the
  beacon signal and advance the routine at the fast fishing cadence rather than
  the slow idle cadence, so that transient cast and bite signals are observed in
  time and the reel action is not delayed.
- **FR-005**: The system MUST evaluate fishing timers against the same time
  source on which they are set, so that a routine started from the app does not
  suffer clock skew between when a deadline is set and when it is judged.
- **FR-006**: The default fishing timeouts MUST be set to conservative values
  that give the arm window adequate margin and keep the reel and recast timing
  within the game's reel window. The provisional defaults are an arm timeout of
  8000 ms, a reel delay of 100 ms, and a recast delay of 3000 ms. These defaults
  MUST remain configurable and are provisional pending in-game validation.
- **FR-007**: The fishing status MUST present each working state in plain
  language rather than an internal state name.
- **FR-008**: When the routine returns to idle, the system MUST record and
  surface the reason: the player stopped it, no cast was detected within the arm
  window, or the beacon signal was lost. The reason MUST persist in the idle
  status until fishing is next started.
- **FR-009**: The system MUST continue to honor its safety behaviors: no blocking
  work on the input hook thread, input suppression scoped to the focused game
  window only, and cancellation of any pending interact when the beacon signal is
  lost rather than firing it blindly.
- **FR-010**: The correctness logic (cadence selection, status and reason
  derivation, and the fishing state machine) MUST live in tested units that do
  not depend on the graphical layer.

### Key Entities *(include if feature involves data)*

- **Fishing routine state**: the current phase of the routine (idle, casting,
  waiting for a bite, reeling in, recasting) plus, when idle, the reason it last
  stopped.
- **Companion addon manifest**: the metadata that declares the addon package
  version and the game API versions it supports, and that carries the managed
  marker.
- **Poll cadence**: the interval at which the app samples the beacon and advances
  the routine, which differs between an active fishing session and an idle app.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In an in-game validation session of at least 10 casts with the
  addon loaded and the player aimed at a hole, at least 9 casts advance past the
  initial casting state into waiting for a bite instead of reverting to idle.
- **SC-002**: A bite that occurs during a session is reeled in and the catch
  collected in at least 9 of 10 catches, with the reel action landing within the
  game's reel window.
- **SC-003**: On the current live game client, the companion addon appears as
  current (not out of date) in the AddOns list after the app installs or refreshes
  it and the UI is reloaded.
- **SC-004**: Every early stop of the routine displays a reason (player stopped,
  no cast detected, or signal lost); no stop leaves the player with an
  unexplained idle status.
- **SC-005**: A player unfamiliar with the internals can read the fishing status
  and correctly describe what the routine is doing at each phase, with no
  internal state names shown.

## Assumptions

- The current live game API version at implementation time is 101050 (game
  Update 50); this is confirmed against the live client before the manifest value
  is finalized.
- The companion addon renders the fishing signal correctly; exact in-game
  validation of the signal and of the tuned timing defaults is still owed and is
  tracked as owed manual validation, consistent with prior open items.
- The player runs the game in a windowed or borderless mode so the beacon strip
  is captured, consistent with the existing capture assumptions.
- The fishing hotkey itself casts the line; the player does not cast manually
  before turning the routine on. This interaction model is unchanged by this
  feature and is documented separately in the usage documentation slice.
- Delivering the refreshed addon to an existing install requires the player to
  reload the game UI or relog after the app refreshes the files.
