# Data Model: Local Extension Documentation and Verification

**Date**: 2026-09-17

## Documented field inventory

The checked JSON document has this shape:

```json
{
  "schema_version": "1.0.0",
  "fields": [
    {
      "path": "player.combat",
      "type": "observation<string>",
      "meaning": "Current combat signal.",
      "source": "pixel_bus",
      "availability": "Observed while current game and Pixel Bus evidence is usable; otherwise unknown, dormant, or stale."
    }
  ]
}
```

### Invariants

- `schema_version` equals the production external schema version.
- `fields` contains exactly the production `PUBLIC_PATHS` set.
- Every path is unique and sorted in production order for readable diffs.
- Every metadata string is trimmed and non-empty.
- Types describe the JSON value within an observation, or the direct structure for non-observation paths.
- Availability explains when a value can be absent, unavailable, dormant, or stale.

## End-to-end fixture

| Component | Required contents | Authority |
| --- | --- | --- |
| Snapshot publisher | Deterministic application, game, Pixel Bus, player, automation, and interpretation values | `SnapshotPublisher` and `player_state::project` conventions |
| Catalog database | Small public table with integer and text values | Temporary SQLite file |
| Encounter database | Small encounter table or truthful absent state, selected per assertion | Temporary SQLite file |
| Listener | Ephemeral loopback port, fixed test credential, bounded shutdown | `LocalServiceController` |
| MCP client | Official RMCP Streamable HTTP client with bearer header | RMCP dev dependency |

## Normalized parity result

The parity comparison retains all application data and removes only:

- MCP content framing around structured JSON.
- `elapsed_ms` from query results because each transport executes independently.

The comparison does not remove revision, generation, columns, rows, truncation, limits, error code, message, or retryability.

## Lifecycle receipt

| Field | Type | Meaning |
| --- | --- | --- |
| `phase` | enum | Observed controller phase |
| `generation` | unsigned integer or absent | Current listener lifetime identity |
| `discovery_present` | boolean | Whether the owned discovery record exists |
| `endpoint_reachable` | boolean | Whether the generation endpoint accepts authenticated requests |
| `shutdown_elapsed_ms` | unsigned integer | Measured bounded shutdown duration |

Receipts exist only inside tests and never include the bearer credential, absolute path, or private gameplay data.

## State transitions

```text
stopped -> starting -> running -> stopping -> stopped
              |                         |
              +-> failed <--------------+
                    |
                    +-> starting after the cause clears
```

Port collision enters `failed`, publishes no owned running discovery, and permits a later explicit start. Restart increments `service_generation`; old endpoints and discovery generations are stale.
