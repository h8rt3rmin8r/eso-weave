# Feature Specification: PixelBeacon Skill-Cooldown Blocks

**Feature Branch**: `037-cooldown-blocks`

**Created**: 2026-07-27

**Status**: Draft

**Input**: GitHub issue #18 (publish per-slot skill cooldowns as PixelBeacon
blocks). Build plan `docs/plans/plan-012.md`, slice 037, which this feature also
authors. Master specification section 10.3 (the pixel-bus block contract).

## Overview

The bundled PixelBeacon addon publishes a grid of colored squares at the
top-left of the game client, and the companion samples that grid to learn things
about the session it cannot observe from outside. Today the grid carries ten
signals, from the addon heartbeat through the player's resources and mounted
state. None of them says anything about the player's skills.

This feature adds six squares, one for each slot the game actually exposes a
cooldown for (the five skills and the ultimate), each publishing how long that
slot has left before it can be used again. The companion decodes them, shows
them, and stops there.

The reason to add this signal rather than another is that the application
currently guesses at exactly this information. The weave engine's global cooldown
is a fixed setting, and its per-weapon heavy-attack delays are, by the admission
of the code comment beside them, community estimates never validated in game,
with one weapon class an outright guess. The master specification lists those
same measurements as owed. The game knows the real answer per slot and has simply
never been asked. This feature asks it, and puts the answer where a later feature
can act on it.

This feature is also the first to use the space the grid wrap created. The grid
has wrapped at a fixed sixteen columns since slice 035, and at ten blocks it has
used ten of them. Six more takes the count to sixteen, which **exactly fills the
first row**: the widest a single row can be, and the precise bound the fixed
column count exists to enforce. One more block wraps.

That boundary case is worth landing deliberately and testing explicitly, because
it is the last count at which the grid is still one row, and because the slice
that follows this one is the slice that crosses. Build plan 012 sequences this
feature first for that reason: the grid arrives at its single-row maximum in a
slice that only adds squares, so the slice that then crosses onto a second row
does so from a known and asserted starting point.

A note on what "user" means here. This application has one user, the operator
running it beside the game. Everything below is written from what that operator
can observe, except where a requirement is about the contract between the addon
and the companion, which the operator experiences only as the signal being right
or wrong.

## Clarifications

### Session 2026-07-27

Answered under the build-phase autopilot decision policy from the constitution,
build plan 012, GitHub issue #18, and the existing beacon code. None were
escalated.

- Q: The application has seven slots, but two of them are unusual: Ultimate is
  gated by ultimate points rather than a cooldown, and Synergy is a contextual
  prompt rather than a conventional action slot. Do they get squares? -> A: Six
  squares, covering the five skills and the ultimate. Synergy gets none. This was
  settled by verification rather than judgement: the game's own action bar
  iterates its slots from the first normal slot index through the ultimate slot
  index, and Synergy is not in that range because it is not an action slot at
  all. It therefore has no cooldown to read, in any state, ever. Publishing a
  seventh square that could never carry information would spend a square, a
  validity mark, and a permanently muted interface field to buy nothing but
  positional symmetry with the interface, and would additionally push the grid
  onto a second row for the sake of a dead block. The interface simply leaves the
  Synergy row's cooldown field empty. Ultimate does stay, because it is a real
  action slot; whether the game reports a meaningful cooldown for it is then just
  data, and the unavailable case already covers the answer being "none".
- Q: One shared validity mark for all six squares, or six distinct ones? ->
  A: Six distinct. The marks exist so that a geometry error off by one block
  fails loudly instead of decoding a neighbour's value as this slot's, and six
  adjacent squares carrying the same kind of value are precisely where that
  failure would otherwise be silent and plausible. A shared mark would save
  nothing but the effort of choosing values.
- Q: Does this feature change how often the grid is sampled? -> A: No. Nothing
  consumes the values, so nothing has a latency requirement, and the existing
  cadence already samples fast whenever the application can intercept.
- Q: Does the compile-time assertion added by the previous slice, that the block
  count does not exceed the column count, still hold? -> A: Yes, and exactly at
  its limit: sixteen blocks in sixteen columns satisfies it with no margin left.
  It is deliberately NOT relaxed or removed by this feature. It now guards the
  single most valuable edit in this family's future, the seventeenth block, and
  weakening it here to save the next slice an inconvenience would throw away the
  warning at precisely the moment it becomes useful. The slice that adds the
  seventeenth block is the one that should be told, and it will be.
- Q: Does this feature exercise the second row at all, then? -> A: No, and it
  should not pretend to. It lands the grid at the single-row maximum and asserts
  that it is exactly there. The row crossing belongs to the following slice, and
  the geometry for it is already built and already covered by tests that
  construct multi-row cases directly, so nothing about it is unbuilt. What this
  feature owes the next one is a known, asserted starting point rather than a
  speculative one.
- Q: What resolution and range should the remaining time carry? -> A: Fifty
  milliseconds per step up to 12.7 seconds, with a distinct value for ready and
  another for unavailable. That covers the full range of ordinary skill cooldowns
  at a resolution far finer than the sampling interval, so the transport is never
  the limiting factor. A longer range would cost resolution where it matters; a
  finer resolution would buy precision the sampling cadence cannot deliver.
- Q: Do the six values travel to the rest of the application as six separate
  announcements or as one? -> A: One, carrying all six. This follows the
  resource blocks, whose own rationale is recorded in the code: a sample in which
  several values move at once is the common case in combat, and should be one
  announcement rather than several. Six slots make that argument stronger than
  three did, because a single weave can move most of them in one sample. It also
  resolves FR-009 directly: one announcement per changed sample is one log entry
  per changed sample, so the flooding the requirement forbids cannot arise from
  the announcement shape.
- Q: What does a cooldown log entry contain, given six slots changing
  constantly during combat? -> A: One entry per changed sample, naming only the
  slots whose value actually changed. Logging all six every time would bury the
  change in noise, and logging nothing but "cooldowns changed" would make the
  entry useless for confirming the signal in the field, which is the only reason
  the entry exists.
- Q: Where does the operator see this, given the interface already lists the
  seven slots? -> A: As a new field on each existing skill row, not as a separate
  region. The association between a slot and its cooldown is then positional and
  needs no explanation. Two consequences must be carried rather than discovered:
  the interface's skill column set is currently pinned at five columns by an
  automated check, which must be updated deliberately, and the skills region is
  the widest content-sized block in the window, so adding a column changes the
  computed intrinsic width. The window-sizing tests introduced in slice 030 exist
  precisely to catch that and MUST be extended rather than left to pass by
  accident.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - The operator can see which skills are ready (Priority: P1)

The operator runs the application alongside the game with the current PixelBeacon
installed. Each slot that has a cooldown shows whether it is ready and, when it is
not, roughly how long is left. Using a skill makes its slot show a cooldown that
counts down and then clears. The values survive a loading screen.

**Why this priority**: This is the feature. Every other story exists to keep this
one honest.

**Independent Test**: With the game running and the addon installed, use a skill
and confirm its slot reports a cooldown that decreases and then reports ready.
Confirm the other slots are unaffected. Take a loading screen and confirm the
values agree with the game afterwards.

**Acceptance Scenarios**:

1. **Given** the addon is installed and the application is reading the grid,
   **When** the operator uses a skill, **Then** that slot reports a remaining
   time greater than zero without the operator taking any action.
2. **Given** a slot is counting down, **When** its cooldown expires, **Then** the
   slot reports ready.
3. **Given** one slot is on cooldown, **When** the application samples the grid,
   **Then** no other slot's reported value changes as a result.
4. **Given** the slots are in a known state, **When** a loading screen completes,
   **Then** the reported values agree with the game rather than with the state
   from before the load.
5. **Given** no slot's state has changed, **When** the application samples the
   grid repeatedly, **Then** it reports no changes and produces no repeated log
   entries.

---

### User Story 2 - An out-of-date addon never produces a false reading (Priority: P2)

The operator updates the application but has not yet updated the addon, so the
game is still drawing the previous ten-square grid. The application samples where
the six new squares would be and finds whatever the game happens to be drawing
there, including, for the first time, screen content well outside the old grid's
footprint. It must treat all of it as "no cooldown information available" and
never as a cooldown.

**Why this priority**: A wrong reading is worse than a missing one, and this is
the failure mode most likely to occur in the field, because the application and
the addon update independently. This feature raises the stakes: six squares are sampled
instead of one, and the grid reaches the full width of a row for the first time.

**Independent Test**: Point the reader at a grid that has no new squares and
confirm every one of the six reports unavailable, for arbitrary colors behind
each position.

**Acceptance Scenarios**:

1. **Given** an addon version that draws none of the new squares, **When** the
   application samples the grid, **Then** all six slots report unavailable and
   no cooldown changes are announced.
2. **Given** an arbitrary color at any new square's position that is not a valid
   cooldown encoding, **When** the application samples it, **Then** the result is
   unavailable rather than a guess.
3. **Given** the application has been reporting cooldowns, **When** the signal
   from the addon is lost entirely, **Then** every slot is cleared to unavailable
   rather than left showing the last values.
4. **Given** a color valid for one slot's square, **When** it appears at a
   different slot's position, **Then** it does not decode, so a geometry error
   cannot silently report one slot's cooldown as another's.

---

### User Story 3 - The grid lands exactly on its single-row maximum (Priority: P2)

The grid grows from ten squares to sixteen, filling the first row exactly. It is
now as wide as a row can be and still one row tall. Every existing signal keeps
its position and its meaning, the overlay grows sideways to its bound and no
further, and the next square added anywhere in this family will wrap.

**Why this priority**: It ranks alongside the false-reading story rather than
below it because a defect in the grid's extent would corrupt every signal at
once, not just the new ones. It also leaves the following slice a known starting
point instead of a guessed one: that slice crosses the boundary, and it should do
so from a state that has been asserted rather than assumed.

**Independent Test**: Confirm the captured region is exactly one full row wide
and one row tall, that every pre-existing square keeps the exact position it had
at ten squares, and that the overflow warning still reports correctly when the
full-width grid does not fit.

**Acceptance Scenarios**:

1. **Given** the block count now equals the column count, **When** the captured
   region is computed, **Then** it is exactly one full row wide and one row tall.
2. **Given** the grid has grown, **When** any pre-existing square's position is
   computed, **Then** it is unchanged from before this feature.
3. **Given** a client area too small for the full-width grid, **When** the
   application checks the fit, **Then** it reports the overflow.
4. **Given** the block count has reached the column count, **When** the automated
   checks run, **Then** the assertion guarding that boundary still holds and is
   left in place to catch the next block rather than relaxed.

---

### Edge Cases

- **The addon is older than the application.** Covered by User Story 2: the new
  squares are absent and must read as unavailable.
- **The application is older than the addon.** The addon draws squares the
  application does not sample. They are ignored and no existing signal is
  disturbed, because each square is read at its own position.
- **A slot is empty, or the game reports no cooldown for it.** Reported as
  unavailable, the same value as an absent square. This is deliberate: both mean
  "no cooldown information", and distinguishing them would imply a precision the
  signal does not have.
- **A cooldown is longer than the encodable range.** Reported as the maximum
  encodable value until it falls inside the range, so a long cooldown reads as
  "at least this long" rather than wrapping to a small number.
- **A cooldown is shorter than the sampling interval.** It may never be observed.
  Inherent to sampling a screen signal and accepted: the value is a level, not an
  event.
- **The signal is lost while slots are on cooldown.** All six clear to
  unavailable rather than counting down from stale values.
- **The addon is downgraded or reloaded mid-session.** The beacon stays alive but
  the new squares stop being drawn; each clears to unavailable on the next
  sample.
- **A very small square size is configured.** Every new square's position derives
  from the configured size and the shared column count, as all existing squares
  do, so no separate adjustment is needed at any supported size.
- **The taller grid does not fit the client area.** The existing overflow warning
  reports it. The application still samples, because the squares that do fit
  still decode and refusing to sample would turn a partial loss into a total one.
- **Something else on screen is drawing at a new square's position.** Each square
  carries a validity mark and a checksum, so an arbitrary color is rejected.

## Requirements *(mandatory)*

### Functional Requirements

**The signal**

- **FR-001**: The addon MUST publish one square per skill slot the application
  supports, each carrying how long that slot has left before it can be used
  again. The squares MUST be drawn whenever the addon is loaded and rendering,
  and MUST NOT be hidden to express a state, so that absence means only that the
  addon is too old to draw them.
- **FR-002**: Each square MUST distinguish three cases to the companion: a
  remaining time, ready, and unavailable. Unavailable covers an empty slot, a
  slot the game reports no cooldown for, and a square that cannot be read.
- **FR-003**: The remaining time MUST be carried at a resolution finer than the
  application's sampling interval, so the transport is never the limiting factor
  on precision, and MUST cover the range of ordinary skill cooldowns. A value
  beyond the encodable range MUST report as the maximum rather than wrap.
- **FR-004**: The addon MUST re-establish the published values from the game's
  current state after each loading screen.
- **FR-005**: The addon MUST only redraw a square when its value actually
  changes, so a steady state produces a steady signal.
- **FR-006**: Each square MUST carry a validity mark distinct from every other
  mark on the grid, so no other square's color and no unrelated screen content
  can be decoded as a cooldown, and so a color valid for one slot cannot decode
  at another slot's position. Separation is measured against the reader's default
  per-channel color-match tolerance, which is the fixed reference for this
  requirement. Every new mark MUST be added to the shared registry of marks in
  use, so the separation is proven by the existing automated check rather than
  asserted by the author. An operator who raises that tolerance far enough to
  collide encodings degrades every square on the grid equally; that is a
  pre-existing property of the beacon design and correcting it is out of scope.
- **FR-007**: The companion MUST decode each square into the three cases of
  FR-002, returning unavailable whenever the validity mark does not match or the
  encoding fails its integrity check.
- **FR-008**: The companion MUST announce a slot's value only when it changes,
  and MUST clear every slot to unavailable when the beacon signal is lost, and
  also on any sample where a square is present but does not decode, rather than
  holding the last known value. This follows the decision recorded for the combat
  block.
- **FR-009**: The companion MUST record cooldown changes in the application log
  at the debug level, as one entry per changed sample naming only the slots whose
  value changed. It MUST NOT emit one entry per slot per sample, because six
  slots changing continuously during combat would flood the log.
- **FR-010**: The companion MUST carry all six values in a single announcement
  rather than one per slot, following the resource blocks, so a sample in which
  several slots move at once is one change rather than several.
- **FR-011**: The companion MUST present the current cooldown state as a field on
  each skill row the interface already displays, so the association between a
  slot and its cooldown is positional, and MUST render the unavailable case in
  the same muted treatment the existing decoded readouts use. The automated check
  pinning the interface's skill column set MUST be updated deliberately, and the
  window-sizing tests MUST be extended to cover the wider skills region, because
  that region determines the window's intrinsic width.

**The single-row maximum**

- **FR-012**: The grid MUST end this feature exactly filling one row, and the
  captured region MUST be exactly one full row wide and one row tall. This MUST
  be asserted rather than assumed, because it is the boundary case: one more
  square wraps.
- **FR-013**: Every square that existed before this feature MUST keep the exact
  position it had, so no existing signal moves.
- **FR-014**: The existing overflow report MUST continue to detect a grid that
  does not fit the client area.
- **FR-015**: The compile-time assertion that the block count does not exceed the
  column count MUST be left in force and MUST NOT be relaxed, removed, or widened
  to accommodate a future count. It now sits exactly at its limit and guards the
  next block added anywhere in this family. Weakening it here, to spare the
  following slice an inconvenience, would discard the warning at precisely the
  moment it becomes useful.

**Compatibility and geometry**

- **FR-016**: The companion MUST treat every new square as optional: an addon
  that does not draw them MUST yield unavailable and MUST NOT disturb any
  existing signal.
- **FR-017**: Each new square's sampled position MUST derive from the configured
  square size and the shared column count using the same rule as every other
  square.
- **FR-018**: The number of squares MUST remain stated exactly once on each side
  of the contract, with every dependent geometry derived from it, and the
  existing automated cross-language check MUST be extended to cover the new count
  and every new mark and encoding constant, so a disagreement between the two
  sides cannot reach a release.
- **FR-019**: The addon manifest version MUST advance so the application's beacon
  manager offers the update, and the manifest description MUST name the new
  signal.

**Boundaries**

- **FR-020**: This feature MUST NOT change any behavior based on cooldown values.
  Weave timing, key synthesis, input interception, and the fishing controller
  behave exactly as they do today.
- **FR-021**: This feature MUST NOT add, remove, or alter any input path, and
  MUST leave the existing safety behaviors untouched: input suppression stays
  scoped to the focused game window, synthesized input stays flagged against
  recursion, the hook thread stays free of blocking work, and fishing still
  degrades to disabled when the beacon signal is lost.
- **FR-022**: The master specification's pixel-bus section MUST document the new
  squares, the changelog MUST record the feature plus a dated decision for each
  contract later slices inherit, and the build plan sequencing this slice and the
  two that follow MUST be authored as part of this feature.
- **FR-023**: This feature MUST NOT change the grid sampling cadence.

### Key Entities

- **Slot cooldown**: What the game reports about how long one skill slot has
  before it can be used again. Three values as seen by the companion: a remaining
  time, ready, and unavailable. Unavailable is not a duration; it means the
  companion could not read the value or the game reports none.
- **Beacon grid**: The ordered run of squares the addon draws and the companion
  samples, wrapped at a fixed shared column count. This feature makes it
  sixteen squares filling exactly one row.
- **Validity mark**: The part of a square's color identifying which signal it
  carries. Each encoding square has one, and they must stay far enough apart that
  the reader's tolerance cannot confuse two squares or mistake unrelated screen
  content for a signal.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: With the game running and the current addon installed, using a
  skill is reflected as a non-zero remaining time on that slot within one
  sampling interval, on 100 percent of uses across a validation run of at least
  ten uses.
- **SC-002**: Across a validation run in which no slot changes state, the
  application reports zero cooldown changes.
- **SC-003**: Using one slot changes the reported value of no other slot, across
  every slot.
- **SC-004**: With an addon that does not draw the new squares, all six slots
  report unavailable and zero cooldown changes are reported, for every color that
  can appear behind any of the six positions.
- **SC-005**: All ten signals that exist today behave identically before and
  after this feature, and every pre-existing square occupies the identical
  position, verified by the existing automated checks continuing to pass
  unchanged in intent.
- **SC-006**: The captured region is proven to be exactly one full row wide and one
  row tall, and the two sides of the contract are proven to agree on every new
  constant automatically.
- **SC-007**: A color valid for one slot decodes as unavailable at every other
  slot's position, in 100 percent of cross-position combinations.
- **SC-008**: The full merge gate (formatting, lint at deny-warnings, and the
  whole test suite) passes with no test weakened, skipped, or made conditional.

## Assumptions

- **The game exposes per-slot cooldown directly and reliably.** The current
  remaining time and the usability of a slot are both available to an addon, and
  both names are verified present. This feature has no unresolved API question.
- **A cooldown is a level, not an event stream.** The companion cares what the
  remaining time is now, not how many times a slot has been used. This is what
  makes a sampled screen signal an adequate transport.
- **Ultimate and Synergy may report no cooldown.** Ultimate is gated by ultimate
  points and Synergy is a contextual prompt rather than a conventional slot, so
  the game may report nothing for either. The unavailable case already covers
  that correctly, so it is an observation to confirm rather than a risk to
  mitigate.
- **The operator updates the addon through the application.** The manifest
  version bump is the mechanism by which they are offered the update; an operator
  who declines is covered by User Story 2.
- **No consumer of cooldown values exists yet.** The observable is added on the
  expectation that a later feature acts on it. Adding it without a consumer is
  deliberate: it makes the signal verifiable in the field before anything depends
  on it being correct.
- **The grid wrap is correct and merely unexercised.** The geometry was built for
  multiple rows and is covered by tests that construct multi-row cases directly;
  what has never happened is a shipping block count that crosses the boundary.

## Dependencies

- The bundled PixelBeacon addon and its manifest, which the application installs,
  updates, and removes.
- The companion's beacon grid reader, its configured square size, and the shared
  column count, which already single-source block geometry.
- The application's event routing and interface view model, which already carry
  decoded signals from the reader to the operator, and the skills region of the
  interface, which already lists the seven slots.
- The existing cross-language check that parses the embedded addon source to
  prove the two sides of the contract agree.
- The master specification's pixel-bus section, which is the architecture of
  record for the grid contract. It is stale at v0.2.0 overall and scheduled for a
  separate refresh; this feature updates the section it touches rather than
  waiting on that refresh.
