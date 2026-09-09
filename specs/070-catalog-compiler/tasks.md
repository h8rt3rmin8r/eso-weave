# Tasks: Deterministic SQLite Catalog Compiler

**Input**: Design documents from `specs/070-catalog-compiler/`

## Phase 1: Specification and governance

- [x] T001 Create the S070 spec with prioritized, independently testable user
  stories and issue #114 acceptance traceability.
- [x] T002 Run autonomous clarification and record the tool placement, schema,
  hashing, publication, package, and rights decisions.
- [x] T003 Complete requirements and catalog-integrity domain checklists.
- [x] T004 Produce research, data model, JSON contract, quickstart, and the
  pre-implementation analyze gate.
- [x] T005 Assign Slice S070 and move issue #114 to Specced in the Delivery
  Project.

## Phase 2: Failing compiler and runtime tests

- [x] T006 [US1] Add normalized live, changed-live, PTS, multilingual, and invalid
  fixture builders in `tests/catalog_support/mod.rs` and the S070 fixture set.
- [x] T007 [US1] Add failing tests for bounded input, stable identity,
  provenance, coverage, constraints, and deterministic repeated builds in
  `tests/catalog_compiler.rs`.
- [x] T008 [US2] Add failing tests for live/PTS isolation, candidate diff,
  last-known-good preservation, rollback evidence, and failed publication in
  `tests/catalog_compiler.rs`.
- [x] T009 [US3] Add failing tests for read-only typed access, missing/corrupt or
  incompatible databases, semantic checksum mismatch, and controlled reopen in
  `tests/catalog_runtime.rs`.
- [x] T010 [US4] Add failing MSI, Debian, AppImage, tarball, baseline-rights, and
  release-layout assertions in `tests/catalog_packaging.rs`.

## Phase 3: Catalog model and database foundation

- [x] T011 Add locked bundled rusqlite, SHA-256, and atomic temporary-file
  dependencies to `Cargo.toml` and `Cargo.lock`.
- [x] T012 [US1] Implement strict bounded JSON types, vocabularies, sorting, and
  cross-record validation in `src/catalog/model.rs`.
- [x] T013 [US1] Implement schema version 1, SQL constraints, forward-migration
  boundary, canonical row projection, and database verification in
  `src/catalog/schema.rs`.
- [x] T014 [US1] Implement transactional deterministic insertion, semantic and
  artifact hashing, grouped diagnostics, and stable reports in
  `src/catalog/compiler.rs`.

## Phase 4: Safe publication and maintainer command

- [x] T015 [US2] Implement candidate sync/reopen, categorized diff, rollback
  copy and manifest, atomic persistence, and failure preservation in
  `src/catalog/compiler.rs`.
- [x] T016 [US2] Implement explicit `build`, `verify`, and `diff` commands with
  stable JSON output in `src/bin/catalog-compiler.rs`.
- [x] T017 [US1] Generate and verify the valid, changed-live, and PTS S070 fixture
  evidence.

## Phase 5: Typed application access

- [x] T018 [US3] Implement the read-only `CatalogReader`, `CatalogAccess`, typed
  release/entity queries, diagnostics, and controlled locator in
  `src/catalog/mod.rs`.
- [x] T019 [US3] Wire startup probing, warning logging, model ownership, and one
  user-visible Catalog status line through `src/main.rs` and `src/app/`.
- [x] T020 [US3] Confirm the catalog connection performs no startup writes,
  network access, background polling, or in-place replacement.

## Phase 6: Baseline and packages

- [x] T021 [US4] Add the reviewable rights-compatible authority at
  `assets/catalog/baseline.json` and generate `assets/catalog/catalog.sqlite`.
- [x] T022 [US4] Add the baseline to MSI and Debian package manifests.
- [x] T023 [US4] Add the baseline to AppImage and Linux tarball assembly with the
  documented `catalog/catalog.sqlite` relative layout.
- [x] T024 [US4] Extend packaging tests to prove every layout and prohibit
  third-party icon bytes and user-collected prose.

## Phase 7: Documentation and project state

- [x] T025 Publish maintainer build, inspect, diff, approval, rollback, package,
  and checksum guidance under `docs/src/development/` and navigation.
- [x] T026 Update release and architecture documentation for the immutable
  catalog and typed runtime seam.
- [x] T027 Update Plan 038, plan index, migration ledger, feature metadata, and
  issue #114 Project stage.
- [x] T028 Add S070 Added and dated Decisions entries to `CHANGELOG.md`, including
  the pinned packaging and release workflow changes.

## Phase 8: Analysis and verification

- [x] T029 Complete the catalog integrity checklist and post-implementation
  analyze gate with requirement-to-test traceability.
- [x] T030 Run fmt, clippy, locked full tests, documentation policy, mdBook,
  linkcheck, spelling, package-policy, UTF-8, LF, forbidden-dash, and mojibake
  checks.
- [x] T031 Inspect the final diff for generated-artifact reproducibility,
  prohibited content, unrelated edits, and preserved user-owned files.

## Phase 9: Delivery

- [x] T032 Commit as `feat(070): build deterministic SQLite catalogs` with the
  required co-author trailer.
- [x] T033 Push the authorized branch, open the official PR with `Closes #114`,
  and move the issue to PR review.
- [x] T034 Resolve every CI and external review finding, optionally request only
  the one authorized second `@Codex` round, and stop when all checks and reviews
  are satisfied.

## Dependencies and Execution Order

Specification and the analyze gate precede all implementation. Failing compiler
and runtime tests precede dependency and source changes. Model validation and
schema verification precede insertion and publication. The runtime seam consumes
only verified catalogs. Package work consumes the generated baseline. Project
records and final analysis follow the completed implementation. Remote delivery
starts only after all local gates pass.
