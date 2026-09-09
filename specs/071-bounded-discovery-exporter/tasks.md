# Tasks: Bounded ESO Discovery Exporter

**Input**: Design documents from `specs/071-bounded-discovery-exporter/`

**Prerequisites**: `spec.md`, both checklists, `research.md`, `data-model.md`,
`contracts/`, `plan.md`, and `quickstart.md`

**Tests**: Required by the constitution and issue #115. Each implementation
phase begins with failing tests or fixtures before production changes.

## Phase 1: Spec-kit and Analyze Gate

- [x] T001 Create the S071 feature, specification, clarification record, and
  requirements checklist from issue #115 and the S068/S070 authorities.
- [x] T002 Create the collector-safety checklist covering isolation, limits,
  hostile input, atomic staging, privacy, and verification.
- [x] T003 Produce research, data model, envelope contract, implementation plan,
  and quickstart with all routine architecture decisions recorded.
- [x] T004 Run the pre-implementation analyze gate across spec, plan, tasks,
  constitution, issue #115, and canonical docs; resolve every finding.

## Phase 2: Hostile Import Tests

- [x] T005 [US2] Add invented multi-class live and PTS SavedVariables fixtures
  in `specs/071-bounded-discovery-exporter/fixtures/`.
- [x] T006 [US2] Add parser rejection tests for functions, expressions,
  references, comments, long strings, duplicate keys, mixed shapes, deep tables,
  oversized files/strings, invalid escapes, and trailing syntax in
  `tests/collector_import.rs`.
- [x] T007 [US2] Add envelope rejection tests for incomplete status, unsupported
  versions, missing/duplicate chunks, false counts/bytes/checksums, invalid
  records, category/type mismatches, duplicate records, and channel mismatch.
- [x] T008 [US2] Add atomic staging and path-alias tests proving all failed
  imports preserve existing output and valid repeated imports are byte-stable.
- [x] T009 [US2] Add end-to-end tests proving valid live and PTS staging passes
  the S070 model validator and compiler and retains local-only text provenance.

## Phase 3: Restricted Parser and Envelope

- [x] T010 [US2] Define collector constants, errors, typed envelope, chunk,
  coverage, checkpoint, and record models in `src/collector/mod.rs`.
- [x] T011 [US2] Implement the bounded lexer and recursive table/scalar parser
  in `src/collector/parser.rs` without a Lua runtime.
- [x] T012 [US2] Enforce byte, token, depth, entry, string, numeric, root,
  duplicate-key, and trailing-syntax limits during parsing.
- [x] T013 [US2] Convert the parsed fixed root into typed models with strict
  unknown-field and table-shape rejection.
- [x] T014 [US2] Verify complete status, supported versions, selection/coverage,
  contiguous chunks, UTF-8 bytes, record counts, and Adler-32 checksums.

## Phase 4: Deterministic Compiler Staging

- [x] T015 [US2] Validate category/entity matrices, stable IDs, parent
  relationships, scalar attributes, local text, virtual paths, and duplicates.
- [x] T016 [US2] Map envelope provenance and records into sorted S070 catalog
  bundle structures in `src/collector/import.rs`.
- [x] T017 [US2] Populate bounded selected-category coverage and truthful Unknown
  declarations for every unselected S070 category.
- [x] T018 [US2] Compute raw, record, and staged SHA-256 provenance and return a
  redacted deterministic import receipt.
- [x] T019 [US2] Implement canonical path separation and durable same-directory
  atomic staging publication with failure preservation.
- [x] T020 [US2] Add `import-collector` to `src/bin/catalog-compiler.rs` without
  allowing direct SQLite publication.

## Phase 5: Independent Lifecycle Tests and Implementation

- [x] T021 [US3] Add lifecycle tests for absent, managed, mismatched, unmanaged,
  file, directory, symlink/reparse, write-failure, update, and removal states in
  `tests/collector_lifecycle.rs`.
- [x] T022 [US3] Add confinement tests proving every collector write/removal
  stays inside `EsoWeaveCollector` and PixelBeacon bytes never change.
- [x] T023 [US3] Define embedded collector identity, marker, manifest, Lua,
  version parsing, status, and pure SavedVariables path helpers in
  `src/collector/lifecycle.rs`.
- [x] T024 [US3] Implement transactional best-effort install/update and strict
  marker-gated removal with linked/non-regular target refusal.
- [x] T025 [US3] Add `collector-status`, `collector-install`, and
  `collector-remove` catalog-compiler commands with explicit AddOns roots.

## Phase 6: Addon Contract Tests and Implementation

- [x] T026 [US1] Add static addon tests for dedicated identity, managed marker,
  SavedVariables root, API versions, slash commands, category adapter matrix,
  stable IDs, and prohibited PixelBeacon/input/network APIs in
  `tests/collector_addon.rs`.
- [x] T027 [US1] Add static tests for explicit start, combat refusal/pause,
  explicit resume, cancel/failure state, checkpoint, record/time budgets, hard
  limits, sorted normalization, JSON escaping, chunking, and Adler-32.
- [x] T028 [US1] Create `addon/EsoWeaveCollector/EsoWeaveCollector.txt` with
  unique ownership, supported API versions, and SavedVariables declaration.
- [x] T029 [US1] Implement bounded collector state, commands, progress, combat
  transitions, checkpoints, warnings, completion, and save-boundary guidance in
  `addon/EsoWeaveCollector/EsoWeaveCollector.lua`.
- [x] T030 [US1] Implement the five approved iterator adapters using stable IDs
  and truthful bounded visibility declarations.
- [x] T031 [US1] Implement canonical JSON record encoding, deterministic sorting,
  chunk limits, byte/count metadata, and Adler-32 finalization.

## Phase 7: Documentation and Project Records

- [x] T032 [US4] Add the user and maintainer collector guide under `docs/src/`
  and link it through SUMMARY and relevant reference/development indexes.
- [x] T033 [US4] Update architecture, test strategy, status reference, source
  rights, and troubleshooting with scope, privacy, save, staging, and removal
  behavior.
- [x] T034 Update `CHANGELOG.md` Added and dated Decisions entries for S071 and
  the constrained JSON-line SavedVariables bridge.
- [x] T035 Advance Plan 038 and its index chronologically from completed S070 to
  active S071, preserving the non-blocking verification and encounter tracks.
- [x] T036 Update the migration ledger and spec-kit feature pointer for S071.

## Phase 8: Analyze, Verification, and Delivery

- [x] T037 Complete the post-implementation analyze gate and write
  `specs/071-bounded-discovery-exporter/analysis.md` with requirement and
  constitution traceability.
- [x] T038 Run focused collector tests, full fmt, strict Clippy, locked tests,
  optimized builds, documentation policy, mdBook, JSON, text hygiene, forbidden
  dash, mojibake, and package assertions.
- [x] T039 Inspect the complete diff for secrets, personal data, third-party art,
  unrelated edits, and preservation of user-owned files.
- [ ] T040 Commit as `feat(071): build bounded discovery exporter` with the
  required co-author trailer.
- [ ] T041 Push the authorized branch, open the official PR with `Closes #115`,
  and move the project item to PR review.
- [ ] T042 Resolve every CI and external review finding, request at most the one
  authorized second `@Codex` round, and stop when all checks and reviews pass.

## Dependencies and Execution Order

Spec-kit and the initial analyze gate precede implementation. Hostile-import
tests precede parser and staging code. Lifecycle tests precede managed addon
mutation. Static addon tests precede Lua implementation. Documentation and
project records follow stable behavior. The post-implementation analyze gate and
all local verification precede the first commit and authorized remote delivery.
