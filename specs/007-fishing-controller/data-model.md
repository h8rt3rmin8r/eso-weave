# Phase 1 Data Model: Fishing Controller

All types live in the `fishing` module. The controller and config are pure; the
`RealFishingSink` and `PixelBusDetector` are the thin adapters.

## DetectorEvent

The `BiteDetector` event set (no Latency).

| Variant | Meaning |
| --- | --- |
| `Heartbeat` | The signal is live (observed, no state change). |
| `FishingStarted` | A cast became active and is waiting. |
| `BiteDetected` | A bite occurred. |
| `FishingStopped` | Fishing stopped (cast ended without an active bite path). |
| `SignalLost` | The beacon heartbeat was lost. |

## BiteDetector (trait)

- `fn poll(&mut self, now_ms: u64) -> Vec<DetectorEvent>`: advances the detector to
  `now_ms` and returns any events produced.
- `StubDetector`: a test detector returning scripted events per poll.
- `PixelBusDetector`: wraps a `PixelBusReader` and a `SurfaceSampler`; `poll` calls
  `reader.sample_and_observe(sampler, now_ms)` and maps each event through
  `map_event`, dropping `None` (Latency).

### map_event (pure)

`map_event(PixelBusEvent) -> Option<DetectorEvent>`:

| PixelBusEvent | DetectorEvent |
| --- | --- |
| `Heartbeat` | `Heartbeat` |
| `SignalLost` | `SignalLost` |
| `FishingStarted` | `FishingStarted` |
| `BiteDetected` | `BiteDetected` |
| `FishingStopped` | `FishingStopped` |
| `Latency(_)` | `None` (dropped) |

## FishingState

The observable controller state (via `state()`).

| Variant | Meaning |
| --- | --- |
| `Disabled` | Not fishing; emits nothing. |
| `Armed` | Cast sent; awaiting FishingStarted until the arm timeout. |
| `Waiting` | Cast active; awaiting a bite. |
| `Reeling` | Bite seen; awaiting the reel deadline to emit the reel interact. |
| `Recast` | Reel emitted; awaiting the recast deadline, then the recast interact, then FishingStarted. |

Internally the controller also holds an optional deadline `(at_ms, TimerKind)`
where `TimerKind` is one of `ArmTimeout`, `ReelDue`, `RecastDue`, or
`RecastArmTimeout`. The deadline is cleared on any transition that leaves the
scheduling state.

## Transition table (event and tick driven)

Inputs: `set_enabled(true|false)`, detector events, and `tick` (deadline firing).
"emit" means a full interact (key Down then Up).

| From | Input | To | Emit | Deadline set |
| --- | --- | --- | --- | --- |
| Disabled | set_enabled(true) | Armed | interact (cast) | ArmTimeout = now + arm_timeout_ms |
| any != Disabled | set_enabled(false) | Disabled | none | cleared |
| any != Disabled | SignalLost | Disabled | none | cleared |
| any | Heartbeat | unchanged | none | unchanged |
| Armed | FishingStarted | Waiting | none | cleared |
| Armed | BiteDetected (defensive) | Reeling | none | ReelDue = now + reel_delay_ms |
| Armed | tick @ ArmTimeout | Disabled | none | cleared |
| Waiting | BiteDetected | Reeling | none | ReelDue = now + reel_delay_ms |
| Waiting | FishingStopped | Armed | interact (cast) | ArmTimeout = now + arm_timeout_ms |
| Reeling | tick @ ReelDue | Recast | interact (reel) | RecastDue = now + recast_delay_ms |
| Reeling | FishingStopped | Armed | interact (cast) | ArmTimeout (pending reel cancelled) |
| Recast | tick @ RecastDue | Recast | interact (recast) | RecastArmTimeout = now + arm_timeout_ms |
| Recast | FishingStarted | Waiting | none | cleared |
| Recast | tick @ RecastArmTimeout | Armed | interact (cast) | ArmTimeout = now + arm_timeout_ms |
| Recast | FishingStopped | Armed | interact (cast) | ArmTimeout (pending recast cancelled) |

Events not listed for a state are ignored (no emit, no state change). A deadline
fires only when `now_ms >= at_ms`; a tick exactly at the deadline fires it.

## FishingConfig

| Field | Type | Default | Meaning |
| --- | --- | --- | --- |
| `arm_timeout_ms` | `u32` | 5000 | Max wait for FishingStarted after a cast/recast. |
| `reel_delay_ms` | `u32` | 100 | Delay after BiteDetected before the reel interact. |
| `recast_delay_ms` | `u32` | 3000 | Delay after reeling before the recast interact. |
| `interact_key` | `Key` | `Key::E` | The key synthesized to cast, reel, and recast. |

- Loaded from the additive `fishing` settings section: absent or null yields
  defaults; an out-of-range timing falls back to its default with an
  `InvalidValue` notice (the weave `checked` pattern); an unparsable interact key
  falls back to the default with a notice.

## FishingSink (trait)

- `fn key(&mut self, key: Key, transition: Transition)`.
- `MockFishingSink { ops: Vec<(Key, Transition)> }` records emitted operations.
- `RealFishingSink<B: InputBackend>` calls `backend.synthesize(key, transition)`
  and logs a warning on synthesis failure (never panics, never blocks).

## FishingController

| Item | Signature | Notes |
| --- | --- | --- |
| `new` | `fn new(config: FishingConfig) -> Self` | Starts Disabled. |
| `state` | `fn state(&self) -> FishingState` | Current observable state. |
| `set_enabled` | `fn set_enabled(&mut self, enabled: bool, now_ms: u64, sink: &mut impl FishingSink)` | Idempotent arm/disarm. |
| `on_event` | `fn on_event(&mut self, event: DetectorEvent, now_ms: u64, sink: &mut impl FishingSink)` | Drives transitions. |
| `tick` | `fn tick(&mut self, now_ms: u64, sink: &mut impl FishingSink)` | Fires a due deadline. |
