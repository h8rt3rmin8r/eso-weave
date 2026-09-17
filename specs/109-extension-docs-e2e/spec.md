# Feature Specification: Local Extension Documentation and End-to-End Verification

**Feature Branch**: `codex/s109-extension-docs-e2e`

**Created**: 2026-09-17

**Status**: Implemented

**Input**: Work slice S109 implements GitHub issue #180 and completes Plan 045 and epic #174 against the shipped S105 through S108 local extension surface.

## Clarifications

### Session 2026-09-17

- Q: Where does the public contract live? -> A: One self-contained reference guide is the user and integrator entry point. Existing settings, architecture, responsible-use, data, and troubleshooting pages link to it instead of duplicating the contract.
- Q: How is player-state documentation kept complete? -> A: A checked-in machine-readable field inventory records every `PUBLIC_PATHS` entry with type, meaning, source, and availability or freshness semantics. An automated test compares the inventory paths with the production constant exactly.
- Q: What makes examples safe and copyable? -> A: Examples use loopback endpoints and an explicit placeholder or environment variable for the bearer credential. No real credential, filesystem path, or captured player value is committed. Documentation smoke tests validate the examples and named operations against production constants.
- Q: What counts as cross-transport parity? -> A: One end-to-end scenario starts the production listener over deterministic snapshot and database fixtures, uses raw authenticated HTTP plus the official RMCP client, and compares capabilities, player state, database inventory, successful query results, and denied-query errors. Only transport framing and independently measured `elapsed_ms` may differ.
- Q: Which lifecycle cases belong in the final suite? -> A: Disabled state, enable and discovery publication, authenticated use, port collision, recovery after the collision clears, client disconnect, restart with a new generation, and bounded clean shutdown all run against the production controller.
- Q: How much schema prose is required? -> A: The guide documents the top-level envelope, the common observation wrapper, every public field path, both database descriptors, query inputs and typed outputs, fixed limits, stable error codes, compatibility rules, and unavailable or stale behavior.
- Q: Does S109 change the service contract? -> A: No. S109 documents and verifies the shipped version `1.0.0` contract. Any discovered mismatch is fixed only when necessary to make current behavior truthful and remains within issue #180.

## User Scenarios and Testing

### User Story 1 - Connect using published guidance (Priority: P1)

A user or integrator can enable the optional local service, discover its endpoints, provide the copied bearer credential, and complete one HTTP and one MCP request without reading source code.

**Why this priority**: The extension surface has no practical value if a trusted local client cannot discover and authenticate to it from the published documentation.

**Independent Test**: Follow only the published guide against a running deterministic service fixture, retrieve capabilities over HTTP, initialize an official MCP client, and read the capabilities resource.

**Acceptance Scenarios**:

1. **Given** a default installation, **when** the user follows the guide, **then** the service remains off until the exact settings toggle is enabled and the warning matches the application UI.
2. **Given** a running service, **when** a client reads discovery and supplies the copied bearer credential, **then** the documented HTTP and MCP endpoints authenticate successfully.
3. **Given** a missing or invalid credential, blocked browser origin, or stopped service, **when** the client connects, **then** the guide identifies the observable failure and a bounded recovery step.

### User Story 2 - Interpret state and database results correctly (Priority: P1)

An integrator can determine what every public field, observation state, database operation, limit, and error means without guessing from example values.

**Why this priority**: Incorrect interpretation of stale, unavailable, or truncated data can turn a truthful local contract into unsafe or misleading downstream behavior.

**Independent Test**: Compare the published machine-readable field inventory with `PUBLIC_PATHS`, verify every observation wrapper state and query type is documented, and run the documented HTTP and MCP query examples against deterministic fixtures.

**Acceptance Scenarios**:

1. **Given** any canonical player-state field, **when** its path is looked up, **then** its JSON value type, meaning, source, and availability or freshness rule are present.
2. **Given** unknown, unavailable, dormant, fresh, or stale evidence, **when** a client interprets the observation, **then** the documentation distinguishes knowledge from freshness and forbids treating absent current evidence as current truth.
3. **Given** a database query result, **when** a client reads typed values, truncation facts, limits, or errors, **then** the documentation matches the shared production model for both transports.

### User Story 3 - Preserve the contract across changes (Priority: P1)

A contributor receives an immediate test failure when the production field inventory or either adapter drifts from the documented and shared contract.

**Why this priority**: S109 closes the implementation sequence, so future changes need executable guards rather than relying on manual documentation memory.

**Independent Test**: Run the new documentation-contract and end-to-end tests, then demonstrate that removing one documented field or changing one adapter projection causes a targeted failure.

**Acceptance Scenarios**:

1. **Given** one production service generation, **when** HTTP and MCP expose state and database operations, **then** normalized documents and canonical errors match exactly.
2. **Given** a port collision or service restart, **when** the controller recovers, **then** discovery ownership, service generation, authentication, and shutdown remain truthful and bounded.
3. **Given** a change to `PUBLIC_PATHS`, **when** tests run without a corresponding documentation review, **then** a focused inventory mismatch names the missing or unexpected path.

### Edge Cases

- The service is disabled, starting, stopping, failed, or restarted while a client holds old discovery data.
- Discovery exists from a prior process or generation and must not be trusted as a credential source.
- A client omits the bearer credential, uses a malformed credential, sends an untrusted Host or Origin, or targets the wrong endpoint.
- HTTP and MCP requests observe different snapshot revisions because state changes between requests.
- A database is absent, unusable, locked, replaced, or queried with denied SQL.
- Query results contain null, integer, real, text, blob, empty rows, duplicate column names, or truncation metadata.
- A client disconnects during a request or leaves a transport connection open during shutdown.
- Documentation examples are copied on Windows PowerShell or a POSIX shell without exposing a credential in repository text.
- A future field is added beneath an existing object but omitted from the documented field inventory.

## Requirements

### Functional Requirements

- **FR-001**: A single public guide MUST describe the exact **Local API and MCP Server** toggle, its off-by-default persistence, the factual capability warning, and the stopped, starting, running, stopping, and failed phases.
- **FR-002**: The guide MUST describe endpoint discovery, the discovery record fields, default loopback port, credential copy action, bearer authentication, Host and Origin restrictions, and discovery cleanup ownership without publishing a credential or user path.
- **FR-003**: The guide MUST list all HTTP routes, MCP resources, and MCP tools shipped in external schema `1.0.0` with request and response roles.
- **FR-004**: The guide MUST include minimal copyable HTTP and standard Streamable HTTP MCP client examples that use explicit credential placeholders and local-only endpoints.
- **FR-005**: The guide MUST include representative read-only agent workflows for current-state reasoning and database-assisted analysis while stating that the service does not orchestrate agents or authorize gameplay actions.
- **FR-006**: Documentation MUST define `schema_version`, `snapshot_revision`, `captured_at`, and `service_generation`, including which changes across state publication and service restart.
- **FR-007**: Documentation MUST define the common observation wrapper fields and distinguish `observed`, `unknown`, `unavailable`, and `dormant` knowledge from `fresh`, `stale`, and `not_applicable` freshness.
- **FR-008**: A machine-readable documentation inventory MUST contain every path in `player_state::PUBLIC_PATHS` exactly once with its value type, meaning, source, and availability or freshness rule.
- **FR-009**: Automated coverage MUST fail when the machine-readable documentation inventory and `PUBLIC_PATHS` differ, contain duplicates, or omit required descriptive metadata.
- **FR-010**: Documentation MUST describe the fixed `catalog` and `encounters` database inventory, public schema discovery, availability behavior, and exclusion of filesystem paths and SQLite internals.
- **FR-011**: Documentation MUST describe query SQL, database selection, positional and named parameter forms, supported typed values, row limits, typed result values, truncation facts, and canonical errors.
- **FR-012**: Documentation MUST state the fixed request, SQL, parameter, concurrency, duration, row, and result-envelope bounds from production constants without duplicating a contradictory value.
- **FR-013**: Documentation MUST explain compatibility policy for additive fields, breaking schema changes, stable identifiers, and client behavior when an unknown field or enum appears.
- **FR-014**: Troubleshooting MUST cover disabled state, port collision, stale discovery, authentication, Host or Origin rejection, protocol or schema mismatch, unavailable or busy databases, query limits or timeout, client disconnect, and clean restart.
- **FR-015**: Existing settings, architecture, responsible-use, data or storage, feature-index, reference-index, troubleshooting, and test-strategy pages MUST link to the canonical guide where relevant.
- **FR-016**: A production-adapter end-to-end test MUST start `LocalServiceController` with deterministic snapshot, catalog, and encounter fixtures and use the actual HTTP router and official RMCP client.
- **FR-017**: The end-to-end test MUST compare HTTP and MCP capabilities, player state, database inventory, one successful typed query, and one denied query error after removing only transport framing and `elapsed_ms`.
- **FR-018**: Parity assertions MUST bind comparisons to the same `snapshot_revision` and `service_generation` or retry a bounded number of times when publication races between requests.
- **FR-019**: Lifecycle integration coverage MUST verify disabled state, discovery publication, port collision, recovery, client disconnect, restart generation change, stale endpoint rejection, and clean bounded shutdown.
- **FR-020**: Documentation smoke coverage MUST verify the canonical guide is in `SUMMARY.md`, required operations and limits are present, example placeholders are used, internal links resolve through the existing documentation build, and UI warning text remains synchronized.
- **FR-021**: Existing local-service, player-state, database-query, safety-critical, documentation, trust-policy, and release gates MUST remain passing without weakened assertions.
- **FR-022**: Plan 045, its active-plan index, the canonical local-extension contract, the changelog, and issue tracking MUST identify S109 as the final implementation slice.
- **FR-023**: S109 completion MUST leave issue #180 ready to close through its pull request and MUST leave epic #174 open for merge housekeeping rather than closing it before operator merge.
- **FR-024**: S109 MUST NOT add remote binding, new credentials, writes, telemetry, agent orchestration, new player observations, new database access, or a second transport-specific state model.

### Key Entities

- **Local extension guide**: Canonical public entry point for enablement, discovery, authentication, operations, examples, compatibility, and troubleshooting.
- **Documented field entry**: Machine-readable record connecting one public JSON path to type, meaning, source, and evidence semantics.
- **Parity fixture**: Deterministic snapshot plus catalog and encounter databases shared by the production HTTP and MCP adapters in one service generation.
- **Normalized transport result**: HTTP or MCP document after removal of transport framing and independently measured elapsed time only.
- **Lifecycle receipt**: Test-observed phase, discovery generation, endpoint, and shutdown outcome without credentials or private paths.

## Success Criteria

### Measurable Outcomes

- **SC-001**: A documentation-only user journey enables the service and completes authenticated HTTP capabilities and MCP initialization against the production test service.
- **SC-002**: Every `PUBLIC_PATHS` entry has exactly one complete documented inventory entry, and equality is enforced in automated tests.
- **SC-003**: HTTP and MCP capabilities, player state, database inventory, successful query content, and canonical denied-query error are equal under the stated normalization rule.
- **SC-004**: Port collision, recovery, disconnect, restart, and shutdown tests complete within the existing three-second lifecycle bound and leave no owned discovery record.
- **SC-005**: The full documentation policy, mdBook test, build, link check, example smoke tests, Cargo merge gate, release build, trust policy, encoding, LF, mojibake, and forbidden-dash checks pass.
- **SC-006**: No example, fixture, log assertion, or documentation artifact contains a real credential, absolute user path, or captured private gameplay value.

## Assumptions

- External schema version `1.0.0`, the fixed operations, and all resource limits shipped by S105 through S108 remain authoritative.
- The existing local-service controller and official RMCP test client are sufficient for production-adapter end-to-end coverage without a new runtime dependency.
- Documentation may use deterministic synthetic fixtures and placeholder credentials because live ESO evidence is not required to verify the transport contract.
- Installed-release verification issues #110, #129, #131, and #190 remain independent and do not block Plan 045 completion.

## Out of Scope

- Remote access, TLS termination, browser application support, additional authentication methods, or credential rotation changes.
- New API routes, MCP operations, player observations, database tables, or query capabilities.
- Agent hosting, prompt libraries, automated coaching, gameplay recommendations, or action execution.
- Release publication, installed-game verification, or closing epic #174 before the S109 pull request is merged.
