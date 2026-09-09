# Tasks: External Encounter Model

**Input**: Design documents from `specs/069-encounter-model/`

## Phase 1: Specification and governance

- [x] T001 Create the complete S069 spec-kit design chain and pre-implementation analysis.
- [x] T002 Establish active Plan 038 and update the plan index and migration ledger.
- [x] T003 Move issue #113 to In progress and assign Slice S069 in the Delivery Project.
- [x] T004 Create a separate live Combat Metrics verification issue.
- [x] T005 Create ordered capture, import, calculation, UI, and recommendation issues.

## Phase 2: Failing contract and projection tests

- [x] T006 Add failing documentation-policy fixtures for required encounter authority fields and vocabularies.
- [x] T007 Add failing ordering, gap, duplicate, privacy, storage-plane, Pixel Bus, and automation-independence rules.
- [x] T008 Add failing deterministic metric, unknown-ID rejoin, and storage-receipt tests.

## Phase 3: Encounter authority and synthetic spike

- [x] T009 Add pinned source snapshots and feature-parity roadmap.
- [x] T010 Add capture envelope, event families, ordering, loss, privacy, integrity, and catalog-join policies.
- [x] T011 Add the complete synthetic encounter with declared loss and an unknown ID.
- [x] T012 Add deterministic observed DPS, HPS, ability share, effect uptime, and cast sequence projections.
- [x] T013 Add exact fixture storage receipts and clearly labeled linear estimates.

## Phase 4: Canonical documentation and handoff

- [x] T014 Publish the user-owned encounter storage, privacy, and retention contract.
- [x] T015 Publish the Combat Metrics parity matrix and synthetic spike interpretation.
- [x] T016 Document capture, import, calculation, UI, and recommendation handoffs.
- [x] T017 Update navigation, external sources, project ledgers, and Plan 038.
- [x] T018 Add S069 Added and dated Decisions entries to `CHANGELOG.md`.

## Phase 5: Analysis and validation

- [x] T019 Complete the encounter governance checklist and post-implementation analyze gate.
- [x] T020 Run fixture, production policy, mdBook, link, spelling, UTF-8, punctuation, and mojibake checks.
- [x] T021 Confirm no Rust source changed and record the constitution CI-parity exemption.

## Phase 6: Delivery

- [x] T022 Commit with a `feat(069)` message and Co-Authored-By trailer.
- [ ] T023 Push the authorized branch, open the official PR with `Closes #113`, and update Project stage.
- [ ] T024 Resolve every CI and external review finding, with at most one authorized second `@Codex` round.

## Dependencies and Execution Order

Specification and governance precede tests. Contract and projection tests fail
before the authority and fixture are added. Canonical documentation derives from
the validated authority. Live comparison remains separate and cannot upgrade a
synthetic parity claim. Downstream issues are ordered capture, import,
calculation, UI, then recommendations.
