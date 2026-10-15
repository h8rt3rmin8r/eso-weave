# S130 native parser and checkpoint research

Date: 2026-10-15. Research only. This document declares reviewed evidence,
implementation proposals and unresolved producer grammar separately. It contains
no private source values and requests no installed-game verification.

Scope resolution: after this research, the owner explicitly approved S130 for
normal/separate version 15 only, deferred normal/inline to #296 and deferred
verbose combinations to #297. The producer grammar gap below remains evidence
for those follow-ups and no longer blocks the revised S130 scope. The concrete
normal/separate contract is [incremental-native.md](contracts/incremental-native.md).

## Reviewed sources

1. [ESOUI stock format help, current reviewed live commit](https://github.com/esoui/esoui/blob/6639eb2adecc0480557d9068579319919a0c3fe6/esoui/ingame/slashcommands/slashcommands_shared.lua#L43-L100).
   It lists the positional record fields, ten-field unit state, self-target
   abbreviation and nested equipment lists. It exposes independent verbose and
   inline flags. It does not implement the native serializer.
2. [ESOUI pinned S127 help](https://github.com/esoui/esoui/blob/f76cf16c4e5be7b234d15dc7f676febffa64c5bb/esoui/ingame/slashcommands/slashcommands_shared.lua#L43-L100).
   The relevant declaration is unchanged in the current reviewed commit.
3. [ZOS Update 22 API notes](https://forums.elderscrollsonline.com/en-gb/discussion/comment/6170265).
   ZOS describes compact output and the verbose/inline options, and describes
   effective and overflow quantities separately. This establishes option
   existence, not the exact verbose/inline byte grammar or present disk cadence.
4. [Public version 15 parser and samples](https://github.com/sheumais/logs/tree/3f28d10ab81450843bcde3530d58b98879053598/parser).
   The author's parser is positional. Its Scribing_Specimen.log and synthetic
   test material in another parser demonstrate a version 15 ABILITY_INFO record
   with three additional quoted script strings. These are sample evidence, not
   a ZOS serializer declaration. No code from this GPL-licensed parser is proposed
   for incorporation.
5. [Independent parser's synthetic log lines](https://github.com/thisisths/eso-log-filter/blob/89a727491462fe46d9d78ba6f4d556c36cb71def/src/tests/TestLogLines.cs).
   Its AbilityInfoScribed fixture also contains the three additional script
   strings. It provides corroborating shape coverage, not installed behavior.
6. [Independent current log field audit](https://github.com/brainsnorkel/eso-live-encounterlog-sets-abilities/blob/7a1a7e79ee3616e620f8ff983d608f02766f2c7b/docs/reference/encounterlog-format.md).
   This author's measurements identify the extra END_CAST integer as the
   interrupted ability and describe per-player scribing associations. Those
   interpretations are external observations and must not silently replace the
   S127 opaque-value contract.

The repository evidence remains `specs/127-native-completed-import/research.md`,
`contracts/native-import.md`, the sanitized representative fixture and
`tests/encounter_native.rs`. That fixture observes only normal/separate version 15.

## Declared normal/separate grammar

All counts include elapsed time and record type. Existing accepted shapes are:

| Record | Total fields |
| --- | --- |
| BEGIN_LOG | 7 |
| END_LOG, BEGIN_COMBAT, END_COMBAT | 2 |
| COMBAT_EVENT | 20 with self target, 29 with expanded target |
| BEGIN_CAST | 17 or 26 |
| EFFECT_CHANGED | 17/18 or 26/27 |
| HEALTH_REGEN | 13 |
| UNIT_ADDED, UNIT_CHANGED, UNIT_REMOVED | 18, 13, 3 |
| ABILITY_INFO | 7; reviewed public version 15 examples also have 10 |
| EFFECT_INFO | 6 or 7 |
| PLAYER_INFO | 8, with bounded nested arrays |
| MAP_CHANGED/MAP_INFO, ZONE_CHANGED/ZONE_INFO | 5 |
| END_CAST | 5 or 7 in S127 evidence |

`src/encounter/native.rs::fields` already handles commas outside quoted strings
and balanced bracket groups. `quoted_value` accepts explicit doubled-quote and
backslash escapes. The original owner's sanitized fixture did not demonstrate
every accepted escape; keep that distinction. Fields cannot be split with a
plain comma tokenizer. Integer parsing must remain exact, including character
identifiers larger than an IEEE-754 exact integer range. Numbers in unit state
may have signs, decimal points or exponents, but nonfinite spellings are invalid.

Known trial layouts are declared by stock help and may receive strict validation
in this slice. Unknown types remain retained evidence; a known type with an
unrecognized layout must not be relabeled unknown merely to keep parsing.
Do not infer API or addon version from log version 15 or the game patch string.

## Unresolved verbose/inline producer grammar

Reviewed ESOUI help says that verbose output names fields and inline output puts
ability information in event records. It does not declare the separator between
name and value, container spelling, exact inline ability expansion, field-name
capitalization, optional-field encoding or mode transitions. The reviewed public
Rust, Go, C# and Python parsers only demonstrated positional input. Targeted public
GitHub searches did not locate a producer-backed verbose/inline fixture.

Consequently these sources cannot substantiate a claim that a guessed
`name=value`, `name:value`, nested ability tuple or extra positional expansion is
the game's actual grammar. Synthetic fixtures do not resolve this by testing a
parser against its own assumptions. This was a planning dependency for the
original four-combination scope. The owner then explicitly deferred those three
combinations to #296/#297; S130's revised normal/separate plan may proceed.
Do not replace the deferred requirements with guessed normalization, mark their
producer fidelity proven or ask the owner to change game settings.

Regardless of accepted syntax, normalize supported serialization into one typed
record view before boundary/context/projection code. Preserve the exact original
line independently. Avoid rewriting raw evidence to pretend it was positional.
Use one semantic validator for normal and verbose records so format expansion
cannot bypass integer, arity, nesting or resource bounds.

### Inspected source and search inventory

The following inspection was completed before reaching that conclusion:

| Inspected source | Exact inspected revision and result |
| --- | --- |
| ZOS stock slash-command format help | ESOUI commit `6639eb2adecc0480557d9068579319919a0c3fe6`, lines 43-100. It prints positional names and toggles C++ serializer options; it contains no serializer or representative verbose/inline record. The declaration is unchanged from the pinned S127 commit. |
| [ESO Logs uploader](https://github.com/RPGLogs/Uploaders-esologs/tree/721c8705c9424e8b23729545d8ef0f40097c7ed1) | Entire public tree at `721c8705c9424e8b23729545d8ef0f40097c7ed1`: only README.md. |
| [ESO Logs test uploader](https://github.com/RPGLogs/TestUploaders-esologs/tree/5978c66fd36634b05ad1f758a020b9498366d341) | Entire public tree at `5978c66fd36634b05ad1f758a020b9498366d341`: only README.md. |
| [ESO Logs beta uploader](https://github.com/RPGLogs/Uploaders-esologs-beta/tree/ed0f6332f063180950fed684fe7eb17559921f89) | Entire public tree at `ed0f6332f063180950fed684fe7eb17559921f89`: only README.md. |
| Flatpak uploader wrapper | `zastrixarundell/com.esologs.Esologs`, inspected main tree and com.esologs.Esologs.yml on 2026-10-15. The manifest downloads a prebuilt RPGLogs AppImage; it contains no parser source or format fixture. |
| sheumais/logs parser and uploader converter | `3f28d10ab81450843bcde3530d58b98879053598`: parser/src/parse.rs, lib.rs, event.rs, parser/samples listing and Scribing_Specimen.log, cli/src/esologs_convert.rs. Input interpretation is positional; no verbose/inline implementation was found. |
| lparkes/esoelog | `a3ad60f82606d26a5c1171084513f63ac96003bb`: encounterlog.md, log.go and log_test.go. Positional input; format document repeats stock help. |
| brainsnorkel/esologprocessor | `f8daa63b246a9ca61b93d23c215215077b099159`: esolog/parser.py. Positional input. |
| brainsnorkel/eso-live-encounterlog-sets-abilities | `7a1a7e79ee3616e620f8ff983d608f02766f2c7b`: docs/reference/encounterlog-format.md. Positional observed vocabulary; no verbose/inline byte grammar. |
| thisisths/eso-log-filter | `89a727491462fe46d9d78ba6f4d556c36cb71def`: src/tests/TestLogLines.cs. Positional synthetic examples, including scribed ABILITY_INFO. |
| Flamindemigod/ELViewer | `0f1654fc5f4cb1b146c91f451d1387f7212b12e4`: src/modules/parser.rs. Positional input. |
| Camel-RD/esologs | `4044e36b60f9cbf58907542943a7b60c241817ff`: LogData.cs. Positional input. |

Public GitHub code-search results on 2026-10-15:

- Exact phrases `/encounterlog verbose` and `/encounterlog inline`: no results.
- `IsEncounterLogVerboseFormat`, `SetEncounterLogVerboseFormat` and
  `SetEncounterLogAbilityInfoInline`: stock ESOUI help was the only result.
- `sourceUnitState` plus `COMBAT_EVENT` plus `verbose`: only stock ESOUI help.
- `timeSinceEpochMS=`: no results. `timeSinceEpochMS:`: only unrelated projects.
- `org:RPGLogs encounterlog`: zero public code results.
- Repository search for esologs and public RPGLogs organization enumeration led
  to the uploader distribution repositories inspected above.
- Web searches for exact option names and verbose/inline native record examples
  led to the ZOS API notes and positional parsers above, not a serializer/sample.

These are bounded negative results for public indexed sources, not proof that no
such grammar or sample exists anywhere. No unavailable binary was downloaded,
executed or decompiled, and no game setting was changed. The current evidence
cannot decide the actual structural layout of verbose or inline records.

## Existing identity defect and compatibility

`run_id` at the research baseline hashes native schema, physical run ordinal,
channel and the original BEGIN_LOG line. `NativeEncounter` canonical JSON includes
absolute record ordinals and offsets. Copying an entire file works, and append
does not change earlier IDs. Extracting a later run or prepending a run changes
its ID; moving the same run between files can also change canonical bytes while
retaining a candidate logical encounter identity. These are separate concerns.

Recommended rules:

- Reader identity uses canonical environment root, channel, filesystem file ID
  and an independently checked generation witness. It is not encounter identity.
- Logical run identity uses the typed BEGIN_LOG tuple: channel, epoch milliseconds,
  log version, realm, language and game version. Filename, physical run ordinal,
  absolute byte position and serialization labels do not establish a new run.
- Logical fight identity uses the logical run key and run-local combat ordinal,
  with original ordered evidence/content validation before deduplication.
- Normalize evidence only for identity comparison by using run-relative positions;
  retain original physical provenance and raw canonical bytes separately.
- A matching header/key with divergent evidence is an explicit collision or fork,
  never an already-present result. Similar headers with distinct epoch or context
  remain distinct. Identical recorded evidence alone cannot establish two
  different real-world events; avoid claiming otherwise.

Keep existing native schema 1 validation and immutable rows readable. Add a
versioned logical-identity/alias index and a native schema 2 validation path
using the same NativeRun/NativeEncounter structs and projector. Preserve their
physical u64 provenance while schema2 IDs ignore placement. Do not duplicate
native model/table families or rewrite old canonical blobs and hashes. Index legacy rows by a verified
run-relative comparison to preserve their session/fight IDs when possible.
Any SQLite schema/alias migration belongs inside the first successful write
transaction; invalid parser/checkpoint input must leave previous store bytes and
schema intact. The existing strict schema/row validators and no-update triggers
must be deliberately extended, not bypassed.

## Incremental state proposal

Extract the loop in `parse_native_log` into an independently testable parser:
`feed(bytes)` produces bounded completed fights plus progress/controlled state;
an explicit source-generation termination method accounts for unfinished evidence.
Empty input and current EOF never synthesize combat or log endings.

State must retain:

- Parser schema/profile version and channel, cumulative source byte position,
  complete-record position and unconsumed tail bytes.
- Current run/header, physical and run-local ordinals, combat ordinal, previous
  elapsed clock and current selected serialization context.
- Exact unit/ability/effect/player/map/zone metadata and unknown context, including
  ordering, current byte/entry accounting and any identity comparison witness.
- Pending combat's start, original records, relevant starting context, referenced
  units/abilities, counts, byte reservations and clock-quality evidence.
- Controlled recovery state, loss/incomplete counters and the last verified
  boundary. No raw values appear in default Debug/errors/status.

Split bytes before UTF-8 decoding. Only decode a complete LF-terminated record;
retain incomplete UTF-8 and a split CRLF pair verbatim. At confirmed termination,
account for the unconsumed tail and unfinished combat without inventing closing
records. A fresh valid BEGIN_LOG provides an explicit resynchronization boundary
after a malformed segment, with uncertainty/incomplete accounting carried forward.

Retain existing per-record, nesting, metadata and pending-combat ceilings. Replace
whole-file 256 MiB, 2-million-record and 10000-run limits with per-feed and current
state budgets; otherwise a lifetime file eventually stops working. Context-copy
budgets must charge before allocation and reset per bounded emitted work, not
accumulate forever or reset in a way that permits one feed to amplify output.
The caller should cap bytes/records/fights per turn and yield catchup fairly.

## Checkpoint, generation and transaction proposal

Checkpoint encoding needs strict versioning, bounded decoding and structural
revalidation of every raw retained record, current context, pending fight, counters
and logical identities. Corrupt/incompatible checkpoints produce no fights. A
serialized state round trip alone proves determinism, not authenticity; match it
to a verified source-generation witness before resuming.

Keep checkpoint ownership with the orchestrator/store, outside the parser's
filesystem-free contract. Persist fights, aliases, deletion suppression and the
corresponding ingest checkpoint atomically. Publish the new in-memory cursor only
after commit. On DB busy/disk-full/transaction failure retain the previous cursor
and retry the same source bytes; do not advance progress after a failed insert.
Check eligibility/generation immediately before the transaction's stop boundary.

The existing `bounded_file` stable whole-file reader rejects append and imposes a
whole-file ceiling. Reuse its no-follow/link and same-file checks, but add a
bounded prefix/range reader that permits append beyond the sampled boundary.
Never implement live ingestion by rereading/reparsing the whole lifetime file.

A checkpoint witness should cover file ID, length at the consumed boundary,
bounded prefix/header and a bounded trailing-window digest around that boundary.
Truncation, replacement or a mismatched witness discards the active parser state
and begins a new generation at a verified run boundary. Rotation can retain an
already-open renamed generation long enough to consume its bounded remainder;
never concatenate an old partial UTF-8 tail with a replacement's new prefix.
Unchanged file identity alone cannot detect an in-place rewrite, and a bounded
witness cannot prove an arbitrarily rewritten entire prefix. Declare the actual
verified append model and preserve completed history on ambiguity.

## Required decisive tests

1. Feed the sanitized S127 fixture one chunk and at every meaningful byte split,
   including Unicode, quotes, escapes, brackets and CRLF; compare raw/canonical
   fights, ordering, context, IDs and supported quantities exactly.
2. Emit a fight immediately at END_COMBAT without END_LOG. Repeat empty feeds and
   quiet polls; no duplicates or fabricated interruption may appear.
3. Checkpoint inside metadata, UTF-8 tail and combat; deserialize/validate/resume
   to the same result. Reject changed versions, offsets, counts, context and
   identities without producing partial output.
4. Exercise append, copied files, extracted later runs, rotation, replacement and
   truncation; prove stable logical deduplication and separate source generations.
5. Open/migrate legacy native/addon stores and compare exact IDs/canonical bytes,
   ordering, backups and projections. Failed migration/checkpoint/insert must
   leave previous evidence intact.
6. Feed long logical streams through small chunks past old whole-file ceilings;
   bound retained state and work without requiring source deletion.
7. Test the declared normal/separate grammar using sanitized representative or
   explicit synthetic fixtures and label the evidence type accurately. Deferred
   inline/verbose support is not claimed. Unknown, malformed
   known, unsupported-version and resource-limit outcomes stay distinct.

## Owner resolution (2026-10-15)

S130 is explicitly scoped to normal/separate v15. Missing normal/inline grammar is tracked in #296 and verbose/separate plus verbose/inline in #297. The all-four evidence gap above is historical research and no longer blocks this approved scope. Preserve controlled unsupported outcomes and do not change game settings.
