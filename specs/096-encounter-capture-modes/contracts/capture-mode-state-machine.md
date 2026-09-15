# Contract: Capture Mode State Machine

## Commands

- `/ewencounter mode single|continuous` selects one of exactly two modes while off.
- `/ewencounter channel live|pts` selects the source channel while off.
- `/ewencounter toggle` enables the selected mode or disables current authority.
- `/ewencounter status` reports selected, requested, and active mode; channel;
  effective state; current encounter presence; session disposition; encounter and
  interruption counts; last interruption; and any controlled failure class.
- `/ewencounter clear confirm` clears retained encounter evidence only while not capturing.
- Historical arm, disarm, and stop forms may delegate to the same transitions;
  they cannot create another mode or control authority.

## Transitions

| Current | Input | Condition | Next | Evidence |
| --- | --- | --- | --- | --- |
| stopped | select | no request or retained conflict | stopped | selected mode/channel only |
| stopped | toggle on | outside combat | waiting | new stable session |
| stopped | toggle on | already in combat | capturing | new partial current record from exact API sample |
| waiting | combat true | requested mode present | capturing | next ordinal current record |
| capturing | combat true | duplicate notification | capturing | exact callback retained |
| capturing | combat false | single | stopped | terminal record, session stopped |
| capturing | combat false | continuous | waiting | terminal record, same active session |
| waiting | toggle off | any mode | stopped | no fabricated encounter |
| capturing | toggle off | any mode | stopped | user-stopped partial terminal record |
| capturing | deactivated/recovered | single | stopped | partial record plus interruption marker |
| capturing | deactivated/recovered | continuous | interrupted | partial record plus interruption marker |
| waiting | deactivated/reloaded | single | stopped | marker only, then single authority ends |
| waiting | deactivated | continuous | interrupted | marker only |
| interrupted | valid reload | continuous requested | waiting or capturing | same session; exact combat API sample if already in combat |
| any enabled | hard failure | controlled failure | failed | retained records plus terminal failure |
| failed | reload/toggle | any | failed | no automatic retry |

Detailed capture handlers are registered only in `capturing`. The always-small
combat-state and deactivation handlers own transitions.

## Mid-combat rule

`IsUnitInCombat("player") == true` is recorded exactly as the opening authority.
The current record is immediately marked `started-mid-combat`; no missed event
count or fake callback is added. The next real combat-exit callback terminates it.

## Hard failure rule

Callback failure, clock reset, interruption exhaustion, or inability to preserve
both current terminal and outer failure facts clears requested/active mode and
enters `failed`. A malformed same-version recovered state is instead preserved
byte-for-byte, held inactive as `state-invalid`, and may only be explicitly
cleared. Retained evidence is never deleted automatically.
