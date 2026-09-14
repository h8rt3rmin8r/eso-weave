# Tasks: Persistent Data Addon Foundation

**Input**: Design documents from `specs/092-data-addon-foundation/`
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`

**Tests**: Required by the specification and constitution. Each behavior change
starts with a failing or deliberately updated test before production code.

## Phase 1: Setup and Governance

- [x] T001 Amend the approved two-addon governance boundary in `.specify/memory/constitution.md`
- [x] T002 Update the active spec-kit context reference in `CLAUDE.md`
- [x] T003 Complete and validate the S092 specification artifacts in `specs/092-data-addon-foundation/`
- [x] T004 Assign Slice S092 and truthful Project stages to GitHub issues #184, #187, and #189 while activating epic #182, then record the result in `specs/092-data-addon-foundation/tasks.md`

---

## Phase 2: Foundational Failing Tests

**Purpose**: Freeze the package, parser, mutation, and module-isolation contracts
before changing the addon topology.

- [x] T005 Add exact two-directory repository and four-file managed package inventory tests in `tests/data_addon.rs`
- [x] T006 Add marker, link, foreign-entry, rollback, confinement, and PixelBeacon-preservation lifecycle tests in `tests/data_addon.rs`
- [x] T007 [P] Update catalog addon harness expectations for the shared root and module-local clear in `tests/collector_addon.rs`
- [x] T008 [P] Update encounter addon harness expectations for the shared root and isolated module state in `tests/encounter_addon.rs`
- [x] T009 Add cross-module dormancy, namespace, and clear-preservation tests in `tests/data_addon.rs`
- [x] T010 [P] Add shared-root and 128 MiB outer-bound parser fixtures in `tests/collector_import.rs` and `tests/encounter_import.rs`
- [x] T011 Add a regression test prohibiting desktop deletion of the shared SavedVariables file in `tests/catalog_update.rs`

**Checkpoint**: The new S092 tests fail for the old three-addon topology and
would catch shared-file deletion or module namespace collision.

---

## Phase 3: User Story 1 - Install One Durable Data Addon (Priority: P1)

**Goal**: Replace the two development data packages with one marker-managed
`EsoWeaveData` package while preserving both modules' current behavior.

**Independent Test**: Exercise the exact package inventory, managed lifecycle,
each Lua module separately, both modules together, both parser projections, and
module-local clear preservation.

### Implementation

- [x] T012 [US1] Create the shared package manifest and bootstrap in `addon/EsoWeaveData/EsoWeaveData.txt` and `addon/EsoWeaveData/EsoWeaveData.lua`
- [x] T013 [US1] Port catalog capture into the isolated module and add `/ewcollect clear confirm` in `addon/EsoWeaveData/Catalog.lua`
- [x] T014 [US1] Port encounter capture into the isolated module with unique event namespaces in `addon/EsoWeaveData/Encounter.lua`
- [x] T015 [US1] Implement exact multi-file marker-managed lifecycle and rollback in `src/data_addon.rs`
- [x] T016 [US1] Export the data-addon lifecycle authority and remove collector-owned lifecycle exports in `src/lib.rs` and `src/collector/mod.rs`
- [x] T017 [US1] Parse `EsoWeaveDataSaved.catalog` under the 128 MiB outer bound while retaining catalog limits in `src/collector/parser.rs` and `src/collector/mod.rs`
- [x] T018 [US1] Parse `EsoWeaveDataSaved.encounter` under the shared outer bound while retaining encounter validation in `src/encounter/mod.rs`
- [x] T019 [US1] Replace collector package lifecycle commands and paths with data-addon ownership in `src/catalog_update/worker.rs`, `src/catalog_update/mod.rs`, and `src/catalog_update/availability.rs`
- [x] T020 [US1] Remove desktop shared-file deletion and present the module-local clear instruction in `src/catalog_update/worker.rs` and `src/app/ui.rs`
- [x] T021 [US1] Update encounter capture-path resolution to `SavedVariables/EsoWeaveData.lua` in `src/app/mod.rs` and encounter CLI consumers
- [x] T022 [US1] Update compiler and application imports to use data-addon running state and lifecycle types in `src/bin/catalog-compiler.rs` and `src/app/ui.rs`
- [x] T023 [US1] Adapt catalog, encounter, history, metrics, CLI, and application fixtures to the shared root in `tests/` and `tests/fixtures/`
- [x] T024 [US1] Remove obsolete `addon/EsoWeaveCollector/`, `addon/EsoWeaveEncounter/`, and `tests/collector_lifecycle.rs` after replacement coverage passes

**Checkpoint**: User Story 1 passes independently with exactly two addon
directories and no legacy reader, migration, or dual deployment.

---

## Phase 4: User Story 2 - Choose a Truthful Encounter-Ingestion Path (Priority: P1)

**Goal**: Publish the provisional native-log decision without claiming missing
Windows or Linux/Proton measurements.

**Independent Test**: Trace every comparison and qualification rule to primary
source or repository evidence and verify that field-only rows are assigned to a
separate Release verification owner.

- [x] T025 [US2] Finalize source-pinned ingestion findings and fallback hierarchy in `specs/092-data-addon-foundation/research.md`
- [x] T026 [US2] Finalize the reproducible field matrix and receipt schema in `specs/092-data-addon-foundation/contracts/ingestion-transport-decision.md`
- [x] T027 [US2] Document the native-log candidate, supported ESO controls, ownership cautions, and provisional status in `docs/src/development/encounter-ingestion.md`
- [x] T028 [US2] Create a separate native-log Windows and Linux/Proton verification issue, reconcile #187, and record both URLs in `specs/092-data-addon-foundation/research.md`

**Checkpoint**: Native incremental tailing is explicitly provisional, terminal
SavedVariables remains the safe fallback, and no unmeasured result is called a
pass.

---

## Phase 5: User Story 3 - Decide Whether Command Ingress Is Acceptable (Priority: P1)

**Goal**: Publish a supported-evidence no-go decision with zero approved desktop
commands and explicit reconsideration gates.

**Independent Test**: Account for every documented candidate across all seven
comparison dimensions and statically prove no command implementation or binding
surface was introduced.

- [x] T029 [US3] Finalize source-pinned command candidate analysis in `specs/092-data-addon-foundation/research.md`
- [x] T030 [US3] Finalize the no-go disposition and reconsideration gate in `specs/092-data-addon-foundation/contracts/command-transport-decision.md`
- [x] T031 [US3] Publish the user-driven control decision and separation from native binding discovery in `docs/src/development/companion-addon-commands.md`
- [x] T032 [US3] Add documentation and static policy coverage prohibiting custom actions, binding mutation, and command ingress in `.github/scripts/docs-policy.test.mjs`
- [x] T033 [US3] Reconcile #189 with the no-go decision and record the issue URL in `specs/092-data-addon-foundation/research.md`

**Checkpoint**: The minimum approved desktop command vocabulary is zero and
user-driven addon controls remain the selected operational path.

---

## Phase 6: Canonical Documentation and Delivery Evidence

- [x] T034 Update two-addon architecture and responsible-use guidance in `docs/src/development/architecture.md` and `docs/src/getting-started/responsible-use.md`
- [x] T035 Update catalog and encounter module guidance in `docs/src/development/discovery-collector.md`, `docs/src/features/encounter-capture.md`, and related reference pages
- [x] T036 Update embedded release inventory in `docs/src/development/release-and-packaging.md` and `docs/project/releasing.md`
- [x] T037 Update navigation, glossary, project model, and source-of-truth references in `docs/src/SUMMARY.md`, `docs/src/reference/glossary.md`, and `docs/project/encounter-model.json`
- [x] T038 Update `CHANGELOG.md` `[Unreleased]` Added and dated Decisions entries for S092, constitution 3.0.0, the 128 MiB outer bound, native-log provisional direction, and command no-go
- [x] T039 Reconcile `CLAUDE.md` and `docs/project/build-autopilot.md` safety wording with constitution 3.0.0
- [x] T040 Run UTF-8, LF, forbidden-dash, mojibake, spelling, link, and documentation-policy checks from `.github/scripts/` across every S092 text artifact

---

## Phase 7: Analysis, Verification, and Publication

- [x] T041 Run the read-only `/speckit.analyze` gate and resolve every CRITICAL or HIGH finding in `specs/092-data-addon-foundation/analysis.md`
- [x] T042 Run `cargo fmt --all -- --check` in the foreground
- [x] T043 Run `cargo clippy --all-targets --all-features -- -D warnings` in the foreground
- [x] T044 Run `cargo test --all --locked` in the foreground
- [x] T045 Complete a local code, security, and domain review and record corrected high-confidence findings in `specs/092-data-addon-foundation/analysis.md`
- [x] T046 Mark every completed task and set `spec.md` status to implemented in `specs/092-data-addon-foundation/`
- [ ] T047 Commit S092 with changelog evidence, push `codex/s092-data-addon-foundation`, and open the official pull request
- [ ] T048 Wait for CI and external reviews, address every finding, resolve every thread, and run at most the authorized second `@Codex` review round
- [ ] T049 When CI and reviews are satisfied, record completion in `specs/092-data-addon-foundation/tasks.md` and request the operator's final review and merge ritual

---

## Dependencies and Execution Order

- Phase 1 precedes planning and metadata assertions.
- Phase 2 tests precede every User Story 1 production change.
- User Story 1 must complete before canonical documentation describes the final
  package.
- User Stories 2 and 3 are decision work and may proceed independently after
  governance, but their final documentation lands after the package terminology
  is stable.
- Phase 6 depends on all three stories.
- Phase 7 is the blocking integration and publication gate.

## Parallel Opportunities

- T007 and T008 target separate Lua harnesses.
- T010 targets separate catalog and encounter parser fixtures after the shared
  outer-envelope contract is fixed.
- User Stories 2 and 3 have independent sources and decision contracts.
- Documentation files in T034 through T037 may be drafted independently, then
  receive one terminology pass.

## Implementation Strategy

The MVP is User Story 1: one managed data addon with isolated modules and safe
shared state. User Stories 2 and 3 add only decision evidence and do not enlarge
the runtime boundary. Deliver chronologically in the order above so each commit
and pull-request review sees one stable authority chain.
