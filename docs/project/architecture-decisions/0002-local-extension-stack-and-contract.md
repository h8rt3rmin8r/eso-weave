# ADR 0002: Local Extension Stack and Contract

Date: 2026-09-15

Status: Accepted

## Context

Epic #174 adds an opt-in local HTTP API and MCP server for complete current player state and bounded read-only access to ESO Weave's application databases. The desktop application is synchronous, already owns SQLite data, and has safety-sensitive PixelBus, input, automation, and UI loops that a client must not block. Issues #176 through #180 need one transport and data authority before production code lands.

Loopback binding alone is not an authorization boundary. Browser DNS rebinding, unrelated local processes, slow clients, unsafe SQL, torn state snapshots, partial startup, and unbounded shutdown all require explicit handling.

## Decision

Use exact future production versions Axum 0.8.9, RMCP 3.4.0, Tokio 1.53.1, and Tokio Util 0.7.16 with required features only. RMCP is the official Apache-2.0 Rust MCP SDK and mounts as a Tower service in MIT-licensed Axum. A disposable stripped release probe measured 566,272 bytes for minimal Axum and 1,830,912 bytes with RMCP, an acceptable approximately 1.21 MiB increment.

Use one IPv4 listener at `127.0.0.1:18765`, one Axum router, and one dedicated OS thread containing a current-thread Tokio runtime. HTTP lives under `/api/v1`; stateless MCP Streamable HTTP lives at `/mcp`. Production fails visibly on port collision. Tests may bind port 0.

Require one persistent per-install bearer token with at least 256 bits of entropy on every operation. Validate Host and browser Origin against the effective loopback authority, emit no permissive CORS policy, and keep the token out of URLs, discovery, logs, and errors. Atomically publish non-secret endpoint, process, generation, and schema metadata only while the service is running.

Treat both transports as adapters over one immutable revisioned snapshot service and one bounded database query service. The version 1 schema preserves observed, unknown, unavailable, dormant, fresh, and stale distinctions with source, protocol, and observation-time metadata. HTTP capabilities and player-state routes match MCP resources; HTTP database discovery and query match an MCP resource and tool.

Expose exactly two runtime database identities, `catalog` and `encounters`. Enforce read-only queries with read-only and no-follow opening, query-only and defensive configuration, absent extension loading, an authorizer denylist, exactly one prepared statement, a final statement-readonly check, progress cancellation, and reduced SQLite limits. Bound queries to 2 concurrent, 2 seconds, 1,000 rows, 1 MiB serialized data, 16 KiB SQL, 64 parameters, and 64 KiB request bodies. Materialize bounded results before transport serialization.

Start and stop HTTP and MCP atomically. Partial startup leaves no listener. Disable and application exit stop acceptance, invalidate discovery, cancel adapter and query work, release handles, and join the owner thread within 3 seconds. A timeout remains a visible failure.

Schema version 1 follows semantic compatibility. Additive optional fields may advance the minor version. Removal, type change, or semantic change requires a new major route and resource contract.

The complete implementer authority is [the local extension contract](../local-extension-contract.md). Detailed evidence and inventories live in [the S104 spec packet](../../../specs/104-local-extension-contract/spec.md).

## Alternatives

- Actix Web and Salvo are maintained but do not match RMCP's direct Tower and Axum integration as closely and would add adaptation or broader defaults.
- Unofficial Rust MCP SDKs were rejected because the official SDK provides current protocol and conformance ownership.
- Hand-written Hyper and MCP framing would make protocol negotiation, transport behavior, and security repository-owned.
- Separate HTTP and MCP listeners or caches would duplicate lifecycle, authorization, and semantics.
- Ephemeral production ports would complicate ordinary MCP client configuration. Silent fallback would hide collision behavior.
- OAuth authorization-server behavior is disproportionate for the initial single-user loopback surface. Remote access remains out of scope.
- Statement-readonly alone is insufficient because SQLite can classify some control statements as read-only.

## Consequences

- RMCP 3.4.0 released on the decision date. Exact pins, minimal features, protocol integration tests, and reviewed upgrades are mandatory containment.
- The future executable gains one optional background runtime and approximately 1.21 MiB of measured stripped dependency footprint.
- Clients must obtain the bearer token through an explicit settings action; the discovery record alone is intentionally insufficient.
- State and query behavior cannot diverge between HTTP and MCP without violating the adapter contract.
- A port collision is visible and recoverable but does not silently relocate the service.
- Version 1 exposes observation and read-only data only. It does not authorize game commands, input synthesis, automation control, writes, or remote access.
- S104 records the decision only. Issues #176 through #180 own production implementation and user documentation.

## References

- [Axum](https://github.com/tokio-rs/axum)
- [Official MCP Rust SDK](https://github.com/modelcontextprotocol/rust-sdk)
- [MCP Streamable HTTP security](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports)
- [SQLite security recommendations](https://www.sqlite.org/security.html)
- [S104 research](../../../specs/104-local-extension-contract/research.md)
