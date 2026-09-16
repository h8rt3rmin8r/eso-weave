# Contract: Local Extension Service Lifecycle

## Network surface

| Property | Version 1 contract |
| --- | --- |
| Bind | IPv4 `127.0.0.1` only |
| Production port | `18765`, fail on collision |
| Test port | `0` permitted through test construction only |
| HTTP root | `/api/v1` |
| MCP endpoint | `/mcp`, Streamable HTTP |
| MCP sessions | Stateless |
| Remote transport | Unsupported |
| Legacy MCP SSE or stdio | Unsupported |

## Shared operations

| Canonical service | HTTP adapter | MCP adapter |
| --- | --- | --- |
| Capabilities | `GET /api/v1/capabilities` | resource `esoweave://capabilities` |
| Current snapshot | `GET /api/v1/player-state` | resource `esoweave://player-state` |
| Database inventory | `GET /api/v1/databases` | resource `esoweave://databases` |
| Read-only query | `POST /api/v1/databases/{database_id}/query` | tool `query_database` |

Neither adapter owns a second cache, state model, query implementation, limit, or error vocabulary.

## Security middleware order

1. Reject an authority or `Host` that is not the effective loopback authority.
2. If `Origin` is present, require an HTTP or HTTPS loopback host and matching effective port. A non-browser client may omit `Origin`.
3. Require exactly one `Authorization: Bearer` credential and compare it without timing-dependent early exit.
4. Enforce request-body and content-type bounds.
5. Attach an opaque request identifier and enter the canonical adapter.

Discovery, capabilities, and MCP initialization are authenticated. Responses do not emit `Access-Control-Allow-Origin: *`. Credentials never enter URLs, discovery files, logs, errors, or telemetry.

## Credential and discovery

On first enablement, create a persistent token from at least 32 cryptographically random bytes. Store it with the existing per-user settings data and restrictive user-only permissions where the platform supports them. Rotation invalidates existing clients and increments the service generation.

While running, atomically replace `local-extension.json` in the existing application data directory with:

```json
{
  "discovery_version": 1,
  "service_generation": 12,
  "process_id": 1234,
  "http_base_url": "http://127.0.0.1:18765/api/v1",
  "mcp_url": "http://127.0.0.1:18765/mcp",
  "schema_version": "1.0.0"
}
```

The record contains no credential. Remove it on stop and failed startup. A client must treat a dead process, unreachable endpoint, or mismatched generation as stale discovery.

## Runtime ownership

The synchronous application creates one lifecycle controller. Enabling sends a start command to one named background OS thread. That thread constructs a Tokio current-thread runtime, listener, shared router, RMCP service, cancellation token, and bounded query semaphore. `spawn_blocking` or an equivalent dedicated blocking boundary owns SQLite work.

The application publishes immutable snapshots through an atomically replaceable reference. Network handlers never lock mutable UI, PixelBus, input-hook, or controller state and never invoke automation.

## State transitions

```text
stopped -> starting -> running -> stopping -> stopped
                    -> failed  -> starting
starting -> stopping -> stopped
running  -> failed   -> stopping -> stopped
```

- Start succeeds only after the listener, both adapters, authentication state, and discovery record are ready.
- Any partial start failure cancels all components, closes the listener, removes discovery, and reports `failed`.
- A rapid disable during `starting` cancels startup and reaches `stopped`.
- Re-enable after a completed stop or recoverable failure creates a new generation.
- A collision on port 18765 reports `address_in_use` with the address but no internal path.

## Shutdown

Disable and application exit use the same sequence:

1. Enter `stopping`, stop accepting new requests, and remove discovery.
2. Cancel HTTP and MCP serving and reject new query work.
3. Interrupt in-flight SQLite work through its progress handler.
4. Allow bounded response cleanup, close database handles, and release the listener.
5. Join the owner thread within 3 seconds and report `stopped`.

Failure to join within 3 seconds reports a visible non-retryable shutdown failure. The UI thread may poll status but must not block a frame waiting for shutdown.

## Structured response errors

HTTP uses a JSON error object and a suitable 4xx or 5xx status. MCP uses the matching protocol error or tool error with the same extension code. Safe messages omit credentials, filesystem paths, SQL text, and stack traces.
