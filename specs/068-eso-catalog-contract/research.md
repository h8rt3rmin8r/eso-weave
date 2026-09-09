# Research: ESO Catalog Source Contract

## Evidence snapshot

Research was refreshed on 2026-09-09. API 101050 is the current live channel and
API 101051 is the PTS channel. The attached ESOUI files returned 403 to an
ordinary unauthenticated fetch, so S068 did not work around that control. The
same documentation is available in immutable esoui/esoui commits and serves as
the reproducible source.

| Channel | Game source | API | Immutable commit | Documentation SHA-256 |
| --- | --- | ---: | --- | --- |
| Live | 12.0.8 | 101050 | `f76cf16c4e5be7b234d15dc7f676febffa64c5bb` | `f9faa484d8f2d875d00ab78b32e9e236e06922101b25e356f2101591db2e3d83` |
| PTS | 12.1.4 | 101051 | `1baf1131560c2bcd38ffd2bd070728273b25f934` | `baf4e5173ce9ac478e76a5b81820b55bfe6525dfa45830b4dbc706916ba48055` |

The live documentation is 1,087,138 bytes. The PTS documentation is 1,098,724
bytes. Hashes were calculated over the raw `ESOUIDocumentation.txt` bytes from
the named commits.

## API discovery findings

The pinned documentation confirms iterable surfaces for skill types, skill
lines, skill abilities, crafted abilities and scripts, champion disciplines and
skills, item-set collections, classes, and collectibles. These surfaces return
stable IDs beside iterator indexes. The indexes are traversal tools only.

Known-ID lookup surfaces include ability name, description, icon, costs, cast
shape, duration, and target text. Item-link functions describe a supplied bag
slot or item link and do not enumerate a universal item space. Combat and effect
events emit ability IDs as encountered. These distinctions force separate
bounded and opportunistic coverage classes.

The stock item-set manager calls `GetNextItemSetCollectionId` and walks pieces by
set ID. That is strong evidence for a version-bounded set collection iterator,
but whether all locked and uncollected entries are visible remains a field
verification question.

## Source-family decisions

### ZeniMax API documentation and stock UI source

Use as the primary technical authority. Pin exact commits and raw documentation
hashes. The moving `live` and `pts` branches are freshness signals only. The
repository README presents the source unchanged for reference and carries no
general content license, so availability does not authorize redistribution of
source, records, text, or art.

### Purpose-built collector

Use for public APIs that require an active client context. Exports must label
character, account, unlock, locale, channel, and encounter visibility. The
collector must emit a restricted versioned data envelope, not arbitrary Lua.
ESO writes SavedVariables at supported UI save boundaries such as logout or
`/reloadui`; a desktop reader must treat a mid-write or stale file as untrusted.

Exact size, serialization time, multi-character coverage, locale changes, and
flush behavior require game access and move to the separate verification issue.

### Community implementations

- `uesp/uesp-esoapps` commit `875e8e9acd484134f8b7dfe38db1de2afd9f064b`
  demonstrates a collector, SavedVariables, and targeted archive tooling. Its
  repository metadata reports MIT while its README also states GPL v2 unless
  otherwise stated. This conflict makes direct reuse inappropriate without
  clarification. The underlying game records have no proven compatible data
  license.
- `uesp/uesp-esolog` commit `7e9e32d36b8255f0e9ff04801966d84a72e4f5bc`
  demonstrates server-side schemas and category-specific exports. Its MIT code
  license does not license collected game data for ESO Weave redistribution.
- `Solinur/CombatMetrics` commit `6ec1deea4ef8801800dfe88ec79b1f94d0d6303b`
  is Artistic-2.0 code and supports encounter-oriented design. S068 uses it only
  as a reference. S069 owns the external encounter model.

### Installed-client archive extraction

Do not use as the default. UESP documents up to 120 GB for broad extraction and
more than one million files for expanded subrecords. Targeted extraction can be
technically bounded by file or name, but permission and format stability remain
unresolved. Future work may evaluate a user-local, opt-in resolver without
redistributing its output.

### Remote or CDN sources

No anonymous bulk icon endpoint with suitable terms, quotas, stability, and
redistribution permission was established. S068 rejects scraping around access
controls. A future source must record authentication, rate limit, cache and
takedown behavior, content hash, offline fallback, and license before use.

## Rights decision

Current ZeniMax terms protect game graphics and do not clearly authorize
packaging extracted icon bytes in an external application. Add-on terms govern
user-created add-ons and do not clearly extend to an external executable's art
bundle. Accordingly:

- ESO Weave code, project-created placeholders, stable numeric IDs, provenance,
  and virtual texture paths may be distributed.
- ZeniMax icon bytes, prebuilt extracted icon packs, and unlicensed community
  data may not be distributed.
- User-supplied directories are permitted as inputs.
- A future user-local resolver can be evaluated source by source, but its output stays local.
- Commercial, sponsorship, monetization, and telemetry branches are not applicable while the current project facts remain unchanged.

## Completeness vocabulary

- **Exhaustive**: The source enumerates the entire named category for the exact version tuple, with counts or relationship checks.
- **Bounded**: The source enumerates a declared client, account, character, unlock, locale, or channel view.
- **Opportunistic**: Records appear only when observed through links, inventory, combat, effects, or another event.
- **Unknown**: Neither enumeration nor a defensible boundary has been proven.

S068 deliberately assigns bounded or opportunistic status to all gameplay
content until field experiments prove a narrower exhaustive statement.

## Promotion decision

Live and PTS snapshots never merge by default. A promotion receipt must name the
target live API, source revision, source hash, schema version, reviewer, and
review time. If PTS bytes differ from live bytes, build a new live snapshot. If
they match, the receipt may refer to the same normalized content but must still
retain distinct source provenance.

## Alternatives rejected

- **Treat a character scan as the complete game catalog**: rejected because class, unlock, locale, link, and encounter boundaries make it false.
- **Import a community database wholesale**: rejected because code licensing and data licensing are different and no compatible data grant was proven.
- **Ship extracted icons because the project is noncommercial**: rejected because project purpose does not create redistribution permission.
- **Block all downstream work until live experiments finish**: rejected because placeholder, schema, compiler, and bounded collector contracts can proceed honestly with unverified claims marked as such.
