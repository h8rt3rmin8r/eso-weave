# Tasks: Reviewed Catalog Candidate Pipeline

## Phase 1: Authority and contracts

- [x] T001 Trace S073 to issue #117, Plan 038, S068 through S072, and the
  constitution.
- [x] T002 Complete specify, clarify, requirements checklist, pipeline safety
  checklist, research, data model, request schema, candidate schema, plan, and
  quickstart before implementation.
- [x] T003 Record the deliberate exclusion of a second stock UI parser and all
  application update behavior.

## Phase 2: Test-first contract

- [x] T004 Add failing strict request and version-tuple tests in
  tests/catalog_pipeline.rs.
- [x] T005 Add failing mode/channel, same-channel baseline, threshold, and
  no-automatic-promotion tests.
- [x] T006 Add failing bounded local source, mock HTTPS, cache reuse, stale
  fallback, hash, link, alias, and no-clobber tests.
- [x] T007 Add failing S071 capture, S070 build/verify/diff, and S072 placeholder
  composition tests.
- [x] T008 Add failing candidate allowlist, privacy, determinism, verification,
  and failed-publication preservation tests.
- [x] T009 Add failing pinned workflow permission, trigger, artifact, and
  prohibited-authority tests in tests/catalog_pipeline_workflow.rs.

## Phase 3: Shared bounded input and versions

- [x] T010 Extract the stable bounded no-follow reader into src/bounded_file.rs
  and keep every S072 regression green.
- [x] T011 Add a shared catalog game/version tuple in src/catalog/version.rs
  and migrate the startup API checker without behavior change.
- [x] T012 Implement strict relative path, distinct-root, source identity, mode,
  channel, locale, revision, and threshold validation.

## Phase 4: Source acquisition

- [x] T013 Implement verified bounded local acquisition and content-addressed
  immutable source cache reuse.
- [x] T014 Implement the mockable opt-in HTTPS adapter with hardcoded host,
  immutable revision, redirect, timeout, streaming byte, and hash gates.
- [x] T015 Implement explicit refresh and stale-cache reporting without silent
  fallback.
- [x] T016 Validate source pins against S068 policy or the narrow
  project-authored fixture exception.

## Phase 5: Candidate orchestration

- [x] T017 Implement S071 capture import or normalized-bundle staging from the
  verified input source.
- [x] T018 Validate the complete version tuple and call S070 build, verify, and
  same-channel diff.
- [x] T019 Extract eligible icon references, build an S072 generation in the
  separate local cache, and write only a redacted receipt.
- [x] T020 Evaluate configured removal and coverage thresholds into stable
  findings that block publication.
- [x] T021 Write canonical source inventory, validation, diff, build, verify,
  icon, and checksum reports.
- [x] T022 Implement allowlisted manifest hashing, complete candidate
  verification, immutable no-clobber publication, and reuse.
- [x] T023 Add pipeline-build and pipeline-verify commands to the existing
  maintainer CLI.

## Phase 6: Automation and canonical documentation

- [x] T024 Add invented request/source fixtures for Live, PTS, offline, and
  capture modes.
- [x] T025 Add the SHA-pinned, read-only manual/scheduled candidate workflow.
- [x] T026 Extend documentation policy tests for workflow authority and candidate
  artifact boundaries.
- [x] T027 Update architecture, catalog compiler, source rights, status, test
  strategy, and troubleshooting documentation.
- [x] T028 Update CHANGELOG.md, Plan 038, the build-plan index, and migration
  ledger in chronological order.

## Phase 7: Analysis and verification

- [x] T029 Run focused pipeline, compiler, collector, icon, workflow, packaging,
  and documentation-policy tests.
- [x] T030 Run JSON, UTF-8, BOM, LF, forbidden-dash, mojibake, placeholder, and
  spec-kit scans.
- [x] T031 Run format, strict all-feature Clippy, and the complete locked test
  suite in the foreground.
- [x] T032 Build both optimized binaries and run mdBook test/build plus generated
  documentation policy validation.
- [x] T033 Re-run spec-kit analysis, resolve all findings, and complete the
  quickstart evidence.

## Phase 8: Delivery

- [x] T034 Commit S073 with the required attribution trailer, push the authorized
  branch, and open an official pull request closing #117.
- [ ] T035 Wait for CI and automated reviews, respond to every finding, push
  verified corrections, and resolve all review threads.
- [ ] T036 Use no more than the one authorized second Codex round, then wait for
  green checks and request the final human review and merge ritual.

## Dependency order

1. Phase 1 freezes authority before tests.
2. Phase 2 establishes red failures before production code.
3. Phase 3 supplies shared safe file and version primitives.
4. Phase 4 admits only verified source bytes.
5. Phase 5 composes prior catalog foundations and publishes review candidates.
6. Phase 6 adds constrained automation and canonical guidance.
7. Phase 7 blocks every Rust commit.
8. Phase 8 begins only after all local gates pass.
