# Research: Versioned Encounter Metrics

## Existing authority

S069 defines three distinct planes and five baseline projections. S075 emits
bounded privacy-minimized captures. S076 validates terminal truth and stores
canonical raw bytes immutably. S077 starts at `load_encounter`; it does not parse
SavedVariables again or normalize raw rows into a second authority.

## Player attribution

Actor identifiers cannot identify the player because the addon allocates them in
observation order. The pinned ESO API calls the local player combat source type
`COMBAT_UNIT_TYPE_PLAYER`; its numeric value is 1, and S075 retains source type.

Decision: v1 outgoing damage and healing require `source_type == 1`. Pets are
excluded because capture schema v1 has no stable ownership relation. Actor ID 1
is never treated as special.

## Effective healing

S075 retains callback amount and overflow. Effective amount is
`amount - min(amount, overflow)`. Saturating arithmetic handles defensive edge
cases without changing the raw fact.

## Effect intervals

S075 records encounter-relative monotonic event time plus API begin/end
milliseconds, but not the absolute clock origin or effect-slot identity. Treating
API begin/end as encounter-relative would be false.

Decision: derive nonnegative duration from `end_ms - begin_ms`, anchor it at event
monotonic time, clip it to encounter duration, group by `(target_actor, ability_id)`,
merge overlaps, then union target intervals by ID. Invalid or zero duration is
ignored. Live semantic parity remains issue #131.

## Quality and zero denominators

All baseline metrics cover the complete encounter sequence range. Event times and
`ended_monotonic_ms` are elapsed from zero, while `started_monotonic_ms` retains
the raw clock origin. Duration is therefore the validated elapsed end value, not
a subtraction across clock domains. Any declared loss degrades every result and
remains embedded in it. Zero duration cannot yield a rate or uptime, so those
numeric values are null. Cast order remains available. An empty damage total
produces an empty share collection.

## Catalog joins

The catalog reader verifies schema, integrity, and semantic SHA-256 on open.
Capture channel and API version are exact evidence boundaries.

Decision: require exact channel and API equality. Ability-bearing events resolve
as catalog abilities. Effect IDs resolve as either effect or ability because ESO
effect callbacks use ability IDs while the catalog can model either concept.
Receipt lists are sorted and deduplicated.

## Derived persistence

The upcoming history UI, not S077, owns indexing, retention, and query needs for
derived records. Adding tables now would guess that contract.

Decision: atomically publish one explicit canonical JSON projection. The output
is disposable and reproducible from raw plus catalog. Wall-clock creation time is
omitted so identical inputs are byte-identical.

## Reused boundaries

- S076 `load_encounter` for store validation and immutable raw retrieval.
- `canonical_bytes` and SHA-256 for raw semantic identity.
- `CatalogAccess` and `CatalogReader` for verified read-only catalog access.
- Existing atomic-file publication and distinct-path rules.

No new crate, network access, telemetry, game scan, or action integration is needed.
