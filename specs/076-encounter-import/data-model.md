# Data Model: Encounter SavedVariables Import

## Encounter Import Request

| Field | Type | Rules |
| --- | --- | --- |
| `input_path` | filesystem path | Existing regular file, no symlink or reparse point, stable bounded read |
| `store_path` | filesystem path | Caller-selected dedicated SQLite file, distinct from input |
| `expected_channel` | enum | `live` or `pts`, must equal the capture |

The input's canonical parent is the bounded-read root. No directory traversal or
background scanning is performed.

## Canonical Encounter Capture

The canonical capture mirrors S075 schema version 1 with deny-unknown typed
structures:

- envelope versions, terminal status, channel, privacy profile, source provenance
- opaque session and encounter IDs
- wall-clock strings and monotonic integer bounds
- first and last sequence, stored and omitted counts, byte estimate
- optional partial reason, ordered warning counts, and ordered events
- each event's repeated identity, sequence, monotonic time, kind, and ordered
  scalar payload

Canonical format version 1 is compact UTF-8 JSON with no trailing newline. Struct
fields use contract order. Warning and payload object keys use lexical byte order.
All numbers are signed 64-bit integers during parsing and validated into their
nonnegative domain before storage.

## Event Payload Contract

| Kind | Required keys | Optional keys | String tokens |
| --- | --- | --- | --- |
| `encounter-start` | `reason` | none | `combat-started` |
| `encounter-end` | `reason`, `complete` | none | terminal reason set |
| `damage`, `healing` | `result`, `ability_id`, `amount`, `source_actor`, `target_actor` | `overflow`, `power_type`, `damage_type`, `source_type`, `target_type` | none |
| `effect` | `change_type`, `ability_id`, `stack_count`, `target_actor` | `begin_ms`, `end_ms`, `source_type`, `effect_type`, `ability_type`, `status_effect_type` | none |
| `resource` | `actor`, `power_type`, `value`, `maximum`, `effective_maximum` | none | none |
| `cast` | `slot`, `ability_id` | none | none |
| `bar-change` | `active_pair`, `locked` | none | none |
| `death`, `resurrection` | `actor`, `source` | `source_actor`, `result`, `ability_id` as one complete combat-result group | numeric actor source or `player-event`, `combat-result` |
| `boss-health` | `boss_index`, `actor`, `value`, `maximum`, `effective_maximum` | none | none |
| `performance` | `frames_per_second`, `latency_ms` | none | none |
| `quickslot` | `action`, `slot`, `ability_id` | none | `selected`, `used` |
| `discontinuity` | `missing_sequence_from`, `missing_sequence_to`, `reason` | none | partial reason set |

All numeric payloads are nonnegative integers. Actor fields are at most 4,096.
Boss index is 1 through 6. An end payload's `complete` value and `reason` must
match the envelope terminal state.

## Raw Encounter Record

```sql
CREATE TABLE encounter_store_meta (
    singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
    schema_version INTEGER NOT NULL CHECK (schema_version = 1),
    canonical_format_version INTEGER NOT NULL CHECK (canonical_format_version = 1)
);

CREATE TABLE raw_encounters (
    content_sha256 TEXT PRIMARY KEY CHECK (length(content_sha256) = 64),
    source_sha256 TEXT NOT NULL CHECK (length(source_sha256) = 64),
    session_id TEXT NOT NULL,
    encounter_id TEXT NOT NULL,
    channel TEXT NOT NULL CHECK (channel IN ('live', 'pts')),
    capture_schema_version INTEGER NOT NULL CHECK (capture_schema_version = 1),
    addon_version INTEGER NOT NULL CHECK (addon_version = 1),
    status TEXT NOT NULL CHECK (status IN ('complete', 'partial')),
    started_at TEXT NOT NULL,
    finished_at TEXT NOT NULL,
    first_sequence INTEGER NOT NULL,
    last_sequence INTEGER NOT NULL,
    stored_event_count INTEGER NOT NULL,
    omitted_event_count INTEGER NOT NULL,
    canonical_json BLOB NOT NULL,
    UNIQUE (session_id, encounter_id)
);

CREATE TRIGGER raw_encounters_no_update
BEFORE UPDATE ON raw_encounters
BEGIN
    SELECT RAISE(ABORT, 'raw encounter records are immutable');
END;
```

`PRAGMA user_version = 1` is an independent fast schema gate. The metadata row
provides queryable schema and canonical format ownership.

## Import Receipt

| Field | Type | Meaning |
| --- | --- | --- |
| `outcome` | enum | `imported` or `already-present` |
| `source_sha256` | 64-char hex | Stable source-byte digest for this attempt |
| `content_sha256` | 64-char hex | Canonical content identity |
| `channel` | enum | Preserved source channel |
| `status` | enum | `complete` or `partial` |
| `session_id` | string | Opaque capture identity |
| `encounter_id` | string | Opaque encounter identity |
| `stored_event_count` | integer | Canonical event length |
| `omitted_event_count` | integer | Declared source loss |

No event payload or filesystem path appears in the receipt.

## Encounter Summary Record

The list operation returns stored metadata plus content hash in deterministic
`started_at`, `session_id`, `encounter_id` order. It does not deserialize or
execute payloads and is not a derived metric.

## Backup Receipt

| Field | Type | Meaning |
| --- | --- | --- |
| `schema_version` | integer | Encounter store schema, currently 1 |
| `byte_length` | integer | Final snapshot file length |
| `sha256` | 64-char hex | Digest of the final snapshot bytes |

The receipt deliberately omits local paths.

## State Transitions

```text
missing store -> initialize v1 -> valid store
valid input + valid store -> transaction insert -> imported
same canonical input -> no mutation -> already-present
same identity + different content -> no mutation -> collision error
invalid input/store -> no mutation -> typed error
valid store -> snapshot temp -> hash -> atomic publish -> backup receipt
valid store -> explicit transaction delete -> valid store
```

Raw records have no update transition. Future schema migration must first create
a verified backup and then use one transaction; S076 performs no migration.
