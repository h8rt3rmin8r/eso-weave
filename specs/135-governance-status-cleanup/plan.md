# Implementation Plan: S135 Governance Status Cleanup

**Branch**: `codex/135-governance-status-cleanup` | **Date**: 2026-10-17 | **Spec**: [spec.md](spec.md)

**Input**: Atomic documentation issue #319 and owner-selected S135.

## Summary

Make historical audits unmistakably historical, reconcile stale current ledger/plan pointers, and separate closed verification lifecycles from unperformed evidence. Use existing documents and validators.

## Technical Context

- Language/storage: existing Markdown and maintainer JSON documents, UTF-8 without BOM and LF.
- Dependencies: existing Node documentation/render/brand/trust policy tools; GitHub issue/PR/Project metadata. No new dependency.
- Testing: 184 existing policy tests, full source policy, protected trust scan, semantic/boundary comparisons and text/whitespace checks.
- Platform: repository documentation only; current Windows hidden/noninteractive launch pattern.
- Performance: no new runtime or build path.
- Scope: governance.md; migration-ledger.md/.json; documentation-visualization-audit.md; encounter-model.json and catalog-sources.json lifecycle context; constitution sync metadata/version; S135 planning/index/changelog/context.
- Constraints: no application code, dependency, release identity, packaging, workflow, archive or historical-result rewrite; no field task; no merge/tag/publication.

## Constitution Check

PASS before research and after design under protected-main 5.1.0. Full specify/clarify/checklist/plan/tasks/analyze precede implementation. Principles, safety surfaces and test gates remain unchanged. The planned constitutional patch is metadata wording only and follows its Sync Impact/version procedure outside analyze. Constitution IV explicitly exempts non-Rust changes from local Cargo gates. Existing policy suites are appropriate; no prose-mirroring test is added. No pinned artifact requires modification. Issue-first delivery, conventional co-author commit, authorized branch/PR publication and owner final merge are preserved.

## Phases

1. Inspect live lifecycle evidence and existing policy consumers; consolidate read-only research and deliberate retention decisions.
2. Complete design, written-quality checklist and task graph; run blocking read-only analysis.
3. Execute metadata-only constitution clarification with version/sync report and unchanged principles. Reconcile scoped governance, ledger and plan pointers; add separate machine lifecycle context.
4. Register chronological Plan061 and S135 in current index and machine/prose ledgers, update agent context and Unreleased changelog, and create the scoped audit inventory.
5. Run required checks, compare protected source/evidence boundaries, record observed results, commit and publish the issue-linked review PR. Resolve findings; establish green exact-head checks; stop for owner merge.

## Project Structure

- specs/135-governance-status-cleanup/: spec, checklists, research, model, validation guide, plan, tasks, analysis and audit/verification receipts.
- docs/project/governance.md: dated historical inventory and current-state authority.
- docs/project/migration-ledger.json/.md: corrected lifecycle summaries and Plan061.
- docs/project/documentation-visualization-audit.md: completed/archived Plan049 pointer.
- docs/project/encounter-model.json and catalog-sources.json: additive lifecycle_context only.
- .specify/memory/constitution.md: metadata-only patch 5.1.1; no principle/gate changes.
- docs/project/build-plans/plan-061.md and README.md, CLAUDE.md, CHANGELOG.md: current slice records.

This is an internal documentation interface; no external contract is added, so contracts/ is omitted under the installed planning command. No complexity exception or unresolved clarification remains. Optional agent-context hooks are fulfilled by the required CLAUDE.md plan-reference update; no mandatory hook is configured.
