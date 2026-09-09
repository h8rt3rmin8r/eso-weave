# Data Model: External Encounter Model

## Storage planes

1. **Catalog**: shipped reference data such as abilities and effects.
2. **Raw observations**: user-owned, append-only encounter envelopes and events.
3. **Derived analysis**: rebuildable catalog receipts, build snapshots, and metric projections.

No raw or derived encounter row belongs in `catalog.sqlite`.

## CaptureEnvelope

| Field | Meaning |
| --- | --- |
| `schema_version` | Capture contract version |
| `session_id` | Opaque local session identifier |
| `encounter_id` | Opaque identifier unique within the session |
| `source` | Addon and API metadata |
| `started_monotonic_ms` | Duration origin |
| `ended_monotonic_ms` | Duration boundary |
| `first_sequence` / `last_sequence` | Authoritative event range |
| `content_sha256` | Hash of the canonical raw event array |
| `privacy_profile` | Applied minimization profile |

## RawEvent

Every raw event has `session_id`, `encounter_id`, `sequence`,
`monotonic_ms`, `kind`, and a kind-specific `payload`.

Required kinds are `encounter-start`, `encounter-end`, `damage`, `healing`,
`effect`, `resource`, `cast`, `bar-change`, `death`, `resurrection`,
`boss-health`, `performance`, `quickslot`, and `discontinuity`.

Actor references are encounter-local opaque IDs. Ability and effect references
retain numeric source IDs even when the current catalog does not recognize them.

## Discontinuity

A discontinuity declares `missing_sequence_from`, `missing_sequence_to`, and a
reason such as `capture-overflow`, `reload`, `truncation`, or `clock-reset`.
The marker immediately follows its declared missing range. Projections spanning
the marker use quality `degraded` and expose the range.

## BuildSnapshot

A derived build snapshot records only observations needed for calculation, such
as equipped bars or resource maxima. It carries an algorithm version and source
sequence range. It is not an authority for account or character identity.

## CatalogJoinReceipt

| Field | Meaning |
| --- | --- |
| `catalog_snapshot` | Catalog evidence revision |
| `raw_content_sha256` | Immutable raw input identity |
| `known_ids` | IDs resolved in the snapshot |
| `unknown_ids` | IDs retained but unresolved |
| `joined_at` | Receipt creation time |

A new catalog can produce a new receipt. It cannot rewrite the raw event array.

## MetricProjection

Each projection records `metric_id`, `algorithm_version`, `value`, `unit`,
`first_sequence`, `last_sequence`, `quality`, and declared loss ranges.

- Observed DPS = outgoing damage divided by encounter duration in seconds.
- Observed HPS = outgoing effective healing divided by encounter duration in seconds.
- Ability damage share = outgoing damage for one ability divided by total outgoing damage.
- Effect uptime = union of clipped active intervals across actor and effect-instance keys divided by encounter duration.
- Ordered cast sequence = cast ability IDs sorted by authoritative sequence.

## ParityRoadmapEntry

Each feature-parity row records a stable capability ID, Combat Metrics evidence,
S069 baseline state, target phase, acceptance evidence, owner issue, and risk.
Allowed states are `modeled`, `fixture-proved`, `verification-required`,
`deferred`, and `out-of-scope`.

## State transitions

```text
captured raw events
  -> integrity validation
  -> catalog join receipt
  -> versioned metric projection
  -> UI presentation
  -> optional recommendation logic
```

Failure at any stage preserves the prior raw material. No derived stage may
silently promote incomplete or cross-channel evidence.
