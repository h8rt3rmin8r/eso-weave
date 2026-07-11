# Contract: Fishing Controller (`fishing` module)

The public surface the later worker loop and GUI consume. Signatures are the
intended Rust shape; names may be refined during implementation provided the
behavior and safety guarantees below hold.

## Types

```rust
pub enum DetectorEvent { Heartbeat, FishingStarted, BiteDetected, FishingStopped, SignalLost }

pub trait BiteDetector {
    fn poll(&mut self, now_ms: u64) -> Vec<DetectorEvent>;
}

pub enum FishingState { Disabled, Armed, Waiting, Reeling, Recast }

pub struct FishingConfig {
    pub arm_timeout_ms: u32,   // default 5000
    pub reel_delay_ms: u32,    // default 100
    pub recast_delay_ms: u32,  // default 3000
    pub interact_key: Key,     // default Key::E
}

pub trait FishingSink {
    fn key(&mut self, key: Key, transition: Transition);
}
```

## Pure event mapping (unit tested)

```rust
pub fn map_event(event: PixelBusEvent) -> Option<DetectorEvent>;
```

- Maps each reader event to its detector event; `Latency(_)` maps to `None`.

## Controller

```rust
impl FishingController {
    pub fn new(config: FishingConfig) -> Self;                 // starts Disabled
    pub fn state(&self) -> FishingState;
    pub fn set_enabled(&mut self, enabled: bool, now_ms: u64, sink: &mut impl FishingSink);
    pub fn on_event(&mut self, event: DetectorEvent, now_ms: u64, sink: &mut impl FishingSink);
    pub fn tick(&mut self, now_ms: u64, sink: &mut impl FishingSink);
}
```

### Behavior (see data-model.md for the full transition table)

- `set_enabled(true)` from Disabled enters Armed and emits one interact (cast),
  arming the arm-timeout deadline. From any other state it is a no-op.
- `set_enabled(false)` from any active state returns to Disabled, clears any
  deadline, and emits nothing. From Disabled it is a no-op.
- On FishingStarted: Armed and Recast go to Waiting.
- On BiteDetected: Waiting (and defensively Armed) go to Reeling with a reel
  deadline.
- On FishingStopped: Waiting, Reeling, and Recast return to Armed (re-cast), any
  pending interact cancelled.
- `tick` fires a due deadline: ReelDue emits the reel interact and enters Recast;
  RecastDue emits the recast interact and arms the recast arm-timeout; ArmTimeout
  in Armed disarms to Disabled; the recast arm-timeout in Recast returns to Armed
  and re-casts.
- A deadline fires only when `now_ms >= at_ms`.

### Safety guarantees (required non-weakened tests)

1. **SignalLost disables**: from Armed, Waiting, Reeling, or Recast, `on_event(SignalLost)`
   enters Disabled, clears the deadline, and emits nothing; advancing the clock
   past any previously scheduled deadline emits nothing.
2. **Cancellation on leaving**: any transition out of a state that scheduled a
   delayed interact clears that deadline, so the queued interact never fires.
3. **Silent while Disabled**: no interact is emitted while Disabled, for any tick or
   ignored event.
4. **Idempotent toggles**: `set_enabled(true)` when already active emits no second
   cast; `set_enabled(false)` when Disabled emits nothing.

## Configuration

```rust
impl FishingConfig {
    pub fn load(value: &serde_json::Value, notices: &mut Vec<Notice>) -> FishingConfig;
    pub fn store(&self) -> serde_json::Value;
}
```

- `load` on null yields defaults; an out-of-range timing falls back to its default
  with an `InvalidValue` notice; an unparsable `interact_key` falls back to
  `Key::E` with a notice. Persisted via the additive `Settings.fishing` opaque
  section (no `schema_version` bump).

## Detector adapter

```rust
pub struct PixelBusDetector { /* PixelBusReader + a SurfaceSampler */ }
impl BiteDetector for PixelBusDetector {
    fn poll(&mut self, now_ms: u64) -> Vec<DetectorEvent>; // sample_and_observe then map_event
}
```

## Testability

- `StubDetector` returns scripted events; `MockFishingSink` records `(Key,
  Transition)` operations; the clock is an explicit `now_ms` argument. The entire
  controller and its timing are verifiable with these, with no real device, game,
  or wall clock (FR-013).
