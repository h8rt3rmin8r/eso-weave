# Research: Linux Input and Copy Parity

## Linux capability discovery

`LinuxBackend` currently builds a uinput device from a hand-written 13-code array that omits `KEY_E` and `KEY_F3` while `to_ev_key` can synthesize both. The interception loop grabs a physical keyboard, forwards every raw event, and discards `emit` failures. A uinput device cannot reliably forward key codes it never advertised.

**Decision**: Derive application capabilities from `Key::ALL`, add both mouse buttons, union those with `Device::supported_keys()` before grab, and replace an already-created app-only virtual device when required.

**Rationale**: This covers shipped synthesis and preserves ordinary physical keys without expanding the application's configurable key model.

## Event forwarding boundary

Physical evdev batches may include KEY and metadata such as MSC_SCAN. The virtual device is configured only for keys.

**Decision**: Forward KEY events only. Return an explicit `InputError` when a forwarded key cannot be emitted. Ignore non-key metadata intentionally and document that boundary.

**Alternatives rejected**:

- Ignore all emit failures: silently loses grabbed user input.
- Advertise every evdev event family: unnecessary scope and device impersonation complexity.
- Advertise only application keys: continues to lose unrecognized physical keys.

## Menu evidence startup

Routing treats `MenuGate(None)` as gated, but `InputEngine` and `FishingController` initialize ungated. `PixelBusReader` also suppresses a first transition when both cached and decoded menu values are `None`. This means absent startup evidence can remain open.

**Decision**: Initialize generated-input menu gates closed, preserve Auto Potion's closed default, and ensure the first valid `Some(MenuSurface::None)` observation is published. Missing or lost evidence remains closed.

**Rationale**: Unknown evidence cannot authorize synthesis under the fail-closed safety contract.

## Behavior copy

The latency runtime adds `round(scale * measured_latency)` to Light Attack and Bash delays, clamped to 0 through 300 ms. The Live Log selector updates the global logging filter, changes both ring and file capture, and persists to settings.

**Decision**: Use semantic wording that names affected actions, additive direction, the cap, global capture, file behavior, and persistence. Add negative assertions for the obsolete claims.

## Sources inspected

- `src/input/linux.rs`, `src/input/key.rs`, `src/input/mod.rs`
- `src/fishing/mod.rs`, `src/potion/mod.rs`
- `src/pixelbus/mod.rs`, `src/app/routing.rs`
- `src/weave/sequence.rs`, `src/app/strings.rs`, `src/app/mod.rs`
- Existing input, Fishing, PixelBus, app string, view-model, and documentation-policy tests
