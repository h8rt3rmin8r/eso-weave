# Contract: Presentation Retention

## Inputs

- current rendered HUD projection
- current game runtime, focus, heartbeat freshness, and surface evidence
- current whole-second retention preference
- current millisecond value from the existing injected monotonic clock

## Output

The projection returns either current HUD fields with no stale line, retained HUD fields with one stale line, or the existing dormant/unavailable fields with no stale line.

## Coherence

Current fields become the retained candidate only when runtime is Active, focus is Focused, freshness is Fresh, and surface is Observed. No single field refreshes the snapshot independently.

## Retention

The first covered loss stamps `lost_at_ms`. Before `lost_at_ms + retention_seconds * 1000`, the last coherent fields are returned. The stale line names the current cause and reports `floor((now_ms - lost_at_ms) / 1000)` seconds of age.

A later loss cause can update the cause text but cannot change `lost_at_ms`. A settings edit re-evaluates the same loss time and therefore cannot restart retention.

## Recovery and expiry

Coherent evidence immediately replaces the snapshot and removes the stale interval. Zero retention or a reached deadline drops retained state and returns the ordinary current fallback. Repeated projections after expiry remain ordinary and do not emit a separate event.

## Safety boundary

The contract is output-only. Retained fields cannot be read by routing, `GameState`, `WeaveEngine`, Fishing, Auto Potion, `InputEngine`, or another input-producing path. Those paths continue to receive the underlying loss immediately.
