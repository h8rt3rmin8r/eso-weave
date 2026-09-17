# Local Service Lifecycle Checklist

**Purpose**: Review the security, lifecycle, persistence, and UI contract before implementation

**Created**: 2026-09-16

## Opt-in and Persistence

- [x] Fresh-install enablement defaults false
- [x] Requested enablement is a user preference, not session or derived state
- [x] Credential creation precedes listener availability
- [x] Credential remains absent from discovery and presentation

## Network Security

- [x] Bind is IPv4 loopback only
- [x] Production port is fixed and collision is a visible failure
- [x] Every route is authenticated
- [x] Host and present Origin values use the effective loopback authority
- [x] Body size is bounded before handler work
- [x] No permissive CORS behavior is introduced

## Atomic Lifecycle

- [x] One thread owns one current-thread runtime and listener
- [x] HTTP and MCP share one router and failure domain
- [x] Discovery publication is the final start step
- [x] Partial start cleanup releases the listener and owned discovery
- [x] Disable, rapid toggle, failure recovery, and exit share explicit transitions
- [x] Shutdown and join are bounded to 3 seconds

## Slice Boundaries

- [x] Minimal MCP initialization is included so both transports exist
- [x] No player-state, database, query, or action service enters S105
- [x] No MCP resources or tools preempt issue #178
- [x] User and integrator documentation remains owned by issue #180

## UI and Diagnostics

- [x] The control is one focusable toggle
- [x] Warning copy is factual and calm
- [x] Status and endpoints remain safe to display
- [x] Credential retrieval requires an explicit copy action and never renders the secret
- [x] Diagnostics are transition-based and secret-free
