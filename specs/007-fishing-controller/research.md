# Phase 0 Research: Fishing Controller

All decisions were made under the Build-Phase Autopilot Protocol against the
constitution, the master specification (section 8), and existing code patterns.
None were escalated.

## Decision: Event-and-tick-driven, non-blocking state machine with an injected clock

- **Decision**: The controller exposes `on_event(event, now_ms, sink)`,
  `set_enabled(enabled, now_ms, sink)`, and `tick(now_ms, sink)`. All delays and
  timeouts are deadlines (an absolute millisecond time plus a timer kind) checked
  in `tick` against the injected clock; nothing sleeps.
- **Rationale**: The reader (feature 005) is already clock-injected and polled per
  tick; a matching non-blocking controller composes cleanly with the future worker
  loop and is fully unit-testable with a virtual clock, satisfying Principle III.
  Blocking sleeps would freeze the loop and hide behind wall-clock timing in tests.
- **Alternatives considered**: A blocking sequence using a sink `wait` like the
  weave engine (rejected: the controller is event-driven, not a fixed sequence, and
  must interleave detector events with timeouts). A separate timer thread (rejected:
  unnecessary; a deadline plus tick is simpler and deterministic).

## Decision: A dedicated `FishingSink` seam over the input backend, not the weave sink

- **Decision**: Define `FishingSink { fn key(&mut self, key: Key, transition:
  Transition); }` with a `MockFishingSink` (records ops) and a
  `RealFishingSink<B: InputBackend>` that calls `backend.synthesize`. The interact
  emission is a `Down` then an `Up`.
- **Rationale**: The controller only ever emits key transitions and is non-blocking,
  so the weave `WeaveSink` (which carries a blocking `wait` and a mouse variant, and
  lives in the weave engine) is the wrong seam and would couple fishing to feature
  003. A minimal key-only sink keeps the dependency graph (007 depends on 002 and
  005) intact and matches Principle V's bounded scope.
- **Alternatives considered**: Reusing `WeaveSink`/`InputOp` (rejected: drags in the
  weave engine and a blocking `wait`). Driving `InputBackend` directly from the
  controller (rejected: not injectable; the mock sink is the test seam).

## Decision: `set_enabled(bool)` semantics for the toggle, for idempotency

- **Decision**: The fishing toggle is modeled as `set_enabled(true|false)`.
  Enabling from Disabled arms and casts once; enabling when already active is a
  no-op; disabling from any state returns to Disabled; disabling when already
  Disabled is a no-op.
- **Rationale**: The spec requires idempotent toggles ("redundant on does not
  re-cast; redundant off emits nothing"), which a literal flip cannot express. A
  set-enabled input makes both the in-game hotkey and the GUI button drive the same
  state safely (both call the same operation).
- **Alternatives considered**: A flip-toggle (rejected: not idempotent, double
  presses would double-cast).

## Decision: Recast is the auto-continue path; Armed is the retry-then-disarm path

- **Decision**: On BiteDetected: Reeling holds a reel deadline (bite + reel_delay);
  when due, emit the reel interact and enter Recast with a recast deadline (now +
  recast_delay); when due, emit the recast interact and set an arm-timeout deadline;
  FishingStarted returns to Waiting, and an arm-timeout in Recast returns to Armed
  (which re-casts), matching the master diagram's `Recast --> Armed: recast
  timeout`. An arm-timeout in Armed disarms to Disabled.
- **Rationale**: This reproduces the master specification's two interacts after a
  bite (reel then recast) and its distinct Recast auto-continue versus Armed
  give-up timeouts, while keeping every delay a deadline.
- **Alternatives considered**: Collapsing Recast into Armed (rejected: loses the
  auto-continue-versus-retry distinction the master diagram draws). Emitting a single
  interact after a bite (rejected: contradicts the master prose, which sends the
  interact twice).

## Decision: Add `Key::E` for the default interact key

- **Decision**: Add a `Key::E` variant to `src/input/key.rs` (with `as_str` `"e"`
  and `parse` `"e"`), and default the fishing `interact_key` to `Key::E`.
- **Rationale**: ESO's default use/interact key is `E`, and the input engine's `Key`
  enum did not include it. The interact key is configurable, but the out-of-the-box
  default must be the real interact key. Extending the input key set is additive and
  the `input` module is the right owner.
- **Alternatives considered**: Defaulting to an existing key like `F2` (rejected:
  `F2` is the fishing toggle default, not the interact key). A free-form key string
  in fishing config bypassing `Key` (rejected: diverges from the input engine's
  typed key model and its settings serialization).

## Decision: Additive `fishing` settings section, loaded like `timing`

- **Decision**: Add `fishing: serde_json::Value` to `Settings`, owned by the fishing
  module, with a `load`/`store` that uses the same `checked` range validation the
  weave timing uses and `Key::parse` for the interact key, falling back to defaults
  with a notice on invalid values. No `schema_version` bump.
- **Rationale**: Mirrors the established additive opaque-section pattern (`timing`,
  `skills`, `beacon`) and the constitution's additive-settings rule, and reuses the
  weave module's proven range-check-with-notice approach.
- **Alternatives considered**: A typed field on `Settings` (rejected: couples config
  to the fishing module and breaks the opaque-section pattern). A schema bump
  (rejected: unnecessary for an additive field).

## Decision: `PixelBusDetector` adapts reader events; only `map_event` carries logic

- **Decision**: `map_event(PixelBusEvent) -> Option<DetectorEvent>` is a pure,
  unit-tested mapping that drops `Latency`. `PixelBusDetector` owns a
  `PixelBusReader` and a `SurfaceSampler` and implements `BiteDetector::poll(now_ms)`
  by calling `sample_and_observe` and mapping the events.
- **Rationale**: Keeps the correctness in the tested pure mapping and the reader; the
  adapter is a thin glue layer, matching the maximal-testable-core pattern.
- **Alternatives considered**: Re-deriving fishing state from raw samples in the
  detector (rejected: duplicates the reader). Passing Latency through the detector
  (rejected: the BiteDetector event set excludes it and the controller ignores it).
