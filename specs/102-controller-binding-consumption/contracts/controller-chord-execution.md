# Contract: Autonomous Controller Chord Execution

## Admission

1. The caller names exactly one native action, Interact or Quickslot.
2. The executor receives an admitted autonomous epoch or captures the current epoch for immediate work.
3. It rejects a closed safety gate, changed epoch, or any non-valid action state.
4. It copies the chord and rechecks the epoch after binding lookup.
5. It rejects when physical modifiers are not a subset of the target modifiers.

## Emission

1. Missing generated modifiers press in Control, Alt, Shift, Command order.
2. Admission is checked before each new generated down event.
3. The native primary presses once.
4. A non-momentary primary releases once after a successful down, even if authority closes between them.
5. Generated modifiers release in reverse order and physically held modifiers are never released.
6. A momentary primary emits one event and no up event.

## Result

- `true` means the primary down succeeded.
- `false` means no successful primary down occurred.
- Auto Potion updates retry accounting only for `true`.
- Fishing advances its state only for `true`; rejected work is disabled with binding remediation.

## Invalidation

Binding replacement and signal loss advance the autonomous epoch before publishing replacement evidence or acquiring controller locks. Older scheduled work cannot start another down event.
