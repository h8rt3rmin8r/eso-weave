# Feature Specification: Local Service Lifecycle

**Feature Branch**: `codex/s105-local-service-lifecycle`

**Created**: 2026-09-16

**Status**: Implemented

**Input**: Work slice S105 implements GitHub issue #176 under Plan 045 and the S104 contract.

## Clarifications

### Session 2026-09-16

- Q: How can S105 start both transports while issue #178 owns MCP projection? -> A: S105 mounts an authenticated, stateless RMCP server that supports initialization but advertises no resources, tools, or prompts. Issue #178 adds canonical resources and parity without changing lifecycle ownership.
- Q: Where does requested enablement live? -> A: It is a durable user preference in `config.json`. Starting, running, stopping, failed, generation, process, and endpoint values remain runtime or discovery state and are never persisted as settings.
- Q: When is the credential created and rotated? -> A: Create one persistent 256-bit credential on the first requested enablement, retain it across stop and restart, and do not rotate it in S105 because no rotation control is in scope.
- Q: What may be public before issue #177 adds canonical operations? -> A: Only authenticated MCP initialization and transport-correct empty capability discovery. HTTP canonical operations remain unavailable until their owning slices.
- Q: How should an unavailable settings directory behave? -> A: Enabling fails truthfully without starting a listener because neither credential durability nor owned discovery cleanup can be guaranteed.
- Q: How does a trusted client obtain the secret? -> A: The settings cluster provides an explicit Copy Credential action. The credential is copied to the clipboard but never rendered inline, placed in discovery, or logged.

## User Scenarios & Testing

### User Story 1 - Enable one coherent local service (Priority: P1)

An operator can enable one Local API & MCP server setting and see both transports become available together on the accepted loopback endpoint.

**Why this priority**: The local extension surface must never create separate or partially active transport states.

**Independent Test**: Start from default settings, enable the toggle with an ephemeral test port, authenticate an MCP initialization request, and verify one running status and discovery record.

**Acceptance Scenarios**:

1. **Given** a fresh installation, **when** settings load, **then** the local service is disabled and no listener or discovery record exists.
2. **Given** a writable settings directory and free loopback port, **when** the operator enables the setting, **then** one background runtime starts the HTTP router and stateless MCP endpoint atomically and reports the effective connection information.
3. **Given** the service is enabled, **when** the application restarts, **then** requested enablement and the existing credential are reused without a second confirmation.

### User Story 2 - Understand capability and failure state (Priority: P1)

An operator can read calm capability warning copy and truthful stopped, starting, running, stopping, or failed state in the settings interface.

**Why this priority**: Broad local read capability needs informed activation, and silent half-starts are unsafe and difficult to recover from.

**Independent Test**: Drive each lifecycle status, including a port collision, and inspect the keyboard-accessible settings control, status text, endpoint text, and safe diagnostic output.

**Acceptance Scenarios**:

1. **Given** the settings interface, **when** the control is focused by keyboard, **then** its label, warning, current state, and action are understandable without a pointer.
2. **Given** port 18765 is occupied, **when** enablement is requested, **then** the status becomes failed with an actionable address-in-use message and no discovery record or half-running adapter remains.
3. **Given** an unauthenticated request, untrusted Host, or untrusted browser Origin, **when** either transport is addressed, **then** the request is rejected without exposing credential or internal-path data.

### User Story 3 - Stop and recover without orphan work (Priority: P1)

An operator can disable, rapidly toggle, re-enable, or exit while the service releases all owned resources within the accepted bound.

**Why this priority**: A local listener that outlives user intent or prevents reliable recovery violates the opt-in boundary.

**Independent Test**: Exercise disable during startup, repeated toggle sequences, re-enable after stop and collision recovery, and owner drop while observing status, port reuse, discovery ownership, and thread completion.

**Acceptance Scenarios**:

1. **Given** a running service, **when** the toggle is disabled, **then** new requests stop, owned discovery is removed, the listener is released, and status reaches stopped.
2. **Given** start and stop commands arrive rapidly, **when** they settle, **then** the last requested state wins without an orphan listener or stale discovery record.
3. **Given** the application exits with clients connected, **when** the lifecycle owner drops, **then** the same cancellation path runs and the owner thread joins within 3 seconds.

### Edge Cases

- The preferred port is occupied by another process or another ESO Weave instance.
- The settings directory is absent, unwritable, replaced, or unavailable.
- Discovery publication fails after listener bind but before running status.
- A stale discovery record belongs to another process or service generation.
- Disable arrives while bind, router construction, or discovery publication is in progress.
- Enable arrives while shutdown is still releasing the listener.
- The command channel disconnects or the runtime owner panics.
- Requests contain duplicate Authorization headers, malformed bearer syntax, mismatched Host ports, `Origin: null`, non-loopback origins, or oversized bodies.
- The persisted credential is missing or malformed while enablement is requested.

## Requirements

### Functional Requirements

- **FR-001**: Configuration MUST contain one `local_service.enabled` user preference that defaults to false and round-trips across application restarts.
- **FR-002**: First enablement MUST create and persist one credential containing at least 256 bits of operating-system cryptographic randomness before a listener becomes usable.
- **FR-003**: The credential MUST remain stable across ordinary disable, enable, and restart cycles and MUST never appear in discovery, logs, status text, error text, or URLs.
- **FR-004**: Production MUST bind exactly IPv4 `127.0.0.1:18765`; test construction MAY request port zero and report the kernel-selected loopback port.
- **FR-005**: One named background OS thread MUST own a Tokio current-thread runtime, one listener, one Axum router, and one cancellation tree.
- **FR-006**: The shared router MUST mount HTTP under `/api/v1` and a stateless RMCP Streamable HTTP service at `/mcp` on the same origin. S105 MUST advertise no application resources, tools, or prompts.
- **FR-007**: Every routed request, including MCP initialization and fallback responses, MUST require exactly one bearer credential and constant-work comparison.
- **FR-008**: Requests MUST reject an authority other than the effective `127.0.0.1:<port>` authority and reject every present browser Origin other than HTTP or HTTPS loopback with the effective port. No permissive CORS headers may be emitted.
- **FR-009**: The service MUST enforce the S104 64 KiB request-body ceiling at the transport boundary.
- **FR-010**: Lifecycle status MUST distinguish stopped, starting, running, stopping, and failed, and MUST expose only safe failure code and message details.
- **FR-011**: Running status MUST expose effective HTTP base URL, MCP URL, process identity, service generation, and external schema version without exposing the credential.
- **FR-012**: A successful start MUST atomically publish `local-extension.json` only after the listener, authentication, router, and both transport surfaces are ready.
- **FR-013**: Discovery cleanup MUST remove the record only after its process identity and generation match the current owner.
- **FR-014**: Any bind, router, runtime, or discovery failure MUST cancel the whole start attempt, release the listener, clean only owned discovery, and report failed rather than leave one adapter available.
- **FR-015**: Disable and application exit MUST share one shutdown path, stop new requests, cancel transport work, remove owned discovery, release the listener, and join the runtime owner within 3 seconds.
- **FR-016**: Rapid commands MUST converge on the latest requested enabled state, and re-enable after a completed stop or recoverable failure MUST work without restarting ESO Weave.
- **FR-017**: The settings interface MUST provide one keyboard-accessible toggle, calm warning copy that connected clients can read live player state and query application data, truthful status, running endpoint information, and an explicit keyboard-accessible Copy Credential action that never renders the secret inline.
- **FR-018**: Lifecycle diagnostics MUST log state changes and safe failure codes only when they change, without query contents, credentials, authorization headers, request bodies, or high-frequency request lines.
- **FR-019**: Tests MUST cover default and round-trip persistence, credential creation and secrecy, MCP initialization, authentication, Host and Origin checks, discovery ownership, port collision, partial-start cleanup, rapid toggle, re-enable, and application exit.
- **FR-020**: S105 MUST NOT expose canonical player state, database inventory, query execution, application control, remote binding, TLS termination, legacy SSE, stdio, helper processes, or session state.
- **FR-021**: The exact S104-selected dependency versions and required features MUST be used: Axum `=0.8.9`, RMCP `=3.4.0`, Tokio `=1.53.1`, and Tokio Util `=0.7.16`.
- **FR-022**: Plan 045 tracking, the changelog, and the S105 spec packet MUST reflect the implemented slice and its architecture-affecting dependency addition.

### Key Entities

- **Local service preference**: Durable requested enablement plus a secret credential, with disabled and absent credential as the fresh-install default.
- **Lifecycle controller**: UI-facing non-blocking command and status handle for the dedicated runtime owner.
- **Lifecycle status**: Current phase, generation, safe failure, and optional running connection information.
- **Runtime owner**: Dedicated thread containing the current-thread async runtime, listener, shared router, MCP service, and cancellation tree.
- **Discovery record**: Atomic non-secret JSON record for the currently running process and generation.

## Success Criteria

### Measurable Outcomes

- **SC-001**: A fresh configuration has requested enablement false, produces no listener, and creates no discovery record.
- **SC-002**: A successful ephemeral-port test completes authenticated MCP initialization and publishes matching HTTP and MCP URLs from one bound port.
- **SC-003**: Every tested authentication, Host, Origin, body-size, bind, and discovery failure leaves no usable listener or falsely running state.
- **SC-004**: Disable and owner drop release the test port and owned discovery within 3 seconds in every lifecycle test.
- **SC-005**: Rapid start and stop sequences settle to the final request in 100% of deterministic integration cases.
- **SC-006**: Settings UI tests find one focusable toggle, capability warning, status text, and running connection detail without finding credential text.
- **SC-007**: Repository CI parity, dependency review, formatting, spelling, links, UTF-8, LF, and mojibake checks pass.

## Assumptions

- Issue #177 will add canonical HTTP capabilities and player-state routes; issue #178 will add MCP resources and parity over this host.
- The existing per-user configuration directory supplies the platform account boundary. Unix credential files receive user-only permissions; Windows inherits the per-user application-data ACL.
- S105 has no long-running SQLite work, so the shutdown interrupt contract becomes active when issue #179 adds query execution.
- The settings warning describes the eventual approved capability because enabling the service opts into that surface as dependent slices ship.

## Out of Scope

- Canonical player-state snapshots and HTTP projection from issue #177.
- MCP resources, tools, and adapter parity from issue #178.
- SQLite database inventory and query execution from issue #179.
- Public user and integrator documentation from issue #180.
- Inline credential display, rotation, revocation UI, remote access, OAuth, TLS, browser application support, and all write or action APIs.
