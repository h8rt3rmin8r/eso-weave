# Research: Read-Only Database Queries

**Date**: 2026-09-17

## Decision 1: Use rusqlite safe hooks and limits

**Decision**: Enable rusqlite 0.40.2 `hooks` and `limits` features while retaining bundled SQLite and leaving extension loading disabled.

**Rationale**: Rusqlite exposes safe wrappers for the SQLite authorizer, progress handler, and runtime limits. This directly implements the accepted defense-in-depth contract without raw FFI or a second database library.

**Alternatives considered**:

- Token-only SQL validation was rejected because it cannot authorize SQLite operations after parsing and is vulnerable to grammar drift.
- Raw SQLite FFI was rejected because rusqlite already exposes the required owned callback and limit APIs.

## Decision 2: Execute SQLite on blocking workers

**Decision**: Acquire one of two shared permits with `try_acquire_owned`, then call `tokio::task::spawn_blocking` for inventory or query execution.

**Rationale**: Rusqlite is synchronous. Running it on the local service's current-thread Tokio runtime would stall HTTP, MCP, cancellation, and shutdown. Immediate permit acquisition provides the fixed retryable busy behavior.

**Alternatives considered**:

- A dedicated permanent database thread was rejected because two independent queries are allowed and lifecycle ownership would become more complex.
- Running directly inside handlers was rejected because a long query would block every local-service task.

## Decision 3: Use a private dynamic path registry

**Decision**: Store current catalog and encounter paths behind a cloneable lock shared by the app model and query service. Clone one path before each connection open.

**Rationale**: Catalog installs and rollbacks replace the active generation. New queries must follow that selection while in-flight queries remain coherent. Paths must never appear in external documents or errors.

**Alternatives considered**:

- Keeping one long-lived connection was rejected because it would pin a retired generation and could hold locks across clients.
- Recomputing selection from configuration in request handlers was rejected because the application already owns the authoritative accepted resolution.

## Decision 4: Use layered SQLite enforcement

**Decision**: Combine `SQLITE_OPEN_READ_ONLY`, `SQLITE_OPEN_URI`, `SQLITE_OPEN_NO_MUTEX`, `SQLITE_OPEN_NOFOLLOW`, query-only, defensive mode, untrusted schema, no virtual tables at prepare time, a fail-closed authorizer, statement readonly verification, and runtime limits.

**Rationale**: No single guard fully expresses the contract. Independent layers make a parser, schema, or platform mistake insufficient to authorize mutation.

**Alternatives considered**:

- A SELECT-prefix allowlist was rejected because writable common-table expressions, pragmas, and SQLite extensions are not safely classified by prefixes.
- Read-only open mode alone was rejected because the contract also excludes schema attachment, unsafe pragmas, resource abuse, and multiple statements.

## Decision 5: Materialize exact typed rows

**Decision**: Copy every accepted SQLite value into owned tagged data on the blocking worker. Encode signed integers and finite reals as canonical strings, named non-finite reals as fixed strings, and blobs with standard padded base64.

**Rationale**: Owned rows close all SQLite resources before transport serialization and preserve values across JSON implementations without numeric precision loss.

**Alternatives considered**:

- Streaming rows was rejected because slow clients would retain statements and database locks.
- JSON numbers were rejected because many clients cannot represent every signed 64-bit integer exactly.

## Decision 6: Return structured MCP results

**Decision**: Advertise one `query_database` tool with a fixed JSON schema. Return success through structured content and execution failures through structured error content using the canonical HTTP error object.

**Rationale**: Structured content preserves adapter parity and avoids forcing clients to parse prose. Protocol-level invalid-tool errors remain reserved for unknown tool names.

**Alternatives considered**:

- Returning only text content was rejected because it weakens typed parity.
- Converting every query failure into a JSON-RPC error was rejected because the tool executed and produced a domain failure.
