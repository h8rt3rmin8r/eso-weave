# Data Model: Native Binding Consumption

## Physical input

`PhysicalInput` is either a supported `NativeControl` primary or one of four normalized modifiers. A physical event adds transition and origin. Modifier events update `PhysicalModifierState` and always pass through. Primary events are classified against one exact modifier snapshot.

## Native combat authority

`NativeBindingSet` remains the wire-derived source. `CombatBindingSnapshot` derives only Skill 1 through 5, Ultimate, Synergy, Attack, and Block and carries the current binding generation. Any non-valid required fact prevents plan construction. A valid fact can still be locally ambiguous when another migrated trigger has the same chord.

## Weave requirement

| Weave type | Required chords |
| --- | --- |
| Light attack | skill, Attack |
| Heavy attack | skill, Attack |
| Bash attack | skill, Attack, Block |
| Block casting | skill, Block |

## Queued combat plan

`CombatChordPlan` contains a skill chord, optional Attack chord, optional Block chord, and captured authorization generation. It is copied into `QueuedAction` at admission and is absent for application toggles.

## Ownership ledger

The real sink reads physical modifiers, tracks generated held primaries, and owns temporary modifiers for the current primary-down only. Cleanup releases temporary modifiers and held primaries in reverse order. Physical modifiers never enter the application ledger.

## Toggle binding table

The table contains only Toggle Suspend (F1), Toggle Fishing (F2), and Toggle Auto Potion (F3). Legacy combat entries are obsolete input and omitted from the normalized settings map. Toggle collisions remain rejected.

## State transitions

```text
unavailable evidence
  -> coherent valid set
  -> exact trigger and complete plan admitted
  -> queued plan
  -> running plan
  -> completed with empty ownership

binding replacement or safety closure
  -> authorization increments
  -> queued plan rejected or running plan cancels
  -> generated ownership cleanup
  -> no later generated down event
```
