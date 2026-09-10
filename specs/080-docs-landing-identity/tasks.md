# Tasks: Documentation Landing Identity

**Input**: Design documents from `/specs/080-docs-landing-identity/`
**Prerequisites**: `spec.md`, `plan.md`, `research.md`, `data-model.md`, `contracts/`

## Phase 1: Setup and authority capture

- [x] T001 Confirm issue #121, parent #119, and Plan 039 traceability in `specs/080-docs-landing-identity/spec.md`
- [x] T002 Record package and changelog authority decisions in `specs/080-docs-landing-identity/research.md`
- [x] T003 Record the current asset size and byte-identity requirement in `specs/080-docs-landing-identity/plan.md`

## Phase 2: User Story 1 - Recognize the project immediately

**Goal**: Publish the official responsive wordmark with one correct accessible H1.

**Independent Test**: Source and generated policy accept the required identity and reject prohibited substitutes.

- [x] T004 [US1] Add failing identity-structure and banner mutation tests in `.github/scripts/docs-policy.test.mjs`
- [x] T005 [US1] Add landing identity validation in `.github/scripts/docs-policy.mjs`
- [x] T006 [US1] Copy `assets/eso-weave-banner.png` byte-for-byte to `docs/src/assets/brand/eso-weave-banner.png`
- [x] T007 [US1] Replace the square landing mark and visible duplicate name in `docs/src/README.md`
- [x] T008 [US1] Add scoped wordmark and visually hidden text styles in `docs/theme/eso-weave.css`

## Phase 3: User Story 2 - Judge documentation applicability

**Goal**: Present semantic, authoritative, clearly static project and release metadata.

**Independent Test**: The landing metadata exactly matches package and changelog sources and its repository link resolves.

- [x] T009 [US2] Add failing metadata mismatch and missing-release tests in `.github/scripts/docs-policy.test.mjs`
- [x] T010 [US2] Extract package and changelog authorities in `.github/scripts/docs-policy.mjs`
- [x] T011 [US2] Add the semantic metadata definition list and snapshot disclosure to `docs/src/README.md`
- [x] T012 [US2] Add responsive metadata styles in `docs/theme/eso-weave.css`

## Phase 4: User Story 3 - Prevent silent metadata drift

**Goal**: Enforce source, bytes, and generated-delivery consistency without network access.

**Independent Test**: Focused mutations fail and the real repository plus built site pass.

- [x] T013 [US3] Connect real `Cargo.toml`, `CHANGELOG.md`, landing page, and banner bytes in `.github/scripts/docs-policy.mjs`
- [x] T014 [US3] Assert the generated `index.html` semantics and local banner output in `.github/scripts/docs-policy.mjs`
- [x] T015 [US3] Add focused generated-output and byte-drift tests in `.github/scripts/docs-policy.test.mjs`

## Phase 5: Governance and traceability

- [x] T016 Update S079 completion and S080 progress in `docs/project/build-plans/plan-039.md`
- [x] T017 Add spec 080 to the active-plan evidence in `docs/project/migration-ledger.json`
- [x] T018 Add S080 Added and dated Decisions entries to `CHANGELOG.md`
- [x] T019 Update `specs/080-docs-landing-identity/spec.md` status after implementation
- [x] T020 Complete post-implementation analysis in `specs/080-docs-landing-identity/analysis.md`

## Phase 6: Verification and delivery

- [x] T021 Run all commands in `specs/080-docs-landing-identity/quickstart.md`
- [x] T022 Run repository UTF-8, LF, mojibake, forbidden-dash, and `git diff --check` gates
- [ ] T023 Push `codex/s080-docs-landing-identity` and open the official pull request closing #121
- [ ] T024 Process every first-round review finding and hosted CI result
- [ ] T025 If needed, request at most one authorized second Codex review and process it fully
- [ ] T026 Confirm all required checks are green and every review thread is resolved before requesting final merge review

## Dependencies and execution order

- T001 through T003 precede implementation.
- T004 and T009 are failing tests written before T005, T010, T011, and T013.
- T005 precedes T007 because the source contract defines the accepted identity.
- T006 and T008 may proceed after T004 independently.
- T009 precedes T010 and T011.
- T013 precedes T014 and T015.
- T016 through T020 follow functional implementation.
- T021 and T022 precede publication.
- T023 through T026 are ordered remote-delivery gates.

## Implementation strategy

Deliver the identity and metadata as one atomic landing-page contract. Keep the
static page portable, enforce its values from existing repository authorities,
and avoid adding any generator, remote request, or runtime dependency.
