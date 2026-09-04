# Contract: Auto-potion Effective State

**Feature**: [spec.md](../spec.md) | **Date**: 2026-09-03

## Inputs

The controller consumes only established normalized facts:

- session-only requested enablement
- S041 game activity and focus
- PixelBus heartbeat availability
- application input suspension
- permitted or gated game context
- configured resource watches and thresholds
- fresh Health, Magicka, and Stamina percentages
- S042 quickslot classification and potion availability
- independent S042 cooldown state
- retry history and current monotonic time

## Ordered outcome contract

The first failing category becomes the effective state in this order:

1. request Off
2. game inactive
3. game unfocused
4. beacon unavailable
5. input suspended
6. game context gated
7. no watched resources configured
8. no watched resource has a fresh value
9. quickslot observation unavailable
10. quickslot empty or non-potion
11. potion depleted or blocked
12. potion cooldown not ready or unknown
13. retry interval active
14. no fresh watched resource at or below threshold, producing Ready
15. first deterministic low resource, producing Triggered

## Synthesis contract

Triggered authorizes exactly this sequence through the existing sink:

1. submit configured quickslot key Down
2. submit configured quickslot key Up
3. record the attempt timestamp
4. store Triggered with its cause

No other outcome authorizes any input event.

## Lifecycle contract

- Heartbeat marks beacon available.
- SignalLost marks beacon unavailable and preserves requested enablement.
- Game exit marks game inactive and clears observations through the existing S041 path.
- Focus loss, suspension, and menu gates block immediately.
- Recovery requires the established positive facts to arrive again.

## View contract

The main view displays the requested switch independently from one concise effective-state phrase. The phrase identifies the current blocker or shows Ready or `Triggered: <resource>`.

The UI does not infer safety from raw observations and does not own controller transitions.
