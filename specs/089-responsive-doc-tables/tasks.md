# Tasks: Responsive Documentation Tables

## Phase 1: Setup and specification

- [x] T001 Create the S089 feature workspace and switch to `codex/s089-responsive-doc-tables`.
- [x] T002 Audit all Markdown tables, named issue pages, generated mdBook wrappers, theme behavior, print, and offline delivery in `docs/src/` and `target/docs-site/html/`.
- [x] T003 Complete `specs/089-responsive-doc-tables/spec.md`, requirements checklist, accessibility checklist, research, data model, contract, plan, and quickstart.
- [x] T004 Run the pre-implementation analysis gate and record its coverage in `specs/089-responsive-doc-tables/analysis.md`.

## Phase 2: Foundational test-first contracts

- [x] T005 [P] Add failing-first S089 inventory, JavaScript, CSS, print, and generated-structure tests in `.github/scripts/docs-policy.test.mjs`.
- [x] T006 [P] Add failing-first S089 observation, receipt, keyboard, resize, zoom, print, and blocked-script tests in `.github/scripts/docs-render-smoke.test.mjs`.
- [x] T007 Run the focused Node suite and record the expected red state in `specs/089-responsive-doc-tables/analysis.md`.

## Phase 3: User Story 1, readable contained tables

**Goal**: Preserve useful column widths and keep every compact or dense table readable without page-level horizontal overflow.

**Independent test**: Observe compact and dense tables at 320 and 1280 CSS pixels in navy and light themes and prove readable cells plus local containment.

- [x] T008 [US1] Add table classification, width-tier, natural wrapping, cell alignment, and page-containment rules in `docs/theme/eso-weave.css`.
- [x] T009 [US1] Add idempotent table discovery, classification, shell, and initial geometry state in `docs/theme/eso-weave.js`.
- [x] T010 [US1] Add named-page table observations and readable geometry validation in `.github/scripts/docs-render-smoke.mjs`.

## Phase 4: User Story 2, keyboard and assistive access

**Goal**: Make only genuinely overflowing table boundaries discoverable, named, focusable, and natively keyboard scrollable without changing table semantics.

**Independent test**: Focus an overflowing wrapper, verify its visible instruction and accessible relationships, dispatch trusted ArrowRight input, and prove table scroll changes while page position does not.

- [x] T011 [US2] Implement unique contextual region names, one associated visible instruction, conditional focusability, and fit-state cleanup in `docs/theme/eso-weave.js`.
- [x] T012 [US2] Add visible instruction, focus, overflow-position, scrollbar, theme, and non-color cue rules in `docs/theme/eso-weave.css`.
- [x] T013 [US2] Add trusted keyboard scrolling, focus visibility, redundant-focus rejection, and semantic table evidence in `.github/scripts/docs-render-smoke.mjs`.

## Phase 5: User Story 3, adaptive and fallback states

**Goal**: Keep table state truthful through resize, 200 percent page zoom, print, and blocked scripts.

**Independent test**: Cross an overflow threshold by resizing, validate true page scale 2, emulate print, and block scripts while proving containment and semantics in every state.

- [x] T014 [US3] Add one batched ResizeObserver, font-readiness refresh, window-resize refresh, scroll-position tracking, and obsolete-offset reset in `docs/theme/eso-weave.js`.
- [x] T015 [US3] Add print reset and script-free local-containment rules in `docs/theme/eso-weave.css`.
- [x] T016 [US3] Add resize, true zoom, print, and blocked-script browser journeys in `.github/scripts/docs-render-smoke.mjs`.

## Phase 6: User Story 4, complete maintenance contract

**Goal**: Freeze the 54-table source inventory and reject semantic, responsive, accessibility, generated-output, or named-page evidence drift.

**Independent test**: Mutate inventory, script, style, generated markup, and browser receipts and verify each missing contract fails with an actionable message.

- [x] T017 [US4] Add the exact 26-page and 54-table inventory plus five named-page counts to `.github/scripts/docs-policy.mjs`.
- [x] T018 [US4] Add table JavaScript, CSS, print, generated semantics, and integration validation to `.github/scripts/docs-policy.mjs`.
- [x] T019 [US4] Complete policy mutation coverage and repository integration tests in `.github/scripts/docs-policy.test.mjs`.
- [x] T020 [US4] Complete the 20-cell table receipt, compact state, special journeys, failure aggregation, and S089 sentinel in `.github/scripts/docs-render-smoke.mjs`.
- [x] T021 [US4] Complete receipt mutation coverage in `.github/scripts/docs-render-smoke.test.mjs`.
- [x] T022 [US4] Publish inventory, classification, accessibility, maintenance, and verification guidance in `docs/project/documentation-table-system.md`.

## Phase 7: Cross-cutting validation and governance

- [x] T023 Update S088 and S089 chronology in `docs/project/build-plans/plan-039.md` and `docs/project/build-plans/README.md`.
- [x] T024 Update `CHANGELOG.md` with the S089 outcome and dated responsive-table architecture decision.
- [x] T025 Re-run the spec-kit analysis gate, resolve all findings, and mark the spec plus both requirements-quality checklists implemented.
- [x] T026 Perform correctness, security, accessibility, documentation, regression, and scope review with findings filtered at 80 percent confidence.
- [x] T027 Run focused Node, mdBook, policy, generated-browser, spelling, UTF-8, LF, mojibake, forbidden-dash, `git diff --check`, and foreground Cargo parity gates.

## Phase 8: Delivery

- [x] T028 Commit S089 as `feat(089): complete responsive documentation tables` with issue linkage and co-author attribution.
- [x] T029 Push the authorized feature branch and open the official pull request closing #127.
- [x] T030 Set issue #127 to Status In Progress, Stage PR review, and Slice S089 in the delivery Project.
- [ ] T031 Address every first-round CI, Codex, security, and reviewer finding.
- [ ] T032 Request and address at most one authorized second Codex review round.
- [ ] T033 Confirm every review conversation is resolved and every required check is green.
- [ ] T034 Hand off to the maintainer for final review and merge.

## Dependencies

- Phase 1 precedes the Phase 2 red gate.
- Phase 2 precedes all implementation phases.
- User Story 1 establishes the shared shell and classification used by User Stories 2 and 3.
- User Story 2 establishes accessible overflow state used by User Story 3 transitions.
- User Story 4 integrates policy and browser evidence after the runtime behavior exists.
- Cross-cutting validation precedes delivery.

## Parallel opportunities

- T005 and T006 affect independent test files.
- After runtime behavior stabilizes, T019 and T021 affect independent mutation suites.
- T022 can proceed alongside final browser receipt work after the contract is stable.

## Implementation strategy

The MVP is User Story 1: readable local containment for compact and dense tables. User Story 2 adds conditional accessible interaction, User Story 3 proves changing and fallback states, and User Story 4 freezes the complete maintenance contract. All four stories ship together because issue #127 requires complete corpus evidence.
