# Quickstart and Validation: PixelBeacon Menu-State Input Gate

**Feature**: 032-menu-state-gate | **Date**: 2026-07-27

## Prerequisites

- The pinned toolchain (Rust 1.96.0). No new dependency.
- For the in-game section: ESO installed, the application able to resolve the
  AddOns directory, and at least one weave slot bound to an ordinary letter key so
  typing it in a text field is a real test.

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

### What the suite proves without the game

| Behavior | Where |
| --- | --- |
| The gate can only relax interception, over every input combination | `tests/input_engine.rs` |
| Focus scoping still passes everything when unfocused, gate or not | `tests/input_engine.rs` |
| Exempt toggle hotkeys still work while gated | `tests/input_engine.rs` |
| A gated fishing controller starts no interact keypress | `tests/fishing.rs` |
| Work already in progress still completes while gated | `tests/fishing.rs` |
| Every surface code decodes, and invalid samples yield no surface | `tests/pixelbus.rs` |
| An arbitrary color never decodes as a surface | `tests/pixelbus.rs` |
| The gate clears on signal loss and on a non-decoding block | `tests/pixelbus.rs` |
| Cadence is fast while intercepting or fishing, idle otherwise | `tests/pixelbus.rs` |
| Addon and companion agree on the block count and every menu constant | `tests/beacon.rs` |
| The view names each surface, gameplay, and not-detected | `tests/app_view_model.rs` |

The first row is the one that matters most. It is an exhaustive cross product, not
a set of chosen scenarios, because the risk it guards against is the combination
nobody thought of.

## In-game validation

Required before this feature is validated in the field. It is the only part that
exercises the real game API, the real capture path, and real typing.

### Setup

1. Build and run the application. Update the addon (the manager should offer it,
   since the manifest advanced to version 7) and run `/reloadui`.
2. Set the log level to DEBUG and open the live log pane.
3. Confirm at least one weave slot is bound to a letter you will type.

### Scenario 1: chat, the case this feature exists for (SC-001)

1. Stand in the world with the application running and not suspended.
2. Open chat and type a sentence containing the bound letter several times.

**Pass**: every character appears in the chat box, no weave fires, and the log
shows the gate engaging. **This is the scenario that would have failed had the gate
used the scene test instead of UI mode**, so a failure here is worth diagnosing
before anything else.

### Scenario 2: the other surfaces (SC-001)

Repeat scenario 1 for the system menu, the map, inventory, and composing a mail,
typing where a text field exists.

**Pass**: same result each time, and the readout names the surface (or reports the
generic one).

### Scenario 3: clean resumption (SC-003)

1. Close the surface and immediately weave.
2. Open and close surfaces at least ten times, weaving between each.

**Pass**: weaves fire every time after closing. A single failure to resume is a
hard fail; a gate that sticks is worse than no gate.

### Scenario 4: fishing is gated too (SC-001, the checklist finding)

1. Start a fishing session and wait for the cast to settle.
2. Open chat and type for several seconds.

**Pass**: no interact keypress lands in the chat box, and the log shows no cast or
reel initiated while the gate is active. **This is the path a gate on interception
alone would have missed.**

### Scenario 5: the operator keeps control (SC-004)

1. With a surface open, press the suspend hotkey, then press it again.
2. With a surface open, press the fishing hotkey.

**Pass**: both work, exactly as they do without a surface open.

### Scenario 6: older addon changes nothing (SC-006)

1. Close the application, restore the previous addon version (five blocks), and
   restart.

**Pass**: interception and weaving behave exactly as they did before this feature.
The readout shows not-detected. Any behavior change here is a defect.

### Scenario 7: latency is tolerable (SC-002)

While typing quickly, note whether any keystroke at the very start of opening chat
is swallowed or triggers a weave.

**Pass**: at most the first keystroke or two, consistent with the stated
two-interval bound. This scenario records the limitation rather than asserting it
away; if it feels worse than that, the addon publish cadence is the first thing to
check.
