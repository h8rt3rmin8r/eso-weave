# Tasks: Data Addon Lifecycle UI

**Input**: Issue #185 and the S093 design artifacts

## Phase 1: Governance and Specification

- [x] T001 Create the S093 spec-kit package and resolve routine clarifications
- [x] T002 Reconcile issue #185 ownership, Slice S093, and In progress stage
- [x] T003 Archive completed Plan 041 and create chronological active Plan 042
- [x] T004 Complete the read-only specification analysis gate in `analysis.md`

## Phase 2: Test-First Observation and Lifecycle

- [x] T005 Add failing projection matrix tests in `tests/app_view_model.rs`
- [x] T006 Add failing safe-action and reload-retention tests in `tests/app_view_model.rs`
- [x] T007 Add failing lifecycle neighbor, race, and remediation tests
- [x] T008 Add failing unconfirmed-evidence and inspection-bound tests
- [x] T009 Implement cached data-addon observation and pure view projection
- [x] T010 Add Install, Update, Repair, and Uninstall data-addon intents
- [x] T011 Preserve status on failure and retain reload outcomes

## Phase 3: First-Class Interface

- [x] T012 Add failing row-order and supported-width tests
- [x] T013 Add failing accessible-label and data-specific confirmation tests
- [x] T014 Render ESO Weave Data immediately beneath PixelBeacon
- [x] T015 Render enabled, loaded, reload, runtime, catalog, and encounter separately
- [x] T016 Share equivalent lifecycle vocabulary with PixelBeacon
- [x] T017 Remove duplicate Catalog Update lifecycle controls
- [x] T018 Add centralized strings and copy-policy tests

## Phase 4: Documentation and Screenshots

- [x] T019 Update interface, first-launch, status-reference, and troubleshooting guidance
- [x] T020 Add canonical data-addon feature guidance and navigation
- [x] T021 Update test strategy and Catalog Update guidance
- [x] T022 Update `[Unreleased]` Added and dated Decisions changelog entries
- [x] T023 Refresh deterministic screenshots and manifest metadata
- [x] T024 Verify UTF-8, spelling, forbidden punctuation, and mojibake hygiene

## Phase 5: Validation and Publication

- [x] T025 Run `cargo fmt --all -- --check`
- [x] T026 Run strict all-targets all-features locked Clippy
- [x] T027 Run the full locked Cargo test suite
- [x] T028 Run documentation policy, mdBook, link, and text gates
- [x] T029 Complete parallel local code, security, and UI-domain review
- [x] T030 Resolve every actionable local finding and repeat affected gates
- [ ] T031 Mark implemented, commit, push, and open the PR closing #185
- [ ] T032 Move issue #185 and PR to Slice S093 and PR review stage
- [ ] T033 Address every first-round external review and CI finding
- [ ] T034 Trigger exactly one authorized second `@Codex review` round
- [ ] T035 Address every second-round finding without a third trigger
- [ ] T036 Confirm CI green and request final review and merge ritual

## Execution Order

Specification precedes tests. Tests precede production changes. Observation and
lifecycle precede rendering. Documentation follows stable UI copy. Publication
follows every local gate; final handoff follows both external review rounds and
green CI.
