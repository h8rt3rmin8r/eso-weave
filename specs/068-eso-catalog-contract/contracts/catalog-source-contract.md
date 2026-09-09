# Contract: Catalog Source Matrix

## Authority

`docs/project/catalog-sources.json` is the machine-readable authority for S068.
The canonical documentation explains it to contributors, but downstream code
must consume or deliberately supersede this contract rather than infer source
behavior from issue prose.

## Required top-level fields

```text
schema_version
as_of
project_facts
completeness_values
redistribution_values
source_snapshots
promotion_policy
collector_policy
categories
```

## Required category set

```text
player-skills
crafted-abilities
ability-metadata
effects-and-status
items-and-gear
item-sets
champion-skills
consumables
mundus-effects
companions-races-classes
combat-statistics
constants
localized-text
icon-references
icon-bytes
```

The contract uses 15 rows because icon references and icon bytes are distinct
content classes. A later category may be added without changing schema version,
but removing or changing the meaning of a required row is a breaking change.

## Invariants

1. Every category contains every field defined by CatalogCategory.
2. `stable_key` cannot be `index`, `luaindex`, `array-index`, or another transient position.
3. `source_ids` resolve to declared immutable evidence or a clearly marked non-recommended research source.
4. Live and PTS snapshots use different IDs and exact version tuples.
5. Recommended file sources use a 64-character lowercase SHA-256.
6. PTS promotion is explicit, reviewed, and never automatic.
7. `icon-references` may be allowed as metadata.
8. `icon-bytes` must remain prohibited until an explicit compatible grant changes both this contract and the dated changelog decision.
9. Project-created placeholders remain allowed regardless of icon source state.
10. Community code licenses never imply a data or art license.
11. A category with pending field experiments cannot be exhaustive.
12. Collector input never executes Lua and never replaces good data partially.

## Downstream compiler handoff

The compiler in #114 must require `source_snapshot_id`, source record identity,
channel, API version, locale, acquisition method, and content hash for each
ingested batch. It may merge complementary sources only while retaining field
provenance and the lower defensible completeness class.

## Downstream collector handoff

The collector in #115 may enumerate only APIs listed by the matching category.
It must emit the CollectorEnvelope, declare visibility, bound record and byte
counts, and preserve unknown observed IDs. It must not call a known-ID getter
over guessed numeric ranges and label the result exhaustive.

## Downstream icon handoff

The icon work in #116 consumes virtual paths and project placeholders. Any
resolver is optional and local-only. A cache entry records the original virtual
path, acquisition method, content hash, transformation, and local timestamp.
Neither cache bytes nor derived image bytes enter distributed artifacts without
a later explicit redistribution decision.
