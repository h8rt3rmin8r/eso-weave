# S131 Additional Public Source Research

Historical initial search, superseded by the official-uploader inspection and
pinned static producer evidence indexed in [research.md](research.md).

Date: 2026-10-15. Research scope: producer documentation and publicly inspectable
native specimens for normal/inline, verbose/separate and verbose/inline version 15.
This supplements S130 `research-native.md` and S131 `research-inline.md`.

## Result

No additional source inspected here establishes the exact requested profiles.
The stock help confirms the independent mode controls and general meanings, but
does not declare the serialized named-field or inline-ability grammar. Newly
located raw specimens and parser implementations cover positional/separate input.
These are bounded research findings, not a claim that no public evidence exists.

No product code, source logs, game setting, binary or executable was changed or
run. Repository fixtures were inspected as source text, not used to manufacture
an asserted producer format. No owner field-verification task is proposed.

## Primary declarations and original public specimen

- [Stock help at 6639eb2](https://github.com/esoui/esoui/blob/6639eb2adecc0480557d9068579319919a0c3fe6/esoui/ingame/slashcommands/slashcommands_shared.lua#L43-L100)
  documents record fields, independent verbose and inline flags, named verbose
  fields, and ability information being inline or separate. It delegates changes
  to native functions; this Lua file does not serialize native records.
- [2019 help at 0569f38](https://github.com/esoui/esoui/blob/0569f38e70254b4e08a5eab088c4ce5e7e46be55/esoui/ingame/slashcommands/slashcommands_shared.lua#L35-L94)
  already describes the same two flags. The file's history through
  [c6a91c3](https://github.com/esoui/esoui/commit/c6a91c390a9acee843560542923c69d06c438cd3)
  adds no exact serialization declaration. That latest file change affects random
  roll player naming, not encounter serialization.
- [ZOS Update 22 API notes](https://forums.elderscrollsonline.com/en/discussion/469015/update-22-api-patch-notes-change-log-pts)
  establish compact default output and the readability options. The linked
  `ESOUIDocumentationP22.txt` attachment could not be retrieved by the web reader
  (cache miss); no content claim is made for that attachment.
- [Original 2019 forum specimen](https://forums.elderscrollsonline.com/en/discussion/comment/6028364)
  presents player-generated raw records under both anonymity settings. Its headers
  declare version 12 and its ability records are separate positional records.
  It proves neither version 15 nor an inline or verbose profile. Player names and
  payload records are not copied into this repository.

## Newly inspected public source inventories

| Pinned repository | Inspected material | Relevance and limit |
| --- | --- | --- |
| [ESO-Toolkit/kalpa at 4fcfe19](https://github.com/ESO-Toolkit/kalpa/tree/4fcfe196e03e37bdc51c300684a9ce1bd5c5e6a1/src-tauri/src/uploader/native) | Full tree; `events.rs`, `encode.rs`, `convert.rs`, scanner; uploader plans and findings; two raw sample files | Public native input parser and upload encoder. `AbilityInfo::parse` in `encode.rs:60` expects positional ability fields. No target-profile handling found in these inspected files. |
| [Kalpa raw sample](https://github.com/ESO-Toolkit/kalpa/blob/4fcfe196e03e37bdc51c300684a9ce1bd5c5e6a1/src-tauri/src/uploader/native/testdata/sample_raw_encounter.log) | 108 nonempty records | Header declares version 15; separate `ABILITY_INFO`; no named-field record. |
| [Kalpa chunk sample](https://github.com/ESO-Toolkit/kalpa/blob/4fcfe196e03e37bdc51c300684a9ce1bd5c5e6a1/src-tauri/src/uploader/native/testdata/chunk1_raw.log) | 1,265 nonempty records | Header declares version 15; positional records and separate ability metadata. Does not establish requested layouts. |
| [sfl-eso/eso-logs-analyzer at fdfe5f3](https://github.com/sfl-eso/eso-logs-analyzer/tree/fdfe5f35dd0dd8bdaf57b1d5e64d180f5f933953) | Full tree; `target_event.py`, `event.py` | Positional constructor fields and shared stock-help reference. Its Python object debug representation is consumer output, not a native verbose specimen. No raw log fixture in the tree. |
| [Eurymachos/esoelog at 215da0f](https://github.com/Eurymachos/esoelog/tree/215da0f84c8a8fffbd07f5310bd3b83ebd879bff) | Full tree; `encounterlog.md` | Repeats positional version 15 documentation. Adds no inline or verbose serialization evidence beyond the S130 lparkes source. |
| [FuyuByakko/ESO-Logs-parser at 3e897a6](https://github.com/FuyuByakko/ESO-Logs-parser/blob/3e897a66339cf5165140fbd25c4b55ba7a97ea7c/utils/parseAndSeed.js) | Full tree; parser; README | Student implementation splits positional CSV. Referenced local `src_file/Encounter.log` is absent from the tree. |
| [HatefulHuske/ESOlogs-Reader-metadata at 1b625a1](https://github.com/HatefulHuske/ESOlogs-Reader-metadata/tree/1b625a10cf961f5d7cb3c87a294dfe5ce2bdf154) | Full tree | Privacy/terms text only, no parser or native specimen. |
| [knowlen/esologs-python at 052cfc3](https://github.com/knowlen/esologs-python/tree/052cfc3319d37314f5dee8fd583de8dbca716297) | Full recursive tree | ESO Logs GraphQL client and response tests, not native record serialization. |

Kalpa's documentation also discusses its upload transport and empirical output
encoding research. The upload wire format is distinct from the native source
format. Neither upload-encoder behavior nor synthetic tests prove game-produced
inline or verbose input. No third-party implementation was copied.

## Concrete remote parser avenue and access result

[TylerGrayStudent/WarcraftLogsUploader at d608b05](https://github.com/TylerGrayStudent/WarcraftLogsUploader/tree/d608b056e8fb00e3dac272c8b366192e369046fd)
contains a shared chunk consumer with an explicit ESO session-boundary comment.
Its complete tree has an IPC wrapper but no parser implementation.
[`loadParser` at lines 214-224](https://github.com/TylerGrayStudent/WarcraftLogsUploader/blob/d608b056e8fb00e3dac272c8b366192e369046fd/background-controller.js#L214-L224)
loads a remote origin plus `/client/parser` and a version suffix. The inspected
Warcraft game configuration returns an empty suffix; this does not establish the
current ESO uploader's suffix or grammar.

A read-only .NET HTTP GET to `https://www.esologs.com/client/parser` on this date
returned HTTP 403 with a Cloudflare challenge, not parser source:

- Final URL: `https://www.esologs.com/client/parser`.
- Content type: `text/html; charset=UTF-8`; 5,348 response bytes.
- ETag and Last-Modified: absent.
- SHA-256 of returned bytes:
  `74f6107165bd41945d43db815aa0100371bf65cae4080672ab12160e697b789d`.
- Ignored local response and receipt: `target/s131-research/parser-source.js` and
  `parser-source.metadata.json`. Despite the requested capture filename, the
  captured response is challenge HTML. It is not a JavaScript grammar artifact.

The web reader also could not access the parser URL. No challenge workaround,
authenticated request, upload, executable extraction or script execution was used.

## Bounded additional discovery

GitHub repository search covered `ESO encounter log`, `encounterlog`, `esologs`,
`elder scrolls log parser`, `esologs parser` and uploader repositories. Commit and
issue searches combined encounter logging with verbose and inline terms. Global
code searches paired the record vocabulary with `verbose`, `inline`,
`sourceUnitState`, `realmName`, `logVersion` and possible named ability values.
Exact flag-function searches returned the same stock ESOUI help.

Web searches combined the options with native record tokens and excluded known
sources to look for additional original examples. Retrieved useful results were
the ZOS notes and the historical version 12 specimen above. Unrelated addon event
callbacks, application debug text and API response examples were excluded as
native serialization evidence. Empty or irrelevant search results are not a
general absence proof.

## Remaining grammar evidence and useful next avenues

Before implementing a target profile, obtain a producer declaration or a reviewed
representative producer specimen that resolves:

1. Exact verbose field names, case, separators, nesting, optional-field spelling,
   absent values, and whether every record including boundaries is named.
2. The precise inline ability payload and placement for casts, combat events,
   effect changes, interrupted casts, synergy references and player-info arrays,
   including scribed abilities and any retained standalone metadata.
3. Quote/escape rules, unit-state grouping and self-target abbreviation for each
   profile; mode transitions inside a run and observable profile declarations.

The concrete remaining public avenue is a readable current ESO uploader parser
resource or a public producer specimen attached to an ESOUI/ZOS technical thread
that explicitly identifies the selected profile and native version. Historical
public normal/separate samples can supplement regression evidence, but cannot
settle these missing serialization rules. Do not promote invented layouts or
consumer pretty-printing into a producer contract.
