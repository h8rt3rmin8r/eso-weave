# Implementation Plan: PixelBeacon Movement-State Block

**Branch**: `main` (trunk-based) | **Date**: 2026-07-27 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/036-movement-state-block/spec.md`

## Summary

Add a tenth beacon block (B9) publishing the player's movement state, whose only
live axis is whether the player is mounted. The addon renders the block from
`IsMounted()`, driven by `EVENT_MOUNTED_STATE_CHANGED` and re-baselined on
`EVENT_PLAYER_ACTIVATED`; the companion samples it, decodes it to a tri-state,
routes it to the view model beside the combat readout, and logs changes at DEBUG.
Nothing acts on the value.

The technical approach is deliberately unoriginal: the combat block (slice 031)
is the reference implementation and this slice is one more application of it. The
encoding reuses the marker-plus-checksum shape combat and menu share, the
optional-block semantics are combat's byte for byte, and the geometry rides the
derived `block_center` the grid wrap already provides. The only genuinely new
design work is the choice of a validity mark, the code table's reservation of the
deferred sprint axis, and stating the capture-extent invariant in terms of what
it actually depends on.

## Technical Context

**Language/Version**: Rust (edition and toolchain pinned by `rust-toolchain.toml`)
and Lua 5.1 for the embedded addon, the version the game runtime provides.

**Primary Dependencies**: no new dependencies. The feature touches existing
modules only.

**Storage**: none. Movement state is runtime state and is never written to the
config file, per the constitution's configuration constraint.

**Testing**: `cargo test --all --locked`, with the new coverage landing in
`tests/pixelbus.rs` (decode, observe, geometry) and `tests/beacon.rs` (the
cross-language constant agreement check and the manifest version).

**Target Platform**: Windows 10/11 x64 and Linux x64. This feature is
platform-agnostic; it adds no platform-specific code and touches neither
`src/pixelbus/windows.rs` nor `src/pixelbus/linux.rs`.

**Project Type**: desktop application, single Rust crate, with the PixelBeacon
addon embedded in the binary and installed to the game's AddOns directory.

**Performance Goals**: unchanged. FR-022 forbids a cadence change, so this
feature adds one sampled point per existing capture and no additional capture.

**Constraints**: the addon-to-companion color contract must be identical byte for
byte on both sides and proven so automatically; all text stays UTF-8 without BOM,
LF, and free of em-dashes and en-dashes.

**Scale/Scope**: one new block on a ten-block grid, one new decoder, one new
event variant, one new view field. No new module or file in `src/`.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

| Principle | Status | Evidence |
| --- | --- | --- |
| I. Spec-Driven Development | PASS | Traces to master specification section 10.3 and `docs/plans/plan-010.md` slice 036. Full spec-kit sequence run; this is the plan step. `analyze` runs before `implement` and is not skipped. |
| II. Safety-Critical Surfaces Are Sacrosanct | PASS | The feature adds no input path and gates nothing (FR-019, FR-020). No safety test is touched, weakened, or made conditional. The one adjacent surface, fishing degrading to disabled on `SignalLost`, is unaffected because the new block clears to `Unknown` through the same signal-loss branch that already exists. |
| III. Test-First With Explicit Seams | PASS | Every behavior lands as a failing test first. The seams already exist and are reused unchanged: `BlockSamples` for the reader and `parse_lua_constant` for the cross-language check. No new seam is needed, which is itself the evidence that this slice fits the established pattern. |
| IV. CI Parity Before Every Commit | PASS | `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all --locked` all run in the foreground before the commit and are watched to completion. |
| V. Bounded Scope: Outside The Game | PASS | Reads no process memory and no network traffic. The addon uses only the published, unprotected `IsMounted()` and `EVENT_MOUNTED_STATE_CHANGED`, and communicates solely through the existing screen-signal contract. |

**Post-design re-check**: PASS, unchanged. The Phase 1 design introduces no new
module, dependency, thread, file, or input path, so no gate moves.

**Complexity Tracking**: not applicable. No violations to justify.

## Design Decisions

Recorded here per the autopilot decision policy. Full alternatives analysis is in
[research.md](research.md).

### D1: The validity mark is `0x43`

The green channel identifies which signal a block carries. Ten greens are already
in use (`0x00`, `0x16`, `0x2D`, `0x5A`, `0x6D`, `0x80`, `0xA5`, `0xBB`, `0xD2`,
`0xFF`). Sorted, their largest interior gaps are `0x2D` to `0x5A` and `0xD2` to
`0xFF`, both 45 wide. `0x43` is the midpoint of the first, sitting 22 from `0x2D`
and 23 from `0x5A`, and further than that from all eight others. Twenty-two is
eleven times the default tolerance of 2, and matches the separation the resource
markers already ship with.

`0xE8`, the midpoint of the other gap, is exactly as good by the separation
metric. The tiebreak is distance from the extremes: unrelated screen content
behind the overlay clusters at black and white far more than at mid-range, and
`0x43` sits 67 from `0x00` and 188 from `0xFF` where `0xE8` sits 23 from `0xFF`.
With the checksum as a second gate this only shifts an already small
false-positive probability, but it shifts it in the right direction at no cost.

The mark is added to `BLOCK_CENTER_GREENS`, which makes `tests/pixelbus.rs` prove
the separation rather than leaving it asserted here (FR-006).

Note the nibble-swap convention (`0xA5`/`0x5A`, then `0x2D`/`0xD2`) is not
continued. It was a mnemonic for choosing a second value near a first, it was
already abandoned by the resource markers, and `0x34` (the swap of `0x43`) would
sit 7 from `0x2D`, well inside the separation this slice needs.

### D2: The code table is a two-bit code with two reserved values

The red channel carries the code and blue carries `255 - red` as a checksum,
exactly as combat and menu do. The code is the two-bit value issue #11 proposed,
bit 0 for mounted and bit 1 for sprinting, mapped to four evenly spaced reds:

| Code | Meaning | Red | Status |
| --- | --- | --- | --- |
| `0b00` | On foot | `0x20` | Live |
| `0b01` | Mounted | `0x60` | Live |
| `0b10` | Sprinting, on foot | `0xA0` | Reserved, never emitted |
| `0b11` | Sprinting, mounted | `0xE0` | Reserved, never emitted |

Even spacing of 64 is 32 times the default tolerance, so no code can be read as
its neighbour. Reserving the two sprint codes rather than packing the live values
tighter is what lets a future sprint feature add its axis by emitting codes that
already exist in the table, with no new block, no new mark, and no renumbering of
the live values (FR-011).

Per FR-012 the reserved codes are defined and rejection-tested only on the
companion side. Defining them in the addon would create constants it never emits,
which the cross-language check would then have to special-case for values living
on one side only.

`decode_movement` matches the two reserved codes explicitly and returns the
unavailable state, rather than letting them fall through the catch-all that
already rejects anything unrecognized. The behavior is identical either way; the
explicit arm is chosen for two reasons. It documents the reservation in
executable form, so a reader of the decoder sees the deferred axis without
consulting the contract. And it keeps the reserved constants read by `src/`,
which matters concretely: every sibling marker in this module is a private
`const`, and two private constants that nothing reads would trip `dead_code`
under the `-D warnings` gate that constitution Principle IV requires. This was
caught by the `analyze` gate as finding C1.

### D3: The names are movement names, the values are mounted names

`MovementSignal { Unknown, OnFoot, Mounted }`, `PixelBusEvent::Movement(..)`,
`ReaderConfig::movement_point()`, `movement_view()`, and `BlockSamples.movement`.
"Mounted" appears only as a value. This is FR-011's naming half: when sprint
arrives it adds variants to an enum that is already named for the right concept
instead of forcing a rename across the reader, the event, the router, the view
model, and the interface.

### D4: The operator sees "Mounted", "On foot", "Not detected"

Read from `combat_view` rather than invented, closing checklist item CHK038.
`combat_view` renders "In combat", "Out of combat", "Not detected" with
`StatusRole::Active` when detected and `StatusRole::Muted` when not.
`movement_view` mirrors that structure exactly, so the operator reads three
adjacent player-state fields under one convention.

### D5: The extent assertion is about the column count, not about ten

FR-017, as sharpened by checklist item CHK028. The capture region is unchanged
because `NUM_BLOCKS <= COLUMNS`, not because `NUM_BLOCKS == 10`. The test asserts
the general property (`grid_extent` for any count at or below `COLUMNS` is one
row) plus the concrete instance for the shipping constants, so the slice that
eventually adds the seventeenth block inherits a test that already describes the
boundary instead of one that quietly stops meaning anything.

### D6: The manifest advances 9 to 10

`## Version` and `## AddOnVersion` both move to 10 so the beacon manager offers
the update, and the description line gains "whether the player is mounted"
(FR-018). `tests/beacon.rs::embedded_manifest_version_is_nine` is renamed and
retargeted; it is a version assertion, not a safety test, so retargeting it is
correct rather than a weakening.

## Project Structure

### Documentation (this feature)

```text
specs/036-movement-state-block/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/
│   └── movement-block.md
├── checklists/
│   ├── requirements.md
│   └── beacon-contract.md
├── spec.md
└── tasks.md             # Phase 2 output (/speckit-tasks)
```

### Source Code (repository root)

```text
addon/PixelBeacon/
├── PixelBeacon.lua      # NUM_BLOCKS 9->10, MOVEMENT_* constants, renderMovement,
│                        # EVENT_MOUNTED_STATE_CHANGED, PLAYER_ACTIVATED re-baseline
└── PixelBeacon.txt      # manifest Version/AddOnVersion 9->10, description line

src/pixelbus/
└── mod.rs               # MovementSignal, MOVEMENT_MARKER + code constants,
                         # BLOCK_CENTER_GREENS entry, NUM_BLOCKS 9->10,
                         # movement_point, BlockSamples.movement, decode_movement,
                         # PixelBusEvent::Movement, observe wiring, capture wiring

src/app/
├── mod.rs               # MovementView, movement_view, view-model field
├── routing.rs           # PixelBusEvent::Movement -> weave.set_movement
└── ui.rs                # readout beside the combat field

tests/
├── pixelbus.rs          # decode, observe, signal-loss, geometry, extent invariant
└── beacon.rs            # cross-language constants, manifest version
```

**Structure Decision**: the existing single-crate layout is unchanged. This
feature adds no file and no module; every change is an extension of a file that
already carries the equivalent combat-block code. That is the intended shape,
because build plan 010's whole argument is that slice 031 paid the structural
cost once so the four blocks after it would not have to.

## Phase Outputs

- **Phase 0**: [research.md](research.md), resolving the marker choice, the code
  table, the reserved-axis representation, and the extent invariant. No
  NEEDS CLARIFICATION markers remain.
- **Phase 1**: [data-model.md](data-model.md) (the movement entity and its state
  transitions), [contracts/movement-block.md](contracts/movement-block.md) (the
  byte-level block contract shared by both sides), and
  [quickstart.md](quickstart.md) (the desk and in-game validation guide).
