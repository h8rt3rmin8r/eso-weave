# Data Model: Input Engine

Language-neutral entities. Concrete Rust types are fixed in the tasks phase.

## Key

A platform-neutral key identifier used by the core and the binding table. Covers
at least the keys the default bindings need (`Digit1` through `Digit5`, `R`, `X`,
`F1`, `F2`) plus room to grow. Serialized as a stable lowercase or canonical
string in settings; each backend maps between a `Key` and its native scan or key
code.

## Action

A named operation the engine can be triggered to perform.

| Action | Default key | Suspend-exempt |
| --- | --- | --- |
| Skill1..Skill5 | Digit1..Digit5 | No |
| Ultimate | R | No |
| Synergy | X | No |
| ToggleSuspend | F1 | Yes |
| ToggleFishing | F2 | Yes |

Execution of an action is out of scope for this slice; the engine only classifies
and hands off.

## Transition

`Down` or `Up`. A bound key suppresses both; a hand-off happens only on a new
`Down`.

## Origin

`Real` or `SelfOriginated`. Self-originated events (synthesized by the engine) are
never suppressed and never handed off (recursion breaking).

## KeyEvent

| Field | Type | Notes |
| --- | --- | --- |
| key | Key | The platform-neutral identifier. |
| transition | Transition | Down or up. |
| origin | Origin | Real or self-originated. |

## Binding and BindingTable

| Field | Type | Notes |
| --- | --- | --- |
| action | Action | The bound action. |
| key | Key | The physical key. |
| suspend_exempt | bool | Whether it works while suspended. |

The `BindingTable` is the full set. Invariants and operations:

- No two actions may map to the same key. `rebind(action, key)` rejects a change
  that would collide and leaves the table unchanged (FR-015).
- `lookup(key)` returns the bound action and its suspend-exempt flag, or nothing.
- `default()` returns the section 6.4 defaults.
- Loading from settings: a conflicting or unknown entry falls back to that
  action's default and yields a notice (FR-017). The table serializes as the
  additive `bindings` settings section (FR-021).

## Decision

The result of `classify`: whether to suppress the event. Suppression means the
original keystroke does not reach the game.

## EngineState

Focused (bool) and suspended (bool), plus the set of currently-held bound keys.
Classification reads this state per event; the backend updates focused, and the
toggle-suspend action updates suspended.

## Hand-off channel

A bounded channel of `Action` from the interception path to the worker.
`try_send` is non-blocking; a full channel drops the action with a warning
(FR-023).
