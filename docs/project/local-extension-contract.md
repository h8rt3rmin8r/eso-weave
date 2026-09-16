# Local Extension Implementation Contract

Status: Accepted for implementation by ADR 0002 on 2026-09-15. The service is not shipped yet.

## Fixed stack and topology

- Axum `=0.8.9`, RMCP `=3.4.0`, Tokio `=1.53.1`, and Tokio Util `=0.7.16`, required features only.
- One `127.0.0.1:18765` listener, one router, one dedicated current-thread Tokio runtime owner, and one cancellation tree.
- HTTP under `/api/v1`; stateless MCP Streamable HTTP at `/mcp`; no remote bind, legacy SSE, stdio, or helper process.
- One persistent 256-bit-or-stronger bearer credential on every operation, strict Host and browser Origin validation, and no permissive CORS.
- Non-secret atomic discovery containing endpoints, process, service generation, and schema version only while running.
- Atomic combined lifecycle with truthful stopped, starting, running, stopping, and failed phases and a 3-second shutdown bound.

## Canonical operations

| Service | HTTP | MCP |
| --- | --- | --- |
| Capabilities | `GET /api/v1/capabilities` | `esoweave://capabilities` resource |
| Snapshot | `GET /api/v1/player-state` | `esoweave://player-state` resource |
| Databases | `GET /api/v1/databases` | `esoweave://databases` resource |
| Query | `POST /api/v1/databases/{database_id}/query` | `query_database` tool |

The adapters share one immutable revisioned snapshot service, one query service, one error vocabulary, and one limit set.

## State authority

External schema `1.0.0` covers all current application, game, PixelBus, player, automation, and interpretation facts. Every observation preserves knowledge, value, source, protocol, observation time, age, and freshness. Unknown, unavailable, dormant, fresh, and stale are distinct. Retained values never hide immediate focus or signal loss.

The complete field and non-public inventories are [the canonical player-state contract](../../specs/104-local-extension-contract/contracts/player-state-v1.md). Issue #177 must turn this inventory into a maintained source-coverage test before adding the HTTP projection.

## Database authority

The only runtime database identifiers are `catalog` and `encounters`. The catalog inventory covers schema migration, release, source, coverage, entity, relationship, alias, localized text, and icon tables. The encounter inventory covers store metadata, raw encounters, and session snapshots.

Queries are one typed parameterized read-only statement with defense in depth. Exact shared bounds are 2 concurrent queries, 2 seconds, 1,000 rows, 1 MiB serialized result data, 16 KiB SQL, 64 parameters, and 64 KiB request bodies. Results preserve SQLite types and materialize before client serialization.

The full authority is [the database query contract](../../specs/104-local-extension-contract/contracts/database-query-v1.md).

## Dependent implementation sequence

1. Issue #176 implements persisted opt-in lifecycle, authentication, discovery, status, and the shared runtime host.
2. Issue #177 implements the canonical immutable state projection and HTTP adapters.
3. Issue #178 mounts RMCP and proves state parity against the same snapshot revision.
4. Issue #179 implements one defended query executor and both transport adapters.
5. Issue #180 publishes shipped user and integrator documentation and cross-surface end-to-end parity tests.

Detailed lifecycle transitions and middleware order are [the service lifecycle contract](../../specs/104-local-extension-contract/contracts/service-lifecycle.md). Implementations may refine internal names, but changing a fixed external behavior, security boundary, limit, or compatibility rule requires an ADR update before code diverges.
