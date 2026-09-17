# Contract: HTTP Player State 1.0.0

## `GET /api/v1`

Returns the same capability document as `/api/v1/capabilities` for discoverability.

## `GET /api/v1/capabilities`

Returns HTTP 200 and a JSON capability document containing:

- external schema version `1.0.0`
- domains `application`, `game`, `pixel_bus`, `player`, `automation`, and `interpretation`
- HTTP operations `capabilities` and `player_state`
- database identifiers `catalog` and `encounters`, with query execution false
- MCP player-state availability false until issue #178
- current PixelBus protocol and layout support facts

## `GET /api/v1/player-state`

Returns HTTP 200 and one immutable canonical player-state document. The response includes one positive `snapshot_revision`, one RFC 3339 `captured_at`, the current `service_generation`, capabilities, and all six domains.

## Errors

- A non-GET method on a defined read route returns HTTP 405 with code `method_not_allowed`.
- An unknown `/api/v1/*` path returns HTTP 404 with code `not_found`.
- Existing security and shutdown errors remain unchanged.
- Error bodies contain `error.code`, `error.message`, and `error.retryable` only.

## Concurrency

The handler clones one immutable revision before serialization. It holds no application, controller, input, or publisher lock while producing or sending the response.
