# Feature Specification: Read-Only Database Queries

**Feature Branch**: `codex/s108-database-query`

**Created**: 2026-09-17

**Status**: Implemented

**Input**: Work slice S108 implements GitHub issue #179 under Plan 045 and the accepted S104 database query contract.

## Clarifications

### Session 2026-09-17

- Q: Which databases are externally visible? -> A: Exactly `catalog` and `encounters`. Discovery always lists both and reports absent or unusable files truthfully without exposing paths.
- Q: How are schemas discovered? -> A: The inventory reports user tables and views plus ordered visible columns, declared SQLite types, nullability, and primary-key positions. Internal `sqlite_%` objects and filesystem details are excluded.
- Q: How are parameters represented? -> A: Positional parameters are an ordered array without names and support `?` or contiguous `?N` slots. Named parameters all carry unique exact SQLite names beginning with `:`, `@`, or `$`. Mixed modes, missing slots, extra values, and duplicate names are invalid.
- Q: Where does query work run? -> A: One shared service admits at most two queries globally and moves synchronous SQLite work to blocking workers. HTTP and MCP await materialized results without running SQLite on the local-service runtime thread.
- Q: How are cancellation and timeouts enforced? -> A: A progress handler checks the service-generation cancellation token and a two-second monotonic deadline. Shutdown cancels in-flight statements, and no connection or statement survives result materialization.
- Q: How is the byte bound interpreted? -> A: The complete compact JSON result envelope must remain at or below 1 MiB. Rows are admitted only when a complete row fits. If no row can fit, the request fails with `result_too_large`; otherwise it succeeds with byte truncation metadata.
- Q: How do MCP errors preserve parity? -> A: `query_database` returns canonical structured success or structured error content. The error object is identical to HTTP after removing transport framing.

## User Scenarios & Testing

### User Story 1 - Discover queryable databases (Priority: P1)

A local client can discover the fixed database inventory, current availability, and public schema without learning local filesystem paths.

**Why this priority**: Clients need truthful schema and availability facts before generating or submitting SQL.

**Independent Test**: Start the service over temporary catalog and encounter fixtures, compare HTTP inventory with `esoweave://databases`, then remove either optional file and confirm both transports report it unavailable.

**Acceptance Scenarios**:

1. **Given** both supported databases, **when** either transport requests inventory, **then** exactly `catalog` and `encounters` appear with equivalent ordered schema documents.
2. **Given** an absent database, **when** inventory is requested, **then** its stable identifier remains present with `available: false`, an empty schema, and no path.
3. **Given** an unusable database, **when** inventory is requested, **then** discovery remains successful and reports a safe availability reason without leaking SQLite or filesystem detail.

### User Story 2 - Execute equivalent typed read-only queries (Priority: P1)

A local client can submit one parameterized read-only statement through HTTP or MCP and receive the same typed columns, rows, limits, and truncation facts.

**Why this priority**: Query execution and adapter parity are the primary outcome of issue #179.

**Independent Test**: Run identical positional and named queries against temporary fixtures through both transports and compare their structured results after excluding elapsed time.

**Acceptance Scenarios**:

1. **Given** a supported database and valid `SELECT`, **when** either adapter executes it, **then** ordered duplicate column names and every SQLite value type are preserved losslessly.
2. **Given** the same database generation, SQL, parameters, and row limit, **when** HTTP and MCP execute it, **then** both return equivalent results and canonical error codes.
3. **Given** a current catalog replacement, **when** a later request starts, **then** it opens the replacement generation while an already materialized result remains unchanged.

### User Story 3 - Reject unsafe or unbounded work (Priority: P1)

The desktop remains responsive and application data cannot be changed by external SQL, oversized work, excessive concurrency, or slow clients.

**Why this priority**: Query access shares the desktop process and must preserve local data integrity and bounded shutdown.

**Independent Test**: Attempt mutation, schema changes, attachment, unsafe pragmas, multiple statements, excessive parameters, oversized SQL, long recursive queries, three concurrent calls, large rows, disconnects, and shutdown under load.

**Acceptance Scenarios**:

1. **Given** any mutating or schema-changing statement, **when** it is prepared, **then** layered read-only enforcement rejects it deterministically and the database bytes remain unchanged.
2. **Given** two active external queries, **when** a third begins, **then** it fails immediately with retryable `query_busy`.
3. **Given** a query exceeding time, row, or byte bounds, **when** it runs, **then** it is interrupted or truncated according to the fixed contract without retaining a database lock.
4. **Given** service shutdown, **when** query work is active, **then** cancellation closes all query connections within the existing lifecycle bound.

### Edge Cases

- Catalog or encounter history is absent, replaced, malformed, locked, or deleted between discovery and execution.
- SQL is empty, comment-only, contains trailing comments, or contains a second statement.
- A statement is read-only according to its first token but invokes a denied pragma, virtual table, attachment, transaction, or schema action.
- Parameters use gaps, repeated names, unsupported prefixes, mixed named and positional slots, non-finite real input, invalid base64, or values outside signed 64-bit range.
- Results contain nulls, empty text, invalid UTF-8 SQLite text, empty blobs, duplicate column names, infinities, or an empty row set.
- A single complete row exceeds the byte limit.
- A client disconnects after execution begins or reads the materialized response slowly.
- Catalog replacement or service stop races with connection opening.

## Requirements

### Functional Requirements

- **FR-001**: Discovery MUST list exactly `catalog` and `encounters` in stable order through `GET /api/v1/databases` and `esoweave://databases`.
- **FR-002**: Each database descriptor MUST include identifier, role, availability, a safe availability reason, and an ordered public schema containing user tables and views with visible columns.
- **FR-003**: Discovery MUST exclude paths, SQLite internal objects, triggers, indexes, SQL definitions, secrets, and implementation diagnostics.
- **FR-004**: An absent or unusable database MUST remain in discovery with `available: false` and MUST NOT fail discovery of the other database.
- **FR-005**: HTTP MUST expose `POST /api/v1/databases/{database_id}/query`; MCP MUST expose exactly one `query_database` tool in addition to the three fixed resources.
- **FR-006**: Both adapters MUST invoke one shared validator, executor, result model, and error vocabulary.
- **FR-007**: A request MUST contain exactly one non-empty statement no larger than 16,384 UTF-8 bytes, at most 64 typed parameters, and an optional integer row limit from 1 through 1,000.
- **FR-008**: Positional and named parameters MUST follow the clarified exact binding rules; mixed or incomplete bindings MUST return `invalid_request`.
- **FR-009**: Parameter values MUST support null, signed 64-bit integer decimal strings, finite real decimal strings, text, base64 blob, and boolean input bound as integer 0 or 1.
- **FR-010**: Each request MUST open a short-lived connection to the selected current path using read-only, URI, no-mutex, and no-follow flags.
- **FR-011**: Each connection MUST enable query-only, defensive, and untrusted-schema behavior before preparing external SQL.
- **FR-012**: The SQLite authorizer MUST deny mutation, DDL, attachment, detachment, reindex, analyze, transaction, savepoint, virtual-table mutation, pragmas, and unknown authorization actions.
- **FR-013**: Preparation MUST reject multiple statements, comment-only SQL, virtual-table use, and any prepared statement not reported read-only.
- **FR-014**: SQLite runtime limits MUST constrain SQL length, variables, values, columns, expression depth, compound selects, function arguments, attached databases, trigger depth, worker threads, and virtual-machine operations.
- **FR-015**: One global shared service MUST admit at most two concurrent external queries and reject excess work immediately with retryable `query_busy`.
- **FR-016**: A progress handler MUST interrupt work after 2,000 ms or when the service generation is cancelled, mapping those cases to `query_timeout` or `service_stopping`.
- **FR-017**: Results MUST preserve ordered columns and rows using null, integer, real, text, and blob tagged values. Integers and finite reals MUST use round-trippable strings, non-finite output MUST use named strings, and blobs MUST use standard padded base64.
- **FR-018**: Results MUST contain no more than 1,000 rows and no more than 1,048,576 compact JSON bytes, with explicit row or byte truncation facts when at least one complete row fits.
- **FR-019**: A result in which no complete row can fit MUST return `result_too_large`; partial rows MUST never be returned.
- **FR-020**: The executor MUST fully materialize the bounded result and close its statement and connection before either adapter serializes or transmits it.
- **FR-021**: Dynamic catalog replacement MUST affect later requests without changing any already-open connection or materialized result.
- **FR-022**: Unknown database identifiers, unavailable databases, invalid SQL, denied SQL, busy databases, timeouts, limit failures, stopping, and internal failures MUST map to deterministic non-disclosing canonical errors.
- **FR-023**: HTTP error status and MCP structured error content MUST preserve the same error code, message, and retryability.
- **FR-024**: Capability documents MUST advertise database discovery and query execution only after both adapters are installed.
- **FR-025**: Tests MUST cover inventory parity, query parity, every value type, named and positional binding, empty results, mutation rejection with byte preservation, all exact bounds, locks, replacement, cancellation, disconnect, and shutdown.
- **FR-026**: Existing safety-critical input, addon, encounter, player-state, and local-service tests MUST remain unchanged and passing.
- **FR-027**: Plan 045, the active-plan index, canonical local-extension summary, and changelog MUST reflect S108 while preserving issue #180 ownership of public documentation and final end-to-end coverage.
- **FR-028**: S108 MUST NOT add writes, remote binding, arbitrary database paths, query persistence, credentials, telemetry, agent orchestration, or public documentation reserved for S109.

### Key Entities

- **Database query service**: Cloneable shared authority for inventory, admission, dynamic paths, validation, execution, cancellation, and materialization.
- **Database descriptor**: Safe public availability and schema facts for one fixed identifier.
- **Query request**: One database identifier, SQL statement, homogeneous parameter mode, and optional row limit.
- **Query result**: Ordered columns, typed rows, truncation facts, elapsed time, and fixed limit metadata.
- **Query error**: Stable code, safe message, and retryability shared by both transports.

## Success Criteria

### Measurable Outcomes

- **SC-001**: HTTP and an official RMCP client discover equivalent descriptors for exactly two databases in automated tests.
- **SC-002**: Equivalent HTTP and MCP queries return identical structured data except for independently measured elapsed milliseconds.
- **SC-003**: Mutation and schema-change attempts leave fixture hashes and row counts unchanged in every test.
- **SC-004**: Three simultaneous admitted requests produce exactly two active executions and one immediate retryable busy error.
- **SC-005**: Time-bound queries stop within 2,000 ms plus a 500 ms scheduling allowance, and service shutdown remains within its existing three-second bound.
- **SC-006**: Every serialized successful result is at most 1,048,576 bytes and contains only complete rows.
- **SC-007**: Full repository CI parity, release build, documentation policy, encoding, LF, mojibake, and forbidden-dash gates pass.

## Assumptions

- Rusqlite 0.40.2 with bundled SQLite remains the database authority.
- Enabling rusqlite's `hooks` and `limits` features exposes the required safe wrappers without enabling extension loading.
- The encounter database remains optional until encounter history is first stored.
- S109 will publish copy-paste client documentation and the final complete extension end-to-end suite.

## Out of Scope

- SQL writes, migrations, transactions, attachments, extensions, arbitrary paths, or non-SQLite stores.
- Query subscriptions, streaming rows, cursors, saved queries, or pagination tokens.
- Remote clients, additional authentication modes, or credential changes.
- Public setup, troubleshooting, examples, and compatibility documentation from issue #180.
