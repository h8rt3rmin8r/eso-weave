# Data Model: Encounter Capture Modes

## Capture controller

`EncounterModuleState` is the new `EsoWeaveDataSaved.encounter` outer object.

| Field | Rule |
| --- | --- |
| `state_schema_version` | exactly 1 |
| `addon_version` | exactly 4 |
| `selected_mode` | `single` or `continuous` |
| `selected_channel` | `live`, `pts`, or absent before selection |
| `requested_mode` | selected mode while enabled, otherwise absent |
| `active_mode` | requested mode while waiting/capturing, absent while interrupted/stopped/failed |
| `state` | `stopped`, `waiting`, `capturing`, `interrupted`, or `failed` |
| `session` | one session object while retained evidence or authority exists |
| `current` | at most one nonterminal capture while `capturing` |
| `records` | terminal captures keyed by zero-padded ten-digit ordinal |
| `interruptions` | ordered bounded gap markers |
| `stop_reason` | controlled reason only while stopped |
| `failure` | controlled reason only while failed |
| `revision` | positive monotonic state revision |

Unknown fields and inconsistent combinations are rejected. Off is represented
by absent requested and active modes plus `stopped`; it is not a third mode.

## Capture session

| Field | Rule |
| --- | --- |
| `session_id` | 1-128 safe ASCII bytes, stable for one enablement period |
| `mode` | immutable selected mode |
| `channel` | immutable `live` or `pts` |
| `status` | `active`, `stopped`, or `failed` |
| `started_at` | validated decimal timestamp |
| `finished_at` | absent while active, required when stopped/failed |
| `next_encounter_ordinal` | completed count plus one, or current ordinal plus one |
| `completed_encounter_count` | exact terminal record count |
| `degraded_encounter_count` | exact partial terminal record count |
| aggregate counts | checked sums of terminal plus current evidence |
| `interruption_count` | exact marker count |

Session mode, channel, and identity must match every terminal record and current
status. Aggregate counts cannot exceed the fixed spool ceilings.

## Session encounter

- `ordinal` is the numeric value of its ten-digit map key and is contiguous from 1.
- `capture` is an unchanged terminal `EncounterCapture` with schema version 2
  and addon-format version 3.
- `capture.session_id` equals the outer session identity.
- `capture.encounter_id` is unique and bounded; order never depends on its text.
- Complete callback-started records retain S095 verified replay.
- Mid-combat records use `started-mid-combat`, are partial, and remain replay-indeterminate.
- A valid active record recovered after runtime interruption terminates with the
  controlled `runtime-interrupted` partial reason and a counted
  `recovered_interruption` warning before session recovery proceeds.

Legacy singleton schema-v1 and schema-v2 captures adapt to single mode, ordinal
1 for storage metadata only. Their canonical bytes are never rewritten.

## Current record

The addon retains its bounded working capture only while capturing. The desktop
accepts the bounded object as historical status input but never imports or
canonicalizes it as terminal evidence. Controlled outer metadata supplies the
only displayed current ordinal/state; raw current values never reach UI or logs.

## Interruption marker

| Field | Rule |
| --- | --- |
| `sequence` | contiguous positive marker sequence |
| `occurred_at` | validated decimal timestamp |
| `after_encounter_ordinal` | zero through completed encounter count |
| `reason` | `player-deactivated` or `runtime-interrupted` |

Markers identify an unobserved gap without estimating missing encounters or observations.

## Session failure

Failure is a value-free controlled record with `reason`, `occurred_at`, and an
optional valid encounter ordinal. Reasons are `storage-pressure`,
`callback-failed`, `clock-reset`, `terminal-reserve-exhausted`,
`interruption-limit`, or `state-invalid`. Failed sessions have no requested or
active mode and never resume automatically. A malformed controller that cannot
be proven safe remains byte-for-value preserved outside this valid-state model;
the runtime treats it as inactive `state-invalid` rather than rewriting fields.

## Batch import report

The report contains source hash, imported count, already-present count, ordered
per-record receipts, and a bounded last-saved session summary. It contains no raw
event values or hostile identifiers before validation.

## SQLite store v4

`raw_encounters` gains immutable `capture_mode` and `encounter_ordinal` columns,
plus uniqueness on `(session_id, encounter_ordinal)`. Legacy rows become
`single` with stable deterministic ordinals within each historical session
(normally ordinal 1) without changing either hash or canonical blob.

`encounter_session_snapshots` stores small append-only canonical snapshots:
session identity, revision, channel, mode, disposition, timestamps, interruption
facts, and ordered `(ordinal, encounter_id, content_sha256)` references. Newer
snapshots must preserve prior encounter and marker references as exact prefixes.
