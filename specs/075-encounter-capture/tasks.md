# Tasks: Privacy-Minimized Encounter Capture

**Input**: S075 design documents under `specs/075-encounter-capture/`

**Tests**: Required by the constitution and issue #132 safety boundary

## Phase 1: Spec-kit and Governance

- [x] T001 Create the S075 feature directory and specification from issue #132
- [x] T002 Resolve clarify decisions for consent, scope, hashing, lifecycle, and overflow
- [x] T003 Complete requirements and capture-safety checklists
- [x] T004 Complete research, data model, contracts, fixture, and quickstart
- [x] T005 Write this chronological task plan
- [x] T006 Run and pass the pre-implementation analysis gate
- [x] T007 Amend constitution 2.1.0 to 2.2.0 and align `CLAUDE.md`
- [x] T008 Move issue #132 and project Slice S075 to In progress

## Phase 2: Test-First Capture Contract

- [x] T009 Add the vendored Lua 5.1 test-only dependency
- [x] T010 Add a failing harness test for dormant load, explicit channel arm, and one-shot authority
- [x] T011 Add a failing representative capture test for all required event families
- [x] T012 Add failing privacy tests that inject sensitive callback strings
- [x] T013 Add failing sequence, elapsed-time, overflow, and discontinuity tests
- [x] T014 Add failing stop, deactivation, clear, callback failure, and teardown tests
- [x] T015 Add failing static confinement and independent addon identity tests
- [x] T016 Add failing normalized fixture invariants for the S075 contract

## Phase 3: Encounter Addon Implementation

- [x] T017 Add the dedicated `EsoWeaveEncounter` manifest and SavedVariables root
- [x] T018 Implement dormant, armed, capturing, complete, and partial states
- [x] T019 Implement encounter-local actor mapping with an explicit bound
- [x] T020 Implement sequence and nondecreasing elapsed-time ownership
- [x] T021 Implement damage, healing, effect, resource, cast, bar, life, boss, performance, and quickslot handlers
- [x] T022 Implement event and estimated-byte budgets with reserved terminal capacity
- [x] T023 Implement overflow and clock-reset discontinuities
- [x] T024 Implement normal, stop, deactivation, and callback-failure finalization
- [x] T025 Implement arm, disarm, stop, status, clear-confirm, and help commands
- [x] T026 Make every executed Lua and confinement test pass

## Phase 4: Documentation and Project Records

- [x] T027 Add the canonical encounter-capture user and maintainer guide
- [x] T028 Update encounter reference, architecture, testing, and responsible-use documentation
- [x] T029 Update Plan 038 and its index chronologically for completed S074 and active S075
- [x] T030 Update machine-readable and prose migration ledgers
- [x] T031 Add S075 Added and dated Decisions entries to `CHANGELOG.md`
- [x] T032 Reconcile the representative fixture and post-implementation analysis

## Phase 5: Verification

- [x] T033 Run `cargo fmt --all -- --check`
- [x] T034 Run `cargo clippy --all-targets --all-features -- -D warnings`
- [x] T035 Run `cargo test --all --locked`
- [x] T036 Build optimized application and catalog compiler binaries
- [x] T037 Run mdBook tests, build, linkcheck, policy, and spelling gates
- [x] T038 Parse every JSON artifact and run whitespace checks
- [x] T039 Run UTF-8 without BOM, forbidden-dash, and mojibake scans
- [x] T040 Confirm only intended files changed and the user-owned untracked note is untouched

## Phase 6: Publication and Review

- [x] T041 Commit S075 with required attribution
- [x] T042 Push `codex/s075-encounter-capture` and open the official PR closing #132
- [x] T043 Move issue #132 to PR review and record the PR link
- [x] T044 Wait for all CI checks and first-round external reviews
- [x] T045 Resolve every actionable review thread with verified corrections
- [x] T046 Trigger at most one authorized second Codex review round if appropriate
- [x] T047 Resolve second-round feedback and confirm all checks green
- [x] T048 Ask the operator for the final review and merge ritual

## Dependencies

- T006 blocks implementation.
- T007 must precede the third addon artifact.
- T009 through T016 precede T017 through T026 under test-first discipline.
- T026 blocks documentation completion claims and final verification.
- T033 through T040 block the first commit and remote publication.
- T044 through T047 block the merge-ritual request.

## Parallel Opportunities

Within a phase, independent test cases or documentation pages may be authored in
parallel, but phase order remains chronological and no implementation precedes
the analysis gate or its failing behavioral tests.
