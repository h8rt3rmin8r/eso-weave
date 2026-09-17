# Contract: S108 Database Query Adapters 1.0.0

This contract specializes the accepted S104 database query contract for the shipped HTTP and MCP adapters.

## Inventory

HTTP:

```text
GET /api/v1/databases
```

MCP:

```text
esoweave://databases
```

Both return the same compact JSON document:

```json
{
  "schema_version": "1.0.0",
  "databases": [
    {
      "database_id": "catalog",
      "role": "active ESO catalog",
      "available": true,
      "availability_reason": null,
      "objects": [
        {
          "name": "entity",
          "kind": "table",
          "columns": [
            {
              "ordinal": 0,
              "name": "kind",
              "declared_type": "TEXT",
              "nullable": false,
              "primary_key_position": 1
            }
          ]
        }
      ]
    }
  ]
}
```

The actual document always contains both stable database descriptors.

## Query

HTTP:

```text
POST /api/v1/databases/{database_id}/query
Content-Type: application/json
```

MCP tool:

```text
query_database(database_id, sql, parameters, row_limit)
```

Positional request:

```json
{
  "sql": "SELECT entity_id, kind FROM entity WHERE kind = ?1 LIMIT ?2",
  "parameters": [
    { "type": "text", "value": "skill" },
    { "type": "integer", "value": "20" }
  ],
  "row_limit": 20
}
```

Named request:

```json
{
  "sql": "SELECT entity_id FROM entity WHERE kind = :kind",
  "parameters": [
    { "name": ":kind", "type": "text", "value": "skill" }
  ]
}
```

Success:

```json
{
  "database_id": "catalog",
  "columns": [
    { "ordinal": 0, "name": "entity_id" },
    { "ordinal": 1, "name": "kind" }
  ],
  "rows": [
    [
      { "type": "integer", "value": "123" },
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

## Error

HTTP returns an appropriate status with:

```json
{
  "error": {
    "code": "query_denied",
    "message": "The statement is not permitted by the read-only query contract.",
    "retryable": false
  }
}
```

MCP returns the same object as structured error content. Unknown MCP tool names remain protocol errors.

## Methods and media types

- Inventory supports GET only.
- HTTP query supports POST with `application/json` only.
- MCP inventory is one UTF-8 `application/json` resource content item.
- The MCP query tool advertises read-only behavior and has no destructive hint.

## Parity

- Inventory JSON is exactly equal after parsing.
- Successful query JSON is equal after replacing independently measured `elapsed_ms` with one comparison sentinel.
- Canonical error objects are exactly equal after parsing.
- Authentication, Host, Origin, body size, stopping, and unknown-route guards remain shared.
