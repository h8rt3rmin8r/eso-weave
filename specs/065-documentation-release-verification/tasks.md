# Tasks: Documentation Release Verification

## Phase 1: Spec-kit and Entry Evidence

- [x] T001 Synchronize `main`, publish authorized v0.15.0, and create `codex/s065-docs-release-verification`.
- [x] T002 File governance clarification #104 without changing S065 scope.
- [x] T003 Confirm release workflow 34302551069 and all five expected assets are published.
- [x] T004 Create the S065 specification and resolve environment, package, evidence, isolation, and defect-boundary clarifications.
- [x] T005 Create requirements, release-evidence, and delivery checklists.
- [x] T006 Create research, data model, receipt contract, quickstart, and implementation plan.
- [x] T007 Run and pass pre-evidence cross-artifact analysis.

## Phase 2: Artifact Integrity

- [x] T008 [US1] Download all v0.15.0 release assets into an isolated temporary directory.
- [x] T009 [US1] Verify names, byte sizes, and all package digests against `SHA256SUMS`.
- [x] T010 [US1] Inspect MSI, deb, AppImage, and tarball payloads for expected binaries and sidecar absence.

## Phase 3: Windows Package

- [x] T011 [US2] Attempt the released MSI upgrade, record the elevation blocker, and run the exact administratively extracted MSI payload.
- [x] T012 [US2] Exercise Help > Documentation by pointer and record the operator's direct acceptance and instruction to end desktop interaction.
- [x] T013 [US2] Map the operator's presentation acceptance and release-commit contracts to search, navigation, theme, layout, asset, code, link, and 404 behavior.
- [x] T014 [US2] Directly inspect loopback ownership and bind address, with method, failure, reuse, and shutdown support from release-commit contracts.

## Phase 4: Linux Packages

- [x] T015 [US3] Install and launch the released deb under Ubuntu 24.04 WSLg and record the environment and package path.
- [x] T016 [US3] Record offline deb launch, release-contract support, cleanup, and the operator's accepted variance for further desktop interaction.
- [x] T017 [US3] Inspect the released AppImage and tarball for portable payload integrity and sidecar independence.

## Phase 5: Receipt and Lifecycle

- [x] T018 [US4] Write `docs/project/release-verification/v0.15.0-documentation.md` with exact evidence and limitations.
- [x] T019 [US4] Map every #84 completion criterion and epic #83 gate to its evidence and operator acceptance decision.
- [x] T020 [US4] File separate Debian metadata defect #105 without mislabeling it as a documentation failure.
- [x] T021 Archive plan 034 with v0.15.0 evidence, establish plan 035, and update indexes and the migration ledger.
- [x] T022 Add the S065 verification record to Unreleased without claiming a behavior change.

## Phase 6: Validation and Delivery

- [x] T023 Run documentation policy, receipt checks, whitespace, UTF-8, BOM, punctuation, mojibake, and secret checks.
- [x] T024 Run applicable repository contract tests and final package-evidence checks.
- [x] T025 Complete post-evidence cross-artifact analysis and diff audit.
- [ ] T026 Commit as `feat(065): verify bundled release documentation` with attribution.
- [ ] T027 Push and open an official pull request with conditional closing references for #84 and #83.
- [ ] T028 Move the slice to PR review and monitor every CI check and first-round review.
- [ ] T029 Resolve every first-round review finding.
- [ ] T030 Trigger exactly one authorized second `@Codex review` and resolve every result.
- [ ] T031 Confirm green checks, no unresolved review threads, and merge readiness before requesting the merge ritual.

## Dependencies

- T007 blocks artifact work.
- T008 through T010 block platform execution.
- T011 through T017 block a passing completion matrix.
- An unresolved bundled-documentation failure prevents closing #84 and #83;
  the operator's explicit acceptance decision and any recorded non-documentation
  variance remain part of the durable evidence.
- T023 through T025 block commit and publication.
- T031 ends with operator review; S065 never merges its own pull request.
