# Tasks: PixelBeacon Skill-Cooldown Blocks

**Feature**: [spec.md](spec.md) | **Plan**: [plan.md](plan.md) | **Date**: 2026-07-27

**Contract**: [contracts/cooldown-blocks.md](contracts/cooldown-blocks.md) |
**Data model**: [data-model.md](data-model.md)

Test tasks are mandatory here, not optional: constitution Principle III requires a
failing test before the code that satisfies it.

**Total**: 31 tasks. US1 14, US2 5, US3 4, with 5 setup/foundational and 3 polish.

## Phase 1: Setup

- [X] T001 Confirm the baseline merge gate is green before any edit by running `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all --locked` in the foreground from the repository root
- [X] T002 Re-read the resource blocks as the reference implementation, specifically `decode_resource`, `ResourceLevel`, `ResourceSet`, and the resources arm of `observe` in `src/pixelbus/mod.rs`, plus `renderResource` in `addon/PixelBeacon/PixelBeacon.lua`

## Phase 2: Foundational (blocks every user story)

- [X] T003 Add the failing cross-language agreement test in `tests/beacon.rs` asserting `parse_lua_constant` finds the six marks (`0x0B`, `0x21`, `0x4E`, `0x92`, `0xC6`, `0xE8`), `COOLDOWN_STEP_MS` = 50, `COOLDOWN_MAX_STEPS` = 254, and `COOLDOWN_UNAVAILABLE` = 255 in the embedded addon source
- [X] T004 Add the constants and widen the count in `src/pixelbus/mod.rs`: the six `COOLDOWN_*_MARKER` values, `COOLDOWN_STEP_MS`, `COOLDOWN_MAX_STEPS`, `COOLDOWN_UNAVAILABLE`, `NUM_BLOCKS` from 10 to 16, and the six new entries in `BLOCK_CENTER_GREENS` (widening its array to 17)
- [X] T005 Mirror the constants in `addon/PixelBeacon/PixelBeacon.lua` as `local NUM_BLOCKS = 16` and the six marks plus the three quantization constants, deriving the slot indices from the game's own action-bar constants rather than hardcoded integers

**Checkpoint**: T003 passes and the registry separation test in `tests/pixelbus.rs` proves all six marks are clear of the eleven incumbents.

## Phase 3: User Story 1 - The operator can see which skills are ready (Priority: P1)

- [X] T006 [P] [US1] Add failing decode tests in `tests/pixelbus.rs`: ready (`red = 0`), a mid-range duration, saturation at `0xFE`, and the `0xFF` unavailable sentinel, for each of the six marks
- [X] T007 [P] [US1] Add the failing geometry test in `tests/pixelbus.rs` asserting the six point helpers resolve to `block_center` at indices 10 through 15, row 0 columns 10 to 15, for the default and a non-default block size
- [X] T008 [US1] Add `SlotCooldown { Unknown, Ready, RemainingMs(u16) }` and `CooldownSet` to `src/pixelbus/mod.rs`, mirroring `ResourceLevel` and `ResourceSet`
- [X] T009 [US1] Implement `decode_cooldown` in `src/pixelbus/mod.rs` taking the block's mark as a parameter, following `decode_resource`: marker, then checksum, then the range rule mapping 0 to ready, a value up to `COOLDOWN_MAX_STEPS` to a duration, and `COOLDOWN_UNAVAILABLE` to unavailable. The range boundary MUST be expressed through the constants rather than as literals: it keeps the bound stated once instead of in the decoder and the addon separately, and it is what keeps `COOLDOWN_MAX_STEPS` read by `src/` so the private-const `dead_code` lint does not fail the `-D warnings` gate, which is the failure the analyze gate caught in slice 036
- [X] T010 [US1] Add the six point helpers to `ReaderConfig` in `src/pixelbus/mod.rs` beside the ten existing ones
- [X] T011 [US1] Add the six `Option<Rgb>` fields to `BlockSamples` and populate them from the new point helpers in the capture path in `src/pixelbus/mod.rs`
- [X] T012 [US1] Add the failing observe test in `tests/pixelbus.rs`: a change in any slot emits exactly one `PixelBusEvent::Cooldowns` carrying all six, and an unchanged set emits none
- [X] T013 [US1] Add `PixelBusEvent::Cooldowns(CooldownSet)` and the `cooldowns: CooldownSet` reader field to `src/pixelbus/mod.rs`, wiring the observe arm to emit only on change, following the resources arm
- [X] T014 [US1] Add the DEBUG log line on a cooldown change in the observe arm in `src/pixelbus/mod.rs`, emitting one entry per changed sample that names only the slots whose value changed
- [X] T015 [US1] Add `renderCooldowns()` to `addon/PixelBeacon/PixelBeacon.lua` reading `GetSlotCooldownInfo` per slot with change detection, rendered whenever the status block renders, re-baselined on `EVENT_PLAYER_ACTIVATED`, and re-synced on the existing tick
- [X] T016 [US1] Add the inert `set_cooldowns` and `cooldowns` accessors to `src/weave/mod.rs`, following `set_resources`, with the doc comment recording that nothing reads them
- [X] T017 [P] [US1] Add `CooldownView` and its derivation to `src/app/mod.rs`, rendering ready, a duration, and the muted unknown placeholder, and add the per-slot field to the skills view rows
- [X] T018 [US1] Route `PixelBusEvent::Cooldowns` to the weave engine in `src/app/routing.rs`, following the resources routing
- [X] T019 [US1] Add the cooldown column to `strings::SKILL_COLUMNS` and render the cell on each skill row in `src/app/ui.rs`, leaving the Synergy row's cell muted because it has no block

**Checkpoint**: US1 is independently testable end to end.

## Phase 4: User Story 2 - An out-of-date addon never produces a false reading (Priority: P2)

- [X] T020 [P] [US2] Add the failing cross-slot test in `tests/pixelbus.rs`: a colour valid for one slot decodes to `Unknown` at all five other slots' positions, for every ordered pair
- [X] T021 [P] [US2] Add the failing wrong-marker and failed-checksum tests in `tests/pixelbus.rs` for each of the six marks
- [X] T022 [P] [US2] Add the failing exhaustive sweep in `tests/pixelbus.rs` proving no arbitrary colour decodes as a cooldown at any of the six positions
- [X] T023 [US2] Add the failing clear-on-non-decode and signal-loss tests in `tests/pixelbus.rs`: a block that stops decoding clears that slot, and a lost signal clears all six, with no repeat once already unknown
- [X] T024 [US2] Wire the signal-loss clearing into the observe path in `src/pixelbus/mod.rs`, in the same branch that already clears combat, menu, resources, and movement

## Phase 5: User Story 3 - The grid lands exactly on its single-row maximum (Priority: P2)

- [X] T025 [US3] Update the capture-extent test in `tests/pixelbus.rs` to assert the shipping constants produce exactly one full row wide and one row tall, keeping the general property that any count at or below `COLUMNS` is one row
- [X] T026 [US3] Confirm the compile-time assertion `NUM_BLOCKS <= COLUMNS` in `tests/pixelbus.rs` still holds at sixteen and leave it unmodified, adding a comment recording that it now sits at its limit and guards the next block
- [X] T027 [US3] Extend the contract table test `block_center_and_capture_dims_match_contract_table` in `tests/pixelbus.rs` with the six new centres per block size and the new capture dimensions
- [X] T028 [US3] Extend `tests/app_ui_sizing.rs` for the wider skills region and update the pinned column count in `tests/app_strings.rs`

## Phase 6: Polish and cross-cutting concerns

- [X] T029 Advance the manifest in `addon/PixelBeacon/PixelBeacon.txt` from 10 to 11 on both version lines, extend the description to name the new signal, and rename and retarget `embedded_manifest_version_is_ten` in `tests/beacon.rs`
- [X] T030 Update section 10.3 of `docs/ESO-Weave-Specification-v0.2.0.md` with B10 to B15, and add the `[Unreleased]` entry to `CHANGELOG.md` with an `Added` line plus dated decisions for the six-block scope, the marker set and its 11-unit minimum separation, and leaving the boundary assertion in force

- [X] T031 Author `docs/plans/plan-012.md` sequencing slices 037, 038, and 039 against issues #18, #19, and #20, add its row to the index in `docs/plans/README.md`, and point the managed plan reference in `CLAUDE.md` at this feature. Ships in the same commit, matching how build plan 011 shipped with slice 035 (FR-022).

**Final gate**: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all --locked`, foreground, watched to completion.

## Dependencies

```text
Phase 1 (T001-T002)
   |
Phase 2 (T003-T005)   <-- the contract must exist first
   |
   +--> Phase 3 US1 (T006-T019)   <-- MVP
   |        |
   |        +--> Phase 4 US2 (T020-T024)   <-- needs the decoder from T009
   |
   +--> Phase 5 US3 (T025-T028)   <-- needs NUM_BLOCKS from T004 only
   |
Phase 6 (T029-T030)
```

Phase 5 depends only on the block count, so it can run alongside Phase 3.

## Parallel execution opportunities

- **Phase 3**: T006 and T007 are independent tests; T017 touches `src/app/mod.rs`
  while T008 through T015 are in `src/pixelbus/mod.rs` and the addon.
- **Phase 4**: T020, T021, and T022 are independent decode tests.
- Tasks touching the same file are deliberately not marked `[P]`: T008 through
  T014 all edit `src/pixelbus/mod.rs` and must be sequential.

## Implementation strategy

**MVP**: Phases 1 through 3. The signal exists on both sides, decodes, and is
shown to the operator.

**Increment 2**: Phase 4, which hardens the failure modes. Not optional polish:
without it a build can report a cooldown from an addon that never drew one, and
the six-block cross-slot risk is new to this slice.

**Increment 3**: Phases 5 and 6, which pin the grid boundary and update the
architecture of record.
