# Quickstart and Validation: Out-Of-Band Display Detection

**Feature**: 034-display-detection | **Date**: 2026-07-27

## Prerequisites

The pinned toolchain (Rust 1.96.0). No new dependency and no new Cargo feature.
No addon change, so nothing to reinstall and no `/reloadui`.

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
| A realistic settings file parses every key of interest | `tests/pixelbus_display.rs` |
| A key with a `.N` version suffix still matches | `tests/pixelbus_display.rs` |
| Missing, empty, unrelated, truncated, and non-`SET` content yields a partial or empty reading, never a panic | `tests/pixelbus_display.rs` |
| An unparsable value leaves only its own field absent | `tests/pixelbus_display.rs` |
| A half-present resolution pair yields no pair | `tests/pixelbus_display.rs` |
| A duplicate key resolves to the last assignment | `tests/pixelbus_display.rs` |
| An unknown window-mode value is carried raw and maps to no named mode | `tests/pixelbus_display.rs` |
| A zero or absent surface yields no descriptor | `tests/pixelbus_display.rs` |
| A measured descriptor carries what the probe supplied, and absent fields stay absent | `tests/pixelbus_display.rs` |
| A configured descriptor is produced only when both stored pairs agree, and carries no display geometry | `tests/pixelbus_display.rs` |
| Reconciliation returns agreed, ambiguous, disagreed, and the no-settings cases correctly | `tests/pixelbus_display.rs` |
| A stored reading never alters a measured descriptor | `tests/pixelbus_display.rs` |
| An unchanged measurement produces no update **and no settings read** | `tests/pixelbus_display.rs` |
| A changed measurement produces exactly one update and exactly one settings read | `tests/pixelbus_display.rs` |
| A lost measurement clears the descriptor and reads nothing | `tests/pixelbus_display.rs` |
| A recovered measurement re-resolves without restart | `tests/pixelbus_display.rs` |
| Detection creates and modifies nothing on disk | `tests/pixelbus_display.rs` |
| The settings path is the AddOns directory's parent and is never created | `tests/beacon.rs` |
| Every pre-existing pixel-bus behavior is unchanged | `tests/pixelbus.rs`, unmodified |

The read-gating rows are the ones worth reading the assertions of. They count
closure invocations across a scripted sequence of measurements, which is what
turns "the settings file is not read on every cycle" from an intention into a
tested guarantee.

## What cannot be proven at the desk

Two success criteria need a real window on a real monitor, and the plan does not
pretend otherwise:

- **SC-001**: the reported surface and display geometry match what the game and
  the operating system say, on both a scaled and an unscaled display.
- **SC-002**: moving between monitors, resizing, and changing display mode each
  produce an updated descriptor within one sampling cycle.

## In-game observation

If you want to eyeball it: run the application at debug log level with the game
running, then move the game window to another monitor, resize it, and toggle
between windowed and fullscreen. Each change should produce one line reporting
the new surface and display geometry, and leaving the window alone should
produce none at all.

The line worth looking for specifically is the reconciliation one. When the
stored settings are readable it reports which stored resolution pair the measured
surface matched and what the raw window-mode value was alongside it. That pairing
is the evidence issue #3 asked someone to go and gather by toggling modes and
diffing the file; it now accumulates from ordinary use. Nothing acts on it, so
there is nothing to verify beyond noticing that it appears.

On Linux, expect no scale in the output (X core does not expose one) and expect
the display size to be the whole X screen on a multi-head session. Both are
documented behavior, not defects.
