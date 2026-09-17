# Tasks: Canonical Player-State HTTP API

**Input**: S106 specification and plan packet

## Phase 1: Setup and Tracking

- [x] T001 Advance `.specify/feature.json` and Plan 045 tracking to S106
- [x] T002 Record S106 scope and architecture decision in `CHANGELOG.md`

## Phase 2: Foundational Snapshot Authority

- [x] T003 Add failing publisher revision, bootstrap, and immutable-read tests in `tests/player_state.rs`
- [x] T004 Add failing S104 public and non-public inventory coverage tests
- [x] T005 Implement typed snapshot envelope, capability model, observation helpers, and immutable publisher in `src/player_state.rs`
- [x] T006 Export the player-state module from `src/lib.rs`

## Phase 3: User Story 1 - Complete Canonical Snapshot

- [x] T007 [US1] Add failing active fixture and complete path-shape tests
- [x] T008 [US1] Implement authoritative projection from application, game, PixelBus, player, automation, and interpretation sources
- [x] T009 [US1] Align process-transition mutation and snapshot capture lock order
- [x] T010 [US1] Publish canonical state from the application update boundary

## Phase 4: User Story 2 - HTTP Discovery and State

- [x] T011 [US2] Add failing capability, player-state, method, and route integration tests in `tests/local_service.rs`
- [x] T012 [US2] Inject the shared publisher into `LocalServiceController` and its runtime owner
- [x] T013 [US2] Implement GET `/api/v1`, `/api/v1/capabilities`, and `/api/v1/player-state`
- [x] T014 [US2] Preserve existing authentication, Host, Origin, body, cancellation, and stopping guards

## Phase 5: User Story 3 - Loss, Recovery, and Concurrency

- [x] T015 [US3] Add failing focus-loss, signal-loss, dormancy, stale, and recovery projection tests
- [x] T016 [US3] Add failing no-torn-revision, slow-reader, and bounded-shutdown tests
- [x] T017 [US3] Implement conservative knowledge and freshness projection without HUD retention
- [x] T018 [US3] Verify semantic no-op publication preserves revision and public change advances it once

## Phase 6: Analysis and Delivery

- [x] T019 Complete `analysis.md` with no unresolved critical, high, or medium finding
- [x] T020 Run focused tests and quickstart validation
- [x] T021 Run fmt, strict clippy, full locked tests, and release build
- [x] T022 Run documentation policy, spelling, link, encoding, BOM, mojibake, and forbidden-dash checks
- [ ] T023 Commit, push, and open the authorized PR closing issue #177
- [ ] T024 Resolve every hosted review and CI finding, with at most one authorized second Codex review
- [ ] T025 Stop for the operator final review and merge ritual

## Dependencies and Execution Order

- Setup precedes all implementation.
- Publisher and inventory tests precede projection.
- Projection precedes HTTP route implementation.
- Loss and concurrency verification depend on both publisher and routes.
- Hosted review begins only after all local gates pass.

## Test-First Rule

Each implementation phase starts with its named failing tests. Existing safety-critical tests are never weakened, skipped, or made conditional.
