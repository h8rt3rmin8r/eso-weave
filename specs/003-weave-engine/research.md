# Research: Weave Engine

Phase 0 decisions. No open NEEDS CLARIFICATION items remain.

## Pure sequence builder plus a single sink seam

**Decision**: `sequence_for(slot, timing) -> Vec<WeaveStep>` is a pure function
producing the ordered steps. Execution and cooldown use one `WeaveSink` trait with
`emit(op)`, `wait(ms)`, and `now_ms()`. A `MockSink` records steps and controls the
clock; a `RealSink` drives synthesis and real sleeping.

**Rationale**: A pure builder makes sequence correctness (the highest-value
property) assertable as data, with no timing or synthesis involved. Folding emit,
wait, and clock into one sink keeps a single ordered event log for tests and keeps
the worker loop simple.

**Alternatives considered**: Separate `Synthesizer` and `Clock` traits (rejected:
two logs make ordered assertions awkward; one sink is simpler). Executing inline
without a builder (rejected: sequence correctness would only be testable through
timing side effects).

## Cooldown model

**Decision**: Cooldown is measured start-to-start against a monotonic millisecond
clock from the sink. On a non-dropped execution the engine records `now_ms()` at
the start; a later action whose `now_ms()` is within `global_cooldown` of that
timestamp is dropped without running a sequence.

**Rationale**: Matches "minimum interval between weave executions" (FR-010) and is
deterministic under the virtual clock (FR-011, SC-002, SC-006).

**Alternatives considered**: End-to-start measurement (rejected: sequence duration
would leak into the cooldown window inconsistently across weave types).

## Inactive-slot pass-through via input activity

**Decision**: Extend `InputEngine` with a per-action active set (all actions active
by default) and `set_action_active(action, active)`. `classify` treats a bound but
inactive action as pass-through (not suppressed, not handed off). The weave engine
sets each weave action's activity from its slot's active flag on load and on
change.

**Rationale**: Honors section 7.1 ("an inactive slot's key passes through
unmodified") without weakening section 6 suppression. The default all-active keeps
the existing S002 tests valid, and a new test covers inactive pass-through. The
added check is a brief, uncontended lock on the same non-blocking path.

**Alternatives considered**: Dropping inactive actions in the weave engine after
hand-off (rejected: the key was already suppressed, so it would not pass through,
violating 7.1).

## Mouse synthesis

**Decision**: Add a `MouseButton` (primary, secondary) and `synthesize_mouse` to
the input backend. Windows uses `SendInput` with the mouse event flags; Linux uses
uinput `BTN_LEFT` and `BTN_RIGHT`; the mock records them. Injected/self-origin
handling matches key synthesis so synthesized clicks are never re-intercepted.

**Rationale**: Weaves need left and right mouse actions; synthesis belongs to the
input backend that already owns key synthesis, keeping one synthesis surface.

## Persistence

**Decision**: Add additive `skills` and `timing` sections to the settings. Absent
sections load the section 7.1 and 7.3 defaults, so no schema version bump. A timing
value that is missing or out of range falls back to its global default with a
notice, reusing the Config Store notice mechanism.

**Rationale**: Consistent with the S002 additive `bindings` section and the S001
resilience contract (FR-015, FR-016).

**Alternatives considered**: A schema bump with migration (rejected: unnecessary
for additive sections).
