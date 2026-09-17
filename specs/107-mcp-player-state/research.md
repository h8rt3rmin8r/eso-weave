# Research: MCP Player-State Resources

## Decision 1: Use RMCP's native resource protocol

**Decision**: Implement `ServerHandler::get_info`, `list_resources`, and `read_resource` for two fixed resources.

**Rationale**: Native resources give standard MCP clients discovery and reads without a product-specific tool or wire format. RMCP already owns initialize negotiation, JSON-RPC framing, error codes, and Streamable HTTP behavior.

**Alternatives considered**:

- Tools returning state were rejected because the data is read-only addressable context, not an action.
- A custom JSON-RPC method was rejected because it would not interoperate with standard clients.
- Resource templates were rejected because the inventory has no parameters.

## Decision 2: Reuse the S106 snapshot publisher directly

**Decision**: Construct a cloneable handler with the existing `SnapshotPublisher` and service generation. Clone one immutable snapshot reference per read, then serialize it outside publisher locks.

**Rationale**: This guarantees HTTP/MCP parity and retains S106 revision, knowledge, freshness, and source semantics. It also avoids divergent caches and lock ordering.

**Alternatives considered**:

- A separate MCP cache was rejected because it could lag HTTP and require a second revision authority.
- Calling the HTTP endpoint internally was rejected because it adds transport overhead and couples two adapters unnecessarily.
- Serializing while holding a mutable producer lock was rejected because a slow client could impede publication.

## Decision 3: Keep the server stateless

**Decision**: Continue using `NeverSessionManager`, JSON responses, no legacy SSE, and no resource subscriptions or list-change notifications.

**Rationale**: The resources are fixed and current-state reads need no per-client state. This preserves the S104 and S105 security and lifecycle contract.

**Alternatives considered**:

- Stateful sessions were rejected because they add shutdown, retention, and recovery complexity without user value.
- Subscriptions were rejected because they require notification delivery, backpressure, and session ownership outside S107.

## Decision 4: Test with the official RMCP client

**Decision**: Enable RMCP's client and Reqwest Streamable HTTP transport features for dev builds and run contract tests over a real listening service.

**Rationale**: The test covers initialize negotiation, authentication headers, resource discovery, content decoding, and protocol error mapping. Direct handler tests alone could miss transport-level incompatibility.

**Alternatives considered**:

- Handwritten JSON-RPC requests were rejected as a primary contract test because they duplicate protocol assumptions.
- Handler-only mocks were rejected because issue #178 explicitly requires standard client interoperability.

## Decision 5: Return standard resource errors

**Decision**: Exact canonical URI matches succeed. Every other URI returns RMCP resource-not-found. Serialization failure returns a generic internal error.

**Rationale**: Stable protocol codes are actionable for clients, while generic messages avoid reflecting untrusted text or disclosing implementation details.

**Alternatives considered**:

- Normalizing case, suffixes, queries, or fragments was rejected because resource identifiers are exact and fixed.
- Echoing invalid URIs was rejected because it is unnecessary reflection of attacker-controlled data.

## Decision 6: Advertise only implemented capabilities

**Decision**: Enable MCP resources without subscribe or list-change options. Keep tools and prompts absent. Set `mcp_player_state` true and `query_execution` false in the canonical capability document.

**Rationale**: Capability negotiation and the document must be truthful at S107 boundaries. Database work remains issue #179.
