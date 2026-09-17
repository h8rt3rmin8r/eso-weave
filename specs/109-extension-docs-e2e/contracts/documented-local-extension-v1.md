# Contract: Documented Local Extension 1.0.0

**Status**: Planning contract for S109

## Published entry point

`docs/src/reference/local-api-and-mcp.md` is the canonical public guide. It owns:

- enablement and status;
- discovery and bearer authentication;
- HTTP routes and MCP resources or tools;
- player-state envelope and observation semantics;
- database inventory, query request, typed result, limits, and errors;
- compatibility rules, examples, workflows, and troubleshooting.

Other public pages link to this guide and may summarize only their local relationship.

## Operations

| Capability | HTTP | MCP |
| --- | --- | --- |
| Capabilities | `GET /api/v1/capabilities` | `esoweave://capabilities` |
| Player state | `GET /api/v1/player-state` | `esoweave://player-state` |
| Databases | `GET /api/v1/databases` | `esoweave://databases` |
| Query | `POST /api/v1/databases/{database_id}/query` | `query_database` |

All operations require the same bearer credential and remain bound to loopback.

## Documentation inventory contract

`docs/src/reference/local-extension-state-fields.json` is valid UTF-8 JSON without BOM and contains:

- `schema_version` equal to `player_state::SCHEMA_VERSION`;
- one `fields` entry for every `player_state::PUBLIC_PATHS` value;
- no duplicate or extra paths;
- non-empty `type`, `meaning`, `source`, and `availability` metadata.

The Rust test reports missing, unexpected, duplicate, or incomplete entries separately.

## Example contract

- HTTP examples target `http://127.0.0.1:18765/api/v1` and use `ESOWEAVE_TOKEN` or an explicit replacement placeholder.
- MCP configuration targets `http://127.0.0.1:18765/mcp` with an `Authorization: Bearer ...` header.
- Examples contain no 64-character credential literal, user-specific absolute path, remote host, write statement, or gameplay action instruction.

## Verification contract

The production-adapter suite proves:

1. bearer-authenticated HTTP and official RMCP initialization;
2. exact capabilities, player-state, and database inventory parity;
3. typed query parity after normalizing `elapsed_ms` only;
4. denied-query canonical error parity;
5. collision failure, recovery, generation rollover, disconnect tolerance, and bounded shutdown;
6. discovery removal after stop or shutdown.

## Compatibility

- Clients must use `schema_version` for compatibility decisions.
- Additive object fields may appear within the same compatible line and must be ignored when unknown.
- Clients must treat unknown enum values as unknown evidence, not as a known safe state.
- Removing or retyping a documented field, changing stable identifiers, or changing required semantics requires a schema-version change.
- `snapshot_revision` and `service_generation` are runtime counters, not schema versions.
