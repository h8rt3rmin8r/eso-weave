# S130 store and consumer verification

Date: 2026-10-15. Evidence covers repository fixtures and deterministic local
SQLite/service behavior. It is not installed-game or platform field evidence.

## Changed behavior

- Store schema 7 transactionally widens native schema checks to 1/2 while preserving
  old canonical BLOBs, hashes, IDs, session order, addon evidence and exact old
  schema validation. Schema 1-6 compatibility readers remain supported.
- A persistent validated NativeIngestWriter uses a 100 ms busy timeout and commits
  migration, aliases, native evidence, supported native projection and checkpoint
  revision together. The caller admits its one final commit under the shared
  root/token cancellation gate. Failed or omitted admission rolls back.
- Validated source-key/generation metadata, channel/position checks and revision
  compare-and-swap reject stale or contradictory checkpoint progress. Terminal
  generation checkpoints can be archived before shorter replacement generations.
- Logical aliases preserve first accepted physical evidence for copies and reject
  divergent logical identities. Native deletion suppression survives catchup,
  restart and backups. Both automatic and contributor compatibility intake honor
  deletion intent; contributor batches return a controlled error and roll back
  when any logical fight was deliberately deleted.
- Background EncounterProcessor owns existing typed addon projections and
  recommendations, uses actual historical Catalog receipts and records controlled
  unavailable results. Native supported facts remain independent of Catalog.
- Processing examines at most 32 keys per source family per turn, returns at most
  16 pending records and bounds canonical input to 64 MiB. Persistent cursors and
  indexed derivation keys avoid whole-history idle scans. Catalog-context hashes
  cache unchanged paths/receipts and remain stable across restart, preventing
  unchanged rebuild loops. Failed writes do not advance the scan cursor.
- Derived commit admission also serializes with Catalog generation changes.
  Retired picker/import/Refresh/Catalog Update/recording instructions were removed
  from retained domain diagnostics without exposing source values or private paths.

## Test-first evidence

`target/s129-s130-ingest-red.log` records the initial expected unresolved imports
for the new writer/parser interface before implementation.

`target/s129-s130-store-final.log` records a successful run of:

```text
cargo test --locked --test encounter_ingest --test encounter_native \
  --test encounter_import --test encounter_history --test app_encounter_history \
  -- --nocapture
```

The 11 new ingest regressions cover:

1. Atomic raw/projection/checkpoint commit, omitted admission and revision conflict.
2. Legacy-copy aliasing with exact retained bytes, identity and mixed-history order.
3. Native deletion suppression across a copy, new genuine fight and backup/reopen.
4. SQLite busy failure retaining previous checkpoint bytes/revision.
5. Schema-6 migration rollback and successful migration preserving original BLOBs.
6. Missing-Catalog automatic addon derivation, unchanged restart and later Catalog
   activation producing actual projection/recommendation receipts.
7. Shifted copied input preserving first evidence while accepting a later new fight
   in the same logical run, including strict backup validation.
8. Injected checkpoint write failure rolling back tentative raw/projection writes.
9. Divergent logical evidence and inconsistent checkpoint position rejection.
10. Corrupt checkpoint store rejection without database reset or byte changes.
11. Replaced/Truncated terminal checkpoint archives followed by lower-offset new
    generations under durable revision rules and strict reopen validation.

Retained suites cover schema-1-5 fixture migration, old native canonical encoding,
metadata/tampering rejection, preserved calculations and domain reader behavior.
Legacy fixture builders now remove schema-7-only objects when constructing old
schemas, preserving their original strict-schema intent.

## Remaining delivery gates and boundaries

Central full formatting, all-target/all-feature Clippy, locked full Rust tests,
HTTP/MCP and runtime integration, docs/policy/visual checks and final-head hosted
gates remain owned by the primary delivery workflow. Earlier Clippy feedback on
the store checkpoint condition was fixed; its final central result is separate
from the focused tests above. Normal/inline and verbose producer support remain
owner-deferred to #296 and #297. No release/tag, merge or owner field-testing task
was performed by this workstream.

## Retired Catalog presentation worker

The final #292 cleanup removed `src/catalog_update/worker.rs` and its public
presentation-worker exports. The remaining consumers were two tests in
`tests/catalog_update.rs`; their meaningful assertions now call the retained
CatalogUpdateService directly. They prove saved selection remains usable when
candidate inventory fails, and a missing candidate produces a redacted error
without changing selection or claiming a successful installation receipt.
The obsolete source-include assertion for the deleted worker was removed; actual
Catalog capture/source-preservation domain tests remain.

`target/s129-s130-worker-retired.log` records 20 passing Catalog service tests.
No replacement manual worker or presentation architecture was added.

The worker-only candidate lookup, terminal-event receipt writer/sequence/lock
helpers and their scheduling-specific concurrency test were subsequently removed.
Successful domain receipts, source preservation, cross-process operation locks,
recovery, cancellation and selection tests remain. Current diagnostics state
unchanged, incomplete, unsupported or mismatched Catalog facts without player
collection commands, manual selection or rollback instructions.

`target/s129-s130-worker-service-final.log` records the final 20 passing Catalog
service tests after that helper cleanup. `cargo fmt --all` completed successfully;
final shared formatting/Clippy/full-test gates remain with the primary workflow.

## Codex review round 1, compatibility intake identity

The P1 finding reproduced in two failing-first regressions:
`target/s129-s130-review1-store-red.log`. A schema-1 contributor import of already
retained automatic schema-2 evidence inserted another raw row, and the same path
could recreate deleted schema-2 history.

Both intake paths now call one logical disposition check before inserting any
run, raw fight or first-import order. A matching retained alias returns its actual
session ID, encounter ID and canonical hash with AlreadyPresent; first canonical
bytes and ordering remain authoritative. Tombstoned evidence produces a redacted
deletion-suppressed error and rolls back the whole contributor batch, including
otherwise-new fights. No suppression is misreported as stored or already present,
and no report schema change was needed. Copies repeated within one contributor
batch also share one logical preflight identity; divergent fingerprints fail
atomically. The earlier legacy resurrection assertion now proves deletion intent,
database non-mutation and exact source preservation.

`target/s129-s130-review1-store-green.log` records both new regressions passing with
the retained native suite. `target/s129-s130-review1-store-final.log` records the
expanded focused rerun: 75 passed (14 ingest, 23 native, 27 import and 11 history),
including mixed deleted/new batch rollback and duplicate logical runs. Formatting completed through the hidden foreground runner; final
shared gates and PR review disposition remain with the primary workflow.
