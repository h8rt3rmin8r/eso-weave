# Quickstart and Validation: Pixel Bus Grid Wrap

**Feature**: 035-grid-wrap | **Date**: 2026-07-27

## Prerequisites

The pinned toolchain (Rust 1.96.0). No new dependency and no new Cargo feature.
The addon changes, so an existing install is offered an update to version 9.

## Desk validation

Foreground, watched to completion. Never backgrounded.

```bash
cargo fmt --all -- --check
```

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

```bash
cargo test --all --locked
```

### What the suite proves

| Behavior | Where |
| --- | --- |
| Positions wrap: index maps to column then row, at several column counts | `tests/pixelbus.rs` |
| No two indices below the count share a position | `tests/pixelbus.rs` |
| Every index below the count lands inside the extent | `tests/pixelbus.rs` |
| A partial final row is allowed and the extent is not required to be full | `tests/pixelbus.rs` |
| The extent width is the lesser of the count and the column count | `tests/pixelbus.rs` |
| Row count is the ceiling, with no phantom row at an exact multiple | `tests/pixelbus.rs` |
| **Every block position at nine blocks equals the pre-wrap formula** | `tests/pixelbus.rs` |
| **The captured region at nine blocks equals the pre-wrap region** | `tests/pixelbus.rs` |
| Sampled centres stay whole pixels at every supported block size | `tests/pixelbus.rs` |
| The heartbeat block is at grid (0, 0) for every column count tried | `tests/pixelbus.rs` |
| The shipped column count satisfies both bounds (at least the block count; one row fits 1024 at the largest block) | `tests/pixelbus.rs` |
| The addon and the companion state the same column count | `tests/beacon.rs` |
| The addon manifest is at version 9 | `tests/beacon.rs` |
| A grid inside the client area fits; one wider or taller does not | `tests/pixelbus_display.rs` |
| No report without a measurement, or from a configured descriptor | `tests/pixelbus_display.rs` |
| One report per change of outcome, not per change of descriptor | `tests/pixelbus_display.rs` |
| Every pre-existing signal decodes exactly as before | `tests/pixelbus.rs`, unchanged assertions |

The two bolded rows are the ones that matter most. They are the evidence that
this is a contract change and not a behaviour change, and they are written as
explicit assertions against the old arithmetic spelled out in the test rather
than as a reference to the new arithmetic, so they cannot both drift together.

## In-game observation

The expected observation is that nothing happens, which makes this shorter than
the recent slices.

Install the update (version 9), run `/reloadui`, and look at the top-left corner
of the game. The strip of nine squares should be exactly where it was, the same
size, in the same order. Every readout in the application should behave as it did:
combat state, menu state, the weapon bars, the three resources, and fishing.

The one thing worth a deliberate look: at the operator's normal log level there
should be **no** grid-fit warnings. One appearing means a block size and block
count combination is putting part of the grid off-screen, which at nine blocks
and any supported block size should be impossible on any real display, and would
point at something wrong with the measurement rather than with the grid.

If you want to see the wrap actually wrap, that needs a seventeenth block, which
is the next feature that adds one. There is deliberately nothing here that
demonstrates row 1 in the live game, because demonstrating it would have required
drawing a block that carries no signal.
