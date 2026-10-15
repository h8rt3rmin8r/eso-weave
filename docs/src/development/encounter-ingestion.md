# Encounter Ingestion

The application owns native discovery, incremental parsing, durable progress and
downstream processing. These services run without a viewer or selected encounter.
[Automatic Native Processing](../features/encounter-capture.md) describes normal operation.
Bulk encounter evidence never uses Pixel Bus and never becomes gameplay input.

## Automatic discovery and ingestion

Eligible combined-addon contexts provide actual Live or PTS source roots.
Discovery includes supported Windows Documents and Linux Steam/Proton locations
and configured overrides. Later sources are discovered without restarting the
application. The source context remains attached to queued work if the UI's
selected environment changes.

Bounded turns catch up completed fights and follow growing files. Parser state
retains incomplete UTF-8/record tails, run metadata and ordered context; a quiet
file does not terminate a combat. Combat completion is independent from run
completion. Native processing and automatic Catalog maintenance use independent
preferences and scheduling. No picker, inspection, import or acknowledgement is
required.

Fights and accepted progress commit consistently. Restart validates progress
against source identity before resuming. Copy, rotation, replacement and truncation
cannot mix old partial state with a new file generation. Unsupported or malformed
records leave committed history intact and diagnostics omit raw values. Cancellation
and commit admission share the declared stop boundary. The desktop never changes
game logging, serialization, anonymity or source bytes.

## Supported profiles and evidence

Normal and verbose native log version 15 support separate and inline ability
information. S131 pins direct static producer writers, including a literal v15
header, exact labels, TAB/comma delimiters, flat inline tails and nested equipment
arrays. The official uploader was inspected; its current parser is authenticated
and its recovered public CDN parser is historical. See the
[S131 grammar contract](https://github.com/h8rt3rmin8r/eso-weave/blob/main/specs/131-native-v15-format-coverage/contracts/native-v15-grammar.md)
for hashes, source addresses and evidence limits. Synthetic fixtures derive from
those writers; they are not captured game sessions or installed-game proof.

The parser accepts LF/CRLF UTF-8 records, an optional initial BOM, quoted separators
and bounded bracket groups. Verbose fields use exact ordered `name: value`
labels separated by TAB, including repeated flat unit-state labels. Producer
strings contain literal bytes inside quotes; the existing normal/separate reader
also preserves its historical accepted escape forms. Inline numeric ability IDs
are followed by four metadata fields, and effects append their own metadata.
Scalar ID arrays stay scalar in inline mode. Independent format switches emit
no marker, so each record is decoded locally without changing the run identity.
Known version 15 shapes include
headers, combat boundaries/events, casts, effects, regeneration, units, ability,
effect and player metadata, maps, zones and endless-dungeon records. Other types
remain unknown evidence; unsupported layouts are not guessed into valid records.

Native evidence preserves exact ordered original lines and relevant preceding
context locally. Semantic decoding never replaces those original lines. Existing
schema 1/2 canonical evidence, hashes and unknown-record classification remain
unchanged; no store/checkpoint migration is needed for format coverage. Epoch,
realm, language and game-version fields retain their
source meaning; native log version does not supply an ESO API or addon version.
Unknown evidence and uncertainty remain explicit. The lifetime size or age of a
log does not replace per-turn, per-record, context, fight and transaction limits.

## Storage and consumers

The user-owned `encounters/encounters.sqlite` retains addon and native canonical
evidence, first-import ordering and compatible historical rows. Durable ingest
progress and rebuildable derived results are separate from immutable source
content. Retained deletion utilities do not cause automatic catchup to resurrect
deleted fights. Unsupported/corrupt stores are preserved, never silently reset.

The completed-file native compatibility importer checks logical aliases before
insertion, including schema-1 copies of automatically ingested schema-2 evidence.
An identical fight returns the first retained identity as already present. A
deliberately deleted native fight rejects that compatibility batch with a
controlled suppression error and no partial writes. Source files remain unchanged.

For a newly discovered source, the application prepares a complete empty store
privately and publishes it without replacement through the same root cancellation
and commit gate. An empty log therefore exposes either an absent store or valid
empty history, never an uninitialized public database. Existing invalid databases
remain rejected and unchanged.

Store schema 7 retains the existing native run/fight tables, accepting canonical
native schemas 1 and 2. Schema 1-6 migration preserves original rows, canonical
bytes and hashes. Logical aliases, validated ingest checkpoints, explicit-deletion
suppression and derived receipts occupy separate tables. Automatic schema-2
`source_sha256` identifies retained literal header/context/fight lines joined with
LF, independently from the final growing file. Legacy whole-file hashes keep
their original meaning. Derived checksummed JSON binds raw/source/algorithm and
Catalog identities, Catalog context, and processing generation.

Background consumers calculate supported results without opening a window.
Catalog changes invalidate applicable derived generations using actual source,
Catalog and algorithm identities. Stale work cannot replace newer results;
unchanged inputs reuse results. Authenticated HTTP/MCP queries keep their fixed,
read-only database authority, independent of collection eligibility.

Native duration and observed event/cast/effect counts need no Catalog. Reviewed
damage/healing hit and overflow remain separate. Local-player totals/rates need a
unique observed local PLAYER relation; rates remain unavailable when duration or
quality cannot support them. Effect-change counts do not establish uptime.
Callback-only quickslot/death-recap observations, addon replay, Catalog API joins
and recommendations remain unavailable for this native profile with explicit
reasons. See [Encounter Data and Metrics](../reference/encounter-data-and-metrics.md#native-observations).

## Maintainer validation

Retained contributor utilities inspect/import a completed source into an isolated
store for investigation. They are not a player prerequisite or product interface:

These frozen schema-1 utilities accept normal/separate input. The automatic
schema-2 pipeline supports all four profiles described above.

- `catalog-compiler native-encounter-inspect --input PATH --channel live|pts`
- `catalog-compiler native-encounter-import --input PATH --store PATH --channel live|pts`

Historical addon readers, deterministic replay, store backup and explicit deletion
remain domain utilities. Preserve canonical bytes and hashes during investigation.
Repository tests cover incremental partitions, restart, failure, source generations,
cancellation, retained history and local reads. They do not establish installed-game
flush cadence, platform qualification or callback parity. No owner field-testing
task is a completion gate.
