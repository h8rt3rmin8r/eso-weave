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
- `original_deadline_ms: u64`
- `cause: StaleHudCause`

The shared Game State records `presentation_loss_at_ms` at the first coherent-to-incoherent observation transition. It stores no rendered values and does not change action authority. When App Model first projects that loss, the stale interval copies the transition time and stores the saturating original deadline. Later projections use the lesser of that deadline and `lost_at_ms + current stale_retention_seconds * 1000`, so a smaller value can shorten retention but a larger value cannot extend it.

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
| Stale | Interval changed | Recompute from original loss time, capped at original deadline |

## Invariants

1. Presentation retention never changes authoritative game or controller state.
2. A stale snapshot exists only if a previously coherent snapshot existed.
3. One stale interval governs every retained field.
4. Expiry and zero retention remove both the retained snapshot and stale interval.
5. Construction and process restart begin with empty retention state.
