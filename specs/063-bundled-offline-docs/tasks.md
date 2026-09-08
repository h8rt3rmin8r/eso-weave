# Tasks: Bundled Offline Documentation

## Phase 1: Spec-kit and Evidence

- [x] T001 Synchronize `main`, create the S063 branch, and select issue #82.
- [x] T002 Explore mdBook output, workflows, UI ownership, platform launchers, tests, and lifecycle.
- [x] T003 Create the specification and requirements checklist.
- [x] T004 Resolve generation, embedding, loopback, path, CSP, browser, and shutdown policies.
- [x] T005 Create security and delivery checklists.
- [x] T006 Create research, data model, contracts, quickstart, and plan.
- [x] T007 Run and pass pre-implementation analysis.

## Phase 2: Build and Embed

- [x] T008 [US3] Add failing tests for manifest ordering, paths, media types, index, and reference completeness.
- [x] T009 [US3] Add the checked fixture and build-support manifest generator.
- [x] T010 [US3] Make release builds verify tools, generate the book, and fail on incomplete output.
- [x] T011 [US3] Emit and include immutable path, media type, and byte entries without a new crate.
- [x] T012 [US3] Prove generated HTML stays ignored and record size evidence.

## Phase 3: Secure Loopback Service

- [x] T013 [US2] Add failing tests for routing, GET, HEAD, content type, 404, 405, and headers.
- [x] T014 [US2] Add adversarial tests for encoding, slashes, dots, malformed targets, limits, stalls, and isolation.
- [x] T015 [US2] Implement exact lookup and mount routing.
- [x] T016 [US2] Implement the bounded loopback request worker and response policy.
- [x] T017 [US2] Add lifecycle tests for reuse, concurrent reads, listener failure, and shutdown.
- [x] T018 [US2] Implement one reusable service with explicit shutdown and join.

## Phase 4: Application Access

- [x] T019 [US1] Add browser-seam and headless UI tests for Help > Documentation, reuse, and failure.
- [x] T020 [US1] Implement native Windows and Linux openers with hidden, non-interactive behavior.
- [x] T021 [US1] Add stable menu strings and Help > Documentation.
- [x] T022 [US1] Surface start and launch errors without blocking or exiting.
- [x] T023 [US1] Complete offline navigation, search, theme, font, image, deep-link, and repeat-open checks.

## Phase 5: CI, Release, Documentation, and Lifecycle

- [x] T024 Install exact tools and exercise release embedding on Windows and Linux CI.
- [x] T025 Add the same prerequisites to Windows and Linux release package jobs.
- [x] T026 Update operator access, architecture, test strategy, release packaging, and release guidance.
- [x] T027 Archive plan 032 with PR #100 evidence and establish active plan 033.
- [x] T028 Update the migration ledger, indexes, and feature lifecycle state while preserving the intentionally frozen S059 coverage projection.
- [x] T029 Add S063 changes and dated pinned workflow and release-guidance decisions to the changelog.

## Phase 6: Analyze, Validate, and Review

- [x] T030 Complete security and delivery checklists against implementation evidence.
- [x] T031 Run and pass post-implementation analysis.
- [x] T032 Run format, Clippy, tests, release builds, docs, spelling, hygiene, whitespace, and mojibake gates.
- [x] T033 Complete security, lifecycle, UI, build, test, docs, and spec-kit reviews and resolve findings.

## Phase 7: Delivery

- [x] T034 Commit as `feat(063): bundle offline documentation` with attribution.
- [x] T035 Push and open an official PR with `Closes #82`.
- [x] T036 Move issue #82 to PR Review and monitor every CI check and review.
- [ ] T037 Resolve every first-round hosted review finding.
- [ ] T038 Trigger exactly one authorized second `@Codex review` and resolve every result.
- [ ] T039 Confirm green checks, no unresolved review threads, and merge readiness before requesting the merge ritual.

## Dependencies

- T007 blocks implementation.
- T008 precedes T009 through T012.
- T013, T014, and T017 precede T015, T016, and T018.
- T019 precedes T020 through T022.
- T024 and T025 require stable release behavior.
- T026 through T029 require stable implementation.
- T030 through T033 block publication.
- T035 and one second review are pre-authorized.
