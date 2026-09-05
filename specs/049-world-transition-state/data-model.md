# Data Model: World Transition State

## WorldState

Runtime-only enum owned by the pixel-bus protocol:

- `Unknown`: no valid current evidence
- `Transitioning`: ESO has deactivated the player for a world transition
- `Active`: ESO has activated the player and the addon baseline is complete

Transitions:

```text
startup -> Unknown
Unknown|Active -> Transitioning (EVENT_PLAYER_DEACTIVATED)
Unknown|Transitioning -> Active (complete EVENT_PLAYER_ACTIVATED baseline)
any -> Unknown (invalid sample, signal loss, or inactive game process)
```

Duplicate state observations are idempotent.

## B22 wire value

```text
G = 0xCC
R = 0x20 Unknown | 0x80 Transitioning | 0xE0 Active
B = 255 - R
```

The marker and checksum must validate within reader tolerance before the red
payload is considered. There is no nearest-value fallback.

B22 exists only in negotiated-header protocol version 2. Protocol version 1
retains its 22-cell payload extent and never samples a B22 candidate.

## GameObservations.world

The shared runtime snapshot stores `WorldState`. It defaults to Unknown, receives
reader transitions, and is reset to Unknown on signal loss or non-Active process
runtime. It is never serialized.

## WorldStateView

Derived each frame for System and State:

| State | Text | Role |
| --- | --- | --- |
| Unknown | Not detected | Muted |
| Transitioning | Transitioning | Warning |
| Active | Active | Healthy |
| Inactive game override | Game not active | Muted |
