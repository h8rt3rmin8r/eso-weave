# Specification Analysis Report: Persistent Data Addon Foundation

**Analyzed**: 2026-09-13
**Artifacts**: `spec.md`, `plan.md`, `tasks.md`, constitution 3.0.0

## Findings

No CRITICAL or HIGH findings remain.

| ID | Category | Severity | Location | Finding | Resolution |
| --- | --- | --- | --- | --- | --- |
| I1 | Identifier consistency | MEDIUM | `spec.md` success criteria | One inserted module-clear criterion reused a suffixed identifier | Renumbered success criteria as contiguous SC-001 through SC-010 |
| C1 | Task traceability | MEDIUM | `tasks.md` T004, T028, T033, T040, T045, T049 | Six operational tasks lacked a durable file or artifact reference | Added exact repository evidence paths to all six tasks |
| G1 | Governance | RESOLVED | constitution Principle V | The approved two-addon topology conflicted with constitution 2.2.0 | Applied the explicitly approved 3.0.0 amendment before analysis and retained every module safety rule |
| R1 | Lifecycle safety | HIGH | `src/data_addon.rs` | Per-file replacement could expose partial state, lose prior bytes on rollback failure, or follow a swapped package directory | Replaced it with complete sibling staging, atomic no-replace directory quarantine and commit on Linux and Windows, post-move snapshot validation, retained recovery artifacts, and deterministic race tests |
| R2 | Lifecycle safety | HIGH | `src/data_addon.rs` | Recursive cleanup could remove a raced-in unexpected entry | Cleanup now removes only the three known Lua files and manifest, with the ownership marker removed last, then requires an empty directory |
| R3 | Manifest integrity | MEDIUM | `src/data_addon.rs`, `tests/data_addon.rs` | Status accepted drift in the SavedVariables declaration or Lua load order | Status now compares the complete rendered manifest while allowing only the selected primary API version to vary; repair coverage proves drift is corrected |
| R4 | Parser safety | HIGH | `src/collector/import.rs` | Catalog import separated metadata checks from the later file read | Catalog import now uses the shared bounded stable no-follow reader and verifies file identity through the read |
| R5 | Parser contract | MEDIUM | shared-root fixtures | Hostile parser fixtures still targeted obsolete top-level assignments and outer version fields were not enforced | Fixtures now exercise `EsoWeaveDataSaved`; both projections reject unsupported outer schema and addon versions before selecting their module |
| R6 | Module isolation | MEDIUM | `tests/data_addon.rs` | Static substring checks did not prove the two Lua modules coexist safely | A Lua 5.1 harness loads the manifest order, proves distinct namespaces and dormancy, and exercises both clear orders without cross-module mutation |
| R7 | Governance sync | HIGH | `CLAUDE.md`, `docs/project/build-autopilot.md` | Pinned-artifact lists did not fully match constitution 3.0.0 | Both lists now include the complete constitutional set, including release guidance and repository policy files |
| R8 | Module isolation | HIGH | `src/catalog_update/mod.rs`, `src/collector/import.rs` | Catalog freshness and provenance fingerprinted the entire shared file, so encounter-only writes could impersonate a later catalog flush | Freshness, source staging, catalog version identity, and import provenance now use one canonical catalog-only SavedVariables projection; regression coverage proves encounter-only rewrites are inert |
| R9 | Version ownership | MEDIUM | `src/data_addon.rs` | Package lifecycle status compared the manifest version with the catalog module version | The data addon now owns an explicit package-version constant independent of either module envelope |
| R10 | Documentation accuracy | MEDIUM | `docs/src/development/catalog-updates.md` | The canonical guide retained the removed desktop deletion behavior and separate collector ownership | The guide now documents module-local in-game clearing, shared-file preservation, and the combined package uninstall boundary |
| R11 | Cross-platform fixture integrity | HIGH | `specs/073-reviewed-catalog-pipeline/fixtures/capture-request.json`, `tests/catalog_pipeline.rs` | Windows source-cache state masked a committed request that still pinned the pre-isolation shared-file hash; clean Linux CI correctly rejected the mismatch | Added a checked-in canonical catalog projection, pinned committed-mode provenance to its exact hash, and added a test that regenerates the projection from the shared fixture to prevent drift |
| R12 | Documentation accuracy | MEDIUM | `docs/src/development/test-strategy.md`, `docs/src/reference/status-reference.md` | The test map and status reference still named the deleted collector lifecycle suite and dedicated package topology | Both canonical references now point to `tests/data_addon.rs` and describe the shared package with isolated catalog and encounter modules |

## Coverage Summary

| Requirement group | Coverage | Task evidence |
| --- | --- | --- |
| FR-001 through FR-011, package and governance | 100% | T001 through T024, T034 through T039 |
| FR-012 through FR-015, encounter ingestion | 100% | T025 through T028, T040 through T044 |
| FR-016 through FR-020, command decision | 100% | T029 through T033, T040 through T045 |
| FR-021 through FR-022, issue evidence and exclusions | 100% | T004, T028, T033, T047 through T049 |
| SC-001 through SC-010 | 100% | T005 through T045 |

## Metrics

- Functional requirements: 22
- Success criteria: 10
- Tasks: 49
- Requirements with at least one task: 32 of 32 (100%)
- Unmapped tasks: 0
- Ambiguity findings: 0
- Duplication findings: 0
- Critical findings: 0
- High findings: 0

## Gate Result

PASS. The specification, plan, tasks, implementation, and constitution are
mutually consistent. Local code, security, and domain re-reviews report no
remaining actionable findings. Formatting, lint, full locked tests,
documentation build and link checks, spelling, text hygiene, and documentation
policy all pass.
