# GitHub Issue Snapshot: #171

**Title**: `[Feature]: Retain the last observed HUD state briefly after focus or signal loss`

**URL**: https://github.com/h8rt3rmin8r/eso-weave/issues/171

**Captured**: 2026-09-15

## Outcome

Keep the most recently observed player-state values visible for a configurable period after ESO disconnects, becomes inactive, loses focus, or the live signal disappears, instead of wiping the HUD immediately.

## Context

Transient focus and connection changes currently erase useful context as soon as the event is observed. A player switching to ESO Weave should still be able to inspect the state the application just saw.

This is a presentation-retention change. Automation and synthesized-input gates must continue to react immediately to focus, runtime, context, and signal loss. Retained stale values must never be treated as fresh evidence that authorizes an action.

## Scope

- Add a persisted **Stale retention** setting expressed in whole seconds.
- Default to `120` seconds when no saved value exists.
- Accept `0` through `999` seconds inclusive. `0` preserves immediate clearing.
- Present the setting as either a slider or a numeric field with vertically stacked increment/decrement tick buttons.
- On game inactivity/disconnection, focus loss, or signal loss, retain the last coherent HUD values until fresh observations arrive or the window expires.
- Clearly distinguish retained values from live values and expose their stale cause/age without replacing the values themselves.
- When the timer expires, transition to the existing unavailable/dormant presentation.
- Closing ESO Weave does not preserve or restore live player-state values; process exit clears them immediately by definition.

## Acceptance Criteria

- [ ] A missing setting loads as 120 seconds and the saved value round-trips across restarts.
- [ ] The UI control cannot produce a value below 0 or above 999 seconds and supports precise keyboard entry/adjustment.
- [ ] With retention set above 0, the latest coherent HUD snapshot remains visible after each covered loss event until the deadline.
- [ ] The retained presentation identifies itself as stale and communicates the loss reason and age accessibly.
- [ ] A fresh observation before expiry replaces the stale snapshot and cancels the pending wipe.
- [ ] Expiry clears the stale values exactly once and presents the existing truthful unavailable/dormant states.
- [ ] A value of 0 clears immediately.
- [ ] Auto Potion, fishing, weaving, and every other input-producing path block immediately on the underlying loss event regardless of the retention setting.
- [ ] Unit/integration tests cover the default, bounds, each loss cause, recovery before expiry, expiry, and separation from automation authorization.
- [ ] Settings and interface documentation describe the behavior directly.

## Dependency and Verification

No known dependency. Reuse one monotonic freshness/deadline model rather than adding independent timers per widget. Repository tests prove state transitions, persistence, bounds, and automation separation; a manual UI pass confirms the chosen control and stale-state accessibility.
