# Tasks: Local Extension Documentation and End-to-End Verification

**Input**: S109 specification and plan packet

**Tests**: Documentation-contract, policy, production-adapter parity, and lifecycle tests are mandatory and precede the artifacts or assertions they validate.

## Phase 1: Specification and Tracking

- [x] T001 Create the S109 branch and complete the spec-kit specification, clarification, requirements checklist, extension checklist, plan, research, data model, contract, quickstart, tasks, and analysis packet
- [x] T002 Assign issue #180 and advance its project Stage to In progress and Slice to S109
- [x] T003 Advance `.specify/feature.json`, Plan 045, the active-plan index, canonical local-extension summary, and `CHANGELOG.md` to S109

## Phase 2: Test-First Documentation Contract

- [x] T004 [US3] Add failing guide-presence, UI-copy, operation, bound, placeholder, privacy, and navigation assertions in `tests/local_extension_contract.rs`
- [x] T005 [US3] Add failing machine-readable field inventory equality, uniqueness, schema-version, and metadata assertions in `tests/local_extension_contract.rs`
- [x] T006 [US3] Add failing new-page table and example inventory expectations in `.github/scripts/docs-policy.test.mjs`
- [x] T007 Run the focused Rust and Node.js tests and record the expected failures before adding the guide, inventory, or policy registration

## Phase 3: Canonical User and Integrator Documentation

- [x] T008 [US1] Add enablement, status, discovery, bearer authentication, HTTP operations, and MCP operations to `docs/src/reference/local-api-and-mcp.md`
- [x] T009 [US1] Add copyable Bash HTTP, PowerShell HTTP, and standard Streamable HTTP MCP client configuration examples using placeholders
- [x] T010 [US2] Document the player-state envelope, revisions, observation knowledge and freshness, and compatibility policy
- [x] T011 [US2] Add all 58 typed public field entries to `docs/src/reference/local-extension-state-fields.json`
- [x] T012 [US2] Document database discovery, parameter forms, typed values, limits, truncation, errors, and safe read-only workflows
- [x] T013 [US1] Document disabled service, collision, stale discovery, authentication, Host or Origin, protocol, schema, database, limit, disconnect, restart, and shutdown troubleshooting
- [x] T014 [US1] Add the canonical guide to `docs/src/SUMMARY.md` and `docs/src/reference/README.md`
- [x] T015 [US1] Link settings, general troubleshooting, responsible use, architecture, data or storage, and test-strategy pages to the canonical guide

## Phase 4: Documentation Policy and Executable Examples

- [x] T016 [US3] Register the new canonical page, semantic tables, and language-tagged examples in `.github/scripts/docs-policy.mjs` without relaxing existing checks
- [x] T017 [US3] Complete focused policy tests in `.github/scripts/docs-policy.test.mjs` for the exact S109 additions
- [x] T018 Record the dated pinned documentation-policy decision and user-visible guide delivery in `CHANGELOG.md`
- [x] T019 Run the focused documentation-contract and policy tests to green

## Phase 5: Production-Adapter End-to-End Evidence

- [x] T020 [US3] Strengthen `tests/local_service.rs` to exercise one deterministic snapshot plus catalog and encounter fixtures through the production listener
- [x] T021 [US3] Compare HTTP and official RMCP capabilities, player state, database inventory, typed query success, and denied-query error with only transport framing and `elapsed_ms` normalized
- [x] T022 [US3] Bind state parity to one snapshot revision and service generation
- [x] T023 [US3] Complete collision recovery, client disconnect, generation rollover, stale endpoint, discovery cleanup, and bounded shutdown assertions without duplicating existing lifecycle harnesses
- [x] T024 Run the focused `local_extension_contract`, `local_service`, `player_state`, and `database_query` tests to green

## Phase 6: Roadmap and Canonical Contract Closure

- [x] T025 Keep Plan 045 active through operator merge, identify S109 as its final active slice, and reserve archival for post-merge housekeeping with concrete pull-request evidence
- [x] T026 Update `docs/project/local-extension-contract.md` to identify the published guide and final verification authority
- [x] T027 Leave issue #180 and epic #174 open for their pull-request and post-merge closure evidence
- [x] T028 Complete `specs/109-extension-docs-e2e/analysis.md` with post-implementation traceability and no unresolved critical, high, or medium finding

## Phase 7: Full Verification

- [x] T029 Run `cargo fmt --all -- --check`
- [x] T030 Run strict Clippy for all targets and features
- [x] T031 Run the full locked Rust test suite
- [x] T032 Run the release build for the `eso-weave` binary
- [x] T033 Run documentation policy tests, mdBook tests, build, generated-site policy, and link checks
- [x] T034 Run repository trust policy tests and scanner
- [x] T035 Verify UTF-8 without BOM, LF-only text, no forbidden dashes, no mojibake, no credential-shaped documentation literal, and a clean diff
- [x] T036 Mark the S109 specification implemented and every completed task checked

## Phase 8: Publication and Hosted Review

- [ ] T037 Commit the verified S109 implementation and push `codex/s109-extension-docs-e2e`
- [ ] T038 Open the official pull request with `Closes #180`, scope, evidence, and explicit exclusions
- [ ] T039 Advance issue #180 project Stage to PR review
- [ ] T040 Wait for hosted CI, Codex, and security review results and answer every review comment
- [ ] T041 Apply and verify any required review changes, resolve every review thread, and push updates
- [ ] T042 Trigger at most one authorized second Codex review, then process it to completion
- [ ] T043 Confirm every required check is green, every review is satisfied, the branch is clean, and the pull request is mergeable
- [ ] T044 Stop for the operator's final review and merge ritual

## Dependencies and Execution Order

- Phase 1 completes the full spec-kit chain before implementation.
- Phase 2 adds focused failing tests before the documentation artifacts and policy registration.
- Phase 3 satisfies the public user and integrator journey and unblocks policy registration.
- Phase 4 turns examples and presentation structure into executable documentation evidence.
- Phase 5 reuses the existing production adapter seams and adds only missing parity or lifecycle evidence.
- Phase 6 closes repository planning records only after shipped behavior and documentation agree.
- Phase 7 is the local merge gate for Phase 8 publication.
- Phase 8 may trigger only the automatic opening review and one explicit second Codex review.

## Parallel Opportunities

- T004 through T006 touch separate Rust and Node.js test surfaces.
- T008 through T013 contribute independent guide sections after the initial page exists.
- T014 and T015 touch separate navigation and cross-reference pages.
- T020 through T023 share `tests/local_service.rs` and therefore execute chronologically.
- Documentation, Rust, and trust gates may run concurrently only when each process remains foreground-observable and output is retained.
