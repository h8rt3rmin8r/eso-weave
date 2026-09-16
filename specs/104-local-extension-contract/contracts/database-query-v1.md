# Contract: Read-Only Database Query 1.0.0

## Runtime database inventory

| Identifier | Runtime role | Current table families |
| --- | --- | --- |
| `catalog` | Active ESO catalog database | `schema_migration`, `catalog_release`, `source_snapshot`, `source_record`, `coverage`, `entity`, `entity_source`, `entity_attribute`, `entity_relation`, `entity_alias`, `localized_text`, `icon_reference`, `icon_asset` |
| `encounters` | User encounter history | `encounter_store_meta`, `raw_encounters`, `encounter_session_snapshots` |

These are the only application-owned runtime SQLite query surfaces. Temporary test fixtures, import staging files, backups, replacement candidates, documentation databases, and SQLite sidecar files are not separate database identifiers. An absent optional database remains in discovery with `available: false`.

## Request

```json
{
  "sql": "SELECT entity_id, kind FROM entity WHERE kind = ?1 LIMIT ?2",
  "parameters": [
    { "type": "text", "value": "skill" },
    { "type": "integer", "value": 20 }
  ],
  "row_limit": 20
}
```

One request contains exactly one statement of at most 16 KiB and at most 64 typed named or positional parameters. Mixed named and positional parameters are rejected. The requested row limit is optional and cannot exceed 1,000.

## Result

```json
{
  "database_id": "catalog",
  "columns": [
    { "ordinal": 0, "name": "entity_id" },
    { "ordinal": 1, "name": "kind" }
  ],
  "rows": [
    [
      { "type": "integer", "value": 123 },
      { "type": "text", "value": "skill" }
    ]
  ],
  "row_count": 1,
  "truncated": false,
  "truncation_reason": null,
  "elapsed_ms": 2,
  "limits": {
    "rows": 1000,
    "bytes": 1048576,
    "duration_ms": 2000
  }
}
```

SQLite values use `null`, `integer`, `real`, `text`, and `blob`. Blob values carry base64 text. Non-finite real inputs are rejected. Boolean inputs are explicit and bind as integer 0 or 1. Rows are arrays, so duplicate column names and column order remain lossless.

## Defense in depth

Each request receives a short-lived connection to the selected current database generation using read-only, URI, no-mutex, and no-follow flags. The service enables `query_only`, defensive mode, and untrusted schema behavior before preparation. Extension loading is not compiled or enabled for this surface.

The authorizer denies insert, update, delete, DDL, attach, detach, reindex, analyze, vacuum, transaction and savepoint control, unsafe pragmas, writable virtual table operations, and unknown action codes. Preparation must consume exactly one statement and no non-comment trailing SQL. The prepared statement must also report read-only.

A progress handler checks cancellation and the 2-second deadline. Reduced SQLite limits constrain SQL length, variable count, expression depth, compound selects, attached databases to zero, and result sizes. Transport body parsing stops at 64 KiB.

## Exact limits

| Resource | Bound | Completion |
| --- | ---: | --- |
| Concurrent external queries | 2 globally | reject as retryable busy |
| Request body | 65,536 bytes | reject before JSON parsing |
| SQL text | 16,384 UTF-8 bytes | invalid request |
| Parameters | 64 | invalid request |
| Execution | 2,000 ms | interrupt and `query_timeout` |
| Returned rows | 1,000 | successful result with row truncation |
| Serialized result data | 1,048,576 bytes | successful result with byte truncation if a complete row fits; otherwise limit error |

The executor materializes the complete bounded result and closes the statement and connection before an HTTP or MCP client receives bytes. Slow clients therefore retain no database lock.

## Adapter parity

HTTP uses `POST /api/v1/databases/{database_id}/query`. MCP uses `query_database` with `database_id`, `sql`, `parameters`, and `row_limit`. Both invoke the same input validator and executor and return equivalent columns, values, rows, truncation facts, timing, and error codes.

## Compatibility

Database identifiers and result value types are stable for schema major version 1. Database table and column discovery is descriptive and can grow with application migrations. Clients must not assume a catalog or encounter schema revision without checking discovery. A change to query semantics, type representation, security boundaries, or limit interpretation requires a reviewed external schema version change.
