# Feature Specification: Fishing Bite Signal Correction

**Feature Branch**: `026-fishing-bite-signal`

**Created**: 2026-07-13

**Status**: Draft

**Input**: User description: "Fishing bite signal correction (slice 026 of build plan 008). Remove the false primary bite trigger slice 025 introduced and make the lure-scoped bait-consumption inventory event the sole bite signal in PixelBeacon."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - The app waits for a real bite (Priority: P1)

The operator casts at a fishing hole with bait selected. The application
detects the cast and then waits, doing nothing, for as long as it takes a
fish to actually take the bait. Only when the fish is genuinely hooked does
the application reel in, and it then recasts and repeats. No bait is spent
on anything except real catches.

**Why this priority**: The v0.6.1 field session reeled 100 milliseconds
after every cast because the standing "reel in" interact prompt was
misread as a bite, looping at roughly two casts per second and consuming
the operator's bait stock with nothing to show for it. The fishing feature
is worse than useless if a cast can never survive to a real bite.

**Independent Test**: With the updated addon, cast once at a hole and
observe the application hold the Fishing (waiting) state, without any reel
keypress, until the fish visibly strikes; then observe reel, catch, and
recast.

**Acceptance Scenarios**:

1. **Given** a cast is active and no fish has struck, **When** any amount
   of time passes (including the many seconds a slow bite takes), **Then**
   the application stays in the waiting state and sends no reel keypress.
2. **Given** a cast is active, **When** the fish takes the bait, **Then**
   the bite signal is reported, the application reels after the configured
   reel delay, and the catch resolves.
3. **Given** a catch has resolved and the recast delay elapses, **When**
   the application recasts, **Then** the next cycle proceeds identically,
   and across three or more consecutive catches exactly one bait is
   consumed per catch.
4. **Given** a cast is active, **When** the operator uses an unrelated
   consumable (a potion, food), **Then** no bite is reported and no reel
   keypress is sent.

---

### User Story 2 - The operator receives the corrected addon (Priority: P2)

The operator opens the application, sees the installed addon is out of
date, uses the existing update control, reloads the game interface, and is
running the corrected bite detection without manual file management.

**Why this priority**: The defect and its fix live entirely inside the
embedded addon; the fix only reaches the game through the version advance
and the existing update path.

**Independent Test**: With version 4 installed, the update control replaces
it with version 5, preserving the managed-folder safety guarantee.

**Acceptance Scenarios**:

1. **Given** an installed addon at version 4, **When** the operator
   activates the update control, **Then** version 5 replaces it and the
   uninstall step still verifies the managed marker before deleting.

---

### Edge Cases

- The interact prompt that reads "reel in" is shown for the entire time
  the line is in the water; it must never be interpreted as a bite.
- A bite that arrives extremely fast (between the waiting signal first
  rendering and the application's next signal sample) is still a valid
  bite; the application accepts it even if it never observed the waiting
  state.
- If the bite event never fires on a real strike (the residual unknown
  until the in-game run), the failure mode must be benign: the application
  keeps waiting, the fish escapes, the interaction ends, and the cycle
  recasts; no reel is sent and no bait is spent by the application, and
  the log shows casts with no bite line.
- An inventory stack decrease that is not the equipped bait (consumables,
  crafting, mount feed) while a cast is active must not be reported as a
  bite.
- Opening a menu mid-cast ends the interaction; the signal returns to idle
  as in the previous slice, unchanged.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The addon MUST NOT use the interact-prompt text (the standing
  "reel in" prompt) as a bite signal in any form; the prompt-comparison
  trigger introduced by slice 025 MUST be removed.
- **FR-002**: The sole bite signal MUST be the bait-consumption inventory
  event: a single-stack decrease carrying the lure item-sound category,
  while a cast is active and no menu is open (the mechanism already
  implemented and scoped in the current addon, unchanged).
- **FR-003**: The periodic fishing tick MUST retain only its cast-tracking
  duties: idle to waiting while the fishing interaction is active, waiting
  or bite back to idle when it ends, and never demoting a rendered bite.
- **FR-004**: The waiting state MUST be able to persist indefinitely (no
  timeout, no synthetic bite) until a real bite, the interaction ending, or
  the operator stopping.
- **FR-005**: The rendered signal colors, block positions, and geometry
  MUST remain byte-identical; the application decoder, controller behavior,
  and timings are unchanged.
- **FR-006**: The addon manifest version MUST advance (4 to 5) so the
  existing update control offers the corrected addon.
- **FR-007**: The master specification's fishing-detection contract MUST be
  corrected to the single bite signal and MUST document the reel-in prompt
  as the standing cast prompt rather than a bite indicator; the change MUST
  be recorded as a dated changelog decision correcting the slice 025
  decision, and the feature's research notes MUST record the field-log
  evidence and reference citations.
- **FR-008**: The safety invariants MUST hold unchanged: fishing degrades
  to disabled on signal loss, and addon uninstall deletes a folder only
  after verifying the managed marker.

### Key Entities

- **Fishing signal**: the addon's three-valued state (idle, waiting, bite);
  unchanged encoding, corrected semantics for entering bite.
- **Bite event**: the equipped bait leaving the inventory, the game's one
  reliable "fish hooked" observable, per both proven reference
  implementations.
- **Standing reel-in prompt**: the interact prompt shown for the whole
  cast; explicitly documented as not a bite indicator.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In a live session, a cast survives without any reel keypress
  until a real bite, on 10 out of 10 consecutive casts (no reel within the
  first seconds of any cast unless a fish genuinely struck).
- **SC-002**: At least 3 consecutive full cycles (cast, real bite, reel,
  catch, recast) complete unattended, with exactly one bait consumed per
  catch and zero bait consumed by application-initiated early reels.
- **SC-003**: An unrelated consumable used during a cast produces no bite
  and no reel in the same session.
- **SC-004**: The existing automated test suite passes with no changes to
  the fishing or pixel-decode tests; only the addon version pin advances.

## Assumptions

- The bait-consumption event fires on a real bite at the current game
  version, as evidenced by two independent reference implementations (one
  currently maintained and installed on the operator's machine, one the
  detection core of a working external automation); this is the last
  field-unverified link and is exactly what the in-game validation run
  exercises.
- Bait attaches to the cast, so an application-initiated early reel wastes
  bait; this is why the false-bite defect was expensive and why SC-002
  counts bait per catch.
- The operator applies the addon update and reloads the game interface
  before the next session.
