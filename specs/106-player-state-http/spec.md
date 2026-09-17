# Feature Specification: Canonical Player-State HTTP API

**Feature Branch**: `codex/s106-player-state-http`

**Created**: 2026-09-17

**Status**: Implemented

**Input**: Work slice S106 implements GitHub issue #177 under Plan 045 and the S104/S105 local-extension contracts.

## Clarifications

### Session 2026-09-17

- Q: May S106 serialize the existing `AppView`? -> A: No. `AppView` contains presentation strings, colors, and display-only stale retention. S106 creates a dedicated canonical snapshot from authoritative observations and configuration.
- Q: How are HTTP readers kept coherent without blocking the UI or worker threads? -> A: Producers publish one immutable snapshot reference through a short, non-blocking shared handoff. Each request clones one reference and serializes it without holding producer locks.
- Q: When does `snapshot_revision` advance? -> A: Only when a public value or its knowledge, freshness, source, or protocol metadata changes. Capture time alone does not create a new revision.
- Q: How is service generation added without duplicating application state? -> A: The lifecycle host owns generation. It overlays the running generation onto the immutable application snapshot envelope at response time without changing snapshot content or revision.
- Q: What HTTP operations ship in S106? -> A: Authenticated `GET /api/v1/capabilities` and `GET /api/v1/player-state`. Other API paths retain structured not-found or unavailable behavior. MCP resources and queries remain later slices.
- Q: How are newly added internal observations kept from silently disappearing? -> A: A maintained inventory test maps every declared source observation to either the public S104 inventory or its deliberate non-public table.

## User Scenarios & Testing

### User Story 1 - Read one complete truthful snapshot (Priority: P1)

A local client can authenticate and retrieve one versioned representation of every current player-state fact ESO Weave has approved for external use.

**Why this priority**: The extension surface is useful only if one request returns a coherent and semantically complete view rather than a partial UI projection.

**Independent Test**: Publish a deterministic active fixture, request `/api/v1/player-state`, and compare every public contract path, value, metadata field, revision, and service generation with the canonical in-process snapshot.

**Acceptance Scenarios**:

1. **Given** an enabled service and active deterministic observations, **when** an authenticated client requests player state, **then** every stable domain and public field is present with schema, revision, capture, generation, knowledge, source, protocol, time, age, and freshness semantics.
2. **Given** a field is unknown, unavailable, dormant, or stale, **when** the snapshot is serialized, **then** that state is explicit and is never converted to zero, false, an empty string, or ambiguous omission.
3. **Given** concurrent source changes, **when** multiple clients read, **then** each response contains one immutable revision and never combines envelope or domain values from different published revisions.

### User Story 2 - Discover supported capabilities (Priority: P1)

A client can discover the external schema, supported state domains, source protocol support, and currently implemented operations before consuming the snapshot.

**Why this priority**: Clients need stable compatibility facts rather than inferring support from failed requests or optional fields.

**Independent Test**: Request `/api/v1/capabilities` from deterministic protocol configurations and verify stable schema and domain inventory plus truthful operation and source-protocol declarations.

**Acceptance Scenarios**:

1. **Given** a running S106 service, **when** an authenticated client requests capabilities, **then** it receives schema `1.0.0`, all six state domains, HTTP state operations, database identifiers, current supported source protocol facts, and no premature MCP resource or query claim.
2. **Given** a source protocol or layout is unavailable or incompatible, **when** capabilities and state are requested, **then** the capability reason and affected observations agree.
3. **Given** an unsupported method or unknown API route, **when** it is requested, **then** the service returns a deterministic structured error without leaking internal details.

### User Story 3 - Observe loss and recovery immediately (Priority: P1)

A client sees focus loss, signal loss, dormancy, recovery, and automation effective-state changes without inheriting the HUD's display retention.

**Why this priority**: A downstream agent must not treat retained display values as current authority or continue reasoning from gameplay state after authority is lost.

**Independent Test**: Drive active, focus-loss, signal-loss, inactive, and recovery fixtures while retaining a prior HUD snapshot, then verify canonical responses change knowledge and freshness immediately and advance revision only for public semantic changes.

**Acceptance Scenarios**:

1. **Given** a fresh active snapshot, **when** focus or PixelBus signal is lost, **then** game and signal lifecycle facts update immediately and dependent values become stale, unknown, unavailable, or dormant according to contract.
2. **Given** the HUD retains a prior display snapshot, **when** the API is read, **then** no rendered label, color, or display-only retained value appears as canonical current state.
3. **Given** authoritative evidence recovers, **when** a new canonical snapshot is published, **then** the revision advances and observations regain truthful current knowledge and freshness.

### Edge Cases

- The service starts before the first application snapshot has been published.
- Capture time advances while every public value and metadata field remains equal.
- A source changes while the canonical projection is being assembled.
- Focus loss and a PixelBus update arrive close together.
- Source observation timestamps are absent, in the future, or older than the freshness threshold.
- The active layout is legacy, corrupt, unsupported, or not yet decoded.
- A fixed-member array has unavailable bindings or cooldowns.
- Serialization approaches the response-size contract without request-side blocking.
- The client disconnects during serialization or shutdown begins during a read.
- The application version, catalog, or managed-addon state is unavailable.

## Requirements

### Functional Requirements

- **FR-001**: S106 MUST define one immutable canonical player-state snapshot matching external schema `1.0.0` and containing `schema_version`, `snapshot_revision`, `captured_at`, `service_generation`, `capabilities`, and stable `application`, `game`, `pixel_bus`, `player`, `automation`, and `interpretation` objects.
- **FR-002**: Every public path in the S104 canonical player-state contract MUST be represented, and every source observation MUST be classified by a maintained test as public or deliberately non-public.
- **FR-003**: The canonical model MUST NOT serialize `AppView`, rendered labels, colors, UI retention state, logs, paths, secrets, raw pixels, input internals, controller timers, or other declared non-public state.
- **FR-004**: Every observed leaf MUST carry or inherit `knowledge`, typed `value`, `observed_at`, `age_ms`, `freshness`, stable `source`, and optional protocol metadata with the S104 meanings.
- **FR-005**: Unknown, unavailable, dormant, fresh, and stale MUST remain distinct. Values MUST NOT use zero, false, empty text, or field omission as an undocumented substitute.
- **FR-006**: A dedicated snapshot publisher MUST assemble from authoritative application, game, PixelBus, weave, fishing, auto-potion, configuration, catalog, addon, and binding sources at a single publication boundary.
- **FR-007**: Publication MUST expose one immutable reference. A request MUST acquire one reference without holding subsystem, UI, or publisher locks during JSON serialization.
- **FR-008**: `snapshot_revision` MUST be monotonic, start at a positive value, and advance exactly when public content or its semantic metadata changes. `captured_at` and service generation are excluded from content-change comparison.
- **FR-009**: Current focus and signal loss MUST be published immediately and MUST NOT inherit display-only HUD retention. Derived facts MUST inherit the least-fresh input authority.
- **FR-010**: `captured_at` MUST be a UTC RFC 3339 publication timestamp. Leaf ages MUST be nonnegative and derived against that capture boundary when observation time is available.
- **FR-011**: The lifecycle host MUST receive the shared snapshot publisher when constructed and MUST serve the latest immutable snapshot to every running generation.
- **FR-012**: `GET /api/v1/player-state` MUST return the canonical snapshot with the current running service generation and content type `application/json`.
- **FR-013**: `GET /api/v1/capabilities` MUST return schema version, domain inventory, supported HTTP operations, database identifiers, source protocol support, and truthful reasons for unavailable protocol capabilities.
- **FR-014**: The S106 capability response MUST NOT advertise MCP player-state resources, database query execution, write operations, remote access, or application control.
- **FR-015**: Non-GET methods on S106 read routes MUST return a structured `method_not_allowed` response. Unknown API routes MUST return structured `not_found`; `/api/v1` MUST return capability-oriented discovery or an explicit stable response.
- **FR-016**: Existing bearer, Host, Origin, request-body, cancellation, and service-stopping guards MUST protect the new routes unchanged.
- **FR-017**: Slow clients and JSON serialization MUST NOT hold the pixel reader, input hook, automation controller, GUI-frame, or canonical publisher locks.
- **FR-018**: Service start before first publication MUST serve a complete bootstrap snapshot whose fields are explicitly unknown, unavailable, or dormant rather than omit domains.
- **FR-019**: Tests MUST cover complete active projection, bootstrap state, inactive and dormant state, focus loss, signal loss, stale evidence, unavailable protocols, recovery, revision stability, revision advance, concurrent readers, cancellation, shutdown, and transport security.
- **FR-020**: Existing safety-critical input, addon, shared-data, and lifecycle tests MUST remain unchanged and passing.
- **FR-021**: Plan 045 tracking, the active-plan index, canonical local-extension summary, and the changelog MUST reflect S106 and preserve issue #178 ownership of MCP resources and parity.
- **FR-022**: S106 MUST NOT add MCP resources, database discovery or query execution, public integrator documentation, new telemetry, action APIs, remote binding, or configuration persistence for runtime snapshot data.

### Key Entities

- **Canonical snapshot**: One immutable externally approved application-state revision with stable domains and semantic observation metadata.
- **Snapshot publisher**: Shared producer/consumer handoff that detects semantic content changes, assigns monotonic revisions, and exposes immutable references without serialization locks.
- **Observation**: A typed value plus knowledge, freshness, time, source, and optional protocol facts.
- **Capabilities document**: Stable compatibility and operation inventory for schema 1.0.0 and the current source protocols.
- **Source inventory**: Maintained mapping from internal observation authorities to public contract paths or explicit non-public classifications.

## Success Criteria

### Measurable Outcomes

- **SC-001**: The maintained inventory test accounts for 100% of public S104 paths and every declared current source observation.
- **SC-002**: Deterministic active, dormant, stale, unavailable, and recovery fixtures compare field-for-field with their HTTP JSON responses.
- **SC-003**: In 100% of concurrency tests, one response contains exactly one snapshot revision and one internally consistent content hash.
- **SC-004**: Repeated publication with no semantic public change keeps the same revision; every tested public value or metadata change advances it exactly once.
- **SC-005**: Focus or signal loss becomes visible in the next published snapshot even while the HUD retains a prior display value.
- **SC-006**: A stalled or disconnected HTTP reader cannot prevent snapshot publication, UI model progress, or bounded service shutdown.
- **SC-007**: Repository CI parity, documentation policy, encoding, LF, mojibake, and forbidden-dash checks pass.

## Assumptions

- The S104 contract is the authoritative external inventory and schema for S106.
- The existing application update loop provides a regular safe publication opportunity; source-specific timestamps remain authoritative when available.
- S106 may add a focused `player_state` module because the canonical model and publisher are independent of the HTTP adapter and will be reused by issue #178.
- RFC 3339 capture timestamps may use a small direct time dependency only if the standard library and existing dependencies cannot produce the accepted format cleanly.
- Database identifiers may be advertised as contract inventory while query operations remain unavailable until issue #179.

## Out of Scope

- MCP resources, tools, prompts, and HTTP/MCP parity from issue #178.
- SQLite database inventory details and query execution from issue #179.
- Complete user and integrator documentation and cross-surface end-to-end testing from issue #180.
- New in-game telemetry, raw diagnostic or log export, action or automation control, remote access, TLS termination, and credential-management changes.
