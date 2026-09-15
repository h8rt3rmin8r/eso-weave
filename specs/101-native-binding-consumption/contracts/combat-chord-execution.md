# Contract: Combat Chord Execution

## Admission

For a real physical primary-down, admission succeeds only when ESO is active and focused, all existing gates are open, the exact chord matches one active non-ambiguous trigger, every weave-required chord is valid in the same snapshot, physical modifiers are a subset of every target chord, and authorization remains unchanged before handoff.

Success suppresses the primary lifecycle and queues one immutable plan. Failure passes the physical lifecycle and queues nothing.

## Generated primary-down

1. Recheck authorization.
2. Read current physical modifiers.
3. Cancel if physical modifiers are not a subset of the target chord.
4. Press missing modifiers in Control, Alt, Shift, Command order.
5. Emit the primary activation.
6. Release only those temporary modifiers in reverse order.

Failure blocks later generated down events and starts best-effort application-owned cleanup.

## Generated primary-up

- Keyboard keys and mouse buttons emit up only when the application owns the matching held primary.
- Wheel directions emit nothing because activation is a relative pulse.
- Gate closure never blocks ownership cleanup.

## Invalidation

Every native binding set replacement increments weave authorization before the replacement becomes interceptable. Signal loss publishes unavailable evidence through the same boundary.

## Pass-through lifecycle

A passed primary-down records its identity so its up also passes after state changes. A committed trigger and its up are suppressed. Auto-repeat never creates another queue entry.

## Platform mapping

Windows low-level keyboard and mouse hooks ignore injected events. Linux grabbed keyboard and pointer devices forward unrelated events through a separate virtual device. Both synthesize every S100 primary and all four modifiers.

## Configuration migration

Loading uses only `toggle_suspend`, `toggle_fishing`, and `toggle_auto_potion`. Saving writes exactly those keys. No migrated combat field influences runtime behavior.
