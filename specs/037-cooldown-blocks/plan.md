# Implementation Plan: PixelBeacon Skill-Cooldown Blocks

**Branch**: `main` (trunk-based) | **Date**: 2026-07-27 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/037-cooldown-blocks/spec.md`

## Summary

Add six beacon blocks, B10 through B15, publishing the remaining cooldown of the
five skill slots and the ultimate. The addon reads `GetSlotCooldownInfo` per
slot, quantizes the remaining milliseconds, and renders on the existing tick with
change detection; the companion decodes all six into one aggregate value, routes
it to the view model as a new column on the skills grid, and logs changes at
DEBUG. Nothing acts on the values.

The approach is the resource blocks (slice 033) applied to a wider set: the same
numeric-payload-with-sentinel encoding, the same aggregate-set event so a sample
in which several values move is one change rather than six, and the same
marker-plus-checksum validation. This slice adds no new pattern.

It also lands the grid at exactly sixteen blocks, the single-row maximum.

## Technical Context

**Language/Version**: Rust (pinned by `rust-toolchain.toml`) and Lua 5.1 for the
embedded addon.

**Primary Dependencies**: none new.

**Storage**: none. Cooldowns are runtime state and are never written to config.

**Testing**: `cargo test --all --locked`, with new coverage in
`tests/pixelbus.rs` (decode, observe, geometry), `tests/beacon.rs` (cross-language
constants, manifest), `tests/app_strings.rs` (the skill column count), and
`tests/app_ui_sizing.rs` (the wider skills grid).

**Target Platform**: Windows 10/11 x64 and Linux x64. No platform-specific code.

**Project Type**: desktop application, single Rust crate, addon embedded in the
binary.

**Performance Goals**: unchanged. FR-023 forbids a cadence change; this adds six
sampled points per existing capture and no additional capture.

**Constraints**: the addon-to-companion contract is identical byte for byte and
proven so automatically; text stays UTF-8 without BOM, LF, free of em-dashes and
en-dashes.

**Scale/Scope**: six new blocks taking the grid from 10 to 16, one new decoder,
one new aggregate type, one new event variant, one new interface column.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

| Principle | Status | Evidence |
| --- | --- | --- |
| I. Spec-Driven Development | PASS | Traces to master specification section 10.3 and to `docs/plans/plan-012.md`, which this feature authors. Full sequence run; `analyze` runs before `implement`. |
| II. Safety-Critical Surfaces | PASS | Adds no input path and gates nothing (FR-020, FR-021). No safety test touched. The signal-loss branch that already clears combat, menu, resources, and movement gains one more clear. |
| III. Test-First With Explicit Seams | PASS | Every behavior lands as a failing test first. Existing seams reused unchanged: `BlockSamples` for the reader, `parse_lua_constant` for cross-language agreement, `Harness`/`build_ui_state` for window sizing. |
| IV. CI Parity Before Every Commit | PASS | `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked`, foreground, watched to completion. |
| V. Bounded Scope: Outside The Game | PASS | Reads no process memory, no network traffic. The addon uses only published unprotected calls and communicates solely through the screen-signal contract. |

**Post-design re-check**: PASS, unchanged. No new module, dependency, thread, or
input path.

**Complexity Tracking**: not applicable.

## Design Decisions

Full alternatives analysis in [research.md](research.md).

### D1: Six blocks, not seven. Synergy gets none.

The spec's clarification records the finding; this is the consequence. The game's
action bar iterates its slots from the first-normal index through the ultimate
index. Synergy is outside that range because it is not an action slot, so
`GetSlotCooldownInfo` has nothing to return for it in any state.

A seventh permanently-unavailable block would cost a square, a validity mark, a
permanently muted interface field, and would push the grid to seventeen blocks
and onto a second row, all to buy positional symmetry with an interface row. The
interface simply leaves the Synergy row's cooldown cell empty, which is both
honest and free.

The application's slot 6 (Ultimate) maps to the game's ultimate slot; slots 1
through 5 map to the five normal slots. The addon derives the indices from the
game's own named constants rather than hardcoding integers, so a future change to
the bar layout does not silently misalign the blocks.

### D2: The six markers

Eleven greens are in use (`0x00`, `0x16`, `0x2D`, `0x43`, `0x5A`, `0x6D`, `0x80`,
`0xA5`, `0xBB`, `0xD2`, `0xFF`). The six new marks are the midpoints of the six
widest remaining gaps:

| Block | Slot | Mark | Nearest neighbour |
| --- | --- | --- | --- |
| B10 | Skill 1 | `0x0B` | 11 |
| B11 | Skill 2 | `0x21` | 11 |
| B12 | Skill 3 | `0x4E` | 11 |
| B13 | Skill 4 | `0x92` | 18 |
| B14 | Skill 5 | `0xC6` | 11 |
| B15 | Ultimate | `0xE8` | 22 |

The minimum separation across the whole registry becomes 11, which is 5.5 times
the default tolerance of 2. That is tighter than the 22 slice 036 achieved, and
it is the honest price of adding six marks at once: with seventeen values in a
256-wide channel, 11 is the best achievable minimum, and no alternative placement
improves it. It remains a wide margin against the tolerance that actually
governs decoding.

The marks ascend with slot index, which is a mnemonic rather than a contract; the
decoder matches each block against its own mark and never infers one from
another.

### D3: The encoding

Marker in green, quantized remaining time in red, `255 - red` in blue, exactly as
combat, menu, movement, and the resource blocks do.

`red = min(remaining_ms / 50, 254)`, so `0` means ready, `254` means 12.7 seconds
or longer, and `0xFF` means unavailable. The maximum encodable value saturates
rather than wraps, which is what FR-003 requires: a long cooldown reads as "at
least this long" rather than as a small number.

`0xFF` as the unavailable sentinel is deliberately the same choice
`RESOURCE_UNAVAILABLE` already made. It passes the marker and checksum checks and
fails the range check, so it needs no special case in the decoder, and a reader
of either decoder recognizes the other.

### D4: One aggregate value, not six

`CooldownSet` carrying six `SlotCooldown` values, and one
`PixelBusEvent::Cooldowns(CooldownSet)`, following `ResourceSet` and
`PixelBusEvent::Resources` whose own doc comment gives the rationale: a sample in
which several values move at once should be one event rather than several. Six
slots make that argument stronger than three did, because a single weave can move
most of them within one sample.

This also discharges FR-009 structurally rather than by discipline: one event per
changed sample is one log entry per changed sample, so the flooding the
requirement forbids cannot arise from the event shape. The entry names only the
slots whose value changed.

### D5: The interface column, and what it disturbs

The skills region is an `egui::Grid` driven by `strings::SKILL_COLUMNS`. Adding a
cooldown column has two consequences that must be handled rather than discovered:

- `tests/app_strings.rs` asserts `SKILL_COLUMNS.len() == 5`. That is a pinned
  expectation and moves to 6 deliberately.
- The skills region is the widest content-sized block in the window, so it
  determines the intrinsic width that `intrinsic_extent` computes. The window
  sizing tests added by slice 030 (`tests/app_ui_sizing.rs`) exist to catch
  exactly this and are extended, not left to pass by accident.

### D6: The single-row maximum, and leaving the assertion alone

`NUM_BLOCKS` goes 10 to 16 against `COLUMNS = 16`. `grid_rows(16, 16)` is 1 and
`grid_extent` is `(block_px * 16, block_px)`: one full row, one row tall.

The const assertion in `tests/pixelbus.rs` asserting `NUM_BLOCKS <= COLUMNS` is
**left exactly as it is**. It now sits at its limit with no margin, which is
precisely when it becomes valuable: the next block added anywhere in this family
trips it, and that block belongs to slice 038. Relaxing it here to spare the next
slice an inconvenience would discard the warning at the moment it starts earning
its keep. The test asserting the extent is updated from "one row while the count
fits" to the concrete single-row-maximum case, and the boundary test that already
proves `COLUMNS + 1` starts a second row is left in place as the description of
what happens next.

### D7: The manifest advances 10 to 11

Both `## Version` and `## AddOnVersion`, with the description naming the new
signal, so the beacon manager offers the update.
`tests/beacon.rs::embedded_manifest_version_is_ten` is renamed and retargeted.

## Project Structure

### Documentation (this feature)

```text
specs/037-cooldown-blocks/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/
│   └── cooldown-blocks.md
├── checklists/
│   ├── requirements.md
│   └── beacon-contract.md
├── spec.md
└── tasks.md             # Phase 2 output
```

### Source Code (repository root)

```text
docs/plans/plan-012.md   # NEW: sequences slices 037, 038, 039

addon/PixelBeacon/
├── PixelBeacon.lua      # NUM_BLOCKS 10->16, COOLDOWN_* constants, renderCooldowns,
│                        # slot index derivation, PLAYER_ACTIVATED re-baseline
└── PixelBeacon.txt      # manifest 10->11, description

src/pixelbus/
└── mod.rs               # SlotCooldown, CooldownSet, six markers + quantization,
                         # BLOCK_CENTER_GREENS entries, NUM_BLOCKS, six point
                         # helpers, BlockSamples fields, decode_cooldown,
                         # PixelBusEvent::Cooldowns, observe + signal-loss arms

src/weave/mod.rs         # inert cooldowns() / set_cooldowns() store
src/app/
├── mod.rs               # CooldownView per slot, view-model field
├── routing.rs           # PixelBusEvent::Cooldowns -> weave.set_cooldowns
├── strings.rs           # SKILL_COLUMNS gains a cooldown column
└── ui.rs                # the new cell on each skill row

tests/
├── pixelbus.rs          # decode, observe, geometry, single-row maximum
├── beacon.rs            # cross-language constants, manifest version
├── app_strings.rs       # SKILL_COLUMNS length
└── app_ui_sizing.rs     # the wider skills region
```

**Structure Decision**: unchanged single-crate layout, no new module. Every change
extends a file that already carries the equivalent resource-block code.

## Phase Outputs

- **Phase 0**: [research.md](research.md) resolving the slot mapping, the marker
  set, the quantization, and the aggregate shape. No NEEDS CLARIFICATION remain.
- **Phase 1**: [data-model.md](data-model.md),
  [contracts/cooldown-blocks.md](contracts/cooldown-blocks.md),
  [quickstart.md](quickstart.md).
