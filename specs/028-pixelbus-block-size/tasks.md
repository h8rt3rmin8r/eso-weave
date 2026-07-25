# Tasks: Pixel-Bus Block Size Single Source of Truth

**Feature**: [spec.md](spec.md) | **Plan**: [plan.md](plan.md) |
**Contract**: [contracts/geometry.md](contracts/geometry.md)

Test-first per Constitution Principle III: each behavior gets a failing test
before the implementation that satisfies it. Paths are repo-relative.

## Phase 1: Setup

- [x] T001 Confirm the baseline merge gate is green before any change: run `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all --locked` in the foreground.

## Phase 2: Foundational (pure geometry, blocks all user stories)

- [x] T002 [P] Add failing unit tests for the geometry helpers in tests/pixelbus.rs: `block_center(block_px, 0..=3)` and `capture_dims(block_px)` equal the contract table for block sizes 2, 4, 8, 16, and 32 (per specs/028-pixelbus-block-size/contracts/geometry.md).
- [x] T003 Add the geometry single-source-of-truth primitives in src/pixelbus/mod.rs: `pub const NUM_BLOCKS: u32 = 4`, `pub const DEFAULT_BLOCK_PX: u32 = 16`, `pub const MIN_BLOCK_PX: u32 = 2`, `pub const MAX_BLOCK_PX: u32 = 32`, `pub fn block_center(block_px: u32, index: u32) -> (u32, u32)` = `(block_px * index + block_px / 2, block_px / 2)`, and `pub fn capture_dims(block_px: u32) -> (u32, u32)` = `(block_px * NUM_BLOCKS, block_px)`. Make T002 pass.
- [x] T004 [P] Add failing unit tests for `sanitize_block_px` in tests/pixelbus.rs: odd rounds down to the next even, below-range clamps to 2, above-range clamps to 32, and a changed value records exactly one `NoticeKind::InvalidValue` notice while an in-range even value records none.
- [x] T005 Implement `pub fn sanitize_block_px(value: u32, notices: &mut Vec<Notice>) -> u32` in src/pixelbus/mod.rs (even-and-range correction with a non-fatal notice on change). Make T004 pass.

## Phase 3: User Story 1 - Block size is a single shared value (P1)

**Goal**: One `block_px` drives the reader points, the Windows capture region,
and the deployed addon; default 16 is byte-for-byte unchanged.

**Independent test**: With default block size the four blocks decode exactly as
today; derived points match the addon centers for every supported size.

- [x] T006 [P] [US1] Add failing tests in tests/pixelbus.rs: a default `ReaderConfig` has `block_px == 16` and its four derived points equal `(8,8),(24,8),(40,8),(56,8)`; for sizes 2/4/8/16/32 the four points match the contract table; `sample_and_observe` reads the derived points (via a `MockSampler` seeded at those coordinates).
- [x] T007 [US1] Refactor `ReaderConfig` in src/pixelbus/mod.rs: add `pub block_px: u32`, remove the four stored `*_point` fields, and expose `status_point()/fishing_point()/latency_point()/weapon_point()` as methods computed by `block_center`. Update `Default` (block_px = 16) and `sample_and_observe` to use the methods. Make T006 pass.
- [x] T008 [US1] Update `GdiSampler` in src/pixelbus/windows.rs to derive its capture region from `block_px`: replace the `CAPTURE_W`/`CAPTURE_H` consts with per-instance dimensions from `capture_dims(block_px)`, and change the constructor to `for_window(title: &str, block_px: u32)`.
- [x] T009 [US1] Thread `block_px` through `resolve_sampler` in src/main.rs so the Windows sampler is built with `reader_config.block_px` (the Linux `X11Sampler` needs no size and stays as is).
- [x] T010 [P] [US1] Add failing tests in tests/beacon.rs: `render_lua(block_px)` changes only the `local BLOCK_PX = N` line (all other bytes identical to the embedded Lua) for sizes 8 and 16, and a full `install(..., block_px)` writes a Lua whose `BLOCK_PX` matches while the manifest still passes `has_managed_marker`.
- [x] T011 [US1] Add `pub fn render_lua(block_px: u32) -> String` in src/beacon/mod.rs (rewrite only the `local BLOCK_PX` line, mirroring `rewrite_api_version`), give `install` a `block_px` parameter and write `render_lua(block_px)` instead of the verbatim `LUA`, and update all `install` call sites. Make T010 pass.
- [x] T012 [US1] De-hardcode the stale "16 by 16" wording in the addon header comment in addon/PixelBeacon/PixelBeacon.lua so it reads as a templated size, and confirm every geometry expression there derives from `BLOCK_PX` (no change to logic).

## Phase 4: User Story 2 - Shrink the overlay with an advanced setting (P2)

**Goal**: An advanced Block size setting persists, and changing it drives a
managed addon re-deploy.

**Independent test**: Change the setting on a managed test install; the deployed
Lua's `BLOCK_PX` updates and the managed marker survives; unmanaged/absent
installs are not written.

- [x] T013 [US2] Add `block_px: Option<u32>` to `RawPixelBus` and wire `load_reader_config` (derive via `sanitize_block_px`, defaulting to 16) and `store_reader_config` (emit `block_px`) in src/pixelbus/mod.rs; add a round-trip test and an "older config without block_px loads as 16" test in tests/pixelbus.rs.
- [x] T014 [US2] Add the advanced Block size control to the Pixel Beacon cluster in src/app/ui.rs (an even-stepped numeric control bounded to 2..=32) and its label/help strings in src/app/strings.rs, with help text stating that changing it requires a PixelBeacon re-deploy and takes effect after `/reloadui` and an app restart.
- [x] T015 [US2] In src/app/mod.rs, drive the re-deploy: capture the pre-apply `block_px`, and in `apply_settings` when the applied `block_px` differs and `beacon::status` is managed (`ManagedUpToDate`/`ManagedVersionMismatch`), call `install` with the new size; on `Unmanaged` or `NotInstalled` do not write and record a notice; pass `block_px` through `install_beacon`.
- [x] T016 [US2] Add failing-then-passing tests in tests/beacon.rs (and tests/app_settings.rs as needed): a managed re-deploy at a new size rewrites the deployed `BLOCK_PX` and preserves the marker; a re-deploy against an unmanaged folder writes nothing and deletes nothing; a not-installed root is left untouched.

## Phase 5: User Story 3 - Invalid or unsupported sizes never break the app (P3)

**Goal**: An invalid persisted block size is corrected with a notice, never a
crash.

**Independent test**: Load a config with an invalid `block_px` and confirm a
corrected value plus a recorded notice, no panic.

- [x] T017 [US3] Add a test (tests/pixelbus.rs or tests/app_settings.rs) that `load_reader_config` on a `pixelbus` section with an odd, out-of-range, and wrong-typed `block_px` yields a corrected even in-range value and records a notice, and that a valid even value records none. (Implementation is already provided by T005 + T013; this locks the end-to-end behavior.)

## Phase 6: Polish and cross-cutting

- [x] T018 [P] Update CHANGELOG.md `[Unreleased]`: an Added line for the configurable block size and geometry single source of truth, plus a dated Decisions entry for the block_px SSOT, the managed-only re-deploy behavior, and the even 2..=32 bound.
- [~] T019 [P] OMITTED by decision: no docs/plans build-plan row is added. Build plans decompose the master specification; slice 028 is issue-driven (issue #1), the same as slice 027, which also carries no plan row. Traceability lives in specs/028-pixelbus-block-size/ and the CHANGELOG Decisions entry. Recorded here to make the omission explicit.
- [x] T020 Record the owed in-game validation (quickstart OV-1..OV-3) as the tracked follow-up for the minimum-reliable-size determination; keep the default at 16.
- [x] T021 Run the full merge gate in the foreground and confirm green: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked`.

## Dependencies

- Phase 2 (T002-T005) blocks all user stories (helpers used everywhere).
- US1 (T006-T012) depends on the geometry helpers (T003).
- US2 (T013-T016) depends on US1 (block_px on `ReaderConfig`, `render_lua`/`install` signature) and on `sanitize_block_px` (T005) for `load`.
- US3 (T017) depends on T005 + T013 (its behavior is emergent; the task is the verifying test).
- Polish (T018-T021) last; T021 is the final gate.

## Parallel opportunities

- T002 and T004 (independent test files/sections) can be written together.
- Within US1, T010 (beacon tests) is independent of T006 (pixelbus tests).
- T018 and T019 (docs) are independent and parallelizable.

## Implementation strategy

MVP is US1: the single source of truth with zero behavior change at the default,
which is the foundation the issue asks for and de-risks issues #2 and #3. US2
adds the user-facing setting and safe re-deploy; US3 hardens the config path.
