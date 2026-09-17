# Domain Checklist: Defended Database Queries

**Purpose**: Verify the security, resource, parity, and lifecycle boundaries unique to S108

**Created**: 2026-09-17

## Inventory

- [x] The fixed inventory contains only catalog and encounters
- [x] Missing databases remain discoverable without paths
- [x] Public schema excludes internal objects and SQL definitions
- [x] Catalog replacement is represented as a dynamic current path

## Read-Only Enforcement

- [x] Read-only and no-follow open flags are required
- [x] Query-only, defensive, and untrusted-schema settings are required
- [x] Authorizer denial is fail-closed, including unknown actions
- [x] Prepared-statement read-only and one-statement checks are required
- [x] Extension loading is not enabled

## Bounds

- [x] Two-query global admission is immediate and shared by both adapters
- [x] The two-second deadline and generation cancellation reach SQLite
- [x] Every S104 numeric bound has an explicit completion behavior
- [x] Byte truncation admits complete rows only
- [x] Materialization releases the connection before transport serialization

## Parity and Errors

- [x] HTTP and MCP use one request, result, and error model
- [x] Exact typed-value encodings are fixed
- [x] Transport status differences do not change canonical errors
- [x] Unsafe failures do not expose paths, SQL fragments, secrets, or engine diagnostics

## Notes

Checklist passed before planning. Implementation tests must preserve every checked boundary.
