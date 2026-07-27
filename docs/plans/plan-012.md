# Build Plan 012: Skill Cooldowns, Quickslot State, and Auto-Potion

Plan: 012
Status: active
Master specification: `docs/ESO-Weave-Specification-v0.2.0.md`
Constitution: `.specify/memory/constitution.md`

## Purpose

Build plan 010 completed with slice 036, and the tracker fell to a deferred
specification refresh and a sprint axis the game does not expose. Two things were
left standing.

The first is that the weave engine guesses at timing. Its global cooldown is a
fixed setting, and its per-weapon heavy-attack delays are, by the admission of
the comment beside them, community estimates never validated in game, with one
weapon class an outright guess. The master specification's Appendix A.3 lists
those same measurements as owed. The game knows the real per-slot cooldown and
has never been asked.

The second is that the grid wrap shipped in slice 035 removed the width ceiling
and nothing used the space. At ten blocks the bus used ten of sixteen columns.

This plan responds to both, and adds the feature that motivated the operator's
request: automatically drinking a potion when a resource runs low. That last one
is a first for the project, and the reason this plan has three slices rather than
two.

## Why auto-potion is isolated

Every beacon signal shipped to date is deliberately inert. The code comments say
so, and the tests enforce it: `tests/weave.rs` asserts the engine behaves
identically for every value of the signals it stores, so wiring one into a
decision has to be a deliberate change that breaks a test rather than an
accident.

Auto-potion is the first consumer that acts on a beacon-derived value, and it
acts by synthesizing a keypress. That puts it on a constitution NON-NEGOTIABLE
surface: input suppression scoped to the focused window, synthesized input
flagged against recursion, no blocking work on the hook thread. It also needs the
menu gate from slice 032, and it needs the lesson that slice found, which is that
a controller synthesizing on its own timers never passes through the interception
path and must therefore be gated directly.

Building that on top of signals that are already proven in the field is worth a
slice boundary. Debugging a potion that fires at the wrong moment is much harder
when it is not yet known whether the trigger logic or the underlying reading is
at fault.

## Ordering

The three slices run one at a time. Each block-adding slice claims the next
physical block indices, so two in flight would collide, exactly as build plan 010
documented.

The observable slices run before the consumer for the reason above. Between the
two observable slices, cooldowns run first because they take the grid to exactly
its single-row maximum, which leaves the quickslot slice a known and asserted
starting point for the row it crosses.

## Slices

### Slice 037: Skill cooldown blocks (B10 to B15)

Closes issue #18. Feature under `specs/037-cooldown-blocks/`.

Six blocks, one per action slot the game exposes a cooldown for: the five skills
and the ultimate. Synergy gets none, because it is a contextual prompt rather
than an action slot and the game reports nothing for it in any state; that was
established by checking the game's own action-bar iteration rather than assumed,
and it is what reduced this slice from seven blocks to six.

Each block carries a validity mark in green, the remaining time quantized to
50 ms steps in red, and the complement checksum in blue, saturating at 12.7
seconds with a reserved unavailable value. The six values travel as one aggregate
event, following the resource blocks, so a single weave that moves several slots
is one change rather than six. The companion shows them as a new column on the
skills grid it already draws.

This slice takes the block count from ten to sixteen, which is exactly the column
count: the grid fills one row completely and one more block wraps. The
compile-time assertion that the count does not exceed the column count is left in
force rather than relaxed, so the slice that adds the seventeenth block is told.

Observable only: nothing acts on the values.

### Slice 038: Quickslot state blocks (B16 to B19)

Closes issue #19. Feature under `specs/038-<name>/`.

Four blocks describing the active quickslot: the remaining quickslot cooldown
with an unavailable value that doubles as "empty, or not a potion", and the
24-bit item id across three blocks.

**This slice crosses the row boundary.** The count goes from sixteen to twenty,
so four blocks land on row 1 and the captured region becomes two rows tall. The
geometry for that was built correctly by slice 035 and is covered by tests that
construct multi-row cases directly, so nothing is unbuilt; what changes is the
expectations written when one row was the only shipping possibility, including
the compile-time assertion slice 037 deliberately left in place. This slice also
owes an answer to a question slice 037 dissolved rather than settled: what the
operator sees when the overlay in the corner of the game client becomes twice as
tall.

The item id is carried rather than the potion's restore types, because the
restore types are not machine-readable: they appear only inside a localized
ability description that the game's own interface consumes as tooltip text.
Parsing that would be a locale-dependent heuristic of exactly the kind the sprint
verification rejected in slice 036. The id is machine-readable, gives the
consumer swap detection, and leaves restore awareness addable later in the
companion without touching the bus contract.

Observable only.

### Slice 039: Auto-potion

Closes issue #20. Feature under `specs/039-<name>/`.

The first consumer. Fires the quickslot key when any enabled resource is at or
below its own threshold, the quickslot holds a potion, its cooldown is zero, and
a minimum retry interval has elapsed. The trigger is an OR across the enabled
stats and never an AND: waiting for all three to cross would mean firing only
when it no longer helps.

Thresholds and enables are per resource, in a new Advanced section of the
settings modal, because the right number genuinely differs between health and
magicka and because per-stat controls make the OR rule explicit in the interface
rather than implied. A new toggle hotkey defaulting to `F3` mirrors `F1` for
suspend and `F2` for fishing; `F3` does not currently exist in the key
enumeration and is added.

The controller is modelled directly on the fishing controller, with the same
enable, gate, tick, and sink seam, so the whole trigger rule is testable with a
virtual clock and no game running.

Safety requirements, none optional and all test-covered: synthesis goes through
the existing input engine so it stays focus-scoped and recursion-flagged; the
menu gate is applied to the controller directly, not only to interception;
nothing fires while suspended; the controller degrades to disabled on signal loss;
no blocking work reaches the hook thread; and the feature defaults to off.

## Traceability

This plan traces to the master specification's pixel-bus contract (section 10.3),
which slices 037 and 038 each extend, and to its input-suppression scoping, which
slice 039 must respect. The master specification is itself stale at v0.2.0 and is
scheduled for the refresh tracked in issue #15; each slice updates the sections it
touches rather than waiting on that refresh.
