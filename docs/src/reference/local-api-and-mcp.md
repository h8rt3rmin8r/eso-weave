# Local API and MCP

ESO Weave can host one authenticated local extension service for trusted tools
and AI clients. It exposes current application knowledge and bounded read-only
queries. It does not accept gameplay actions, write application data, host an
agent, or bind beyond the local computer.

## Enable the service

Open **File > Settings**, find **Local API and MCP**, and enable **Local API and MCP Server**.
The setting is off by default and persists across restarts. The
interface shows this exact warning:

> Connected clients can read live player state and query ESO Weave application data. Enable only for local clients you trust.

Both transports start and stop together.

| Status | Meaning | Client action |
| --- | --- | --- |
| `stopped` | No listener is running and owned discovery is absent | Enable the setting before connecting |
| `starting` | The loopback listener and discovery record are being prepared | Wait for `running` |
| `running` | HTTP and MCP share one authenticated generation | Use the displayed endpoints and copied credential |
| `stopping` | Existing work is being cancelled within the shutdown bound | Stop starting requests and rediscover later |
| `failed` | Startup failed, commonly because the port is occupied | Resolve the displayed cause, then disable and enable the setting |

Use **Copy Credential** for the bearer credential. The credential is secret. It
is stored with local settings and is never written to discovery or logs. A
client sends it as `Authorization: Bearer <credential>` on every HTTP and MCP
request.

## Discover endpoints

The running interface displays the HTTP and MCP URLs. ESO Weave also writes
`local-extension.json` in its platform configuration directory while the
service runs. Discovery is not authentication and contains no credential.

| Discovery field | Type | Meaning |
| --- | --- | --- |
| `discovery_version` | integer | Discovery-file format, currently `1` |
| `service_generation` | integer | Listener lifetime identity; changes after restart |
| `process_id` | integer | Owning ESO Weave process |
| `http_base_url` | string | Versioned HTTP base URL |
| `mcp_url` | string | Streamable HTTP MCP endpoint |
| `schema_version` | string | External data contract, currently `1.0.0` |

The default listener is `127.0.0.1:18765`. A test or future configuration may
select another loopback port, so clients that automate discovery should use the
published URLs. Reject discovery owned by a process or generation that is no
longer current.

The listener accepts only loopback Host values. Browser requests with an Origin
must also identify a loopback origin. The service does not publish permissive
CORS headers.

## HTTP example

Set `ESOWEAVE_TOKEN` to the value copied from the interface. These examples use
the default endpoint. Replace it with the displayed or discovered URL when it
differs.

```bash
export ESOWEAVE_TOKEN='replace-with-copied-credential'
curl --fail --silent \
  --header "Authorization: Bearer ${ESOWEAVE_TOKEN}" \
  http://127.0.0.1:18765/api/v1/capabilities
```

```powershell
$env:ESOWEAVE_TOKEN = 'replace-with-copied-credential'
Invoke-RestMethod `
  -Headers @{ Authorization = "Bearer $env:ESOWEAVE_TOKEN" } `
  -Uri 'http://127.0.0.1:18765/api/v1/capabilities'
```

## MCP client example

Use a standard MCP client that supports Streamable HTTP and request headers.
Replace the placeholder with the copied credential. Environment-variable
substitution is client-specific, so do not assume every client expands it.

```json
{
  "mcpServers": {
    "eso-weave": {
      "type": "http",
      "url": "http://127.0.0.1:18765/mcp",
      "headers": {
        "Authorization": "Bearer replace-with-copied-credential"
      }
    }
  }
}
```

The server is stateless. It supports MCP initialization, resource listing and
reading, tool listing, and `query_database`. It does not publish prompts,
subscriptions, legacy SSE, or stdio transport.

## Operations

| Purpose | HTTP | MCP | Result |
| --- | --- | --- | --- |
| Discover capabilities | `GET /api/v1/capabilities` | `esoweave://capabilities` | Schema, domains, operations, databases, and Pixel Bus capability |
| Read current state | `GET /api/v1/player-state` | `esoweave://player-state` | One immutable canonical player-state snapshot |
| Discover databases | `GET /api/v1/databases` | `esoweave://databases` | Fixed database availability and safe public schema |
| Run a read-only query | `POST /api/v1/databases/{database_id}/query` | `query_database` | Typed columns, rows, truncation, elapsed time, and limits |

HTTP and MCP project the same authorities. MCP resource content is JSON text.
The query tool returns the same structured success or canonical error object as
HTTP after transport framing is removed.

## Player-state envelope

| Field | Type | Change rule |
| --- | --- | --- |
| `schema_version` | string | Changes only when the external contract requires a new version |
| `snapshot_revision` | integer | Advances when canonical snapshot content changes |
| `captured_at` | RFC 3339 string | Time associated with the published snapshot |
| `service_generation` | integer | Identifies the current listener lifetime and changes after restart |
| `capabilities` | object | Operations and compatible Pixel Bus facts |
| `application`, `game`, `pixel_bus`, `player`, `automation`, `interpretation` | objects | Canonical state domains |

Compare state from the same `snapshot_revision` and `service_generation` when
transport parity matters. A new service generation invalidates old endpoint and
discovery assumptions even when `schema_version` is unchanged.

Most leaf values use an observation wrapper. Knowledge and freshness are
separate axes: a retained value may be known but stale.

| Observation field | Type | Meaning |
| --- | --- | --- |
| `knowledge` | string | `observed`, `unknown`, `unavailable`, or `dormant` |
| `value` | any, optional | Present only when a value is known |
| `observed_at` | RFC 3339 string or null | Source observation time when retained |
| `age_ms` | integer or null | Age when the source supplies it |
| `freshness` | string | `fresh`, `stale`, or `not_applicable` |
| `source` | string | Canonical subsystem that supplied the fact |
| `protocol` | object or null | Source protocol and capability detail when applicable |

`unknown` means the application cannot currently determine the fact.
`unavailable` means the source or capability cannot provide it. `dormant` means
the source is intentionally inactive in the current context. Never convert any
of these states into a positive or safe assertion. A stale value is retained
history, not current authorization for gameplay input.

The downloadable
[player-state field inventory](local-extension-state-fields.json) lists every
public path with its JSON type, meaning, source, and availability rule. The
build compares that inventory with the production `PUBLIC_PATHS` constant, so a
new field requires an explicit documentation review.

## Database discovery

The fixed database identifiers are `catalog` and `encounters`. Discovery always
lists both in stable order. An absent or unusable database remains present with
`available: false`, a safe availability reason, and an empty schema. Discovery
does not reveal filesystem paths, SQL definitions, triggers, indexes, or
internal `sqlite_%` objects.

Each available database reports user tables and views with ordered visible
columns, declared SQLite types, nullability, and primary-key position. The
catalog contains shipped or selected game-data reference material. Encounters
contains user-owned imported encounter history when that store exists.

## Query requests and results

HTTP puts the database identifier in the route. MCP adds `database_id` to the
same request object. `sql` contains exactly one non-empty read-only statement.
`row_limit` is optional. `parameters` is an ordered array and defaults to empty.

Use either positional `?` or contiguous `?N` parameters, or named parameters
whose exact names begin with `:`, `@`, or `$`. Do not mix modes. Named values
must be unique and every statement parameter must be supplied.

| Parameter `type` | JSON `value` | SQLite binding |
| --- | --- | --- |
| `null` | omitted | NULL |
| `integer` | canonical signed decimal string produced by `i64::to_string()` | 64-bit integer; `+1`, `01`, and `-0` are rejected |
| `real` | finite decimal string | 64-bit real |
| `text` | string | UTF-8 text |
| `blob` | standard padded base64 string | bytes |
| `boolean` | boolean | integer `0` or `1` |

Success contains ordered `columns`, typed `rows`, `row_count`, `truncated`,
`truncation_reason`, `elapsed_ms`, and `limits`. Each cell is tagged as `null`,
`integer`, `real`, `text`, or `blob`. Integer and real output uses strings to
preserve exact representation; blob output uses standard padded base64. Empty
results retain column metadata and return an empty row array. Duplicate column
names remain in their original positions. A finite real uses its decimal string;
non-finite reals use `positive_infinity`, `negative_infinity`, or `nan`.

The executor fully materializes the bounded result and closes its SQLite
statement and connection before either transport sends data. A slow reader does
not keep a database lock.

| Bound | Value | Behavior |
| --- | --- | --- |
| Request body | 65536 bytes | Larger or incomplete bodies are rejected |
| SQL text | 16384 UTF-8 bytes | Empty, comment-only, oversized, or multiple statements are invalid |
| Parameters | 64 | Extra, missing, mixed, or unsupported bindings are invalid |
| Concurrent external queries | 2 | A third admitted request receives retryable `query_busy` |
| Execution | 2000 ms | The statement is interrupted with retryable `query_timeout` |
| Rows | 1000 | Whole rows are returned with row truncation metadata |
| Compact result envelope | 1048576 bytes | Whole rows are returned with byte truncation metadata; no fitting row returns `result_too_large` |

Read-only enforcement combines a read-only no-follow connection, defensive and
query-only SQLite configuration, an authorizer, read-only statement checks,
single-statement validation, and SQLite runtime limits. Clients cannot mutate
data, attach databases, load extensions, control transactions, or select an
arbitrary path.

| Error code | Retryable | Meaning |
| --- | --- | --- |
| `invalid_request` | no | Request shape, parameter, or bound is invalid |
| `database_not_found` | no | Identifier is not `catalog` or `encounters` |
| `database_unavailable` | yes | The selected fixed database is not currently usable |
| `query_denied` | no | The statement violates the read-only contract |
| `query_invalid` | no | SQLite could not prepare or execute the statement safely |
| `query_busy` | yes | Both external query permits are active |
| `query_timeout` | yes | Execution exceeded 2000 ms |
| `database_busy` | yes | SQLite reports a temporary lock or busy state |
| `result_too_large` | no | No complete row fits in the 1048576-byte result envelope |
| `service_stopping` | yes | The active service generation is shutting down |
| `internal_error` | no | The request could not be completed safely without disclosing internals |

## Compatibility

Use `schema_version` for compatibility decisions. Additive object fields may
appear within a compatible version; ignore fields you do not understand. Treat
an unknown enum value as unknown evidence, never as a known safe state.

Removing or retyping a documented field, changing a stable database or
operation identifier, or changing required semantics requires a schema-version
change. `snapshot_revision` and `service_generation` are runtime counters, not
schema versions.

## Read-only client workflows

A feedback client can read `player-state`, confirm current generation and
freshness, then describe the observed state. It must not treat a stale resource,
unknown life state, or unavailable binding as current authorization.

An analysis client can discover the catalog schema, submit a parameterized
`SELECT`, and combine reference rows with the same-generation snapshot. It can
query encounter history when available. The service does not decide what an
agent should say, retain prompts, execute actions, or expand database access.

## Troubleshooting

- **No endpoint**: confirm the setting is enabled and status is `running`.
- **Port collision**: another process owns port 18765. Stop that process or its
  service, then disable and enable ESO Weave's setting.
- **Stale discovery**: require the current process and service generation. A
  stopped service removes only discovery that it owns.
- **401 authentication**: copy the credential again and send exactly one bearer
  Authorization header. Discovery never contains the credential.
- **400 Host or 403 Origin**: use the displayed loopback URL. Remote hosts and
  non-loopback browser origins are rejected.
- **Protocol or schema mismatch**: initialize with Streamable HTTP MCP and check
  `schema_version` before interpreting documents.
- **Database unavailable**: inspect database discovery. Optional encounter
  history may not exist yet.
- **Busy or limit error**: honor `retryable`; reduce concurrency or query work.
  Do not retry non-retryable invalid or denied requests unchanged.
- **Disconnected client**: reconnect through current discovery. Completed query
  materialization holds no database lock for the disconnected reader.
- **Restart or shutdown**: stop sending work during `stopping`, discard the old
  generation, and rediscover after `running`. Normal shutdown is bounded to
  three seconds.

For general application startup and game-observation diagnosis, continue with
[Troubleshooting](../getting-started/troubleshooting.md).
