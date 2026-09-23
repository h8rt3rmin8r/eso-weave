# Tasks: BrandBuilder 2.0.1 Density Correction

**Input**: Design documents from `specs/119-brandbuilder-2-0-1-density/`

**Prerequisites**: `spec.md`, `plan.md`, `research.md`, `data-model.md`, `contracts/native-density.md`, `quickstart.md`

**Tests**: Required by the constitution and the feature specification. New regressions must fail before production changes.

## Phase 1: Setup and exact authority

- [x] T001 Confirm issue #241, branch, clean baseline, active feature directory, upstream S044 revision, and current 2.0.0 package hash in `specs/119-brandbuilder-2-0-1-density/evidence.md`
- [x] T002 Obtain the CI-certified or official BrandBuilder 2.0.1 ESO Weave kit, validate its bundle and manifest, and record exact publication facts in `specs/119-brandbuilder-2-0-1-density/evidence.md`
- [x] T003 Validate the spec, plan, research, contract, quickstart, and checklists before implementation

## Phase 2: Failing regression coverage

- [x] T004 [US1] Add failing theme assertions for the 28-point control height and 8-by-4 button padding in `src/app/theme.rs`
- [x] T005 [US2] Add failing theme assertions for 8-by-2 global item spacing and preserve explicit sizing coverage in `tests/app_ui_sizing.rs`
- [x] T006 [US3] Add failing BrandBuilder 2.0.1, egui adapter 1.0.1, recovery, and density-policy assertions in `.github/scripts/brand-kit-policy.test.mjs`
- [x] T007 Run the focused Rust and Node regressions, confirm expected S117 failures, and record red evidence

## Phase 3: Runtime density correction

- [x] T008 [US1] Apply the comfortable precise-pointer height and padding in `src/app/theme.rs`
- [x] T009 [US2] Apply dense global row spacing in `src/app/theme.rs` without changing product-owned resource meters
- [x] T010 [US1] Run focused theme and UI sizing tests and record exact green measurements

## Phase 4: Immutable kit repin

- [x] T011 [US3] Update `assets/brand/brand-kit-adoption.json` with exact 2.0.1 package, compiler, adapter, source, checksum, and density facts
- [x] T012 [US3] Replace the retained recovery distribution with `assets/brand/recovery/shruggie-brandbuilder-2.0.1.skill` and remove the superseded 2.0.0 recovery file
- [x] T013 [US3] Update `.github/scripts/brand-kit-policy.mjs` to enforce the new immutable metadata, recovery hash, and runtime density values
- [x] T014 [US3] Reconcile every consumed artifact hash and preserve unchanged identity, font, palette, and platform bytes

## Phase 5: Documentation and governance

- [x] T015 [US3] Update `assets/brand/README.md` with 2.0.1 provenance and recovery guidance
- [x] T016 [US1] [US2] Update `docs/src/development/brand-standard.md` with the precise-pointer and conservative-target distinction
- [x] T017 Record the dated BrandBuilder 2.0.1 provenance and density decision in `CHANGELOG.md`
- [x] T018 Complete the brand checklist and post-implementation analysis

## Phase 6: Verification and local delivery

- [x] T019 Run focused brand policy, theme, and UI sizing tests
- [x] T020 Run fmt, clippy, complete locked tests, and locked release build
- [x] T021 Run documentation policy, render smoke, mdBook test/build, and repository docs validation
- [x] T022 Run diff, encoding, LF, BOM, mojibake, forbidden-dash, and untracked-file audits
- [x] T023 Review the complete diff for provenance, accessibility, scope, safety boundaries, and unrelated changes
- [x] T024 Mark completed tasks and commit locally with S119 and issue linkage
- [x] T025 Halt before `git push -u origin codex/s119-brandbuilder-2-0-1-density` and report the autopilot breakdown

## Dependencies and Execution Order

- Exact upstream authority and local baseline precede test changes.
- Red Rust and Node evidence blocks production implementation.
- Runtime correction and package repin precede documentation.
- Full verification and review precede the local commit.
- The pre-push halt is mandatory because kickoff did not explicitly authorize remote publication.
