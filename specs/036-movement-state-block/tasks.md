# Tasks: PixelBeacon Movement-State Block

**Feature**: [spec.md](spec.md) | **Plan**: [plan.md](plan.md) | **Date**: 2026-07-27

**Contract**: [contracts/movement-block.md](contracts/movement-block.md) |
**Data model**: [data-model.md](data-model.md)

Test tasks are mandatory in this project, not optional: constitution Principle
III requires a failing test before the code that satisfies it. Each test task
below is written to fail before its implementation task and pass after it.

**Total**: 31 tasks. US1 12, US2 6, US3 4, with 5 setup/foundational and 4 polish.

## Phase 1: Setup

- [X] T001 Confirm the baseline merge gate is green before any edit, by running `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all --locked` in the foreground from the repository root
- [X] T002 Re-read the combat block as the reference implementation, specifically `decode_combat`, the `CombatSignal` definition, and the combat arm of `observe` in `src/pixelbus/mod.rs`, so the new code mirrors it rather than paraphrasing it

## Phase 2: Foundational (blocks every user story)

These establish the shared contract. No story can proceed until the block exists
on both sides and the two sides are proven to agree.

- [X] T003 Add the failing cross-language agreement test for the new constants in `tests/beacon.rs`, asserting `parse_lua_constant` finds `MOVEMENT_MARKER` = `0x43`, `MOVEMENT_ON_FOOT_RED` = `0x20`, and `MOVEMENT_MOUNTED_RED` = `0x60` in the embedded addon source, and that `NUM_BLOCKS` parses as 10
- [X] T004 Add the movement constants and widen the block count in `src/pixelbus/mod.rs`: `MOVEMENT_MARKER: u8 = 0x43`, `MOVEMENT_ON_FOOT_RED: u8 = 0x20`, `MOVEMENT_MOUNTED_RED: u8 = 0x60`, the companion-only reserved `MOVEMENT_SPRINT_ON_FOOT_RED: u8 = 0xA0` and `MOVEMENT_SPRINT_MOUNTED_RED: u8 = 0xE0`, `NUM_BLOCKS` from 9 to 10, and the `("B9 movement marker", MOVEMENT_MARKER)` entry in `BLOCK_CENTER_GREENS` (widening its array length to 11)
- [X] T005 Mirror the constants in `addon/PixelBeacon/PixelBeacon.lua`: `local NUM_BLOCKS = 10`, `local MOVEMENT_MARKER = 0x43`, `local MOVEMENT_ON_FOOT_RED = 0x20`, `local MOVEMENT_MOUNTED_RED = 0x60`, deliberately omitting the two reserved sprint codes per research decision R3

**Checkpoint**: T003 passes. The two sides agree on the contract and the registry
separation test in `tests/pixelbus.rs` proves `0x43` is clear of all ten
incumbent greens.

## Phase 3: User Story 1 - The operator can see movement state (Priority: P1)

**Goal**: mounting and dismounting are reflected in the application, survive a
loading screen, and produce no repeated output when nothing changes.

**Independent test**: with the game running and the addon at version 10, mount
and dismount and confirm the readout follows both ways; zone while mounted and
confirm the readout agrees afterwards.

- [X] T006 [P] [US1] Add failing decode tests for the live codes in `tests/pixelbus.rs`: a sample with green `0x43`, red `0x20`, blue `0xDF` decodes to `MovementSignal::OnFoot`, and green `0x43`, red `0x60`, blue `0x9F` decodes to `MovementSignal::Mounted`
- [X] T007 [P] [US1] Add the failing geometry test in `tests/pixelbus.rs` asserting `ReaderConfig::movement_point()` resolves to `block_center(9, block_px)`, at row 0 column 9, for both the default block size and a non-default one
- [X] T008 [US1] Add `MovementSignal { Unknown, OnFoot, Mounted }` to `src/pixelbus/mod.rs` deriving `Debug, Clone, Copy, PartialEq, Eq, Default` with `Unknown` as `#[default]`, mirroring `CombatSignal`
- [X] T009 [US1] Implement `decode_movement` in `src/pixelbus/mod.rs` following `decode_combat` exactly: marker within tolerance, then `red + blue` within tolerance of 255, then the red code selects the state, with every failure yielding `Unknown`. The two reserved sprint codes MUST be matched explicitly and returned as `Unknown` with a comment naming the deferral, rather than falling through the catch-all: it documents the reservation in executable form, and it is what keeps the reserved constants read by `src/` so the private-const `dead_code` lint does not fail the `-D warnings` gate
- [X] T010 [US1] Add `movement_point()` to `ReaderConfig` in `src/pixelbus/mod.rs` beside the nine existing point helpers, deriving from `block_center(9, ..)`
- [X] T011 [US1] Add the `movement: Rgb` field to `BlockSamples` and populate it from `movement_point()` in the capture path in `src/pixelbus/mod.rs`
- [X] T012 [US1] Add the failing observe test in `tests/pixelbus.rs`: a decoded change emits exactly one `PixelBusEvent::Movement`, and repeated identical samples emit none
- [X] T013 [US1] Add `PixelBusEvent::Movement(MovementSignal)` and the `movement: MovementSignal` reader field to `src/pixelbus/mod.rs`, wiring the movement arm of `observe` to emit only on change, following the combat arm byte for byte
- [X] T014 [US1] Add the DEBUG log line on a movement change in the observe arm in `src/pixelbus/mod.rs`, matching the wording and level of the existing "combat state detected" entry
- [X] T015 [US1] Add `renderMovement()` to `addon/PixelBeacon/PixelBeacon.lua` setting the block from `IsMounted()` with change detection, registered on `EVENT_MOUNTED_STATE_CHANGED` and re-baselined from `IsMounted()` on `EVENT_PLAYER_ACTIVATED`, and drawn whenever the status block is drawn
- [X] T016 [P] [US1] Add `MovementView` and `movement_view` to `src/app/mod.rs` mirroring `CombatView` and `combat_view`, rendering "Mounted", "On foot", and "Not detected" with `StatusRole::Active` when detected and `StatusRole::Muted` otherwise
- [X] T017 [US1] Route `PixelBusEvent::Movement` to the view model in `src/app/routing.rs` and add the `movement: MovementView` field to the view struct in `src/app/mod.rs`, following the combat routing
- [X] T018 [US1] Render the movement readout beside the combat field in `src/app/ui.rs`, using the same label and color treatment

**Checkpoint**: US1 is independently testable. The signal decodes, changes emit
one event each, and the operator sees it.

## Phase 4: User Story 2 - An out-of-date addon never produces a false reading (Priority: P2)

**Goal**: every failure path lands on unavailable, never on a state.

**Independent test**: point the reader at a grid with no tenth block and confirm
unavailable for arbitrary colors behind that position.

- [X] T019 [P] [US2] Add the failing wrong-marker test in `tests/pixelbus.rs`: a sample whose green is any of the ten incumbent block-center greens, with an otherwise valid red and blue, decodes to `MovementSignal::Unknown`
- [X] T020 [P] [US2] Add the failing checksum test in `tests/pixelbus.rs`: a sample with the correct marker and a live red but a blue that is not `255 - red` decodes to `Unknown`
- [X] T021 [P] [US2] Add the failing exhaustive absence test in `tests/pixelbus.rs`: for every green value from 0 to 255 that is not within tolerance of `0x43`, decoding yields `Unknown` regardless of red and blue, proving no color behind an absent block can be read as a state
- [X] T022 [US2] Add the failing clear-on-non-decode test in `tests/pixelbus.rs`: after reporting `Mounted`, a sample that fails to decode while the beacon is alive emits `Movement(Unknown)` and clears the stored state
- [X] T023 [US2] Add the failing signal-loss test in `tests/pixelbus.rs`: after reporting `Mounted`, a lost beacon signal emits `Movement(Unknown)`, and a reader already at `Unknown` emits nothing on further loss
- [X] T024 [US2] Wire the signal-loss and non-decode clearing into the observe path in `src/pixelbus/mod.rs`, in the same branch that already clears combat, so T022 and T023 pass

**Checkpoint**: US2 is independently testable. No failure mode produces a state.

## Phase 5: User Story 3 - Adding sprint later costs a code, not a square (Priority: P3)

**Goal**: the deferred axis has a reserved, documented, rejection-tested home.

**Independent test**: confirm the reserved codes decode to unavailable and that
the table documents why they are unused.

- [X] T025 [P] [US3] Add the failing reserved-code test in `tests/pixelbus.rs`: samples with the correct marker and valid checksum but red `0xA0` or `0xE0` decode to `MovementSignal::Unknown`, never to a state
- [X] T026 [US3] Document the reserved codes on the constants in `src/pixelbus/mod.rs` with the two-bit layout, the reason the sprint axis is deferred, and a pointer to the spec's verification section, so the next reader does not re-derive it
- [X] T027 [US3] Review the naming contract across `src/pixelbus/mod.rs`, `src/app/mod.rs`, `src/app/routing.rs`, and `src/app/ui.rs`, confirming every introduced name is a movement name and that "mounted" appears only as a value. This is a review step rather than an automated check, and is surfaced at the authorization halt as such; there is no lint that can distinguish a well-chosen name from a poor one
- [X] T028 [US3] Assert the reserved codes are absent from the addon in `tests/beacon.rs`, confirming `parse_lua_constant` finds no sprint constant, so the companion-only reservation of research decision R3 is enforced rather than assumed

## Phase 6: Polish and cross-cutting concerns

- [X] T029 Advance the manifest in `addon/PixelBeacon/PixelBeacon.txt` from version 9 to 10 on both `## Version` and `## AddOnVersion`, extend the `## Description` to name the mounted signal, and rename and retarget `embedded_manifest_version_is_nine` in `tests/beacon.rs`
- [X] T030 Add the capture-extent invariant test in `tests/pixelbus.rs`, asserting `grid_extent` yields one row for any block count at or below `COLUMNS` and that the shipping constants produce an unchanged region, expressed on the dependency rather than on the literal ten per plan decision D5
- [X] T031 Update the pixel-bus block table in `docs/ESO-Weave-Specification-v0.2.0.md` section 10.3 with B9, and add the `[Unreleased]` entry to `CHANGELOG.md` with an `Added` line plus dated decisions for the reduced mounted-only scope, the `0x43` marker, and the reserved sprint codes

**Final gate**: run `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all --locked` in the foreground, watched to completion, per constitution Principle IV.

## Dependencies

```text
Phase 1 (T001-T002)
   |
Phase 2 (T003-T005)  <-- blocks everything; the contract must exist first
   |
   +--> Phase 3 US1 (T006-T018)  <-- MVP
   |        |
   |        +--> Phase 4 US2 (T019-T024)  <-- needs the decoder from T009
   |                 |
   |                 +--> Phase 5 US3 (T025-T028)  <-- needs the code table live
   |
Phase 6 (T029-T031)  <-- after all stories
```

US2 depends on US1 only for the decoder and the observe arm; its tests are
otherwise independent. US3 depends on the code table existing, which T004 and
T009 provide.

## Parallel execution opportunities

- **Phase 3**: T006 and T007 are independent test files' concerns and can be
  written together; T016 touches `src/app/mod.rs` while T008 through T015 are in
  `src/pixelbus/mod.rs` and the addon, so it can proceed alongside them.
- **Phase 4**: T019, T020, and T021 are three independent decode tests and can be
  written in one pass.
- **Phase 5**: T025 is independent of T026 through T028.

Tasks touching the same file are deliberately not marked `[P]`. T008 through
T014 all edit `src/pixelbus/mod.rs` and must be sequential.

## Implementation strategy

**MVP scope**: Phase 1 through Phase 3. That delivers the whole user-visible
value of the feature: the signal exists on both sides, decodes, and is shown to
the operator.

**Increment 2**: Phase 4. Hardens the failure modes. This is where the feature
becomes safe to ship, because User Story 2's false-reading risk is the one most
likely to occur in the field.

**Increment 3**: Phase 5 and Phase 6. Locks in the deferred axis's reservation
and updates the architecture of record.

Ship order is strict rather than advisory: Phase 4 is not optional polish. A
build with Phase 3 but not Phase 4 can report a movement state from an addon that
never drew one, which is the exact defect the contract exists to prevent.
