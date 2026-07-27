# Tasks: PixelBeacon Quickslot-State Blocks

**Input**: Design documents from `/specs/038-quickslot-blocks/`

**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md),
[research.md](research.md), [data-model.md](data-model.md),
[contracts/quickslot-blocks.md](contracts/quickslot-blocks.md)

**Tests**: required. Constitution Principle III (Test-First With Explicit Seams)
makes a failing test before the code non-optional for this project, so every
behavior below lands as a test task before its implementation task.

**Organization**: by user story, in priority order. US3 (the row crossing) is
sequenced first among the stories despite being P2, because its compile-time
assertion fails at the first edit of `NUM_BLOCKS` and nothing else compiles until
it is resolved. This is stated in plan.md's implementation outline and is the
design, not an accident.

## Format

`- [ ] [ID] [P?] [Story?] Description with file path`

`[P]` marks tasks that touch different files with no dependency on an incomplete
task.

---

## Phase 1: Setup

No project initialization is needed. The crate, the addon, the test files, and
every seam this feature uses already exist.

- [ ] T001 Confirm a clean tree and a green baseline by running `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all --locked` in the foreground before any edit

---

## Phase 2: Foundational (blocking prerequisites)

These block every user story. The block count and the geometry assertions must
land together, because raising the count without replacing the assertion leaves
the crate uncompilable.

- [ ] T002 Make `grid_rows` and `grid_position` `const fn` in `src/pixelbus/mod.rs`, so the replacement assertion can call the real function rather than open-code its arithmetic
- [ ] T003 Raise `NUM_BLOCKS` from 16 to 20 in `src/pixelbus/mod.rs` and update its doc comment to name B16 to B19 and state that the grid now occupies two rows
- [ ] T004 Replace the compile-time assertion `NUM_BLOCKS <= COLUMNS` in `tests/pixelbus.rs` with three assertions stating exactly two rows (`grid_rows(NUM_BLOCKS, COLUMNS) == 2`), a full first row (`NUM_BLOCKS > COLUMNS`), and a partial last row (`NUM_BLOCKS < COLUMNS * 2`), each with its own message naming what a future edit broke
- [ ] T005 Introduce `BLOCKS_AT_WRAP` in `tests/pixelbus.rs` and restate the bound in `the_column_count_satisfies_both_bounds_that_governed_its_choice` as `COLUMNS >= BLOCKS_AT_WRAP`, recording in the comment why the bound was always about the wrap-era count rather than the current one
- [ ] T006 Add the four marks (`QUICKSLOT_MARKER` `0x38`, `QUICKSLOT_ID_HI_MARKER` `0xB0`, `QUICKSLOT_ID_MID_MARKER` `0xDD`, `QUICKSLOT_ID_LO_MARKER` `0xF3`) to `src/pixelbus/mod.rs` and register all four in `BLOCK_CENTER_GREENS`, taking it from 17 entries to 21, so the existing separation check proves them

**Checkpoint**: the crate compiles, the separation check passes, and the geometry
assertions state the two-row shape.

---

## Phase 3: User Story 3 - The grid crosses onto a second row (Priority: P2, sequenced first)

**Goal**: twenty blocks occupy two rows, every pre-existing block keeps its exact
position, and the captured region is one full row wide by two rows tall.

**Independent test**: the captured region is exactly one full row wide and two
rows tall, the four new blocks are the first four positions of row 1, every
pre-existing block is where the strip put it, and the overflow report still
detects a grid that does not fit.

### Tests for User Story 3

- [ ] T007 [US3] Update `block_center_and_capture_dims_match_contract_table` in `tests/pixelbus.rs` to twenty centres per case, with the four row-1 entries at `y = block_px + block_px / 2`, the capture dimensions to `(block_px * 16, block_px * 2)`, and the pinned count to 20
- [ ] T008 [P] [US3] Update `grid_position_wraps_column_then_row` in `tests/pixelbus.rs` to assert row 0 for indices below `COLUMNS` and row 1 for indices from `COLUMNS` to `NUM_BLOCKS`, rather than row 0 for every shipped index
- [ ] T009 [P] [US3] Update `every_current_block_sits_exactly_where_the_strip_put_it` in `tests/pixelbus.rs` to assert the strip formula for indices below `COLUMNS` and the wrapped formula for the four above it, and rename it to say what it now asserts
- [ ] T010 [P] [US3] Replace `the_captured_region_is_exactly_what_the_strip_captured` in `tests/pixelbus.rs` with the two-row expectation, one full row wide by two rows tall, at every supported block size
- [ ] T011 [P] [US3] Rewrite `the_capture_region_is_one_row_while_the_count_fits_the_columns` in `tests/pixelbus.rs` as the crossing case: keep the parametric one-row-per-count assertions, and replace the concrete instance with `NUM_BLOCKS > COLUMNS`, `grid_rows(NUM_BLOCKS, COLUMNS) == 2`, and `capture_dims(block_px) == (block_px * COLUMNS, block_px * 2)`
- [ ] T012 [P] [US3] Add a test in `tests/pixelbus.rs` asserting the four new blocks occupy exactly the first four positions of row 1, in index order, at every supported block size
- [ ] T013 [P] [US3] Add a test in `tests/pixelbus_display.rs` asserting the overflow report fires for a client area wide enough but shorter than two block rows, which this feature makes reachable for the first time

### Implementation for User Story 3

- [ ] T014 [US3] Confirm no production geometry change is needed beyond T002 and T003 by running the suite; `block_center`, `grid_extent`, and `capture_dims` already derive the two-row case, and any edit to them here would mean the wrap was never correct

**Checkpoint**: the geometry is asserted at two rows and every pre-existing
signal is proven not to have moved.

---

## Phase 4: User Story 1 - The operator can see what is in the quickslot (Priority: P1)

**Goal**: the addon publishes the four values, the companion decodes them into one
state, announces changes, and shows them.

**Independent test**: drinking the quickslotted potion produces a decreasing
remaining time then ready; swapping the potion changes the identity; a non-potion
quickslot reports unknown; a steady state produces no repeated announcements.

### Tests for User Story 1

- [ ] T015 [P] [US1] Add `decode_quickslot` tests in `tests/pixelbus.rs` covering ready, a duration, saturation at the maximum step count, and the unavailable payload, mirroring the slice 037 cooldown cases against `QUICKSLOT_MARKER`
- [ ] T016 [P] [US1] Add a test in `tests/pixelbus.rs` asserting a decoded identity is assembled most significant byte first from B17, B18, B19
- [ ] T017 [P] [US1] Add a test in `tests/pixelbus.rs` asserting `item_id` is `None` whenever the cooldown decodes as unknown, whatever the identity blocks carry
- [ ] T018 [P] [US1] Add a test in `tests/pixelbus.rs` asserting the invariant `item_id.is_some()` implies `has_potion()`, over every combination of the four blocks decoding or not
- [ ] T019 [P] [US1] Add reader tests in `tests/pixelbus.rs` asserting one `Quickslot` event per change, none for an unchanged sample, and one carrying the whole state rather than four events
- [ ] T020 [P] [US1] Add a test in `tests/weave_engine.rs` asserting the engine behaves identically for every value of the stored quickslot state, so the observable is provably inert
- [ ] T021 [P] [US1] Add view-model tests in `tests/app_view_model.rs` for the ready, counting-down, and unknown readouts, and for the identity rendered as its numeric value with no name lookup
- [ ] T022 [P] [US1] Add a `tests/beacon.rs` assertion that the manifest version and addon version both advance to 12 and that the description names the new signal

### Implementation for User Story 1

- [ ] T023 [US1] Add `QuickslotState` with `cooldown`, `item_id`, `new_unknown()`, and a derived `has_potion()` to `src/pixelbus/mod.rs`, per data-model.md
- [ ] T024 [US1] Add `decode_quickslot` to `src/pixelbus/mod.rs`, reusing `decode_cooldown` for B16 and a marker-plus-checksum byte decode for B17 to B19, returning no identity when the cooldown is unknown or any identity block fails
- [ ] T025 [US1] Add the four `BlockSamples` fields and the four `ReaderConfig` sample points (indices 16 to 19) to `src/pixelbus/mod.rs`
- [ ] T026 [US1] Add `PixelBusEvent::Quickslot(QuickslotState)`, the reader's `quickslot` field, change detection, and the DEBUG log entry to `src/pixelbus/mod.rs`, and read the four points in `sample_and_observe`
- [ ] T027 [US1] Add the quickslot blocks to `addon/PixelBeacon/PixelBeacon.lua`: the four constants, the four block controls positioned at indices 16 to 19, the encoder, and the compute pass reading `GetCurrentQuickslot`, `GetSlotItemLink`, `GetItemLinkItemType`, `GetItemLinkOnUseAbilityInfo` (for `hasAbility` only), and `GetSlotCooldownInfo` with the quickslot hotbar category, per research.md R2
- [ ] T028 [US1] Reduce the identity modulo 2^24 in `addon/PixelBeacon/PixelBeacon.lua` before splitting it into bytes, so no block can ever carry an unencodable value, per FR-003
- [ ] T029 [US1] Register `EVENT_ACTIVE_QUICKSLOT_CHANGED` and `EVENT_ACTION_SLOT_UPDATED` in `addon/PixelBeacon/PixelBeacon.lua`, re-baseline on the existing `EVENT_PLAYER_ACTIVATED` handler, and add the quickslot pass to the existing tick, adding no new timer
- [ ] T029a [US1] Give the quickslot pass in `addon/PixelBeacon/PixelBeacon.lua` a compute-then-render-if-changed shape following `updateCooldowns`, so a steady quickslot redraws nothing (FR-005). Raised by the analyze gate: FR-005 was otherwise folded into T027 with no task of its own.
- [ ] T030 [US1] Advance `Version` and `AddOnVersion` to 12 in `addon/PixelBeacon/PixelBeacon.txt` and extend the description to name the active quickslot's cooldown and item identity
- [ ] T031 [US1] Add `set_quickslot` and `quickslot()` to `src/weave/mod.rs`, following `set_cooldowns`/`cooldowns()`, stored and acted on by nothing
- [ ] T032 [US1] Route `PixelBusEvent::Quickslot` to `weave.set_quickslot` in `src/app/routing.rs` and document it in the routing doc comment alongside the other inert observables
- [ ] T033 [US1] Add `QuickslotView` and its derivation to `src/app/mod.rs`, with the cooldown and the identity as independently degrading halves per FR-012, and surface it on the view model
- [ ] T034 [US1] Add the two status rows and their strings to `src/app/ui.rs` and `src/app/strings.rs`, placed after the movement row, using the existing muted treatment for the unknown case
- [ ] T034a [US1] Re-run `tests/app_ui_sizing.rs` and `tests/app_window_sizing.rs` against the taller status region and update any expectation the two new rows move, deliberately and with the reason recorded. Raised by the analyze gate: slice 037 required this for the width change its new column caused, and two new rows are the height equivalent. The sizing assertions are relative rather than absolute, so the expected outcome is that nothing moves; this task exists so that is verified rather than assumed.

**Checkpoint**: the signal is published, decoded, announced, and displayed, and
nothing acts on it.

---

## Phase 5: User Story 2 - An out-of-date addon never produces a false reading (Priority: P2)

**Goal**: absent or disturbed blocks report unknown, never a value.

**Independent test**: with a grid drawing none of the four blocks, the quickslot
reports unknown for arbitrary colors behind each of the four positions.

### Tests for User Story 2

- [ ] T035 [P] [US2] Add a test in `tests/pixelbus.rs` asserting all four blocks absent yields unknown and announces no quickslot change
- [ ] T036 [P] [US2] Add a cross-position test in `tests/pixelbus.rs` asserting a color valid for any one of the four new blocks decodes as unknown at every other new block's position, over all combinations
- [ ] T037 [P] [US2] Add a sweep test in `tests/pixelbus.rs` asserting no arbitrary color decodes as a quickslot cooldown, following the existing cooldown sweep
- [ ] T038 [P] [US2] Add a test in `tests/pixelbus.rs` asserting the state clears to unknown when a block stops decoding while the beacon is still alive, and does not announce again once already unknown
- [ ] T039 [P] [US2] Add a test in `tests/pixelbus.rs` asserting the state clears to unknown on signal loss
- [ ] T040 [P] [US2] Add a test in `tests/pixelbus.rs` asserting a failed checksum on any one identity block yields no identity while the cooldown still decodes, the partial state FR-012 requires
- [ ] T041 [P] [US2] Extend `addon_and_companion_agree_on_the_pixel_bus_contract` in `tests/beacon.rs` with the four new marks, so a disagreement cannot reach a release

### Implementation for User Story 2

- [ ] T042 [US2] Add the signal-loss clear for the quickslot state to the reader's loss branch in `src/pixelbus/mod.rs`, alongside combat, menu, resources, movement, and cooldowns
- [ ] T043 [US2] Guard the addon against an absent `HOTBAR_CATEGORY_QUICKSLOT_WHEEL` or `ITEMTYPE_POTION` in `addon/PixelBeacon/PixelBeacon.lua` by publishing the unavailable payload and reading nothing, never passing a nil hotbar category, per research.md R3

**Checkpoint**: every way the reading can be wrong reports unknown instead.

---

## Phase 6: User Story 4 - The operator can find out what the overlay covers (Priority: P3)

**Goal**: the grid's footprint is knowable from the application and from the
documentation.

**Independent test**: the reported extent matches the square size in effect and
the block count, and follows a change to the square size.

### Tests for User Story 4

- [ ] T044 [P] [US4] Add a test in `tests/app_settings.rs` asserting the derived footprint caption matches the block count and the drafted square size at every supported size

### Implementation for User Story 4

- [ ] T045 [US4] Add the derived footprint caption beside the block-size setting in `src/app/ui.rs`, computed from the drafted value so it follows what the operator is editing, with its string in `src/app/strings.rs`
- [ ] T046 [US4] Log the grid extent in squares, rows, and physical pixels at DEBUG when the sampling thread starts in `src/main.rs`, beside the existing grid-fit report
- [ ] T047 [US4] Extend the `SET_BLOCK_PX` help text in `src/app/strings.rs` to state the current footprint at the default size and name the setting as the way to reduce it

**Checkpoint**: the doubled overlay is documented rather than discovered.

---

## Phase 7: Polish and cross-cutting

- [ ] T048 [P] Update section 10.3 of `docs/ESO-Weave-Specification-v0.2.0.md` to document B16 to B19, the twenty-block count, and the two-row grid
- [ ] T049 [P] Add the overlay footprint note to `README.md`, stating the size at the default block size and naming the block-size setting as the remedy
- [ ] T050 [P] Add the `[Unreleased]` changelog entry in `CHANGELOG.md`: an Added line for the feature, plus dated Decisions entries for the slot-cooldown source over the item link (D1), the derived `has_potion` shape (D3), the replaced geometry assertion (D4), and the report-do-not-manage stance on the overlay footprint (D5)
- [ ] T051 Update the header comments in `addon/PixelBeacon/PixelBeacon.lua` that state the block count and the single-row grid
- [ ] T052 Run the full merge gate in the foreground and watch it to completion: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked`

---

## Dependencies

- **Phase 1** blocks everything (a green baseline is what makes a later red result attributable).
- **Phase 2** blocks every story. T002 blocks T004. T003 blocks T004 and T005.
- **Phase 3 (US3)** is sequenced before the other stories because T004 fails at T003's edit. It depends only on Phase 2.
- **Phase 4 (US1)** depends on Phase 2. T023 blocks T024; T024 blocks T026; T025 blocks T026; T031 blocks T032; T033 blocks T034.
- **Phase 5 (US2)** depends on Phase 4's decoder and reader existing. T042 depends on T026.
- **Phase 6 (US4)** depends only on Phase 2 (it needs the new block count, nothing else) and can run alongside Phases 4 and 5.
- **Phase 7** depends on everything.

## Parallel opportunities

- T008 through T013 are six independent test edits in two files with no shared state.
- T015 through T022 are eight independent test additions across four files.
- T035 through T041 are seven independent test additions across two files.
- T048, T049, and T050 are three independent documentation files.

Within a phase, the `[P]` tasks may be done in any order; the implementation
tasks that follow them may not start until their tests are written and failing.

## Implementation strategy

The MVP is Phase 2 plus Phase 3: the grid crosses the row boundary correctly and
provably, with no new signal. That is a shippable state in the sense that
matters here, every existing signal is proven unmoved, though it delivers nothing
new to the operator.

Phase 4 is the feature. Phase 5 is what makes it safe to build a consumer on.
Phase 6 is what stops the doubled overlay reading as a defect.

Nothing in any phase acts on a quickslot value. That is enforced by T020, not
merely intended.

## Analyze gate

Run before implementation. Result: 0 CRITICAL, 0 HIGH, 2 MEDIUM, 2 LOW, with
100 percent requirement coverage. Both MEDIUM findings were coverage gaps and are
closed above by T029a (FR-005 had no task of its own) and T034a (the height
equivalent of the window-sizing obligation slice 037 carried for width). The two
LOW findings are the deliberate square/block register split and the untested log
entry, both consistent with every block slice since 031, and neither is actioned.

Two risks were checked rather than assumed and both cleared: neither platform
sampler contains a one-row assumption (`GdiSampler::for_window` already sizes its
capture from `capture_dims`), and the window-sizing assertions are relative rather
than absolute.
