# Contract: Capture Seam and Pixel Extraction

This slice adds no network or IPC surface. Its unit-tested contract is the pure
`strip_pixel` helper and the unchanged reader/decoder behavior; the OS capture is
verified in-game.

## strip_pixel (pure, unit-tested)

```rust
pub fn strip_pixel(buffer: &[u8], width: u32, height: u32, x: u32, y: u32) -> Option<Rgb>;
```

Behavioral contract:

1. Reads a 32-bit little-endian BGRA pixel: at offset `(y * width + x) * 4` the
   bytes are `b, g, r, a`; returns `Rgb::new(r, g, b)`.
2. Returns `None` when `x >= width`, `y >= height`, or the buffer is shorter than
   `(y * width + x) * 4 + 3`.
3. Pure and deterministic; no I/O.

Required tests (in `src/pixelbus/mod.rs`):

- A crafted 2x2 BGRA buffer returns the expected `Rgb` for each in-range point
  (channel order correct: magenta `#FF00FF` stored as bytes `FF 00 FF xx`).
- Out-of-range `x` or `y` returns `None`.
- A truncated buffer returns `None` rather than panicking.

## SurfaceSampler::prepare (default no-op)

- Adding `prepare` with a default empty body does not change `MockSampler` or the
  Linux backend behavior.
- `PixelBusReader::sample_and_observe` calls `prepare()` once, then samples the four
  points; the returned events are identical to before for any given set of sampled
  colors (the existing reader/decoder tests still pass unchanged).

## Reader behavior preserved (existing tests unchanged)

- `observe` emits the same events for the same samples; the added `had_heartbeat`
  flag only drives `debug!`/`trace!` diagnostics and changes no event.
- Fishing still degrades to disabled on `SignalLost` (unchanged signal-loss path).

## In-game verification (not unit-tested)

The Windows capture backend and the end-to-end fix are verified against the running
game per `quickstart.md`: with all preconditions met, the log shows the heartbeat
acquired and, on a real cast, the fishing signal becomes Waiting and the UI
advances Casting to Fishing to Reeling.
