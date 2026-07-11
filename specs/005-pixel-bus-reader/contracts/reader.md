# Contract: Pixel Bus Reader

Language-neutral. The tasks phase fixes exact Rust signatures.

## Rgb and decoders (pure)

- `Rgb { r, g, b: u8 }`.
- `status_present(sample: Rgb, tol) -> bool`: true when the sample is magenta
  (`255, 0, 255`) within tolerance on every channel.
- `fishing_signal(sample: Rgb, tol) -> FishingSignal`: waiting for `0, 128, 255`,
  bite for `0, 255, 0`, else none.
- `decode_latency(sample: Rgb, tol) -> Option<u16>`: `Some(red * 4)` when the green
  channel is within tolerance of `0xA5` and `red + blue` is within tolerance of
  255; otherwise `None`.

## FishingSignal and PixelBusEvent

- `FishingSignal { Waiting, Bite, None }`.
- `PixelBusEvent { Heartbeat, SignalLost, FishingStarted, BiteDetected, FishingStopped, Latency(u16) }`.

## SurfaceSampler (the seam)

- `sample(&self, x: u32, y: u32) -> Option<Rgb>`: the color at a client-area
  point, or `None` when the surface cannot be sampled.
- Implementations: `MockSampler` (crafted colors for tests), `GdiSampler`
  (Windows), `X11Sampler` (Linux).

## ReaderConfig

- `tolerance` (default 2), `heartbeat_timeout_ms` (default 2000), the three sample
  points (defaults `(8,8)`, `(24,8)`, `(40,8)`), and the sampling interval
  (default 100 ms fishing / 1000 ms otherwise; consumed by the runtime loop).

## PixelBusReader

- `new(config) -> PixelBusReader`.
- `observe(&mut self, b0: Option<Rgb>, b1: Option<Rgb>, b2: Option<Rgb>, now_ms) -> Vec<PixelBusEvent>`:
  - Status present: push Heartbeat, clear any lost state, record `now_ms`; decode
    fishing transitions and latency and push their events.
  - Status absent and `now_ms - last_heartbeat > heartbeat_timeout` and not
    already lost: push exactly one SignalLost, mark lost, reset fishing.
- `sample_and_observe(&mut self, sampler: &dyn SurfaceSampler, now_ms) -> Vec<PixelBusEvent>`:
  samples the three points and calls `observe` (the thin runtime path).
- `signal_lost(&self) -> bool`.

## Safety invariants (tested)

1. Absent status past the timeout yields exactly one SignalLost (FR-004).
2. Fishing and latency are not decoded without a heartbeat (FR-005).
3. Latency failing marker or checksum yields no value (FR-010).
4. Tolerance boundary: a shift of `tol` matches; `tol + 1` does not (SC-004).
