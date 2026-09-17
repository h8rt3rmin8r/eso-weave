# Feature Specification: Local Extension Stack and Contract

**Feature Branch**: `codex/s104-local-extension-contract`

**Created**: 2026-09-15

**Status**: Implemented

**Input**: Work slice S104 implements GitHub issue #175.

## User Scenarios & Testing

### User Story 1 - Implement one secure local service without reopening foundation decisions (Priority: P1)

A maintainer can implement the opt-in local HTTP API and MCP server from one approved architecture record that fixes the component versions, listener model, authentication, lifecycle, runtime boundary, discovery behavior, and compatibility policy.

**Why this priority**: Issues #176 through #180 depend on a stable foundation. Ambiguity here would create transport drift, security gaps, or repeated design work.

**Independent Test**: Review the architecture record and contract artifacts against the current desktop architecture, then demonstrate that each lifecycle and transport decision has one implementable answer.

**Acceptance Scenarios**:

1. **Given** a maintainer implementing issue #176, **when** they follow the contract, **then** one background runtime owns a loopback listener, starts both adapters atomically, reports truthful state, and shuts down within a bounded interval.
2. **Given** a local client, **when** it discovers and connects to either adapter, **then** it uses the same bearer credential, version vocabulary, canonical services, and security boundary.
3. **Given** a port conflict or partial startup failure, **when** enablement is attempted, **then** no half-running service remains and the user receives one actionable failure.

### User Story 2 - Observe every current state without semantic loss (Priority: P1)

A client can retrieve one versioned, internally coherent snapshot that represents every current public player-state observation and preserves unknown, unavailable, stale, dormant, and observed distinctions.

**Why this priority**: State parity is the primary extension capability and must remain faithful to the application rather than becoming a second interpretation.

**Independent Test**: Compare the canonical inventory field for field with current game observation, PixelBus, binding, automation, and interpretation configuration sources.

**Acceptance Scenarios**:

1. **Given** any currently known state observation, **when** the inventory is inspected, **then** the observation is mapped to one canonical external path or deliberately classified as non-public with a reason.
2. **Given** stale retained presentation values, **when** a snapshot is created, **then** current focus and signal loss are immediate while retained values carry their observation time and stale status.
3. **Given** the same logical snapshot revision, **when** HTTP and MCP adapters serialize it, **then** their machine-readable values are equivalent.

### User Story 3 - Query all application-owned SQLite data safely and consistently (Priority: P2)

A client can discover and issue bounded read-only queries against the catalog and encounter databases through either transport without gaining mutation, attachment, extension-loading, or unbounded resource capability.

**Why this priority**: Database access is intentionally broad, so its shared semantics and limits must be fixed before implementation.

**Independent Test**: Compare the database inventory with all runtime SQLite open paths and evaluate representative queries, rejected operations, and limit conditions against the query contract.

**Acceptance Scenarios**:

1. **Given** a parameterized read-only query, **when** it executes through HTTP or MCP, **then** both adapters return equivalent column metadata, typed values, rows, and truncation metadata.
2. **Given** SQL that writes, changes schema, attaches a database, loads an extension, controls a transaction, or contains multiple statements, **when** it is submitted, **then** it is rejected before application data can change.
3. **Given** an absent optional database or exceeded limit, **when** a query is attempted, **then** the response uses one deterministic shared error vocabulary.

### Edge Cases

- The preferred port is already occupied, including by another ESO Weave process.
- A rapid disable arrives while startup or a query is in progress.
- The application exits while clients remain connected or a query is blocked.
- A client disconnects while a response or query result is being produced.
- A request has a missing or malformed bearer token, an untrusted `Host`, or an untrusted browser `Origin`.
- A snapshot contains retained values from before signal or focus loss.
- A current state domain is unsupported by an older PixelBus protocol revision.
- A query returns nulls, blobs, empty rows, large text, duplicate column names, or numeric values at SQLite type boundaries.
- A catalog or encounter database is absent, being replaced, busy, or locked.
- A client uses a newer incompatible external schema major version.

## Requirements

### Functional Requirements

- **FR-001**: The decision MUST select exact reviewed versions of one embedded HTTP framework and one maintained MCP server SDK, record license and maintenance evidence, and quantify disposable release-build footprint evidence.
- **FR-002**: The production implementation MUST use Axum `=0.8.9`, RMCP `=3.4.0`, Tokio `=1.53.1`, and Tokio Util `=0.7.16`, with only required features enabled and exact pins reviewed through normal dependency updates.
- **FR-003**: HTTP and MCP MUST share one Axum router, one IPv4 loopback listener, one Tokio runtime owner, one authentication boundary, one canonical snapshot service, and one database query service.
- **FR-004**: The listener MUST bind only `127.0.0.1` on preferred port `18765`. Tests MAY request port `0`; production MUST fail truthfully rather than silently selecting a different port.
- **FR-005**: The service MUST expose HTTP under `/api/v1` and MCP Streamable HTTP at `/mcp` on the same origin. It MUST NOT expose legacy SSE, stdio, remote binding, TLS termination, or a helper process in version 1.
- **FR-006**: Every route, including discovery and MCP initialization, MUST require a persistent per-install bearer credential with at least 256 bits of cryptographic entropy. Credentials MUST NOT appear in URLs, discovery files, logs, or error text.
- **FR-007**: Requests MUST validate `Host` against the effective loopback authority and reject browser requests with an absent or non-loopback `Origin`. The server MUST emit no permissive CORS policy.
- **FR-008**: A non-secret discovery record MUST atomically publish the enabled state, effective HTTP base URL, MCP URL, process identity, external schema version, and generation. Credential retrieval remains an explicit settings action.
- **FR-009**: One persisted toggle, off by default, MUST request the whole service. Starting, running, stopping, stopped, and failed states MUST be explicit; neither transport may remain available after partial startup failure.
- **FR-010**: One dedicated background OS thread MUST own a current-thread Tokio runtime and listener. Blocking SQLite work MUST run outside the async reactor under bounded concurrency and MUST NOT borrow mutable UI, PixelBus, input-hook, or automation state.
- **FR-011**: State publication MUST use immutable revisioned snapshots. Handlers MUST capture one snapshot reference before serialization so clients cannot observe torn revisions.
- **FR-012**: Disable and application exit MUST cancel acceptance and in-flight adapter work, stop new queries, remove or invalidate discovery only when its process and generation match the current service owner, release listener and database handles, and join the owner thread within 3 seconds. Timeout MUST remain visible as a failure.
- **FR-013**: The contract MUST define one canonical player-state schema, currently `1.1.0`, with stable field paths, types, availability rules, source metadata, protocol metadata, observation time, freshness state, snapshot revision, and schema version.
- **FR-014**: Every current observation MUST appear in the maintained inventory or in an explicit non-public classification. Rendered status prose, logs, secrets, internal handles, and raw mutable controller state MUST remain non-public.
- **FR-015**: Unknown, unavailable, dormant, stale, and observed values MUST remain distinguishable. Missing knowledge MUST NOT be coerced to zero, false, an empty string, or an omitted field with ambiguous meaning.
- **FR-016**: HTTP `GET /api/v1/capabilities` and `GET /api/v1/player-state` plus MCP resources `esoweave://capabilities` and `esoweave://player-state` MUST adapt the same canonical values and error vocabulary.
- **FR-017**: MCP MUST use stateless Streamable HTTP, current stable protocol negotiation supported by RMCP 3.4.0, JSON responses, and no application session state. A compatible earlier protocol revision MAY be negotiated only when the SDK provides it without contract loss.
- **FR-018**: The runtime database inventory MUST contain exactly `catalog` and `encounters`, with truthful availability and schema discovery. Filesystem paths MUST NOT be exposed.
- **FR-019**: HTTP `GET /api/v1/databases`, `POST /api/v1/databases/{database_id}/query`, MCP resource `esoweave://databases`, and MCP tool `query_database` MUST adapt one query service.
- **FR-020**: Query inputs MUST use one SQL statement of at most 16 KiB and at most 64 named or positional typed parameters. Only read-only SQLite operations are permitted.
- **FR-021**: Read-only enforcement MUST combine read-only and no-follow open flags, `query_only`, defensive configuration, disabled extension loading, an authorizer denylist, single-statement preparation, and a final statement-readonly check. No one check is sufficient by itself.
- **FR-022**: The query service MUST cap concurrent external queries at 2, execution at 2 seconds, returned rows at 1,000, serialized result data at 1 MiB, and request bodies at 64 KiB. Limit completion MUST return explicit truncation or structured error metadata.
- **FR-023**: Query results MUST preserve SQLite null, integer, real, text, and blob types. Integers and reals MUST use exact JSON-safe string encodings, including named non-finite real values. Blobs MUST use base64 plus an explicit type marker. Column order and duplicate names MUST remain representable.
- **FR-024**: All HTTP and MCP failures MUST map from one stable structured error model containing code, message, retryability, and optional limit detail, without internal paths, SQL contents, credentials, or stack traces.
- **FR-025**: External schema compatibility MUST follow semantic versioning: additive optional fields are minor-compatible, clarified prose and new enum values are patch-compatible only where clients already handle unknown values, and removals or semantic/type changes require a new major route/resource contract.
- **FR-026**: The decision MUST reconcile issues #176 through #180 so their implementations refer to the selected architecture, canonical inventories, parity requirements, and limits.
- **FR-027**: S104 MUST remain a decision and contract slice. It MUST NOT ship a listener, setting, endpoint, MCP server, query executor, new production dependency, or claim that the extension surface is already available.
- **FR-028**: Build-plan chronology, architecture records, project tracking, and a dated changelog decision MUST be updated under repository policy.

### Key Entities

- **Local extension service**: The atomic lifecycle owner for the shared listener, HTTP adapter, MCP adapter, authentication, discovery, snapshot reference, and query executor.
- **Discovery record**: Non-secret local connection metadata for the current service generation.
- **Canonical snapshot**: An immutable, versioned observation of all public state captured at one logical revision.
- **Observed value**: A typed value paired with knowledge, source, protocol, observation-time, and freshness semantics.
- **Database descriptor**: Stable database identifier, availability, schema metadata, and capability limits without a filesystem path.
- **Query request and result**: One bounded parameterized statement and its ordered typed columns, rows, truncation facts, and timing metadata.
- **Extension error**: Transport-neutral code, safe message, retryability, and optional limit detail.

## Success Criteria

### Measurable Outcomes

- **SC-001**: The decision names exact selected versions, licenses, maintenance evidence, and release-probe sizes for the HTTP-only and HTTP-plus-MCP builds.
- **SC-002**: Every requirement needed by issues #176 through #180 has one concrete implementation choice and no clarification marker remains.
- **SC-003**: The state inventory accounts for 100% of current public observations and every entry has path, type, knowledge, freshness, source, and protocol semantics.
- **SC-004**: HTTP and MCP contract tables map every capability, player-state, database-discovery, and query operation to one shared service.
- **SC-005**: The database inventory covers 100% of runtime SQLite databases owned by the application and documents every current table family.
- **SC-006**: Security review finds loopback binding, bearer authentication, Host and Origin validation, secret handling, query defense in depth, and bounded shutdown explicit.
- **SC-007**: Query bounds are exactly 2 concurrent requests, 2 seconds, 1,000 rows, 1 MiB results, 16 KiB SQL, 64 parameters, and 64 KiB bodies across both transports.
- **SC-008**: Repository policy, links, spelling, formatting, UTF-8, LF, mojibake, tests, and hosted CI pass.

## Assumptions

- The extension is intentionally local and opt-in; remote access and remote authorization are separate future designs.
- A fixed preferred port makes common MCP client configuration stable, while a test-only ephemeral port keeps lifecycle tests isolated.
- A persistent bearer credential plus strict loopback, Host, and Origin checks is proportionate for the local version 1 surface. OAuth authorization-server behavior is out of scope.
- RMCP 3.4.0 is selected despite its same-day release because it is the official Tier 1 Rust SDK, supports the current protocol, and exposes the required Axum-compatible service and validation controls. Exact pinning and integration tests contain that freshness risk.
- Public state represents facts already available to the application. It does not expose logs, credentials, filesystem paths, or command/control capability.

## Out of Scope

- Production implementation owned by issues #176 through #180.
- Remote interfaces, LAN binding, TLS, OAuth, multi-user authorization, browser applications, write APIs, game commands, or automation control.
- Arbitrary filesystem database selection, raw file download, database writes, virtual-table creation, extension loading, attachment, or transaction control.
- Historical telemetry not already stored in the catalog or encounter databases.
