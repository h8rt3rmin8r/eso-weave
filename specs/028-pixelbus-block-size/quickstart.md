# Quickstart: Pixel-Bus Block Size Single Source of Truth

Validation guide for slice 028. Proves the geometry single source of truth, the
safe managed re-deploy, and no behavior change at the default.

## Prerequisites

- Repo checked out on `main`, Rust toolchain per `rust-toolchain.toml`.
- No game required for the automated checks; the in-game step at the end is the
  owed field validation.

## Automated verification (CI parity)

Run in the foreground and watch to completion:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

Expected: all green. The new tests cover:

- `pixelbus`: `block_center` and `capture_dims` match the contract table for
  sizes 2, 4, 8, 16 (and 32); `sanitize_block_px` corrects odd/out-of-range/
  absent inputs and records a notice; the `block_px = 16` geometry equals the
  current release points and 64x16 capture.
- `beacon`: `render_lua(block_px)` rewrites only the `local BLOCK_PX` line and
  preserves the managed marker through a full install; a block-size re-deploy
  refuses an unmanaged folder (no write, no delete).

## Manual verification: settings and re-deploy (no game needed)

1. Run the app (`cargo run`). Open Settings, find the Pixel Beacon cluster, and
   locate the advanced Block size control.
2. With a managed PixelBeacon installed to a test AddOns directory (set an AddOns
   path override to a scratch folder and click Install), change Block size from
   16 to 8.
   - Expected: the deployed `PixelBeacon.lua` now reads `local BLOCK_PX = 8`; the
     manifest still contains the managed marker; a notice states the change takes
     effect after `/reloadui` and an app restart.
3. Point the AddOns override at a folder containing a hand-made `PixelBeacon`
   folder WITHOUT the managed marker, then change Block size.
   - Expected: the folder is not modified; a notice explains it is unmanaged.
4. Restart the app after a size change and confirm no config errors; inspect
   `config.json` and confirm `pixelbus.block_px` persisted and older configs load
   with `block_px` defaulting to 16.

## Owed in-game validation (field test, not a merge blocker)

This determines the smallest block size that reads reliably. Do it on a live
game and record the result before recommending or defaulting to a smaller size.

- OV-1: With the default `block_px = 16`, confirm the companion still reads all
  four blocks (heartbeat, fishing waiting/bite, latency, weapon bar) exactly as
  before this slice, across at least one non-1.0 UI scale.
- OV-2: Set `block_px = 8`, `/reloadui`, restart the app, and confirm all four
  blocks still decode on both a Windows (GDI capture) machine and, where
  available, a Linux (X11) machine.
- OV-3: Repeat OV-2 at `block_px = 4` and `block_px = 2`, noting the smallest size
  that still decodes cleanly (watch for blended edge pixels and capture-path
  filtering at the smallest sizes and any UI scale). Record the reliable floor.

Until OV-3 establishes a reliable smaller floor, the default stays at 16.

## Rollback

The change is additive and default-preserving. Reverting the commit restores the
prior behavior; existing `config.json` files with a `block_px` field still load
under the old code (the field is ignored by the old `RawPixelBus`).
