# Data Model: Privacy-Minimized Encounter Capture

## CaptureEnvelope

| Field | Type | Rule |
| --- | --- | --- |
| `schema_version` | positive integer | S075 value is 1 |
| `addon_version` | positive integer | Embedded addon contract version |
| `status` | enum | `idle`, `armed`, `capturing`, `complete`, or `partial` |
| `channel` | enum or nil | Explicit `live` or `pts` while armed or terminal |
| `privacy_profile` | string | Fixed `anonymous-local-v1` for S075 |
| `source` | table | API, game, locale, and platform provenance without personal identity |
| `session_id` | opaque string | Local time and nonce, never account or character based |
| `encounter_id` | opaque string | Unique inside the local session |
| `started_at` / `finished_at` | decimal strings | Provenance only, not ordering |
| `started_monotonic_ms` | integer | Raw game clock origin |
| `ended_monotonic_ms` | integer | Exported elapsed duration |
| `first_sequence` / `last_sequence` | integer | Stored authoritative range |
| `stored_event_count` | integer | Exact length of events |
| `omitted_event_count` | integer | Count declared by loss ranges |
| `estimated_bytes` | integer | Conservative runtime bound, not exact file size |
| `partial_reason` | enum or nil | Stable terminal reason |
| `warnings` | table | Bounded numeric counters only |
| `events` | array | Ordered raw events |

## CaptureState

```text
idle --arm--> armed --clean combat start--> capturing
armed --disarm--> idle
capturing --normal combat end--> complete
capturing --stop/deactivate/error/loss--> partial
complete|partial --clear confirm--> idle
```

An existing complete or partial envelope blocks `arm` until clear. Addon load
never transitions idle to armed or capturing without retained explicit armed
state.

## RawEvent

Every event has:

- `session_id`
- `encounter_id`
- `sequence`
- `monotonic_ms`
- `kind`
- `payload`

`sequence` is strictly increasing. `monotonic_ms` is nondecreasing elapsed time.
Payload depth is one nested table and values are integers, booleans, or stable
contract strings. No arbitrary callback string is legal.

## Event Payloads

| Kind | Minimum payload |
| --- | --- |
| `encounter-start` | `reason` |
| `encounter-end` | `reason`, `complete` |
| `damage` | result, ability ID, amount, overflow, damage and power types, local source/target actors and unit types |
| `healing` | result, ability ID, amount, overflow, local source/target actors and unit types |
| `effect` | change, ability ID, stacks, begin/end milliseconds, target actor, source/effect/ability/status types |
| `resource` | actor, power type, value, maximum, effective maximum |
| `cast` | slot and bound ability ID |
| `bar-change` | active pair and locked state |
| `death` | actor and source classification |
| `resurrection` | actor and source classification |
| `boss-health` | boss index, actor, current, maximum, and effective maximum |
| `performance` | rounded frames per second and latency milliseconds |
| `quickslot` | action, slot, and bound collectible or item ID when visible |
| `discontinuity` | missing sequence from/to and stable reason |

## ActorRegistry

The runtime registry maps a private source key to `1..4096`. It is never saved.
Numeric combat unit ID is the preferred key. Unit tags may be used internally
for tag-only APIs. Actor 0 means unavailable or beyond the actor budget.

## CaptureBudget

| Limit | Value |
| --- | ---: |
| Stored events | 100,000 |
| Reserved terminal events | 2 |
| Estimated SavedVariables bytes | 33,554,432 |
| Reserved terminal bytes | 2,048 |
| Actor registry entries | 4,096 |
| Warnings | Fixed named counters only |

Regular append stops before either reserved boundary. Omitted observations use
constant-memory range counters.

## Terminal Reasons

Stable partial reasons are `capture-overflow`, `clock-reset`, `user-stopped`,
`player-deactivated`, and `callback-failed`. Normal completion uses
`combat-ended`. A capture may contain more than one discontinuity; the terminal
partial reason reports the first integrity-degrading cause.

## Ownership

The addon owns only `EsoWeaveEncounterSaved`. S075 does not copy, import, hash,
delete from disk, or upload that value. Issue #133 owns the separate desktop raw
store and canonical content hash.
