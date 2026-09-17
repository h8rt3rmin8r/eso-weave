# Tasks: MCP Player-State Resources

**Input**: S107 specification and plan packet

## Phase 1: Setup and Tracking

- [x] T001 Create the S107 branch and complete the spec-kit specification, clarification, checklists, plan, research, data model, contract, quickstart, tasks, and analysis packet
- [x] T002 Assign issue #178 and advance its project Stage to In progress and Slice to S107
- [x] T003 Advance `.specify/feature.json`, Plan 045 tracking, the active-plan index, and `CHANGELOG.md` to S107

## Phase 2: Test-First MCP Contract

- [x] T004 Add the official RMCP client transport as a dev-only test capability
- [x] T005 [US1] Add failing real-client initialize and exact resource-discovery integration tests
- [x] T006 [US1] Add failing real-client capability and player-state read tests
- [x] T007 [US2] Add failing HTTP/MCP equality tests over the same revision and generation
- [x] T008 [US3] Add failing unknown-resource and unauthenticated-client tests

## Phase 3: Resource Adapter

- [x] T009 [US1] Add a focused cloneable MCP state adapter with stable server and resource metadata
- [x] T010 [US1] Implement exact capability and player-state reads over one immutable snapshot reference
- [x] T011 [US1] Install the adapter in the existing stateless RMCP service factory
- [x] T012 [US2] Mark canonical MCP player-state capability true while leaving query execution false
- [x] T013 [US3] Map unknown resources and serialization failure to non-disclosing standard MCP errors

## Phase 4: Parity and Lifecycle Boundaries

- [x] T014 [US2] Verify bootstrap, active, stale, unavailable, dormant, and recovery semantics remain identical across HTTP and MCP
- [x] T015 [US2] Verify reads do not advance revision and restart generation matches both transports
- [x] T016 [US3] Add concurrent publication and multi-reader coherence coverage
- [x] T017 [US3] Add client disconnect and active-read bounded-shutdown coverage
- [x] T018 [US3] Verify bearer, Host, Origin, body-size, cancellation, and stopping guards remain unchanged

## Phase 5: Documentation and Analysis

- [x] T019 Update the canonical local-extension summary while preserving issue #179 database ownership and issue #180 documentation ownership
- [x] T020 Complete pre-implementation `analysis.md` with no unresolved critical, high, or medium finding
- [x] T021 Re-run analysis after implementation and mark the spec packet Implemented

## Phase 6: Validation and Delivery

- [x] T022 Run focused RMCP client and local-service tests
- [x] T023 Run format, strict clippy, full locked tests, and release build
- [x] T024 Run documentation policy, spelling, link, encoding, BOM, mojibake, and forbidden-dash checks
- [ ] T025 Commit, push, and open the authorized PR closing issue #178
- [ ] T026 Resolve every hosted review and CI finding, with at most one authorized second Codex review
- [ ] T027 Stop for the operator final review and merge ritual

## Dependencies and Execution Order

- The complete spec-kit and analysis packet precedes implementation.
- Real-client tests precede the resource adapter.
- The adapter precedes parity and lifecycle verification.
- Documentation and post-implementation analysis follow the final behavior.
- Hosted review begins only after all local gates pass.

## Test-First Rule

Each implementation phase starts with its named failing tests. Existing safety-critical tests are never weakened, skipped, or made conditional.
