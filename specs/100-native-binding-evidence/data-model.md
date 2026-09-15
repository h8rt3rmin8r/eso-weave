# Data Model: Native ESO Binding Evidence

## NativeAction

Stable ordered identifiers:

1. Skill 1 (`ACTION_BUTTON_3`)
2. Skill 2 (`ACTION_BUTTON_4`)
3. Skill 3 (`ACTION_BUTTON_5`)
4. Skill 4 (`ACTION_BUTTON_6`)
5. Skill 5 (`ACTION_BUTTON_7`)
6. Ultimate (`ACTION_BUTTON_8`)
7. Synergy (`USE_SYNERGY`)
8. Attack (`SPECIAL_MOVE_ATTACK`)
9. Block (`SPECIAL_MOVE_BLOCK`)
10. Interact (`GAME_CAMERA_INTERACT`)
11. Quickslot (`ACTION_BUTTON_9`)

The order is both the desktop array order and the B29 through B39 payload order.

## KeyboardControl

A closed enum whose discriminants are the portable control codes for:

- top-row digits 0 through 9;
- letters A through Z;
- function keys F1 through F24;
- navigation, editing, whitespace, lock, and system keys;
- numpad digits and operators;
- ordinary OEM punctuation keys;
- left and right Windows keys.

Modifier keys are intentionally absent as primaries. Unknown and gamepad values do not enter this enum.

## MouseControl

A closed enum for left, right, middle, button 4, button 5, wheel up, and wheel down.

## NativeControl

A tagged union of `KeyboardControl` or `MouseControl`. It converts to and from exactly one portable wire code.

## ModifierSet

Four normalized flags:

- Shift, bit 0
- Control, bit 1
- Alt, bit 2
- Command, bit 3

Valid range is 0 through 15. Ordering from ESO is discarded after duplicate and membership validation.

## NativeChord

- `primary: NativeControl`
- `modifiers: ModifierSet`

Invariant: the primary is never itself a modifier and all modifiers are unique members of the supported set.

## NativeBindingState

- `Unavailable`: discovery or evidence is absent, stale, malformed, or incompatible.
- `Unbound`: the action has no non-empty native assignment.
- `Conflicting`: more than one distinct native assignment exists.
- `Unsupported`: exactly one assignment exists but cannot be represented safely.
- `Valid(NativeChord)`: exactly one distinct supported assignment exists.

Only `Valid` carries a chord.

## NativeBindingSet

A fixed array of eleven `NativeBindingState` entries indexed by `NativeAction`. The default is all unavailable. Equality compares the complete set so the reader emits a single change event.

## BindingCell

One RGB value at a fixed action position:

- red: expanded high nibble of the state or portable control code;
- green: expanded low nibble;
- blue high nibble: modifiers for valid controls, zero for non-valid states;
- blue low nibble: action-and-control check value.

A cell that fails any invariant produces `Unavailable` for that action only.

## State transitions

```text
API absent or indices absent -> Unavailable
no non-empty slots           -> Unbound
two or more distinct chords  -> Conflicting
one unrepresentable chord    -> Unsupported
one representable chord      -> Valid

missing, stale, or corrupt pixel evidence -> Unavailable
```
