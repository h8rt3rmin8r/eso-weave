# Implementation Plan: PixelBeacon In-Combat State Block

**Branch**: `031-combat-state-block` | **Date**: 2026-07-27 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/031-combat-state-block/spec.md`

## Summary

Add a fifth block (B4) to the PixelBeacon strip carrying the player's combat
state, decode it in the companion, and surface it beside the weapon-bar readout.
The block is encoded with a dedicated green marker plus a red state code and a
blue checksum, following the latency block's marker-and-checksum pattern rather
than the weapon block's exact-match pattern, because an exact match on adjacent
integer codes cannot survive the reader's own match tolerance.

Alongside the block, this slice pays the one-time factoring that build plan 010
depends on: the strip's block count becomes a named constant on each side of the
addon-to-companion contract with an automated agreement check, and the reader's
four-positional-argument `observe` becomes a defaulted `BlockSamples` struct so
the next three slices add a field instead of rewriting every call site.

Nothing consumes the decoded value. That boundary is enforced by a test, not
just asserted.

## Technical Context

**Language/Version**: Rust 1.96.0 (pinned in `rust-toolchain.toml`), edition 2021, plus Lua for the bundled ESO addon

**Primary Dependencies**: egui/eframe for the interface, tracing for structured logging, serde/serde_json for config. No new dependency is introduced by this feature.

**Storage**: None. Combat state is live observed state and is never persisted; the constitution forbids writing runtime state to the config file.

**Testing**: `cargo test --all --locked`. Pure decoders and view derivations are unit tested with crafted samples; the addon-to-companion contract is tested by parsing the Lua source embedded in the binary via `include_str!`.

**Target Platform**: Windows 10/11 x64 and Linux x64, running beside the game client. The addon runs inside ESO.

**Project Type**: Desktop companion application, single Rust crate with per-OS backend modules.

**Performance Goals**: Unchanged. This feature adds one point sample per existing strip read and no additional screen capture; per FR-019 the sampling cadence is untouched.

**Constraints**: The added block widens the captured strip from four to five block widths. Decoding stays pure and allocation-free on the sampling path. All text files UTF-8 without BOM, LF, and free of em-dashes and en-dashes including code comments.

**Scale/Scope**: One addon block, one decoder, one view row, and one signature change with its call sites. Roughly five source files and four test files.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

| Principle | Assessment |
| --- | --- |
| I. Spec-Driven Development | PASS. Feature traces to build plan 010 slice 031 and master specification section 10.3, and runs the full spec-kit sequence. `spec.md` and both checklists precede this plan. |
| II. Safety-Critical Surfaces Are Sacrosanct | PASS, and explicitly so. This feature adds no input path and touches no safety surface. FR-017 names each surface that must remain untouched. The one risk of drift, someone later assuming the combat signal gates something, is closed by FR-016 plus a test asserting the weave engine's behavior is identical with combat state set to each value. |
| III. Test-First With Explicit Seams | PASS. The decoder, the view derivation, and the cross-side contract check are all pure and testable without the game or a screen. The existing `SurfaceSampler` seam and `MockSampler` cover the sampling path unchanged. Every task is written failing-test-first. |
| IV. CI Parity Before Every Commit | PASS. This feature changes Rust sources, so the full gate runs in the foreground before commit. |
| V. Bounded Scope: Outside The Game | PASS. The signal is published by the existing PixelBeacon screen-signal contract, which is the one sanctioned in-game surface. No process memory, no packet traffic, no new in-game functionality beyond one more color block. |

**Post-design re-check (after Phase 1)**: PASS, unchanged. The design added no
new dependency, no new thread, no new synchronization primitive, and no new
persisted field. The one design choice with a governance dimension, where the
decoded value is stored, is recorded as Decision 5 below and was made to avoid
introducing a second shared-state mechanism.

## Project Structure

### Documentation (this feature)

```text
specs/031-combat-state-block/
├── plan.md                # This file
├── research.md            # Phase 0 output
├── data-model.md          # Phase 1 output
├── quickstart.md          # Phase 1 output
├── contracts/
│   └── pixel-bus-b4.md    # The B4 wire contract
├── checklists/
│   ├── requirements.md    # Spec quality (from /speckit-specify)
│   └── beacon-contract.md # Block contract discipline (from /speckit-checklist)
└── tasks.md               # Phase 2 output (/speckit-tasks)
```

### Source Code (repository root)

```text
addon/PixelBeacon/
├── PixelBeacon.lua      # B4 render, combat events, NUM_BLOCKS, shared constants
└── PixelBeacon.txt      # Manifest version 5 to 6, description line

src/
├── pixelbus/
│   └── mod.rs           # CombatSignal, decode_combat, BlockSamples, NUM_BLOCKS = 5,
│                        # the block-center green registry, reader state and event
├── beacon/
│   └── mod.rs           # parse_lua_constant for the cross-side agreement check
├── weave/
│   └── mod.rs           # set_combat and combat storage (stored, never read for decisions)
└── app/
    ├── mod.rs           # CombatView, combat_view, AppView field
    ├── routing.rs       # PixelBusEvent::Combat routed to the store
    ├── strings.rs       # COMBAT_TITLE, COMBAT_TOOLTIP
    └── ui.rs            # The combat row beside the weapon-bar row

tests/
├── pixelbus.rs          # Decoder, tri-state, marker registry, BlockSamples, reader events
├── beacon.rs            # parse_lua_constant, cross-side NUM_BLOCKS and color agreement
├── app_view_model.rs    # combat_view derivation, routing, AppView
└── weave_engine.rs      # FR-016 boundary: combat state changes no engine behavior

docs/
└── ESO-Weave-Specification-v0.2.0.md  # Section 10.3 gains B4
```

**Structure Decision**: The existing single-crate layout is unchanged. This
feature adds no module; every change lands in a module that already owns the
concern (`pixelbus` for the wire contract and decoding, `beacon` for the addon
lifecycle and its embedded source, `app` for routing and display).

## Decisions

Recorded under the build-phase autopilot decision policy. Each enumerates the
alternatives, evaluates them against the constitution, build plan 010, and the
existing code, and states what was chosen. Full reasoning is in
[research.md](research.md).

### Decision 1: B4 encoding

**Chosen**: green marker `0x2D`; red carries the state (`0xE0` in combat, `0x20`
out of combat); blue carries the complement checksum (`255 - red`), validated the
way the latency block validates its own.

The marker sits at least 45 away from every green already at a block center
(`0x00` status, `0x80` and `0xFF` fishing, `0xA5` latency, `0x5A` weapon), which
is more than 20 times the default tolerance of 2. The two state codes sit 192
apart. Both satisfy FR-006 with a large margin.

The checksum is the part worth justifying. A boolean does not need one, but User
Story 2 requires that arbitrary screen content behind an absent block never
decode as a state. Marker plus checksum means an accidental match needs the green
channel near `0x2D` and red near one of two values and blue equal to its
complement. The latency block already establishes this pattern, so this is the
existing discipline rather than a new one.

Deliberately not copied: the weapon block's `ActiveBar::from_code`, which matches
its blue channel exactly against `0`, `1`, `2`. Codes one apart cannot be
distinguished under a tolerance of 2, so that block is only safe because its
capture path happens to be exact. Repeating it in a new block would be building
in a known fragility.

`0xD2`, the nibble swap of `0x2D`, is noted in the source as the natural marker
for the next block, continuing the `0xA5` and `0x5A` pairing the strip already
uses.

### Decision 2: the color registry (checklist item CHK006)

**Chosen**: create it, as a documented constant in `src/pixelbus/mod.rs` naming
every green that appears at a block center, plus a test asserting they are
pairwise separated by more than the default tolerance.

A prose registry would rot. A constant with a test does not: slice 032 adds its
marker to the list, and if the value collides with anything already there, the
test fails and names the collision. That converts FR-006 from a rule someone has
to remember into one the suite enforces, which is what makes it safe to hand to
three following slices.

Rejected: leaving it to prose in the master specification (unenforced), and
deferring it entirely (the point of this slice is to leave the pattern better
than it found it, and the registry costs one constant and one test).

### Decision 3: the `observe` signature

**Chosen**: a `BlockSamples` struct with one `Option<Rgb>` field per block and a
`Default` implementation, replacing the four positional arguments.

Existing tests construct sample sets one field at a time; with struct-update
syntax they read `BlockSamples { status: Some(c), ..Default::default() }`. When
slice 032 adds a sixth block it adds a field, and every existing construction
keeps compiling untouched. That is precisely FR-014.

Rejected: a slice (`&[Option<Rgb>]`) loses all arity and position safety, so a
caller passing blocks in the wrong order still compiles; and a fixed array
(`[Option<Rgb>; NUM_BLOCKS]`) keeps arity safety but forces every literal to gain
an element on every future slice, which is the churn this decision exists to
prevent.

This slice pays a one-time cost: every current `observe` call site and test moves
to the struct. That cost is the reason build plan 010 sequences this feature
first.

### Decision 4: the block count and the cross-side check

**Chosen**: `local NUM_BLOCKS = 5` in the addon with its root width computed as
`BLOCK_PX * NUM_BLOCKS`; `pub const NUM_BLOCKS: u32 = 5` in the companion; and a
test that parses the embedded addon source and asserts the two agree.

The check is possible because `beacon::LUA` embeds the addon source with
`include_str!`, so the companion's own test suite can read exactly the Lua it
ships. A small pure helper, `parse_lua_constant(source, name)`, handling decimal
and hex literals, mirrors the existing `parse_manifest_version` and is reused to
assert the marker and both state codes agree across the two languages, giving the
"shared byte for byte" discipline actual teeth for the first time.

This also repairs an existing inaccuracy: the doc comment on
`beacon::rewrite_block_px` already tells the reader the addon derives its strip
width from `BLOCK_PX * NUM_BLOCKS`, describing a constant the Lua does not
currently have. After this slice that sentence is true.

Rejected: rewriting the count into the addon at deploy time the way `BLOCK_PX`
is rewritten. `BLOCK_PX` is rewritten because it is a user setting; the block
count is a property of the code pair, not of the user, and making it a deploy
parameter would let a stale addon and a fresh companion disagree silently, which
is the exact failure the check exists to catch.

### Decision 5: where the decoded value is stored

**Chosen**: on the weave engine, beside the latency and weapon-bar state it
already holds, reached through `set_combat` and `combat`.

The engine is already the shared home for every beacon-derived observable, it
already sits behind the mutex that both the reader thread and the interface
thread take, and using it means `route_reader_event` and the view builder gain a
line each rather than a new parameter threaded through the existing call sites in
`src/main.rs` and `tests/app_view_model.rs`.

The obvious objection is that storing it in the engine hints that the engine uses
it, which FR-016 forbids. That is answered with a test rather than a comment: the
engine's behavior is asserted identical with combat state set to in combat, out
of combat, and unavailable. If a later slice wires combat into timing, that test
fails and forces the change to be deliberate.

Rejected: a separate shared cell for combat state. It adds a second
synchronization primitive and a new parameter for a value nothing reads. When
slice 032 needs an input-engine-side home for the menu gate, it will have a real
consumer to justify the plumbing; this slice does not.

## Complexity Tracking

No constitution violations. No entries.
