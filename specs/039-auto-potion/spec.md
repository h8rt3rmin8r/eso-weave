# Feature Specification: Auto-Potion

**Feature Branch**: `039-auto-potion`

**Created**: 2026-07-27

**Status**: Draft

**Input**: GitHub issue #20 (auto-potion, the first consumer of a beacon-derived
value). Build plan `docs/plans/plan-012.md`, slice 039. Master specification
sections 6.4 (bindings) and 10.3 (the pixel-bus contract), and the constitution's
NON-NEGOTIABLE input-safety surfaces.

## Overview

The application has spent eight slices learning to read the game and has
deliberately acted on none of it. Every beacon signal shipped so far is inert:
the code says so, and a test for each one asserts the engine behaves identically
for every value it can take, so wiring one into a decision has to break a test
rather than slip through.

This feature is the first consumer. It drinks a potion when a resource runs low.

That single sentence changes the project's risk profile, which is why this is its
own slice and why it is sequenced last. Everything before it could be wrong and
produce a misleading readout. This can be wrong and press a key in the operator's
game, unbidden, while they are doing something else. It therefore lands squarely
on the constitution's NON-NEGOTIABLE surface: input suppression scoped to the
focused window, synthesized input flagged against recursion, and no blocking work
on the hook thread.

The design response is to add no new input path at all. Synthesis goes through
the input engine that already exists and is already tested for those properties.
The controller is modelled directly on the fishing controller, which is the
project's proven shape for "a thing that acts on its own timers": an enable, a
gate, a tick, and a sink seam, so the entire trigger rule is testable with a
virtual clock and no game running.

One lesson from an earlier slice is carried rather than rediscovered. When the
menu gate landed, it was found that gating the interception path is not enough,
because the fishing controller synthesizes on its own timers and never passes
through interception. Auto-potion has exactly that shape, so the gate is applied
to the controller directly.

A note on what "user" means here. This application has one user, the operator
running it beside the game. Everything below is written from what that operator
can observe.

## Clarifications

### Session 2026-07-27

Answered under the build-phase autopilot decision policy from the constitution,
build plan 012, GitHub issue #20, and the existing controller and input code.
None were escalated.

- Q: What does an unreadable resource count as? -> A: Not low. A resource the
  companion cannot read MUST NOT satisfy its threshold, ever. This is the single
  most important decision in the feature and it is not symmetric: treating
  unknown as low would fire a potion every time the beacon hiccups, an addon is
  mid-reload, or a loading screen clears the blocks, which is both a wasted potion
  and an unexplained keypress. Treating unknown as not-low means the feature
  silently does nothing in that window, which is the correct failure direction for
  something that presses keys. The same reasoning covers the quickslot: an unknown
  quickslot is not a potion, and an unknown cooldown is not zero.
- Q: OR across the enabled resources, or AND? -> A: OR, always, and this is not
  configurable. The issue is explicit and the reasoning is sound: waiting for
  health, magicka, and stamina to be low simultaneously would fire only in
  situations where the potion no longer helps. Per-stat thresholds are what make
  the OR sensible, because the right number genuinely differs between health and
  magicka, and per-stat controls make the rule visible in the interface rather
  than buried in a comment.
- Q: Does the feature act while the application is suspended? -> A: No, and this
  is a hard condition checked in the controller rather than an emergent property.
  Suspend is the operator saying "stop touching my game". An automation that kept
  firing through it would be the most alarming possible bug, and "the controller
  happens not to tick because of how the loop is wired" is not a guarantee.
- Q: What is the retry interval for, given the quickslot cooldown already exists?
  -> A: It guards the window between pressing the key and the game reporting the
  resulting cooldown, which is at least one sampling interval and can be several.
  Without it the controller would see "low, potion, ready" on every sample in that
  window and fire repeatedly, emptying the operator's potions in under a second.
  The quickslot cooldown is the authority once it updates; the retry interval
  covers the gap before it does. It is therefore a floor on the firing rate, not a
  duplicate of the cooldown.
- Q: What happens when the potion is drunk but the resource stays below its
  threshold? -> A: It fires again once both the quickslot cooldown and the retry
  interval allow. This is correct: a potion that did not bring the resource back
  above the threshold is a situation where another potion is exactly what is
  wanted. The cooldown is what stops this being a loop.
- Q: Does the feature need to know what the potion restores? -> A: No, and it
  cannot. The previous slice established that the restore types are not
  machine-readable, existing only inside a localized ability description. The
  operator chooses which stats to watch, which is strictly more flexible than
  inferring it: someone slotting a tri-restoration potion can enable all three,
  and someone slotting a health potion can enable health alone.
- Q: What key does it press? -> A: A configured quickslot key, defaulting to `Q`,
  which is the game's default quickslot bind. It is configurable for the same
  reason the fishing interact key is: an operator who rebound it in game must be
  able to say so.
- Q: Does the toggle hotkey work while suspended? -> A: Yes, like the other two
  toggles. `F1` (suspend) and `F2` (fishing) are both suspend-exempt, because a
  hotkey the operator cannot reach while suspended is not a toggle. `F3` joins
  them. Note this makes the hotkey reachable while suspended but does not make the
  feature act while suspended; those are separate conditions.
- Q: Does the feature default to on? -> A: No. It defaults to off, every
  per-stat enable defaults to off, and nothing changes for an operator who does
  not go looking for it. A feature that presses keys does not arrive switched on.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - A potion is drunk when a resource runs low (Priority: P1)

The operator enables auto-potion, sets a health threshold, and plays. When health
drops to or below that threshold and a usable potion is in the active quickslot,
the application presses the quickslot key. It does not press it again until the
potion's cooldown has expired.

**Why this priority**: This is the feature.

**Independent Test**: With the trigger conditions satisfied in a test harness
driving a virtual clock, confirm exactly one keypress is emitted, and that a
second sample under the same conditions emits nothing until both the cooldown and
the retry interval have elapsed.

**Acceptance Scenarios**:

1. **Given** auto-potion is on, health is enabled at a threshold, a potion is in
   the quickslot and ready, **When** health falls to the threshold, **Then** the
   quickslot key is pressed exactly once.
2. **Given** the same, **When** health falls below the threshold rather than
   exactly to it, **Then** the key is pressed, because the rule is at-or-below.
3. **Given** a potion was just drunk, **When** the next sample arrives with the
   resource still low, **Then** nothing is pressed until both the quickslot
   cooldown and the retry interval allow it.
4. **Given** two resources are enabled and only one is low, **When** the
   application evaluates the rule, **Then** the key is pressed, because the rule
   is an OR.
5. **Given** three resources are enabled and none is low, **When** the
   application evaluates the rule, **Then** nothing is pressed.

---

### User Story 2 - It never fires when it must not (Priority: P1)

The application does not press the quickslot key when the operator has suspended
it, when a game menu or text field is open, when the beacon signal is lost, when
the quickslot holds no usable potion, when the potion is on cooldown, or when the
resource readings cannot be trusted.

**Why this priority**: **This ranks equal with the feature itself, not below it.**
Every condition here is a way the application presses a key in the operator's game
at a moment they did not choose. A feature that fires correctly nine times out of
ten and wrongly once is worse than no feature, because the wrong keypress arrives
in a text box, a menu, or a fight where the potion was being saved.

**Independent Test**: For each blocking condition in isolation, with every other
trigger condition satisfied, confirm nothing is emitted.

**Acceptance Scenarios**:

1. **Given** every trigger condition is satisfied, **When** the application is
   suspended, **Then** nothing is pressed.
2. **Given** every trigger condition is satisfied, **When** a game menu or text
   field is open, **Then** nothing is pressed.
3. **Given** every trigger condition is satisfied, **When** the beacon signal is
   lost, **Then** nothing is pressed and the feature returns to disabled rather
   than continuing to evaluate.
4. **Given** the resource is low and the quickslot reports no potion, **When** the
   application evaluates the rule, **Then** nothing is pressed.
5. **Given** the resource is low and the quickslot cooldown is not zero, **When**
   the application evaluates the rule, **Then** nothing is pressed.
6. **Given** a resource cannot be read, **When** the application evaluates the
   rule, **Then** that resource does not satisfy its threshold and, if it is the
   only enabled one, nothing is pressed.
7. **Given** auto-potion has never been enabled, **When** the application runs
   normally, **Then** it behaves exactly as it did before this feature existed.

---

### User Story 3 - The operator controls it and can see what it is doing (Priority: P2)

The operator turns auto-potion on and off with a hotkey and with a control in the
interface, sets a threshold and an enable per resource, and can see whether it is
currently on.

**Why this priority**: The feature is unusable without controls, but it is the
trigger rule and the safety conditions that determine whether it is correct.

**Independent Test**: Toggle by hotkey and by control and confirm both reach the
same state; change each setting and confirm it persists and reloads.

**Acceptance Scenarios**:

1. **Given** the application is running, **When** the operator presses the toggle
   hotkey, **Then** auto-potion turns on, and pressing it again turns it off.
2. **Given** the application is suspended, **When** the operator presses the
   toggle hotkey, **Then** it still registers, like the suspend and fishing
   hotkeys.
3. **Given** the operator changes a threshold, **When** the settings are saved and
   reloaded, **Then** the value persists.
4. **Given** an invalid stored value, **When** the settings load, **Then** the
   default is used and a notice is surfaced, rather than the application failing
   to start.

---

### Edge Cases

- **A resource cannot be read.** Never counts as low. Covered by User Story 2.
- **The quickslot cannot be read.** Never counts as a usable potion.
- **The beacon signal is lost while auto-potion is on.** The feature returns to
  disabled rather than firing blind, matching fishing.
- **The operator suspends mid-session with a low resource.** Nothing fires.
- **A menu opens between the decision and the keypress.** The gate is checked at
  the point of firing, not only when the state was entered.
- **The potion is drunk but the resource stays low.** Fires again once the
  cooldown and the retry interval allow, which is the wanted behavior.
- **Every resource is disabled while auto-potion is on.** Nothing ever fires;
  there is no implicit fallback to watching a resource the operator turned off.
- **A threshold of zero.** A resource can reach zero, so a zero threshold is
  meaningful (it fires only when the pool is empty) and is allowed rather than
  treated as "off". Turning a resource off is what the per-stat enable is for.
- **A threshold of 100.** Fires whenever the resource is readable, which is
  unusual but not incoherent, and is the operator's choice.
- **Fishing and auto-potion both active.** Both synthesize through the same input
  engine and neither is aware of the other. They can interleave; nothing about
  either becomes unsafe, because each keypress is independently gated.
- **The application starts with auto-potion enabled from a previous session.**
  It does not: the feature is not restored on launch, following the safest reading
  of "defaults to off" for something that presses keys.

## Requirements *(mandatory)*

### Functional Requirements

**The trigger rule**

- **FR-001**: The application MUST press the configured quickslot key when, and
  only when, ALL of the following hold: auto-potion is enabled; at least one
  enabled resource is readable and at or below its own threshold; the active
  quickslot holds a usable potion; the quickslot cooldown is zero; the minimum
  retry interval since the last attempt has elapsed; the application is not
  suspended; and no game menu or text field is gating input.
- **FR-002**: The resource condition MUST be an OR across the enabled resources
  and MUST NOT be an AND. This MUST NOT be configurable.
- **FR-003**: Each resource MUST have its own threshold and its own enable. A
  disabled resource MUST NOT contribute to the trigger regardless of its value.
- **FR-004**: A resource the companion cannot read MUST NOT satisfy its threshold.
  Unknown is not low. The same MUST hold for the quickslot: an unreadable
  quickslot is not a usable potion, and an unreadable cooldown is not zero.
- **FR-005**: The comparison MUST be at-or-below the threshold, not strictly
  below.
- **FR-006**: The minimum retry interval MUST bound the firing rate independently
  of the quickslot cooldown, because the cooldown does not update until at least
  one sampling interval after the key is pressed.
- **FR-007**: One trigger MUST press the key exactly once (a press and its
  release), never a repeat within the same evaluation.

**Safety, none optional and all test-covered**

- **FR-008**: Synthesis MUST go through the existing input engine, so it remains
  scoped to the focused game window and flagged against recursion. This feature
  MUST NOT introduce a new input path.
- **FR-009**: The menu gate MUST be applied to the controller directly, not only
  to the interception path, because the controller acts on its own timers and
  never passes through interception.
- **FR-010**: The feature MUST do nothing while the application is suspended, as
  a condition checked in the controller rather than as a consequence of how the
  worker loop happens to be wired.
- **FR-011**: The feature MUST return to disabled when the beacon signal is lost,
  rather than continuing to evaluate against stale readings.
- **FR-012**: No blocking work MUST reach the input hook thread. The controller
  MUST tick on the existing worker loop and MUST NOT introduce a thread or a
  timer.
- **FR-013**: The feature MUST default to off, with every per-resource enable also
  defaulting to off, so it changes nothing for an operator who does not enable it.
- **FR-014**: The existing safety tests MUST continue to pass unchanged in intent.
  This feature MUST NOT weaken, skip, or make conditional any test covering
  injected-input recursion breaking, focus-scoped suppression, or hook-thread
  non-blocking.

**Controls and configuration**

- **FR-015**: A toggle hotkey MUST turn the feature on and off, defaulting to a
  key not currently bound, and MUST remain active while the application is
  suspended, matching the suspend and fishing toggles. The key enumeration MUST
  gain the new key across every place a key is represented.
- **FR-016**: The predicates that classify an action as suspend-exempt and as an
  application-level toggle MUST include the new action, so it routes like the
  toggles it mirrors rather than like a weave action.
- **FR-017**: The interface MUST expose the per-resource thresholds and enables,
  the quickslot key, and the retry interval, and MUST show whether auto-potion is
  currently on.
- **FR-018**: Settings MUST persist and reload, and an invalid stored value MUST
  fall back to its default with a notice rather than failing the load.

**Boundaries**

- **FR-019**: This feature MUST NOT change the trigger for any other synthesized
  input: weave sequences, the fishing interact, and hotkey handling behave exactly
  as they do today.
- **FR-020**: This feature MUST NOT change the pixel-bus contract, the block
  count, the grid geometry, or the addon. It consumes signals that already exist.
- **FR-021**: This feature MUST NOT change the sampling cadence.
- **FR-022**: The master specification MUST document the feature and its safety
  conditions, and the changelog MUST record it plus a dated decision for each
  choice a later feature inherits.

### Key Entities

- **Trigger rule**: The conjunction of conditions in FR-001. Every one is
  necessary; the resource condition is itself a disjunction across the enabled
  resources.
- **Per-resource watch**: One resource's enable and threshold. Three of them,
  independent.
- **Retry interval**: The minimum time between two attempts, covering the window
  in which the game has not yet reported the cooldown the last press caused.
- **Auto-potion controller**: The state machine that owns the rule, the gate, the
  enable, and the last-attempt time, and emits through a sink seam so the whole
  rule is testable with a virtual clock.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: With every trigger condition satisfied, exactly one keypress is
  emitted per eligible moment, across a run of at least twenty evaluations.
- **SC-002**: For each blocking condition in isolation (suspended, gated, signal
  lost, no potion, cooldown not zero, resource unreadable, resource disabled,
  feature disabled), with every other condition satisfied, zero keypresses are
  emitted, in 100 percent of cases.
- **SC-003**: A resource reported as unreadable never satisfies its threshold, for
  every threshold value from 0 to 100.
- **SC-004**: With one resource enabled and low and the others high, the key is
  pressed, proving the rule is an OR and not an AND, for each of the three
  resources in turn.
- **SC-005**: After a press, no second press occurs until both the quickslot
  cooldown and the retry interval have elapsed, verified against a virtual clock.
- **SC-006**: With the feature never enabled, the application emits byte-identical
  synthesized input to what it emitted before this feature, across the existing
  weave and fishing test suites.
- **SC-007**: The full merge gate (formatting, lint at deny-warnings, and the
  whole test suite) passes with no test weakened, skipped, or made conditional,
  and every existing input-safety test passes unchanged in intent.

## Assumptions

- **The signals this consumes are correct.** The resource blocks and the quickslot
  blocks were shipped as observables specifically so they could be proven in the
  field before anything depended on them. This feature assumes that proving has
  either happened or will, and its failure modes are all in the safe direction if
  it has not.
- **The game's quickslot key is a single keypress.** Drinking a quickslotted
  potion is one key, not a hold or a sequence.
- **One potion at a time is the whole feature.** There is no queueing, no
  prioritizing between potions, and no awareness of what the potion restores.
- **The operator wants a floor on the firing rate.** The retry interval exists
  because the cooldown reading lags the press; an operator who sets it to zero is
  choosing to rely on the cooldown alone.

## Dependencies

- The resource blocks (B6 to B8) and the quickslot blocks (B16 to B19), both
  already shipped as observables.
- The input engine's synthesis path, its suspend state, and its recursion
  flagging.
- The menu gate, and the lesson recorded with it that a controller acting on its
  own timers must be gated directly.
- The fishing controller, as the structural model for the enable, gate, tick, and
  sink seam.
- The existing worker loop in the application entry point, which already ticks the
  fishing controller and will tick this one beside it.
