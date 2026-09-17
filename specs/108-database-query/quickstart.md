# Quickstart: Verify S108 Database Queries

This is a contributor verification path. Public client setup remains S109.

## 1. Run focused service tests

```powershell
cargo test --test database_query --locked
cargo test --test local_service --locked database
```

Expected: inventory, typed execution, mutation rejection, exact limits, HTTP parity, and official RMCP client parity pass.

## 2. Run the full merge gate

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
cargo build --release --locked
```

Expected: every command exits successfully.

## 3. Inspect scope

Confirm that the diff contains:

- one shared database query service
- one HTTP inventory route and one HTTP query route
- one MCP database resource and one MCP query tool
- dynamic current catalog path updates
- capability, plan, and changelog updates

Confirm that the diff does not contain:

- write SQL or arbitrary paths
- remote binding or authentication changes
- streaming SQLite rows
- public integration documentation reserved for S109
