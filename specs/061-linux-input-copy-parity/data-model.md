# Data Model: Linux Input and Copy Parity

## Canonical key universe

- Type: `Key::ALL: [Key; 13]`
- Invariants: every variant appears exactly once; parse/display/native tests consume the same set.

## Linux capability profile

- `application_keys`: native Linux keys derived from `Key::ALL`
- `mouse_buttons`: BTN_LEFT and BTN_RIGHT
- `physical_keys`: all keys reported by the selected keyboard
- `advertised_keys`: union of the three sets
- `physical_coverage_known`: whether a keyboard capability set has been applied to the current virtual device

Transitions:

1. No virtual device.
2. Optional app-only device created by early synthesis.
3. Keyboard discovered and union calculated.
4. Device retained if it already covers the union, otherwise replaced before grab.
5. Keyboard grabbed and key-only forwarding begins.

## Menu evidence state

- `Unavailable`: fail-closed; input, Fishing, and Auto Potion gated.
- `Gameplay`: explicit `MenuSurface::None`; applicable menu gates open.
- `Menu(surface)`: a gating surface; applicable menu gates closed.

The initial state is `Unavailable`. Only an explicit valid gameplay observation opens it.

## Behavior copy contract

- Latency: additive, bounded, Light Attack and Bash, 300 ms cap.
- Logging: global captured level, Live Log ring, optional file sink, persisted choice.
- Menu evidence: unavailable means gated, including startup and signal loss.
