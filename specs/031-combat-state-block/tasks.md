# Tasks: PixelBeacon In-Combat State Block

**Feature**: 031-combat-state-block | **Date**: 2026-07-27
**Input**: [spec.md](spec.md), [plan.md](plan.md), [research.md](research.md), [data-model.md](data-model.md), [contracts/pixel-bus-b4.md](contracts/pixel-bus-b4.md), [quickstart.md](quickstart.md)

Test-first throughout, per constitution principle III: the failing test is written
before the code that satisfies it. Where a task is a pure refactor with no new
behavior, the existing suite is the test and the migration lands in one step so
the tree never sits uncompilable.

## Phase 1: Setup

- [x] T001 Establish the baseline: run `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all --locked` in the foreground and confirm all three are green before any change, so a later failure is unambiguously this feature's.

## Phase 2: Foundational

Blocking prerequisites. Every user story depends on these. The two migration
tasks each keep the tree compiling by moving definition and call sites together.

- [x] T002 Introduce `BlockSamples` (one `Option<Rgb>` field per block, derived `Default`) in `src/pixelbus/mod.rs`, change `PixelBusReader::observe` to take it instead of four positional arguments, update `sample_and_observe`, update the caller in `src/main.rs`, and migrate every existing construction in `tests/pixelbus.rs` to struct-update syntax. One commit-sized step; the suite must be green at the end of it with no behavior change. [FR-014]
- [x] T003 Raise `NUM_BLOCKS` to 5 in `src/pixelbus/mod.rs`, delete its "adding blocks is a separate feature" doc comment, and add `ReaderConfig::combat_point()` deriving from `block_center(self.block_px, 4)`. Confirm `capture_dims` widens through `NUM_BLOCKS` with no separate edit, and confirm `src/pixelbus/windows.rs` needs no change because it already calls `capture_dims`. In the same step update the two existing tests this necessarily breaks in `tests/pixelbus.rs`: the `assert_eq!(NUM_BLOCKS, 4)` pin, and `block_center_and_capture_dims_match_contract_table`, whose five cases enumerate four block centers and four-block capture widths. Extend each case to a fifth center and the widened capture dims, keeping the table's link to the slice 028 geometry contract intact. [FR-012, FR-013]
- [x] T004 [P] Write the failing test for `parse_lua_constant` in `tests/beacon.rs`: decimal form, `0x` hex form, surrounding whitespace, a name that is absent, and a value that is not a number. [FR-013]
- [x] T005 [P] Implement `parse_lua_constant(source, name) -> Option<u32>` in `src/beacon/mod.rs`, mirroring the existing `parse_manifest_version` in shape and doc style. [FR-013]

## Phase 3: User Story 1, the operator can see combat state (P1)

**Goal**: The player's combat state reaches the operator's screen.

**Independent test**: With the game running and the current addon installed, enter
and leave combat and confirm the interface follows on both transitions, then take
a loading screen in a known state and confirm the interface agrees afterwards.

### Tests

- [x] T006 [P] [US1] Write failing `decode_combat` tests in `tests/pixelbus.rs` for the two valid colors: `(0xE0, 0x2D, 0x1F)` decodes to in combat and `(0x20, 0x2D, 0xDF)` decodes to out of combat, each also at the edge of the default tolerance. [FR-002, FR-007]
- [x] T007 [P] [US1] Write failing reader tests in `tests/pixelbus.rs`: a decoded change emits exactly one `Combat` event, and repeated identical samples emit none. [FR-005, FR-008, SC-002]
- [x] T008 [P] [US1] Write failing `combat_view` tests in `tests/app_view_model.rs` for all three states: in combat and out of combat are detected with the active role, unavailable is "Not detected" with the muted role. [FR-010]
- [x] T009 [P] [US1] Write the failing FR-016 boundary test in `tests/weave_engine.rs`: the engine produces identical behavior with combat state set to in combat, out of combat, and unavailable. This is what defends the boundary when a later slice is tempted to consume the value. [FR-016]

### Implementation

- [x] T010 [US1] Add `CombatSignal` (`Unknown`, `OutOfCombat`, `InCombat`) and the shared constants `COMBAT_MARKER = 0x2D`, `COMBAT_IN_RED = 0xE0`, `COMBAT_OUT_RED = 0x20` to `src/pixelbus/mod.rs`, with a doc comment recording that `0xD2` is reserved as the next block's marker. [FR-002]
- [x] T011 [US1] Implement `decode_combat(sample, tolerance) -> CombatSignal` in `src/pixelbus/mod.rs`: validate the green marker, validate the `red + blue == 255` complement checksum the way `decode_latency` does, then match the red state code. Any failure returns `Unknown`. [FR-006, FR-007]
- [x] T012 [US1] Add the `combat: CombatSignal` field to `PixelBusReader`, the `PixelBusEvent::Combat(CombatSignal)` variant, and the decode-and-emit-on-change wiring in `observe`, plus the DEBUG log line on change matching the existing weapon-bar entry. Sample the combat point in `sample_and_observe`. [FR-008, FR-009]
- [x] T013 [US1] Add `set_combat` and `combat` to the weave engine in `src/weave/mod.rs`, stored beside latency and weapon-bar state and read by nothing in any decision path. [Decision 5]
- [x] T014 [US1] Route `PixelBusEvent::Combat` to `weave.set_combat` in `src/app/routing.rs`, updating the function's doc comment list, and confirm `map_event` in `src/fishing/detector.rs` returns `None` for it so it never reaches the fishing controller. [FR-016]
- [x] T015 [US1] Add `CombatView` and `combat_view` to `src/app/mod.rs` mirroring `WeaponBarView` and `weapon_bar_view`, add the `combat` field to `AppView`, and populate it in `AppModel::view` from the weave engine. [FR-010]
- [x] T016 [US1] Add `COMBAT_TITLE` and `COMBAT_TOOLTIP` to `src/app/strings.rs` and render the combat row directly after the weapon-bar row in `src/app/ui.rs`, using `widgets::label_strong` and `status_color` exactly as that row does. [FR-010]
- [x] T017 [US1] In `addon/PixelBeacon/PixelBeacon.lua`: add `local NUM_BLOCKS = 5` and the three shared color constants, add the combat block to `buildBlocks` positioned at `BLOCK_PX * 4`, change the root width to `BLOCK_PX * NUM_BLOCKS`, and add a change-detected `renderCombat()` that hides only when the status block hides. [FR-001, FR-005, FR-013]
- [x] T018 [US1] Wire the addon events in `addon/PixelBeacon/PixelBeacon.lua`: `EVENT_PLAYER_COMBAT_STATE` drives the state, and the existing `EVENT_PLAYER_ACTIVATED` handler re-baselines from `IsUnitInCombat("player")` alongside the weapon-bar re-baseline. [FR-003, FR-004]
- [x] T019 [US1] Advance `addon/PixelBeacon/PixelBeacon.txt` from version 5 to 6 on both `## Version:` and `## AddOnVersion:`, and extend the `## Description:` line to name the combat signal. The companion single-sources the version from this manifest, so no other pin changes. [FR-015]

**Checkpoint**: combat state decodes, routes, displays, and the addon publishes it.

## Phase 4: User Story 2, an out-of-date addon never produces a false reading (P2)

**Goal**: A missing or unreadable block is always unavailable, never a state.

**Independent test**: Point the reader at a strip with no fifth block and confirm
the reported state is unavailable for arbitrary colors behind that position.
Runs at the desk with no game.

### Tests

- [x] T020 [P] [US2] Write failing rejection tests in `tests/pixelbus.rs`: a wrong green marker, a valid marker with a failed checksum, and a valid marker and checksum with an unrecognized red code each decode to `Unknown`. [FR-007, FR-011]
- [x] T021 [P] [US2] Write the failing arbitrary-color test in `tests/pixelbus.rs`: sweep a broad set of colors at the B4 point (including the four existing block colors, black, white, and a spread of the color cube) and assert none decodes to a combat state. This is the User Story 2 guarantee. [FR-011, SC-004]
- [x] T022 [P] [US2] Write the failing clearing tests in `tests/pixelbus.rs`: after a decoded state, the reader clears to `Unknown` on signal loss, and also clears on a sample where the heartbeat is present but the combat block does not decode. The second case is the deliberate divergence from the weapon-bar block, which holds; assert the weapon block still holds in the same scenario so the divergence stays intentional rather than accidental. [FR-008]

### Implementation

- [x] T023 [US2] Make the clearing behavior match: in `src/pixelbus/mod.rs`, the decoded `Unknown` from a non-decoding block must flow through the same change-detection path as a real state (emitting `Combat(Unknown)` once), and signal loss must reset the stored state to `Unknown`. Add a comment at the weapon-bar handling noting that it holds by design and combat does not, so the next reader of this code sees the divergence is chosen. [FR-008]

**Checkpoint**: no color anywhere can be mistaken for a combat state.

## Phase 5: User Story 3, the next block costs one entry (P3)

**Goal**: Slices 032 to 035 extend the strip without repeating this slice's work.

**Independent test**: Change the block count on one side and confirm every
geometry on that side follows; make the two sides disagree and confirm the suite
reports it.

### Tests

- [x] T024 [P] [US3] Write the failing cross-side agreement test in `tests/beacon.rs`: parse `beacon::LUA` and assert the addon's `NUM_BLOCKS`, `COMBAT_MARKER`, `COMBAT_IN_RED`, and `COMBAT_OUT_RED` each equal the companion constant of the same name. [FR-013, SC-006]
- [x] T025 [P] [US3] Write the failing registry test in `tests/pixelbus.rs`: every entry in the block-center green registry is pairwise separated from every other by more than the default tolerance, and the failure message names the colliding pair. [FR-006]

### Implementation

- [x] T026 [US3] Add the documented `BLOCK_CENTER_GREENS` registry constant to `src/pixelbus/mod.rs`, naming each block and its green, including the two fishing colors, so a later slice picks a marker against a list the suite checks. [FR-006]
- [x] T027 [US3] Re-read the doc comment on `beacon::rewrite_block_px` in `src/beacon/mod.rs` against the post-T017 addon. It already tells the reader the addon derives its strip width from `BLOCK_PX * NUM_BLOCKS`, which was false before this feature and true after it. Edit only if the wording still misleads; if it now reads correctly, record that in the commit rather than changing it. This task may legitimately be a no-op.

**Checkpoint**: the block-extension pattern is enforced, not merely documented.

## Phase 6: Polish and Cross-Cutting

- [x] T028 [P] Update section 10.3 of `docs/ESO-Weave-Specification-v0.2.0.md` to document B4: its position, encoding, marker, state codes, checksum, the clear-on-non-decode rule, and the block count moving to 5. [FR-018]
- [x] T029 [P] Update `CHANGELOG.md`: an `Added` entry under `[Unreleased]` for the combat state block, plus dated decision entries for the `0x2D` marker value and the reserved `0xD2`, the `observe` signature change to `BlockSamples`, and the clear-on-non-decode divergence from the weapon-bar block. [FR-018]
- [x] T030 Run the full merge gate again in the foreground: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked`. Never backgrounded. A red result that cannot be fixed within this feature is a halt. [Constitution IV, SC-007]
- [x] T031 Verify the non-regression boundaries that this feature asserts but no functional task would otherwise check. Review `git diff --stat` and confirm: no file implementing or testing a safety-critical surface is modified (injected-input recursion breaking, suppression scoped to the focused game window, no blocking work on the hook thread, marker-gated beacon uninstall, AddOns subtree confinement, fishing degrading to disabled on signal loss); no input path is added anywhere; and `poll_interval` together with the `interval_fishing_ms` and `interval_idle_ms` config handling is byte-identical. A diff touching any of them is a stop-and-justify, not a proceed. [FR-017, FR-019, Constitution II]
- [x] T032 Verify text hygiene across every file this feature touched: UTF-8 without BOM, LF line endings, and no em-dashes or en-dashes anywhere including code comments and the Lua.

## Owed after merge

- [ ] T033 In-game validation per [quickstart.md](quickstart.md), scenarios 1 through 5. Operator-owned; it exercises the real game API and the real capture path, which no automated test reaches. Track it with the other owed field validations rather than treating this feature as fully validated without it.

## Dependencies

```text
Phase 1 (T001)
   |
Phase 2 (T002, T003 sequential; T004 and T005 parallel to them)
   |
Phase 3 US1 (T006 to T009 parallel, then T010 to T019)
   |
   +--> Phase 4 US2 (T020 to T022 parallel, then T023)
   |
   +--> Phase 5 US3 (T024, T025 parallel, then T026, T027)
   |
Phase 6 Polish (T028, T029 parallel; then T030, T031, T032)
```

## Analyze gate record (2026-07-27)

`/speckit-analyze` reported zero CRITICAL findings, so this feature proceeds
without an early halt. Four findings were resolved into the tasks above rather
than deferred:

- FR-017 and FR-019 had no task at all. Both are negative requirements ("must
  not"), and a negative requirement with no verification step is an assertion,
  not a guarantee. T031 now checks them against the diff.
- Two existing tests in `tests/pixelbus.rs` (the `NUM_BLOCKS` pin and the slice
  028 geometry contract table) are guaranteed to fail when T003 lands, and no
  task owned the update. Folded into T003, where the break originates.
- T027 was underspecified and may correctly be a no-op; restated so that
  "unchanged" is an acceptable, recorded outcome.

Final coverage: 19 of 19 functional requirements have at least one task.

US2 and US3 both depend on Phase 3 but not on each other, so they can run in
either order or interleaved.

## Parallel execution

- Phase 2: T004 and T005 touch `beacon`, independent of the `pixelbus` work in T002 and T003.
- Phase 3 tests: T006, T007, T008, and T009 are four different files or four independent test functions.
- Phase 4 tests: T020, T021, and T022 are independent test functions.
- Phase 5: T024 (`tests/beacon.rs`) and T025 (`tests/pixelbus.rs`) are different files.
- Phase 6: T028 and T029 are different files.

## Implementation strategy

The MVP is Phase 1 through Phase 3: the signal exists end to end and the operator
can see it. Phase 4 is what makes it safe to ship, since it closes the false
reading, and it should not be deferred past this feature. Phase 5 is the
investment build plan 010 sequences this slice first to make; deferring it means
the next three slices each pay the cost this one was supposed to remove.

## Completion record (2026-07-27)

T001 to T032 complete; T033 (in-game validation) is owed and operator-owned.

Corrections made during implementation, recorded rather than silently absorbed:

- The master specification's pixel-bus section is **10.3**, not 9.3. Issue #9 and
  build plan 010 both said 9.3, and that was carried into the spec and this plan
  before anyone opened the document; 9.3 is the fishing bait requirement. Every
  artifact now points at 10.3.
- T002 named `src/main.rs` as an `observe` call site. It is not: `main.rs` calls
  `sample_and_observe`, which absorbed the change internally. No edit was needed.
- The FR-016 boundary test went to `tests/weave_engine.rs`; there is no
  `tests/weave.rs`.
- An existing test, `embedded_manifest_version_is_five`, pinned the addon version
  and had to advance to six, as it did in slices 016, 025, and 026. It was renamed
  rather than duplicated by the new cross-side test.
- T027 was a genuine no-op, which is the outcome the analyze gate restated it to
  allow. The `rewrite_block_px` doc comment already described the addon as
  deriving its strip width from `BLOCK_PX * NUM_BLOCKS`; that was false when it
  was written and is true now, so it was left alone.
