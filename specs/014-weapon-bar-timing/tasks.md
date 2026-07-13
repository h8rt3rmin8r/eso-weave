---

description: "Task list for slice 014: weapon-bar-aware adaptive timing"
---

# Tasks: Weapon-Bar-Aware Adaptive Timing

**Input**: Design documents from `specs/014-weapon-bar-timing/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, quickstart.md

**Tests**: Included (constitution test-first discipline; safety surfaces never
weakened). The addon Lua and the live pixel signal are validated in-game, not by
unit tests; everything else is unit-tested.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: US1..US4
- Exact file paths in each task

---

## Phase 1: Setup (Shared Types)

- [ ] T001 [P] Add `ActiveBar`, `WeaponClass` (fixed codes 0..6), and `WeaponBarSignal` with code-to-enum mapping in `src/pixelbus/mod.rs`

---

## Phase 2: Foundational (Decode and Relay)

**CRITICAL**: blocks the app-side stories.

### Tests

- [ ] T002 [P] Add `tests/pixelbus.rs` cases for `decode_weapon_bar`: the `0x5A` marker gates decoding; every weapon class and active bar decodes from red/blue; the tolerance boundary matches at `tol` and not at `tol + 1`
- [ ] T003 [P] Add `tests/pixelbus.rs` cases: `observe` emits a `WeaponBar` event only on a change and only with a heartbeat present, and the existing status, fishing, and latency decoding is unchanged when a fourth sample is added

### Implementation

- [ ] T004 [US1] In `src/pixelbus/mod.rs` add `decode_weapon_bar(sample, tolerance) -> Option<WeaponBarSignal>`, the `PixelBusEvent::WeaponBar` variant, a `weapon_point` (default `(56, 8)`) on `ReaderConfig`, and extend `observe`/`sample_and_observe` to read the fourth block and edge-detect the weapon-bar signal
- [ ] T005 In `addon/PixelBeacon/PixelBeacon.lua` add the B3 weapon-bar block (active pair from `GetActiveWeaponPairInfo`, per-bar class from `GetItemWeaponType` on the four equip slots, mapped to the shared class codes), widen the root to `BLOCK_PX * 4`, register `EVENT_ACTIVE_WEAPON_PAIR_CHANGED`, `EVENT_PLAYER_ACTIVATED`, death and revive, and equip/inventory update, and edge-detect so it re-renders only on a real change; hold last good on locked or none
- [ ] T006 Bump `## APIVersion` and the addon version in `addon/PixelBeacon/PixelBeacon.txt`, keeping the `## X-ESO-Weave-Managed: true` marker verbatim
- [ ] T007 Update the pinned contracts: add the B3 Weapon-bar row and encoding to `specs/004-pixelbeacon-addon/contracts/pixel-bus.md`, and the weapon-bar decoder, signal, sample point, and event to `specs/005-pixel-bus-reader/contracts/reader.md`; record a dated Decisions entry in `CHANGELOG.md` for each pinned change

**Checkpoint**: the reader decodes the weapon-bar block; the addon relays it

---

## Phase 3: User Story 1 - Correct timing on the active bar (Priority: P1) MVP

**Goal**: per-bar timing profiles selected by the detected active bar.

**Independent Test**: a reported bar change switches the effective timing profile; an
indeterminate or repeated report holds it.

### Tests

- [ ] T008 [P] [US1] Add `tests/weave_engine.rs` cases: `effective_timing` returns the back profile when the active bar is back and the front profile when front or unknown; a distinct back profile changes the emitted sequence delays after a bar change
- [ ] T009 [P] [US1] Add `tests/app_view_model.rs` case: routing a `WeaponBar` event updates the engine's active bar and weapon classes

### Implementation

- [ ] T010 [US1] In `src/weave/types.rs` / `src/weave/mod.rs` add `timing_back: TimingConfig` and `auto_timing: bool` to `WeaveConfig` (serde-defaulted, back-compatible) and the runtime `active_bar`/`front_class`/`back_class` state on the engine
- [ ] T011 [US1] Add `WeaveEngine::set_weapon_bar(signal)` and `effective_timing(...)`, and change `handle` to compute the effective timing (base profile by active bar) and pass it to `sequence_for_adapted`; extend `store`/`load` to round-trip the back profile and auto flag
- [ ] T012 [US1] In `src/app/routing.rs` route `PixelBusEvent::WeaponBar` to `weave.set_weapon_bar`

**Checkpoint**: timing follows the active bar

---

## Phase 4: User Story 2 - Sensible timing from the equipped weapon (Priority: P1)

**Goal**: weapon-class heavy-attack presets applied per bar when auto timing is on.

**Independent Test**: with auto on, a faster-heavy class bar has a shorter heavy delay
than a slower-heavy class bar; auto off uses manual values that were preserved.

### Tests

- [ ] T013 [P] [US2] Add `tests/weave_engine.rs` cases: `heavy_preset` orders dual wield < two handed < staves and bow; with auto on each bar's effective `d_heavy` equals its class preset; with auto off the manual profile is used and manual values are preserved after toggling auto

### Implementation

- [ ] T014 [US2] Add `heavy_preset(class) -> Option<u32>` in `src/weave/types.rs` (dual wield 640, two handed 1050, sword and shield 900, bow 1380, destruction staff 1180, restoration staff 1360, unknown none) and apply it in `effective_timing` when `auto_timing` is on

**Checkpoint**: auto timing derives delays from the equipped weapon

---

## Phase 5: User Story 3 - See the active bar and weapons (Priority: P2)

**Goal**: display the detected active bar and each bar's weapon class.

### Tests

- [ ] T015 [P] [US3] Add `tests/app_view_model.rs` cases: the weapon-bar view derivation shows the active bar and both weapon classes, and an unknown state when there is no signal

### Implementation

- [ ] T016 [US3] Expose the weapon-bar state in `AppView` and add a tested `WeaponBarView` derivation in `src/app/mod.rs`; add the labels, tooltips, and weapon-class display names to `src/app/strings.rs`
- [ ] T017 [US3] Render a weapon-bar status line in `src/app/ui.rs`, and add the "auto timing from weapon" toggle plus a back-bar timing group (shown when auto is off) to the Combat timing settings cluster, reusing the slice 013 toggle, heading, help, and auto-save

**Checkpoint**: the active bar and weapons are visible and the auto toggle works

---

## Phase 6: User Story 4 - Documented evidence-based defaults (Priority: P3)

- [ ] T018 [US4] Add the R1 timing appendix to `docs/ESO-Weave-Specification-v0.2.0.md` (global-cooldown context, per-weapon-class heavy-attack defaults with sources, and the in-game validation owed) and update section 16 to mark R1 closed

---

## Phase 7: Persistence and Polish

- [ ] T019 [P] Add a `tests/config.rs` case (or weave test): the back timing profile and the auto-timing flag round-trip through settings and default back-compatibly when absent
- [ ] T020 Update `CHANGELOG.md` `[Unreleased]` with an Added line for slice 014 and the dated Decisions entries for the pixel-bus and reader contracts, the addon manifest, and the R1 appendix
- [ ] T021 Run the `quickstart.md` app-level manual validation and record results; note the in-game validation owed (pixel signal, exact presets, one-hand-and-shield)
- [ ] T022 Run the CI-parity gate in the foreground: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked`

---

## Dependencies and Execution Order

- Setup (T001) then Foundational (T002-T007) block the app-side stories.
- US1 (per-bar selection) is the MVP; US2 (presets) builds on the same timing model;
  US3 (visibility) layers on the decoded state; US4 (appendix) is independent docs.
- Tests precede implementation within each story; the safety surfaces (managed marker,
  signal-loss degrade) are never weakened.

## Notes

- The weapon-class codes MUST be identical in `PixelBeacon.lua` and `src/pixelbus`; a
  comment and a test document the shared table.
- The addon Lua and the live pixel signal are validated in-game; the automated gate
  covers the reader, timing model, routing, view-model, and config.
- Commit per story checkpoint; run the CI-parity gate before every Rust-source commit;
  halt once before push.
