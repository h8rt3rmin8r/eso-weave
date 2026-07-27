# Implementation Plan: PixelBeacon Quickslot-State Blocks

**Branch**: `main` (trunk-based) | **Date**: 2026-07-27 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/038-quickslot-blocks/spec.md`

## Summary

Add four beacon blocks, B16 through B19, publishing the active quickslot's
remaining cooldown and the 24-bit identity of the item in it. The addon reads the
current quickslot, checks the slotted item is a potion with an on-use ability,
and renders the cooldown with the same quantization the skill cooldown blocks
use plus three identity bytes; the companion decodes all four into one value,
routes it to the view model as two status rows, and logs changes at DEBUG.
Nothing acts on the values.

The encoding adds no new pattern: it is the slice 037 cooldown block for B16 and
a plain marker-plus-checksum byte for B17 to B19.

**What is new is the geometry.** This is the first shipping block count to cross
the column boundary. The count goes from 16 to 20, four blocks land on row 1, and
the captured region becomes two rows tall. Seven expectations written when one
row was the only shipping possibility are updated to state the two-row shape,
including the compile-time assertion the previous slice deliberately left in
place to fail at this slice's first edit.

## Technical Context

**Language/Version**: Rust (pinned by `rust-toolchain.toml`) and Lua 5.1 for the
embedded addon.

**Primary Dependencies**: none new.

**Storage**: none. Quickslot state is runtime state and is never written to
config.

**Testing**: `cargo test --all --locked`, with new coverage in `tests/pixelbus.rs`
(decode, observe, the two-row geometry), `tests/beacon.rs` (cross-language
constants, manifest), `tests/app_view_model.rs` (the two readouts and their
independent degradation), and `tests/weave_engine.rs` (the new signal is inert).

**Target Platform**: Windows 10/11 x64 and Linux x64. No platform-specific code.

**Project Type**: desktop application, single Rust crate, addon embedded in the
binary.

**Performance Goals**: unchanged. FR-027 forbids a cadence change. This adds four
sampled points to the existing capture and makes that capture one block row
taller: at the default square size the blit grows from 256x16 to 256x32 physical
pixels, which is 4096 additional pixels at up to 10 Hz.

**Constraints**: the addon-to-companion contract is identical byte for byte and
proven so automatically; text stays UTF-8 without BOM, LF, free of em-dashes and
en-dashes.

**Scale/Scope**: four new blocks taking the grid from 16 to 20 and from one row to
two, one new decoder, one new state type, one new event variant, two new status
rows, one derived settings caption, seven updated geometry expectations.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

| Principle | Status | Evidence |
| --- | --- | --- |
| I. Spec-Driven Development | PASS | Traces to master specification section 10.3 and to `docs/plans/plan-012.md` slice 038. Full sequence run; `analyze` runs before `implement`. |
| II. Safety-Critical Surfaces | PASS | Adds no input path and gates nothing (FR-024, FR-025). No safety test touched. The signal-loss branch that already clears combat, menu, resources, movement, and cooldowns gains one more clear. |
| III. Test-First With Explicit Seams | PASS | Every behavior lands as a failing test first. Existing seams reused unchanged: `BlockSamples` for the reader, `MockSampler` for geometry, `parse_lua_constant` for cross-language agreement. |
| IV. CI Parity Before Every Commit | PASS | `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked`, foreground, watched to completion. |
| V. Bounded Scope: Outside The Game | PASS | Reads no process memory, no network traffic. The addon uses only published unprotected calls (R1) and communicates solely through the screen-signal contract. |

**Post-design re-check**: PASS, unchanged. No new module, dependency, thread, or
input path. Two functions become `const fn` (R7); nothing else about them
changes.

**Complexity Tracking**: not applicable.

## Design Decisions

Recorded under the autopilot decision policy. The first is a deliberate
deviation from the tracker issue and is surfaced at the pre-push halt.

### D1: The cooldown comes from the slot, not from the item link

`GetSlotCooldownInfo(GetCurrentQuickslot(), HOTBAR_CATEGORY_QUICKSLOT_WHEEL)`
rather than the `remainingCooldown` return of `GetItemLinkOnUseAbilityInfo` that
issue #19 proposed. Full reasoning in [research.md](research.md) R2. In short: the
slot is the authority on whether the thing can be used right now (potions share a
cooldown), it is the same call the skill cooldown blocks already make so the
quantization contract is shared rather than parallel, and the signature turned
out to take an explicit hotbar category, which is the doubt that led the issue to
the item link in the first place. `GetItemLinkOnUseAbilityInfo` is still used,
for its `hasAbility` return only.

### D2: A missing game constant publishes unavailable rather than guessing

If `HOTBAR_CATEGORY_QUICKSLOT_WHEEL` or `ITEMTYPE_POTION` is absent, the addon
publishes the unavailable payload and reads nothing. Passing a nil hotbar
category would let the game resolve some other hotbar and the reader would
receive a valid, checksum-passing colour about a slot nobody asked about: a false
reading with full integrity checks behind it. See R3.

### D3: `has_potion` is derived, not stored

`QuickslotState` carries a cooldown and an optional identity, and answers
"is there a potion" as `cooldown != Unknown`. The issue's three-field shape is
equivalent but representable in states that cannot exist. See
[data-model.md](data-model.md).

### D4: The replaced assertion states the shape, not a bound

`NUM_BLOCKS <= COLUMNS` is replaced by three compile-time assertions saying
exactly two rows, first row full, last row partial, rather than by a looser
bound. `grid_rows` becomes a `const fn` so the assertion calls the real function
instead of open-coding its arithmetic. Separately,
`the_column_count_satisfies_both_bounds_that_governed_its_choice` had a bound
whose stated justification was always about the block count *at the wrap*, not
the current one; it is restated against a named `BLOCKS_AT_WRAP` constant. See
R7.

### D5: The overlay footprint is reported, not managed

Two places, one derived caption beside the square-size setting and one debug log
line at sampler start. The anchor is not moved and the square size is not
auto-adjusted; both would change shared geometry on one side of a byte-for-byte
contract. See R8.

## Phase 0: Research

Complete. See [research.md](research.md): R1 API availability, R2 the cooldown
source, R3 missing-constant degradation, R4 identity over restore types, R5 the
four marks, R6 width and byte order, R7 the row crossing inventory, R8 the
overlay footprint, R9 update cadence.

## Phase 1: Design

Complete. See [data-model.md](data-model.md),
[contracts/quickslot-blocks.md](contracts/quickslot-blocks.md), and
[quickstart.md](quickstart.md).

## Implementation Outline

Test-first throughout; each step's tests are written and seen to fail first.

1. **Geometry first, because it fails loudly.** Raise `NUM_BLOCKS` to 20, make
   `grid_rows` and `grid_position` `const fn`, replace the compile-time
   assertion with the three-part two-row form, and update the six other
   single-row expectations listed in R7. The build is red until this is done,
   which is the design.
2. **Contract constants.** Four marks into `BLOCK_CENTER_GREENS`; the separation
   check proves them.
3. **Decoder.** `QuickslotState`, `decode_quickslot`, and the partial-identity
   and cross-position rejection tests.
4. **Reader.** Four `BlockSamples` fields, four `ReaderConfig` points, the
   `Quickslot` event, change detection, clear-on-non-decode, clear-on-signal-loss.
5. **Addon.** Four blocks, the encoder, the event registrations, the tick
   backstop, the manifest bump to version 12.
6. **Cross-language check.** Extend `tests/beacon.rs` with the four marks.
7. **Application.** `WeaveEngine::set_quickslot`, routing, `QuickslotView`, two
   status rows, and the inertness test.
8. **Footprint.** The settings caption and the startup log line.
9. **Documentation.** Master specification section 10.3, the README overlay note,
   and the changelog.

## Risks

- **A single-row expectation is missed and passes by coincidence.** Mitigated by
  R7 being a search result rather than a memory, and by the compile-time
  assertion failing at the first edit rather than at the end.
- **The quickslot hotbar category resolves to something else.** Mitigated by D2:
  absent means unavailable, never a guess.
- **The taller capture costs more than expected.** The blit doubles in height at
  a fixed 10 Hz ceiling. Accepted and measured in Technical Context; the region is
  still 4 percent of a 1024-wide client's top strip.
