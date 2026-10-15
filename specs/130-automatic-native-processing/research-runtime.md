# S130 runtime and consumer research

Research date: 2026-10-15. Scope: #290 and #293, coordinated with #289 and #292.
This document records source inspection and proposed design. It does not claim
implementation, producer-format coverage, installed-game behavior, or field evidence.

Scope resolution later on 2026-10-15: the owner approved normal/separate version-15
processing for S130, with normal/inline tracked by #296 and verbose formats by
#297. The producer-grammar dependency described below records the earlier research
snapshot. The final storage/runtime interfaces and current scope are specified in
`contracts/native-ingest-store.md`; all-four support is not claimed for S130.

## Producer-grammar dependency

Native grammar research is still establishing authoritative layouts for all four
serialization combinations requested by #289. The current parser supports a
completed normal version-15 profile. Invented synthetic layouts cannot establish
producer support. The runtime design below can consume a bounded incremental
parser contract, but its final data model, normalization, checkpoint serialization,
capabilities, fixtures and supported-format claims depend on that research.

Do not silently narrow the requested formats or present a partial parser as S130
completion. If authoritative producer evidence does not resolve the requested
grammar, preserve the specific unresolved boundary for the owner decision.

## Existing ownership seams

- `src/main.rs` constructs `EncounterHistoryService` during normal startup and
  starts `MaintenanceRuntime::spawn_with_history` before `eframe::run_native`.
  Transfer remaining calculation responsibilities before removing its
  `with_encounter_history` UI attachment.
- `src/data_maintenance/runtime.rs` already owns a bounded Catalog coordinator and
  serialized worker. `publish_resolution` updates `DatabaseQueryService` and shared
  history catalog paths independently of paint, then coalesces the typed AppModel
  activation. Extend this authority instead of duplicating activation logic.
- `src/data_maintenance.rs::discover_contexts` discovers up to 16 actual source
  contexts and binds Live/PTS attribution to their roots. Derive native Logs from
  each context's AddOns parent; do not use a later UI selection as queued provenance.
- `MaintenanceControl::native_token`, `apply`, `set_roots` and `set_presence`
  already invalidate native work. Only Catalog currently has per-root job tokens.
  Native work also needs per-root cancellation so losing one context does not
  interrupt its healthy peer.
- `src/app/mod.rs::publish_player_state` currently exposes
  `native_worker_implemented: false`, eligibility and persisted preferences.
  Extend public observations additively with actual service state, generation and
  controlled diagnostics after implementation.

## Discovery and bounded scheduling

Windows discovery in `src/beacon/windows.rs::addons_dir` uses
`dirs::document_dir`, preserving redirected Documents known-folder behavior.
Linux `src/beacon/linux.rs::addons_dir` currently chooses the first supported
Steam root and first library containing ESO. Extend the discovery inventory to
enumerate and deduplicate supported Steam roots/library prefixes while retaining
existing override inputs and deterministic injected seams.

Use an independently scheduled native worker so Catalog compilation cannot stall
encounter ingest. Reconcile package eligibility periodically and service sources
in round-robin order. Polling can detect directories and files appearing after
startup without adding platform watcher lifetime complexity. Do not enqueue one
command per line or require a viewer event.

Proposed budgets to settle in the feature plan are 16 contexts, bounded directory
enumeration and retained candidates, 256 KiB input per scheduling turn, one pending
turn per source, bounded parser tail/context/fight/checkpoint state, yielding
catchup turns, and error backoff capped at 60 seconds. Missing or idle logs are
waiting states. Failure of one source must not monopolize the worker.

Open short-lived read-only handles, including Windows read/write/delete sharing.
Read bounded appended prefixes without requiring the growing file's total length
to remain stable. Do not repeatedly invoke the completed-file import path. Bind
each turn to root identity, channel, source generation and control epoch. Compare
identity, length and bounded anchored prefix/checkpoint-window signatures to
detect replacement, truncation and same-length changes. Never join replacement
tails to old parser context.

Keep logging activity separate from available bytes and catchup. Existing file
presence is not proof that native logging is enabled. An `unknown` logging
observation is valid without introducing an outbound addon protocol. Do not change
the game's logging, serialization or anonymity settings.

## Atomic encounter and checkpoint persistence

`src/encounter/store.rs::append_native_set` already preflights immutable identities,
preserves canonical bytes and first-import session ordering, and commits a batch
transactionally. Factor its transaction-level validation/insertion for reuse by
manual compatibility imports and automatic batches.

A proposed store schema v7 adds:

- Native ingest checkpoints per root/source generation: parser/checkpoint version,
  expected revision, physical identity/signatures, consumed byte position, bounded
  parser state and channel.
- Durable native suppression rows keyed by persistent encounter identity and hash.
- Rebuildable derivations keyed by raw content hash, source kind, algorithm version
  and any applicable Catalog semantic identity, with source/generation qualifications.

One `BEGIN IMMEDIATE` transaction contains migration, new native runs/fights,
session order, derivation queue or supported projection state, and checkpoint
revision advancement. Any insertion or checkpoint failure rolls back the entire
unit. A successful count is published only after commit. Failed writes retain old
durable progress and retryable source bytes. Checkpoint revision compare-and-swap
rejects obsolete writers.

A token read just before `COMMIT` does not alone prevent a disable racing between
the read and the commit. Acquire the SQLite transaction first, then enter a small
shared commit/cancellation gate, revalidate epoch and eligibility, and commit under
that gate. `apply`, shutdown and root removal serialize cancellation through the
same gate. Define stop completion after preceding admitted commits finish; later
commits cannot pass admission. Keep database busy retry and work within this gate
bounded, with no filesystem discovery, parsing or whole-store validation inside it.

`store::open_store` currently runs `PRAGMA quick_check` and validates all historical
canonical rows on every open, with a five-second busy timeout. Reusing this path
for each appended chunk makes ingest cost grow with the whole history. Prefer a
worker-owned validated connection, bounded/cancellable initial validation, short
busy retries and transaction-level validation of newly touched rows. Preserve
strict compatibility-reader/backup validation and corrupt-store diagnostics.

Audit version-specific branches in `validate_schema`, `validate_records`,
`validate_session_order`, `list_encounters` and `load_native_encounter`. Several
currently check version 6 exactly. Preserve explicit v6 metadata/schema recognition,
v1-v6 readers, native inclusion in newer stores, and transactional first-write
migration. Never reset or replace an unsupported store with an empty database.

## Legacy identities and canonical evidence

`src/encounter/native.rs::run_id` currently hashes native schema, physical-file run
ordinal, channel and literal BEGIN_LOG line. Encounter identity adds combat ordinal.
Canonical NativeRecord values also retain absolute physical-file record ordinal
and byte offset.

A copy containing only one later run can renumber that run. Reopened or shifted
material can also reuse a logical header while changing canonical offsets. Preserve
existing v6 identities and canonical bytes. Never rewrite stored hashes to make
deduplication pass. The incremental parser must explicitly distinguish exact
copies, shifted evidence and fresh recording generations, and reject conflicting
identity reuse without overwriting good history. Final reconciliation depends on
the parser identity contract and authoritative producer research.

## Deletion, backups and shared readers

`store::delete_encounter` and `delete_all` currently physically remove records and
allow manual reimport. Insert native tombstones in those same deletion transactions
before removing native rows. Automatic preflight suppresses matching deleted
identities while advancing verified progress. Delete-all must retain checkpoints
and suppression so catchup cannot recreate deleted records, while truly new fights
remain eligible. Keep legacy-addon deletion and retained session-snapshot semantics
explicit.

`backup_store` already uses SQLite backup, validates the snapshot and atomically
persists it. Include checkpoint/suppression schema in validation so restored stores
retain coherent progress and deletion intent.

## Background derivation and API integration

`EncounterHistoryService::native_detail` uses `project_native_encounter` for
source-independent facts. `detail` resolves compatible historical Catalog
generations and invokes `calculate_projection`, preserving channel/API/game-version
matching. The UI-owned `EncounterHistoryWorker::LoadDetail` currently triggers these
calculations, and `EncounterDetail::from_projection` triggers recommendations.
Move domain orchestration into the encounter service/background lifecycle before
removing UI worker usage.

Calculate supported native projections automatically after successful input commit,
independent of Catalog availability. Process retained addon inputs using bounded
pages of missing or stale derivation keys after startup or relevant Catalog
activation. Unchanged keys do nothing. Do not scan or rebuild the whole store on
every appended line or frame. Bind derived results to actual raw source, algorithm,
Catalog identity and processing generation; stale completions cannot become current.

Preserve native counts, reviewed hit/overflow totals, qualified local-player facts
and explicit unavailable evidence. Native log version and game-version strings do
not establish an addon API version. Do not invent Catalog joins, callback replay,
effect-uptime parity, death recap or recommendation capabilities.

`DatabaseQueryService` keeps fixed `catalog` and `encounters` paths and independent
read-only query connections. Addon absence and either collection disable preference
must not clear these paths. Catalog swaps use immutable paths, allowing in-flight
queries to finish on their actual generation while new queries use the accepted
generation. Preserve the authenticated opt-in HTTP/MCP operations, identifiers,
bounds and read-only authority. Document and test additive service fields/aliases
and independently advertised storage schema evolution.

Remove obsolete workflow instructions from `history.rs` diagnostics, including
legacy recording commands, manual inspection/import, Catalog Update, Refresh and
select-again guidance. Preserve bounded diagnostic codes and raw-value restraint.

## Required evidence

Use deterministic fixtures for v1-v6 migration and exact retained bytes/hashes/order,
crash before/after commit, checkpoint conflicts, deletion/copy resurrection,
cross-environment roots, simultaneous Catalog failure, disable immediately before
commit, root removal, concurrent read/backup, malformed stores and bounded shutdown.
Test append/partial UTF-8, delayed flush, restart, rotation/replacement/truncation and
source byte preservation through every error journey. These tests establish only
the supplied fixtures and repository behavior, not actual game flush cadence or
installed platform behavior. Authoritative all-four producer-format support remains
an unresolved prerequisite in this research snapshot.
