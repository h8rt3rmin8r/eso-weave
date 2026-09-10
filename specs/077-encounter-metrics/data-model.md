# Data Model: Versioned Encounter Metrics

## EncounterProjection

| Field | Type | Rule |
| --- | --- | --- |
| `schema_version` | integer | Exactly 1 |
| `algorithm_version` | string | Exactly `s069-v1` |
| `session_id` / `encounter_id` | string | Validated raw identity |
| `channel` | enum | `live` or `pts` |
| `duration_ms` | integer | Validated elapsed `ended_monotonic_ms` |
| `first_sequence` / `last_sequence` | integer | Validated raw range |
| `raw_content_sha256` | string | Immutable S076 canonical raw identity |
| `catalog_join` | CatalogJoinReceipt | Deterministic catalog provenance |
| `observed_dps` | MetricResult | Damage per second |
| `effective_hps` | MetricResult | Healing per second |
| `ability_damage_share` | array | Sorted by numeric ID |
| `effect_uptime` | array | Sorted by numeric ID |
| `ordered_cast_sequence` | OrderedCastSequence | Sequence-ordered IDs |

## MetricResult

`metric_id`, optional numeric `value`, `unit`, `algorithm_version`, source
sequence range, quality, and exact sorted loss ranges. Value is null only when
duration is zero.

## CatalogJoinReceipt

The receipt contains catalog schema/version/semantic hash, channel, API version,
raw content SHA-256, and sorted deduplicated known/unknown positive IDs. It never
contains a timestamp, path, payload, personal identifier, or catalog prose.

## Calculation rules

1. Sort a working event reference list by authoritative sequence.
2. Derive duration from validated elapsed `ended_monotonic_ms`; the start field is
   the raw clock origin and is not in the elapsed event clock domain.
3. Select local-player outgoing damage/healing with `source_type == 1`.
4. Use saturating effective healing after overflow subtraction.
5. Merge clipped effect intervals before dividing union length by duration.
6. Carry every declared loss range into every encounter-wide result.
7. Serialize through struct field order and sorted collections for canonical bytes.

## State transition

```text
validated immutable raw + verified compatible catalog
  -> pure s069-v1 projection
  -> canonical JSON bytes
  -> atomic explicit output replacement
```

Any failure leaves both authorities and any prior output unchanged.
