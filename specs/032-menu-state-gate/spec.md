# Feature Specification: PixelBeacon Menu-State Input Gate

**Feature Branch**: `032-menu-state-gate`

**Created**: 2026-07-27

**Status**: Draft

**Input**: GitHub issue #10 (add native game-menu state block(s) to PixelBeacon to
gate keystroke interception). Build plan `docs/plans/plan-010.md`, slice 032.
Master specification section 10.3 (the pixel-bus block contract) and its
input-suppression scoping.

## Overview

The application intercepts keys while the game window has focus, so pressing a
bound key runs a weave instead of reaching the game. That is correct while the
operator is playing. It is wrong the moment they start typing: opening chat,
composing an in-game mail, renaming an item, or searching the guild store all put
a text field on screen, and the bound keys are ordinary letters. Today the
application has no way to tell, so it keeps intercepting, and the operator either
loses characters or has a weave fire into their sentence.

This feature gives the game a way to say "a UI surface is up". The addon publishes
it as a sixth square on the beacon strip, the companion reads it, and while it is
active the input engine stops intercepting and stops synthesizing. When the
surface closes, normal behavior resumes.

The signal is not free of consequence, which is why the feature is scoped tightly.
It touches the one part of the application the constitution names as sacrosanct:
the decision about which keys are suppressed. The design point that makes this
safe is that the gate can only ever *relax* interception. It is an additional
reason to leave a key alone, never a reason to take one. Focus scoping is
untouched and remains the unconditional first test; the gate narrows what is
intercepted inside a focused game window and can never reach outside one.

The honest limitation, stated up front because it bounds what this feature can
promise: the signal travels by being drawn on screen and sampled, so the
application learns about a surface only after the addon publishes the change and
the companion next samples it. The feature is therefore "the application stops
interfering promptly", not "instantaneously". Both halves of that delay are
constrained by requirements below rather than left to chance, because constraining
only the half you happen to be thinking about is how a latency promise ends up
unmeetable.

## Clarifications

### Session 2026-07-27

Answered under the build-phase autopilot decision policy from the constitution,
build plan 010, GitHub issue #10, the live `esoui/esoui` source, and the existing
input engine. None were escalated.

- Q: Which signal is authoritative for "a UI surface is active"? -> A: The game's
  UI-mode flag, not the addon's existing scene test. This reverses the issue's
  leading suggestion and it is the most consequential answer here. The scene test
  asks whether the gameplay HUD scenes are hidden, and **opening chat does not hide
  them**, so the scene test reads "no menu" while the operator is typing in the
  single most common text field in the game, which is the primary use case. The
  game's own scene manager proves UI mode is the right signal: its
  `ConsiderExitingUIMode` refuses to leave UI mode while chat text entry is open.
  The gate therefore reads UI mode, with the chat text-entry state ORed in
  explicitly as a belt-and-braces guarantee for the primary use case.
- Q: The strip is sampled once a second when not fishing. Is that prompt enough
  for a gate whose purpose is to not disturb typing? -> A: No, and this feature
  raises the cadence. Slice 031 deliberately left the cadence alone and recorded
  that "the first consumer that needs sub-second latency raises it when it exists".
  This is that consumer. The gate is sampled at the existing fast cadence at all
  times, not only while fishing, because a menu can open at any moment. The cost is
  a small screen-strip capture ten times a second instead of once, which the
  application already performs at that rate during every fishing session.
- Q: What happens to a weave sequence already in flight when a surface opens?
  -> A: It runs to completion; the gate stops new weaves from starting and does not
  abort a running one. Aborting mid-sequence risks leaving a synthesized key held
  down, which is a worse failure than a sequence finishing, and the exposure is
  bounded by one sequence's duration. This is recorded as a known limitation rather
  than hidden: combined with the sampling latency, the feature's promise is that
  typing is not disturbed after the gate engages, not that a keystroke already in
  motion is recalled.
- Q: Do the operator's application hotkeys keep working while a surface is up?
  -> A: Yes. The gate behaves exactly like an automatic, game-driven version of the
  existing manual suspend, including its exemption for the toggle hotkeys, so the
  operator can still suspend the application or stop fishing from inside a menu.
  Treating the gate as "suspend, decided by the game" rather than as a new concept
  keeps one behavior to reason about instead of two.
- Q: One block carrying both the gate and the surface detail, or one block each?
  -> A: One block. The issue asks for a mandatory gate plus optional per-surface
  detail; a single code value delivers both, because "which surface" already
  answers "is any surface active". Two blocks would spend an extra square and
  introduce the possibility of the two disagreeing. The gate stays correct for a
  surface nobody enumerated because the addon decides the boolean from UI mode
  first and only then labels it, falling back to a generic code.
- Q: Is the gate something the operator can turn off? -> A: No, it has no setting.
  It is a correctness behavior, not a preference, and an operator who wants the
  application to stop interfering already has the manual suspend. Adding a setting
  would also create a supported configuration in which the stated defect persists.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Typing in game is not disturbed (Priority: P1)

The operator is playing with the application running and bound keys active. They
open chat and type a message, or open their mail and compose one. Their keystrokes
reach the game unchanged, no weave fires, and nothing is swallowed.

**Why this priority**: This is the entire reason the feature exists. Issue #10
opens with it.

**Independent Test**: With the game running and the addon installed, open chat,
type a sentence containing bound keys, and confirm the text arrives intact and no
weave fires. Repeat for a mail composition. Delivers the whole value on its own.

**Acceptance Scenarios**:

1. **Given** the application is intercepting normally, **When** the operator opens
   chat and types characters bound to weaves, **Then** the characters reach the
   game and no weave runs.
2. **Given** a surface is open, **When** the operator presses a bound key, **Then**
   the key passes through untouched rather than being suppressed.
3. **Given** a surface is open, **When** the application would otherwise start a
   weave, **Then** it does not.
4. **Given** a surface is open during an active fishing session, **When** the
   application would otherwise send the interact keypress to cast or reel,
   **Then** it does not, so a reel does not land in the message being typed.
5. **Given** the operator is typing, **When** they use the application's own toggle
   hotkeys, **Then** those still work, because the gate exempts them exactly as the
   manual suspend does.

---

### User Story 2 - Normal play resumes cleanly (Priority: P1)

The operator closes the menu and goes back to fighting. Interception resumes
immediately and weaves work exactly as before. Nothing is left half-suppressed.

**Why this priority**: Equal to User Story 1. A gate that engages but does not
release is worse than no gate, because it silently disables the application's
whole purpose and looks like a crash.

**Independent Test**: Open and close a surface repeatedly and confirm weaves fire
before, do not fire during, and fire again after, with no residue.

**Acceptance Scenarios**:

1. **Given** the gate is active, **When** the surface closes, **Then** interception
   resumes without operator action.
2. **Given** the operator opens and closes surfaces repeatedly, **When** they play
   normally between each, **Then** behavior is identical to the application without
   this feature.
3. **Given** a weave sequence was already running when a surface opened, **When**
   it finishes, **Then** the application is left in a consistent state with no key
   held down.

---

### User Story 3 - The gate can never make things worse (Priority: P1)

An operator running an older addon, or one whose beacon signal is lost, sees
exactly the behavior they see today. The gate never causes a key to be suppressed
that would otherwise pass, and never affects a window that is not the focused game
window.

**Why this priority**: Also P1, and it is the constitutional one. This feature
edits the safety-critical decision path. If it can only relax, every failure mode
degrades to today's behavior; if it could tighten, a bug here reaches outside the
game window, which the constitution forbids outright.

**Independent Test**: Assert, exhaustively over the decision inputs, that every
key the application passes through today is still passed through, and that no
input combination produces suppression that the current behavior would not.
Testable at the desk with no game.

**Acceptance Scenarios**:

1. **Given** any combination of inputs to the interception decision, **When** the
   gate is inactive, **Then** the decision is identical to the decision made
   without this feature.
2. **Given** the gate is active, **When** any key is considered, **Then** the
   outcome is either the same as today or more permissive, never less.
3. **Given** the game window does not have focus, **When** anything at all is
   pressed, **Then** the key passes through, regardless of the gate.
4. **Given** an addon too old to publish the signal, or a lost beacon signal,
   **When** the operator plays, **Then** interception behaves exactly as it does
   today.

---

### User Story 4 - The operator can see which surface is active (Priority: P3)

The operator can tell, from the application, whether it currently believes a
surface is open and which kind, so a misbehaving gate can be diagnosed without
guesswork.

**Why this priority**: Diagnostic value only, and the feature is correct without
it. It ranks above nothing because the previous field failures in this project
were all diagnosed by adding observability after the fact.

**Independent Test**: With each surface open in turn, confirm the application
reports the corresponding one.

**Acceptance Scenarios**:

1. **Given** a surface is open, **When** the operator looks at the application,
   **Then** it names the surface, or reports a generic one for a surface it does
   not enumerate.
2. **Given** no surface is open, **When** the operator looks, **Then** it reports
   that gameplay is active.

---

### Edge Cases

- **Chat entry, the case that reverses the obvious design.** Opening chat does not
  change which gameplay scene is showing, so a scene-based test reads "no menu"
  while the operator types. The gate must use the signal that covers it.
- **A surface opens between two samples.** The application learns about it up to
  one sampling interval late, so a keystroke in that window is still intercepted.
  Inherent to a sampled screen signal. Mitigated by sampling at the fast cadence
  rather than the idle one, and stated as a limitation rather than papered over.
- **A weave is already running when a surface opens.** It completes. See the
  clarification; aborting risks a stuck key.
- **The addon is older than the application.** The block is absent, the gate reads
  inactive, and behavior is exactly today's.
- **The beacon signal is lost entirely.** Same: the gate clears to inactive, which
  is the safe direction, because inactive means "behave as we always have".
- **A surface the addon does not enumerate.** The gate is still correct, because
  the boolean is derived from the game's UI-mode state rather than from a list of
  known surfaces; the detail simply reports a generic value.
- **The game window loses focus while a surface is open.** Focus scoping already
  passes everything through; the gate changes nothing about that.
- **The operator has manually suspended the application.** Both conditions pass the
  key through. They compose without interacting.
- **Resuming from suspend with a surface already open.** While suspended the strip
  is sampled at the idle cadence, because a suspended application has nothing to
  gate. On resume the gate may therefore be up to one idle interval stale, so
  interception can briefly resume inside an open surface before the next sample
  corrects it. Bounded, self-correcting, and accepted: the operator has just
  pressed a key deliberately, so they are present, and the alternative is sampling
  fast while suspended purely to keep a value nothing is reading.
- **The gate's core guarantee cannot be proven at the desk.** That the signal is
  true while chat entry is open depends on game behavior no automated test in this
  project can reach. It rests on the in-game validation, which is why that scenario
  is called out as the gating one rather than listed among equals.

## Requirements *(mandatory)*

### Functional Requirements

**The signal**

- **FR-001**: The addon MUST publish, as a dedicated square on the beacon strip,
  whether a native game UI surface is currently active.
- **FR-002**: The active/inactive determination MUST be derived from the game's own
  UI-mode state, and MUST be true while the in-game chat text entry is open. It
  MUST NOT be derived from which gameplay scene is showing, because that test does
  not cover chat entry.
- **FR-003**: The same square MUST also carry which surface is active, as a code
  from a fixed shared table, with a generic value for any surface not enumerated.
  The active/inactive answer MUST NOT depend on the surface being enumerated.
- **FR-004**: The square MUST be drawn whenever the addon is loaded and rendering,
  and MUST NOT be hidden to express a state, so its absence means only that the
  addon is too old to publish it.
- **FR-005**: The square MUST be redrawn only when the published state changes.
- **FR-006**: The square MUST carry a validity mark distinct from every other mark
  on the strip, and its encoded values MUST be distinguishable from each other,
  both measured against the reader's default match tolerance.
- **FR-007**: The companion MUST decode the square into an active/inactive answer
  plus the surface detail, and MUST report inactive whenever validation fails.

**The gate**

- **FR-008**: While the gate is active, the application MUST NOT intercept bound
  keys; they pass through to the game unchanged.
- **FR-009**: While the gate is active, the application MUST NOT begin a new weave
  sequence, and therefore MUST NOT synthesize the input one would produce.
- **FR-009a**: While the gate is active, the application MUST NOT send an
  *autonomous* fishing interact, meaning the reel and the recast, which fire on the
  controller's own timers with no operator involvement. Those MUST be deferred and
  retried rather than dropped, so the controller's state never advances past an
  interact the game did not receive. This is a separate requirement because fishing
  synthesizes on its own schedule in response to the beacon rather than in response
  to an intercepted key, so a gate placed only on interception would not cover it.
  Waiting for a bite is a likely moment for the operator to open chat, which makes
  this path a real source of the exact harm the feature exists to prevent, not a
  theoretical one.

  The *initial* cast is deliberately excluded. It happens only in direct response
  to the operator pressing the fishing hotkey, which is exempt from the gate so
  that they keep control, and it is sent in that same instant. Suppressing it would
  mean the operator presses the key and nothing happens, which is a worse and more
  confusing outcome than one interact the operator explicitly asked for. The
  distinction the requirement draws is autonomous versus operator-initiated, not
  new versus continuing.
- **FR-010**: The gate MUST exempt the application's own toggle hotkeys, exactly as
  the existing manual suspend does, so the operator retains control from inside a
  surface.
- **FR-011**: The gate MUST NOT abort work already in progress, whether a weave
  sequence or a fishing interaction. It prevents new work from starting. This is
  the deliberate carve-out from FR-009 and FR-009a: aborting mid-sequence risks
  leaving a synthesized key held down, which is a worse outcome than a short
  sequence completing.
- **FR-012**: When the gate becomes inactive, interception and synthesis MUST
  resume with no operator action and no residual state.
- **FR-013**: The gate MUST default to inactive and MUST become inactive whenever
  the signal is absent, undecodable, or lost, so every failure mode degrades to the
  application's current behavior.
- **FR-014**: The gate MUST have no user setting. It is a correctness behavior.

**Safety, and these are constitutional**

- **FR-015**: The gate MUST be capable only of causing a key to pass through. For
  every possible combination of decision inputs, the outcome with this feature MUST
  be either identical to the outcome without it, or a pass where there was a
  suppression. It MUST NOT be possible for the gate to cause a suppression.
- **FR-016**: Focus scoping MUST remain unconditional: when the game window does
  not hold focus, every key MUST pass through regardless of the gate's value or of
  any other input to the decision. Suppression outside the focused game window MUST
  remain impossible. The gate ANDs with focus and never replaces it. (Evaluating
  focus first is the natural way to satisfy this and is what the implementation
  does, but the requirement is the property, not the ordering; both conditions
  produce the same outcome, so order alone cannot make an implementation wrong.)
- **FR-017**: The interception decision MUST remain synchronous and non-blocking,
  with no added waiting, locking beyond what it already does, or timed work.
- **FR-018**: The existing safety tests covering injected-input recursion breaking,
  focus-scoped suppression, and the non-blocking decision MUST NOT be weakened: no
  scenario removed, no assertion loosened, no test made conditional or skipped.
  Mechanically updating a test that no longer compiles (for example because a
  constructor gained a parameter) is permitted and expected; changing what a test
  asserts is not. New coverage is added alongside them, not in place of them.

**Cadence, geometry, and distribution**

- **FR-019**: The strip MUST be sampled at the existing fast cadence at all times,
  not only during a fishing session, so the gate engages promptly.
- **FR-019a**: The addon MUST publish a change to the gate at its own fast cadence,
  not on its once-per-second general tick. Sampling quickly is worthless if the
  signal is drawn slowly: end-to-end latency is the addon's publish delay plus the
  companion's sampling delay, and SC-002 is only achievable if both are fast.
- **FR-020**: The square's position MUST derive from the configured square size and
  the strip length MUST remain stated once per side of the contract, with the
  existing automated agreement check extended to cover this square's values.
- **FR-021**: The addon manifest version MUST advance so the application offers the
  update, and its description MUST name the new signal.
- **FR-022**: The companion MUST record gate transitions in the application log at
  the debug level, and MUST present the current surface in the interface.
- **FR-023**: The master specification's pixel-bus section MUST document the new
  square, and the changelog MUST record the feature plus a dated decision for each
  contract later slices inherit.

### Key Entities

- **Gate state**: Whether the application should currently leave input alone.
  Active or inactive. Inactive is both the default and the failure value.
- **Surface**: Which kind of game UI is up: gameplay (none), the system menu, the
  map, inventory, mail, and so on, plus a generic value for anything unenumerated.
- **Beacon strip**: The run of squares the addon draws and the companion samples.
  This feature makes it six.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: With a surface open, typing a message containing bound keys delivers
  100 percent of the characters to the game and fires zero weaves, across a
  validation run covering chat, mail, the system menu, the map, and inventory.
- **SC-002**: The gate engages within one addon publish interval plus one sampling
  interval of a surface opening, and releases within the same bound of it closing.
  Both intervals are the fast cadence, so the bound is small and, more importantly,
  is derivable from the requirements rather than hoped for.
- **SC-003**: Weaves fire normally before and after every surface interaction, with
  zero cases of interception failing to resume, across at least ten open/close
  cycles.
- **SC-004**: The application's own toggle hotkeys work while a surface is open, on
  100 percent of attempts.
- **SC-005**: Exhaustive evaluation of the interception decision over every
  combination of its inputs shows zero cases where this feature turns a pass into a
  suppression.
- **SC-006**: With an addon that does not publish the signal, the interception
  decision is identical to the current behavior for every input combination.
- **SC-007**: Every existing signal on the strip behaves identically before and
  after this feature.
- **SC-008**: The full merge gate passes with no test weakened, skipped, or made
  conditional, and no existing safety test modified.

## Assumptions

- **The game's UI-mode state is a faithful proxy for "the operator may be typing
  or navigating a menu".** It is the flag the game itself uses to decide whether
  the cursor is up, and the game's own scene manager consults chat entry when
  maintaining it. A false positive costs a missed weave; a false negative costs a
  disturbed keystroke, so erring toward active is the correct bias.
- **A missed weave is cheap and a disturbed keystroke is not.** This asymmetry
  justifies every conservative choice here, including sampling faster and treating
  an unenumerated surface as a surface.
- **The operator wants the application to be quiet during menus without being
  asked.** No setting is offered on that basis.
- **One sampling interval of latency is acceptable.** It is inherent to a screen
  signal, and the alternative (reading the game from outside) is forbidden by the
  project's scope boundary.
- **The gate composes with the manual suspend rather than replacing it.** Both
  cause a pass; neither needs to know about the other.

## Dependencies

- The bundled PixelBeacon addon and its manifest.
- The beacon strip reader, whose block geometry and cross-language agreement check
  were factored in the previous slice.
- The input engine's interception decision, which is a constitutional safety
  surface and the reason this feature carries the safety requirements above.
- The master specification's pixel-bus section and its input-suppression scoping.
