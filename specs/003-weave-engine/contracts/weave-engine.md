# Contract: Weave Engine Core and Sink Seam

Language-neutral. The tasks phase fixes exact Rust signatures.

## sequence_for (pure)

- `sequence_for(slot, timing) -> Vec<WeaveStep>`
  - Returns the ordered steps for the slot's weave type, substituting per-slot
    overrides where present and the global timing otherwise. No side effects.

## WeaveSink (the seam)

- `emit(&mut self, op: InputOp)` performs one synthesized operation.
- `wait(&mut self, ms: u32)` waits the given duration.
- `now_ms(&self) -> u64` returns a monotonic millisecond timestamp.

Implementations:

- `MockSink`: records each `emit` and `wait` as an ordered `WeaveStep` log and
  advances a virtual clock on `wait`, so `now_ms` is deterministic. Used for all
  sequence and cooldown tests.
- `RealSink`: `emit` drives the Input Engine synthesis (key and mouse), `wait`
  sleeps the worker thread, `now_ms` reads a monotonic clock.

## WeaveEngine

- `new(config) -> WeaveEngine`
- `handle(&mut self, action, sink)`:
  - Maps the action to its slot (toggles map to nothing and are ignored, FR-014).
  - If the slot is inactive, does nothing (defense in depth; the input layer
    already passed the key through).
  - If within `global_cooldown` of the last executed weave (via `sink.now_ms()`),
    drops the request without running a sequence (FR-010).
  - Otherwise records the start time, builds the sequence with `sequence_for`, and
    runs each step through the sink.
- `slot_for_action(action) -> Option<&SkillSlot>`
- `apply_activity(&self, input_engine)`: sets each weave action's activity in the
  Input Engine from its slot's active flag (FR-002).
- `load(settings)` and `store(settings)` integrate with the Config Store, with
  fallback notices for invalid timing (FR-015, FR-016).

## Input Engine additions (S002 integration)

- `MouseButton { Primary, Secondary }` and
  `InputBackend::synthesize_mouse(button, transition)`.
- `InputEngine::set_action_active(action, active)`; `classify` treats a bound but
  inactive action as pass-through.

## Safety invariants (tested)

1. `sequence_for` yields exactly the specified order per weave type (FR-003 to
   FR-006).
2. A request within the cooldown window runs no sequence (FR-010).
3. An inactive slot's action is passed through by `classify` (FR-002).
4. No engine work runs on the interception path; execution is worker-side.
