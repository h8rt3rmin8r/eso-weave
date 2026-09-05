# Contract: B22 World Transition State

## Addon publication

PixelBeacon version 16 publishes negotiated-header protocol version 2 and payload
block B22 after B21 life state.

```text
marker green: 0xCC
Unknown:       (0x20, 0xCC, 0xDF)
Transitioning: (0x80, 0xCC, 0x7F)
Active:        (0xE0, 0xCC, 0x1F)
```

The addon begins Unknown. `EVENT_PLAYER_DEACTIVATED` writes Transitioning
immediately. `EVENT_PLAYER_ACTIVATED` calls the complete player-state baseline,
then writes Active. No timer writes Active.

## Companion decoding

The reader accepts a state only when:

1. green matches `0xCC` within configured tolerance;
2. `abs((R + B) - 255)` is within tolerance; and
3. red matches exactly one documented payload within tolerance.

Otherwise it returns `WorldState::Unknown`. A missing B22 sample follows the same
rule. Signal loss changes a previously non-Unknown value to Unknown exactly once.

## Event and routing contract

`PixelBusEvent::World(WorldState)` is emitted only when normalized state changes.
The event updates `GameObservations.world`. `SignalLost` and inactive game runtime
also clear the stored state to Unknown.

## Presentation contract

System and State adds a World state row after the Game summary. The row uses
Active, Transitioning, or Not detected while the ESO process is active. Process
inactivity overrides all three with Game not active.

## Compatibility

Versions 14 and 15 retain negotiated-header protocol version 1 (`0x20`) and omit
B22. Their geometry, heartbeat, and earlier blocks remain readable, while the
reader suppresses B22 sampling and world state stays Unknown. This prevents an
ordinary screen pixel just beyond the older overlay from impersonating B22.
Pre-version-14 heartbeat layouts remain on the existing legacy path. The managed
addon status exposes the normal Update action because the embedded manifest
advances to version 16.

## Exclusions

This contract does not authorize synthesis, infer pre-loading travel, or replace
the existing life gate.
