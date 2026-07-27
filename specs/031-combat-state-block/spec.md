# Feature Specification: PixelBeacon In-Combat State Block

**Feature Branch**: `031-combat-state-block`

**Created**: 2026-07-27

**Status**: Draft

**Input**: GitHub issue #9 (add in-combat / out-of-combat state block to
PixelBeacon). Build plan `docs/plans/plan-010.md`, slice 031. Master
specification section 10.3 (the pixel-bus block contract).

## Overview

The bundled PixelBeacon addon publishes a small strip of colored squares at the
top-left of the game client, and the companion samples that strip to learn things
about the game session that it cannot observe from outside. Today the strip
carries four signals: the addon is loaded, a fishing cast is active, the server
latency, and which weapon bar is drawn with what weapons on it. It does not carry
whether the player is in combat, which is one of the most basic facts about a
session and one the game exposes directly.

This feature adds that signal. The player's combat state becomes a fifth square
on the strip, the companion decodes it, and the operator can see it change in the
running application. Nothing acts on it yet: this feature adds an observable and
deliberately stops there.

The feature carries a second obligation that is larger than the signal itself.
Four of the five open tracker issues add a block to this strip, and today the
strip's size is stated three separate times in three different forms: a constant
in the companion, an unrelated literal in the addon that must be kept in step
with it by hand, and the fixed shape of the function that observes a sample set.
Each of the four issues would have to widen all three. This feature is the first
of the four, so it reduces those restatements to one authoritative statement on
each side of the contract, with everything else derived from it and an automated
check that the two sides agree. The three features that follow then extend it by
adding a single entry. Build plan 010 sequences this feature first for exactly
that reason.

A note on what "user" means here. This application has one user, the operator
running it beside the game. Everything below is written from what that operator
can observe, except where a requirement is about the contract between the addon
and the companion, which the operator experiences only as the signal being right
or wrong.

## Clarifications

### Session 2026-07-27

Answered under the build-phase autopilot decision policy from the constitution,
build plan 010, GitHub issue #9, and the existing beacon code (the latency,
weapon-bar, and fishing squares and the reader that decodes them). None were
escalated.

- Q: Does this feature change how often the strip is sampled, so a combat
  transition is seen sooner? -> A: No. The existing cadence stands: fast only
  while a fishing session is active, otherwise the idle interval, one second by
  default. Combat state has no consumer under FR-016, so nothing has a latency
  requirement, and raising the capture rate for a display-only signal would add
  screen-capture cost to every second of every session. The first consumer that
  needs sub-second combat latency raises the cadence when it exists.
- Q: What does the interface show when combat state is unavailable? -> A: "Not
  detected", in the muted palette role, exactly as the weapon-bar readout renders
  its own undetected case today. The operator reads both fields in the same
  region, so a second convention for the same idea would be noise.
- Q: At what log level is a combat change recorded? -> A: DEBUG, matching the
  existing weapon-bar detection line. The per-sample trace channel would bury it,
  and the operator's default level would flood the live log, because combat state
  changes constantly in normal play.
- Q: The beacon is alive but the combat square does not decode (an addon
  downgraded or reloaded mid-session, or a transient misread). Hold the last
  state or clear it? -> A: Clear to unavailable, on every sample where the square
  does not decode. This deliberately diverges from the weapon-bar precedent,
  which holds its last decoded value while the beacon is alive and clears only on
  signal loss. Unavailable is defined as "the companion could not read the
  signal", so holding would make that definition false, and a stale "in combat"
  persisting after the square stops being drawn is exactly the false reading User
  Story 2 exists to prevent. The competing concern, a one-sample flap from a
  transient misread, costs nothing while FR-016 holds that nothing consumes the
  value; a later consumer that needs hysteresis adds it at the consumer. Recorded
  as a dated decision, because the following slices inherit it.
- Q: Can the addon ever hide the combat square, the way the fishing square hides
  when idle? -> A: No. It renders whenever the status square renders and is
  hidden only when the status square is, following the latency and weapon
  squares. If the square could hide, absence would be ambiguous between "no
  combat information right now" and "an addon too old to draw it", and User Story
  2 depends on absence meaning exactly the latter. The tri-state therefore comes
  entirely from the decoded color, never from presence.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - The operator can see combat state in the companion (Priority: P1)

The operator runs the application alongside the game with the current PixelBeacon
installed. When their character enters combat, the application reflects that
within its normal sampling interval; when combat ends, it reflects that too. The
state survives a loading screen: after zoning, the application shows the true
state rather than whatever was true before the load.

**Why this priority**: This is the feature. Every other story exists to keep this
one honest.

**Independent Test**: With the game running and the addon installed, enter and
leave combat and confirm the application follows on both transitions, then take a
loading screen while in a known state and confirm the application agrees
afterwards. Fully testable on its own; delivers the whole user-visible value.

**Acceptance Scenarios**:

1. **Given** the addon is installed and the application is reading the strip,
   **When** the character enters combat, **Then** the application reports the
   in-combat state without the operator taking any action.
2. **Given** the character is in combat and the application reports it, **When**
   combat ends, **Then** the application reports the out-of-combat state.
3. **Given** the character is in a known combat state, **When** a loading screen
   completes, **Then** the application reports the state that is actually true
   after the load, not the state from before it.
4. **Given** the character's combat state has not changed, **When** the
   application samples the strip repeatedly, **Then** it reports no change and
   produces no repeated log entries.

---

### User Story 2 - An out-of-date addon never produces a false reading (Priority: P2)

The operator updates the application but has not yet updated the addon, so the
game is still drawing the previous four-square strip. The application samples
where the fifth square would be and finds whatever the game happens to be
drawing there. It must treat that as "no combat information available" and never
as a combat state.

**Why this priority**: A wrong reading is worse than a missing one, and this is
the failure mode most likely to occur in the field, because the application and
the addon update independently. The existing weapon-bar square already sets this
precedent and this feature must match it.

**Independent Test**: Point the reader at a strip that has no fifth square and
confirm the reported state is the unavailable one, for arbitrary colors behind
the square's position. Testable at the desk with no game.

**Acceptance Scenarios**:

1. **Given** an addon version that draws no fifth square, **When** the
   application samples the strip, **Then** combat state reports as unavailable
   and no combat change is announced.
2. **Given** an arbitrary color at the fifth square's position that is not a
   valid combat encoding, **When** the application samples it, **Then** the
   result is the unavailable state rather than a guess.
3. **Given** the application has been reporting a combat state, **When** the
   signal from the addon is lost entirely, **Then** the combat state is cleared
   to unavailable rather than left showing the last value indefinitely.

---

### User Story 3 - The next block costs one entry, not a refactor (Priority: P3)

A developer adding the sixth square (the menu-state gate, the next slice in build
plan 010) states the strip's new size once and adds one sample point. They do not
have to find and update a hand-maintained literal in the addon, and they do not
have to change the shape of the observing function or every one of its existing
call sites and tests.

**Why this priority**: It is invisible to the operator, so it cannot rank above
the signal itself, but it is the reason this slice runs first in its plan and the
reason the three following slices are cheap. Deferring it means paying the same
widening cost four times.

**Independent Test**: Confirm the strip's block count is stated exactly once and
that the addon's drawn width and the companion's capture region both derive from
it, by changing that one value and observing both follow. Testable at the desk.

**Acceptance Scenarios**:

1. **Given** the block count is changed in its single location on one side,
   **When** that side computes its geometry, **Then** every geometry on that side
   reflects the new count with no other edit.
2. **Given** the two sides are changed to disagree about the count, **When** the
   automated checks run, **Then** the disagreement is reported rather than
   shipped.
2. **Given** a new block is added to the observed sample set, **When** the
   observing function is called, **Then** existing callers do not need their
   argument lists rewritten for each added block.

---

### Edge Cases

- **The addon is older than the application.** Covered by User Story 2: the
  fifth square is absent and must read as unavailable, never as a state.
- **The application is older than the addon.** The addon draws a square the
  application does not sample. The extra square is ignored and no existing
  signal is disturbed, because each square is read at its own position.
- **The signal is lost while in combat.** The last reported state must not
  persist as though it were still true; it clears to unavailable, matching how
  the weapon-bar signal already behaves on signal loss.
- **The addon is downgraded or reloaded mid-session.** The beacon stays alive but
  the combat square stops being drawn. The state clears to unavailable on the
  next sample rather than holding the last value, which is the one behavior here
  that deliberately differs from the weapon-bar square.
- **A single sample misreads the square.** The state flaps to unavailable and
  back on the following sample. Accepted, because nothing consumes the value
  under FR-016; a consumer that cannot tolerate the flap adds its own hysteresis.
- **Combat state changes faster than the sampling interval.** A transition that
  begins and ends between two samples is not observed. This is inherent to
  sampling a screen signal and is accepted: the state is a level, not an event,
  so the next sample reports the truth even if an intermediate flip was missed.
- **Combat state flaps rapidly during a fight.** The application must not emit a
  stream of duplicate reports; only real changes are announced.
- **A very small square size is configured.** Block geometry already derives from
  the configured square size down to a 2 pixel edge, and the new square must
  derive from the same source rather than assuming the default size.
- **Something else on screen is drawing at the strip's position.** The square
  carries a validity mark, so an arbitrary color is rejected as unavailable
  rather than decoded. The mark must be far enough from every other mark already
  on the strip that the reader's color-match tolerance can never confuse them.
- **The game is not running or the strip is not visible.** No change from today:
  the whole strip reads as absent and combat state is unavailable.

## Requirements *(mandatory)*

### Functional Requirements

**The signal**

- **FR-001**: The addon MUST publish the player's combat state as a dedicated
  square on the beacon strip. The square MUST be drawn whenever the addon is
  loaded and rendering, and MUST NOT be hidden to express a state, so that its
  absence means only that the addon is too old to draw it.
- **FR-002**: The published state MUST distinguish exactly three cases to the
  companion: in combat, out of combat, and unavailable.
- **FR-003**: The addon MUST update the published state on the game's own combat
  transition notification, so the strip changes at the moment combat starts or
  ends rather than at the next periodic refresh.
- **FR-004**: The addon MUST re-establish the published state from the game's
  current value after each loading screen, because a transition notification may
  not fire for the state that is already true when the world loads.
- **FR-005**: The addon MUST only redraw the square when the state actually
  changes, so a steady state produces a steady signal.
- **FR-006**: The square MUST carry a validity mark distinct from every other
  mark on the strip, so no other square's color and no unrelated screen content
  can be decoded as a combat state. The two encoded combat states MUST likewise
  be distinct from each other. Both separations are measured against the reader's
  default per-channel color-match tolerance, which is the fixed reference for
  this requirement. An operator who raises that tolerance far enough to collide
  encodings degrades every square on the strip equally; that is a pre-existing
  property of the whole beacon design, not something this feature introduces, and
  correcting it is out of scope here.
- **FR-007**: The companion MUST decode the square into the three states of
  FR-002, returning the unavailable state whenever the validity mark does not
  match.
- **FR-008**: The companion MUST announce a combat state only when the decoded
  state changes. It MUST clear the state to unavailable when the beacon signal is
  lost, and also on any sample where the square is present but does not decode,
  rather than holding the last known value.
- **FR-009**: The companion MUST record each combat state change in the
  application log at the debug level, matching the existing weapon-bar detection
  entry, so the signal can be confirmed in the field without a debugger and
  without flooding the log at the operator's default level.
- **FR-010**: The companion MUST present the current combat state in the
  application interface alongside the existing weapon-bar readout, and MUST
  render the unavailable case as "Not detected" in the muted palette role, the
  same treatment the weapon-bar readout already uses.

**Compatibility and geometry**

- **FR-011**: The companion MUST treat the combat square as optional: an addon
  that does not draw it MUST yield the unavailable state and MUST NOT disturb any
  existing signal.
- **FR-012**: The combat square's sampled position MUST derive from the
  configured square size using the same rule as every other square, so a
  non-default square size needs no separate adjustment.
- **FR-013**: The number of squares on the strip MUST be stated exactly once on
  each side of the contract, and every geometry that depends on it MUST derive
  from that statement rather than restate it: the width the addon draws, the
  placement of each square, and the region the companion captures. Because the
  two sides are separate codebases, an automated check MUST detect any
  disagreement between them, so a mismatch cannot reach a release on a reviewer
  failing to notice.
- **FR-014**: The set of samples the companion observes MUST be expressible
  without changing the observing function's shape for each square added, so a
  later square does not force every caller and test to be rewritten.
- **FR-015**: The addon manifest version MUST advance so the application's beacon
  manager offers the update to operators running the previous addon, and the
  manifest description MUST name the new signal.

**Boundaries**

- **FR-016**: This feature MUST NOT change any behavior based on combat state.
  Weave timing, key synthesis, input interception, and the fishing controller
  behave exactly as they do today.
- **FR-017**: This feature MUST NOT add, remove, or alter any input path, and
  MUST leave the existing safety behaviors untouched: input suppression stays
  scoped to the focused game window, synthesized input stays flagged against
  recursion, the hook thread stays free of blocking work, and fishing still
  degrades to disabled when the beacon signal is lost.
- **FR-018**: The master specification's pixel-bus section MUST document the new
  square, and the changelog MUST record the feature plus a dated decision for
  each contract later slices inherit.
- **FR-019**: This feature MUST NOT change the strip sampling cadence. Combat
  state is observed at whatever interval the reader already uses, so no
  additional screen capture cost is introduced.

### Key Entities

- **Combat state**: What the game reports about whether the player is currently
  in combat. Three values as seen by the companion: in combat, out of combat, and
  unavailable. Unavailable is not a fourth game state; it means the companion
  could not read the signal.
- **Beacon strip**: The ordered run of squares the addon draws and the companion
  samples. Its length is the number of squares; its geometry derives from the
  configured square size. This feature makes it five squares.
- **Validity mark**: The part of a square's color that identifies which signal it
  carries. Each square that encodes data carries one, and they must stay far
  enough apart that the reader's tolerance cannot confuse two squares or mistake
  unrelated screen content for a signal.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: With the game running and the current addon installed, entering or
  leaving combat is reflected by the application within one sampling interval
  (the reader's existing idle interval, one second by default), on 100 percent of
  transitions across a validation run of at least ten transitions.
- **SC-002**: Across a validation run in which combat state does not change, the
  application reports zero combat changes.
- **SC-003**: After a loading screen taken in a known combat state, the
  application agrees with the game's actual state on 100 percent of attempts.
- **SC-004**: With an addon that does not draw the combat square, the application
  reports combat state as unavailable and reports zero combat changes, for every
  color that can appear behind the square's position.
- **SC-005**: All four signals that exist today (addon loaded, fishing, latency,
  weapon bar) behave identically before and after this feature, verified by the
  existing automated checks continuing to pass unchanged in intent.
- **SC-006**: Changing the number of squares requires editing exactly one
  location on each side of the contract, and a disagreement between the two sides
  is caught automatically rather than by review.
- **SC-007**: The full merge gate (formatting, lint at deny-warnings, and the
  whole test suite) passes with no test weakened, skipped, or made conditional.

## Assumptions

- **The game exposes combat state directly and reliably.** The current value and
  a transition notification are both available to an addon, and both names are
  verified present in the live ESO API source. This feature has no unresolved
  API question, which is why build plan 010 sequences it before the movement
  block, whose sprint signal is unverified.
- **Combat state is a level, not an event stream.** The companion cares what the
  state is now, not how many times it changed. This is what makes a sampled
  screen signal an adequate transport and what makes a missed intermediate flip
  acceptable.
- **The operator updates the addon through the application.** The manifest
  version bump is the mechanism by which they are offered the update; an operator
  who declines is covered by User Story 2.
- **Presenting the state beside the weapon-bar readout is the right surface.**
  The weapon bar is the existing example of a decoded player-state signal shown
  to the operator, so combat state follows it rather than inventing a new
  surface.
- **No consumer of combat state exists yet.** The observable is added on the
  expectation that a later feature acts on it. Adding it without a consumer is
  deliberate: it makes the signal verifiable in the field before anything depends
  on it being correct.
- **One second of latency on a combat transition is acceptable.** Nothing acts on
  the signal yet, so the only cost of the existing sampling cadence is how
  promptly the readout updates for a human watching it. A consumer with a real
  timing requirement is what would justify sampling faster, and none exists.
- **The unavailable state and the out-of-combat state must stay distinguishable.**
  Collapsing them would make a missing addon look like a peaceful session, which
  is the false reading User Story 2 exists to prevent.

## Dependencies

- The bundled PixelBeacon addon and its manifest, which the application installs,
  updates, and removes.
- The companion's beacon strip reader and its configured square size, which
  already single-sources block geometry.
- The application's event routing and interface view model, which already carry
  the weapon-bar signal from the reader to the operator.
- The master specification's pixel-bus section, which is the architecture of
  record for the strip contract. It is stale at v0.2.0 overall and scheduled for
  a separate refresh; this feature updates the section it touches rather than
  waiting on that refresh.
