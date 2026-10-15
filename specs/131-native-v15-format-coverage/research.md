# S131 Research and Grammar Gate

Date: 2026-10-15. Scope: #296 normal/inline and #297 verbose/separate plus verbose/inline native version 15.

## Current result after owner-directed deeper inspection

The grammar gate is resolved. Static inspection of the installed readable
producer establishes native v15 from its literal BEGIN_LOG version and traces
the serialization primitives and event call sites. Exact normal/inline and
both verbose profiles are pinned in
[the grammar contract](contracts/native-v15-grammar.md). The independent
[primitive](research-producer.md), [header/profile](research-producer-header.md)
and [ability/effect](research-producer-abilities.md) reports distinguish direct
writer evidence from inference and synthetic tests.

The owner requested official uploader inspection before implementation.
[Current and historical official uploader assets](research-official-uploader.md)
were inspected passively. They fetch an authenticated remote parser. The
[recovered public CDN parser](research-parser-resource.md) is historical and
does not establish the new grammar. No uploader or game code was executed.

Design decisions: one bounded record-local semantic decoder; exact raw lines;
no run profile state, checkpoint/store migration or synthetic metadata records;
frozen schema 1 and old normal schema 2 compatibility/classification; signed
producer observations retained as signed quantities. Shared feed, canonical,
context, restart and projection validation prevent divergent interpretations.
All three profiles remain in scope and owner field testing remains excluded.

## Earlier result (superseded by the evidence above)

Phase 0 remains incomplete: the reviewed evidence does not establish the exact serialized grammar of any of the three requested profiles. The owner's grammar-first requirement is not satisfied. All three profiles remain in scope; this is a dependency report, not a completed feature or a narrower implementation.

No product code, schema migration, parser fixture asserting a guessed grammar, commit, push, PR or Codex review round has occurred. Source logs and game settings are unchanged. No owner field-testing task is requested.

## Evidence reviewed

- [Inline research](research-inline.md): new public log corpora, source histories, another desktop parser and the distinction between symbolic consumer attributes and serialized field names.
- [Verbose research](research-verbose.md): new Python/Go/JavaScript/C# readers, original stock-help history and exact unresolved named-field dimensions.
- [Additional public-source research](research-sources.md): Kalpa's v15 raw specimens and native parser, a historical original forum specimen, uploader source and the remote parser access receipt.
- [S130 baseline research](../130-automatic-native-processing/research-native.md): prior positional evidence and the historical owner-approved scope split. S131 adds the sources above rather than treating those old negative searches as conclusive.

The reviewed [ESOUI help](https://github.com/esoui/esoui/blob/6639eb2adecc0480557d9068579319919a0c3fe6/esoui/ingame/slashcommands/slashcommands_shared.lua#L43-L100) establishes independent verbose and inline options and lists positional semantic fields. It calls native serializer controls and does not implement the native serializer. The [ZOS API announcement](https://forums.elderscrollsonline.com/en-gb/discussion/comment/6170265) establishes option intent without the missing byte syntax.

The public uploader source at [d608b056](https://github.com/TylerGrayStudent/WarcraftLogsUploader/blob/d608b056e8fb00e3dac272c8b366192e369046fd/background-controller.js#L214-L224) identifies a remotely loaded parser resource. The attempted ESO resource, `https://www.esologs.com/client/parser`, returned HTTP 403 challenge HTML, not JavaScript. The web reader failed to retrieve it; normal in-app browser navigation failed with `net::ERR_HTTP_RESPONSE_CODE_FAILURE`. No source claim is made for this unavailable content. The uploader tree's presence also does not prove its current ESO version or support for the requested profiles.

## Existing local-source check

A read-only structural scan of the existing native log under the resolved Documents Live root examined 43,518,907 bytes and 298,596 lines. It found three seven-field version-15 BEGIN_LOG records, zero non-positional lines, 3,855 separate ABILITY_INFO records and no non-integer ability references in COMBAT_EVENT, BEGIN_CAST or EFFECT_CHANGED. Observed event/state arities match the established normal/separate profile. No PTS log directory was present at the resolved Documents root.

The scan emitted aggregate counts and validated record-type tokens only. It did not reproduce account/character values, source record lines or raw event values in public artifacts, upload a source log or change settings. The ignored local helper/receipt are `target/s131-inspect-local-log.mjs` and `target/s129-s131-local-log-structure.log`. This structural audit is not a field-behavior or installed-feature validation claim.

## Unresolved grammar inventory

| Profile | Required evidence still missing |
| --- | --- |
| Normal/inline v15 | Exact ability payload placement and expansion, ability-bearing arrays and optional references, grouping/escaping, retained standalone metadata and observable profile transitions |
| Verbose/separate v15 | Exact field names/case, initial time/type encoding, name/value separators, whitespace, unit-state grouping, arrays, optionals, escapes and transitions |
| Verbose/inline v15 | All verbose rules plus exact inline ability expansion and its interaction with unit states, optional references and arrays |

Existing positional samples, downstream object pretty-printing and the uploader's distinct transport encoding do not resolve this inventory. Synthetic partitions may later exercise substantiated grammar but cannot establish producer fidelity.

## Decisions

1. **Decision**: Preserve grammar-first as an unmet prerequisite. **Rationale**: Neither source evidence nor a producer specimen establishes how the target profiles encode their facts. **Alternatives considered**: Guessing labels/groups or accepting several speculative forms would validate parser assumptions rather than actual producer output; implementing only one already-supported profile would not complete these issues.
2. **Decision**: Keep exact raw records separate from a typed semantic view when design can proceed. **Rationale**: Shared validation/projection preserves consistent facts without falsifying source evidence. **Alternatives considered**: Rewriting verbose raw lines into positional strings loses the distinction required by #297.
3. **Decision**: Preserve existing canonical/checkpoint/identity contracts until substantiated grammar demonstrates a necessary version change. **Rationale**: Premature schema expansion duplicates working infrastructure and risks old evidence. **Alternatives considered**: A second store/projector/worker has no demonstrated need.
4. **Decision**: Complete independent slice setup and chronological housekeeping while retaining the unresolved dependency. **Rationale**: The exact branch/specification, issue mapping, Plan057 and archived Plan056 are useful resumable work without claiming parser completion.

## Research limits and resumption

Public search is bounded and omits some large, ignored, historical or unindexed files. Shared GitHub rate-limit responses were recorded as failed requests, not empty results. These findings do not prove no suitable specimen exists anywhere.

The concrete remaining input is a readable native serializer/parser source or representative producer records that settle the inventory above, including v15 attribution. Once available, pin the evidence and supported shapes, finish Phase 0, produce data model/contracts/quickstart, generate tasks and pass blocking analysis before implementation. Existing push/PR and two-review authorization continues to apply to S131; no new publication permission is needed.
