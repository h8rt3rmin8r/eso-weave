# Contract: External Encounter Model

## Authority

`docs/project/encounter-model.json` is the machine-readable authority. This
document defines the rules its repository validator must enforce.

## Required top-level fields

- `schema_version`
- `as_of`
- `source_snapshots`
- `storage_planes`
- `capture_envelope`
- `ordering_policy`
- `loss_policy`
- `privacy_policy`
- `integrity_policy`
- `catalog_join_policy`
- `actor_policy`
- `build_snapshot_policy`
- `retention_policy`
- `recommendation_policy`
- `catalog_schema_requirements`
- `transport_policy`
- `event_kinds`
- `metrics`
- `parity_roadmap`
- `follow_up_order`
- `follow_up_issues`
- `synthetic_fixture`

## Required event kinds

```text
encounter-start
encounter-end
damage
healing
effect
resource
cast
bar-change
death
resurrection
boss-health
performance
quickslot
discontinuity
```

## Required metrics

```text
observed-dps
observed-hps
ability-damage-share
effect-uptime
ordered-cast-sequence
```

## Invariants

1. Catalog, raw observation, and derived analysis planes are distinct.
2. `(session_id, sequence)` is the event identity and ordering authority.
3. Monotonic milliseconds drive durations; wall clocks cannot order events.
4. Actor identifiers are encounter-local and opaque.
5. Names, account handles, chat, guild, and location are excluded by default.
6. Source revisions and licenses are pinned for every parity claim.
7. Duplicate sequences, undeclared gaps, and backward monotonic time fail validation.
8. A discontinuity marker declares the immediately preceding missing range.
9. Projections spanning declared loss are degraded and expose that loss.
10. Raw observations are immutable; derived records are versioned and rebuildable.
11. Unknown numeric IDs remain in raw data and appear in join receipts.
12. A later catalog resolution cannot change the raw-content hash.
13. PTS evidence cannot silently promote to the live channel.
14. Data stays local by default and has no implicit upload path.
15. Imports are bounded, atomic, schema-validated, and non-executing.
16. Pixel Bus is prohibited as the bulk encounter transport.
17. Observation and metric calculation remain independent of action automation.
18. Every metric names its algorithm, event range, unit, and quality.
19. Synthetic evidence cannot mark live parity complete.
20. Every non-complete parity row names an ordered owner issue or verification outcome.

## Fixture validation

The canonical dummy encounter must:

- contain every required event kind;
- contain one declared loss range;
- contain one unknown ability ID;
- reproduce all five required metrics deterministically;
- produce the same projection when input records are presented out of order;
- reject duplicate, undeclared-gap, and backward-time variants;
- demonstrate a later catalog receipt resolving the unknown ID without changing raw data;
- report exact raw and gzip byte counts plus labeled linear estimates.

## Downstream handoffs

The roadmap order is capture, import, calculation, UI, then recommendations.
Each handoff must name an issue and acceptance evidence. Recommendations cannot
consume unversioned projections and cannot couple observation to action execution.
