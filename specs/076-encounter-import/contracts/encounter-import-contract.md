# Contract: Encounter Import and Raw Store

## Import Boundary

`import_encounter(request)` is an explicit, synchronous, non-interactive
operation. It reads one file, never executes it, validates the full S075 terminal
contract, canonicalizes accepted facts, and commits at most one raw record.

The call succeeds only when:

1. The stable source is a regular no-follow file at most 64 MiB.
2. The opened handle remains within the resolved parent boundary and its length
   and modification metadata remain stable for the complete read.
3. The root assignment is exactly `EsoWeaveEncounterSaved`.
4. The restricted table grammar and parser work limits pass.
5. The envelope is schema version 1, addon version 1, terminal, and privacy
   profile `anonymous-local-v1`.
6. The expected and captured Live or PTS channels match.
7. Every event, payload, count, sequence, monotonic value, discontinuity, and
   terminal field passes the invariant and privacy validators.
8. The dedicated store is new and empty or is an intact schema-v1 encounter
   store.

Validation failure returns an error before store mutation. A database failure
rolls back the transaction.

## Restricted SavedVariables Grammar

Accepted syntax is one assignment followed by optional whitespace:

```text
EsoWeaveEncounterSaved = {
    ["schema_version"] = 1,
    ["events"] = {
        [1] = { ["kind"] = "encounter-start", ... },
        ...
    },
    ...
}
```

Only table literals, bracketed string/integer keys, quoted strings, integers,
booleans, and nil are data. Comments, identifiers as values, calls, operators,
long strings, metatables, repeated keys, sparse arrays, trailing statements, and
all executable Lua are rejected.

An empty table is interpreted as an object for the encounter schema, permitting
an empty warning map. Event arrays cannot be empty in a terminal capture.

## Terminal Validation

- Event length equals `stored_event_count` and is 2 through 100,000.
- First and last stored sequences equal envelope bounds.
- Every event repeats the envelope session and encounter IDs.
- Event sequences strictly increase and monotonic values never decrease.
- The first event is `encounter-start` at sequence 1 with monotonic value 0.
- The final event is `encounter-end`; its monotonic value equals
  `ended_monotonic_ms`.
- Each sequence gap is immediately followed by one discontinuity declaring that
  exact gap. A discontinuity without a gap is invalid.
- Declared missing intervals are ordered, non-overlapping, and their inclusive
  lengths sum to `omitted_event_count`.
- `last_sequence - first_sequence + 1` equals stored plus omitted counts.
- Complete status has no partial reason, no omissions, a complete end payload,
  and reason `combat-ended`.
- Partial status has an allowed partial reason, an incomplete end payload, and a
  matching terminal reason. It may have no omission for user stop, deactivation,
  or callback failure.
- Unknown numeric identifiers are valid when their numeric domains pass.

## Canonical Format Version 1

Canonical bytes are compact JSON encoded as UTF-8 without BOM or trailing
newline. Envelope, source, and event fields use declared Rust structure order.
Warning and payload keys use lexical byte order. Only validated values enter the
serializer. `SHA-256(canonical_bytes)` is the raw record content identity.

## Store Contract

- The store is a separate SQLite database with `user_version = 1` and the exact
  schema in `data-model.md`.
- One immutable record exists per canonical content hash and semantic encounter
  identity.
- The supported API has no raw update operation, and a database trigger rejects
  SQL updates.
- An exact canonical duplicate returns `already-present` without mutation.
- A semantic identity collision returns an error without mutation.
- List order is deterministic and payload-independent.
- Delete-one and delete-all require distinct explicit calls and run in one
  transaction.
- No operation prunes records automatically.

## Backup Contract

`backup_encounter_store(source, destination)` checks the source store, creates a
consistent SQLite snapshot at a temporary sibling path, closes and validates the
snapshot, hashes its bytes, and atomically replaces the destination. Source and
destination must be distinct. Failure before atomic publication leaves any prior
destination intact.

## CLI Contract

```text
catalog-compiler encounter-import --input PATH --store PATH --channel live|pts
catalog-compiler encounter-list --store PATH
catalog-compiler encounter-backup --store PATH --output PATH
catalog-compiler encounter-delete --store PATH --session ID --encounter ID
catalog-compiler encounter-delete --store PATH --all
```

Successful commands emit JSON receipts to standard output. Errors use standard
error handling and a nonzero exit status. Commands do not prompt, scan, upload,
or print raw event payloads.

## Privacy and Confinement

- No player, account, guild, chat, location, free-form note, or telemetry field
  is accepted.
- String payloads are limited to the exact emitter token sets.
- Unknown identifiers remain numeric and local to the raw capture.
- Input, store, and backup bytes remain on the local filesystem.
- Import has no relationship to input synthesis, automation, Pixel Bus, addon
  lifecycle, or catalog promotion.
