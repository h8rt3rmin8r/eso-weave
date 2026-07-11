# Quickstart: Pixel Bus Reader

Build and validation guide. Implementation lives in `src/pixelbus/` and
`tasks.md`.

## Build and quality gate

```sh
cargo build
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
cargo check --target x86_64-unknown-linux-gnu
```

Expected: all succeed on the host; the Linux sampler type-checks on the linux
target.

## Validation scenarios

Covered by `tests/pixelbus.rs` using crafted `Rgb` samples and an injected clock.
See `contracts/reader.md` and `data-model.md`.

1. Heartbeat and signal loss (US1, SC-001): a present status block yields a
   heartbeat; removing it and advancing the clock past the timeout yields exactly
   one signal-loss; restoring it yields a heartbeat and clears the lost state.
2. Fishing transitions (US2, SC-002): waiting, bite, and absent transitions yield
   fishing-started, bite-detected, and fishing-stopped in order.
3. Latency and checksum (US3, SC-003): representative encodings (including the
   clamped maximum) decode to the correct value; a corrupted marker or checksum
   yields no latency.
4. Tolerance boundary (SC-004): a channel shifted by the tolerance still matches;
   shifted by tolerance plus one does not.
5. Gating (FR-005): with the status block absent, fishing and latency are not
   decoded.

## Manual in-game validation (later, on a live client)

Point the GDI or X11 sampler at the running client with the PixelBeacon addon
installed and confirm heartbeats, fishing transitions during fishing, and latency
values, and that removing the beacon raises signal loss.
