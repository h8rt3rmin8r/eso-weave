# Tasks: Evidence-Scoped Encounter Recommendations

**Input**: Design documents from `specs/090-evidence-scoped-recommendations/`

**Tests**: Required by the constitution and S090 specification. Every behavior
change follows red, green, refactor.

## Phase 1: Setup and specification

- [x] T001 Create branch `codex/s090-evidence-scoped-recommendations` and the
  numbered S090 spec-kit workspace without touching the unrelated untracked draft.
- [x] T002 Complete specification and requirement-quality review against issue
  #136, S077, S078, and canonical encounter documentation.
- [x] T003 Complete autopilot clarification with local deterministic generation,
  provisional thresholds, rule-local unknown-ID handling, and no persistence.
- [x] T004 Complete the recommendation-safety checklist.
- [x] T005 Complete implementation research, data model, report contract,
  quickstart, constitution checks, and agent context update.

## Phase 2: Foundational policy contract

**Goal**: Establish the pure recommendation model and exact evidence gates before
any UI integration.

- [x] T006 [P] Add failing report version, provenance, equality, stable ordering,
  bounded output, and neutral wording tests in
  `tests/encounter_recommendations.rs`. [FR-002 through FR-008, FR-020, SC-001]
- [x] T007 [P] Add failing 9,999/10,000 ms and two/three cast boundary tests.
  [FR-009, FR-010, SC-002]
- [x] T008 [P] Add failing exact loss qualification, 10 percent suppression,
  overlap/edge, reversed-span, and overflow-safe tests. [FR-011 through FR-013]
- [x] T009 [P] Add failing unknown-ID qualification, target omission, unknown
  damage-share 25 percent boundary, and unrelated known-effect survival tests.
  [FR-014 through FR-017]
- [x] T010 [P] Add failing dominant-share, low-uptime, tie-break, unavailable,
  non-finite, invalid-ratio, and no-candidate tests. [FR-018 through FR-021]
- [x] T011 Implement `src/recommendation/mod.rs` report, evidence, reason, advice,
  citation, and enum types with stable public constants. [FR-001, FR-002, FR-003,
  FR-004, FR-005, FR-006, FR-007]
- [x] T012 Implement deterministic global sample and loss gates using widened
  integer union arithmetic. [FR-009, FR-010, FR-011, FR-012, FR-013]
- [x] T013 Implement catalog uncertainty facts and rule-local unknown-target and
  unknown-damage handling. [FR-014, FR-015, FR-016, FR-017]
- [x] T014 Implement the two bounded selection rules and invalid-value filtering.
  [FR-018, FR-019, FR-020, FR-021, FR-022, FR-023]
- [x] T015 Export the recommendation domain from `src/lib.rs` and verify the focused
  pure suite passes without changing Encounter Metrics. [FR-001, FR-024, FR-025]

**Checkpoint**: Equal projections produce equal, bounded, fully cited reports;
suppressed reports contain no advice.

## Phase 3: User Story 1 and 2, app composition and presentation

**Goal**: Present provisional advice after observed facts with exact quality gates.

- [x] T016 Add failing worker tests proving one atomic projection/report detail,
  stable citations, and compatible catalog replacement rebuilds in
  `tests/app_encounter_history.rs`. [US1, US2, FR-004, FR-024]
- [x] T017 Add failing pure presentation-helper tests for fixed ready, qualified,
  suppressed, empty, rule, reason, and citation text. [FR-006 through FR-008,
  FR-022, FR-023]
- [x] T018 Add failing wide and narrow headless UI journeys proving Observed Metrics
  precede Provisional Recommendations and remain visible in every report state.
  [FR-026, FR-027, SC-003, SC-004]
- [x] T019 Add failing headless assertions that no recommendation apply, execute,
  or automation control or intent exists. [US3, FR-025, SC-005]
- [x] T020 Add app-owned `EncounterDetail` and make the history worker generate one
  report from the exact projection it returns. [FR-001, FR-024]
- [x] T021 Add typed presentation helpers in `src/app/encounter_history.rs` with
  fixed provisional, qualification, suppression, empty, and citation copy.
  [FR-005 through FR-008, FR-022, FR-023]
- [x] T022 Render the separate bounded recommendation section in `src/app/ui.rs`
  after observed facts and before provenance, with no action controls. [FR-026]
- [x] T023 Verify catalog replacement and window lifecycle discard and rebuild the
  combined detail without persistence or stale version mixing. [US3, FR-024]

**Checkpoint**: The selected encounter shows truthful provisional advice or exact
gates while its observed facts remain unchanged and independently visible.

## Phase 4: Documentation and governance

- [x] T024 Update `docs/project/encounter-model.json` with implemented S090 policy,
  versions, thresholds, rule-local unknown semantics, and display-only isolation.
  [FR-028]
- [x] T025 Update `docs/src/reference/encounter-data-and-metrics.md` with generation,
  threshold, qualification, citation, wording, non-persistence, and nonblocking
  verification behavior. [FR-028, FR-030]
- [x] T026 Update `docs/src/development/architecture.md` with Recommendation
  subsystem ownership and the projection-to-report data flow. [FR-024, FR-025]
- [x] T027 Add or extend documentation and repository policy tests for S090 machine
  authority, prohibited wording/dependencies, versions, and visible local-only
  boundaries. [FR-002, FR-008, FR-025, FR-028]
- [x] T028 Move Plan 039 to the archive, record S089/PR #161 and epic #119 closure,
  and add its archive index disposition. [FR-029]
- [x] T029 Create active Plan 040 for S090 and update the active build-plan index in
  chronological order. [FR-029]
- [x] T030 Update `CHANGELOG.md` `[Unreleased]` with S090 behavior and dated
  architecture decisions.
- [x] T031 Run UTF-8 without BOM, mojibake, whitespace, forbidden-dash, JSON,
  spelling, documentation build, link, and policy checks. [SC-006]

## Phase 5: Analyze and quality review

- [x] T032 Run the `/speckit.analyze` consistency gate across spec, checklists,
  plan, research, data model, contract, quickstart, and tasks; resolve every
  finding before implementation completion.
- [x] T033 Review the diff for architecture, correctness, security/privacy,
  accessibility, regression, and scope issues with confidence filtering; fix all
  high-confidence findings.
- [x] T034 Confirm static dependency inspection finds no input, automation, addon,
  Pixel Bus, network, telemetry, persistence, catalog-mutation, or logging
  dependency in recommendation code. [FR-025, SC-005]
- [x] T035 Confirm issue #131 remains separate Release verification and no S090
  requirement or task treats it as a blocker. [FR-030]

## Phase 6: Full validation and publication

- [x] T036 Run `cargo fmt --all -- --check` in the foreground.
- [x] T037 Run `cargo clippy --all-targets --all-features -- -D warnings` in the
  foreground.
- [x] T038 Run `cargo test --all --locked` in the foreground.
- [x] T039 Complete final diff, status, UTF-8, BOM, mojibake, forbidden-dash, and
  scope review while preserving the unrelated untracked draft.
- [x] T040 Commit as `feat(090): add evidence-scoped encounter recommendations`
  with issue linkage and co-author attribution.
- [x] T041 Set issue #136 to Status In Progress, Stage PR review, and Slice S090 in
  the delivery Project.
- [ ] T042 Push the authorized short-lived branch and publish an official pull
  request to `main` closing issue #136.
- [ ] T043 Wait for CI and the first external review round, respond to every
  comment, push verified fixes, and resolve every completed thread.
- [ ] T044 Trigger at most one authorized second `@Codex review` round and address
  every response without requesting a third round.
- [ ] T045 When all checks are green and all review threads are resolved, request
  the operator's final review and merge ritual without merging the PR.

## Dependencies and Execution Order

- T006 through T010 may be authored together before T011.
- T011 through T015 satisfy pure policy tests before app work begins.
- T016 through T019 fail before T020 through T023 implement their behavior.
- T024 through T030 follow stable implementation semantics.
- T032 is a blocking pre-completion gate and cannot be weakened or skipped.
- T036 through T038 are mandatory before T040.
- T042 is already authorized by the user's S090 kickoff.
- T044 is the only permitted additional automated review request.
