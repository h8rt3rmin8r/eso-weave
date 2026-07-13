# Feature Specification: Fishing Interaction Detection Rewrite

**Feature Branch**: `025-fishing-interaction-detection`

**Created**: 2026-07-13

**Status**: Draft

**Input**: User description: "Fishing interaction detection rewrite (slice 025 of build plan 007). Replace PixelBeacon's one-shot event-driven fishing detection with poll-authoritative detection mirroring the game's own reticle, and instrument the Rust fishing controller with transition logging."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - A cast is detected and the cycle runs (Priority: P1)

The operator stands at a fishing hole with bait selected, presses the fishing
toggle, and the application casts the line. Within moments the application's
status advances from Casting to Fishing because the companion addon now
reports the active fishing interaction. When a fish bites, the addon reports
the bite, the application reels in, waits, and recasts, looping the cycle
without further operator input.

**Why this priority**: This is the product's fishing feature working at all.
Four consecutive field sessions failed at exactly this point: the cast
happened in-game but the addon never reported it, so the application timed
out back to Idle before the first bite. Everything else in this feature is
in service of this journey.

**Independent Test**: With the updated addon installed and the game at a
fishing hole with bait selected, toggle fishing on and observe the status
advance Casting to Fishing within the arm window, then Reeling on a bite,
then a recast, for at least three consecutive catches.

**Acceptance Scenarios**:

1. **Given** the operator is at a fishing hole with bait selected and the
   beacon heartbeat is being read, **When** the fishing toggle is pressed and
   the cast lands in the water, **Then** the addon renders the waiting signal
   within one polling interval and the application advances from Casting to
   Fishing without reaching the arm timeout.
2. **Given** the application is in the Fishing state, **When** a fish takes
   the bait, **Then** the addon renders the bite signal, the application
   advances to Reeling, and the reel keypress is sent after the configured
   reel delay.
3. **Given** a catch has resolved and the recast delay has elapsed, **When**
   the application recasts, **Then** the addon reports the new cast the same
   way and the cycle continues.
4. **Given** fishing is active, **When** the operator toggles fishing off,
   **Then** the application stops immediately and no further keypresses are
   sent.

---

### User Story 2 - A failed session leaves diagnosable evidence (Priority: P2)

The operator runs a fishing session that fails for any reason and afterwards
opens the log. The log now narrates the fishing engine's behavior: when the
cast was sent, when the engine armed, which state transitions occurred, and
why fishing stopped, alongside the existing per-sample pixel diagnostics.

**Why this priority**: Four field failures produced zero fishing-engine log
lines, forcing whole diagnostic sprints to reconstruct what happened. The
detection rewrite in this slice is evidence-based; future failures must carry
their own evidence.

**Independent Test**: Run any fishing session (even against a stub or with
the game absent), then inspect the log for fishing-engine entries recording
each transition and the stop reason.

**Acceptance Scenarios**:

1. **Given** logging at debug level or lower, **When** the fishing toggle
   arms the engine and the arm timeout later expires with no cast detected,
   **Then** the log contains entries for the cast keypress, the armed state,
   and the disable with its stop reason, each attributable to the fishing
   engine.
2. **Given** logging at debug level or lower, **When** a full
   cast-bite-reel-recast cycle runs, **Then** each state transition appears
   in the log in order.

---

### User Story 3 - The operator receives the corrected addon (Priority: P3)

The operator opens the application, sees that the installed addon is out of
date, uses the existing update control, reloads the game interface, and is
running the corrected detection without manual file management.

**Why this priority**: The fix ships inside the embedded addon; it only
reaches the game through the existing install/update path. Without a version
advance the update control would not offer it.

**Independent Test**: With an older addon version installed, open the
application and confirm the update path replaces the installed addon with
the new version, preserving the managed-folder safety guarantee.

**Acceptance Scenarios**:

1. **Given** an installed addon at the previous version, **When** the
   operator activates the update control, **Then** the newer embedded addon
   replaces it, and the uninstall step still verifies the managed marker
   before deleting anything.

---

### Edge Cases

- The bite signal must never regress: once the addon reports a bite, routine
  polling must not demote it back to waiting; the bite clears only when the
  catch resolves (an item is gained), the safety timeout elapses, or the
  fishing interaction ends.
- If the operator opens a menu or dialog mid-cast, the fishing interaction
  ends in-game; the addon must return its signal to idle rather than holding
  a stale waiting state.
- If the operator has no bait selected, the game never starts the fishing
  interaction; the addon reports nothing, and the application times out to
  Idle with the no-cast-detected stop reason, now visible in the log.
- An inventory stack decrease unrelated to bait (using a potion, feeding a
  mount) while fishing must not be reported as a bite.
- Detection must not depend on the game's language setting.
- Signal loss (game window losing capture, addon unloading) must still
  degrade fishing to disabled, exactly as before.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The addon MUST determine the active-cast (waiting) condition by
  periodically sampling the game's authoritative interaction state, at an
  interval no coarser than 150 milliseconds, rather than relying on any
  one-shot event.
- **FR-002**: The addon MUST stop treating the game's interaction-failure
  alert event as a fishing signal; the current dependency on it MUST be
  removed entirely.
- **FR-003**: The addon MUST report a bite when the game's interact prompt
  for the active fishing interaction changes to the reel-in action, using a
  comparison that is correct in every game language.
- **FR-004**: The addon MUST retain bait consumption as a secondary bite
  signal, and MUST scope it to the equipped bait so that unrelated inventory
  stack decreases are never reported as bites.
- **FR-005**: A reported bite MUST NOT be demoted to waiting by routine
  polling; it clears only on catch resolution, the existing safety timeout,
  or the end of the fishing interaction.
- **FR-006**: When the fishing interaction ends for any reason, the addon
  MUST return the fishing signal to idle within one polling interval.
- **FR-007**: The rendered signal colors, block positions, and block
  geometry MUST remain byte-identical to the current contract; the
  application's decoder is unchanged.
- **FR-008**: The addon manifest version MUST advance so the application's
  existing update control offers the new addon, and the stale unused version
  constant in the addon source MUST be removed.
- **FR-009**: The fishing engine MUST log, at debug level under its own
  attributable component name, every state transition: cast keypress sent,
  armed, armed to waiting, bite to reeling, reel to recast, and every
  disable together with its stop reason.
- **FR-010**: The transition logging MUST NOT change any fishing engine
  behavior; all existing fishing and pixel-decode tests pass unchanged.
- **FR-011**: The master specification's fishing-detection language MUST be
  updated to describe the polling contract, and the change MUST be recorded
  as a dated decision in the changelog. The feature's research notes MUST
  record the official-source citations establishing the corrected game-API
  usage.
- **FR-012**: The safety invariants MUST hold unchanged: fishing degrades to
  disabled on signal loss, and addon uninstall deletes a folder only after
  verifying the managed marker.

### Key Entities

- **Fishing signal**: the addon's three-valued state (idle, waiting, bite)
  rendered as the second beacon block; the sole channel by which the
  application learns fishing progress.
- **Fishing interaction**: the game's own notion of an active fishing cast,
  which the addon now samples directly instead of inferring from events.
- **Stop reason**: the recorded cause when fishing disables (user stop, no
  cast detected, signal lost), now surfaced in the log.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: With bait selected at a fishing hole, a cast is reflected in
  the application's status within 2 seconds of the line entering the water,
  in 10 out of 10 consecutive casts.
- **SC-002**: A full unattended cycle (cast, bite, reel, recast) completes
  at least 3 consecutive times in a live session without the application
  reverting to Idle on the arm timeout.
- **SC-003**: After any fishing session ends, the log contains a complete,
  ordered narration of every fishing state transition and the final stop
  reason; a session that fails can be diagnosed from the log alone without
  reproducing it.
- **SC-004**: Zero false bites are reported during a session that includes
  at least one unrelated inventory stack decrease while a cast is waiting.
- **SC-005**: The existing automated test suite passes unchanged, and the
  application's decoder requires no modification.

## Assumptions

- The game's periodic interaction-state sampling is inexpensive enough to
  run at the chosen interval without measurable frame cost; the game's own
  interface performs the same check every frame.
- The reel-in prompt comparison is stable across game languages because both
  sides of the comparison come from the game's own localized string table.
- The bite-scoping category for bait consumption remains valid at the
  current game API version, as evidenced by a currently maintained
  third-party addon shipping the same mechanism.
- The operator applies the addon update and reloads the game interface
  before the next fishing session; in-game validation is owed before the
  next release, as with the prior slice.
- The persisted arm timeout of 5000 milliseconds remains sufficient because
  the waiting signal now lands within one polling interval of the cast
  (considered and rejected: migrating the persisted value to the newer 8000
  millisecond default).
