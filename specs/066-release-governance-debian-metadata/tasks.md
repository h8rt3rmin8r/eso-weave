# Tasks: Release Governance and Debian Metadata

## Phase 1: Spec-kit and Lifecycle Setup

- [x] T001 Synchronize `main` and create `codex/s066-release-governance-debian-metadata`.
- [x] T002 Create the S066 specification from issues #104 and #105.
- [x] T003 Resolve constitution versioning, maintainer identity, validation, verification-split, and untracked-file clarifications.
- [x] T004 Create requirements, governance, and Debian package checklists.
- [x] T005 Create research, data model, contracts, quickstart, and implementation plan.
- [x] T006 Create this chronological task plan.
- [x] T007 Run and pass pre-implementation cross-artifact analysis.

## Phase 2: Release Lifecycle (US1)

- [x] T008 [US1] Amend `.specify/memory/constitution.md` to 2.0.1 with a complete Sync Impact Report.
- [x] T009 [US1] Align `CLAUDE.md` and `docs/project/build-autopilot.md` with chronological artifact-dependent verification.
- [x] T010 [US1] Align `docs/project/governance.md` and `docs/project/releasing.md` without weakening pre-publication gates.

## Phase 3: Debian Metadata and Prevention (US2)

- [x] T011 [US2] Add failing fixtures in `scripts/validate-debian-package.test.sh`.
- [x] T012 [US2] Implement `scripts/validate-debian-package.sh` to validate the actual control record.
- [x] T013 [US2] Add the explicit maintainer to `Cargo.toml` without changing package layout.
- [x] T014 [US2] Run validator fixtures in Linux pull-request CI.
- [x] T015 [US2] Gate the built `.deb` in the release workflow before packaging and upload.
- [x] T016 [US2] Build and inspect an actual package and preserve its existing payload layout.

## Phase 4: Independent Verification Lifecycle (US3)

- [x] T017 [US3] File a next-release Debian verification issue with exact entry and completion gates.
- [x] T018 [US3] Add the verification issue to the Delivery Project in Release verification.
- [x] T019 [US3] Amend #105 so its repository implementation outcome closes through S066.

## Phase 5: Lifecycle Records

- [x] T020 Archive completed plan 035 and create active plan 036.
- [x] T021 Update active/archive indexes and `docs/project/migration-ledger.json` plus its summary.
- [x] T022 Add S066 Changed and dated Decisions entries to Unreleased for the governance and pinned packaging changes.

## Phase 6: Analysis, Validation, and Delivery

- [x] T023 Complete post-implementation spec-kit analysis and resolve every finding.
- [x] T024 Complete governance and Debian package checklists.
- [x] T025 Run validator fixtures, generated-package inspection, docs policy, text hygiene, and full CI parity.
- [x] T026 Audit the diff for runtime, payload-layout, pinned-artifact, secret, encoding, and user-file boundaries.
- [ ] T027 Commit as `feat(066): clarify releases and validate Debian metadata` with attribution.
- [ ] T028 Push and open the official pull request closing #104 and #105.
- [ ] T029 Move S066 items to PR review and monitor every hosted check and first review.
- [ ] T030 Resolve every first-round review finding.
- [ ] T031 Trigger exactly one authorized second `@Codex review` and resolve every result.
- [ ] T032 Confirm green checks, no unresolved threads, and merge readiness before requesting the merge ritual.

## Dependencies

- T007 blocks implementation.
- T011 must fail before T012 implements the validator.
- T012 blocks T014 through T016.
- T017 and T018 block narrowing #105 in T019.
- T023 through T026 block commit and publication.
- T032 ends with operator review; S066 never merges its own pull request.
