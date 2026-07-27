# Feature Specification: PixelBeacon Quickslot-State Blocks

**Feature Branch**: `038-quickslot-blocks`

**Created**: 2026-07-27

**Status**: Draft

**Input**: GitHub issue #19 (publish the active quickslot state as PixelBeacon
blocks). Build plan `docs/plans/plan-012.md`, slice 038. Master specification
section 10.3 (the pixel-bus block contract).

## Overview

The bundled PixelBeacon addon publishes a grid of colored squares at the
top-left of the game client, and the companion samples that grid to learn things
about the session it cannot observe from outside. Today the grid carries sixteen
signals, from the addon heartbeat through the player's resources and the
per-slot skill cooldowns the previous feature added. None of them says anything
about the quickslot, which is where the player keeps a potion.

This feature adds four squares describing the active quickslot: how long it has
left before it can be used again, and which item is in it. The companion decodes
them, shows them, and stops there.

The reason to add this signal rather than another is that the feature which
follows, automatically drinking a potion when a resource runs low, cannot safely
fire anything without all of it. It needs to know that the quickslot holds a
potion at all, that the potion is off cooldown, and which item it is, so a swap
is noticed rather than assumed away. That consumer is deliberately a separate
slice, because it acts by synthesizing a keypress and therefore lands on a
constitution NON-NEGOTIABLE surface. Publishing the observable first means that
when the potion fires at the wrong moment, it is already known whether the
reading underneath it is correct.

**This feature crosses the grid onto a second row.** The previous feature left
the block count at exactly sixteen against a fixed column count of sixteen,
filling the first row completely and leaving no margin, deliberately. These four
squares take the count to twenty, so four land on a second row and the captured
region becomes two rows tall for the first time in the project's life. The
geometry for that was built by the grid-wrap feature and is covered by tests
that construct multi-row cases directly, so nothing about it is unbuilt. What is
untrue for the first time is a set of expectations written when one row was the
only shipping possibility, including a compile-time assertion the previous
feature deliberately left in place to fail here. Each of those must be found and
changed on purpose, and none may be relaxed pre-emptively to make room.

That crossing also makes the overlay in the corner of the game client twice as
tall, which the previous feature dissolved as a question rather than settling.
This feature owes an answer, and gives one below.

A note on what "user" means here. This application has one user, the operator
running it beside the game. Everything below is written from what that operator
can observe, except where a requirement is about the contract between the addon
and the companion, which the operator experiences only as the signal being right
or wrong.

## Clarifications

### Session 2026-07-27

Answered under the build-phase autopilot decision policy from the constitution,
build plan 012, GitHub issue #19, and the existing beacon code. None were
escalated.

- Q: What does the operator see, and what must the application account for, now
  that the overlay is twice as tall? -> A: The overlay covers twice the screen
  area it did, anchored at the same corner, and at the default square size that
  is the top-left corner of the game's interface to a depth of two squares
  rather than one. Nothing is built to move it and nothing is built to shrink it
  automatically. The square size is already an operator setting with a supported
  range down to a small fraction of the default, so a footprint the operator
  dislikes already has a remedy that keeps both sides of the contract in
  agreement, which an automatic adjustment on one side would not. What the
  application owes is that the operator can learn the footprint from the
  application rather than by measuring pixels on their own screen: the current
  grid extent, in squares and in physical pixels, MUST be reported, and the
  documentation MUST state the footprint and name the square-size setting as the
  remedy. Moving the overlay to a different corner is a real option and is
  deliberately out of scope: the anchor is part of the geometry contract shared
  by both sides, so relocating it is its own change with its own failure mode,
  and bundling it here would put an untested origin underneath twenty squares in
  the same release that first uses two rows.
- Q: Does the compile-time assertion that the block count does not exceed the
  column count get relaxed, widened, or removed? -> A: Replaced, with the
  two-row expectation. It has done exactly the job it was left in place to do:
  it fails at this feature's first edit and names the boundary being crossed.
  Relaxing it to a larger bound would leave a guard that no longer states
  anything true about what ships, and removing it would discard the only
  automatic warning that the grid's shape has changed. The replacement asserts
  what is now true and is just as specific: the block count occupies exactly two
  rows, the last row is partially filled, and the captured region is exactly as
  wide as a full row and exactly two rows tall.
- Q: Is the item's identity published, or what the potion restores? -> A: The
  identity. The obvious design is to publish which stats the potion restores, so
  a consumer can watch exactly those, and it is not available as structured
  data. The game exposes an item's on-use ability as a header, a description,
  and cooldown values; the restore types appear only inside the description,
  which is a localized human-readable string the game's own interface consumes
  as tooltip text. Parsing it would be a locale-dependent heuristic that breaks
  on any non-English client and on any wording change in a patch, and it would
  bake that fragility into a color contract shared byte for byte between two
  codebases. That is the same class of thing the sprint verification rejected
  two features ago. The identity is machine-readable and stable, gives the
  consumer both a name to show and a way to notice a swap, and leaves restore
  awareness addable later in the companion without touching the bus contract.
- Q: Does "there is a potion in the quickslot" get its own square? -> A: No. It
  is carried by the reserved payload value on the cooldown square, exactly as
  the resource squares already carry their own unavailable case. A dedicated
  square would be marginally clearer to read and would spend a whole square, a
  validity mark, and a row position on a single bit that the reserved value
  carries for nothing. It would also push the count to twenty-one for no gain.
- Q: How much of the item's identity is carried, and in what order? -> A:
  Twenty-four bits, one byte per square, most significant byte first, so the
  squares read left to right in the same order the number is written. Twenty-four
  bits covers the live identity range with headroom. Most-significant-first is
  chosen over the reverse because the ordering must be stated identically on
  both sides of the contract and the version that matches how the value is
  written down is the one an author is least likely to get backwards.
- Q: What do the identity squares carry when the quickslot is empty or holds
  something that is not a potion? -> A: Zero, and the companion reports no
  identity in that case regardless of what they carry, because the cooldown
  square has already said there is nothing there. The identity squares keep
  their marks and their integrity checks in that state rather than going dark,
  so absence continues to mean only that the addon is too old to draw them.
- Q: Do the four values travel to the rest of the application as four
  announcements or as one? -> A: One, carrying the whole quickslot state. This
  follows the resource squares and the cooldown squares, whose rationale is
  recorded in the code: the values are read from one sample and describe one
  thing, and a swap changes all four at once. Four announcements for one swap
  would be four log entries for one event.
- Q: Does this feature change how often the grid is sampled? -> A: No. Nothing
  consumes the values, so nothing has a latency requirement, and the existing
  cadence already samples fast whenever the application can intercept.
- Q: Where does the operator see this? -> A: Beside the existing decoded
  readouts in the status region, not on the skills grid. The quickslot is not
  one of the skill slots that grid lists, and adding a row to it for something
  that is not a skill would misstate what the grid is. The status region is
  where the combat, menu, movement, and resource readouts already sit, and it is
  the region that grew for each of them.
- Q: How many outcomes does the decoded quickslot actually have, given that
  "there is no potion" and "the squares could not be read" were both being
  called by their own names? -> A: One outcome, not two. Both mean "there is
  nothing here to act on", and the companion already settled this exact question
  for the skill cooldowns one feature ago, where an empty slot, a slot the game
  reports no cooldown for, and an unreadable square all report the same single
  value. Naming them apart here would invent a distinction the transport cannot
  carry (the reserved payload and an absent square are indistinguishable by
  construction) and would tempt the consumer to branch on it. The requirements
  below use one name throughout.
- Q: What shape does the decoded value take, given the issue proposed a flag, a
  cooldown, and an identity side by side? -> A: The cooldown reuses the existing
  three-case cooldown value the skill squares already decode to, and the identity
  is carried beside it as present-or-absent. Whether a potion is there is then
  derived from the cooldown not being the unknown case rather than stored
  separately. The issue's three-field shape is equivalent but admits states that
  cannot exist, such as "no potion" carrying a remaining time, or a flag saying
  potion while the cooldown says unknown. Making those unrepresentable is worth
  more than matching the issue's wording, and reusing the cooldown value means
  the quantization, the saturation rule, and the ready case are shared with the
  skill squares by construction rather than by a parallel implementation that
  can drift.
- Q: How is the item's identity shown to the operator, given the companion has
  no way to turn an identifier into a name? -> A: As the number itself, with no
  name lookup. The companion would need a bundled item table to show a name, and
  that table would be locale-dependent, would go stale every patch, and would be
  a large data dependency added for a readout with no consumer. The number is
  enough for the two things the readout is for: confirming the signal is live and
  correct in the field, and seeing that a swap was noticed.
- Q: Where is the overlay's footprint reported? -> A: In two places, because two
  different questions are being asked. Beside the square-size setting, as a
  derived caption, because that is where an operator stands when they want the
  overlay smaller and the extent is a direct function of the value they are
  editing. And in the application log when it changes, at the debug level,
  alongside the existing grid-fit report, because that is the record that
  explains a field report after the fact. Neither is a new region or a new
  window.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - The operator can see what is in the quickslot and whether it is ready (Priority: P1)

The operator runs the application alongside the game with the current
PixelBeacon installed. The application shows whether the active quickslot holds
a potion, which one, and whether it is ready or still cooling down. Drinking the
potion makes the readout show a cooldown that counts down and then clears.
Switching to a different quickslot, or slotting a different potion, changes the
identity shown, which appears as the game's own number for the item rather than
its name. The values survive a loading screen.

**Why this priority**: This is the feature. Every other story exists to keep this
one honest.

**Independent Test**: With the game running and the addon installed, drink a
quickslotted potion and confirm the application reports a cooldown that decreases
and then reports ready. Swap to a different potion and confirm the reported
identity changes. Move to a quickslot holding something that is not a potion and
confirm the application reports unknown rather than a cooldown.

**Acceptance Scenarios**:

1. **Given** the addon is installed and the application is reading the grid,
   **When** the operator drinks the quickslotted potion, **Then** the application
   reports a remaining time greater than zero without the operator taking any
   action.
2. **Given** the quickslot is counting down, **When** its cooldown expires,
   **Then** the application reports ready.
3. **Given** a potion is in the active quickslot, **When** the operator switches
   to a quickslot holding a different potion, **Then** the reported identity
   changes to the new one.
4. **Given** the active quickslot is empty or holds something that is not a
   potion, **When** the application samples the grid, **Then** it reports the
   quickslot as unknown, with no cooldown and no identity.
5. **Given** the quickslot state is known, **When** a loading screen completes,
   **Then** the reported state agrees with the game rather than with the state
   from before the load.
6. **Given** nothing about the quickslot has changed, **When** the application
   samples the grid repeatedly, **Then** it reports no changes and produces no
   repeated log entries.

---

### User Story 2 - An out-of-date addon never produces a false reading (Priority: P2)

The operator updates the application but has not yet updated the addon, so the
game is still drawing the previous sixteen-square grid. The application samples
where the four new squares would be and finds whatever the game happens to be
drawing there. For the first time that content is not merely elsewhere on the
same row, it is a strip of the game's interface one full square below the grid,
which the beacon has never occupied. It must all be treated as "no quickslot
information available" and never as a quickslot state.

**Why this priority**: A wrong reading is worse than a missing one, and this is
the failure mode most likely to occur in the field, because the application and
the addon update independently. The consumer built on top of this signal
synthesizes a keypress, so a false reading here becomes a false action one
feature from now.

**Independent Test**: Point the reader at a grid that has no new squares and
confirm the quickslot reports unknown, for arbitrary colors behind each of the
four positions.

**Acceptance Scenarios**:

1. **Given** an addon version that draws none of the new squares, **When** the
   application samples the grid, **Then** the quickslot reports unknown and no
   quickslot change is announced.
2. **Given** an arbitrary color at any new square's position that is not a valid
   encoding for that square, **When** the application samples it, **Then** the
   result is unknown rather than a guess.
3. **Given** the application has been reporting a quickslot state, **When** the
   signal from the addon is lost entirely, **Then** the quickslot is cleared to
   unknown rather than left showing the last value.
4. **Given** a color valid for one of the four squares, **When** it appears at
   another of the four positions, **Then** it does not decode, so a geometry
   error cannot silently report one square's payload as another's.
5. **Given** the cooldown square decodes but one identity square does not,
   **When** the application reports the state, **Then** it reports no identity
   rather than an identity assembled from partial bytes.

---

### User Story 3 - The grid crosses onto a second row without disturbing any existing signal (Priority: P2)

The grid grows from sixteen squares to twenty. The first row stays exactly as it
was, and four squares appear on a second row beneath it. Every pre-existing
signal keeps its position and its meaning, the captured region becomes two rows
tall and no wider, and the checks that assumed one row are updated to state what
is now true.

**Why this priority**: It ranks alongside the false-reading story rather than
below it because a defect in the grid's extent would corrupt every signal at
once, not just the new ones. This is the first shipping block count that crosses
the boundary, so it is the first time the wrap is exercised by what ships rather
than only by tests that construct it.

**Independent Test**: Confirm the captured region is exactly one full row wide
and exactly two rows tall, that every pre-existing square keeps the exact
position it had at sixteen squares, that the four new squares occupy the first
four positions of the second row, and that the overflow warning still reports
correctly when the taller grid does not fit.

**Acceptance Scenarios**:

1. **Given** the block count now exceeds the column count, **When** the captured
   region is computed, **Then** it is exactly one full row wide and exactly two
   rows tall.
2. **Given** the grid has grown, **When** any pre-existing square's position is
   computed, **Then** it is unchanged from before this feature.
3. **Given** the four new squares, **When** their positions are computed,
   **Then** they are the first four positions of the second row, in index order.
4. **Given** a client area too short for the two-row grid, **When** the
   application checks the fit, **Then** it reports the overflow, and the
   application still samples.
5. **Given** the block count has crossed the column count, **When** the automated
   checks run, **Then** the assertion that guarded the boundary has been replaced
   by one stating the two-row shape, rather than relaxed or removed.

---

### User Story 4 - The operator can find out what the overlay now covers (Priority: P3)

The overlay in the corner of the game client is twice as tall as it was. The
operator can learn its exact footprint from the application and from the
documentation, and can make it smaller with the setting that already exists,
without either side of the contract disagreeing with the other.

**Why this priority**: Nothing breaks if this is missing, which is why it is not
P1. It is included because the alternative is an operator measuring their own
screen to answer a question the application already knows the answer to, and
because a doubled footprint that arrives unannounced reads as a defect.

**Independent Test**: Read the reported grid extent and confirm it matches the
square size in effect and the number of squares; change the square size and
confirm the reported extent follows.

**Acceptance Scenarios**:

1. **Given** the application is running, **When** the operator looks for the
   overlay's footprint, **Then** the current extent is reported in squares and in
   physical pixels.
2. **Given** the operator changes the square size, **When** the extent is
   reported again, **Then** it reflects the new size.
3. **Given** the operator wants a smaller overlay, **When** they consult the
   documentation, **Then** it states the footprint and names the square-size
   setting as the way to reduce it.

---

### Edge Cases

- **The addon is older than the application.** Covered by User Story 2: the new
  squares are absent and must read as unknown. This is the first feature where
  the absent squares are sampled from a region of the screen the beacon has
  never drawn on at all.
- **The application is older than the addon.** The addon draws squares the
  application does not sample. They are ignored and no existing signal is
  disturbed, because each square is read at its own position.
- **The active quickslot is empty.** Reported as unknown, the same as a quickslot
  holding something that is not a potion and the same as an unreadable square.
  All mean "nothing to drink", and distinguishing them would imply a precision
  the signal does not have and that no consumer has asked for.
- **The quickslot holds a non-potion consumable, such as food or a siege
  weapon.** Reported as unknown. The consumer must not fire the key at it.
- **The game reports no cooldown information for a slotted potion.** Reported as
  unknown, because the consumer's precondition is "safe to drink now" and a
  potion whose cooldown cannot be read does not meet it.
- **A cooldown is longer than the encodable range.** Reported as the maximum
  encodable value until it falls inside the range, so a long cooldown reads as
  "at least this long" rather than wrapping to a small number.
- **A cooldown is shorter than the sampling interval.** It may never be observed.
  Inherent to sampling a screen signal and accepted: the value is a level, not an
  event.
- **The signal is lost while the quickslot is on cooldown.** The whole state
  clears to unknown rather than counting down from a stale value.
- **The addon is downgraded or reloaded mid-session.** The beacon stays alive but
  the new squares stop being drawn; the quickslot clears to unknown on the next
  sample.
- **The operator swaps the potion for a different one while it is on cooldown.**
  The identity changes and the cooldown continues to report what the game
  reports for the newly slotted item. The companion carries both from the same
  sample, so the two are never from different moments.
- **A very small square size is configured.** Every new square's position derives
  from the configured size and the shared column count, as all existing squares
  do, so no separate adjustment is needed at any supported size, on either row.
- **The taller grid does not fit the client area.** The existing overflow report
  covers it, and now for the first time it can be triggered by height rather than
  only by width. The application still samples, because the squares that do fit
  still decode and refusing to sample would turn a partial loss into a total one.
- **Something else on screen is drawing at a new square's position.** Each square
  carries a validity mark and an integrity check, so an arbitrary color is
  rejected.

## Requirements *(mandatory)*

### Functional Requirements

**The signal**

- **FR-001**: The addon MUST publish the active quickslot's remaining cooldown as
  one square, using the same quantization, range, and saturation rule as the
  skill cooldown squares, so the two carry the same kind of value the same way.
  The square MUST be drawn whenever the addon is loaded and rendering, and MUST
  NOT be hidden to express a state, so that absence means only that the addon is
  too old to draw it.
- **FR-002**: That square MUST distinguish three cases to the companion: a
  remaining time, ready, and unknown. Unknown MUST cover an empty quickslot, a
  quickslot holding an item that is not a potion, a slotted potion the game
  reports no cooldown information for, and a square that cannot be read. These
  are one outcome and MUST NOT be reported apart from each other, because the
  reserved payload and an absent square are indistinguishable by construction.
  The unknown case MUST be carried by a reserved payload value on that square
  rather than by a square of its own.
- **FR-003**: The addon MUST publish the active quickslot item's identity across
  three further squares, one byte each, most significant byte first, covering
  twenty-four bits. Each MUST carry its own validity mark and its own integrity
  check. When the cooldown square carries the unknown payload, they MUST carry
  zero and MUST still be drawn. An identity larger than twenty-four bits MUST be
  reduced to twenty-four deterministically, on the publishing side, so that every
  square always carries a whole byte. It MUST NOT be allowed to produce a byte
  outside the encodable range, because that would fail an integrity check and
  report the whole quickslot as unknown, turning "an identity this feature cannot
  name" into "there is no potion here", which is a different and much worse
  claim.
- **FR-004**: The addon MUST re-establish all four published values from the
  game's current state after each loading screen, and MUST update them when the
  active quickslot changes, when its contents change, and on the existing
  periodic tick as a backstop, so a change that arrives by no event is still
  observed.
- **FR-005**: The addon MUST only redraw a square when its value actually
  changes, so a steady state produces a steady signal.
- **FR-006**: Each new square MUST carry a validity mark distinct from every
  other mark on the grid, so no other square's color and no unrelated screen
  content can be decoded as quickslot state, and so a color valid for one of the
  four cannot decode at another's position. Separation is measured against the
  reader's default per-channel color-match tolerance, which is the fixed
  reference for this requirement. Every new mark MUST be added to the shared
  registry of marks in use, so the separation is proven by the existing automated
  check rather than asserted by the author.
- **FR-007**: The companion MUST decode the four squares into a single quickslot
  state, carrying the remaining cooldown as the same three-case value the skill
  cooldown squares already decode to and the item identity as present or absent,
  returning unknown whenever a validity mark does not match or an encoding fails
  its integrity check. Whether a potion is present MUST be derived from the
  cooldown not being unknown rather than stored as a separate field, so a state
  claiming a potion with no cooldown, or a cooldown with no potion, cannot be
  represented at all.
- **FR-008**: The companion MUST report no identity whenever any of the three
  identity squares fails to decode, rather than assembling an identity from the
  bytes that did, and MUST report no identity whenever the cooldown square
  reports unknown, regardless of what the identity squares carry.
- **FR-009**: The companion MUST announce the quickslot state only when it
  changes, and MUST clear it to unknown when the beacon signal is lost, and also
  on any sample where a square is present but does not decode, rather than
  holding the last known value. This follows the decision recorded for the combat
  block.
- **FR-010**: The companion MUST carry the whole quickslot state in a single
  announcement rather than one per square, following the resource and cooldown
  squares, so a swap that changes every value is one change rather than four.
- **FR-011**: The companion MUST record quickslot changes in the application log
  at the debug level, as one entry per changed sample.
- **FR-012**: The companion MUST present the current quickslot state beside the
  existing decoded readouts in the status region, showing the item identity as
  its numeric value with no name lookup, and MUST render the unknown case in the
  same muted treatment the existing decoded readouts use. It MUST also define
  what is shown in the partial state where the cooldown decoded but the identity
  did not, which is reachable whenever exactly one identity square is disturbed:
  the cooldown MUST be shown as decoded and the identity MUST be shown as
  unknown, rather than the whole readout collapsing to unknown. Collapsing it
  would discard a value that was read correctly, and would make a one-square
  disturbance look identical to a missing addon.

**The row crossing**

- **FR-013**: The grid MUST end this feature occupying exactly two rows, with the
  first row full and the second holding the four new squares. The captured region
  MUST be exactly one full row wide and exactly two rows tall. This MUST be
  asserted rather than assumed.
- **FR-014**: Every square that existed before this feature MUST keep the exact
  position it had, so no existing signal moves.
- **FR-015**: The compile-time assertion that the block count does not exceed the
  column count MUST be replaced by one stating the two-row shape that now ships.
  It MUST NOT be relaxed to a larger bound, widened to accommodate a future
  count, or removed. Replacing it is the point: it was left in place by the
  previous feature to fail here, and what succeeds it must be equally specific
  about what is now true.
- **FR-016**: Every expectation elsewhere in the automated checks that was
  written when a single row was the only shipping possibility MUST be found and
  updated to state the two-row shape. None may be left passing by coincidence,
  and none may be weakened to accommodate the new count.
- **FR-017**: The existing overflow report MUST continue to detect a grid that
  does not fit the client area, including a grid that fits horizontally but not
  vertically, which this feature makes reachable for the first time.

**The overlay footprint**

- **FR-018**: The application MUST report the beacon grid's current extent, in
  squares and in physical pixels, so the operator can learn the overlay's
  footprint from the application rather than by measuring their screen. It MUST
  do so in both of the places the two distinct questions are asked: as a derived
  caption beside the square-size setting, which is where an operator stands when
  they want the overlay smaller, and in the application log at the debug level
  whenever the extent changes, which is the record that explains a field report
  after the fact. Neither MUST introduce a new region or a new window.
- **FR-019**: The operator documentation MUST state the overlay's footprint at
  the default square size and name the square-size setting as the way to reduce
  it. This feature MUST NOT move the overlay's anchor or change the square size
  automatically.

**Compatibility and geometry**

- **FR-020**: The companion MUST treat every new square as optional: an addon
  that does not draw them MUST yield unknown and MUST NOT disturb any existing
  signal.
- **FR-021**: Each new square's sampled position MUST derive from the configured
  square size and the shared column count using the same rule as every other
  square, with no special case for the second row.
- **FR-022**: The number of squares MUST remain stated exactly once on each side
  of the contract, with every dependent geometry derived from it, and the
  existing automated cross-language check MUST be extended to cover the new count
  and every new mark and encoding constant, so a disagreement between the two
  sides cannot reach a release.
- **FR-023**: The addon manifest version MUST advance so the application's beacon
  manager offers the update, and the manifest description MUST name the new
  signal.

**Boundaries**

- **FR-024**: This feature MUST NOT change any behavior based on quickslot
  values. Weave timing, key synthesis, input interception, and the fishing
  controller behave exactly as they do today. Nothing drinks a potion.
- **FR-025**: This feature MUST NOT add, remove, or alter any input path, and
  MUST leave the existing safety behaviors untouched: input suppression stays
  scoped to the focused game window, synthesized input stays flagged against
  recursion, the hook thread stays free of blocking work, and fishing still
  degrades to disabled when the beacon signal is lost.
- **FR-026**: The master specification's pixel-bus section MUST document the new
  squares and the two-row grid, and the changelog MUST record the feature plus a
  dated decision for each contract the following slice inherits.
- **FR-027**: This feature MUST NOT change the grid sampling cadence.

### Key Entities

- **Quickslot state**: What the game reports about the active quickslot, as seen
  by the companion: how long until it can be used again, and which item it is.
  Whether it holds a usable potion is not carried separately; it is exactly the
  cooldown not being unknown. Unknown is the single outcome covering an empty
  quickslot, a non-potion item, a potion with no readable cooldown, and a square
  the companion could not read.
- **Item identity**: The game's own numeric identifier for the slotted item.
  Carried so a consumer can tell one potion from another and notice a swap. It is
  not what the potion restores, which the game does not expose as data.
- **Beacon grid**: The ordered run of squares the addon draws and the companion
  samples, wrapped at a fixed shared column count. This feature makes it twenty
  squares occupying two rows.
- **Validity mark**: The part of a square's color identifying which signal it
  carries. Each encoding square has one, and they must stay far enough apart that
  the reader's tolerance cannot confuse two squares or mistake unrelated screen
  content for a signal.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: With the game running and the current addon installed, drinking the
  quickslotted potion is reflected as a non-zero remaining time within one
  sampling interval, on 100 percent of uses across a validation run of at least
  ten uses.
- **SC-002**: Across a validation run in which the quickslot does not change,
  the application reports zero quickslot changes.
- **SC-003**: A quickslot holding a usable potion is reported as a potion in 100
  percent of samples, and every case that is not one (an empty quickslot, a
  non-potion item, a potion the game reports no cooldown for) is reported as
  unknown, with none of them ever reported as a potion. The three non-potion
  cases are deliberately one outcome and are not required to be distinguishable.
- **SC-004**: With an addon that does not draw the new squares, the quickslot
  reports unknown and zero quickslot changes are reported, for every color that
  can appear behind any of the four positions.
- **SC-005**: All sixteen signals that exist today behave identically before and
  after this feature, and every pre-existing square occupies the identical
  position, verified by the existing automated checks continuing to pass
  unchanged in intent.
- **SC-006**: The captured region is proven to be exactly one full row wide and
  exactly two rows tall, the four new squares are proven to be the first four
  positions of the second row, and the two sides of the contract are proven to
  agree on every new constant automatically.
- **SC-007**: A color valid for any one of the four new squares decodes as
  unknown at every other new square's position, in 100 percent of cross-position
  combinations.
- **SC-008**: A quickslot state with any one identity square unreadable reports
  no identity, in 100 percent of cases, and never a partially assembled one.
- **SC-009**: The reported grid extent matches the square size in effect and the
  block count, at every supported square size.
- **SC-010**: The full merge gate (formatting, lint at deny-warnings, and the
  whole test suite) passes with no test weakened, skipped, or made conditional.

## Assumptions

- **The game exposes the active quickslot, its contents, the item type, the item
  identity, and the on-use cooldown directly.** All five are verified present and
  unprotected, and the cooldown call returns both the total and the remaining
  value, so no timer needs to be reconstructed by the addon.
- **A quickslot cooldown is a level, not an event stream.** The companion cares
  what the remaining time is now, not how many times the potion has been drunk.
  This is what makes a sampled screen signal an adequate transport.
- **Twenty-four bits covers the live item identity range with headroom.** If it
  ever does not, an identity beyond the range would alias to a different one; the
  consumer uses identity for display and swap detection rather than for a safety
  decision, so the failure would be visible rather than dangerous.
- **The operator updates the addon through the application.** The manifest
  version bump is the mechanism by which they are offered the update; an operator
  who declines is covered by User Story 2.
- **No consumer of quickslot values exists yet.** The observable is added on the
  expectation that the following feature acts on it. Adding it without a consumer
  is deliberate: it makes the signal verifiable in the field before anything
  depends on it being correct.
- **The grid wrap is correct and merely unexercised by what ships.** The geometry
  was built for multiple rows and is covered by tests that construct multi-row
  cases directly; what has never happened is a shipping block count that crosses
  the boundary. This feature is that count.

## Dependencies

- The bundled PixelBeacon addon and its manifest, which the application installs,
  updates, and removes.
- The companion's beacon grid reader, its configured square size, and the shared
  column count, which already single-source block geometry, and the grid-wrap
  geometry that has carried multi-row support since it was built.
- The application's event routing and interface view model, which already carry
  decoded signals from the reader to the operator, and the status region of the
  interface, which already lists the decoded readouts.
- The existing cross-language check that parses the embedded addon source to
  prove the two sides of the contract agree.
- The out-of-band display detection, which supplies the measured client area the
  overflow report compares the grid extent against.
- The master specification's pixel-bus section, which is the architecture of
  record for the grid contract. It is stale at v0.2.0 overall and scheduled for a
  separate refresh; this feature updates the section it touches rather than
  waiting on that refresh.
