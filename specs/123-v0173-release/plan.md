# Implementation Plan: S123 v0.17.3 release preparation

**Branch**: `codex/s123-v0173-release` | **Date**: 2026-10-06 | **Spec**: [spec.md](spec.md)

## Summary

Prepare the v0.17.3 candidate through the existing local cargo-release rollover, retain S120-S122 history, archive completed Plan048, and track preparation in Plan049 and issue #255. Publication and installed/game checks are outside this slice.

## Technical Context

**Language/Version**: Existing pinned Rust toolchain; Markdown and repository release scripts.
**Primary Dependencies**: Existing cargo-release configuration, mdBook and documentation policy gates; no dependency changes.
**Storage**: Versioned repository documents and metadata.
**Testing**: Existing release-note, documentation, format, clippy, Rust test and release-build gates.
**Target Platform**: Existing Windows and Linux CI targets.
**Project Type**: Desktop application release preparation.
**Performance Goals**: No runtime behavior changes.
**Constraints**: UTF-8 without BOM, LF, no tag/publish/merge, maximum two requested Codex rounds.
**Scale/Scope**: Five synchronized identity surfaces, changelog, Plan048 archive and Plan049 current plan.
**Resolved integration**: Installed cargo-release 1.1.2; use `--no-confirm` with closed stdin. Canonical release procedure supersedes older release specs.

## Constitution Check

Pre-research gate PASS: specification precedes implementation; scope follows owner authorization; runtime/input safety contracts remain unchanged; existing release procedure governs rollover; automated gates remain intact; no new dependencies or architecture; no installed/game verification. No exceptions requested.

Post-design gate PASS: no runtime/source or pinned-tool changes; existing policy validates archival lifecycle and identity. Research resolved every integration question. Specification, release contract and tasks preserve owner boundaries.

## Implementation Sequence

1. Prove the existing release-note gate rejects missing Highlights, then add bounded candidate notes.
2. Archive Plan048, repair references and the machine/prose migration ledger, and create Plan049. Preserve chronology and unused reservations.
3. Run existing baseline gates and commit preparatory changes so cargo-release receives a clean tracked tree.
4. Inspect the dry run, execute `cargo release 0.17.3 --execute --no-confirm`, and inspect its identity commit. The configured command cannot tag, push or publish.
5. Run candidate gates and repository hygiene, publish the official PR, satisfy all findings and head-specific checks, and hand off the unmerged candidate.

See [research](research.md), [data model](data-model.md), [release contract](contracts/release-candidate.md) and [validation guide](quickstart.md).

## Project Structure

```text
specs/123-v0173-release/
  spec.md
  plan.md
  research.md
  data-model.md
  quickstart.md
  contracts/release-candidate.md
  tasks.md
Cargo.toml
Cargo.lock
CHANGELOG.md
README.md
docs/src/README.md
specs/073-reviewed-catalog-pipeline/fixtures/capture-request.json
docs/project/build-plans/README.md
docs/project/build-plans/plan-049.md
docs/project/migration-ledger.json
docs/project/migration-ledger.md
docs/project/documentation-visualization-audit.md
docs/archive/build-plans/plan-048.md
docs/archive/build-plans/README.md
```

**Structure Decision**: Use existing release and planning paths. Add only S123 specification artifacts and the next chronological build plan.

## Complexity Tracking

No constitutional violations or additional architecture.
