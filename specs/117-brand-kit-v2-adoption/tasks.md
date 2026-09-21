# Tasks: BrandBuilder 2.0 Kit Adoption

## Phase 1: Specification and Analysis

- [x] T001 Create issue #233, branch `codex/s117-brand-kit-v2-adoption`, and the S117 workspace
- [x] T002 Download and hash the official kit, inspect authority, migration, recovery, typography, egui, and platform contracts
- [x] T003 Resolve full-adoption scope, precedence conflict, target sizing, domain colors, applicable platforms, and recovery retention
- [x] T004 Produce specification, clarification record, quality and conformance checklists, research, data model, contract, plan, and quickstart
- [x] T005 Pass the pre-implementation analysis gate with no critical conflict

## Phase 2: Test-First Failure

- [x] T006 [US4] Add Node policy tests for malformed metadata, wrong tokens, and artifact hash drift in `.github/scripts/brand-kit-policy.test.mjs`
- [x] T007 [US1] Add exact semantic role and contrast tests in `src/app/theme.rs`
- [x] T008 [US1] Add 44-point rendered interaction allocation tests in `tests/app_ui_sizing.rs`
- [x] T009 [US2] Add approved font family, weight, fallback, and hash expectations
- [x] T010 Run focused tests and record the expected pre-implementation failures

## Phase 3: Runtime and Typography

- [x] T011 [US1] Replace legacy general palette fields with semantic roles in `src/app/theme.rs`
- [x] T012 [US1] Apply governed dark/light surfaces, actions, state, border, focus, spacing, and radii in `src/app/theme.rs`
- [x] T013 [US1] Enforce the 44-point response target and repair constrained layouts in affected app widgets
- [x] T014 [US2] Add and register `assets/brand/fonts/GeistMono-Regular.ttf`
- [x] T015 [US2] Apply the named mono family to representative identifier, timestamp, path, and technical metadata seams
- [x] T016 Re-run focused Rust tests to green and refactor semantic helper duplication

## Phase 4: Assets, Recovery, and Records

- [x] T017 [US4] Add `assets/brand/brand-kit-adoption.json` with exact package, versions, authority, migration, deviation, token, and artifact records
- [x] T018 [US4] Retain exact recovery bytes at `assets/brand/recovery/shruggie-brandbuilder-2.0.0.skill`
- [x] T019 [US3] Replace `assets/icon.ico` with the official generated classic Win32 ICO
- [x] T020 [US3] Record already-current Linux, installer, window, documentation, reference, font, and SVG asset dispositions
- [x] T021 [US4] Implement `.github/scripts/brand-kit-policy.mjs` and pass its negative and positive tests

## Phase 5: Documentation and Governance

- [x] T022 [US3] Rewrite `assets/brand/README.md` for exact provenance, derivation, and single-ink rules
- [x] T023 [US3] Rewrite `docs/src/development/brand-standard.md` for BrandBuilder 2.0 implementation guidance
- [x] T024 [US4] Record the dated pinned ICO, font, recovery, and governance decisions in `[Unreleased]` in `CHANGELOG.md`
- [x] T025 Complete the local brand conformance checklist and post-implementation analysis

## Phase 6: Verification and Local Delivery

- [x] T026 Run focused policy, theme, and UI sizing tests
- [x] T027 Run fmt, clippy, complete locked tests, and locked release build
- [x] T028 Run documentation policy, render smoke, mdBook test/build, and any repository docs validator
- [x] T029 Run diff, encoding, LF, BOM, mojibake, forbidden-dash, and untracked-file checks
- [x] T030 Review the complete diff for authority, accessibility, platform scope, safety boundaries, and unrelated changes
- [x] T031 Commit with S117 and issue linkage

## Phase 7: Publication and Hosted Review

- [ ] T032 Push the authorized feature branch and open the official pull request with `Closes #233`
- [ ] T033 Attach the pull request to the current task
- [ ] T034 Wait for all CI checks and third-party reviews, including body reactions when no textual review arrives
- [ ] T035 Address every actionable review comment, reply with evidence, and resolve each thread
- [ ] T036 If the first Codex round contains feedback, request one second round using `@Codex`, then address and resolve it
- [ ] T037 Confirm all required checks are green and no review thread remains unresolved
- [ ] T038 Stop for the operator's final review and merge ritual without merging

## Dependencies and Execution Order

- Phase 1 blocks all implementation.
- Phase 2 establishes the required red tests before production changes.
- Runtime and typography can proceed after red evidence and precede record validation.
- Asset and record work supplies the positive validator fixture.
- Documentation must describe final behavior and exact committed hashes.
- Publication begins only after every local gate passes.
- A second Codex round is conditional and may occur only once.
