# Phase 0 Research: Auto-Potion

**Feature**: [spec.md](spec.md) | **Date**: 2026-07-27

All decisions were made under the build-phase autopilot decision policy. None
were escalated. No NEEDS CLARIFICATION markers remain.

## R1: Unknown is not low, and the asymmetry is the whole argument

**Decision**: a resource the companion cannot read never satisfies its threshold.
The same for the quickslot (unknown is not a usable potion) and its cooldown
(unknown is not zero).

**Rationale**: the two failure directions are not equally bad, and that is what
settles it rather than a preference for conservatism.

- **Unknown treated as low**: the potion fires whenever the beacon hiccups, an
  addon is mid-reload, a loading screen clears the blocks, or the operator has an
  older addon that draws no quickslot blocks at all. Each is a wasted potion and,
  worse, a keypress the operator cannot account for.
- **Unknown treated as not low**: the feature silently does nothing for the
  duration of the outage. The operator drinks a potion themselves, as they did
  before this feature existed.

The second is a feature that stops working. The first is an application pressing
keys for reasons the operator cannot see. `ResourceLevel::Unknown` and
`SlotCooldown::Unknown` already exist as distinct variants precisely so this
distinction is representable, which is what the numeric-versus-lookup-table
lesson from the resource blocks was for.

## R2: The retry interval is not a duplicate of the quickslot cooldown

**Decision**: keep both. They cover different windows.

**Rationale**: the quickslot cooldown is read from the screen, so it does not
change until the addon redraws the block and the companion samples it, which is at
least one sampling interval after the key is pressed and can be several. In that
window the rule still evaluates to "low, potion present, cooldown zero" on every
sample. Without a separate floor the controller fires on every tick of that
window, which at the fast cadence empties a stack of potions in about a second.

The retry interval is therefore a floor on the attempt rate that does not depend
on any reading being current, and the cooldown is the authority once it updates.
Deleting either would be wrong: without the interval the lag window is
unprotected, and without the cooldown the feature would fire on its own timer
regardless of what the game says.

## R3: The controller shape, taken from fishing rather than invented

**Decision**: `AutoPotionController` in a new `src/potion/mod.rs`, with
`set_enabled`, `set_gated`, `set_suspended`, `tick`, and a sink seam
(`AutoPotionSink`, `MockAutoPotionSink`, `RealAutoPotionSink`).

**Rationale**: `FishingController` is the project's proven shape for a component
that acts on its own timers, and it has already survived the exact hazard this
feature faces. Reusing the shape means the whole trigger rule is exercised with a
virtual clock and a mock sink, with no game, no window, and no input hardware,
which is what Principle III requires and what makes SC-002's eight isolated
blocking conditions cheap to test properly.

**One deliberate difference from fishing.** Fishing owns a state machine with
five states and a single pending deadline. Auto-potion has no states worth
naming: it is enabled or it is not, and every evaluation is a pure function of
the current readings plus the last-attempt time. Modelling it with a state enum
would add a concept that carries no information. What it keeps from fishing is the
seam and the gate, not the state machine.

## R4: The evaluation is a pure function, and the controller is a thin shell

**Decision**: the trigger rule is a free function taking the readings, the
configuration, and the clock, returning whether to fire. The controller holds only
the enable, the gate, the suspend flag, and the last-attempt time.

**Rationale**: SC-002 requires eight blocking conditions tested in isolation, each
with every other condition satisfied. That is a truth-table test, and a truth
table is far easier to write against a function than against an object with
history. It also makes the OR-across-resources rule (SC-004) directly
enumerable.

This matters more than it usually would because of what CHK012 in the safety
checklist warns about: a condition that appears to block but is only blocking
because another condition happened to be false. A pure function whose inputs are
all explicit makes that class of error visible in the test's own construction.

## R5: The new key and the two predicates

**Decision**: add `Key::F3` and `Action::ToggleAutoPotion` (default `F3`), and
extend both `Action` predicates.

**Rationale**: `F3` is unbound today and sits beside `F1` (suspend) and `F2`
(fishing), so the three application toggles are contiguous and memorable.

The two predicates are the part most likely to be missed, and the issue was right
to call them out by name:

- `suspend_exempt` currently matches `ToggleSuspend | ToggleFishing`. Without the
  new variant the hotkey stops working the moment the operator suspends, which is
  precisely when they might reach for it.
- `is_app_toggle` currently matches the same pair. Without the new variant the
  action routes to the weave worker as though it were a skill, which would try to
  run a weave sequence for it.

Both have existing unit tests that enumerate the expected set, so both fail
loudly rather than silently. `Key` must gain the variant in four places (the enum,
the wire name, the display name, and the parser) plus `Key::ALL`.

## R6: Where the controller ticks

**Decision**: on the existing pixel-bus worker loop in `src/main.rs`, beside
`fishing.tick(now, &mut sink)`.

**Rationale**: that loop already runs at the sampling cadence, already holds the
decoded state the rule needs, and is already off the hook thread. Adding a thread
or a timer would create a second place where synthesized input originates, which
is exactly what FR-012 forbids and what the constitution's hook-thread principle
exists to prevent. The controller reads the readings the loop has just decoded, so
it never samples anything itself.

**The suspend flag is pushed, not pulled.** The controller is told whether the
application is suspended rather than reaching for the input engine, so its rule
stays a pure function of its inputs and the suspended case is testable without
constructing an engine.

## R7: Not restoring the enable across sessions

**Decision**: auto-potion starts off on every launch, even if it was on when the
application last closed. Suspend and fishing are both restored; this is not.

**Rationale**: this is a deliberate inconsistency with the two toggles it
otherwise mirrors, so it is worth stating rather than leaving as an oversight. The
difference is what happens when the operator forgets: a restored fishing session
does nothing until the operator stands at a fishing hole and it times out, while a
restored auto-potion silently waits to press a key the first time a resource dips,
possibly days later, in a fight the operator does not associate with the
application. FR-013's "defaults to off" is read here as "starts off", which is the
safer reading and costs one keypress per session to undo.
