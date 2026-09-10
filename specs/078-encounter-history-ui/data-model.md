# Data Model: Quality-Aware Encounter History UI

## EncounterHistoryService

- `store_path`: app-owned raw store path
- `catalog_path`: current accepted catalog path
- Operations: snapshot, import, detail, delete one, delete all
- Invariant: operations compose existing S076/S077 APIs and never mutate catalog or
  raw records during projection

## EncounterIdentity

- `session_id`: stable session identity
- `encounter_id`: stable encounter identity
- Invariant: both values must match an indexed immutable raw record

## HistorySnapshot

- `encounters`: deterministic `EncounterSummary` sequence
- Empty is a valid state for a missing store or a valid store with no records
- Invariant: snapshot data contains no event payloads or personal identity

## EncounterDetail

- `projection`: one S077 `EncounterProjection`
- Lifetime: disposable in-memory UI state
- Invariant: catalog and raw hashes, versions, quality, loss, and unknown IDs remain
  unchanged from the projection authority

## HistoryDiagnostic

- `kind`: source unavailable, store invalid, catalog unavailable, catalog invalid,
  version mismatch, encounter missing, or operation failed
- `message`: stable path-free user-facing explanation
- Invariant: no raw event, personal name, or local path is included

## WorkerCommand

- `Refresh`
- `Import { source_path, expected_channel }`
- `LoadDetail { identity }`
- `DeleteOne { identity }`
- `DeleteAll`
- `Stop`

Only one non-stop command may wait behind the active command.

## WorkerEvent

- `Snapshot { encounters, message }`
- `Detail { identity, result }`
- `Failed { operation, diagnostic }`

UI busy state begins when a command is accepted and ends on exactly one terminal event.

## DeleteConfirmation

- None
- One encounter identity
- All encounter records

No storage operation occurs on entry to confirmation state. Only the affirmative
control dispatches a worker command.
