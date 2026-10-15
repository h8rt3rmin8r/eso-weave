# Native ingest and retained-store contract

Date: 2026-10-15. Slice: S130. This is a design contract, not implemented API.
Owner scope resolution: normal/separate version-15 processing is in S130;
normal/inline is tracked by #296 and verbose formats by #297. Unsupported profiles
remain explicit capabilities, without requesting a game setting change.

## Ownership

The application runtime owns discovery, source generations, source signatures,
parser instances, eligibility and cancellation. The encounter domain owns validated
storage, durable checkpoints, immutable raw identities, deletion suppression and
supported derivations. Neither domain depends on a viewer, selected encounter or
UI event. The parser remains independent of storage and collection eligibility.

The store writer does not open or mutate game source files. The runtime never
acknowledges parser progress until the writer commits it. A returned successful
receipt refers only to durable database state.

## Proposed public Rust interfaces

Names below are agreed integration seams; exact parser payload names follow the
incremental parser contract. All public debug output omits raw checkpoint bytes,
source lines and private paths.

```rust
pub struct NativeIngestCheckpoint {
    pub source_key: String,
    pub generation: String,
    pub revision: u64,
    pub consumed_bytes: u64,
    pub channel: Channel,
    pub checkpoint_bytes: Vec<u8>,
    pub checkpoint_sha256: String,
}

pub struct NativeIngestBatch {
    pub source_key: String,
    pub generation: String,
    pub expected_revision: Option<u64>,
    pub consumed_bytes: u64,
    pub checkpoint_bytes: Vec<u8>,
    pub completed: Option<NativeCompletedFight>,
}

pub struct NativeIngestReceipt {
    pub revision: u64,
    pub consumed_bytes: u64,
    pub imported_count: usize,
    pub already_present_count: usize,
    pub suppressed_count: usize,
}

impl NativeIngestWriter {
    pub fn open(store_path: impl AsRef<Path>) -> Result<Self, EncounterError>;

    pub fn checkpoint(
        &self,
        source_key: &str,
    ) -> Result<Option<NativeIngestCheckpoint>, EncounterError>;

    pub fn commit_batch<F>(
        &mut self,
        batch: NativeIngestBatch,
        finalize: F,
    ) -> Result<NativeIngestReceipt, EncounterError>
    where
        F: FnOnce(&mut NativeCommit<'_>) -> Result<(), EncounterError>;
}

impl NativeCommit<'_> {
    pub fn commit(&mut self) -> Result<(), EncounterError>;
}
```

`NativeIngestWriter` owns a persistent SQLite connection. `open` validates an
existing supported store once, preserving every raw row and metadata value.
Unsupported/corrupt stores produce a controlled failure without replacement or
schema mutation. Validation checks cancellation between bounded rows and uses a
SQLite progress handler where needed; the implementation may provide an
`open_with_cancellation` variant consumed by the worker. Database busy waiting is
capped at 100 ms per attempt. The runtime schedules retries rather than a writer
internally waiting indefinitely. Opening an absent store may prepare an empty
SQLite file, but its schema and metadata are installed in the first successful
write transaction; cancellation/failure cannot publish migrated state alone.

The writer retains validated state and does not rerun history-wide JSON/hash
validation for each appended prefix. It validates every new payload, checkpoint
and identity association. Reopening/replacing the database requires validation
again. Existing compatibility-reader and backup validation remains strict.

New-store construction uses open_with_finalizer: prepare, commit and close a full
empty schema7 database privately, sync it, then publish without replacement under
the same NativeCommit admission gate. Public readers see absence or a valid store
even for empty logs. Rejected/omitted admission discards only the private candidate;
existing destinations and existing invalid databases are never overwritten or reset.

`source_key` and `generation` are bounded lowercase SHA-256 identifiers generated
by runtime from actual root/path and verified source-generation evidence. The
runtime never derives them from a transient selected environment. Source/generation
association remains runtime-owned metadata/witness evidence. The pure checkpoint
payload validates channel and NativePosition without filesystem dependencies.
`consumed_bytes` must agree with the parser payload, not exceed its
declared source position, and not move backward within one generation.

The checkpoint byte cap and completed-fight byte cap are imported from the parser
contract. A feed accepts at most 256 KiB and stops immediately after one completed
fight, returning the accepted prefix so the runtime resubmits any suffix. Thus a
single feed cannot multiply retained fight evidence by a batch count. The parser
contract declares the encoded checkpoint bound, including escaping overhead; the
store rejects oversize input before starting a write.

`expected_revision: None` requires an absent checkpoint. `Some(n)` requires the
currently stored revision to equal n, including when replacing an older source
generation. Success advances the revision once. Existing checkpoints are never
silently reset to revision zero. A new generation can replace progress only by
this compare-and-swap boundary, preserving old raw observations and suppressions.

## Commit admission and stop boundary

The runtime calls the writer using this shape:

```rust
writer.commit_batch(batch, |commit| {
    control.with_native_commit(&root, &token, || commit.commit())
})
```

The control signature is:

```rust
pub fn with_native_commit<T>(
    &self,
    root: &Path,
    token: &CancellationToken,
    commit: impl FnOnce() -> Result<T, EncounterError>,
) -> Result<T, EncounterError>;
```

`register_native_job(root)` returns a per-root token. Cancelled tokens never reset.
The shared ControlState mutex serializes final root/token/eligibility checking and
SQLite commit with disable, addon removal, root replacement and shutdown. Work
prepared under an obsolete or cancelled token is rejected. The gate never performs
source discovery, parsing, Catalog compilation or whole-store validation.

The store acquires `BEGIN IMMEDIATE` before calling the admission closure. It
prepares all writes first, then gives the closure one `NativeCommit` containing
the transaction. `NativeCommit::commit` consumes that transaction exactly once.
Failure, an omitted commit, repeated commit or closure error cannot return a
successful receipt. Uncommitted state rolls back when the transaction drops.

A stop returns after any previously admitted commit has finished and its token is
cancelled under the same gate. After that completed boundary no later commit
from that root/token can become durable. A bare cancellation-token check followed
by an unlocked commit does not satisfy this contract.

No parser instance becomes the accepted in-memory state until commit success.
On busy, disk-full, I/O failure, revision conflict or admission rejection, reload
the durable checkpoint before subsequent work or retain the unchanged accepted
instance. Never consume more source bytes based on a failed checkpoint.

## Transaction contents and schema compatibility

The next store schema is 7. Reuse NativeRun and NativeEncounter with explicit
schema-1/schema-2 validation branches. Retain schema versions 1-6 as supported
readers. The existing native table constraints currently require schema 1; rebuild
those tables transactionally with schema IN (1,2), copying every existing column
and canonical BLOB exactly and restoring immutability triggers. Preserve exact
schema-6 SQL constants for old-schema validation. Do not rewrite old raw rows.

New schema authorities are:

| Authority | Contents |
| --- | --- |
| `native_runs` | Immutable schema-1/schema-2 run header and canonical bytes/hash |
| `native_encounters` | Immutable schema-1/schema-2 fight, canonical bytes/hash and preserved summary indices |
| `native_logical_aliases` | Logical run/fight key and normalized evidence fingerprint to the retained physical session/fight IDs and raw hash |
| `native_ingest_checkpoints` | Source key, generation, revision, consumed position, bounded validated checkpoint bytes/hash |
| `native_deleted_encounters` | Persistent logical identity/fingerprint and retained raw identity/hash suppression |
| `encounter_derivations` | Rebuildable native/addon result with actual raw, algorithm, optional Catalog and processing identities |

These are application-owned local tables. No new HTTP/MCP mutation endpoint is
added. Ingest provenance is stored with checkpoint/alias metadata, not by
fabricating source content. An unfinished growing file has no final whole-file
hash. The new source receipt must explicitly identify the bounded source evidence
being hashed; retained legacy `source_sha256` values preserve their original
meaning and bytes exactly. The immutable canonical content hash remains distinct.

One transaction performs supported first-write migration, required legacy alias
backfill, identity/collision checks, new immutable raw rows, first-import ordering,
native supported projection rows, and checkpoint compare-and-swap. It commits all
or none. A source batch with no completed fights may still commit a validated
checkpoint. Tombstoned fights may advance progress without adding raw rows and
without being counted as present or newly stored.

Schema checks must continue accepting exact old version metadata, SQL objects and
canonical data. Audit version-6 equality checks in schema validation, native
validation, mixed summary listing, session ordering and native loading so schema
7 includes both native profiles. Unsupported versions never reset the database.

## Logical aliases and collision behavior

The parser supplies NativeCompletedFight containing the reused NativeEncounter
with schema_version 2 and NativeLogicalIdentity {run_key, fight_key,
evidence_sha256}. Physical source coordinates and run ordinal remain in the reused
canonical record fields as provenance, using checked u64 counters without a
lifetime-file-size cap. Logical run identity binds actual channel, epoch, log
version, realm, language and game version independently of placement. Logical fight
identity binds the logical run and combat ordinal.

The parser exposes a shared legacy-normalization helper that derives the same
logical keys and normalized evidence fingerprint from a schema-1 NativeEncounter.
It converts file-relative record positions to run-relative coordinates only in
temporary comparison values. Original schema-1 canonical bytes, content hashes,
session IDs, encounter IDs and first-import ordinals remain immutable.

An identical logical key and normalized fingerprint links to the existing raw
identity through an alias, including legacy rows. An identical logical key with
different evidence is a controlled collision. It must not overwrite evidence,
invent a second accepted identity or report the divergent fight already present.
Alias backfill is part of the successful migration transaction and preserves
established history ordering. Existing native readers retain their return types
and validate the declared schema branch; the projector reuses the same supported
metric logic for both evidence versions.

## Deletion and backups

Retained native deletion inserts suppression before raw deletion in the same
transaction. Suppression contains logical identity/fingerprint and retained raw
identity/hash. Automatic catchup and copies consult it before insertion. Delete-all
retains checkpoints, aliases needed for suppression, and tombstones. New genuine
fight identities remain eligible. Legacy addon deletion keeps its existing source
semantics. Removing a viewer does not invoke deletion.

The retained completed-file native compatibility importer consults these same
logical aliases before inserting schema1 rows. An existing logical fight returns
the first retained identity as already present. A deleted logical fight rejects
the compatibility batch with a controlled suppression error and rolls back the
whole transaction; it cannot resurrect evidence or add partial run/order rows.

Deleting raw observations also invalidates their rebuildable derivations. Backup
validation includes all new tables so restored checkpoints and deletion intent
remain coherent. SQLite read-only clients and backup see an old or new committed
state, never a partially advanced checkpoint.

## Derivation and retained service interfaces

The encounter domain exposes source-specific retained native evidence and a
bounded processing turn, consumed by the application worker on startup, new input
and accepted Catalog activation, returning processed/skipped/deferred counts and
controlled diagnostics with `limit` capped at 16. The implemented seam is `history.processor()` (or
`processor_with_cancellation(token)`) followed by the worker-owned
`EncounterProcessor::process_pending(limit, finalizer)`. It retains its validated
SQLite connection, Catalog context cache and round-robin key cursor. Each turn
examines at most 32 keys per source family (64 total), returns at most 16 pending
results and reads at most 64 MiB of canonical input. Missing-result selection is
indexed by raw/source/algorithm/context. An idle turn never scans the whole
history or reopens/revalidates every Catalog. Failed writes retain the old cursor
for retry; reached tails wrap so newly inserted or reused row IDs remain visible.

Native projection uses schema-specific validated evidence and remains available
without Catalog. Store native projection JSON atomically with the accepted fight
or rebuild it using its raw/algorithm key after restart. Retained addon projection
uses existing `detail` version-compatible Catalog resolution and moves
recommendation orchestration out of UI-only EncounterDetail. Persisted results
must identify the exact raw hash, source kind, algorithm version, Catalog semantic
identity when used, and processing epoch. An obsolete Catalog/epoch completion
cannot replace current availability or a newer derivation.

Catalog activation retains immutable paths and last-valid generations. Existing
HTTP/MCP database discovery and bounded authenticated read-only queries stay
available with collection disabled or the combined addon absent. Native unavailable
evidence remains explicit; normal/separate support is not a claim of inline or
verbose support or addon callback parity.

## Required regressions

Before implementation acceptance, prove atomic batch/checkpoint migration and
failure rollback; omitted/late/repeated commit admission; revision conflicts;
disable/removal/shutdown races; source generations and root attribution; exact
legacy canonical bytes/IDs/order; copied legacy alias deduplication and divergent
identity rejection; deletion/copy suppression across restart/backup; schema-1-6
read compatibility; corrupt-store non-mutation; native facts without Catalog;
bounded pending derivation with unchanged inputs; and concurrent bounded local
reads. Tests use authoritative normal/separate fixtures and synthetic scheduling,
without claiming producer coverage for #296/#297 or installed-game evidence.
