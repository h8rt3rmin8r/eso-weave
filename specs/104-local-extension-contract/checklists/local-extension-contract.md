# Local Extension Contract Review Checklist

**Purpose**: Prevent foundational transport, security, state, and query gaps

**Created**: 2026-09-15

## Stack and Runtime

- [x] Selected components have exact versions and compatible licenses
- [x] Maintenance, integration, runtime, and binary footprint evidence is recorded
- [x] One listener, router, runtime owner, lifecycle, and cancellation path are defined
- [x] Fixed production port and ephemeral test behavior are distinguished

## Security

- [x] Loopback-only binding, bearer authentication, Host validation, and Origin validation are mandatory
- [x] Credentials are excluded from URLs, discovery, logs, and errors
- [x] No permissive CORS, remote binding, or write/control capability is implied
- [x] SQLite access uses defense in depth and exact resource limits

## State and Data

- [x] Current public state domains are inventoried
- [x] Unknown, unavailable, dormant, stale, and observed states remain distinct
- [x] Application-owned runtime databases and table families are inventoried
- [x] HTTP and MCP parity is explicit for state and queries

## Delivery Boundaries

- [x] Compatibility and versioning rules are explicit
- [x] Follow-up issue ownership is preserved
- [x] S104 records an implementable decision but ships no runtime surface
