# S131 verbose v15 grammar research

Historical initial search, superseded by the pinned static producer evidence
and current conclusion in [research.md](research.md).

Date: 2026-10-15. Research only for #297. No product code, producer settings,
private logs, branches or commits were changed by this research.

## Finding

The additional public sources examined below do not establish an exact native
verbose/separate or verbose/inline v15 grammar. Existing first-party help proves
that verbose output names fields and that inline ability information is an
independent option. It does not reveal the serializer's name/value punctuation,
field-name bytes, nested grouping or mode-transition behavior. A named Python
model or an application's formatted debug string is not a producer log sample.

Until a producer-backed specification or representative source lines resolve
those details, #297's grammar acceptance gate remains open. This is the result
of a bounded source review, not a claim that no such evidence exists anywhere.

## New sources and histories examined for S131

| Pinned source | Concrete observation | What it cannot establish |
| --- | --- | --- |
| [sfl-eso/eso-logs-analyzer tree, fdfe5f35](https://github.com/sfl-eso/eso-logs-analyzer/tree/fdfe5f35dd0dd8bdaf57b1d5e64d180f5f933953) | The complete reviewed tree contains Python readers and event models, without a committed raw log corpus. | Exact verbose source serialization or a v15 producer sample. |
| [CSV reader](https://github.com/sfl-eso/eso-logs-analyzer/blob/fdfe5f35dd0dd8bdaf57b1d5e64d180f5f933953/eso_logs_analyzer/loading/utils.py#L38), [event dispatch](https://github.com/sfl-eso/eso-logs-analyzer/blob/fdfe5f35dd0dd8bdaf57b1d5e64d180f5f933953/eso_logs_analyzer/models/data/events/event.py#L81), [BEGIN_LOG model](https://github.com/sfl-eso/eso-logs-analyzer/blob/fdfe5f35dd0dd8bdaf57b1d5e64d180f5f933953/eso_logs_analyzer/models/data/events/begin_log.py#L18), [combat model](https://github.com/sfl-eso/eso-logs-analyzer/blob/fdfe5f35dd0dd8bdaf57b1d5e64d180f5f933953/eso_logs_analyzer/models/data/events/combat_event.py#L15) | The reader splits CSV and passes positional arguments to typed constructors. Ability lookup remains a separate indexed model. | Constructor parameter names are not evidence that those names appear in native bytes. |
| [Event string formatter](https://github.com/sfl-eso/eso-logs-analyzer/blob/fdfe5f35dd0dd8bdaf57b1d5e64d180f5f933953/eso_logs_analyzer/models/data/events/event.py#L48) | The application's own string conversion emits named object properties using `=` and parentheses. | This is downstream diagnostic formatting, not native verbose grammar. It must not be used to invent native `name=value` support. |
| [Eurymachos/esoelog tree, 215da0f8](https://github.com/Eurymachos/esoelog/tree/215da0f84c8a8fffbd07f5310bd3b83ebd879bff) | This public fork adds positional Go processing tools. Its reviewed history contains five commits, including the initial import and tag changes. No additional raw sample files appear in the complete tree. | No independent verbose grammar beyond the already reviewed lparkes positional family. |
| [FuyuByakko/ESO-Logs-parser, 3e897a66](https://github.com/FuyuByakko/ESO-Logs-parser/tree/3e897a66339cf5165140fbd25c4b55ba7a97ea7c), [parser entry](https://github.com/FuyuByakko/ESO-Logs-parser/blob/3e897a66339cf5165140fbd25c4b55ba7a97ea7c/utils/parseAndSeed.js#L18) | This older parser directly splits CRLF and commas and dispatches on the second positional field. Its README requires an external source file; the complete tree does not contain that file. | It supplies neither robust general tokenization nor verbose/inline producer evidence. |
| [BloodStainChild/ESO-Logfile-Splitter, 095b77eb](https://github.com/BloodStainChild/ESO-Logfile-Splitter/tree/095b77eb3426dfa0d5db185c8697a96e81ddc57a), [header handling](https://github.com/BloodStainChild/ESO-Logfile-Splitter/blob/095b77eb3426dfa0d5db185c8697a96e81ddc57a/Editor/Editor.cs#L82) | This utility locates record markers and splits fields. The complete tree has no raw log corpus. Binary build outputs were not downloaded or executed. | Marker matching by a file splitter is not exact native grammar validation. |
| [sheumais parser history](https://github.com/sheumais/logs/commits/3f28d10ab81450843bcde3530d58b98879053598/parser), [conversion introduction, dd871163](https://github.com/sheumais/logs/commit/dd87116347f670e061fa7e1088b8d161ec3f415d) | History exposes a commit described as reverse engineering the uploader's format. The introduced converter still consumes positional native lines and produces a separate compact upload representation. Both reviewed public branch heads resolve to 3f28d10ab81450843bcde3530d58b98879053598. | A derived upload format, including pipe delimiters, cannot declare the game's verbose source format. |
| [ESOUI help introduction, 0569f38e](https://github.com/esoui/esoui/blob/0569f38e70254b4e08a5eab088c4ce5e7e46be55/esoui/ingame/slashcommands/slashcommands_shared.lua#L37) | The 2019 history introduces the same separate verbose and inline toggles and field-list help. Formatting occurs in chat help, while native production is delegated to game functions. | Historical chat-help separators and typography are not native log punctuation. |

The independent uploader-source avenue is coordinated in the companion research:
[normal/inline research](research-inline.md). The newly discovered public
[TylerGrayStudent uploader tree, d608b056](https://github.com/TylerGrayStudent/WarcraftLogsUploader/tree/d608b056e8fb00e3dac272c8b366192e369046fd)
contains an IPC wrapper and a common chunk consumer with ESO handling. Those
files delegate parsing; they do not by themselves reveal verbose grammar. The
surface researcher owns further upstream parser endpoint inspection. A newer
[ESO-Toolkit/kalpa uploader-workspace commit, 4fcfe196](https://github.com/ESO-Toolkit/kalpa/commit/4fcfe196e03e37bdc51c300684a9ce1bd5c5e6a1)
was also handed to that researcher. Its commit title alone is not evidence of
format support.

## Existing first-party boundary

The reviewed [current stock help, 6639eb2a](https://github.com/esoui/esoui/blob/6639eb2adecc0480557d9068579319919a0c3fe6/esoui/ingame/slashcommands/slashcommands_shared.lua#L43)
and [ZOS Update 22 API announcement](https://forums.elderscrollsonline.com/en-gb/discussion/comment/6170265)
remain evidence for the existence and intent of the two options. They are not
newly discovered full grammars. The 2019 source history above adds provenance
for that boundary, rather than filling it.

In particular, the camel-case field labels in help do not prove exact verbose
field spelling. The help also does not establish whether the initial time/type
pair is named, whether nested unit state is flattened, or whether inline ability
metadata uses the same nesting and quoting rules as other values.

## Required evidence matrix

| Grammar dimension | Verbose/separate | Verbose/inline |
| --- | --- | --- |
| Initial time/type spelling and separators | Unresolved | Unresolved |
| Exact names/case and name/value separator | Unresolved | Unresolved |
| Whitespace rules | Unresolved | Unresolved |
| Strings, UTF-8, quote escaping and embedded commas | Unresolved | Unresolved |
| Unit-state grouping and self-target abbreviation | Unresolved | Unresolved |
| PLAYER_INFO arrays and nested equipment entries | Unresolved | Unresolved |
| Ability/effect metadata spelling and optional values | Unresolved | Unresolved |
| Inline ability expansion and associated effect/scribing metadata | Not applicable to separate placement; actual verbose metadata grammar unresolved | Unresolved |
| Optional END_CAST/EFFECT_CHANGED/EFFECT_INFO tails | Unresolved | Unresolved |
| Trial/map/zone and other known record layouts | Unresolved | Unresolved |
| Normal-to-verbose and separate-to-inline transitions within a run | Unresolved | Unresolved |
| Header/log-version signal for the serialization mode | Unresolved | Unresolved |

This applies to every known record family, including BEGIN_LOG, END_LOG,
BEGIN_COMBAT, END_COMBAT, PLAYER_INFO, BEGIN_CAST, END_CAST, COMBAT_EVENT,
HEALTH_REGEN, UNIT_ADDED, UNIT_CHANGED, UNIT_REMOVED, EFFECT_CHANGED,
ABILITY_INFO, EFFECT_INFO, map/zone records and trial records. Existing positional
tests cannot supply those missing named-field rules.

## Search limitations and implementation consequence

Fresh GitHub repository and code searches covered native header/record tokens,
named field labels, verbose references, parser repositories and historical
commit messages. Fresh web searches combined encounter-log, verbose, field-name
and sample terms. Issue searches found no external exact-layout evidence.
Searches can omit large, ignored, archived or unindexed files. A shared GitHub
secondary rate limit was reached during the final search batch; it was not
treated as an empty result and no rapid retry loop was used.

No regex that strips guessed labels, accepts several guessed separators, or
flattens guessed groups can substantiate #297. Once source-backed rules exist,
the implementation should preserve literal source records and build a separate
typed semantic view for the existing validator/projector. Neither automatic
mode detection nor checkpoint state should claim a verified serialization
profile until its exact grammar is established. This research does not narrow
the owner's requested scope or authorize an unsupported profile as complete.
