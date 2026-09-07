# Contract: Action Authorization

## Shared authority

Input interception is the owner of atomic game, focus, suspension, menu, life, roll, world, and travel gates plus a monotonic invalidation epoch. Controllers receive typed projections containing only their applicable gates.

## Closure ordering

1. Publish a gate's unsafe value.
2. Advance the epoch when the transition is safe-to-unsafe.
3. Then acquire controller or weave locks and update local state.

Reopening reverses the order: synchronize controller-local recovery state first, then publish the open value. A close-reopen cycle therefore invalidates work captured before the close.

## Weave checkpoints

A generated weave must be authorized at all of these points:

1. Physical-event classification and queue creation.
2. Worker dequeue before the weave mutex.
3. Worker admission after the weave mutex.
4. Sequence start.
5. Every bounded wait poll.
6. Every generated operation.

Authorization requires all applicable gates open and the queued/admitted epoch equal to the current epoch.

## Cancellation

- Cancellation is sticky for the current sequence.
- No new key or mouse Down begins after cancellation.
- A matching Up is allowed only when the sink recorded that generated input as held.
- An unrelated Up is not emitted.
- Work rejected before its first Down consumes no cooldown.
- Cancelled work is discarded and never replayed after recovery.

## Fishing

- Suspension applies to initial cast, reel, recast, and timeout-driven recast.
- The final real sink checks authorization immediately before synthesis.
- If an interact is rejected, the controller does not advance as if the game received it.
- Suspension clears active state and deadline, records Suspended, and preserves the requested toggle.
- Unsuspending emits nothing.
- Fresh manual FishingStarted under open gates may enter Waiting without synthesis.
- Explicit off then on under open gates may begin a new cast.
- Menu state retains its existing defer-without-advance behavior.

## Invariants

- The hook callback remains non-blocking.
- Self-originated input always bypasses interception.
- Physical input passes through outside focused ESO.
- Application toggle hotkeys retain only their existing narrow gate exemptions.
