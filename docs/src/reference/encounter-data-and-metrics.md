# Encounter Data and Metrics

S069 defines the local encounter-analysis boundary: what ESO Weave collects, how
that data remains private and reproducible, and which claims still require live
comparison. S075 implements the explicitly armed, bounded addon capture. S076
implements its hostile-data import and dedicated raw store. S077 implements
versioned metric projection and catalog reconciliation. S078 implements the
desktop history interface. S090 adds provisional, evidence-scoped encounter
review prompts.

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

S077 reads one validated raw record and one schema-verified catalog, then writes
an explicit canonical schema-v1 JSON projection outside both SQLite authorities.
The file is disposable, atomically replaceable, and rebuildable. Its provenance
contains raw and catalog hashes and versions rather than a creation timestamp, so
equal inputs produce byte-identical output. A future derived database remains a
history-UI decision rather than an extension of the raw store.

S078 gives the desktop one private Encounter History window. UI imports go to
`encounters/encounters.sqlite` beneath the per-user application root. The Import
Current Capture action derives the fixed `SavedVariables/EsoWeaveEncounter.lua`
source and expected channel from the explicitly selected Live or PTS AddOns
environment. It performs no arbitrary scan and uses the same bounded,
non-executing S076 import contract.

Opening history does not create a missing store. Raw summaries remain visible
when the active catalog is missing, invalid, or incompatible. Selecting one
summary rebuilds its S077 projection in memory against the current catalog path
and displays algorithm and catalog versions, hashes, complete or degraded
quality, exact loss ranges, known coverage, and every unknown numeric ID. All
values are labeled observed, and unavailable denominators remain unavailable
rather than becoming zero. Import, listing, calculation, and deletion run on a
serialized background worker so large captures do not block the GUI.

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

The desktop provides separately confirmed Delete Encounter and Delete All
actions. Canceling or closing a confirmation performs no mutation. These actions
remove only raw encounter records and do not affect settings, catalogs, addons,
or action automation.

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

Algorithm `s069-v1` is implemented over S076 capture schema v1. Duration is the
validated elapsed `ended_monotonic_ms`; `started_monotonic_ms` is a raw clock
origin and is not subtracted from it. Outgoing damage and healing require combat
source type 1, the local player, rather than assuming actor 1. Actor allocation is
observation-order dependent. Pets remain excluded until raw evidence can model
ownership without guessing. Effective healing saturates `amount - overflow`.

The lowercase algorithm value is a persisted runtime identifier, not a
work-slice reference.

Effect records retain duration but not the absolute clock origin or effect-slot
identity. The v1 algorithm anchors each nonnegative `end_ms - begin_ms` duration
at the event's elapsed monotonic time, clips to encounter bounds, and unions
overlap by numeric ID. Invalid intervals contribute zero. This is deterministic
repository behavior, not a live-semantic parity claim. Zero-duration encounters
report rate and uptime values as unavailable, and encounters with no outgoing
damage report an empty share collection.

The explicit maintainer operation is:

```bash
catalog-compiler encounter-project \
  --store PATH --catalog PATH --output PATH \
  --session ID --encounter ID
```

The selected catalog must exactly match capture channel and API version. Ability
references join as abilities, while effect callback IDs may resolve as catalog
effects or abilities. The receipt exposes sorted known and unknown positive IDs,
catalog semantic identity, and immutable raw-content identity. Rebuilding with a
later compatible catalog can resolve an unknown ID without changing raw bytes or
metric values.

## Provisional recommendations

S090 adds a separate local `s090-v1` recommendation policy after S077 metric
projection. It consumes one immutable in-memory projection, reopens no catalog or
raw file, performs no network or model call, and writes no recommendation file,
database row, setting, log, or telemetry event. A catalog replacement discards the
selected detail and rebuilds its facts and prompts together.

The Encounter History window always renders Observed Metrics before Provisional
Recommendations. A prompt is a deterministic question for review, not a claim of
cause, an optimal build or rotation, a guaranteed improvement, or Combat Metrics
parity. The first policy has two bounded rules:

- identify at most one known ability contributing at least 40 percent of observed
  outgoing ability damage, then ask whether that concentration matches the user's
  intended encounter context;
- identify at most one known effect with observed uptime at or below 50 percent,
  then ask whether the observed gaps match the user's intended uptime.

The policy generates no more than two prompts. Higher damage share wins the first
rule, lower uptime wins the second, and ascending numeric ID breaks equal values.
Invalid, unavailable, non-finite, negative, or greater-than-one ratios produce no
prompt. Numeric IDs remain explicit because the projection carries catalog
identity but not localized names.

Evidence gates are versioned policy thresholds rather than ESO performance
targets. The first policy accepts only projection schema 1 and the `s069-v1`
metric algorithm. It also requires each recommendation-bearing metric to carry a
matching algorithm, source range, and internally consistent quality plus exact
loss evidence. Unsupported versions, contradictory quality evidence, and
reversed loss ranges fail closed.

| Evidence | `s090-v1` result |
| --- | --- |
| Projection is not schema 1 with `s069-v1` metrics | Suppress all advice |
| Metric quality, coverage, algorithm, or a loss range is inconsistent | Suppress all advice |
| Duration below 10 seconds | Suppress all advice |
| Fewer than three observed casts | Suppress all advice |
| Declared loss below 10 percent of the inclusive sequence span | Qualify retained advice and show the exact loss |
| Declared loss at or above 10 percent | Suppress all advice |
| Any unknown positive ID | Qualify known-target advice and omit unknown targets |
| Unknown abilities own at least 25 percent of observed damage | Suppress only damage-concentration advice |

Unknown IDs from unrelated entity families do not erase a valid known-effect
prompt. This rule-local treatment prevents incomplete catalog knowledge from
becoming a false global blocker. A suppressed report still shows the complete
observed metrics and its exact evidence reasons. A qualified prompt repeats each
applicable qualification next to the prompt.

Every prompt cites recommendation schema and policy, encounter and session IDs,
raw-content SHA-256, projection schema, metric algorithm, catalog schema and
version, catalog semantic SHA-256, channel, and API version. These identities make
the prompt reproducible without treating it as stored truth.

Recommendations have no apply or execute control and cannot enqueue, synthesize,
or authorize input. Weaving, Fishing, Auto Potion, Pixel Bus, game focus, addon
lifecycle, catalogs, and raw encounter storage do not consume recommendation
output. Live comparison in issue #131 may justify a later policy version, but it
does not block `s090-v1` implementation or use.

## Deterministic synthetic spike

The checked-in ten-second S069 dummy encounter exercises every baseline event family,
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

S077 adds an actual capture-schema fixture that reproduces the same five headline
values through the production raw-store, catalog-reader, projection, and atomic
publication boundaries. It also proves degraded loss evidence, actor-order-safe
player attribution, exact catalog compatibility, unavailable zero-duration rates,
output no-clobber, and later resolution of ID `999999`. S076 requires raw event
arrays to already be in authoritative sequence order, so S077 consumes that
stricter validated form rather than accepting the S069 spike's provisional
presentation-order variant.

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
