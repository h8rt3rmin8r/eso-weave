# MCP Player-State Contract v1

## Transport and Security

- Endpoint: `/mcp` on the existing loopback-only local-service listener
- Protocol: MCP Streamable HTTP through RMCP 3.4.0
- Response mode: Stateless JSON
- Authentication: Existing bearer token
- Host, Origin, body-size, cancellation, and service-stopping guards: Existing S105 policy
- Legacy SSE: Disabled

## Initialize

The server identifies ESO Weave and advertises resource support. Resource subscriptions and list-change notifications are absent. Tools and prompts are absent.

## Resource Listing

`resources/list` returns exactly these descriptors in this order and no next cursor:

| URI | Name | Title | Media type | Description intent |
|---|---|---|---|---|
| `esoweave://capabilities` | `capabilities` | `ESO Weave Capabilities` | `application/json` | Supported schema, state domains, source protocols, and available read operations |
| `esoweave://player-state` | `player-state` | `ESO Weave Player State` | `application/json` | Latest complete canonical read-only player-state snapshot with knowledge and freshness metadata |

## Resource Reads

Each successful `resources/read` returns exactly one text content item:

```json
{
  "uri": "esoweave://capabilities",
  "mimeType": "application/json",
  "text": "{...}"
}
```

The player-state response uses URI `esoweave://player-state`. The text parses as the same JSON document returned by its matching HTTP endpoint for the same snapshot revision and service generation.

## Capability Truth

- `schema_version`: `1.0.0`
- `mcp_player_state`: `true`
- `query_execution`: `false`
- No write, action, control, upload, or remote capability is implied

## Errors

- Unknown or noncanonical resource URI: MCP resource-not-found
- Serialization failure: generic MCP internal error
- Missing or invalid bearer: existing unauthorized transport response
- Invalid Host or Origin: existing forbidden transport response
- Service stopping: existing deterministic unavailable response

Error messages do not include attacker-controlled URI text, secrets, local paths, or serialized state.

## Excluded MCP Features

- Resource templates
- Resource subscriptions
- Resource list-change notifications
- Tools
- Prompts
- Sampling
- Elicitation
- Database resources or queries
- Mutating operations
