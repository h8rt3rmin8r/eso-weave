# Research: Local Extension Documentation and End-to-End Verification

**Date**: 2026-09-17

## Decision 1: Use one canonical public reference guide

**Decision**: Publish `docs/src/reference/local-api-and-mcp.md` as the complete user and integrator entry point. Existing pages link to it for details.

**Rationale**: Enablement, authentication, state semantics, database queries, compatibility, examples, and troubleshooting form one client journey. Splitting them across existing feature pages would create drift and force readers to reconstruct the contract.

**Alternatives rejected**:

- Add fragments only to existing pages. Rejected because no single page would satisfy the source-code-free connection journey.
- Publish maintainer-only documentation under `docs/project`. Rejected because issue #180 explicitly requires shipped user and integrator guidance.

## Decision 2: Keep field completeness machine-readable

**Decision**: Add `local-extension-state-fields.json` with one record per `PUBLIC_PATHS` entry and require non-empty `path`, `type`, `meaning`, `source`, and `availability` values. A Rust integration test compares the sorted unique paths exactly.

**Rationale**: A generated prose table would be harder to review and could still drift. A checked JSON artifact is downloadable, reviewable, and executable while the reference guide explains the shared wrapper and domain-level interpretation.

**Alternatives rejected**:

- Parse a large Markdown table. Rejected because Markdown parsing adds fragile formatting coupling.
- Generate documentation during the build. Rejected because generated output would obscure intentional semantic review and complicate the offline documentation snapshot.
- Treat `PUBLIC_PATHS` alone as documentation. Rejected because paths do not carry type, meaning, source, or evidence rules.

## Decision 3: Reuse production adapter seams

**Decision**: Extend the existing `LocalServiceController` tests and official RMCP client coverage. Use temporary SQLite files and deterministic `SnapshotPublisher` content in the same running generation.

**Rationale**: S106 through S108 already established production router and official client seams. A second harness or mock server would provide weaker evidence and duplicate authentication, framing, and cancellation logic.

**Alternatives rejected**:

- Add a test-only server binary. Rejected because it would become a second lifecycle path.
- Use only unit calls into handlers. Rejected because it would not verify HTTP framing, bearer authentication, RMCP initialization, or disconnect behavior.
- Duplicate all existing lifecycle tests in a new file. Rejected because the existing focused tests already cover many cases and should be strengthened rather than copied.

## Decision 4: Test documented examples as contract fragments

**Decision**: Give the guide one Bash HTTP example, one PowerShell HTTP example, and one JSON standard MCP client configuration example. Policy and Rust tests assert endpoint names, bearer placeholders, operation names, and absence of production-looking credentials.

**Rationale**: The examples remain copyable across supported platforms while their stable contract fragments can be checked without launching a user-specific third-party client.

**Alternatives rejected**:

- Pin a new example-client dependency. Rejected because official RMCP already verifies protocol behavior and a new client would expand maintenance without additional contract coverage.
- Use a raw JSON-RPC curl transcript as the MCP example. Rejected because the user requested standard MCP client guidance rather than manual protocol framing.

## Decision 5: Extend the exact documentation policy deliberately

**Decision**: Register the new page and its semantic tables and fenced examples in `.github/scripts/docs-policy.mjs`, add focused policy tests, and record the pinned-script change in `CHANGELOG.md`.

**Rationale**: The repository intentionally fixes table and code-block inventories to make documentation growth reviewable. Evading those checks would weaken the policy; updating them with exact expected counts preserves it.

**Alternatives rejected**:

- Avoid tables and fences solely to keep old counts. Rejected because that degrades the connection and schema reference.
- Relax exact inventory checks globally. Rejected because S109 adds one known page and does not justify weaker corpus controls.

## Decision 6: Treat lifecycle and schema versions as different axes

**Decision**: Document `schema_version` as contract compatibility, `snapshot_revision` as content publication, and `service_generation` as listener lifetime. Tests bind parity comparisons to the same generation and revision.

**Rationale**: Clients otherwise risk comparing different observations or continuing to use stale discovery after restart.

**Alternatives rejected**:

- Describe all counters as versions. Rejected because their invalidation rules differ.
- Hide service generation as an implementation detail. Rejected because it is already public discovery and snapshot data needed for ownership and restart detection.

## Dependency and Security Review

No new runtime or development dependency is required. Tests retain loopback-only temporary fixtures, placeholder credentials, bounded service shutdown, and the existing official RMCP client. Documentation does not authorize writes, remote binding, agent execution, or gameplay action.
