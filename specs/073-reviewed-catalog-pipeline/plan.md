# Implementation Plan: Reviewed Catalog Candidate Pipeline

**Branch**: `codex/s073-reviewed-catalog-pipeline` | **Date**: 2026-09-09 |
**Spec**: [spec.md](spec.md)

**Input**: Feature specification from
`/specs/073-reviewed-catalog-pipeline/spec.md`

## Summary

Implement issue #117 as one maintainer-only pipeline that consumes an explicit
versioned request, verifies content-pinned local or approved HTTPS sources,
imports S071 captures when selected, builds and verifies an S070 catalog,
resolves S072 icon references into a separate local cache, applies review
thresholds, and publishes an immutable review-safe candidate. Add a read-only,
pinned GitHub Actions workflow for manual and scheduled candidate generation.
No path installs, accepts, commits, releases, or promotes a candidate.

## Technical Context

**Language/Version**: Rust 1.96

**Primary Dependencies**: Existing `serde`, `serde_json`, `sha2`, `tempfile`,
`rusqlite`, and `ureq`; standard library filesystem APIs

**Storage**: Content-addressed source cache, separate user-local S072 icon cache,
temporary build workspace, and immutable review candidate directories

**Testing**: Rust integration tests with invented local and mock-network inputs,
existing catalog/collector/icon suites, workflow policy tests, documentation
policy, and full Cargo merge gates

**Target Platform**: Windows 10/11 x64 and Linux x64

**Project Type**: Single Rust crate, existing `catalog-compiler` maintainer CLI,
and one least-privilege GitHub Actions workflow

**Performance Goals**: Linear work over at most 64 MiB and 500,000 catalog or
collector records, at most 128 pinned sources, and at most 500,000 icon
references; all network calls bounded by 15 seconds and per-source byte limits

**Constraints**: Explicit channel identity, no PTS promotion, no active install,
no source execution, no user paths or bytes in candidates, stable no-follow
local reads, redirect-free HTTPS, exact hashes, atomic no-clobber publication

**Scale/Scope**: Four request modes, one request schema, one candidate schema,
one orchestrator module, two CLI commands, and one review-only workflow

## Constitution Check

*GATE: Passed before Phase 0 research and rechecked after Phase 1 design.*

- **Spec-driven development**: PASS. Issue #117, Plan 038, S068 through S072,
  this complete packet, and the blocking analysis form one authority chain.
- **Safety-critical surfaces**: PASS. S073 changes neither input automation nor
  either addon lifecycle. Existing input, PixelBeacon, collector, and fishing
  safety suites remain mandatory.
- **Test-first seams**: PASS. The source fetcher is a trait, and failing request,
  acquisition, channel, threshold, privacy, atomicity, and workflow tests precede
  implementation.
- **CI parity**: PASS. Format, strict Clippy, locked full tests, optimized builds,
  documentation, workflow policy, package, and text hygiene run before commit.
- **Bounded desktop scope**: PASS. The pipeline runs outside the game and only
  composes the approved collector handoff. It adds no process-memory, packet,
  input, upload, or new addon path.
- **Platform/config/text constraints**: PASS. Application configuration is
  unchanged. Files remain UTF-8 without BOM and LF, with forbidden dashes absent.
- **Pinned artifacts**: PASS with recorded decision. The new review workflow is
  required by #117, uses SHA-pinned actions and read-only permissions, and is
  recorded in `CHANGELOG.md`.

Post-design recheck: PASS. Source bytes and local icon objects remain outside
the candidate, while hashes and redacted receipts make the review bundle
reproducible. Candidate acceptance remains a separate human action. No
constitution exception is required.

## Project Structure

### Documentation (this feature)

```text
specs/073-reviewed-catalog-pipeline/
├── analysis.md
├── checklists/
│   ├── pipeline-safety.md
│   └── requirements.md
├── contracts/
│   ├── candidate-manifest.schema.json
│   └── pipeline-request.schema.json
├── data-model.md
├── fixtures/
│   ├── empty-source.bin
│   └── offline-live-request.json
├── plan.md
├── quickstart.md
├── research.md
├── spec.md
└── tasks.md
```

### Source Code (repository root)

```text
.github/workflows/catalog-candidate.yml

src/
├── bounded_file.rs
├── bin/catalog-compiler.rs
├── catalog/version.rs
└── catalog_pipeline/
    ├── acquire.rs
    ├── manifest.rs
    └── mod.rs

tests/
├── catalog_pipeline.rs
└── catalog_pipeline_workflow.rs
```

Canonical documentation updates architecture, catalog compiler, source rights,
test strategy, status reference, Plan 038, the build-plan index, and changelog.

**Structure Decision**: Keep orchestration in a dedicated library module and
extend the existing maintainer binary rather than creating another executable
or Cargo workspace. Extract the S072 stable bounded reader into a crate-private
module so the new hostile local-input boundary does not duplicate platform
filesystem logic.

## Implementation Phases

1. Freeze request, source inventory, finding, candidate, and CLI contracts.
2. Add failing request validation, mode/channel, acquisition, cache, privacy,
   threshold, publication, and workflow tests.
3. Extract and extend stable bounded local-file handling for shared use.
4. Implement local and approved HTTPS source acquisition plus immutable cache.
5. Implement request validation, S071 import, S070 compile/verify/diff, S072 icon
   receipt, blocking review thresholds, and canonical reports.
6. Implement complete candidate verification and immutable no-clobber
   publication plus CLI inspection.
7. Add pinned, read-only manual/scheduled workflow and machine-enforced policy.
8. Update canonical documentation and chronological project records.
9. Run spec-kit analysis, focused tests, complete local gates, authorized PR CI,
   and no more than two Codex review rounds.

## Complexity Tracking

No constitution violation exists. The workflow is a pinned artifact but is an
explicit requirement of issue #117 and its read-only design narrows authority.
The shared bounded-file module is a proportional extraction from S072 because
S073 otherwise needs the same platform-specific no-follow and final-handle path
checks at multiple new hostile file boundaries.
