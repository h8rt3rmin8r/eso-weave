# Data Model: Local Service Lifecycle

## LocalServicePrefs

| Field | Type | Rule |
| --- | --- | --- |
| `enabled` | boolean | Defaults false; durable requested state |
| `credential` | optional string | Exactly 64 lowercase hexadecimal characters when present; never presented or logged |

The credential is created only when enablement is first requested. Disable preserves it.

## ServicePhase

| Value | Meaning |
| --- | --- |
| `stopped` | No listener, runtime service, or owned discovery is active |
| `starting` | A generation is constructing its runtime boundary |
| `running` | Listener, router, both transport surfaces, authentication, and discovery are ready |
| `stopping` | New work is closed and owned resources are being released |
| `failed` | The complete service is unavailable and a safe failure is visible |

## ServiceStatus

| Field | Type | Rule |
| --- | --- | --- |
| `phase` | ServicePhase | Always present |
| `generation` | unsigned integer | Increments for every start attempt |
| `connection` | optional ConnectionInfo | Present only while running |
| `failure` | optional ServiceFailure | Present only while failed |

Status is a runtime snapshot. It is never written to `config.json`.

## ConnectionInfo

| Field | Type | Rule |
| --- | --- | --- |
| `http_base_url` | string | Effective `http://127.0.0.1:<port>/api/v1` |
| `mcp_url` | string | Effective `http://127.0.0.1:<port>/mcp` |
| `process_id` | unsigned integer | Current process |
| `generation` | unsigned integer | Owning start attempt |
| `schema_version` | string | `1.0.0` |

## ServiceFailure

| Field | Type | Rule |
| --- | --- | --- |
| `code` | stable enum string | Safe programmatic category |
| `message` | string | Actionable and free of secret, path, body, or stack detail |
| `retryable` | boolean | True for correctable bind or storage conditions |

Initial codes are `settings_unavailable`, `credential_unavailable`, `address_in_use`, `bind_failed`, `discovery_failed`, `runtime_failed`, and `shutdown_timeout`.

## DiscoveryRecord

| Field | Type | Rule |
| --- | --- | --- |
| `discovery_version` | integer | Exactly 1 |
| `service_generation` | unsigned integer | Exact owner generation |
| `process_id` | unsigned integer | Exact owner process |
| `http_base_url` | string | Non-secret effective URL |
| `mcp_url` | string | Non-secret effective URL |
| `schema_version` | string | Exactly `1.0.0` |

## OwnerCommand

| Command | Effect |
| --- | --- |
| `Start { credential }` | Start or retry a complete generation |
| `Stop` | Cancel the active or starting generation and settle stopped |
| `Shutdown` | Stop, release ownership, and terminate the owner thread |

Commands are ordered. Transition code drains queued commands before publishing running status so the last requested state wins.

## Transition Invariants

1. Only the owner thread writes phase and generation.
2. Running implies the discovery record for the same process and generation exists.
3. Failed implies neither transport is intentionally left available.
4. Stopped implies no owned listener or discovery remains.
5. A newer generation never removes discovery owned by another process or generation.
