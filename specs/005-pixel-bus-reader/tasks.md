# Tasks: Pixel Bus Reader

**Feature**: `specs/005-pixel-bus-reader` | **Branch**: `005-pixel-bus-reader`

Test-first per constitution Principle III. The safety-critical signal-loss
behavior and all decoding are in the pure core, tested with crafted samples and an
injected clock before the code lands. Paths are repository-relative.

## Phase 1: Setup

- [x] T001 Declare `pub mod pixelbus;` in `src/lib.rs` and create compiling stub files `src/pixelbus/mod.rs`, `src/pixelbus/windows.rs`, `src/pixelbus/linux.rs` (backends behind their `cfg`), warning-free.

## Phase 2: Foundational

- [x] T002 Implement `Rgb`, `FishingSignal`, `PixelBusEvent`, `ReaderConfig` (defaults: tolerance 2, heartbeat_timeout_ms 2000, points (8,8)/(24,8)/(40,8), intervals 100/1000), and the `SurfaceSampler` trait plus `MockSampler` in `src/pixelbus/mod.rs`.

## Phase 3: User Story 3 - Decoders with tolerance and checksum (P2, foundational for others)

- [x] T003 [P] [US3] Write failing tests in `tests/pixelbus.rs`: `status_present` matches magenta within tolerance and rejects a shift of tolerance plus one; `fishing_signal` maps the waiting and bite colors and returns none otherwise; `decode_latency` returns red times four for valid marker and checksum (including the clamped maximum) and none for a corrupted marker or checksum.
- [x] T004 [US3] Implement the pure decoders `status_present`, `fishing_signal`, and `decode_latency` in `src/pixelbus/mod.rs` (FR-002, FR-009, FR-010). Run the decoder tests to green.

## Phase 4: User Story 1 - Heartbeat and signal loss (P1)

- [x] T005 [P] [US1] Write failing tests in `tests/pixelbus.rs`: a present status sample yields Heartbeat; an absent status past `heartbeat_timeout_ms` (via the injected clock) yields exactly one SignalLost and sets `signal_lost`; a returning status yields Heartbeat and clears the lost state; fishing and latency are not decoded while the status is absent.
- [x] T006 [US1] Implement `PixelBusReader::observe` heartbeat and signal-loss logic and `signal_lost()` in `src/pixelbus/mod.rs` (FR-003 to FR-005, FR-011). Run US1 tests to green.

## Phase 5: User Story 2 - Fishing transitions (P1)

- [x] T007 [P] [US2] Write failing tests in `tests/pixelbus.rs`: transitions into waiting (from absent and from bite) yield FishingStarted; into bite yields BiteDetected; into absent from waiting or bite yields FishingStopped; a Latency event is emitted for a valid latency sample while the heartbeat is present.
- [x] T008 [US2] Implement the fishing-transition and latency emission in `PixelBusReader::observe` (FR-006 to FR-008, FR-011). Run US2 tests to green.

## Phase 6: Platform samplers (thin)

- [x] T009 Implement `GdiSampler` in `src/pixelbus/windows.rs` (`cfg(windows)`): resolve the game window and read a client-area pixel via `GetDC` and `GetPixel`, returning `Rgb` or `None`. Compiles clippy-clean on the host.
- [x] T010 Implement `X11Sampler` in `src/pixelbus/linux.rs` (`cfg(target_os = "linux")`): read a pixel from the game window via `x11rb` `get_image`, returning `Rgb` or `None`. Type-check with the linux target.
- [x] T011 Implement `PixelBusReader::sample_and_observe(sampler, now_ms)` in `src/pixelbus/mod.rs` to sample the three points and delegate to `observe`.

## Phase 7: Polish and cross-cutting

- [x] T012 [P] Add module and item documentation across `src/pixelbus/*.rs`.
- [x] T013 Update `CHANGELOG.md` `[Unreleased]` with an Added line for the Pixel Bus Reader (adds the GDI feature to `windows-sys`; no new crates).
- [x] T014 Run CI parity: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked`, plus `cargo check --target x86_64-unknown-linux-gnu`.

## Dependencies and order

- Setup (T001) then Foundational (T002). Decoders (T003, T004) precede the state
  machine. US1 (T005, T006) and US2 (T007, T008) build on the decoders. Samplers
  (T009 to T011) depend on the core. Polish (T012 to T014) last; T014 is the gate.

## Parallel opportunities

- Test tasks (T003, T005, T007) share `tests/pixelbus.rs`; land sequentially.
- T009 (Windows) and T010 (Linux) touch different files and can run in parallel.

## MVP scope

The decoders (US3) plus the heartbeat and signal-loss state machine (US1) are the
minimum viable increment: correct decoding and the safety-critical signal-loss
behavior, fully tested. Fishing transitions and the OS samplers follow.
