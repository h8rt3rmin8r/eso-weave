# Research: Local Extension Stack and Contract

## Selected stack

### HTTP: Axum 0.8.9

Axum is maintained in the Tokio organization, uses the Tower service ecosystem, supports explicit graceful shutdown, and can mount RMCP's Streamable HTTP service directly. Version 0.8.9 declares MIT licensing and Rust 1.80 minimum support. The future production dependency should disable defaults and enable only `http1`, `json`, and `tokio` unless implementation evidence requires another feature.

### MCP: RMCP 3.4.0

RMCP is the official Model Context Protocol Rust SDK. Version 3.4.0 declares Apache-2.0 licensing and Rust 1.88 minimum support, below ESO Weave's Rust 1.96 toolchain. Its `transport-streamable-http-server` feature exposes a Tower-compatible service that Axum can mount. It supports stateless sessions, cancellation, allowed Host and Origin controls, and the current stable MCP protocol revision.

RMCP 3.4.0 released on 2026-09-15. That freshness is a real risk. Exact version pinning, feature minimization, adapter parity tests, protocol integration tests, and normal reviewed dependency updates contain it better than selecting an unofficial or obsolete implementation.

### Runtime: Tokio 1.53.1 and Tokio Util 0.7.16

One dedicated OS thread will own a current-thread Tokio runtime. This isolates the synchronous eframe loop from async network ownership. Axum and RMCP share the runtime; blocking SQLite work uses a bounded blocking executor and never runs on the async reactor.

## Disposable footprint evidence

Two ignored release probes used Rust 1.96, stripping, LTO, one codegen unit, and abort-on-panic:

| Probe | Normal packages | Release executable |
| --- | ---: | ---: |
| Minimal Axum HTTP listener | 45 | 566,272 bytes |
| Same listener plus RMCP Streamable HTTP server | 103 | 1,830,912 bytes |

The approximately 1.21 MiB incremental stripped footprint is proportionate for an optional capability embedded in the existing desktop executable. The probes remain disposable research and add no production dependency.

## Alternatives

- Actix Web 4.15.0 is maintained and compatibly licensed, but its default surface is broader and RMCP already exposes Tower and Axum integration. An Actix adapter would add a second service abstraction.
- Salvo 0.96.0 is maintained and Apache-2.0 licensed, but it has no equally direct official RMCP mounting path and a broader default surface.
- `rust-mcp-sdk` 2.0.0 is maintained and MIT licensed, but it is not the official SDK and its defaults span several transports and authentication capabilities not required here.
- `mcp-sdk` 0.0.3 is too immature for a durable desktop boundary.
- Direct Hyper plus hand-written MCP framing was rejected because protocol negotiation, Streamable HTTP behavior, cancellation, and security maintenance would become repository-owned protocol code.

## Listener and discovery

Use one IPv4 loopback listener at `127.0.0.1:18765`. A fixed preferred port keeps HTTP and common MCP client configuration stable. Production fails visibly on collision rather than silently moving. Integration tests may request port `0` and consume the effective address.

When running, atomically write a non-secret discovery record beneath the application's existing per-user data area. It contains endpoints, process identity, schema version, and generation, never the bearer credential. Clean stop or failed-start cleanup removes only a record whose process and generation match the current attempt; a colliding process cannot delete the live owner's record. Clients must also reject a record whose process or generation is no longer live.

## Authentication and browser boundary

Generate a persistent random bearer token with at least 256 bits of entropy and keep it in the user configuration area with restrictive filesystem permissions where supported. The settings interface owns explicit reveal or copy behavior. Every HTTP and MCP request requires the token.

The MCP transport specification requires Origin validation and recommends loopback binding because DNS rebinding can otherwise reach local servers. This contract also validates Host against the effective listener and emits no permissive CORS policy. A local bearer token is intentionally smaller than an OAuth authorization-server implementation and proportionate to this single-user loopback surface.

## MCP behavior

Use the current stable Streamable HTTP transport at `/mcp`, stateless requests, JSON responses, and RMCP protocol negotiation. Do not expose legacy SSE, stdio, or application session state. Capabilities and player state are resources; the bounded database operation is a tool. All values come from the same canonical services as HTTP.

## SQLite enforcement

SQLite's statement-readonly check is not sufficient alone because transaction-control and attachment statements can still report as read-only. Future implementation must combine read-only and no-follow open flags, query-only and defensive configuration, absent extension loading, an authorizer denylist, single-statement preparation, a final statement-readonly check, progress cancellation, reduced SQLite limits, and exact request bounds.

The shared query service materializes its bounded result before transport serialization so a slow client does not retain a database lock.

## Primary references

- [Axum repository and documentation](https://github.com/tokio-rs/axum)
- [RMCP official Rust SDK](https://github.com/modelcontextprotocol/rust-sdk)
- [MCP Streamable HTTP transport](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports)
- [MCP authorization](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization)
- [SQLite security recommendations](https://www.sqlite.org/security.html)
- [SQLite authorizer interface](https://www.sqlite.org/c3ref/set_authorizer.html)
- [SQLite statement read-only behavior](https://www.sqlite.org/c3ref/stmt_readonly.html)
