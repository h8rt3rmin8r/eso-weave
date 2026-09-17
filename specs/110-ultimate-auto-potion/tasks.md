# Tasks: Ultimate Auto Potion Resource Watch

**Input**: S110 specification and plan packet

**Tests**: Exact-ratio, migration, mixed-watch, routing, settings, diagnostic, and documentation tests are mandatory and precede production changes.

## Phase 1: Specification and Planning

- [x] T001 Create `codex/s110-ultimate-auto-potion` and initialize spec-kit feature 110
- [x] T002 Complete specification, autopilot clarification, requirements checklist, and Auto Potion safety checklist
- [x] T003 Complete plan, research, data model, contract, quickstart, tasks, and pre-implementation analysis
- [x] T004 Archive completed Plan 045 with PR #218 evidence and establish Plan 046 with S110 as its active slice

## Phase 2: Test-First Trigger and Safety Contract

- [x] T005 [US1] Add failing Ultimate-only tests for equality, below, above, non-divisible ratios, threshold 0, threshold 100, and above-maximum current in `tests/potion.rs`
- [x] T006 [US3] Add failing mixed four-watch and deterministic first-cause tests in `tests/potion.rs`
- [x] T007 [US2] Add failing unknown-current, unknown-maximum, zero-maximum, signal-loss, and non-active-world tests with zero emitted attempts

## Phase 3: Test-First Configuration and Presentation Contract

- [x] T008 [US3] Add failing legacy-load, invalid-Ultimate-threshold notice, explicit-store, and full round-trip tests in `tests/potion.rs`
- [x] T009 [US1] Add failing structured diagnostic and exact observed-percentage assertions for Ultimate
- [x] T010 [US3] Add failing Ultimate status text and controller settings-refresh assertions in `tests/app_view_model.rs`
- [x] T011 [US2] Add failing application routing evidence that existing Ultimate state reaches Auto Potion without retained presentation
- [x] T012 [US3] Add failing settings label, help, keyboard-control inventory, and modal sizing assertions in `tests/app_strings.rs` and `tests/app_ui_sizing.rs`
- [x] T013 Run focused tests and record expected failures before production edits

## Phase 4: Core Implementation

- [x] T014 [US3] Add the defaulted Ultimate `ResourceWatch` to `AutoPotionConfig`, raw loading, validation notices, and current serialization
- [x] T015 [US1] Add Ultimate to `AutoPotionResource`, typed diagnostic names, and user-visible trigger projection
- [x] T016 [US1] Extend `PotionReadings` and the pure low-resource rule with exact current-over-maximum comparison, ceiling diagnostics, and deterministic fourth position
- [x] T017 [US2] Wire the existing `WeaveEngine::ultimate()` observation into the Auto Potion worker tick without a second cache, thread, or timer
- [x] T018 Run focused controller, configuration, routing, and view-model tests to green

## Phase 5: Settings and Documentation

- [x] T019 [US3] Add Watch Ultimate to the established settings row loop with bounded threshold input and explanatory help
- [x] T020 [US3] Update Auto Potion feature, settings reference, state-machine, test-strategy, and coverage documentation for four watches and exact Ultimate normalization
- [x] T021 Record the S110 feature and any inherited design decision in the `[Unreleased]` changelog
- [x] T022 Perform the quickstart settings layout and accessibility inspection to the extent supported by repository harnesses

## Phase 6: Analysis and Full Verification

- [x] T023 Re-run spec-kit analysis after implementation and resolve every finding
- [x] T024 Run `cargo fmt --all -- --check`
- [x] T025 Run strict Clippy for all targets and features
- [x] T026 Run the full locked Rust test suite
- [x] T027 Run documentation policy, mdBook, link, and repository trust gates applicable to the diff
- [x] T028 Verify UTF-8 without BOM, LF-only text, no forbidden dashes, no mojibake, and a clean scoped diff
- [x] T029 Mark the S110 specification implemented and every completed local task checked

## Phase 7: Publication and Hosted Review

- [x] T030 Commit the verified implementation as `feat(110): add Ultimate auto-potion watch` with attribution and push the feature branch
- [x] T031 Open the official pull request with `Closes #173`, scope, verification, and explicit exclusions
- [ ] T032 Wait for hosted CI, Codex, and security results and answer every review comment
- [ ] T033 Apply and verify required review changes, resolve every completed review thread, and push updates
- [ ] T034 Trigger at most one authorized second `@Codex` review and process it completely
- [ ] T035 Confirm required checks are green, reviews are satisfied, the branch is clean, and the pull request is mergeable
- [ ] T036 Stop for the operator's final review and merge ritual

## Dependencies and Execution Order

- Phase 1 completes the full spec-kit chain before production implementation.
- Phases 2 and 3 establish red evidence before their corresponding production edits.
- Phase 4 implements the pure rule before worker wiring.
- Phase 5 completes operator controls and canonical documentation after runtime behavior is green.
- Phase 6 is the local merge gate for publication.
- Phase 7 may trigger only the automatic opening review and one explicitly authorized second Codex round.

## Parallel Opportunities

- Trigger tests and configuration tests touch separate sections of `tests/potion.rs` but should be applied chronologically to preserve a readable red-green record.
- Settings strings and modal sizing assertions can be developed independently after the configuration field exists.
- Documentation pages are independent after terminology and exact math are fixed.
