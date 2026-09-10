# Tasks: Encounter SavedVariables Import

**Input**: S076 design documents under `specs/076-encounter-import/`

**Tests**: Required by the constitution and issue #133 hostile-data boundary

## Phase 1: Spec-kit and Project State

- [x] T001 Create the S076 feature branch and specification from issue #133
- [x] T002 Resolve clarify decisions for partial data, identity, canonicalization, store ownership, lifecycle, and scope
- [x] T003 Complete requirements and import-safety checklists
- [x] T004 Complete research, data model, import contract, and quickstart
- [x] T005 Write this chronological task plan
- [x] T006 Move issue #133 to In progress with Slice S076
- [x] T007 Run and pass the pre-implementation analysis gate

## Phase 2: Test-First Restricted Parsing

- [x] T008 Add failing shared-parser tests for root, scalar, table, escape, duplicate, sparse-array, limit, and executable-input behavior
- [x] T009 Add failing collector adapter regressions that preserve its existing grammar and empty-array semantics
- [x] T010 Extract a bounded crate-private SavedVariables parser with configurable root, limits, and empty-table policy
- [x] T011 Make shared and collector parser tests pass without changing collector import behavior

## Phase 3: Test-First Encounter Validation

- [x] T012 Add terminal complete, truthful partial, unknown-ID, and canonical round-trip fixtures
- [x] T013 Add failing schema, version, channel, privacy, unknown-field, string, integer, event-count, and byte-limit tests
- [x] T014 Add failing identity, sequence, monotonic-time, boundary-event, count, terminal-reason, and loss-interval tests
- [x] T015 Add failing exact payload-key, payload-type, actor-bound, stable-token, and sensitive-string tests for all fourteen kinds
- [x] T016 Implement typed schema-v1 encounter models with deny-unknown deserialization
- [x] T017 Implement envelope, cross-field, terminal, loss, and payload validators
- [x] T018 Implement deterministic canonical JSON plus source and canonical SHA-256 hashing
- [x] T019 Make encounter validation and canonicalization tests pass

## Phase 4: Test-First Raw Store Lifecycle

- [x] T020 Add failing new-store, empty-file, unrelated-schema, future-version, and corrupt-store tests
- [x] T021 Add failing atomic import, idempotent duplicate, semantic collision, rollback, and raw-update tests
- [x] T022 Add failing deterministic list, delete-one, delete-all, and no-automatic-prune tests
- [x] T023 Add failing consistent backup, destination replacement, digest, integrity, and failed-publication tests
- [x] T024 Add the rusqlite backup feature without adding a new crate
- [x] T025 Implement encounter store schema v1 and pre-mutation integrity gates
- [x] T026 Implement immutable transactional append, receipts, deterministic listing, and collision handling
- [x] T027 Implement explicit transactional deletion operations
- [x] T028 Implement consistent temporary snapshot backup, final hash, and atomic publication
- [x] T029 Make all raw-store lifecycle tests pass

## Phase 5: Import and CLI Integration

- [x] T030 Add failing bounded-read, symlink/reparse, path-alias, stable-metadata, and no-mutation import tests
- [x] T031 Implement the explicit import request pipeline over bounded read, parse, validation, canonicalization, and store append
- [x] T032 Add failing CLI tests for encounter import, list, backup, delete-one, delete-all, usage, JSON receipts, and nonzero errors
- [x] T033 Add non-interactive encounter lifecycle commands to `catalog-compiler`
- [x] T034 Make import and CLI integration tests pass
- [x] T035 Add and pass the 100,000-event production-budget verification

## Phase 6: Documentation and Project Records

- [x] T036 Update canonical encounter-data documentation for implemented import and ownership behavior
- [x] T037 Update architecture and test-strategy documentation if the shared parser or store boundary requires it
- [x] T038 Update Plan 038 and its index chronologically for completed S075 and active S076
- [x] T039 Update machine-readable and prose migration ledgers
- [x] T040 Add S076 Added, Changed, and dated Decisions entries to `CHANGELOG.md`
- [x] T041 Reconcile all contract examples and run the post-implementation analysis gate

## Phase 7: Verification

- [x] T042 Run `cargo fmt --all -- --check`
- [x] T043 Run `cargo clippy --all-targets --all-features -- -D warnings`
- [x] T044 Run `cargo test --all --locked`
- [x] T045 Build optimized application and catalog compiler binaries
- [x] T046 Run mdBook tests, build, linkcheck, policy, and spelling gates
- [x] T047 Parse every JSON artifact and run whitespace checks
- [x] T048 Run UTF-8 without BOM, forbidden-dash, and mojibake scans
- [x] T049 Confirm only intended files changed and the user-owned untracked note is untouched

## Phase 8: Publication and Review

- [x] T050 Commit S076 with required attribution
- [x] T051 Push `codex/s076-encounter-import` and open the official PR closing #133
- [x] T052 Move issue #133 to PR review and record the PR link
- [x] T053 Wait for all CI checks and first-round external reviews
- [x] T054 Resolve every actionable review thread with verified corrections
- [x] T055 Trigger at most one authorized second Codex review round if appropriate
- [x] T056 Resolve second-round feedback and confirm all checks green
- [x] T057 Ask the operator for the final review and merge ritual

## Dependencies

- T007 blocks implementation.
- T008 and T009 precede T010 under test-first discipline.
- T012 through T015 precede T016 through T019.
- T020 through T023 precede T024 through T029.
- T029 blocks integrated import and CLI work.
- T030 and T032 precede their corresponding implementation tasks.
- T034 blocks production-budget verification and completion documentation.
- T042 through T049 block the first commit and remote publication.
- T053 through T056 block the merge-ritual request.

## Parallel Opportunities

Within a phase, independent test cases or documentation pages may be authored in
parallel, but phase order remains chronological and no implementation precedes
the analysis gate or its failing tests.
