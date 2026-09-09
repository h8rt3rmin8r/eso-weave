# Tasks: User-Initiated Catalog Updates

**Input**: Design documents from `/specs/074-user-catalog-updates/`

**Tests**: Required by the constitution and issue #118. Each implementation
phase begins with a failing focused test.

## Phase 1: Contract Setup

- [x] T001 Freeze `selection.schema.json` and `update-receipt.schema.json`.
- [x] T002 Freeze availability, stage, progress, target, summary, and receipt Rust types.
- [x] T003 Add the `catalog_update` module skeleton and public exports.
- [x] T004 [P] Add canonical JSON contract fixtures and parsing tests.

## Phase 2: Foundational S073 Inspection and Roots

- [x] T005 Add failing tests for public redacted candidate inspection.
- [x] T006 Expose S073 candidate metadata only after complete verification.
- [x] T007 Add failing root layout, link/reparse, and alias rejection tests.
- [x] T008 Implement user-data catalog roots and strict directory validation.
- [x] T009 Add failing selection default, canonical parse, and invalid fallback tests.
- [x] T010 Implement bounded no-follow selection loading and target resolution.
- [x] T011 Add failing cross-process lock and concurrent-operation tests.
- [x] T012 Implement standard-library exclusive file-lock ownership.

## Phase 3: User Story 1 - Unified Availability

- [x] T013 [US1] Add the complete Live status and independent PTS decision matrix tests.
- [x] T014 [US1] Implement one pure availability resolver in `availability.rs`.
- [x] T015 [US1] Add explicit fresh/offline evidence to the bounded API check result.
- [x] T016 [US1] Add background import discovery and candidate summary ordering.
- [x] T017 [US1] Switch startup catalog resolution to verified user selection before bundled fallback.
- [x] T018 [US1] Integrate background evidence into the App Model without GUI-thread I/O.

**Checkpoint**: Startup stays non-blocking and all seven Live states plus PTS are truthful.

## Phase 4: User Story 2 - Safe Reviewed Install

- [x] T019 [US2] Add failing install success, PTS rejection, downgrade, and reuse tests.
- [x] T020 [US2] Add failing checksum, schema, policy, extra-file, mutation, and first-open tests.
- [x] T021 [US2] Add failing disk/copy/publish failure tests preserving the active selection.
- [x] T022 [US2] Implement bounded candidate copying into same-filesystem staging.
- [x] T023 [US2] Implement installed candidate verification and immutable no-clobber publication.
- [x] T024 [US2] Implement verified first open and atomic selection replacement.
- [x] T025 [US2] Refresh the App Model catalog handle only after terminal install success.

**Checkpoint**: A reviewed Live candidate survives restart; every injected failure preserves the old target.

## Phase 5: User Story 3 - Progress, Cancellation, and Shutdown

- [x] T026 [US3] Add failing stage ordering and honest determinate/indeterminate progress tests.
- [x] T027 [US3] Add failing cancellation tests before copy, during copy, before open, and at commit.
- [x] T028 [US3] Implement the worker command/event channel and monotonic progress model.
- [x] T029 [US3] Implement cooperative cancellation and the non-interruptible commit boundary.
- [x] T030 [US3] Implement shutdown join/serialization for active update work.
- [x] T031 [US3] Add interrupted staging and startup recovery tests.
- [x] T032 [US3] Implement bounded recovery without deleting accepted targets.

**Checkpoint**: The UI thread never blocks and terminal operation state is unambiguous.

## Phase 6: User Story 4 - Collector-Assisted Build

- [x] T033 [US4] Add failing collector disclosure, lifecycle, and PixelBeacon non-mutation tests.
- [x] T034 [US4] Add failing unchanged, unstable, malformed, oversized, and wrong-channel capture tests.
- [x] T035 [US4] Implement private capture fingerprinting after entering the waiting stage.
- [x] T036 [US4] Generate a local-only S073 request from a later stable complete S071 envelope.
- [x] T037 [US4] Use the active catalog as a zero-removal baseline and retain S072 placeholders.
- [x] T038 [US4] Route the resulting candidate through the reviewed install path.
- [x] T039 [US4] Expose capture deletion and managed collector uninstall cleanup actions.

**Checkpoint**: Collector-assisted data never bypasses S071 or S073 and cannot reduce accepted coverage silently.

## Phase 7: User Story 5 - Rollback and Receipts

- [x] T040 [US5] Add failing verified rollback, invalid previous target, and bundled rollback tests.
- [x] T041 [US5] Implement rollback through the same exclusive selection boundary.
- [x] T042 [US5] Add failing canonical receipt, bounds, and privacy tests for every result.
- [x] T043 [US5] Implement complete, cancelled, failed, interrupted, and rollback receipts.
- [x] T044 [US5] Add support-summary projection without path or content disclosure.

**Checkpoint**: Rollback and diagnosis need no manual filesystem repair.

## Phase 8: Accessible Application UI

- [x] T045 Add failing App View notice, modal, candidate, progress, and terminal projection tests.
- [x] T046 Add failing headless AccessKit tests for open, focus, Escape, cancel, close, and restoration.
- [x] T047 Add failing narrow/normal responsive sizing and reduced-motion tests.
- [x] T048 Add catalog update strings and UI intents.
- [x] T049 Render the dismissible notice and Catalog Update menu action.
- [x] T050 Render Live/PTS summaries, source selection, collector disclosure, and update confirmation.
- [x] T051 Render textual progress, determinate/indeterminate bar, elapsed time, cancel, rollback, and cleanup.
- [x] T052 Poll worker events per frame and request repaint without blocking.

## Phase 9: Documentation and Quality

- [x] T053 [P] Update architecture, candidate pipeline, catalog runtime, status, troubleshooting, and test strategy docs.
- [x] T054 [P] Add user guidance for import roots, collector save boundaries, rollback, cleanup, and diagnostics.
- [x] T055 Add the `[Unreleased]` feature and dated architecture decision to `CHANGELOG.md`.
- [x] T056 Run spec-kit analysis and resolve every finding.
- [x] T057 Run focused catalog update, catalog runtime, App Model, and UI tests.
- [x] T058 Run format, strict Clippy, full locked tests, and optimized binary builds.
- [x] T059 Run mdBook test/build, generated-site policy, typo, JSON, UTF-8, BOM, dash, and mojibake checks.
- [x] T060 Verify the final diff contains no unrelated user files or sensitive fixture values.
- [x] T061 Commit, push, open the closing PR for #118, and move the delivery item to PR review.
- [ ] T062 Address every CI and review finding with no more than the authorized second Codex round.
- [ ] T063 Stop for the operator's final review and merge ritual.

## Dependencies and Execution Order

- Phases 1 and 2 establish contracts required by every user story.
- Availability precedes install because compatibility and channel policy gate every action.
- Install precedes the worker because progress wraps correctness-bearing operations.
- Collector-assisted work reuses the complete reviewed install path.
- Rollback and receipts reuse the selection boundary.
- UI begins after the model contracts stabilize, then documentation and full gates finish the slice.

## Implementation Strategy

Deliver the full issue as one coherent slice. User Story 1 is independently
testable, but publication requires all five stories because activation without
rollback, progress, collector boundaries, and accessible control would not meet
issue #118's safety contract.
