# Tasks: Quickslot Observation Reconstruction

**Input**: Design documents from `/specs/042-quickslot-reconstruction/`

**Tests**: Tests are mandatory and precede implementation changes.

## Phase 1: Protocol Test Baseline

- [x] T001 Add failing B20 constant, geometry, marker-separation, and Lua/Rust agreement tests in `tests/beacon.rs` and `tests/pixelbus.rs`
- [x] T002 Add failing decoder cases for every classification, corrupt state, absent legacy state, partial identity, and cooldown/class independence in `tests/pixelbus.rs`
- [x] T003 Add failing normalized-view cases for unavailable reasons, empty, non-potion kinds, potion availability, cooldown, and inactive game override in `tests/app_view_model.rs`
- [x] T004 Add a failing automation safety case proving S042 states cannot authorize input in `tests/potion.rs`

**Checkpoint**: New contract tests fail for missing S042 behavior while the existing suite remains otherwise stable.

## Phase 2: Reader and Model Foundation

- [x] T005 [US1] Add quickslot classification, reason, kind, and availability enums plus explicit predicates in `src/pixelbus/mod.rs`
- [x] T006 [US1] Add B20 marker/code decoding and compose it with existing cooldown and identity blocks in `src/pixelbus/mod.rs`
- [x] T007 [US3] Expand reader samples, coordinates, capture count, change detection, and loss clearing for B20 in `src/pixelbus/mod.rs`
- [x] T008 [US1] Keep the auto-potion consumer fail-closed for the new observation contract in `src/potion/mod.rs`
- [x] T009 Update affected engine, routing, fishing, and controller fixtures for the discriminated model without changing their behavior

**Checkpoint**: Rust protocol and safety tests pass before publisher changes.

## Phase 3: Publisher Reconstruction

- [x] T010 [US1] Add B20 constants, control, placement, render state, and block-count update in `addon/PixelBeacon/PixelBeacon.lua`
- [x] T011 [US1] Replace the opaque quickslot gate with raw-fact collection and ordered explicit classification in `addon/PixelBeacon/PixelBeacon.lua`
- [x] T012 [US2] Add one-shot and change-only `/pbquickslot` diagnostic receipts with bounded primitive fields in `addon/PixelBeacon/PixelBeacon.lua`
- [x] T013 [US3] Register slot-state, cooldown, and inventory convergence events while retaining the periodic backstop in `addon/PixelBeacon/PixelBeacon.lua`
- [x] T014 Advance the addon manifest version and description in `addon/PixelBeacon/PixelBeacon.txt`

**Checkpoint**: Publisher constants agree with Rust and source-policy tests prove diagnostics are bounded and non-localized.

## Phase 4: Truthful Presentation

- [x] T015 [US1] Replace the two-field quickslot view with classification, availability, and cooldown derivation in `src/app/mod.rs`
- [x] T016 [US1] Update quickslot labels and tooltips in `src/app/strings.rs`
- [x] T017 [US1] Render the new fields and preserve the inactive-game dormant override in `src/app/ui.rs`
- [x] T018 Update view snapshots and tests without modifying the Skills section

**Checkpoint**: Every classification is visibly distinct and no numeric identity remains in the main view.

## Phase 5: Documentation and Validation

- [x] T019 Update README setup/troubleshooting with `/pbquickslot`, explicit states, and field-receipt steps
- [x] T020 Update `CHANGELOG.md` under Unreleased and mark all completed task checkboxes
- [x] T021 Run `cargo fmt --all -- --check`
- [x] T022 Run `cargo clippy --all-targets --all-features -- -D warnings`
- [x] T023 Run `cargo test --all-targets --all-features`
- [x] T024 Run repository policy checks, UTF-8/BOM/mojibake checks, and inspect the complete diff
- [ ] T025 Commit, push, open the official PR, process CI and every Codex review through at most two rounds, then request the operator's real-client and merge review

## Dependencies & Execution Order

1. Phase 1 establishes red tests.
2. Phase 2 supplies the reader contract consumed by later phases.
3. Phase 3 publishes the contract and diagnostic receipt.
4. Phase 4 exposes the normalized state.
5. Phase 5 documents, verifies, and publishes the slice.

Issue #25 remains blocked until this slice has a real-client receipt. Issue #26
owns any broader pixel-row geometry change.
