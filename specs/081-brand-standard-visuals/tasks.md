# Tasks: Brand Standard Visuals

**Input**: Design documents from `/specs/081-brand-standard-visuals/`
**Prerequisites**: `spec.md`, `plan.md`, `research.md`, `data-model.md`, `contracts/`

## Phase 1: Setup and authority capture

- [x] T001 Confirm issue #122, parent #119, and Plan 039 traceability in `spec.md`
- [x] T002 Record approved asset, accessibility, surface, and reproduction decisions in `research.md`
- [x] T003 Record all 25 palette tokens and three asset mappings in the design contract

## Phase 2: User Story 1 - Identify and use approved assets

**Goal**: Publish the three approved assets in clearly labeled valid contexts.

**Independent Test**: Source and generated policy accept the approved gallery and reject missing assets, altered bytes, or prohibited light-surface glyph use.

- [x] T004 [US1] Add failing asset identity, gallery structure, legacy-status, and invalid-surface tests in `.github/scripts/docs-policy.test.mjs`
- [x] T005 [US1] Add pure asset and Brand Standard source validation in `.github/scripts/docs-policy.mjs`
- [x] T006 [US1] Copy authoritative mark and glyph bytes into `docs/src/assets/brand/`
- [x] T007 [US1] Add the labeled approved-asset gallery and surface examples to `docs/src/development/brand-standard.md`
- [x] T008 [US1] Add scoped responsive gallery and surface styles to `docs/theme/eso-weave.css`

## Phase 3: User Story 2 - Compare palettes accessibly

**Goal**: Give all 25 palette rows a visible bounded chip and matching accessible name.

**Independent Test**: Exact token mutations, missing chips, wrong fills, and wrong labels fail policy while the complete source passes.

- [x] T009 [US2] Add failing palette completeness, fill, label, and boundary tests in `.github/scripts/docs-policy.test.mjs`
- [x] T010 [US2] Add exact palette-token and chip validation in `.github/scripts/docs-policy.mjs`
- [x] T011 [US2] Add labeled chips to every dark and light palette row in `docs/src/development/brand-standard.md`
- [x] T012 [US2] Add theme-independent bounded chip styles in `docs/theme/eso-weave.css`

## Phase 4: User Story 3 - Reproduce the identity offline

**Goal**: Provide usable local downloads and practical sizing guidance in both delivery forms.

**Independent Test**: A built book contains all three linked files and preserves source guidance and responsive structure.

- [x] T013 [US3] Add failing local-link, guidance, responsive CSS, and generated-output tests
- [x] T014 [US3] Add clear-space, minimum-size, aspect-ratio, no-recolor, and local download guidance
- [x] T015 [US3] Connect real repository assets, Markdown, CSS, and generated HTML to documentation policy
- [x] T016 [US3] Assert all three built assets and generated Brand Standard semantics

## Phase 5: Governance and traceability

- [x] T017 Update S080 completion and S081 progress in `docs/project/build-plans/plan-039.md`
- [x] T018 Add spec 081 and merged S080 evidence to `docs/project/migration-ledger.json`
- [x] T019 Add S081 Added and dated Decisions entries to `CHANGELOG.md`
- [x] T020 Mark the spec implemented and complete post-implementation analysis

## Phase 6: Verification and delivery

- [x] T021 Run every command in `quickstart.md`
- [x] T022 Run UTF-8, LF, mojibake, forbidden-dash, JSON, asset-identity, and diff-integrity gates
- [x] T023 Perform narrow and wide light/navy generated-page inspection
- [ ] T024 Push `codex/s081-brand-standard-visuals` and open the official PR closing #122
- [ ] T025 Process every first-round review finding and hosted CI result
- [ ] T026 If needed, request at most one authorized second Codex review and process it fully
- [ ] T027 Confirm required checks are green and every review thread is resolved before requesting final merge review

## Dependencies and execution order

- T001 through T003 precede implementation.
- T004, T009, and T013 are failing tests written before their validator and documentation changes.
- T005 precedes T007; T010 precedes T011; T013 precedes T014 through T016.
- T006 and T008 can proceed after T004.
- T017 through T020 follow functional implementation.
- T021 through T023 precede publication.
- T024 through T027 are ordered remote-delivery gates.

## Implementation strategy

Treat the page, exact assets, palette map, and generated delivery as one static contract. Reuse existing mdBook and policy infrastructure, keep styles page-scoped, and add no generator, network request, or runtime dependency.
