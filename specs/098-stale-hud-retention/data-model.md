# Data Model: Stale HUD Retention

## Persisted UI preference

`UiPrefs.stale_retention_seconds: u16`

- Default: `120`
- Inclusive valid range: `0..=999`
- Serialized key: `ui.stale_retention_seconds`
- Missing: default, no notice
- Wrong type or out of range: default plus `InvalidValue` notice

This is user configuration. No retained values, timestamps, or causes are serialized.

## HUD presentation snapshot

One private `HudPresentation` clones the view fields derived from current evidence:

- skill cooldowns
- weapon bar
- combat
- movement
- life
- roll dodge
- world
- travel
- menu
- resources
- Ultimate
- quickslot

The entity contains rendered strings, palette roles, and meter presentation only. It has no decoded-event type, controller handle, engine handle, or input method.

## Retention state

`HudRetentionState` owns:

- `last_coherent: Option<HudPresentation>`
- `stale: Option<StaleInterval>`

`StaleInterval` owns:

- `lost_at_ms: u64`
- `cause: StaleHudCause`

The deadline is derived with saturating arithmetic from `lost_at_ms + stale_retention_seconds * 1000`. It is not stored independently.

## Loss causes

- `GameInactive`
- `RuntimeUnavailable`
- `FocusLost`
- `FocusUnavailable`
- `SignalUnavailable`

Cause priority follows the observation dependency order: runtime, then focus, then live signal or surface. The current cause can change while `lost_at_ms` remains stable.

## State transitions

| Prior state | Evidence | Result |
| --- | --- | --- |
| Empty | Coherent | Record live snapshot, no stale line |
| Empty | Loss | Existing dormant/unavailable display |
| Coherent | Covered loss, positive interval | Retain snapshot, start stale interval |
| Stale | Same or different loss before expiry | Retain values, update cause, preserve loss time |
| Stale | Coherent before expiry | Replace snapshot, cancel interval |
| Stale | At or after derived deadline | Drop snapshot and interval, show existing fallback |
| Any | Covered loss, zero interval | Drop retention and show existing fallback |
| Stale | Interval changed | Recompute from original loss time, never restart |

## Invariants

1. Presentation retention never changes authoritative game or controller state.
2. A stale snapshot exists only if a previously coherent snapshot existed.
3. One stale interval governs every retained field.
4. Expiry and zero retention remove both the retained snapshot and stale interval.
5. Construction and process restart begin with empty retention state.
