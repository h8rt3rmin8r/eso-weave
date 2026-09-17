# Tasks: Read-Only Database Queries

**Input**: S108 specification and plan packet

## Phase 1: Setup and Tracking

- [x] T001 Create the S108 branch and complete the spec-kit specification, clarification, checklists, plan, research, data model, contract, quickstart, tasks, and analysis packet
- [x] T002 Assign issue #179 and advance its project Stage to In progress and Slice to S108
- [x] T003 Advance `.specify/feature.json`, Plan 045 tracking, the active-plan index, and `CHANGELOG.md` to S108

## Phase 2: Test-First Shared Contract

- [x] T004 Enable only the required rusqlite hooks and limits plus base64 encoding support
- [x] T005 [US1] Add failing fixed-inventory, schema, absence, and redaction tests in `tests/database_query.rs`
- [x] T006 [US2] Add failing positional, named, typed-value, empty-result, duplicate-column, row-limit, and byte-limit tests
- [x] T007 [US3] Add failing mutation, DDL, pragma, attachment, multi-statement, malformed-input, timeout, busy, lock, cancellation, and replacement tests

## Phase 3: Shared Query Service

- [x] T008 [US1] Add transport-neutral inventory, schema, request, result, value, limit, and error models in `src/database_query.rs`
- [x] T009 [US1] Add the private dynamic catalog and encounter path registry
- [x] T010 [US3] Add one global two-permit admission gate and blocking-worker execution
- [x] T011 [US3] Open defended short-lived read-only connections with SQLite runtime limits
- [x] T012 [US3] Add fail-closed authorizer, one-statement, readonly, and exact parameter-shape validation
- [x] T013 [US2] Bind every supported typed input and materialize exact typed output
- [x] T014 [US2] Enforce complete-row, row-count, result-byte, duration, and cancellation bounds
- [x] T015 [US1] Implement safe bounded schema inventory for both databases

## Phase 4: HTTP and MCP Adapters

- [x] T016 [US1] Inject the shared query service and service-generation cancellation into the local host
- [x] T017 [US1] Add `GET /api/v1/databases` and `esoweave://databases`
- [x] T018 [US2] Add `POST /api/v1/databases/{database_id}/query` with canonical HTTP statuses
- [x] T019 [US2] Add the exact `query_database` MCP tool and structured result/error content
- [x] T020 [US2] Update capability metadata only after both adapters are present
- [x] T021 [US3] Update accepted catalog replacement to change the private query path for later requests

## Phase 5: Parity and Lifecycle Verification

- [x] T022 [US1] Add HTTP/MCP inventory equality coverage using the official RMCP client
- [x] T023 [US2] Add HTTP/MCP query and canonical-error parity coverage
- [x] T024 [US3] Verify authentication, Host, Origin, body size, disconnect, cancellation, restart, and stopping guards
- [x] T025 [US3] Verify three-query admission, timeout, locked database, slow-client materialization, and bounded shutdown
- [x] T026 [US3] Verify catalog replacement affects only later connections and mutation attempts preserve database bytes

## Phase 6: Documentation and Analysis

- [x] T027 Update Plan 045, the active-plan index, canonical local-extension summary, and changelog while preserving S109 scope
- [x] T028 Complete pre-implementation `analysis.md` with no unresolved critical, high, or medium finding
- [x] T029 Re-run analysis after implementation and mark the spec packet Implemented

## Phase 7: Validation and Delivery

- [x] T030 Run focused database-query and local-service tests
- [x] T031 Run format, strict clippy, full locked tests, and release build
- [x] T032 Run documentation policy, spelling, link, encoding, BOM, mojibake, and forbidden-dash checks
- [ ] T033 Commit, push, and open the authorized PR closing issue #179
- [ ] T034 Resolve every hosted review and CI finding, with at most one authorized second Codex review
- [ ] T035 Stop for the operator final review and merge ritual

## Dependencies and Execution Order

- The complete spec-kit and analysis packet precedes implementation.
- Shared-service red tests precede the query service.
- The defended service precedes HTTP and MCP adapters.
- Both adapters precede parity and lifecycle verification.
- Documentation and post-implementation analysis follow final behavior.
- Hosted review begins only after all local gates pass.

## Test-First Rule

Each implementation phase starts with its named failing tests. Existing safety-critical tests are never weakened, skipped, or made conditional.
