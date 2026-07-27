# Implementation Plan: Pixel Bus Grid Wrap

**Branch**: `035-grid-wrap` | **Date**: 2026-07-27 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/035-grid-wrap/spec.md`

## Summary

Wrap the beacon blocks from one ever-widening row into a grid, on both sides of
the contract, using a fixed column count of 16 that the build asserts the two
sides agree on. Derive the captured region and the addon's drawn extent from the
same grid arithmetic. Add an advisory check that the grid fits inside the
measured client area.

At the current nine blocks the wrapped layout is the strip layout, coordinate for
coordinate, and the captured region is unchanged. Proving that is a deliverable,
not a side effect: it is what makes a geometry change to the contract every signal
depends on safe to land before any signal needs it.

## Technical Context

**Language/Version**: Rust 1.96.0 (pinned), edition 2021, plus Lua for the addon

**Primary Dependencies**: unchanged; no new dependency and no new Cargo feature

**Storage**: None. The grid is derived arithmetic; the column count is a compile-time constant on both sides, deliberately not configurable (see [research.md](research.md) R2).

**Testing**: `cargo test --all --locked`. The grid arithmetic is pure and is tested across column counts other than the shipped one; the no-change property is asserted against the pre-wrap formulas written out explicitly rather than referenced.

**Target Platform**: Windows 10/11 x64 and Linux x64

**Performance Goals**: Unchanged. The captured area is set by the block count and block size, not by the arrangement (research R1), and at nine blocks the captured region is byte-identical to today's.

**Constraints**: Two sides must compute identical positions, enforced by the build rather than by review. Text: UTF-8 without BOM, LF, no em-dashes or en-dashes anywhere including code comments.

**Scale/Scope**: One constant and three small pure functions on the companion side, one helper rewritten on the addon side, one manifest bump, one advisory check, and the tests that pin all of it.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

| Principle | Assessment |
| --- | --- |
| I. Spec-Driven Development | PASS. Traces to build plan 011 slice 035 and issue #16; full sequence run, two checklists precede this plan, one of which found a factually wrong requirement (FR-005 claimed a coverage property that is false for a partial final row). |
| II. Safety-Critical Surfaces Are Sacrosanct | PASS. No input path, no interception decision, no beacon lifecycle change, no change to the marker-gated uninstall. The addon file changes, so the install and uninstall paths carry it, but their rules are untouched. |
| III. Test-First With Explicit Seams | PASS. Every piece of new logic is a pure function over integers. The one runtime behaviour (the fit report) is a pure change-detecting type with the descriptor injected. |
| IV. CI Parity Before Every Commit | PASS. |
| V. Bounded Scope: Outside The Game | PASS. This moves squares the addon already draws; it reads nothing new from the game. |

**Post-design re-check (after Phase 1)**: PASS, unchanged. No entry in Complexity Tracking.

## Project Structure

### Documentation (this feature)

```text
specs/035-grid-wrap/
├── plan.md, research.md, data-model.md, quickstart.md
├── contracts/grid-layout.md
├── checklists/{requirements.md, geometry.md}
└── tasks.md
```

### Source Code (repository root)

```text
addon/PixelBeacon/
├── PixelBeacon.lua      # local COLUMNS, positionBlock by index, root from grid
└── PixelBeacon.txt      # Manifest version 8 to 9

src/
├── pixelbus/
│   ├── mod.rs           # COLUMNS, grid_position, grid_rows, grid_extent;
│   │                    # block_center and capture_dims derive from them
│   └── display.rs       # GridFit, GridFitWatch (advisory, change-detected)
└── main.rs              # Fit evaluated and warned inside the existing
                         # display-change branch

tests/
├── pixelbus.rs          # Geometry table extended; the no-change assertions
├── pixelbus_display.rs  # GridFit and GridFitWatch
└── beacon.rs            # COLUMNS agreement, manifest version pin
```

## Decisions

### Decision 1: sixteen columns

**Chosen**: `COLUMNS = 16`.

The constraints leave `9 <= C <= 32` and the table in [research.md](research.md)
R2 works through the candidates. 16 keeps the widest grid at half the narrowest
supported client width and keeps a 256-block grid square. 32 satisfies the width
bound exactly rather than comfortably, which is a bound one patch away from being
violated.

Capture cost played no part, because it does not depend on the arrangement
(research R1). That is worth recording precisely because "wider is cheaper" is an
intuitive and wrong reason someone might later reach for when revisiting this.

### Decision 2: the column count is fixed and shared, not derived

**Chosen**: a constant stated once per side, asserted equal by the build.

This reverses the premise of issue #3 and demotes what slice 034 built from the
source of the column count to a fit check. The full argument is in research R4;
the short form is that a derived count fails silently and plausibly. A
one-column disagreement between the two sides does not produce garbage, it
produces every signal from row 1 onward reported as some other signal's value,
with markers and checksums all passing, because the error is at the layout layer
underneath the validation that would otherwise catch it.

Saying this plainly matters more than usual here, because the earlier framing is
written down in a filed issue and in slice 034's own specification. Leaving it
uncorrected would leave the next reader with a false account of why the
descriptor exists.

### Decision 3: the position helper keeps its signature

**Chosen**: `block_center(block_px, index)` is unchanged in shape and consults
`COLUMNS` itself; `grid_position(index, columns)` is a separate pure function
that takes the column count as a parameter.

Every existing caller of `block_center` is untouched, which keeps the diff to the
arithmetic rather than to the call graph. The parameterised helper is what lets
the wrap's properties (injectivity, row growth, region coverage) be tested at
column counts other than 16 without changing what ships, which FR-011 requires.

### Decision 4: the fit check needs no new state in the worker

**Chosen**: the fit is evaluated inside the existing display-change branch, and
`GridFitWatch` change-detects the outcome.

The block size is fixed for the lifetime of the worker thread (`reader_config` is
copied into it and never updated), so the fit outcome is a pure function of the
descriptor and can only change when the descriptor changes. Slice 034 already
change-detects the descriptor and hands back an update exactly then, so the check
rides that branch for free.

`GridFitWatch` is still worth its fifteen lines rather than logging directly on
every descriptor change: two successive descriptor changes can both fail to fit
(a too-small window resized to a differently too-small window), and FR-019 asks
for a report per change of *outcome*, not per change of descriptor. Making that
distinction a testable type is cheaper than making it a comment nobody checks.

### Decision 5: warning level, breaking the recent convention deliberately

**Chosen**: the does-not-fit report is a warning, not the debug level slices 031
through 034 used for their new signals.

Those were observations about the game: interesting while diagnosing, noise
otherwise. This is an actionable misconfiguration whose symptom (signals reading
as absent while the addon is visibly installed and running) is one an operator
would otherwise spend real time misdiagnosing. A line at a level nobody runs at
would not do the job the check exists for.

### Decision 6: the manifest advances although the pixels do not

**Chosen**: version 8 to 9.

The usual reason does not apply, and the plan says so rather than implying it
does: both versions draw identical pixels at nine blocks, so nothing breaks for an
operator who never updates. It advances so the deployed addon carries the wrapping
logic, which means the next slice to add a block inherits a working grid instead of
having to ship the wrap and the block together and bump twice.

## Complexity Tracking

No constitution violations. No entries.
