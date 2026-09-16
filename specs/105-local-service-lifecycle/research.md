# Research: Local Service Lifecycle

## R1: RMCP transport boundary

**Decision**: Use RMCP 3.4.0 `StreamableHttpService` with `NeverSessionManager`, `legacy_session_mode = false`, JSON responses, the shared cancellation token, exact Host and Origin allowlists, and the 64 KiB body limit.

**Rationale**: This is the official selected SDK and its stateless manager matches ADR 0002. An empty `ServerHandler` supports initialization without claiming resources, tools, or prompts.

**Rejected alternatives**:

- Hand-written JSON-RPC would duplicate protocol negotiation and violate the selected-stack decision.
- Sessionful MCP would create application state explicitly rejected by S104.
- Delaying the MCP mount would leave issue #176 unable to prove atomic two-transport startup.

## R2: Runtime ownership

**Decision**: Spawn one named owner thread at controller construction. It creates a Tokio current-thread runtime, receives lifecycle commands, and exclusively owns listener creation, serving, discovery publication, cancellation, and resource release.

**Rationale**: The existing desktop is synchronous and frame-driven. A dedicated owner keeps async work and joins away from input hooks while giving lifecycle state one writer.

**Rejected alternatives**:

- Creating a global multi-thread runtime broadens concurrency without a requirement.
- Spawning a new unmanaged thread per toggle complicates generation ownership and rapid-toggle cleanup.
- Running startup or shutdown on the UI thread can block frames.

## R3: Persisted model

**Decision**: Add an opaque `local_service` section to `Settings`, parsed into `LocalServicePrefs { enabled, credential }`. Only the boolean is editable through `SettingsForm`; credential mutation is owned by the lifecycle integration.

**Rationale**: Requested enablement and credential are durable user settings. Lifecycle phase and endpoints are derived runtime facts and therefore stay out of configuration.

**Rejected alternatives**:

- `state.json` is for session/runtime intent and would conflict with issue #176's persisted setting requirement.
- A second secret file adds lifecycle and migration surface without providing a stronger platform boundary than the per-user config directory.

## R4: Secret generation and comparison

**Decision**: Fill 32 bytes with the operating-system random source and encode them as 64 lowercase hexadecimal characters. Compare presented bearer bytes against expected bytes using one length-normalized XOR pass.

**Rationale**: Hex is dependency-light, URL-safe when copied by clients, and represents 256 random bits exactly. Constant-work content comparison avoids ordinary early-exit equality.

**Rejected alternatives**:

- UUID-sized values do not meet the 256-bit entropy requirement.
- Logging a redacted prefix still creates an unnecessary correlation identifier.
- Generating a new value at every start would break stable client configuration.

## R5: Discovery publication

**Decision**: Serialize the accepted S104 record, sync it through a temporary file in the same directory, and atomically replace `local-extension.json`. Remove only after parsing and matching current process plus generation.

**Rationale**: Clients see either a complete old or complete new record, while a stale or foreign owner cannot be deleted by this process.

**Rejected alternatives**:

- Truncating the destination in place exposes partial JSON.
- Unconditional delete can erase another running generation's record.

## R6: Router security order

**Decision**: Apply one global middleware that validates effective Host, present Origin, exactly one bearer credential, and request size before dispatch. Configure RMCP's own Host, Origin, body, and cancellation protections as defense in depth.

**Rationale**: One outer boundary protects HTTP fallbacks and MCP initialization consistently. RMCP validation retains transport-native checks.

**Rejected alternatives**:

- Per-route authentication risks accidentally public discovery or future routes.
- CORS response policy is unnecessary for a non-browser local API and cannot substitute for Origin validation.

## R7: Failure injection and verification

**Decision**: Expose a test configuration seam for port zero and discovery path. Exercise real loopback I/O for authentication and MCP initialization, and direct controller state for transition and cleanup tests.

**Rationale**: Real sockets catch router and release failures, while deterministic paths avoid relying on the production port in parallel tests.

**Rejected alternatives**:

- Mock-only tests cannot prove bind collisions, port release, or HTTP middleware order.
- Production-port tests are flaky and may interfere with a running installation.
