# Analysis: Local Extension Stack and Contract

## Gate result

PASS. The specification, plan, research, data model, lifecycle contract, player-state inventory, query contract, quickstart, tasks, and checklists are mutually consistent and contain no unresolved clarification.

## Coverage

- Issue #175 maps directly to FR-001 through FR-028 and SC-001 through SC-008.
- Issues #176 through #180 each receive one listener, runtime, security, state, query, parity, and compatibility authority.
- The player-state inventory covers application lifecycle; game installation, runtime, focus, context, surface, and world; PixelBus layout, signal, and addon health; weapons, combat, movement, life, roll dodge, travel, resources, Ultimate, cooldowns, quickslot, native bindings; fishing, auto-potion, weave state; and interpretation configuration.
- The database inventory covers every current catalog and encounter table family and excludes temporary and non-runtime files deliberately.

## Constitution and risk review

- S104 makes an architecture-affecting choice but ships no listener, setting, endpoint, query executor, or production dependency.
- RMCP 3.4.0 same-day freshness is explicit and contained through exact pinning, minimal features, adapter parity, and future integration tests.
- The service boundary is local, opt-in, authenticated, Host and Origin validated, and fail-closed on partial startup or shutdown failure.
- SQLite protections are layered because no single read-only indicator covers the threat model.
- User-facing extension documentation is deferred until behavior ships, avoiding claims that an unavailable surface exists.
- All text artifacts require UTF-8 without BOM, LF, standard hyphens, and mojibake checks.

## Findings resolved before implementation

1. A separate HTTP and MCP host would permit lifecycle and semantic drift. Both adapters share one listener, router, runtime, and canonical services.
2. Ephemeral production ports would make ordinary MCP configuration unstable. Production uses fixed port 18765 and fails visibly; tests may use port 0.
3. Loopback alone does not stop DNS rebinding or unrelated local processes. Bearer authentication, Host validation, Origin validation, and no permissive CORS are mandatory.
4. Session state would create cleanup and consistency obligations without value. MCP uses stateless Streamable HTTP.
5. `sqlite3_stmt_readonly` alone accepts some unsafe control statements. Open flags, connection configuration, authorizer rules, single-statement preparation, a final read-only check, progress cancellation, and limits act together.
6. Publishing rendered UI strings would couple clients to presentation. The external schema uses typed canonical observations and explicit non-public classifications.

No critical, high, or medium inconsistency remains. Maintainer record implementation may proceed.
