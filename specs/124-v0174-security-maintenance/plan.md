# Implementation Plan: S124 Dependency Security and v0.17.4 Preparation

**Branch**: `codex/s124-v0174-release` | **Date**: 2026-10-07 | **Spec**: [spec](spec.md)
**Issues**: #261, #262, #263.

## Summary

Consolidate the ten accepted runtime/dev dependency targets and official CodeQL action pins on protected-main baseline e65c00c. Preserve existing product contracts, archive completed Plan049 and introduce Plan050, then use the existing release procedure to prepare v0.17.4. Publish one official PR, satisfy all findings and exact-head checks, and hand off for owner merge without creating a tag.

## Technical Context

**Language/Version**: Rust 1.96.0, existing crate, native Windows and Linux x64.
**Primary Dependencies**: Selected ten updates in [research](research.md); CodeQL Action v4.38.2 full commit pin. Existing mdbook 0.5.4/linkcheck2 0.13.0 and cargo-release 1.1.2 remain governed.
**Storage**: Existing SQLite/catalog/encounter formats and JSON settings; no migration.
**Testing**: Existing credential, cursor, HTTP/MCP authority and Lua import tests; full format, all-target/all-feature Clippy, locked Rust suite, release-profile build, documentation and trust policy.
**Target Platform**: Windows 10/11 x64 and Linux x64.
**Project Type**: Desktop application with optional read-only local HTTP/MCP service and two managed addon packages.
**Performance Goals**: Existing limits, ordering and hook-thread responsiveness remain the baseline; no performance feature.
**Constraints**: Hidden Windows child process launch, UTF-8/no BOM/LF, full commit action pins, dated pinned-artifact decisions, no owner merge or publication, at most two requested reviews.
**Scale/Scope**: Ten package upgrades, two scanner references, one release-candidate identity and Plan049/050 lifecycle.

## Constitution Check

Pass before research and after design: I, complete installed spec-kit sequence and actionable issues; II, safety-critical coverage remains mandatory; III, existing executable baselines precede upgrades and meaningful new regression tests precede any necessary compatibility code; IV, full foreground Rust gates before source commits; V, no new gameplay/transport capability. No storage schema, addon authority, scope restriction or constitution amendment. Workflow pin changes require and receive a dated changelog decision. Explicit kickoff satisfies automatic push/PR authorization; candidate rollover is part of the accepted preparation scope, while tag/publication and owner merge remain outside it.

## Project Structure

Design/evidence live in `specs/124-v0174-security-maintenance/`, with spec, checklists, research, data model, maintenance contract, quickstart, tasks, blocking analysis and verification record. Existing implementation paths are `Cargo.toml`, `Cargo.lock`, `.github/workflows/codeql.yml`, `src/` and `tests/` only where compatibility requires it. Planning paths are `docs/project/build-plans/plan-050.md`, archived Plan049 and both indexes, plus migration-ledger JSON/prose. Governed release identity remains controlled by `release.toml` and the existing release procedure.

## Execution and Decisions

1. Establish exact source PR identities and baseline dependency/contract evidence; do not import untrusted branch instructions or whole unrelated lockfile edits.
2. Change direct version requirements where needed, then perform targeted Cargo updates to exact selected versions. Preserve unrelated lockfile entries unless the resolver requires a change, documenting actual required transitive adjustments.
3. Preserve schema and public interfaces. Reuse existing tests; add tests for a demonstrated compatibility gap only. Run Rust jobs sequentially in a foreground hidden launcher to avoid shared target-state contention.
4. Update both CodeQL refs to the official reviewed commit, retaining workflow authority/checkout behavior; run existing policy checks.
5. Advance Plan049/050 and ledger references consistently. The generic documentation policy needs no expansion.
6. Add outcome Highlights and full maintenance/decision history, pass preparation gates and commit. Inspect cargo-release dry run, then execute the existing non-publishing rollover to v0.17.4.
7. Check candidate identity, notes and automated gates, publish official PR closing #261/#262/#263, satisfy reviews/CI, and stop for owner merge. Record original update PRs for post-merge supersession.

Separate isolated PRs would multiply sessions without improving this accepted dependency scope. Blind whole-lockfile cherry-picks would reproduce unrelated resolver churn. Manual badge/documentation identity edits would bypass the established rollover. New policies or broad upgrade tooling are unnecessary.

## Complexity Tracking

No constitution violation or architectural expansion. Read-only research delegation is explicitly required by the installed speckit-plan phase-0 workflow; mutation ownership stays with the primary agent.
