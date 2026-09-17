# Quickstart: Verify MCP Player-State Resources

## Prerequisites

- Rust toolchain pinned by `rust-toolchain.toml`
- Repository dependencies available through the locked Cargo graph
- A loopback local-service fixture with a known bearer token

## Focused verification

```powershell
cargo test --test local_service mcp_
```

The focused tests start the real local service and use RMCP's official client transport to initialize, list resources, read both JSON documents, compare HTTP/MCP parity, and exercise errors and lifecycle boundaries.

## Full merge gates

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

Run repository documentation, encoding, BOM, mojibake, forbidden-dash, and link checks named by the current CI workflows before commit.

## Expected contract

- `/mcp` initializes successfully with valid bearer authentication.
- Resource discovery returns exactly `esoweave://capabilities` and `esoweave://player-state`.
- Each read contains one `application/json` text item.
- MCP and HTTP documents are equal for one stable revision and generation.
- Unknown resources return resource-not-found.
- Database queries, tools, prompts, templates, and subscriptions are absent.
