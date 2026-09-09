# Implementation Plan: User-Initiated Catalog Updates

**Branch**: `codex/s074-user-catalog-updates` | **Date**: 2026-09-09 |
**Spec**: [spec.md](spec.md)

**Input**: Feature specification from
`/specs/074-user-catalog-updates/spec.md`

## Summary

Complete issue #118 by adding one shared catalog availability model, a
background startup scan, a user-controlled update coordinator, immutable
user-data candidate installation, an atomic active-selection contract, verified
rollback, redacted receipts, collector-assisted local builds, and an accessible
egui update modal. Reuse S070 verification, S071 collector confinement, S072
placeholder handling, and the full S073 candidate verifier. Do not add a remote
candidate feed or any automatic update authority. Describe candidate hashes as
integrity evidence, not authenticated origin, and require explicit acknowledgement.

## Technical Context

**Language/Version**: Rust 1.96

**Primary Dependencies**: Existing `serde`, `serde_json`, `sha2`, `tempfile`,
`rusqlite`, `eframe`, `ureq`, and standard library filesystem, file-locking,
thread, atomic, and channel APIs

**Storage**: User-data `catalog/` root containing import, staging, immutable
Live versions, receipts, source cache, icon cache, an operation lock, and one
canonical atomic `selection.json`; bundled catalog remains read-only fallback

**Testing**: Pure availability matrix tests, hostile filesystem and failure
injection integration tests, worker/controller tests, collector confinement
tests, headless egui accessibility and sizing tests, canonical contract tests,
existing safety suites, documentation policy, and full Cargo merge gates

**Target Platform**: Windows 10/11 x64 and Linux x64

**Project Type**: Single Rust crate with the existing eframe desktop binary

**Performance Goals**: First window never waits for network or candidate scans;
all catalog work stays on one worker; files retain S073 byte caps; UI polling is
non-blocking; candidate copies are linear and progress uses exact artifact bytes

**Constraints**: Explicit user action, Live/PTS separation, no silent downloads,
no install-directory mutation, no path disclosure, one cross-process file lock,
same-filesystem atomic publication, atomic pointer replacement, cooperative
cancellation before the commit boundary, verified first open, safe fallback

**Scale/Scope**: One availability resolver, one update service and worker, two
small JSON contracts, one update modal, reviewed-candidate and collector-source
paths, and updates to startup/catalog selection and canonical documentation

## Constitution Check

*GATE: Passed before Phase 0 research and rechecked after Phase 1 design.*

- **Spec-driven development**: PASS. Issue #118, parent #111, S070 through S073,
  this packet, and the blocking analysis form one authority chain.
- **Safety-critical surfaces**: PASS. Collector operations call the existing
  separately managed lifecycle. PixelBeacon and every input/fishing invariant
  remain unchanged and their suites remain mandatory.
- **Test-first seams**: PASS. Availability is pure; filesystem actions accept a
  progress/cancellation seam; the worker communicates through channels; failure
  injection tests precede each install, rollback, and recovery path.
- **CI parity**: PASS. Full format, strict Clippy, locked tests, optimized binary,
  documentation, text hygiene, and hosted CI gates run before publication.
- **Bounded desktop scope**: PASS. The desktop reads only reviewed candidates or
  explicit flushed captures, never memory, packets, uploads, or new addon data.
- **Platform/config/text constraints**: PASS. Derived catalog selection and
  receipts live outside user settings. Contracts are UTF-8, LF, canonical JSON,
  and contain no local paths.
- **Pinned artifacts**: PASS. No pinned workflow, release, or packaging artifact
  is required. If implementation evidence changes that, the changelog decision
  gate applies before commit.

Post-design recheck: PASS. The selection record stores only target identities,
never paths. Cross-process exclusivity uses standard-library file locking. The
immutable candidate remains the verification authority, and pointer replacement
is the only commit boundary. No constitution exception is required.

## Project Structure

### Documentation (this feature)

```text
specs/074-user-catalog-updates/
├── analysis.md
├── checklists/
│   ├── requirements.md
│   └── update-safety.md
├── contracts/
│   ├── selection.schema.json
│   └── update-receipt.schema.json
├── data-model.md
├── plan.md
├── quickstart.md
├── research.md
├── spec.md
└── tasks.md
```

### Source Code (repository root)

```text
src/
├── app/
│   ├── mod.rs
│   ├── strings.rs
│   └── ui.rs
├── beacon/api_check.rs
├── catalog/
│   ├── mod.rs
│   └── version.rs
├── catalog_pipeline/
│   ├── manifest.rs
│   └── mod.rs
├── catalog_update/
│   ├── availability.rs
│   ├── contract.rs
│   ├── mod.rs
│   └── worker.rs
├── lib.rs
└── main.rs

tests/
├── app_catalog_update.rs
├── app_ui_sizing.rs
├── catalog_runtime.rs
└── catalog_update.rs
```

Canonical documentation updates architecture, catalog compiler/runtime, the
candidate pipeline handoff, status reference, troubleshooting, test strategy,
and changelog.

**Structure Decision**: Keep correctness-bearing update logic in a dedicated
library module. The app model owns only UI intent and a worker handle; it does
not perform filesystem or database work on the GUI thread. Extend S073 with one
public redacted inspection result instead of duplicating its private manifest
and report parsers.

## Implementation Phases

1. Freeze selection, receipt, availability, progress, and public candidate
   inspection contracts.
2. Add failing availability, selection, install, rollback, cancellation,
   concurrency, recovery, privacy, collector, and UI tests.
3. Expose a redacted S073 candidate inspector backed by the complete verifier.
4. Implement roots, strict canonical contract parsing, cross-process locking,
   import discovery, immutable copy, first-open validation, atomic selection,
   receipts, recovery, and bundled fallback.
5. Implement the progress/cancellation worker and collector-assisted S073 build
   adapter.
6. Converge startup API evidence and catalog state in one availability resolver,
   then switch startup selection to verified user-data-first resolution.
7. Add the dismissible notice and accessible responsive modal, including Live
   and PTS summaries, progress, cancellation, rollback, collector instructions,
   cleanup, and diagnostics.
8. Update canonical documentation and chronological decision records.
9. Run spec-kit analysis, focused suites, all local gates, authorized PR CI, and
   no more than two Codex review rounds.

## Complexity Tracking

No constitution violation exists. A dedicated module is warranted because
candidate verification, active selection, rollback, worker lifecycle, and UI
projection have different ownership and failure boundaries. The design adds no
runtime dependency or remote service. The only shared extraction allowed is a
small atomic-file replacement helper if the existing compiler and collector
implementations would otherwise be copied a third time.
