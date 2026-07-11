# Data Model: Weave Engine

Language-neutral entities. Concrete Rust types are fixed in the tasks phase.

## WeaveType

One of `LightAttack`, `HeavyAttack`, `BashAttack`, `BlockCasting`. Determines the
sequence and which delays are relevant.

## MouseButton

`Primary` (left) or `Secondary` (right).

## InputOp

A single synthesized operation in a sequence: a key transition (the slot's key,
down or up) or a mouse transition (a `MouseButton`, down or up). A click is a down
followed by an up.

## WeaveStep

Either `Emit(InputOp)` or `Wait(ms)`. A sequence is an ordered list of steps.

## TimingConfig

| Field | Type | Default (ms) |
| --- | --- | --- |
| global_cooldown | u32 | 500 |
| d_weave | u32 | 50 |
| d_heavy | u32 | 1000 |
| d_bash | u32 | 125 |

Validation: values are milliseconds; a missing or out-of-range persisted value
falls back to the default with a notice.

## SlotOverrides

Optional per-slot overrides for the delays relevant to the slot's type
(`d_weave`, `d_heavy`, `d_bash`). A blank override means use the global default.
Overrides irrelevant to the type are ignored.

## SkillSlot

| Field | Type | Notes |
| --- | --- | --- |
| index | 1..7 | Slot 6 is Ultimate, slot 7 is Synergy. |
| key | Key | The slot's bound key. |
| weave_type | WeaveType | The sequence to run. |
| active | bool | Slots 1 to 5 default active; 6 and 7 default inactive. |
| overrides | SlotOverrides | Per-slot delay overrides. |

## WeaveConfig

The seven `SkillSlot`s plus the `TimingConfig`. Persists as additive `skills` and
`timing` settings sections. Defaults follow master specification sections 7.1 and
7.3.

## Sequences (sequence_for output)

- Light Attack: primary down, primary up, wait d_weave, key down, key up.
- Heavy Attack: primary down, wait d_heavy, key down, key up, primary up.
- Bash Attack: primary down, primary up, wait d_weave, key down, key up, wait
  d_bash, secondary down, primary down, primary up, secondary up.
- Block Casting: secondary down, key down, key up, wait d_weave, secondary up.

("Primary click" expands to primary down then primary up; "send skill key"
expands to key down then key up.)

## Cooldown state

`last_weave_ms: Option<u64>` recorded at the start of a non-dropped execution;
compared against `now_ms()` and `global_cooldown` to gate the next weave.
