# Quickstart and Validation: PixelBeacon Resource Blocks

**Feature**: 033-resource-blocks | **Date**: 2026-07-27

## Prerequisites

The pinned toolchain (Rust 1.96.0). No new dependency.

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
| Every publishable percentage decodes to itself | `tests/pixelbus.rs` |
| Bounded error: no in-tolerance perturbation yields a different percentage | `tests/pixelbus.rs` |
| Monotonicity across the full range | `tests/pixelbus.rs` |
| A wrong marker, failed checksum, or out-of-range payload yields unavailable | `tests/pixelbus.rs` |
| No arbitrary colour decodes as a resource | `tests/pixelbus.rs` |
| The three resources decode independently and are not confused | `tests/pixelbus.rs` |
| Each clears on signal loss and on a non-decoding block | `tests/pixelbus.rs` |
| Every block-centre green stays pairwise separated with ten markers in use | `tests/pixelbus.rs` |
| Addon and companion agree on the block count and all three markers | `tests/beacon.rs` |
| Resource values change no engine behavior | `tests/weave_engine.rs` |
| The view renders a percentage, and not-detected | `tests/app_view_model.rs` |

The bounded-error and monotonicity rows are exhaustive over the full publishable
range crossed with every in-tolerance perturbation, not sampled. That is possible
only because the payload is numeric; it is the concrete benefit of reversing the
issue's encoding.

## In-game observation

If you want to eyeball it: install the updated addon, run `/reloadui`, and watch
the three readouts while spending and regenerating each pool. They should track the
game's own bars, read 100 when full and 0 when empty, and show "Not detected" if
the addon is older than version 8.

The one thing worth a glance rather than a procedure: at the operator's default log
level there should be **no** resource lines at all. If the live log is full of them,
the trace-level decision did not take effect and the log is no longer usable for
diagnosing anything else.
