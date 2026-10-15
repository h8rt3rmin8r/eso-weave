# S131 Official Uploader Distribution Research

Date: 2026-10-15. Scope: passive inspection of publisher release archives and
their text resources. No uploader, installer, downloaded JavaScript or extracted
application was executed. No authentication, upload or game setting was used.
All downloaded bytes and extracted resources remain under ignored
`target/s131-research/uploader/`; no third-party source is added to product code.

## Publisher provenance

The [official release API](https://api.github.com/repos/RPGLogs/Uploaders-esologs/releases/latest)
identified [v9.3.65](https://github.com/RPGLogs/Uploaders-esologs/releases/tag/v9.3.65)
as latest: release 332525550, published 2026-06-01T14:25:18Z.

| Resource | Identity and integrity |
| --- | --- |
| [Official ZIP](https://github.com/RPGLogs/Uploaders-esologs/releases/download/v9.3.65/esologs-v9.3.65.zip) | Asset 435320149; 200,967,313 bytes; SHA-256 `ddfe3052f077cbe53ece50da2eeea5e367193103fcbf0e5ba8d128185260c52e`, exactly matches the publisher API digest. |
| ZIP HTTP receipt | HTTP 200; application/octet-stream; ETag `"0x8DEBFE82106ACBC"`; Last-Modified 2026-06-01T14:14:44Z. |
| `Contents/Resources/app.asar` | 28,480,033 bytes; SHA-256 `2be7a3458f23f7aa30768b04ecc6984a361217aad041b722157c2e3d31323ed3`. |
| ASAR `js/app.js` | 6,739,036 bytes; SHA-256 `6b89d58db26424394137a6ce0882063f8364fbf72ba7d4efab2e9a4d687de99e`. |
| ASAR `package.json` | Identifies esologs / ESO Logs Uploader version 9.3.65 and Electron 37.7.0. |

The ZIP was read with .NET ZipArchive. Only app.asar was extracted. Its bounded
JSON header was parsed as data; text entries were copied by declared byte offsets
after checking each destination stayed inside the research directory. The archive
has 67 entries. The inventory contains CSS source maps, but no JavaScript source
map, native parser file or parser-cache resource.

Local receipts: `release-asset.metadata.json`, `zip-inventory.json`,
`asar-header.json`, `asar-inventory.json`, `asar-text.metadata.json`,
`source-evidence.json` and `source-locations.json`. Signed download redirect query
parameters are excluded from receipts; the public origin/path is retained.

## Actual current parser loading seam

The official bundle uses these ESO-specific facts:

- International base URL `https://www.esologs.com`; China base URL
  `https://cn.esologs.com`; ESO game/version selection `eso-live`.
- The local ESO helper splits files at positional `BEGIN_LOG` records. It is not
  the semantic encounter parser.
- The parser is a remotely loaded hidden iframe at the selected base URL plus
  `/desktop-client/parser`. Its query parameters are `id`, `ts`,
  `gameContentDetectionEnabled`, `metersEnabled` and `liveFightDataEnabled`.
- A `get-parser-version` message requests the remote parser's version. There is
  no parser-version suffix in the current endpoint construction. The application
  version is therefore not evidence of a parser version or a native log version.
- In this bundle, rendering the parser iframe requires both a user object and a
  selected game service. The independent anonymous endpoint result is documented
  by the parallel parser-resource investigation; this UI condition alone does
  not establish endpoint authorization behavior.

For reproducible static navigation, zero-based UTF-8 byte offsets in the exact
`js/app.js` above are: endpoint text 6,408,055; parser-version message 6,388,262;
local `BEGIN_LOG` helper 5,951,805. Minified symbol names are incidental. Full
byte/UTF-16 location pairs are retained in `source-locations.json`.

Searches across extracted app, main, vendor and preload JavaScript found no
`ABILITY_INFO` or `EFFECT_CHANGED` token. `BEGIN_LOG` occurs once in the ESO
file-splitting helper. The loader and IPC calls establish a remote dependency,
not the requested inline/verbose grammar.

## Historical official cross-check

[v8.5.25](https://github.com/RPGLogs/Uploaders-esologs/releases/tag/v8.5.25),
release 162881330, was published 2024-06-27T23:00:55Z. The official ZIP asset
176375974 is 160,655,072 bytes. Its SHA-256 is
`ef60d3138fcc91cc7f03ba32ff6624cd13f40e5694bd1a3181dd53374c7755b7`.
The publisher's GitHub API has no SHA-256 digest for this older asset; the actual
download was instead verified against its official `latest-mac.yml` SHA-512:
`i5Dg9oCG3TmjIWoRSB5cIJYOaOmEhsGKvpVQVeDIMpkIGmO3d45UXeJj9fLdN8fH3B9IkcYgGxDe+Gn0EDAwTg==`.

The older ASAR has 53 entries and likewise has no identified parser/cache
resource. Its `js/app.js` (622,340 bytes, SHA-256
`8345d35e97f3e95f924801e1e0df17eae4033f82b6f687fa4610b2db39ff22e7`)
already uses `/desktop-client/parser` with `id`, `ts`, `metersEnabled` and
`liveFightDataEnabled`, plus the same version-message seam. Its endpoint text is
at UTF-8 byte offset 489,866. This corrects reliance on the older public fork's
`/client/parser` URL; the new endpoint was already distributed in 2024.
Historical receipts and extracted text are under the `v8.5.25/` subdirectory.

## License observation and resulting evidence boundary

Neither inspected `package.json` declares a license field. The identified ASAR
license file contains third-party dependency notices, including classnames and
JSZip. Those notices do not identify a source license for the uploader's own
application/parser. No uploader implementation is copied into repository code.
The recorded endpoints, hashes, archive identities and loading behavior are
observations used to locate authoritative format evidence.

## Follow-through

The parallel resource investigation subsequently recovered public official text
at `https://assets.rpglogs.com/js/parser-eso.js`: HTTP 200, 39,617 bytes,
SHA-256 `07e90fd8227ff9b4d262254a86103eecddb984ef45b31e6b9dd8f03b8996c1d7`.
Its source analysis and response receipt belong to `research-inline.md`; this
distribution investigation does not infer grammar merely from that filename or
from a successful download. Target-profile support still requires checking the
actual parser/producer declarations and representative evidence.
