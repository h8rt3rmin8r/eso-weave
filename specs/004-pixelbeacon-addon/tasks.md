# Tasks: PixelBeacon Companion Addon

**Feature**: `specs/004-pixelbeacon-addon` | **Branch**: `004-pixelbeacon-addon`

This slice delivers Lua addon files, not Rust. There is no game-API unit-test
harness, so per constitution Principle III (adapted) correctness is enforced by a
structural validation of the manifest and Lua plus manual in-game checks. Paths
are repository-relative. `[P]` marks tasks on different files.

## Phase 1: Setup

- [x] T001 Create the `addon/PixelBeacon/` directory and the manifest `addon/PixelBeacon/PixelBeacon.txt` with the title, author, a declared `## APIVersion`, an addon `## Version`, the managed marker line `## X-ESO-Weave-Managed: true`, and the single file entry `PixelBeacon.lua` (FR-011, FR-012).

## Phase 2: User Story 1 - Status heartbeat block (P1)

- [x] T002 [US1] In `addon/PixelBeacon/PixelBeacon.lua`, create the top-level window and the B0 status backdrop at position (0, 0), 16 by 16 physical pixels (UI-scale compensated), solid `#FF00FF`, shown whenever the addon renders and hidden during loading screens by the standard control lifecycle (FR-001, FR-002, FR-005).

## Phase 3: User Story 2 - Fishing state block (P1)

- [x] T003 [US2] Add the B1 fishing backdrop at (16, 0) in `PixelBeacon.lua`, with a fishing-state variable selecting `#0080FF` while waiting, `#00FF00` on a bite, and hidden when idle (FR-003).

## Phase 4: User Story 3 - Bite detection (P1)

- [x] T004 [US3] Implement bite detection in `PixelBeacon.lua`: register `EVENT_INVENTORY_SINGLE_SLOT_UPDATE` and treat a minus-one stack change on the equipped bait during an active fishing interaction as a bite; gate interaction-active via `EVENT_CLIENT_INTERACT_RESULT` and the camera-interaction state; clear on a new item gained, `EVENT_CHATTER_END`, or a safety timeout; suppress while any menu is open. Drive the B1 state from this (FR-006 to FR-009).

## Phase 5: User Story 4 - Latency block (P2)

- [x] T005 [US4] Add the B2 latency backdrop at (32, 0) in `PixelBeacon.lua`, updated at 1 Hz via a registered update and only while B0 renders, encoding `GetLatency()` as red = clamp(latency, 0, 1020) / 4, green = `0xA5`, blue = 255 minus red, in the 0 to 1 channel range (FR-004).

## Phase 6: Polish and validation

- [x] T006 [P] Add a short header comment block to `PixelBeacon.txt` and `PixelBeacon.lua` describing the addon, and confirm no settings, UI beyond the blocks, libraries, or saved variables are used (FR-010).
- [x] T007 Update `CHANGELOG.md` `[Unreleased]` with an Added line for the PixelBeacon addon (no Rust changes; no new dependencies).
- [x] T008 Structural validation and Rust suite: confirm the manifest contains the managed marker, a version, and an API version; confirm the Lua defines the three block colors, the latency source, and the bite-detection events; confirm text hygiene (UTF-8 no BOM, LF, no dashes); and confirm `cargo test --all --locked` is still green (no Rust changed).

## Dependencies and order

- Setup (T001) first. The Lua blocks build up across US1 to US4 in one file, so
  T002 to T005 are sequential (same file). Polish (T006 to T008) last; T008 is the
  structural and Rust-suite gate.

## Parallel opportunities

- The header-comment task (T006) is independent of the changelog and validation.

## MVP scope

User Story 1 (the status heartbeat) plus Setup is the minimum viable increment:
the beacon a reader can detect at all. The fishing block, bite detection, and
latency block follow.
