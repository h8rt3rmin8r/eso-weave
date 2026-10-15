# S130 Tasks

Date: 2026-10-15. Complete all five revised issues; normal/separate v15 only. Formats #296/#297 follow separately. Tests precede their corresponding implementation. No implementation starts until blocking analysis passes.

## Phase 1: Setup

- [x] T001 Archive completed Plan055, establish Plan056 and synchronize plan indexes and migration ledgers in docs/project and docs/archive/build-plans.
- [x] T002 Revise spec/issue acceptance for direct owner format decision and file follow-ups #296/#297 in specs/130-automatic-native-processing/issue-acceptance.md.

## Phase 2: Foundations

- [x] T003 Resolve parser/store/runtime/interface contracts in specs/130-automatic-native-processing/contracts and validate constitution/data-integrity checklists.
- [x] T004 Run blocking cross-artifact analysis and record requirement/task coverage in specs/130-automatic-native-processing/analysis.md.

## Phase 3: US1 and US2 Incremental Native Evidence

- [x] T005 [US1] Write failing chunk/UTF-8/CRLF/context/completion/empty-feed/profile/recovery/limit tests in tests/encounter_incremental.rs.
- [x] T006 [US1] Implement pure bounded normal/separate v15 feed/checkpoint state in src/encounter/native/incremental.rs and shared schema-aware validators in src/encounter/native.rs.
- [x] T007 [US2] Write restart/corrupt-state/identity/copy/extracted-run and schema1 compatibility tests in tests/encounter_incremental.rs and tests/encounter_native.rs.
- [x] T008 [US2] Implement schema2 logical identity/fingerprints and strict bounded checkpoint restoration without rewriting schema1 evidence in src/encounter/native.rs and src/encounter/native/incremental.rs.

## Phase 4: US2 Durable Storage

- [x] T009 [US2] Write atomic progress/CAS/failure/migration/alias/suppression/backup regressions in tests/encounter_ingest.rs.
- [x] T010 [US2] Add validated persistent native writer and shared transaction insertion/preflight in src/encounter/store.rs; atomically migrate supported old stores and persist fights/checkpoint/aliases.
- [x] T011 [US2] Preserve supported deletion with durable native suppression and tested backup/reader validation in src/encounter/store.rs.

## Phase 5: US1 Automatic Runtime

- [x] T012 [US1] Write root-scoped stop/commit/disable races and discovery-generation tests in tests/native_maintenance.rs and tests/data_maintenance.rs.
- [x] T013 [US1] Add root native tokens, commit admission and truthful shared observations in src/data_maintenance.rs.
- [x] T014 [US1] Implement bounded read-only source range/identity witnesses in src/bounded_file.rs and append/catchup/source-generation scheduling in src/data_maintenance/native.rs.
- [x] T015 [US1] Extend supported Linux root enumeration and deterministic discovery coverage in src/beacon/linux.rs and tests/native_maintenance.rs.
- [x] T016 [US1] Integrate native service construction/shutdown with existing runtime in src/data_maintenance/runtime.rs and src/main.rs, independent of paint/Catalog failures.

## Phase 6: US3 Background Consumers

- [x] T017 [US3] Write retained-store/native-fact/Catalog-swap/HTTP-MCP/disabled-collection integration regressions in tests/encounter_ingest.rs, tests/database_query.rs and tests/player_state.rs.
- [x] T018 [US3] Transfer domain calculation/recommendation ownership and bounded idempotent derivation persistence into src/encounter/history.rs and src/encounter/store.rs.
- [x] T019 [US3] Publish actual native service/capability observations additively in src/app/mod.rs and docs/src/reference/local-extension-state-fields.json; retain fixed authenticated read-only data authorities.
- [x] T020 [US3] Remove stale manual UI instructions from domain diagnostics in src/encounter/history.rs, preserving controlled qualified failure/capability results.

## Phase 7: US4 Interface Retirement

- [x] T021 [US4] Write meaningful absence/accessibility/layout and unaffected-behavior tests in tests/app_ui_sizing.rs, tests/app_strings.rs and tests/app_view_model.rs.
- [x] T022 [US4] Remove full Encounter History/Catalog/Data modals, imports, deletion/recording guides, menus, helper ingress, workers and unreachable presentation state in src/app/ui.rs, src/app/strings.rs and src/app/encounter_history.rs after domain transfer.
- [x] T023 [US4] Retire obsolete model intents/projections and startup viewer attachment in src/app/mod.rs and src/main.rs, preserving one PixelBeacon lifecycle and exactly two inactive-without-addon disable controls.

## Phase 8: US4 Final Current-Surface Audit

- [x] T024 [US4] Inventory/dispose every active help/manual/navigation/contract/policy/test/visual surface in specs/130-automatic-native-processing/surface-audit.md; justify retained history/compatibility references.
- [x] T025 [US4] Reconcile current docs/src pages, executable help, current contracts and targeted frozen policy excerpts/digests in docs/project and .github/scripts/docs-policy* with dated CHANGELOG.md decisions.
- [x] T026 [US4] Regenerate final deterministic screenshots/curation/manifests/hashes and changed diagrams/accessibility equivalents using tests/support/documentation_capture.rs and the governed capture scripts.
- [x] T027 [US4] Validate source/generated/search/navigation/render/release-embedded stale-reference absence with pinned docs tooling and policy tests; record exact results in specs/130-automatic-native-processing/verification.md.

## Phase 9: Verification and Delivery

- [x] T028 Run full Rust formatting/all-target-all-feature Clippy/locked tests and applicable build/docs/render/trust/brand/package/text gates; update CHANGELOG.md and specs/130-automatic-native-processing/verification.md.
- [x] T029 Commit verified S130 with co-author attribution, automatically push and publish official PR closing #289/#290/#292/#293/#294, then attach it to this chat.
- [x] T030 Complete at most two Codex rounds, correct/retest/respond/resolve all findings and verify every thread; publish the final hosted-check receipt linked from verification.md and retain owner-only merge. Hosted acceptance remains an explicit final gate in that receipt before owner handoff.

## Dependencies and Execution

T002/T003 -> T004 -> T005-T008 -> T009-T011 -> T012-T016 -> T017-T020 -> T021-T023 -> T024-T027 -> T028-T030. T001 is setup after the analyze gate. Parser and storage owners may develop against the agreed interface in parallel, but integration waits for both; no two agents edit the same files. UI removal waits for required domain transfer. Documentation inventory can proceed during implementation, final captures require the final interface.

All user stories are required for the final single-PR slice; no partial MVP is represented as completion. Every one of the 83 revised issue acceptance criteria needs actual evidence before handoff. No tag/release/merge or owner field-testing task is included.
