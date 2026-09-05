# Contract: B21 Life State and Input Gate

## Pixel block

- Logical payload index: B21
- Physical position: negotiated layout payload point 21
- Red: life-state payload
- Green: dedicated life marker, shared exactly with the companion
- Blue: `255 - red`
- Alpha: 1

Valid payloads are Alive `0x20`, Dead `0x80`, and Reincarnating `0xE0`. All other
payloads decode to Unknown even when marker and checksum pass.

## Addon authority

`computeLifeState()` applies:

1. if `IsUnitReincarnating("player")`, Reincarnating;
2. else if `IsUnitDead("player")`, Dead;
3. else Alive.

It is called during initial block construction, on player dead and alive events,
after player activation, and on the one-second convergence tick.

## Companion transitions

```text
valid B21 change -> PixelBusEvent::Life(state) -> all four synthesis consumers
invalid B21      -> PixelBusEvent::Life(Unknown) -> all four consumers gated
signal loss      -> SignalLost plus Life(Unknown) -> all four consumers gated
```

Heartbeat does not alter the life gate. Only a valid B21 Alive event opens it.

## Synthesis rule

```text
may_synthesize = game_active
              && focused
              && fresh_beacon
              && !suspended
              && gameplay_surface
              && life_state == Alive
```

Existing paths may enforce additional feature-specific conditions. None may
weaken the life term.

## Recovery rule

- Input: a new physical down transition may be intercepted only after Alive.
- Weave: queued actions observed while gated are dropped.
- Fishing: blocked pending autonomous work is canceled, not deferred.
- Auto-potion: a future tick may act only on current readings and retry state.

