# Tasks: ESO Catalog Source Contract

**Input**: Design documents from `specs/068-eso-catalog-contract/`

## Phase 1: Specification and governance

- [x] T001 Create the complete S068 spec-kit artifact chain and pre-implementation analysis.
- [x] T002 Establish active Plan 038 and update the plan index and migration ledger.
- [x] T003 Move issue #112 to In progress and assign Slice S068 in the Delivery Project.
- [x] T004 Create a separate verification issue for game-dependent live/PTS experiments.

## Phase 2: Failing contract tests

- [x] T005 Add failing documentation-policy fixtures for required catalog categories.
- [x] T006 Add failing rules for immutable evidence, stable keys, completeness, and channel promotion.
- [x] T007 Add failing rights and collector rules for icon bytes, placeholders, and non-executing imports.

## Phase 3: Source and coverage contract

- [x] T008 Add pinned live and PTS source snapshots with verified documentation hashes.
- [x] T009 Add the complete category matrix with stable IDs, discovery, visibility, completeness, failure, and validation fields.
- [x] T010 Add community-source, archive, remote-source, and license-scope decisions.
- [x] T011 Add explicit promotion, SavedVariables, privacy, corruption, and offline-fallback policies.

## Phase 4: Canonical documentation and handoff

- [x] T012 Publish the supported source hierarchy and category matrix in canonical documentation.
- [x] T013 Document placeholder-first graphics and user-local resolver boundaries.
- [x] T014 Document compiler, collector, and icon implementation handoffs.
- [x] T015 Update navigation, external sources, and generated search discovery while preserving the frozen S059 content-coverage projection.
- [x] T016 Add S068 Added and dated Decisions entries to `CHANGELOG.md`.

## Phase 5: Analysis and validation

- [x] T017 Complete the source governance checklist and post-implementation analyze gate.
- [x] T018 Run documentation fixture, production policy, mdBook, link, spelling, UTF-8, punctuation, and mojibake checks.
- [x] T019 Confirm no Rust source changed and record the constitution CI-parity exemption.

## Phase 6: Delivery

- [ ] T020 Commit with a `feat(068)` message and Co-Authored-By trailer.
- [ ] T021 Push the authorized branch, open the official PR with `Closes #112`, and update Project stage.
- [ ] T022 Resolve every CI and external review finding, with at most one authorized second `@Codex` round.

## Dependencies and Execution Order

Specification and governance precede tests. Contract tests fail before the
matrix is added. The canonical guide derives from the validated matrix. Analysis
and documentation gates precede commit and publication. Field verification stays
separate and cannot upgrade S068 completeness claims.
