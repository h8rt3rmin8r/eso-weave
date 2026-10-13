# Implementation Plan: S128 v0.18.0 Release Preparation

**Branch**: `codex/s128-v0180-release` | **Date**: 2026-10-13 | **Spec**: [spec.md](spec.md)
**Input**: S128 / issue #279, owner-authorized automatic push and official PR with at most two review rounds.

## Summary

Prepare the S125-S127 candidate using the existing cargo-release rollover, preserve complete change history, archive Plan053 and direct preparation through Plan054. No runtime or pipeline changes are required.

## Technical Context

**Language/Version**: Existing Rust 1.96 toolchain plus Markdown/JSON metadata.
**Primary Dependencies**: Existing cargo-release, mdBook 0.5.4 and mdbook-linkcheck2 0.13.0; no dependency changes.
**Storage**: Versioned repository metadata and documents.
**Testing**: Existing release notes, documentation/render, trust/brand, format, Clippy, locked Rust suite and release-profile bundled documentation build.
**Target Platform**: Existing Windows x64 and Linux x64 CI matrix.
**Project Type**: Desktop release candidate preparation.
**Performance Goals**: No runtime behavior changes.
**Constraints**: UTF-8 without BOM, LF; hidden noninteractive console launches; no merge/tag/publication or field request; at most two review rounds.
**Scale/Scope**: Six rollover files (Cargo.toml, Cargo.lock, CHANGELOG.md, README.md, docs/src/README.md, capture-request.json), specification and chronological planning.

## Constitution Check

Pre-research PASS: Spec precedes implementation and maps to #279/canonical release procedure. Existing safety-critical tests remain mandatory. No architecture, runtime, dependency or pinned process change; all required automated gates and owner publication boundaries apply. No exceptions.

Post-design PASS: research resolved all integration questions, and the candidate contract/data model/tasks preserve all identity, history, issue and owner boundaries. No runtime source, dependency, pinned tool or process changes are required.

## Implementation Sequence

1. Use the existing missing-Highlights rejection as red evidence, then add compact notes for all three slices.
2. Archive Plan053 with actual PR #276 merge evidence; update relative links, indexes and machine/prose ledger, then create Plan054.
3. Validate preparatory policy, text hygiene, notes and Rust gates before the local prep commit, leaving cargo-release a clean tracked tree.
4. Inspect configured dry run; execute `cargo release 0.18.0 --execute --no-confirm`. Inspect exact version/date changes and unchanged dependency graph.
5. Run candidate release/documentation/render/Rust/build gates, publish official PR, address all findings within two rounds and wait for final-head green CI.
6. Deliver the unmerged, untagged candidate for owner review and merge.

## Project Structure

```text
specs/128-v0180-release/{spec,plan,research,data-model,quickstart,tasks,analysis,verification}.md
specs/128-v0180-release/checklists/{requirements,release-governance}.md
specs/128-v0180-release/contracts/release-candidate.md
Cargo.toml, Cargo.lock, CHANGELOG.md, README.md, docs/src/README.md
specs/073-reviewed-catalog-pipeline/fixtures/capture-request.json
docs/archive/build-plans/{plan-053,README}.md
docs/project/build-plans/{plan-054,README}.md
docs/project/migration-ledger.{json,md}
.specify/feature.json and managed CLAUDE.md plan pointer
```

**Structure Decision**: Reuse existing release/planning paths; add only the numbered S128 artifacts and Plan054.

## Complexity Tracking

No constitutional violation or additional architecture.
