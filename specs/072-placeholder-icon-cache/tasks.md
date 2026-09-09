# Tasks: Placeholder-First Local Icon Cache

**Input**: Design documents from `/specs/072-placeholder-icon-cache/`

**Tests**: Required by Constitution Principles II through IV and issue #116.

## Phase 1: Spec-kit and contracts

- [x] T001 Create the complete S072 specification, clarification record,
  requirements checklist, and asset-safety checklist.
- [x] T002 Produce research, data model, JSON schema, quickstart, implementation
  plan, tasks, and pre-implementation analysis.
- [x] T003 Bind issue #116 to S072 and move its delivery stage to Specced.

## Phase 2: Test-first foundation

- [x] T004 Add failing path normalization, traversal, case ambiguity, link,
  root-alias, and source-preservation tests in `tests/icon_cache.rs`.
- [x] T005 Add failing deterministic placeholder, PNG, DDS, corrupt, oversize,
  dimension, layer, depth, and mip-bound tests in `tests/icon_cache.rs`.
- [x] T006 Add failing manifest mapping, deduplication, byte stability,
  publication preservation, object verification, and privacy tests in
  `tests/icon_cache.rs`.
- [x] T007 Add the decode-only `image_dds` dependency configuration in
  `Cargo.toml` and update `Cargo.lock`.

## Phase 3: Safe reference and transform boundary

- [x] T008 Implement typed public requests, states, errors, receipts, limits,
  and module exports in `src/icon_cache/mod.rs` and `src/lib.rs`.
- [x] T009 Implement virtual-path normalization and bounded component-wise local
  resolution in `src/icon_cache/path.rs`.
- [x] T010 Implement platform-aware link-like metadata rejection and resolved
  root containment checks in `src/icon_cache/path.rs`.
- [x] T011 Implement the project placeholder and bounded PNG decode in
  `src/icon_cache/transform.rs`.
- [x] T012 Implement DDS header gates, safe mip-0 decode, RGBA conversion,
  deterministic PNG encoding, and source/output hashing in
  `src/icon_cache/transform.rs`.

## Phase 4: Immutable cache generation

- [x] T013 Implement canonical manifest types, validation, and serialization in
  `src/icon_cache/manifest.rs`.
- [x] T014 Implement no-clobber content-addressed object publication and
  deduplication in `src/icon_cache/mod.rs`.
- [x] T015 Implement complete candidate verification and immutable generation
  publication in `src/icon_cache/mod.rs`.
- [x] T016 Implement generation loading, full hash verification, and typed icon
  resolution in `src/icon_cache/mod.rs`.
- [x] T017 Confirm catalog compilation and distributable baseline remain valid
  with reference-only catalog rows and no user-local bytes.

## Phase 5: Documentation and governance

- [x] T018 Update catalog, architecture, source-rights, status, troubleshooting,
  and testing documentation for the S072 local cache boundary.
- [x] T019 Update `CHANGELOG.md`, Plan 038, the build-plan index, and migration
  ledger in chronological order.
- [x] T020 Add documentation policy coverage for the no-network,
  no-third-party-byte, and explicit cache-manifest requirements.

## Phase 6: Analysis and verification

- [x] T021 Run focused cache, catalog, packaging, and documentation policy tests.
- [x] T022 Run `specify check`, JSON, UTF-8, BOM, LF, forbidden-dash, mojibake,
  and placeholder scans.
- [x] T023 Run `cargo fmt --all -- --check`, strict all-feature Clippy, and the
  complete locked test suite in the foreground.
- [x] T024 Build both optimized binaries and run mdBook test/build plus published
  documentation policy validation.
- [x] T025 Re-run spec-kit analysis, resolve all findings, and complete the
  quickstart evidence.

## Phase 7: Delivery

- [x] T026 Commit S072 with the required attribution trailer, push the authorized
  feature branch, and open an official pull request closing #116.
- [ ] T027 Wait for CI and automated reviews, respond to every finding, push
  verified corrections, and resolve all review threads.
- [ ] T028 Use no more than the one authorized second Codex round, then wait for
  green checks and request the final human review and merge ritual.

## Dependency order

1. Phase 1 freezes the authority chain.
2. Phase 2 tests fail before implementation.
3. Phase 3 supplies bounded acquisition and transformation.
4. Phase 4 publishes and reads immutable generations.
5. Phase 5 makes the contract canonical.
6. Phase 6 blocks every commit.
7. Phase 7 begins only after local gates pass.
