# Encounter Ingestion

Use [Encounter Capture](../features/encounter-capture.md#import-retained-addon-captures)
for retained saved-addon import and historical recorder semantics. Desktop Encounter History also supports **Native Encounter Log** for
completed local files in the **normal/separate native log version 15** profile.
These sources keep their own provenance and metric capabilities. Neither is the
desktop Application Log or File Logging setting. Bulk records never use Pixel Bus.

## Completed native files

1. Finish native recording in ESO, or select a stable copy of the finished file.
2. Open **File > Encounter History** and select **Native Encounter Log**.
3. Select Live or PTS in Settings, then enter the local **Native file path**.
   The selected environment is user-selected provenance. The native header does
   not declare an ESO API version; log version 15 is a separate format version.
4. Choose **Inspect Native File**. Check completed fights, runs, unknown records,
   records outside combat, unfinished fights/runs and unterminated final lines.
5. Choose **Import Native File**. Select an imported fight for Native Observations.
   Reimporting an identical file or copy reports already-present fights.

Inspection is read-only and creates no history. A changed source needs a fresh
inspection before desktop import. The background worker echoes selection and
generation; changing source kind, path or environment discards stale results.
Import rereads a bounded stable no-follow regular file and rejects changes during
the read. Source files remain unchanged.

## Supported profile and evidence

The parser consumes UTF-8 newline-terminated records (LF or CRLF, optional initial
UTF-8 BOM), quoted separators and explicit escapes, and bounded bracket groups.
It validates known version 15 shapes for headers, combat boundaries/events, casts,
effects, regeneration, units, ability/effect/player metadata, maps, zones and
endless-dungeon records. Other types, including undeclared trial shapes, remain
unknown evidence.
Malformed known shapes, unsupported log versions and unsupported inline/verbose
layouts reject the whole import before history writes. Unknown record types remain
retained and counted. The unterminated final line is unconsumed and reported.
Unclosed combats are counted and omitted; independently completed fights in an
unfinished run remain importable. Nested/orphan combat boundaries and reversed
combat endpoints reject the batch.

Each BEGIN_LOG starts isolated context. Completed fights retain exact ordered
original lines and relevant preceding unit, ability, effect, player, map/zone and
unknown context. Names, identifiers and original fields remain local in history;
default summaries, diagnostics and receipts omit those values. Public fixtures
are sanitized. File paths, timestamps and controlled counts are selection facts.

Limits are 256 MiB per source, 2 million records, 64 KiB per line, 4096 tokens per
line, bracket depth 8, 100000 context entries, 10000 runs/completed fights,
200000 records and 32 MiB raw text per fight, 8 MiB relevant context per fight,
512 MiB retained raw batch text and 64 MiB canonical bytes per fight. Across the
whole file, context snapshots may inspect at most 20 million records and 512 MiB
of original context text, charged before cloning or scanning, including metadata
that is ultimately irrelevant. New runs do not reset this work budget. Limit
failures reject atomically. These are ingestion bounds, not platform benchmarks.

Native duration and observed event/cast/effect counts need no catalog. Reviewed
damage/healing hit quantities and overflow are separate; overflow is never
subtracted from hit. Local-player totals/rates require a unique observed local
PLAYER relation, with rates unavailable for unknown evidence, clock reversal or
zero duration. Effect-change counts do not establish uptime. Callback-only
quickslot/death-recap observations, addon replay, catalog API joins and
recommendations remain unavailable with reasons. See
[Encounter Data and Metrics](../reference/encounter-data-and-metrics.md#native-observations).

Store schema 6 adds immutable native runs/fights alongside addon rows and shared
first-import ordering. Migration from schema 1-5 and all accepted fights share one
transaction. Invalid input, identity/content collisions and no-op writes leave
previous history bytes/schema intact. Copy/reimport and append-later-run identities
are deterministic; deletion affects only local copies, and backup includes both sources.

## Maintainer validation

These commands inspect/import a completed file without displaying its raw values:

- `catalog-compiler native-encounter-inspect --input PATH --channel live|pts`
- `catalog-compiler native-encounter-import --input PATH --store PATH --channel live|pts`

Use an isolated store for investigation. The source identity is the original byte
hash; each canonical fight has a separate content hash. The sanitized representative
fixture and synthetic boundary/format/hostile cases support repository behavior.

## Remaining runtime scope

Live tailing, enabling native logging from desktop, every native format variant,
flush cadence, crash durability, callback parity and Windows/Linux/Proton runtime
qualification remain unimplemented or unverified. ESO API declarations alone do
not establish those behaviors. Historical issue #190's closure is not installed
verification and does not block completed-file import. S127/#272 requests no new
owner field-verification task.
