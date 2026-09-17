# Data Model: Read-Only Database Queries

**Date**: 2026-09-17

## DatabaseQueryService

- `paths`: private current catalog and encounter paths
- `permits`: global two-permit admission gate shared by HTTP and MCP clones

Operations:

- `inventory(cancellation) -> DatabaseInventory`
- `execute(database_id, request, cancellation) -> QueryResult`
- `set_catalog_path(path)`

The service never exposes a path and never retains a connection.

## DatabaseInventory

- `schema_version`: external contract version
- `databases`: exactly two ordered `DatabaseDescriptor` values

## DatabaseDescriptor

- `database_id`: `catalog` or `encounters`
- `role`: stable public role string
- `available`: boolean
- `availability_reason`: null, `not_present`, or `unavailable`
- `objects`: ordered tables and views when available, otherwise empty

## SchemaObject

- `name`: SQLite object name
- `kind`: `table` or `view`
- `columns`: ordered visible `SchemaColumn` values

## SchemaColumn

- `ordinal`: zero-based visible position
- `name`: SQLite column name
- `declared_type`: declared type text or null
- `nullable`: boolean
- `primary_key_position`: zero for non-key columns, otherwise SQLite key order

## QueryRequest

- `sql`: one non-empty statement, at most 16,384 UTF-8 bytes
- `parameters`: zero through 64 typed values
- `row_limit`: optional integer 1 through 1,000, default 1,000

Parameter mode is derived from the complete array:

- positional: every parameter has no `name`
- named: every parameter has a unique `name` beginning with `:`, `@`, or `$`
- mixed: invalid

## QueryParameter

- `name`: optional exact SQLite parameter name
- `type`: `null`, `integer`, `real`, `text`, `blob`, or `boolean`
- `value`: absent for null, string for integer/real/text/blob, boolean for boolean

## QueryResult

- `database_id`: selected stable identifier
- `columns`: ordered ordinal and name pairs
- `rows`: ordered arrays of `ResultValue`
- `row_count`: returned complete rows
- `truncated`: boolean
- `truncation_reason`: null, `rows`, or `bytes`
- `elapsed_ms`: monotonic execution milliseconds capped at the timeout bound
- `limits`: fixed rows, bytes, and duration metadata

## ResultValue

- `null`: no value member
- `integer`: canonical signed decimal string
- `real`: shortest round-trippable decimal or `positive_infinity`, `negative_infinity`, `nan`
- `text`: UTF-8 string
- `blob`: standard padded base64 string

## QueryError

- `code`: stable machine code
- `message`: fixed safe message
- `retryable`: boolean

Stable codes:

- `invalid_request`
- `database_not_found`
- `database_unavailable`
- `query_denied`
- `query_invalid`
- `query_busy`
- `query_timeout`
- `database_busy`
- `result_too_large`
- `service_stopping`
- `internal_error`

No error contains SQL, parameter values, paths, SQLite diagnostics, or secrets.
