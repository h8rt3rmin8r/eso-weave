# Feature Specification: MCP Player-State Resources

**Feature Branch**: `codex/s107-mcp-player-state`

**Created**: 2026-09-17

**Status**: Implemented

**Input**: Work slice S107 implements GitHub issue #178 under Plan 045 and the S104 through S106 local-extension contracts.

## Clarifications

### Session 2026-09-17

- Q: Which MCP operations ship in S107? -> A: Exactly two fixed read-only resources, `esoweave://capabilities` and `esoweave://player-state`. Resource templates, subscriptions, list-change notifications, tools, prompts, and database queries remain unavailable.
- Q: How is state shared between HTTP and MCP? -> A: Each MCP read clones one immutable S106 snapshot from the existing publisher. Player-state reads overlay the active service generation exactly as HTTP does. S107 adds no cache, mutable session state, or alternate projection.
- Q: What payload form do resources use? -> A: Each read returns one UTF-8 JSON text content item with media type `application/json` and the requested canonical URI.
- Q: How is protocol behavior verified? -> A: Integration tests connect through RMCP's official client transport, initialize a stateless Streamable HTTP session, discover resources, read both resources, and compare their JSON with the HTTP endpoints.
- Q: What happens for unknown resources? -> A: The server returns the standard MCP resource-not-found error without echoing attacker-controlled values or internal details.
- Q: What does capability discovery report? -> A: Both HTTP and MCP capability documents set `mcp_player_state` to true while database queries and all write or control operations remain unavailable.

## User Scenarios & Testing

### User Story 1 - Discover and read canonical resources (Priority: P1)

A standard MCP client can initialize against the local service, discover the two stable resources, and read their complete JSON documents.

**Why this priority**: MCP interoperability is the delivery objective of S107 and must work through the protocol rather than a custom adapter test.

**Independent Test**: Connect an official RMCP client to an authenticated running service, list all resources, read each advertised URI, and validate its descriptor, media type, URI, and JSON document.

**Acceptance Scenarios**:

1. **Given** a running authenticated service, **when** an MCP client initializes and lists resources, **then** exactly the capability and player-state resources are advertised with stable names and descriptions.
2. **Given** the advertised capability resource, **when** it is read, **then** one JSON content item contains the same capability document exposed by HTTP.
3. **Given** the advertised player-state resource, **when** it is read, **then** one JSON content item contains the latest complete canonical snapshot with the active service generation.

### User Story 2 - Observe one truth across HTTP and MCP (Priority: P1)

A local integrator can use either supported transport without receiving different values, freshness semantics, or revisions for the same published snapshot.

**Why this priority**: Two independently projected state surfaces would create ambiguity and unsafe downstream decisions.

**Independent Test**: Publish deterministic bootstrap, active, stale, unavailable, and recovered snapshots, then compare parsed HTTP and MCP documents field for field while the publisher revision remains unchanged.

**Acceptance Scenarios**:

1. **Given** one published revision, **when** HTTP and MCP player state are read, **then** their parsed JSON documents are equal.
2. **Given** unknown, unavailable, dormant, stale, or recovered observations, **when** either transport is read, **then** knowledge, value, freshness, source, protocol, and timing semantics remain equal.
3. **Given** no semantic publication change, **when** either surface is read repeatedly, **then** reads do not create a new revision.

### User Story 3 - Fail and stop predictably (Priority: P2)

A client receives bounded, deterministic protocol errors, and a slow or disconnected MCP reader cannot block state publication or service shutdown.

**Why this priority**: The embedded MCP surface shares the desktop process and must preserve the lifecycle and responsiveness guarantees established in S105 and S106.

**Independent Test**: Request an unknown resource, disconnect during a read, run concurrent readers and publishers, and stop the service under load while asserting bounded completion and unchanged security behavior.

**Acceptance Scenarios**:

1. **Given** an unknown resource URI, **when** it is read, **then** the server returns resource-not-found without disclosing internal details.
2. **Given** a stalled or disconnected MCP client, **when** a new snapshot is published, **then** publication and HTTP reads continue without waiting for that client.
3. **Given** active MCP requests, **when** service shutdown starts, **then** cancellation and bounded shutdown complete under the existing lifecycle contract.

### Edge Cases

- The service starts before the first application snapshot is published.
- Resource listing includes pagination parameters even though the fixed inventory fits one page.
- A read races with publication of a newer immutable snapshot.
- The service generation changes after a stop and restart.
- A resource URI differs only by case, suffix, query text, or fragment.
- A caller attempts resource templates, subscriptions, tools, prompts, or write operations.
- JSON serialization encounters an internal failure.
- A client disconnects during response transmission.
- Multiple MCP and HTTP readers run while the publisher advances.
- Shutdown begins while an MCP initialize or resource-read request is in flight.

## Requirements

### Functional Requirements

- **FR-001**: S107 MUST expose exactly two fixed MCP resources: `esoweave://capabilities` and `esoweave://player-state`.
- **FR-002**: Resource listing MUST return stable URI, name, title, description, and `application/json` media type metadata for both resources with no pagination token.
- **FR-003**: The MCP server initialization result MUST advertise resource support and MUST NOT advertise resource subscription or list-change support.
- **FR-004**: S107 MUST NOT advertise or implement resource templates, prompts, tools, database queries, write operations, application control, or remote access.
- **FR-005**: Reading a known resource MUST return exactly one UTF-8 JSON text content item whose URI is the requested canonical URI and whose media type is `application/json`.
- **FR-006**: The capability resource MUST serialize the capability document from the latest immutable S106 snapshot.
- **FR-007**: The player-state resource MUST serialize the latest immutable S106 snapshot with the current running service generation overlaid by the lifecycle host.
- **FR-008**: HTTP and MCP reads of the same published revision and service generation MUST be equal after JSON parsing.
- **FR-009**: MCP reads MUST preserve every S106 unknown, unavailable, dormant, fresh, stale, source, protocol, time, age, revision, and generation semantic without remapping.
- **FR-010**: Both HTTP and MCP capability documents MUST truthfully report `mcp_player_state` as true while `query_execution` remains false.
- **FR-011**: The MCP adapter MUST clone one existing immutable snapshot reference per read and MUST NOT introduce a second cache, projection, revision counter, mutable session store, or producer lock during serialization.
- **FR-012**: Unknown, case-mismatched, suffixed, queried, or fragmented resource URIs MUST return the standard resource-not-found error without echoing untrusted URI text or internal details.
- **FR-013**: Internal serialization failure MUST return a deterministic MCP internal error without process panic or disclosure of state, secrets, or paths.
- **FR-014**: Existing bearer authentication, Host and Origin validation, request-body limits, stateless Streamable HTTP configuration, cancellation, and stopping guards MUST protect MCP unchanged.
- **FR-015**: Slow, concurrent, or disconnected MCP clients MUST NOT block snapshot publication, UI progress, HTTP readers, or bounded service shutdown.
- **FR-016**: Integration tests MUST use RMCP's real client and Streamable HTTP transport to initialize, list, and read resources against the running service rather than using a custom protocol mock.
- **FR-017**: Tests MUST cover exact discovery metadata, both reads, HTTP parity, bootstrap state, semantic loss states, unknown resources, unauthorized access, concurrent publication, disconnect, restart generation, and bounded shutdown.
- **FR-018**: Existing safety-critical input, addon, shared-data, player-state, and service lifecycle tests MUST remain unchanged and passing.
- **FR-019**: Plan 045 tracking, the active-plan index, canonical local-extension summary, and the changelog MUST reflect S107 and preserve issue #179 ownership of database resources and queries.
- **FR-020**: S107 MUST NOT add new telemetry, public write surfaces, new state observations, remote binding, credential storage, or public integrator documentation reserved for issue #180.

### Key Entities

- **MCP resource descriptor**: Stable discovery metadata for one fixed read-only JSON document.
- **MCP resource content**: One canonical UTF-8 JSON text item associated with its exact resource URI.
- **Player-state MCP adapter**: Stateless protocol adapter that reads one immutable snapshot reference and applies the existing service-generation envelope.
- **Canonical snapshot publisher**: The sole S106 authority shared by HTTP and MCP readers.

## Success Criteria

### Measurable Outcomes

- **SC-001**: An official RMCP client initializes, discovers exactly two resources, and reads both successfully in automated integration tests.
- **SC-002**: For every deterministic parity fixture, parsed HTTP and MCP documents are field-for-field equal for the same revision and service generation.
- **SC-003**: Unknown-resource tests always receive the MCP resource-not-found code and no tested internal or attacker-controlled detail.
- **SC-004**: Concurrent MCP readers observe internally coherent immutable revisions while publication continues without reader-held publisher locks.
- **SC-005**: Disconnect and active-read shutdown tests complete within the existing bounded lifecycle timeout.
- **SC-006**: All pre-existing tests and repository CI parity, documentation policy, encoding, LF, mojibake, and forbidden-dash checks pass.

## Assumptions

- RMCP 3.4.0 remains the pinned protocol implementation selected in S104.
- A dev-only RMCP client transport feature may be enabled for contract tests without broadening the release artifact's server-only dependency surface.
- The two resources are small enough for one fixed resource-list response and one JSON text content item each.
- The existing S105 stateless Streamable HTTP endpoint remains the only MCP transport.
- The S106 snapshot and capability types remain the canonical serialization authority.

## Out of Scope

- Database discovery and read-only query execution from issue #179.
- Complete public integrator documentation and cross-surface end-to-end verification from issue #180.
- Resource subscriptions, resource templates, tools, prompts, sampling, elicitation, notifications, and mutable MCP sessions.
- New gameplay observations, action automation, remote access, and credential-management changes.
