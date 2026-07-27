# Feature Specification: PixelBeacon Movement-State Block

**Feature Branch**: `036-movement-state-block`

**Created**: 2026-07-27

**Status**: Draft

**Input**: GitHub issue #11 (add mount/sprint movement-state block to
PixelBeacon). Build plan `docs/plans/plan-010.md`, slice 036. Master
specification section 10.3 (the pixel-bus block contract).

## Overview

The bundled PixelBeacon addon publishes a small grid of colored squares at the
top-left of the game client, and the companion samples that grid to learn things
about the game session it cannot observe from outside. Today the grid carries
nine signals: the addon is loaded, a fishing cast is active, the server latency,
which weapon bar is drawn and with what weapons, whether the player is in combat,
which native menu or text field is open, and the player's health, stamina, and
magicka. It does not carry whether the player is mounted.

This feature adds that signal. Mounted state becomes a tenth square, the
companion decodes it, and the operator can see it change in the running
application. Nothing acts on it: this feature adds an observable and
deliberately stops there, exactly as the combat block did.

This is the last slice of build plan 010, and it is the only one that was
blocked. Issue #11 proposed one block covering two axes, mounted and sprinting,
and made a verification a blocking entry condition: confirm the real sprint
observable before any encoding is fixed, and if sprint proves unreliable or
inconsistent between the keyboard and gamepad interfaces, ship the mounted axis
alone rather than encode a flaky signal into a contract shared byte for byte
between two codebases. That verification is now complete and is recorded below.
It found no sprint observable at all, so this feature ships the mounted axis and
sprint becomes a separate follow-up. The encoding still reserves room for
sprint, so adding it later costs a code rather than a second square.

A note on what "user" means here. This application has one user, the operator
running it beside the game. Everything below is written from what that operator
can observe, except where a requirement is about the contract between the addon
and the companion, which the operator experiences only as the signal being right
or wrong.

## The sprint verification (the slice's entry condition)

Issue #11 required this before any encoding could be fixed. It was run against
two independent sources and is conclusive: **the game exposes no sprint state to
an addon.**

Source one, the indexed ESO API database: zero sprint functions, zero sprint
events, and zero sprint constants. `IsUnitSprinting`, `IsPlayerSprinting`, and
`EVENT_SPRINT_STATE_CHANGED` each return no result.

Source two, a direct search of the live `esoui/esoui` interface source (the tool
issue #11 named, whose rate limit cut the original attempt short): the same three
names return zero hits, and a search for `Sprint` anywhere in the source returns
exactly four references, none of which is readable state:

| Reference | What it actually is |
| --- | --- |
| `SPECIAL_MOVE_SPRINT`, `GAMEPAD_SPECIAL_MOVE_SPRINT` in `bindings.xml` | Keybind actions that call into the engine (`OnSpecialMoveKeyDown(SPECIAL_MOVE_INDEX_SPRINT)`). An input binding, exposing no queryable state. |
| `IN_WORLD_UI_SETTING_TOGGLE_SPRINT` in the gameplay options panel | A user preference (hold versus toggle), not runtime state. |
| `globalvars.lua` | A `sprintf` false positive. |

The same evidence also settles the issue's second stated risk. The bindings file
comments that sprint is *toggled* on the gamepad while the keyboard path is a
hold, and the toggle-sprint preference lets a keyboard operator opt into toggle
as well. Any heuristic reconstruction of sprint would therefore have to model
three different input semantics and would still be a guess. Issue #11's own
fallback clause is triggered, and this feature takes it.

The two names the mounted axis needs are verified present and carry no open
question, which is why the mounted axis proceeds unchanged.

## Clarifications

### Session 2026-07-27

Answered under the build-phase autopilot decision policy from the constitution,
build plan 010, GitHub issue #11, and the existing beacon code (in particular the
combat and menu squares, the closest siblings). None were escalated.

- Q: Sprint has no observable. Ship mounted alone, or hold the whole slice until
  sprint can be reconstructed? -> A: Ship mounted alone. Issue #11 names this
  outcome explicitly as the sanctioned fallback, and build plan 010 orders this
  slice last precisely so an unresolved sprint could not block the other four.
  Holding the slice would trade a working, verified signal for an indefinite wait
  on a signal the game does not expose.
- Q: Does the encoding stay a code table, or collapse to a boolean now that only
  one axis ships? -> A: A code table with a reserved position for the sprint
  axis. The cost is zero (the same single square, the same single marker), and it
  means a future sprint feature adds a code rather than claiming an eleventh
  square and an eleventh marker. Issue #11 preferred one block over two for
  exactly this reason and that reasoning survives the axis being deferred.
- Q: Does the reserved sprint position get drawn as a third state today? -> A:
  No. The addon never emits a sprinting code in this feature, and the companion
  treats any code it does not recognize as unavailable, not as sprinting. The
  reservation is a documented hole in the table, not a state the operator can
  reach. This keeps the "unavailable means the companion could not read the
  signal" definition true.
- Q: Hold the last mounted state when the square is present but does not decode,
  or clear it? -> A: Clear to unavailable on every sample that does not decode,
  following the combat block's decision rather than the weapon block's hold. That
  decision was recorded as a dated decision in slice 031 for later slices to
  inherit, and this is a later slice inheriting it. A stale "mounted" persisting
  after the square stops being drawn is the false reading User Story 2 exists to
  prevent.
- Q: Can the addon hide the mounted square when the player is on foot, the way
  the fishing square hides when idle? -> A: No. It renders whenever the status
  square renders, following the latency, weapon, combat, menu, and resource
  squares. If it could hide, absence would be ambiguous between "on foot" and "an
  addon too old to draw it", and User Story 2 depends on absence meaning only the
  latter.
- Q: Does the tenth square change the grid's shape or the region captured? -> A:
  No. The grid wraps at a fixed sixteen columns, so a tenth block is the tenth
  cell of the first row and the captured region is unchanged in height. This must
  be asserted rather than assumed, because it is exactly the kind of invariant
  that stops being true silently at the sixteenth block.
- Q: Does this feature change the sampling cadence so a mount transition is seen
  sooner? -> A: No, matching the combat block. Nothing consumes the value, so
  nothing has a latency requirement, and the existing cadence already samples
  fast whenever the application can intercept.
- Q: Is the signal named for the axis that ships (mounted) or for the concept
  that will grow (movement)? -> A: Movement, everywhere the name is a contract:
  the square, the decoded value, the announced change, and the operator-facing
  label. The values are mounted and on foot. Naming it for the shipping axis
  would force a rename of every one of those surfaces when sprint arrives, which
  is precisely the cost FR-011 exists to avoid; the name is the cheapest half of
  the reservation. "Mounted" survives only as a value, never as the signal's
  name.
- Q: Is the reserved sprint position an explicitly defined code on both sides of
  the contract, or just "anything unrecognized decodes as unavailable"? -> A:
  Documented and rejection-tested on the companion side; NOT defined as a
  constant in the addon. A constant the addon never emits is dead code, and the
  cross-language check that parses the addon source to prove the two sides agree
  would have to grow a special case for a value that exists on only one side.
  The reservation's whole purpose is to stop a future feature choosing a
  colliding code, and a documented, tested rejection on the reading side
  achieves that without putting an unreachable branch in the addon.
- Q: When is the sprint follow-up issue filed? -> A: At the authorization halt,
  not during implementation. Filing an issue is an outward-facing action on the
  operator's repository, so it is presented for approval alongside the push
  rather than taken unilaterally mid-slice. The deferral itself is already
  durably recorded in this spec and the changelog, so nothing is lost if the
  operator prefers to file it differently or not at all.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - The operator can see mounted state in the companion (Priority: P1)

The operator runs the application alongside the game with the current PixelBeacon
installed. When their character mounts, the application reflects that within its
normal sampling interval; when they dismount, it reflects that too. The state
survives a loading screen: after zoning while mounted, the application shows the
true state rather than whatever was true before the load.

**Why this priority**: This is the feature. Every other story exists to keep this
one honest.

**Independent Test**: With the game running and the addon installed, mount and
dismount and confirm the application follows on both transitions, then take a
loading screen while mounted and confirm the application agrees afterwards.
Fully testable on its own; delivers the whole user-visible value.

**Acceptance Scenarios**:

1. **Given** the addon is installed and the application is reading the grid,
   **When** the character mounts, **Then** the application reports the mounted
   state without the operator taking any action.
2. **Given** the character is mounted and the application reports it, **When**
   they dismount, **Then** the application reports the on-foot state.
3. **Given** the character is in a known mounted state, **When** a loading screen
   completes, **Then** the application reports the state that is actually true
   after the load, not the state from before it.
4. **Given** the character's mounted state has not changed, **When** the
   application samples the grid repeatedly, **Then** it reports no change and
   produces no repeated log entries.

---

### User Story 2 - An out-of-date addon never produces a false reading (Priority: P2)

The operator updates the application but has not yet updated the addon, so the
game is still drawing the previous nine-square grid. The application samples
where the tenth square would be and finds whatever the game happens to be drawing
there. It must treat that as "no movement information available" and never as a
movement state.

**Why this priority**: A wrong reading is worse than a missing one, and this is
the failure mode most likely to occur in the field, because the application and
the addon update independently. Every block-adding slice before this one set the
precedent and this feature must match it.

**Independent Test**: Point the reader at a grid that has no tenth square and
confirm the reported state is the unavailable one, for arbitrary colors behind
the square's position. Testable at the desk with no game.

**Acceptance Scenarios**:

1. **Given** an addon version that draws no tenth square, **When** the
   application samples the grid, **Then** movement state reports as unavailable
   and no movement change is announced.
2. **Given** an arbitrary color at the tenth square's position that is not a
   valid movement encoding, **When** the application samples it, **Then** the
   result is the unavailable state rather than a guess.
3. **Given** the application has been reporting a movement state, **When** the
   signal from the addon is lost entirely, **Then** the movement state is cleared
   to unavailable rather than left showing the last value indefinitely.
4. **Given** a code in the movement square that this feature reserves but never
   emits, **When** the application samples it, **Then** the result is unavailable
   rather than a state the operator can reach.

---

### User Story 3 - Adding sprint later costs a code, not a square (Priority: P3)

A developer picking up the deferred sprint axis, once the game exposes it or a
reliable observable is found, adds it to the existing movement square's code
table. They do not have to claim an eleventh square, choose an eleventh validity
mark, or widen the grid.

**Why this priority**: It is invisible to the operator, so it cannot rank above
the signal itself. It is the reason the encoding stays a table rather than
collapsing to a boolean, and it preserves the whole point of issue #11's
one-block-two-axes proposal after one axis was deferred.

**Independent Test**: Confirm the movement encoding has a documented, reserved
position for the sprint axis that the companion rejects today, and that adding it
requires no new square, mark, or geometry change. Testable at the desk.

**Acceptance Scenarios**:

1. **Given** the movement code table, **When** a developer reads it, **Then** the
   sprint axis has a documented reserved position and the reason it is unused.
2. **Given** the reserved position is never emitted by this feature, **When** the
   companion decodes it, **Then** it yields unavailable, so no operator can
   observe a half-built state.

---

### Edge Cases

- **The addon is older than the application.** Covered by User Story 2: the tenth
  square is absent and must read as unavailable, never as a state.
- **The application is older than the addon.** The addon draws a square the
  application does not sample. The extra square is ignored and no existing signal
  is disturbed, because each square is read at its own position.
- **The signal is lost while mounted.** The last reported state must not persist
  as though it were still true; it clears to unavailable.
- **The addon is downgraded or reloaded mid-session.** The beacon stays alive but
  the movement square stops being drawn. The state clears to unavailable on the
  next sample rather than holding the last value.
- **A single sample misreads the square.** The state flaps to unavailable and
  back on the following sample. Accepted, because nothing consumes the value; a
  consumer that cannot tolerate the flap adds its own hysteresis.
- **The player mounts and dismounts between two samples.** The transition is not
  observed. Inherent to sampling a screen signal and accepted: the state is a
  level, not an event, so the next sample reports the truth.
- **The player is mounted while a menu is open, in combat, or fishing.** The
  squares are independent; the movement square is read at its own position and
  carries its own mark, so no combination of other states can change it.
- **A very small square size is configured.** Block geometry already derives from
  the configured square size, and the new square must derive from the same source
  rather than assume the default.
- **The tenth square lands outside the game's client area.** Already handled by
  the existing extent warning, which this feature must not regress. At ten blocks
  in a sixteen-column grid the extent is unchanged, which is asserted rather than
  assumed.
- **Something else on screen is drawing at the grid's position.** The square
  carries a validity mark, so an arbitrary color is rejected as unavailable
  rather than decoded. The mark must be far enough from every other mark on the
  grid that the reader's color-match tolerance can never confuse them.
- **The game is not running or the grid is not visible.** No change from today:
  the whole grid reads as absent and movement state is unavailable.

## Requirements *(mandatory)*

### Functional Requirements

**The signal**

- **FR-001**: The addon MUST publish the player's movement state as a dedicated
  square on the beacon grid, the only axis in this feature being whether the
  player is mounted. The square MUST be drawn whenever the addon is loaded and
  rendering, and MUST NOT be hidden to express a state, so that its absence means
  only that the addon is too old to draw it.
- **FR-002**: The published state MUST distinguish exactly three cases to the
  companion: mounted, on foot, and unavailable.
- **FR-003**: The addon MUST update the published state on the game's own mounted
  transition notification, so the square changes at the moment the player mounts
  or dismounts rather than at the next periodic refresh.
- **FR-004**: The addon MUST re-establish the published state from the game's
  current value after each loading screen, because a transition notification may
  not fire for the state that is already true when the world loads.
- **FR-005**: The addon MUST only redraw the square when the state actually
  changes, so a steady state produces a steady signal.
- **FR-006**: The square MUST carry a validity mark distinct from every other mark
  on the grid, so no other square's color and no unrelated screen content can be
  decoded as a movement state. The encoded states MUST likewise be distinct from
  each other. Both separations are measured against the reader's default
  per-channel color-match tolerance, which is the fixed reference for this
  requirement. The new mark MUST be added to the shared registry of marks already
  in use, so the separation is proven by the existing automated check rather than
  asserted by the author. An operator who raises that tolerance far enough to
  collide encodings degrades every square on the grid equally; that is a
  pre-existing property of the whole beacon design, not something this feature
  introduces, and correcting it is out of scope here.
- **FR-007**: The companion MUST decode the square into the three states of
  FR-002, returning the unavailable state whenever the validity mark does not
  match, the encoding fails its integrity check, or the code is one this feature
  does not emit.
- **FR-008**: The companion MUST announce a movement state only when the decoded
  state changes. It MUST clear the state to unavailable when the beacon signal is
  lost, and also on any sample where the square is present but does not decode,
  rather than holding the last known value. This follows the decision recorded for
  the combat block.
- **FR-009**: The companion MUST record each movement state change in the
  application log at the debug level, matching the existing combat and weapon-bar
  entries.
- **FR-010**: The companion MUST present the current movement state in the
  application interface alongside the existing combat and weapon-bar readouts, and
  MUST render the unavailable case in the same "Not detected" muted treatment
  those readouts already use.

**The deferred sprint axis**

- **FR-011**: The movement encoding MUST reserve a documented position for the
  sprint axis, so a later feature adds sprint without claiming a further square or
  validity mark. Every name this feature introduces as a contract MUST likewise be
  a movement name rather than a mounted one: the square, the decoded value, the
  announced change, and the operator-facing label. "Mounted" is a value of that
  state, never the name of the signal, so adding sprint later renames nothing.
- **FR-012**: The addon MUST NOT emit the reserved sprint encoding, and the
  companion MUST decode it as unavailable rather than as a state. No partially
  built sprint behavior is observable to the operator. The reservation MUST be
  documented and rejection-tested on the companion side and MUST NOT be defined
  as an unused constant in the addon, so the cross-language agreement check needs
  no special case for a value that exists on only one side.
- **FR-013**: The verification that established sprint has no observable MUST be
  recorded in this feature's artifacts with its evidence, because it is the
  deliverable that unblocked the slice and the justification for the reduced
  scope. A follow-up issue for the sprint axis MUST be presented for filing at
  the authorization halt, so the deferral is tracked rather than lost without
  taking an outward-facing action on the operator's repository unilaterally.

**Compatibility and geometry**

- **FR-014**: The companion MUST treat the movement square as optional: an addon
  that does not draw it MUST yield the unavailable state and MUST NOT disturb any
  existing signal.
- **FR-015**: The movement square's sampled position MUST derive from the
  configured square size and the shared column count using the same rule as every
  other square, so neither a non-default square size nor the grid wrap needs a
  separate adjustment.
- **FR-016**: The number of squares MUST remain stated exactly once on each side
  of the contract, with every dependent geometry derived from it, and the existing
  automated cross-language check MUST be extended to cover the new square's count
  and colors so a disagreement between the two sides cannot reach a release.
- **FR-017**: The captured region MUST be verified unchanged by the tenth square.
  The assertion MUST be expressed in terms of what the invariant actually depends
  on, namely that the block count has not exceeded the shared column count, rather
  than in terms of the count happening to be ten. Blocks eleven through sixteen
  keep the region unchanged for the same reason; the seventeenth adds a row and
  changes it. Stating the dependency means the slice that crosses that boundary
  inherits a requirement that already anticipates it, instead of discovering the
  invariant was never about the number ten.
- **FR-018**: The addon manifest version MUST advance so the application's beacon
  manager offers the update to operators running the previous addon, and the
  manifest description MUST name the new signal.

**Boundaries**

- **FR-019**: This feature MUST NOT change any behavior based on movement state.
  Weave timing, key synthesis, input interception, and the fishing controller
  behave exactly as they do today.
- **FR-020**: This feature MUST NOT add, remove, or alter any input path, and MUST
  leave the existing safety behaviors untouched: input suppression stays scoped to
  the focused game window, synthesized input stays flagged against recursion, the
  hook thread stays free of blocking work, and fishing still degrades to disabled
  when the beacon signal is lost.
- **FR-021**: The master specification's pixel-bus section MUST document the new
  square, and the changelog MUST record the feature plus a dated decision for the
  reduced scope and the reserved sprint encoding.
- **FR-022**: This feature MUST NOT change the grid sampling cadence.

### Key Entities

- **Movement state**: What the game reports about whether the player is mounted.
  Three values as seen by the companion: mounted, on foot, and unavailable.
  Unavailable is not a third game state; it means the companion could not read the
  signal.
- **Sprint axis**: The second axis issue #11 proposed, deferred because the game
  exposes no observable for it. Present in this feature only as a reserved,
  never-emitted position in the movement encoding.
- **Beacon grid**: The ordered run of squares the addon draws and the companion
  samples, wrapped at a fixed shared column count. This feature makes it ten
  squares, still one row.
- **Validity mark**: The part of a square's color identifying which signal it
  carries. Each encoding square has one, and they must stay far enough apart that
  the reader's tolerance cannot confuse two squares or mistake unrelated screen
  content for a signal.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: With the game running and the current addon installed, mounting or
  dismounting is reflected by the application within one sampling interval, on 100
  percent of transitions across a validation run of at least ten transitions.
- **SC-002**: Across a validation run in which mounted state does not change, the
  application reports zero movement changes.
- **SC-003**: After a loading screen taken while mounted, the application agrees
  with the game's actual state on 100 percent of attempts.
- **SC-004**: With an addon that does not draw the movement square, the
  application reports movement state as unavailable and reports zero movement
  changes, for every color that can appear behind the square's position.
- **SC-005**: All nine signals that exist today behave identically before and
  after this feature, verified by the existing automated checks continuing to pass
  unchanged in intent.
- **SC-006**: The two sides of the contract are proven to agree on the new square
  automatically, and the captured region is proven unchanged at ten blocks.
- **SC-007**: The reserved sprint encoding decodes as unavailable in 100 percent
  of cases, so no half-built state is reachable.
- **SC-008**: The full merge gate (formatting, lint at deny-warnings, and the
  whole test suite) passes with no test weakened, skipped, or made conditional.

## Assumptions

- **The game exposes mounted state directly and reliably.** The current value and
  a transition notification are both available to an addon and both names are
  verified present. The mounted axis carries no open API question.
- **The game exposes no sprint state at all.** Established by the verification
  recorded above against two independent sources. This is the assumption most
  likely to change over time, since a future game update could add an observable;
  the reserved encoding position is what makes that change cheap.
- **Movement state is a level, not an event stream.** The companion cares what
  the state is now, not how many times it changed. This is what makes a sampled
  screen signal an adequate transport and a missed intermediate transition
  acceptable.
- **The operator updates the addon through the application.** The manifest version
  bump is the mechanism by which they are offered the update; an operator who
  declines is covered by User Story 2.
- **Presenting the state beside the combat and weapon-bar readouts is the right
  surface.** Those are the existing examples of decoded player-state signals shown
  to the operator, so movement follows them rather than inventing a new surface.
- **No consumer of movement state exists yet.** The observable is added on the
  expectation that a later feature acts on it, and adding it without a consumer is
  deliberate: it makes the signal verifiable in the field before anything depends
  on it being correct.
- **The grid has spare capacity.** The wrap introduced in slice 035 removed the
  width ceiling, so a tenth square costs nothing structurally.

## Dependencies

- The bundled PixelBeacon addon and its manifest, which the application installs,
  updates, and removes.
- The companion's beacon grid reader, its configured square size, and the shared
  column count, which already single-source block geometry.
- The application's event routing and interface view model, which already carry
  the combat and weapon-bar signals from the reader to the operator.
- The existing cross-language check that parses the embedded addon source to prove
  the two sides of the contract agree.
- The master specification's pixel-bus section, which is the architecture of
  record for the grid contract. It is stale at v0.2.0 overall and scheduled for a
  separate refresh; this feature updates the section it touches rather than
  waiting on that refresh.
