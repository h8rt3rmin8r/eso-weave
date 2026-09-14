# Data Model: Persistent Data Addon Foundation

## Data addon package

| Field | Rule |
| --- | --- |
| Product title | `ESO Weave Data` |
| Stable identity | `EsoWeaveData` |
| Installed directory | `AddOns/EsoWeaveData/` |
| Manifest | `EsoWeaveData.txt` |
| Source inventory | `EsoWeaveData.lua`, `Catalog.lua`, `Encounter.lua` |
| Managed marker | `## X-ESO-Weave-Data-Managed: true` |
| SavedVariables | `SavedVariables/EsoWeaveData.lua` |
| Root | `EsoWeaveDataSaved` |

The package version owns the complete four-file artifact. A managed install or
update is all-or-nothing. Any link, missing marker, non-regular entry, or foreign
file makes the target unmanaged and prevents mutation.

## SavedVariables envelope

```text
EsoWeaveDataSaved
├── schema_version: positive integer
├── addon_version: positive integer
├── catalog: catalog module envelope
└── encounter: encounter module envelope
```

The outer reader accepts no executable Lua, mixed table keys, duplicate keys,
unsupported escapes, excessive nesting, or input beyond 128 MiB. Selecting a
module does not relax that module's existing schema, identity, content, or
size checks.

### Catalog subtree

The existing collector envelope moves unchanged beneath `catalog`. It retains
its schema version, collector version and checksum, status, channel, source
provenance, selection, coverage, warnings, checkpoint, and bounded chunks.

States remain:

```text
idle -> running -> complete
           |       |
           v       v
         paused  imported
           |
           +-> running
           +-> cancelled
```

Combat or player deactivation pauses active work. Only explicit user commands
start, resume, cancel, or clear the catalog subtree.

### Encounter subtree

The existing encounter envelope moves unchanged beneath `encounter`. It retains
its schema version, addon version, channel, privacy profile, source provenance,
session and encounter identities, sequence and time bounds, loss counters,
warnings, and bounded events.

States remain:

```text
idle -> armed -> capturing -> complete
  ^       |          |            |
  |       v          +---------> partial
  +---- disarmed                    |
  +-------------- cleared <---------+
```

Only explicit `/ewencounter` commands change the requested state. Capture event
handlers and the sampling update exist only while `capturing`.

## Module isolation invariants

1. Catalog operations may replace only `EsoWeaveDataSaved.catalog`.
2. Encounter operations may replace only `EsoWeaveDataSaved.encounter`.
3. Clearing either subtree preserves the other module's semantic values; ESO may
   reorder table serialization during a later flush.
4. Neither module may call PixelBus, input, automation, upload, process-memory,
   packet, equipment, item-use, or navigation surfaces.
5. Package lifecycle may replace files only after proving package ownership and
   must rollback the complete prior managed file set on failure.

## Ingestion decision

| Field | Meaning |
| --- | --- |
| Candidate | Native incremental log, native terminal log, SavedVariables terminal import, or bounded hybrid |
| Evidence class | Official source, repository fixture, Windows operator run, or Linux/Proton operator run |
| Coverage | Observed event families plus explicit gaps |
| Cadence | p50, p95, maximum, and post-boundary availability |
| Lifecycle | Create, append, flush, rotate, truncate, replace, reload, relog, exit, crash, recover |
| Performance | File growth, CPU, memory, and parser backlog |
| Privacy | Game transformations and ESO Weave minimization requirements |
| Status | Provisional, field-qualified, rejected, or blocked |

No candidate becomes field-qualified without independent Windows and
Linux/Proton receipts. Unknown or malformed records lower completeness rather
than becoming silently ignored evidence.

## Command decision

| Field | Meaning |
| --- | --- |
| Candidate | One documented desktop-to-addon mechanism |
| Directionality | Whether information can actually enter the addon |
| Latency | Immediate, reload-bound, next-launch, or unavailable |
| Persistence | Local, server-associated, unknown, or none |
| Visibility | Publicly documented account or client exposure |
| Reliability | Delivery preconditions, acknowledgement, conflicts, and failure |
| Platform behavior | Windows and Linux/Proton support limits |
| Decision | Go, no-go, or explicit prerequisite |

S092's selected state is no-go with zero approved desktop commands. User-driven
slash controls remain the operational control path.
