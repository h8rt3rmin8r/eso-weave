# Research: Pixel Bus Reader

Phase 0 decisions. No open NEEDS CLARIFICATION items remain.

## Pure decoders plus a state machine

**Decision**: Three pure functions decode a sampled `Rgb`: status-present (match
magenta within tolerance), fishing-signal (waiting, bite, or none), and
decode-latency (validate marker and checksum, return red times four). A
`PixelBusReader::observe(b0, b1, b2, now_ms)` state machine turns each timed
sample into events and tracks the heartbeat timeout.

**Rationale**: All correctness and the safety-critical signal-loss behavior live
in pure, clock-injected code that is fully testable without any real sampling
(FR-012, SC-005).

**Alternatives considered**: Decoding inside the OS sampler (rejected: couples
safety logic to untestable code).

## Tolerance and checksum

**Decision**: A configurable per-channel tolerance (default plus or minus 2) is
applied to every channel comparison. Latency validates when the green channel is
within tolerance of `0xA5` and red plus blue is within tolerance of 255; the value
is red times four.

**Rationale**: Absorbs compositor rounding while the checksum guards against
misreads (FR-002, FR-009, FR-010).

## Heartbeat timeout and signal loss

**Decision**: The reader records the last heartbeat time. When the status block is
absent (no match or no sample) and the time since the last heartbeat exceeds the
configurable timeout (default 2000 ms), it emits exactly one signal-loss event and
marks the signal lost until the status block returns. Fishing and latency decode
only while the heartbeat is present.

**Rationale**: Directly implements the safety behavior (FR-003 to FR-005). A
single emission avoids event storms.

## Surface sampling

**Decision**: A `SurfaceSampler` trait returns an `Rgb` for a client-area point, or
nothing when the window cannot be sampled. Windows uses `GetDC` plus `GetPixel` on
the game window (a single-pixel read); Linux uses `x11rb` `get_image` on the game
window and reads the pixel. A `MockSampler` returns crafted colors for tests.

**Rationale**: A minimal seam keeps the OS surface out of the tested path.
`GetPixel` on a hardware-accelerated game window is the mechanism the
specification chose; its behavior on a DirectX surface is validated in-game (noted
as a runtime risk).

**Alternatives considered**: Full-window capture then crop (rejected: heavier than
three point reads for this purpose; may be revisited if `GetPixel` proves
unreliable on the game surface).

## Events

**Decision**: `PixelBusEvent` is Heartbeat, SignalLost, FishingStarted,
BiteDetected, FishingStopped, or Latency(value), matching the section 8.1 detector
event set plus latency so the later fishing controller consumes them directly
(FR-011).
