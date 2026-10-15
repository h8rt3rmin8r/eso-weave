# S131 inline grammar research

Historical initial search, superseded by the pinned static producer evidence
and current conclusion in [research.md](research.md).

Date: 2026-10-15. Scope: #296 normal/inline and #297 verbose/separate plus
verbose/inline native version 15. Research only; no product implementation,
game-setting changes, owner field-test request or commits.

## Current finding

The fresh source/history/corpus work below has not established exact inline
producer serialization. Do not substitute a parser-owned synthetic grammar.
The newly discovered uploader's remotely loaded parser is a concrete source
avenue, but its public endpoint could not be read. Exact grammar remains a
planning dependency for all three requested profiles.

## Fresh source avenue: remotely loaded upstream parser

The public uploader-like source at
[TylerGrayStudent/WarcraftLogsUploader, d608b056](https://github.com/TylerGrayStudent/WarcraftLogsUploader/blob/d608b056e8fb00e3dac272c8b366192e369046fd/background-controller.js#L214-L224)
loads a parser from the selected game's origin plus `/client/parser` and a
version suffix. `scripts/game.js` in this tree provides an empty suffix. The
shared chunk consumer recognizes ESO headers. The local tree does not itself
contain the native parser implementation. This is a Warcraft build containing
shared game code; uploader provenance and ESO-specific runtime source must be
verified before treating it as an authoritative ESO grammar.

The concrete ESO text endpoint is `https://www.esologs.com/client/parser`.
The surface research agent reported an HTTP 403 Cloudflare `Just a moment`
response from read-only .NET HTTP retrieval. The primary agent's independent
in-app browser navigation reported `net::ERR_HTTP_RESPONSE_CODE_FAILURE`.
This research agent's browser fallback was unavailable in its tool context:
hidden in-app browser creation returned `Browser is not available: iab`, and
its surface inventory returned no enabled browsers or apps. These observations
establish an access limitation, not absence of grammar at that endpoint. No
credentials, challenge bypass, executable download or producer code execution
was used to retrieve it.

## First-party help history

The GitHub path history for
`esoui/ingame/slashcommands/slashcommands_shared.lua` returned 14 revisions,
including the following inspected snapshots:

| Revision | Date | Result |
| --- | --- | --- |
| [0569f38e70254b4e08a5eab088c4ce5e7e46be55](https://github.com/esoui/esoui/blob/0569f38e70254b4e08a5eab088c4ce5e7e46be55/esoui/ingame/slashcommands/slashcommands_shared.lua#L37-L96) | 2019-10-21 | Positional field vocabulary plus independent normal/verbose and separate/inline controls. No inline expansion/container syntax or verbose name/value separator. |
| [1b39673fd405ff3560ca55bc6e1eca6225033835](https://github.com/esoui/esoui/blob/1b39673fd405ff3560ca55bc6e1eca6225033835/esoui/ingame/slashcommands/slashcommands_shared.lua#L37-L96) | 2020-05-26 | Same missing byte grammar. Equipment naming changed from quality to displayQuality; no inline serialization declaration appeared. |
| [c6a91c390a9acee843560542923c69d06c438cd3](https://github.com/esoui/esoui/blob/c6a91c390a9acee843560542923c69d06c438cd3/esoui/ingame/slashcommands/slashcommands_shared.lua#L44-L103) | 2026-09-28 | Same option existence and positional declaration. Effect-bar naming evolved, but no native serializer or inline/verbose representative output. |

The format-help implementation prints static descriptors and calls native
getters/setters. It does not serialize encounter records in Lua. Its history does
not establish whether inline data replaces an ability ID, expands adjacent
fields, uses a nested group, or changes optional-reference representation.

## Fresh public log corpus inspection

Public GitHub search for `BEGIN_LOG COMBAT_EVENT extension:log` returned six
indexed files. One is this project's existing sanitized S127 fixture and is not
independent evidence. The five external files were fetched at exact revisions
and examined with a quote/bracket-aware structural tokenizer. Original account,
character, ability text and raw numeric values are not reproduced here.

| Pinned file | Structural result |
| --- | --- |
| [sheumais/logs Mundus_Buffs.log](https://github.com/sheumais/logs/blob/3f28d10ab81450843bcde3530d58b98879053598/parser/samples/Mundus_Buffs.log) | Declares version 15. Contains 14 separate ABILITY_INFO records; 13 BEGIN_CAST, 13 COMBAT_EVENT and 64 EFFECT_CHANGED records all use integer ability references. Observed event arity 20 and self-target effect/cast arity 17 match normal/separate. |
| [sheumais/logs Loadout_Specimen.log](https://github.com/sheumais/logs/blob/3f28d10ab81450843bcde3530d58b98879053598/parser/samples/Loadout_Specimen.log) | Declares version 15. Contains 66 separate ABILITY_INFO records. All 67 EFFECT_CHANGED, 3 BEGIN_CAST and 4 COMBAT_EVENT ability references are integers. Source/target arities 17/26 and 20/29 match normal/separate. |
| [brainsnorkel golden_fight.log](https://github.com/brainsnorkel/eso-live-encounterlog-sets-abilities/blob/7a1a7e79ee3616e620f8ff983d608f02766f2c7b/tests/fixtures/golden_fight.log) | Declares version 15. Contains 13 separate ABILITY_INFO records, 7 COMBAT_EVENT records with integer ability references and 1 EFFECT_CHANGED with integer reference. No inline or named-field record. Fixture status is not itself proof of producer capture. |
| [esologprocessor sample_encounter.log](https://github.com/brainsnorkel/esologprocessor/blob/f8daa63b246a9ca61b93d23c215215077b099159/tests/test_data/sample_encounter.log) | BEGIN_LOG has 5 fields and no version in the reviewed version 15 position. Several other record shapes differ from the first-party normal/separate vocabulary. Positional ability references remain integers. It cannot establish version 15 inline grammar. |
| [esologprocessor full_combat_example.log](https://github.com/brainsnorkel/esologprocessor/blob/f8daa63b246a9ca61b93d23c215215077b099159/tests/test_data/full_combat_example.log) | Same 5-field header problem. Four separate ABILITY_INFO records and integer event/effect references. It cannot establish version 15 inline grammar. |

These are bounded public-index results. GitHub excludes some large/binary files
and does not guarantee coverage of every public recording. Finding no inline
record in this set is not proof that the mode does not exist.

## Additional parsers inspected

[BloodStainChild/ESOLogViewer LogImporter.cs](https://github.com/BloodStainChild/ESOLogViewer/blob/5c65805a2eabf1e9763b88162d2a3a97a0fccb67/src/EsoLogViewer.Core/Services/LogImporter.cs)
uses positional field access and integer ability IDs. It separately handles
ABILITY_INFO. No explicit inline/verbose branch was located. Its recursive
public tree exposed a CSV tokenizer and storage code, with no log/sample/test
corpus identified by the bounded filename inventory. Some semantic comments
also differ from stock help, so this consumer cannot independently establish
unobserved producer semantics.

The unusually promising commit
[brainsnorkel/eso-live-encounterlog-sets-abilities, 9154af9d](https://github.com/brainsnorkel/eso-live-encounterlog-sets-abilities/commit/9154af9dffe90f23ec71e9623fb9b790785dd8a9)
is titled `feat: Add comprehensive structured log parser with symbolic field
names`. Its sole added file,
[src/eso_log_structures.py](https://github.com/brainsnorkel/eso-live-encounterlog-sets-abilities/blob/9154af9dffe90f23ec71e9623fb9b790785dd8a9/src/eso_log_structures.py),
uses symbolic Python dataclass attributes over positional CSV. It does not
demonstrate native verbose records. Header parsing uses CSV fields at lines
198-211, ABILITY_INFO uses an integer ID at lines 372-384, BEGIN_CAST uses
`int(fields[5])` at lines 442-465, EFFECT_CHANGED uses `int(fields[5])` at lines
559-581, and COMBAT_EVENT uses `int(fields[8])` at lines 611-630. No inline or
verbose decoder branch was found. The header's positional interpretation also
calls field 3 `session_id`, whereas the reviewed producer header uses that
position for the native log version. This consumer's attribute names cannot
establish producer field semantics.

## Additional search inventory

- `BEGIN_LOG HEALTH_REGEN UNIT_ADDED`: 26 indexed results, primarily already
  inspected positional parsers, copied API vocabulary and this project's
  historical files.
- Same query with forks enabled: four indexed results, one parser fork and
  stock-help forks; no new format-specific fixture was identified.
- `UNIT_ADDED HEALTH_REGEN inline`: stock help and this project's historical
  research only.
- `ABILITY_INFO EFFECT_INFO verbose`: stock help, generic/synthetic consumer
  material and unrelated projects; no exact producer serialization.
- `BEGIN_LOG COMBAT_EVENT extension:txt` and
  `BEGIN_LOG realmName extension:log`: zero results.
- `abilityId inline repo:sheumais/logs`: zero results.
- Exact `EncounterLog AbilityInfoInline`: stock help only.
- Public web searches restricted to GitHub Gists, GitLab, Bitbucket and ESOUI
  for native records/inline descriptions returned no additional relevant result.
- One additional code-search call received HTTP 429 and was not counted as
  negative evidence.

## Required unresolved field coverage

Before a parser contract can pass, reviewed evidence must settle these exact
questions, without assuming a container or expansion:

| Surface | Required evidence |
| --- | --- |
| Header/profile identification | Exact header bytes, log version position, any profile marker and how a profile is recognized when no marker exists. |
| Ability-bearing cast/event/effect records | Exact inline representation and field boundaries at each ability reference; metadata presence/omission and zero/absent references. |
| Unit states and self-target reuse | Whether inline changes unit-state shape, how groups nest, and the exact self-target abbreviation. |
| Optional references | End-cast interrupting ability/unit references, effect-removal reference and effect-info synergy references under each supported profile. |
| Player/loadout lists | Whether ability IDs inside arrays remain IDs or expand; exact nesting and empty-list representation. |
| Strings | Commas, Unicode, quotes, backslashes, empty strings and the producer's actual escape convention. |
| Transitions | Separate/inline changes within an active run, metadata scope and whether source state or record-level shape selects decoding. |
| Version compatibility | Evidence tied to native version 15 rather than an older game build or a consumer's synthetic fixture. |

## Gate consequence

No exact representative inline or verbose producer records were recovered.
The authoritative option vocabulary and positional consumers do not resolve
the missing grammar. Implementing one of the three requested profiles from
this evidence would require invented syntax or unverified semantics, so this
research does not support passing the grammar planning gate. All three
profiles remain in scope. No product code or guessed candidate grammar was
written by this research task.
