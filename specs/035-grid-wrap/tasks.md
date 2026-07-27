# Tasks: Pixel Bus Grid Wrap

**Feature**: 035-grid-wrap | **Date**: 2026-07-27
**Input**: [spec.md](spec.md), [plan.md](plan.md), [research.md](research.md), [data-model.md](data-model.md), [contracts/grid-layout.md](contracts/grid-layout.md), [quickstart.md](quickstart.md)

Test-first per constitution principle III. Everything here is desk-testable
except the addon Lua, which has no test harness and is covered instead by the
cross-language constant assertion and by the no-change property.

## Phase 1: Setup

- [x] T001 Confirm the baseline is green: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all --locked`, all in the foreground and watched to completion.

## Phase 2: Foundational

Blocking prerequisite: the arithmetic every other phase is about.

- [x] T002 Write the failing grid-arithmetic tests in `tests/pixelbus.rs`: `grid_position` maps index to `(index % columns, index / columns)`; `grid_rows` is the ceiling with no phantom row at an exact multiple and zero for a zero count; `grid_extent` width is `block_px * min(count, columns)` and height is `block_px * rows`. Exercise several column counts, not only the shipped one. [FR-001, FR-011, SC-001]
- [x] T003 [P] Write the failing property tests: over a range of counts and column counts, no two indices below the count share a position, and every index below the count lands inside the extent. Assert explicitly that the converse does NOT hold for a partial final row, so nobody later "fixes" the arithmetic to make it surjective. [FR-005, SC-001]
- [x] T004 Add `COLUMNS`, `grid_position`, `grid_rows`, and `grid_extent` to `src/pixelbus/mod.rs` per [data-model.md](data-model.md). `COLUMNS` is documented as a value stated once per side and asserted equal by the build, with the reason it is not derived. [FR-001, FR-007, FR-011]

**Checkpoint**: the arithmetic exists and its properties hold.

## Phase 3: User Story 3, nothing changes today (P1)

Sequenced first among the stories despite being the least exciting. It is the
safety argument for the whole slice, and writing its assertions before
`block_center` and `capture_dims` are rewritten means the rewrite is done against
a test that already knows the right answer.

### Tests

- [x] T005 [US3] Write the failing no-change test in `tests/pixelbus.rs`: for every supported block size and every index below the current block count, `block_center` equals `(block_px * index + block_px / 2, block_px / 2)`, with that formula spelled out in the test rather than referenced from the source, so the test and the implementation cannot drift together. [FR-012, FR-014, SC-002]
- [x] T006 [P] [US3] Write the failing capture-region no-change test: for every supported block size, `capture_dims` equals `(block_px * NUM_BLOCKS, block_px)`, again spelled out. [FR-013, FR-014, SC-002]
- [x] T007 [P] [US3] Write the failing heartbeat test: block index 0 resolves to grid position `(0, 0)` at every column count tried, so signal-loss detection is layout-independent. [FR-004, SC-006]
- [x] T008 [P] [US3] Write the failing whole-pixel test: every sampled centre is a whole pixel on both axes at every supported block size, extended to indices in rows past the first. [FR-006]

### Implementation

- [x] T009 [US3] Rewrite `block_center` and `capture_dims` in `src/pixelbus/mod.rs` to derive from the grid, keeping both signatures unchanged so no caller moves. [FR-001, FR-003]
- [x] T010 [US3] Extend the existing geometry contract table test in `tests/pixelbus.rs` with the wrapped cases, keeping every existing row intact. The existing rows are the no-change evidence and must not be edited to match new output. [FR-012, FR-013, FR-015]

**Checkpoint**: the companion wraps, and provably does not move anything yet.

## Phase 4: User Story 2, both sides agree (P1)

### Tests

- [x] T011 [P] [US2] Write the failing agreement test in `tests/beacon.rs`: the addon's `COLUMNS` equals `pixelbus::COLUMNS`, alongside the existing `NUM_BLOCKS` assertion. [FR-008, SC-003]
- [x] T012 [P] [US2] Write the failing bounds test in `tests/pixelbus.rs`: the shipped column count is at least `NUM_BLOCKS`, and one row at `MAX_BLOCK_PX` fits the narrowest supported client width. Introduce that width as a named constant in the test with a comment citing the spec assumption it comes from, rather than a bare literal, so the number is traceable to the thing that justifies it. These are the two constraints that governed the choice, pinned so a future change to either input fails rather than silently violating them. [FR-009, FR-010]

### Implementation

- [x] T013 [US2] In `addon/PixelBeacon/PixelBeacon.lua`: add `local COLUMNS = 16`, change `positionBlock` to take a block index and anchor at `(BLOCK_PX * col, BLOCK_PX * row)`, and update the nine call sites in `buildBlocks` to pass indices 0 through 8 rather than pixel offsets. [FR-001, FR-002]
- [x] T014 [US2] Derive the addon root control's dimensions from the grid (`min(NUM_BLOCKS, COLUMNS)` columns by `ceil(NUM_BLOCKS / COLUMNS)` rows) rather than from the strip length. Anchoring, draw layer, and every render function stay untouched. [FR-003]
- [x] T015 [US2] Advance `addon/PixelBeacon/PixelBeacon.txt` to version 9 on both version lines and update the version-pin assertion in `tests/beacon.rs`. The description needs no change: no signal was added. [FR-020]

**Checkpoint**: both sides wrap identically and the build enforces it.

## Phase 5: User Story 1, the beacon stops being bounded by width (P1)

The capability itself. Its tests are the multi-row cases, which T002 and T003
already cover at the arithmetic level; this phase confirms they hold through the
public entry points that callers actually use.

- [x] T016 [US1] Write and satisfy the failing multi-row test in `tests/pixelbus.rs`: driving `grid_extent` at counts well past one row, the width never exceeds one row's width however large the count, and the height grows by exactly one row per row of blocks. [FR-001, FR-003, SC-001]

## Phase 6: User Story 4, a grid that does not fit is reported (P2)

### Tests

- [x] T017 [P] [US4] Write the failing fit tests in `tests/pixelbus_display.rs`: a grid inside the surface fits; one wider, one taller, and one both do not; and equality on either axis fits rather than exceeding. [FR-016, SC-005]
- [x] T018 [P] [US4] Write the failing watch tests: no descriptor reports nothing; a configured descriptor reports nothing; an unchanged outcome reports nothing; two successive descriptor changes that are both exceeding report once, not twice; and an outcome that changes back reports again. [FR-018, FR-019, SC-005]

### Implementation

- [x] T019 [US4] Add `GridFit`, `grid_fit`, and `GridFitWatch` to `src/pixelbus/display.rs` per [data-model.md](data-model.md). Extents only; a measured descriptor only. [FR-016, FR-017, FR-018]
- [x] T020 [US4] In `src/main.rs`, evaluate the fit inside the existing display-change branch and log a returned `Exceeds` at **warn** level, naming the grid extent and the surface. No new state, no new timer, and nothing gated on the result. [FR-017, FR-019]

## Phase 7: Polish and boundaries

- [x] T021 [P] Confirm the boundary: no block, signal, marker, colour, checksum rule, or cadence changed; no input, weave, fishing, or suppression path touched; every pre-existing assertion in `tests/pixelbus.rs` still present and passing. Additionally search the sampler and reader for any remaining assumption that the beacon is one row tall (a hardcoded sample `y`, a buffer indexed as a single strip, a dimension derived from the block count alone). The contract test cannot catch a hardcoded constant that happens to be correct at nine blocks, so this is a read rather than an assertion. [FR-015, FR-022, FR-023, FR-024, SC-004]
- [x] T022 [P] Update the master specification's section 10.3: express block positions as grid coordinates, document the wrap rule and the fixed column count with the reason it is not derived, and note that the grid grows downward only. [FR-021]
- [x] T023 [P] Add the `CHANGELOG.md` unreleased entry: an `Added` line for the grid wrap, plus a dated decision for the fixed shared column count (including that it reverses issue #3's premise and demotes slice 034's descriptor to a fit check). [FR-021]
- [x] T024 Run the full merge gate in the foreground: fmt, clippy at deny-warnings, and `cargo test --all --locked`. Green before commit, per constitution principle IV. [SC-008]

## Dependencies

```text
Phase 1 (T001)
  └─> Phase 2 (T002..T004)             # the arithmetic
        ├─> Phase 3, US3 (T005..T010)  # needs T004
        │     └─> Phase 4, US2 (T011..T015)   # T011 needs T004; T013 needs T009's shape
        │           └─> Phase 5, US1 (T016)
        └─> Phase 6, US4 (T017..T020)  # T019 needs T004 and slice 034's descriptor
              └─> Phase 7 (T021..T024)
```

Phase 6 is independent of Phases 3 through 5 and could be built in parallel with
them; it needs only the arithmetic from Phase 2 and the descriptor slice 034
already delivered.

## Parallel execution

The `[P]` tasks within each phase touch different test functions or different
files. The largest useful batches are T005 through T008 (four independent test
tasks written before the rewrite they constrain) and T021 through T023 (three
documentation and confirmation tasks in three different files).

## Implementation strategy

The MVP is Phases 2 and 3 together: the companion wraps, and a test proves it
changed nothing. That is a shippable increment on its own, because a wrapped
companion reading an unwrapped addon works perfectly at nine blocks, which is
guarantee 6 in [contracts/grid-layout.md](contracts/grid-layout.md).

Phase 4 makes it a real contract by bringing the addon across and putting the
build in charge of agreement. Phase 6 adds the guard.

The one ordering that must not be relaxed: T005 and T006 come before T009. If the
rewrite lands first, the no-change assertions get written against whatever the new
code produces, which proves nothing at all. They are the slice's safety argument
and they only mean something written blind.
