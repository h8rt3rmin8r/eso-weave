# Data Model: Local Extension Stack and Contract

## Service status

- `requested_enabled`: persisted user intent, default false
- `phase`: `stopped`, `starting`, `running`, `stopping`, or `failed`
- `generation`: monotonic identifier for one start attempt
- `effective_http_base_url`: present only while running
- `effective_mcp_url`: present only while running
- `schema_version`: `1.1.0`
- `failure`: optional safe lifecycle error

Only `running` publishes discovery. A failed partial start returns to one closed listener boundary before the failed state is reported.

## Canonical snapshot

- `schema_version`: semantic external schema version
- `snapshot_revision`: monotonic logical revision
- `captured_at`: UTC timestamp for snapshot publication
- `service_generation`: owning service generation
- `capabilities`: supported domains and protocol facts
- `application`, `game`, `pixel_bus`, `player`, `automation`, and `interpretation`: stable domain objects

Handlers obtain one immutable snapshot reference. Every observed leaf carries or inherits a `knowledge` state, source, protocol, observation time, and freshness.

## Observation metadata

- `knowledge`: `observed`, `unknown`, `unavailable`, or `dormant`
- `value`: typed value, present only when the knowledge state permits it
- `observed_at`: optional UTC timestamp of source evidence
- `age_ms`: derived nonnegative age at snapshot capture
- `freshness`: `fresh`, `stale`, or `not_applicable`
- `source`: stable source such as `game_process`, `pixel_bus`, `settings`, or `controller`
- `protocol`: optional source protocol name, revision, layout revision, and capability reason

Retained values may be stale, but lifecycle facts such as focus and signal loss are current in the same snapshot.

## Database descriptor

- `id`: `catalog` or `encounters`
- `available`: current openable state
- `schema_revision`: application schema revision when available
- `tables`: ordered names and column descriptions
- `capabilities`: read-only query support and exact limits
- `unavailable_reason`: safe reason without filesystem paths

## Query request

- `database_id`: stable inventory identifier
- `sql`: one statement, maximum 16 KiB
- `parameters`: ordered positional or named typed values, maximum 64
- `requested_row_limit`: optional integer from 1 through 1,000; an out-of-range value is `invalid_request`

Supported parameter types are null, signed 64-bit integer as a canonical decimal string, finite 64-bit real as a round-trippable decimal string, UTF-8 text, boolean converted deliberately to integer, and base64 blob.

## Query result

- `database_id`
- `columns`: ordered entries with ordinal and name
- `rows`: ordered arrays of typed SQLite values, maximum 1,000
- `row_count`
- `truncated`: whether row or byte bounds ended collection
- `truncation_reason`: `row_limit` or `byte_limit` when applicable
- `elapsed_ms`
- `limits`: effective bounds

Rows are arrays so duplicate column names remain representable. Each value has an explicit `type` and corresponding `value`. Integer values are canonical signed decimal strings. Finite reals use shortest round-trippable decimal strings; non-finite reals use `positive_infinity`, `negative_infinity`, or `nan`.

## Extension error

- `code`: stable lower-snake-case identifier
- `message`: safe actionable text
- `retryable`: boolean
- `limit`: optional limit name and value
- `request_id`: opaque local correlation identifier

Core codes include `unauthorized`, `invalid_host`, `invalid_origin`, `invalid_request`, `schema_version_unsupported`, `database_unknown`, `database_unavailable`, `query_not_read_only`, `query_limit_exceeded`, `query_timeout`, `database_busy`, `service_stopping`, and `internal_error`.
