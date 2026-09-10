# Encounter Data and Metrics

S069 defines the local encounter-analysis boundary: what ESO Weave collects, how
that data remains private and reproducible, and which claims still require live
comparison. S075 implements the explicitly armed, bounded addon capture. S076
implements its hostile-data import and dedicated raw store. Metric calculation,
the desktop history interface, and recommendations remain later work.

The machine-readable authority is
[`docs/project/encounter-model.json`](https://github.com/h8rt3rmin8r/eso-weave/blob/main/docs/project/encounter-model.json).

## Ownership and storage

Encounter history belongs to the user. The implementation keeps three storage
planes distinct:

| Plane | Content | Lifecycle |
| --- | --- | --- |
| Catalog | Shipped ability, effect, item, and reference facts | Replaced by a reviewed catalog build |
| Raw observations | Encounter envelopes and source events | User-owned and append-only |
| Derived analysis | Catalog receipts, build snapshots, and metrics | User-owned, versioned, and rebuildable |

Raw and derived rows do not belong in the bundled `catalog.sqlite`. The current
Pixel Bus remains a small safety and action-observation channel and is not a bulk
encounter transport. [ESO Weave Encounter](../features/encounter-capture.md) is a
separate addon that writes one explicitly armed, bounded SavedVariables capture.
The S076 importer reads one explicitly selected file through a stable no-follow
handle, accepts only the fixed data-only table grammar, validates the complete
terminal contract, and never executes Lua.

Accepted observations become deterministic compact JSON and receive separate
source-byte and canonical-content SHA-256 hashes. The canonical bytes enter a
caller-selected schema-v1 `encounters.sqlite`, not `catalog.sqlite` or the
settings file. Raw records cannot be updated, exact canonical reimports are
idempotent, and changed content under an existing session and encounter identity
is rejected.

There is no automatic upload. Account names, character names, chat, guild, and
location are omitted by default. Actors use opaque encounter-local IDs because
stable personal identity is unnecessary for encounter metrics.

Export and import are always explicit user actions. Users control deletion by
encounter or for the complete local store. The reusable encounter API and
maintainer command expose deterministic listing, delete-one, delete-all, and a
consistent atomically published SQLite backup with a final SHA-256 receipt.
There is no automatic pruning. Corrupt, unrelated, and unsupported future stores
remain in place for user-directed recovery. Gzip remains an optional future local
storage form, not a transport or upload mechanism.

## Ordering and incomplete captures

`(session_id, sequence)` identifies and orders events. Monotonic milliseconds
measure durations. Wall-clock timestamps must not decide order.

A valid stream rejects duplicate sequences, backward monotonic time, and an
undeclared gap. A `discontinuity` event may declare the immediately preceding
missing range and its reason. Metrics spanning that range remain available as
observed values, but their quality is `degraded` and the loss range stays visible.
This prevents an incomplete capture from looking complete.

The implemented capture families are encounter boundaries, damage, healing,
effects, resources, casts, bar changes, deaths, resurrections, boss health,
performance, quickslot use, and discontinuities. Executed Lua 5.1 tests prove
the repository state machine and privacy contract. Live event completeness and
same-parse parity remain issues #129 and #131.

## Raw events and catalog knowledge

Raw events retain numeric ability and effect IDs even if the selected catalog
does not know them. A catalog join produces a receipt containing known and
unknown IDs plus the raw-content hash. When a later catalog learns an ID, a new
receipt can resolve it without changing a raw byte or invalidating provenance.

Live and PTS evidence also stays distinct. A different channel cannot silently
upgrade or reinterpret a capture.

The catalog compiler must provide stable, channel- and API-versioned identities
for abilities, effects, items, item sets, champion skills, mundus effects,
classes, races, food, traits, and enchantments. It must also support ability
aliases, effect sources, pet ownership kinds, item-set membership, and skill-line
membership. The encounter side still accepts unknown IDs when any relationship
is absent.

## Metric contract

Every metric names its algorithm version, source sequence range, unit, quality,
and declared loss. The S069 baseline covers:

- observed outgoing DPS over encounter duration;
- observed outgoing effective HPS over encounter duration;
- outgoing damage share by ability;
- effect uptime from clipped, instance-safe actor intervals;
- cast ability IDs in authoritative sequence order.

These are reproducible descriptive calculations. Observation and calculation do
not authorize input and do not depend on action automation. Later advice must
consume a named, versioned projection rather than an unversioned summary.

## Deterministic synthetic spike

The checked-in ten-second dummy encounter exercises every baseline event family,
a declared two-record loss, and ability ID `999999`, which the first dummy
catalog does not know. It deterministically produces:

| Projection | Observed result |
| --- | ---: |
| DPS | 300 damage per second |
| Effective HPS | 80 healing per second |
| Ability 100 damage share | 0.5 |
| Ability 999999 damage share | 0.5 |
| Effect 200 uptime | 0.6 |
| Cast sequence | 100, 999999, 100 |

All five results are degraded because they span the declared loss. Reordering
the JSON records does not change their sequence-based projection. A later dummy
catalog resolves `999999` while preserving the same raw-content SHA-256.

The fixture occupies exactly 4,281 raw bytes and 775 gzip bytes in this revision.
At that artificial event rate, linear estimates are 1,541,160 raw bytes or
279,000 gzip bytes per hour. One hundred identical fixtures would occupy
428,100 raw bytes or 77,500 gzip bytes. These are reproducibility receipts, not
production retention recommendations.

## Combat Metrics parity roadmap

Primary evidence is pinned to ESO API 101050, LibCombat commit
`80817e6929c7626832f9b9114d3b12bad8d642c1`, and Combat Metrics commit
`6ec1deea4ef8801800dfe88ec79b1f94d0d6303b`. The sources support the baseline
event vocabulary and raw-before-derived architecture.

The synthetic fixture proves only contract determinism. It does not prove
equivalence to Combat Metrics on a real fight. A separate verification issue
owns a same-parse live comparison, declared tolerances, representative storage
measurements, and retention guidance.

Implementation work proceeds in this order: [addon capture](https://github.com/h8rt3rmin8r/eso-weave/issues/132),
[bounded import and local persistence](https://github.com/h8rt3rmin8r/eso-weave/issues/133),
[versioned calculation](https://github.com/h8rt3rmin8r/eso-weave/issues/134),
[quality-aware UI](https://github.com/h8rt3rmin8r/eso-weave/issues/135), then
[optional recommendations](https://github.com/h8rt3rmin8r/eso-weave/issues/136).
[Live parity verification](https://github.com/h8rt3rmin8r/eso-weave/issues/131)
follows the capture, import, and calculation path. Each stage has native GitHub
dependencies so later product behavior cannot weaken capture integrity or
privacy boundaries.
