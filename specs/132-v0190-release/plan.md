# Implementation Plan: S132 v0.19.0 Release Candidate

**Branch**: `codex/s132-v0190-release` | **Date**: 2026-10-15 | **Spec**: [spec.md](spec.md)
**Input**: #302 under #301; incorporate dependency PR #284; automatic push/PR and at most two Codex reviews, then owner merge.

## Summary

Prepare all merged post-v0.18.0 work and #284's exact dependency patch in one reviewed candidate. Reuse the existing local cargo-release rollover and all candidate gates. Reconcile stale release package wording and create chronological Plan058. No runtime source or pipeline change is planned.

## Technical Context

**Language/Version**: Existing Rust 1.96; Markdown, TOML and JSON metadata.
**Primary Dependencies**: serde_json 1.0.152, rmcp 3.5.1 and tokio-util 0.7.20 from #284; configured cargo-release; mdBook 0.5.4 and linkcheck2 0.13.0.
**Storage**: Versioned repository metadata only.
**Testing**: Existing locked Rust suite, format/Clippy, notes and Debian shell contracts, documentation/trust/brand policy, rendered documentation and production build.
**Target Platform**: Existing Windows x64/Linux x64 CI matrix.
**Project Type**: Desktop application release preparation.
**Performance Goals**: No runtime performance change.
**Constraints**: UTF-8/no BOM/LF; hidden noninteractive console launches; no merge/tag/publication; at most two reviews.
**Scale/Scope**: Three dependency updates, six rollover files, release guidance and slice/planning records.

## Constitution Check

Pre-research PASS: Owner kickoff, #302 and canonical release documentation establish scope. Full spec-kit and blocking analysis precede implementation. Existing safety-critical tests remain required; no behavior or authority expansion. The pinned release guide correction is scoped and will receive a dated changelog decision.

Post-design PASS: The existing no-tag/no-push/no-publish rollover, exact #284 patch integration, complete automated gates and owner handoff preserve all constitutional requirements. No exceptions or new architecture.

## Implementation Sequence

1. Record the existing missing-Highlights rejection, then incorporate #284's Cargo files with current-main content preserved.
2. Add concise Highlights and dependency/preparation records; reconcile the two release pages and stale agent overview; establish Plan058 and matching ledger/index state.
3. Validate the preparatory tree and make a local conventional preparation commit, leaving cargo-release a clean tracked tree.
4. Inspect `cargo release 0.19.0` dry run, then execute `cargo release 0.19.0 --execute --no-confirm`; audit every version/date change and unchanged addon versions.
5. Run required candidate gates, push the branch, publish the official PR, address findings within two rounds and establish successful final-head CI.
6. Deliver the open candidate for owner review and merge. #301 and the target tag/publication remain outstanding; #284 retirement follows confirmed integration.

## Project Structure

```text
specs/132-v0190-release/{spec,plan,research,data-model,quickstart,tasks,analysis,verification}.md
specs/132-v0190-release/checklists/{requirements,release-governance}.md
specs/132-v0190-release/contracts/release-candidate.md
Cargo.toml, Cargo.lock, CHANGELOG.md, README.md, docs/src/README.md
specs/073-reviewed-catalog-pipeline/fixtures/capture-request.json
docs/project/releasing.md, docs/src/development/release-and-packaging.md
docs/project/build-plans/{plan-058,README}.md
docs/project/migration-ledger.{json,md}
.specify/feature.json, CLAUDE.md
```

**Structure Decision**: Reuse the single crate and all existing release gates. A pre-existing failure supplies red evidence; metadata-only work needs no mirror tests or new validation framework.

## Complexity Tracking

No constitutional violation or architectural expansion. Research decisions are recorded in [research.md](research.md).
