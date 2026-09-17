# Contract: Local Service Lifecycle Version 1

## Persisted Settings

```json
{
  "local_service": {
    "enabled": false,
    "credential": null
  }
}
```

On first enablement, `credential` becomes a 64-character lowercase hexadecimal secret. It is not cleared by disable.

## Discovery

While running, `<config-dir>/local-extension.json` contains:

```json
{
  "discovery_version": 1,
  "service_generation": 1,
  "process_id": 1234,
  "http_base_url": "http://127.0.0.1:18765/api/v1",
  "mcp_url": "http://127.0.0.1:18765/mcp",
  "schema_version": "1.0.0"
}
```

No credential, filesystem path, query text, or runtime handle is public.

## Credential Retrieval

The settings cluster exposes one explicit Copy Credential action after a valid credential exists. The action places the secret on the user clipboard for local client configuration. The UI never renders the secret inline, and status, discovery, diagnostics, and errors never contain it.

## Request Boundary

1. `Host` must equal `127.0.0.1:<effective-port>`.
2. If `Origin` is present, its scheme must be HTTP or HTTPS and its authority must be the same loopback host and effective port.
3. Exactly one `Authorization` header must contain `Bearer <credential>`.
4. Request bodies are limited to 65,536 bytes.
5. The request is dispatched to `/api/v1` or `/mcp`.

Rejections return safe status and text with no permissive CORS header.

## S105 Surface

| Path | Behavior |
| --- | --- |
| `/mcp` | Stateless RMCP Streamable HTTP initialization with no resources, tools, or prompts |
| `/api/v1` and descendants | Authenticated structured `service_unavailable` response until issue #177 adds owned operations |
| all other paths | Authenticated not-found response |

## Lifecycle

```text
stopped -> starting -> running -> stopping -> stopped
                    -> failed  -> starting
starting -> stopping -> stopped
running  -> failed   -> stopping -> stopped
```

- Discovery is published after both routed surfaces are ready and before running status.
- A failure before that point cancels the entire generation.
- Stop and exit remove discovery only on exact process and generation ownership.
- A stop request observed during start prevents running publication.
- Re-enable after stop or failure uses a new generation.
- Owner shutdown joins within 3 seconds or reports `shutdown_timeout`.

## Diagnostics

Allowed fields are phase, generation, safe failure code, and loopback address. Forbidden fields are credential, Authorization value, request or response body, query content, filesystem path, and stack trace.
