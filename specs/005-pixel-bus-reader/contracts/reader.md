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
- `decode_weapon_bar(sample: Rgb, tol) -> Option<WeaponBarSignal>`: `Some` when the
  green channel is within tolerance of the `0x5A` marker; the front class is `red >>
  4`, the back class is `red & 0x0F`, and the active bar is `blue` (0 unknown, 1
  front, 2 back), each decoded from its fixed code. `None` otherwise.

## Weapon-bar types

- `ActiveBar { Unknown, Front, Back }`, decoded from the blue code.
- `WeaponClass { Unknown, DualWield, TwoHanded, SwordAndShield, Bow,
  DestructionStaff, RestorationStaff }`, codes 0..6, mirrored byte-for-byte with the
  addon.
- `WeaponBarSignal { bar: ActiveBar, front: WeaponClass, back: WeaponClass }`.

## FishingSignal and PixelBusEvent

- `FishingSignal { Waiting, Bite, None }`.
- `PixelBusEvent { Heartbeat, SignalLost, FishingStarted, BiteDetected, FishingStopped, Latency(u16), WeaponBar(WeaponBarSignal) }`.
- `WeaponBar` is emitted only on a change in the decoded weapon-bar signal and only
  while a heartbeat is present, so per-attack redraws do not churn.

## SurfaceSampler (the seam)

- `sample(&self, x: u32, y: u32) -> Option<Rgb>`: the color at a client-area
  point, or `None` when the surface cannot be sampled.
- Implementations: `MockSampler` (crafted colors for tests), `GdiSampler`
  (Windows), `X11Sampler` (Linux).

## ReaderConfig

- `tolerance` (default 2), `heartbeat_timeout_ms` (default 2000), the four sample
  points (defaults `(8,8)`, `(24,8)`, `(40,8)`, `(56,8)`), and the sampling interval
  (default 100 ms fishing / 1000 ms otherwise; consumed by the runtime loop).

## PixelBusReader

- `new(config) -> PixelBusReader`.
- `observe(&mut self, b0: Option<Rgb>, b1: Option<Rgb>, b2: Option<Rgb>, b3: Option<Rgb>, now_ms) -> Vec<PixelBusEvent>`:
  - Status present: push Heartbeat, clear any lost state, record `now_ms`; decode
    fishing transitions and latency and push their events; decode the weapon-bar
    block and push a WeaponBar event only when it changes.
  - Status absent and `now_ms - last_heartbeat > heartbeat_timeout` and not
    already lost: push exactly one SignalLost, mark lost, reset fishing and the
    weapon-bar state.
- `sample_and_observe(&mut self, sampler: &dyn SurfaceSampler, now_ms) -> Vec<PixelBusEvent>`:
  samples the four points and calls `observe` (the thin runtime path).
- `signal_lost(&self) -> bool`.

## Safety invariants (tested)

1. Absent status past the timeout yields exactly one SignalLost (FR-004).
2. Fishing and latency are not decoded without a heartbeat (FR-005).
3. Latency failing marker or checksum yields no value (FR-010).
4. Tolerance boundary: a shift of `tol` matches; `tol + 1` does not (SC-004).
