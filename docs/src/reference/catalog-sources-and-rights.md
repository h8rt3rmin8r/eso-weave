# ESO Catalog Sources and Rights

This page defines which ESO data sources future catalog work may use, how much
coverage each source can honestly claim, and what ESO Weave may distribute. The
machine-readable authority is
[`docs/project/catalog-sources.json`](https://github.com/h8rt3rmin8r/eso-weave/blob/main/docs/project/catalog-sources.json).

## Current source snapshot

S068 captured two independent API channels on 2026-09-09:

| Channel | Game source | API | Stock UI commit | API documentation SHA-256 |
| --- | --- | ---: | --- | --- |
| Live | 12.0.8 | 101050 | [`f76cf16`](https://github.com/esoui/esoui/commit/f76cf16c4e5be7b234d15dc7f676febffa64c5bb) | `f9faa484d8f2d875d00ab78b32e9e236e06922101b25e356f2101591db2e3d83` |
| PTS | 12.1.4 | 101051 | [`1baf113`](https://github.com/esoui/esoui/commit/1baf1131560c2bcd38ffd2bd070728273b25f934) | `baf4e5173ce9ac478e76a5b81820b55bfe6525dfa45830b4dbc706916ba48055` |

The hashes cover the raw `ESOUIDocumentation.txt` bytes at the named commits.
The moving `live` and `pts` branches are freshness signals, not reproducibility
anchors. ESOUI forum attachments may require site access, so the immutable stock
UI repository is the offline-capable technical reference. ESO Weave does not
work around access controls.

## Source hierarchy

Use sources in this order:

1. ZeniMax-published API documentation and immutable stock UI source establish
   public function signatures, constants, iterators, and stock usage patterns.
2. A purpose-built ESO Weave collector may record only public API results and
   must label channel, API version, locale, account, character, unlock, and
   encounter boundaries.
3. Community projects may inform design when their exact code revision and
   license are recorded. Their code license does not license their collected
   game data or art.
4. Installed-client archive access is an optional, user-local research path. It
   is not the default updater and produces no redistributable output.
5. A remote source is unusable until its terms, authentication, quotas, rate
   limits, hashes, takedown behavior, and offline fallback are documented.

Pinned reference implementations include
[`uesp/uesp-esoapps`](https://github.com/uesp/uesp-esoapps/tree/875e8e9acd484134f8b7dfe38db1de2afd9f064b),
[`uesp/uesp-esolog`](https://github.com/uesp/uesp-esolog/tree/7e9e32d36b8255f0e9ff04801966d84a72e4f5bc),
and
[`Solinur/CombatMetrics`](https://github.com/Solinur/CombatMetrics/tree/6ec1deea4ef8801800dfe88ec79b1f94d0d6303b).
They are design evidence only. UESP's repository license metadata and README
also disagree for `uesp-esoapps`, and no compatible license for its collected
records or art was established.

## Completeness vocabulary

| Class | Meaning |
| --- | --- |
| Exhaustive | Complete only for the exact named category and version tuple, backed by an enumerator and count or relationship checks. |
| Bounded | Complete only inside a declared client, account, character, unlock, locale, or channel view. |
| Opportunistic | Learned only from a known ID, link, inventory item, combat event, effect, or other observation. |
| Unknown | No defensible enumeration or visibility boundary has been proven. |

An iterator index is never durable identity. Store the ID returned by the API or
an explicit composite built from stable IDs. A category awaiting field evidence
cannot be called exhaustive.

## Category contract

| Category | Stable identity | Discovery | Visibility | Claim | Distributed form |
| --- | --- | --- | --- | --- | --- |
| Player skills, ranks, morphs, class mastery, passives, ultimates | Ability ID with skill-line and progression IDs | API iterator | Character, class, unlock, locale, channel | Bounded | Stable IDs, relationships, numeric facts |
| Crafted abilities and scripts | Crafted ability ID plus script ID | API iterator | Account, unlock, locale, channel | Bounded | Stable IDs, relationships, numeric facts |
| Ability mechanics | Ability ID plus rank and caster context | Known-ID lookup | Known ID, character, locale, channel | Opportunistic | Normalized numeric facts |
| Buffs, debuffs, status, and aliases | Ability ID plus effect and status types | Combat and effect events | Encounter, character, locale, channel | Opportunistic | Normalized IDs and types |
| Items and gear variants | Item ID plus normalized link tuple | Item link | Inventory, observed link, locale, channel | Opportunistic | Stable IDs and normalized facts |
| Item sets and collection pieces | Set ID plus piece ID | API iterator | Account, unlock, locale, channel | Bounded | Stable IDs and relationships |
| Champion skills | Champion skill ID plus discipline ID | API iterator | Account, unlock, locale, channel | Bounded | Stable IDs and relationships |
| Food, drink, potions, and poisons | Item-link tuple plus effect ability ID | Item link | Inventory, observed link, locale, channel | Opportunistic | Stable IDs and normalized facts |
| Mundus effects | Ability ID plus boon relationship | Effect observation | Character, active effect, locale, channel | Opportunistic | Stable IDs and normalized facts |
| Companions, races, and classes | Typed class, race, collectible, or companion ID | API iterator | Account, character, unlock, locale, channel | Bounded | Stable IDs and relationships |
| Combat statistics | Statistic type plus versioned subject scope | Known-ID lookup | Character, equipment, effects, channel | Bounded | Numeric observations and formula provenance |
| API constants | Constant family plus name | Documentation file | API version and channel | Exhaustive | Names, numeric values, and aliases |
| Localized names and descriptions | Entity type, stable ID, and locale | Known-ID lookup | Locale plus source visibility | Bounded | User-generated local records only |
| Virtual icon references | Entity type, stable ID, and texture path | Known-ID or item-link lookup | Known or observed entity and channel | Opportunistic | Path metadata only |
| Icon image bytes | Source content hash plus transformation ID | User-local file | Installed client or user-supplied directory | Unknown | Not distributed |

The exact API entry points, failure modes, and validation rules live in the
machine-readable contract.

## Live and PTS promotion

PTS records remain PTS records. Promotion is never automatic. A reviewed
promotion receipt must name the target live API version, immutable live source
revision, source hash, normalized content hash, reviewer, time, and decision.
When live bytes differ, build a new live snapshot. Equal normalized content may
be recognized by a receipt, but the original channel provenance remains intact.

## Graphics and redistribution boundary

ESO Weave is an individual, open-source, educational project with no
monetization, sponsorship, or telemetry. Those facts resolve commercial and
project-data-collection questions for the present scope. They do not grant a
license to redistribute third-party graphics.

The current project decision is:

- Stable IDs, normalized numeric facts, provenance, virtual texture paths, and
  project-created placeholders may accompany ESO Weave.
- Verbatim localized descriptions and user-collected records stay in the user's
  local data unless a later compatible grant authorizes distribution.
- ZeniMax icon bytes, extracted icon packs, and community data without a clear
  data license do not enter source control, fixtures, binaries, documentation
  assets, caches shipped with the app, or releases.
- A future resolver may read user-supplied files or another reviewed local
  source. Acquired and transformed bytes remain in a user-local cache.
- Missing or unlicensed art falls back to a generic project-created placeholder
  and never blocks catalog, schema, or interface work.

This is a project source-selection decision, not legal advice. Revisit it if the
project adds monetization, sponsorship, telemetry, a hosted asset service, or
bundled third-party art, or if written permission supplies a compatible grant.

## SavedVariables collector boundary

Later collector work accepts only a restricted, versioned data envelope. It
never executes SavedVariables as Lua. The envelope records game and API version,
channel, locale, platform, megaserver when material, pseudonymous character
scope, acquisition time, content hash, and bounded declarative records.

Imports must check a byte limit before parsing, enforce record and string limits,
reject functions and computed expressions, validate the full content hash, and
commit atomically. Interrupted, partial, malformed, oversized, or unsupported
snapshots leave the last known-good data unchanged. User character and encounter
data stays separate and is never uploaded by default.

The initial defensive limits are 64 MiB per snapshot, 500,000 records, and 64
KiB per string. They are provisional until issue
[#129](https://github.com/h8rt3rmin8r/eso-weave/issues/129) records real size,
serialization, stall, corruption, and logout or `/reloadui` flush evidence.

## Implementation handoff

- The compiler in [#114](https://github.com/h8rt3rmin8r/eso-weave/issues/114)
  must retain source snapshot, record identity, channel, API version, locale,
  acquisition method, field provenance, and content hash.
- The collector in [#115](https://github.com/h8rt3rmin8r/eso-weave/issues/115)
  may call only documented iterators and lookups, preserves unknown observed
  IDs, and cannot turn guessed ID ranges into an exhaustive claim.
- The icon work in [#116](https://github.com/h8rt3rmin8r/eso-weave/issues/116)
  consumes virtual paths and placeholders first. Any asset resolution remains
  optional, reviewed, and local-only.

Field experiments in #129 may narrow coverage or adjust provisional limits. They
do not block these implementation contracts and cannot silently rewrite them.
