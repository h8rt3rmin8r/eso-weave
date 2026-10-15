# S131 official parser-resource research

Date: 2026-10-15. Research only for #296 and #297. No downloaded source was
executed, no account credentials were used, and no game settings were changed.
The requested normal/inline and verbose profiles remain in scope.

## Recovered public official CDN source

An ordinary anonymous GET to
[assets.rpglogs.com/js/parser-eso.js](https://assets.rpglogs.com/js/parser-eso.js)
returned HTTP 200 with `application/javascript` and 39,617 bytes.

| Property | Observed value |
| --- | --- |
| SHA-256 | `07e90fd8227ff9b4d262254a86103eecddb984ef45b31e6b9dd8f03b8996c1d7` |
| Last-Modified | `2021-04-22T16:11:00Z` |
| ETag | `W/"749591c4b29d3a4bb2489a1cfa640a0f"` |
| Local inert evidence | `target/s131-research/parser-eso.js` |
| Encoding | Valid UTF-8 without BOM; no replacement characters |

The source is hosted by the official uploader's asset domain, but its 2021
modification date makes it historical evidence. It has not been established as
the resource used by the current uploader. The source exposes `parseLogLine`,
`resolveESOUnit`, `resolveESOAbility`, `collectEventTypeInformation`,
`parseLocalizedNames` and separate unit/ability tables.

Static readability analysis treated the quoted string table as data. Its 125
literal strings and fixed rotation check identify a unique rotation of 51;
numeric dictionary references can therefore be read without executing the
downloaded program. No `eval`, JavaScript VM, iframe or browser execution was
used.

Concrete findings in `parseLogLine`:

- It initially splits the line on commas, reads the first field as an integer
  timestamp and the second as the positional record kind.
- BEGIN_LOG reads the native version from field 3 and epoch from field 2. The
  version is updated to the maximum observed value, with no format-profile
  declaration at that location.
- ABILITY_INFO requires at least seven fields, reads an integer ID from field
  2, then name, icon and two boolean fields, and populates `esoAbilitiesTable`.
- UNIT_ADDED and UNIT_CHANGED decode positional fields with version-dependent
  branches, including a version 11 layout change.
- The source contains no `verbose`, `inline`, `lineType`, `offsetMS` or
  `timeSinceEpochMS` label. Its observed positional parsing and separate
  ability-table lookups do not establish the missing profiles.

Consequently, recovering this file does not establish version 15 inline or
verbose grammar. It must not be promoted into an all-profile parser contract.

## Provenance of the public CDN lead

[thevinter/combatlog at 2b4668ef55e810ed291e8367dccfe3a89a10ff0d](https://github.com/thevinter/combatlog/blob/2b4668ef55e810ed291e8367dccfe3a89a10ff0d/wcl-upload.py)
explicitly maps the ESO game to `https://www.esologs.com` and parser slug `eso`
at lines 20-27. Its `fetch_parser_code` at lines 172-197 extracts an official
`assets.rpglogs.com/js/...parser-<slug>...` script URL from the parser page.
The bare public CDN path above was derived from that published convention.
The third-party downloader was read as text only and was not run. Its mapping
is a resource-discovery lead, not authoritative evidence for native grammar.

The same pinned repository's
[parser-harness.js](https://github.com/thevinter/combatlog/blob/2b4668ef55e810ed291e8367dccfe3a89a10ff0d/parser-harness.js)
describes legacy global-function and newer class-based parser generations.
Its public tree contains a harness and transport code, but no committed ESO
parser cache. The related candidate path
`https://assets.rpglogs.com/js/log-parsers/parser-eso.js` was also checked and
returned HTTP 403 XML AccessDenied; this is not evidence that no versioned
resource exists.

## Current official uploader resource

The surface research agent passively extracted official uploader release
[v9.3.65](https://github.com/RPGLogs/Uploaders-esologs/releases/tag/v9.3.65).
Its `js/app.js` has SHA-256
`6b89d58db26424394137a6ce0882063f8364fbf72ba7d4efab2e9a4d687de99e`
and renders a parser iframe using this route:

```text
{baseUrl}/desktop-client/parser?id={id}&ts={loadTimestamp}&gameContentDetectionEnabled={bool}&metersEnabled={bool}&liveFightDataEnabled={bool}
```

This research independently read that exact source text. The uploader's normal
processing instance uses ID 1. Parser rendering is conditional on the user/login
state. Parser version is obtained through IPC, rather than a suffix appended to
the resource URL. The surface agent identified published ESO origins
`https://www.esologs.com` and `https://cn.esologs.com`.

Anonymous read-only requests on both origins, using ID 1, timestamp
`1792077298256` and all three feature flags `false`, returned:

| Origin | Status | Body | Body SHA-256 |
| --- | --- | --- | --- |
| `https://www.esologs.com` | HTTP 401 | 0 bytes, `text/html; charset=utf-8` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `https://cn.esologs.com` | HTTP 401 | 0 bytes, `text/html; charset=utf-8` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |

No parser source was recovered from those responses. The timestamp is a cache
refresh input, not a claimed parser revision. No authenticated session,
identity variation or challenge bypass was used.

Historical confirmation from the surface agent: the official v8.5.25 package's
`js/app.js`, SHA-256
`8345d35e97f3e95f924801e1e0df17eae4033f82b6f687fa4610b2db39ff22e7`,
already uses `/desktop-client/parser` with the ID, timestamp, meters and live
fight flags. The current uploader adds the game-content flag. The old public
Warcraft fork's `/client/parser` reference therefore predates this official
2024 route and cannot identify the current ESO source by itself.

## Release history and public caches

- [Official release repository tree 721c8705c9424e8b23729545d8ef0f40097c7ed1](https://github.com/RPGLogs/Uploaders-esologs/tree/721c8705c9424e8b23729545d8ef0f40097c7ed1)
  contains one 19-byte README, with no source or parser cache.
- Public release metadata covered 149 releases, from v4.6.1 through v9.3.65.
  Release bodies were empty. No asset name matched parser, JavaScript, JSON or
  ASAR; distributable packages and update metadata remain separate evidence.
- [Public Flatpak manifest 1e0f5176f769e6de100b39ae12cc02657873d2fc](https://github.com/zastrixarundell/com.esologs.Esologs/blob/1e0f5176f769e6de100b39ae12cc02657873d2fc/com.esologs.Esologs.yml)
  pins the official v8.5.25 AppImage with SHA-256
  `0d3d526bd58c6266b9210d2c51bdb14d0e83b9fb8aae68a1b41e0584b5931db5`.
  It exposes distribution provenance and an official download-page matcher,
  without a cached parser body.
- The previously inspected TylerGrayStudent/WarcraftLogsUploader repository
  returned no public forks from the bounded GitHub forks endpoint.
- Exact public code searches for `esologs.com/client/parser`, `parser-eso`,
  `esologs.com` plus `log-parsers`, and `esologs` in `parser.js` yielded no
  committed actual parser cache. A later query for the exact native serializer
  spelling `grantsSyngeryAbilityId` also returned no indexed file.
- Searching `/desktop-client/parser` identified transport implementations in
  other game uploaders, including pinned FF Logs downloaders. Their parser-FF
  script matching does not establish ESO serialization.
- The public CDN's `mix-manifest.json` was readable: 247 bytes, SHA-256
  `3b188d5f5a117090dbbade39cf35077919aeef8fb93070400782782cec092f78`.
  It lists app, manifest, CSS and vendor resources, with no parser resource.
  The listed manifest JavaScript (1,493 bytes) and app JavaScript (113,151
  bytes) also have April 2021 Last-Modified dates. Their static text supplied
  no current parser asset URL. The same manifest path and homepage on
  `www.esologs.com` returned HTTP 403 Cloudflare responses.

## Legitimate archive checks

- [Common Crawl collection inventory](https://index.commoncrawl.org/collinfo.json)
  was readable and identified the September 2026 index as
  `CC-MAIN-2026-39`.
- Queries for `esologs.com/client/parser*` in `CC-MAIN-2026-39` and
  `CC-MAIN-2024-51` each returned HTTP 404 with `No Captures found`.
- The equivalent `CC-MAIN-2021-49` query returned HTTP 502. A September 2026
  query for `assets.rpglogs.com/js/*parser-eso*` returned HTTP 504. These failed
  requests are not counted as evidence of no historical capture.
- Wayback's CDX request returned HTTP 400, and its public availability API
  returned HTTP 429 for both ESO host spellings. These access failures provide
  no archive-coverage conclusion.
- An exact CDN parser CDX query also returned HTTP 400. Prefix queries for
  `assets.rpglogs.com/js/parser-eso` returned HTTP 502 in September 2026 and
  HTTP 504 in December 2024. These are failed lookups, not negative captures.

The recovered historical CDN parser is useful, concrete progress. Exact
version 15 inline/verbose producer grammar is still unestablished by this
resource investigation; no guessed grammar or format implementation was added.
