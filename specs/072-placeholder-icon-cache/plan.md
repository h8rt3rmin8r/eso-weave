# Implementation Plan: Placeholder-First Local Icon Cache

**Branch**: `codex/s072-placeholder-icon-cache` | **Date**: 2026-09-09 |
**Spec**: [spec.md](spec.md)

**Input**: Feature specification from
`/specs/072-placeholder-icon-cache/spec.md`

## Summary

Implement issue #116 as a local-only icon cache that converts explicitly
user-supplied PNG and DDS files into deterministic content-addressed PNG
objects, binds every canonical catalog virtual path through an immutable
manifest, and uses one project-created placeholder whenever bytes are absent or
rejected. No network, archive extraction, or third-party image byte enters the
repository, catalog database, package, or release.

## Technical Context

**Language/Version**: Rust 1.96

**Primary Dependencies**: Existing `image`, `serde`, `serde_json`, `sha2`, and
`tempfile`; add `image_dds` 0.7 with default features disabled and only
`ddsfile` plus `image` enabled

**Storage**: User-local immutable cache generations, canonical JSON manifests,
and SHA-256-addressed PNG objects outside `catalog.sqlite`

**Testing**: Rust integration tests using generated synthetic PNG and DDS
fixtures, existing catalog/package tests, documentation policy, and full Cargo
merge gate

**Target Platform**: Windows 10/11 x64 and Linux x64

**Project Type**: Single Rust desktop crate and compiler-side library contract

**Performance Goals**: Linear work over selected references; no input above 8
MiB, dimension above 1024, or decoded surface above 1,048,576 pixels; no GUI
thread or network work

**Constraints**: Local user ownership, no source deletion, no symlink or path
escape, no personal paths in manifests, deterministic PNG output, immutable
publication, generic placeholder fallback

**Scale/Scope**: One local-directory adapter, two input formats, one output
format, one manifest schema, and one new cache module

## Constitution Check

*GATE: Passed before Phase 0 research and rechecked after Phase 1 design.*

- **Spec-driven development**: PASS. Issue #116, S068 rights documentation,
  Plan 038, this complete spec-kit packet, and the blocking analysis form one
  authority chain before code implementation.
- **Safety-critical surfaces**: PASS. S072 neither intercepts nor synthesizes
  input and does not touch either addon. Existing input, PixelBeacon, collector,
  and fishing safety suites remain mandatory.
- **Test-first seams**: PASS. Synthetic image, path-containment, publication,
  and resolver tests precede the cache implementation.
- **CI parity**: PASS. Formatting, strict Clippy, locked full tests,
  documentation, packaging, and repository hygiene run before commit.
- **Bounded desktop scope**: PASS. The feature reads only a user-selected local
  directory outside the game. It adds no addon, process-memory, packet,
  network, or automation path.
- **Platform/config/text constraints**: PASS. No settings schema changes are
  needed. Shared filesystem logic has platform-specific link detection only.
  Authored text remains UTF-8 without BOM and LF with forbidden dashes absent.
- **Pinned artifacts**: PASS. No pinned workflow, release, packaging, or script
  edit is planned.

Post-design recheck: PASS. The immutable manifest closes the reference-to-asset
association gap without migrating `catalog.sqlite` or storing user bytes in the
catalog. The decode dependency is safe Rust, decode-only, and avoids the native
encoding toolchain. No constitution exception is required.

## Project Structure

### Documentation (this feature)

```text
specs/072-placeholder-icon-cache/
├── analysis.md
├── checklists/
│   ├── asset-safety.md
│   └── requirements.md
├── contracts/
│   └── icon-cache-manifest.schema.json
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
├── catalog/
└── icon_cache/
    ├── manifest.rs
    ├── mod.rs
    ├── path.rs
    └── transform.rs

tests/
├── catalog_packaging.rs
└── icon_cache.rs
```

Canonical documentation updates the architecture, catalog compiler, source and
rights reference, troubleshooting, status reference, Plan 038, and changelog.

**Structure Decision**: Keep cache bytes and their authoritative mapping in a
new module and user-local filesystem generation. `catalog.sqlite` remains
immutable source metadata. This explicitly corrects the S070 design gap without
claiming user-local transformed bytes are part of a distributable catalog.

## Implementation Phases

1. Freeze the manifest schema, path rules, decoder limits, typed states, and
   synthetic test builders.
2. Implement canonical virtual-path validation and component-wise,
   case-insensitive local resolution with link and containment rejection.
3. Implement bounded PNG and DDS decode, explicit RGBA conversion,
   deterministic PNG encoding, hashing, and placeholder generation.
4. Implement object deduplication, canonical manifest construction, immutable
   generation publication, verification, and runtime resolution.
5. Update canonical documentation and chronological planning records.
6. Run spec-kit analysis, focused tests, full merge gates, CI, and authorized
   review rounds.

## Complexity Tracking

No constitution violation exists. A dedicated cache manifest is necessary
because `icon_reference` records source paths while `icon_asset` records content
metadata with no association between them. Adding a user-byte foreign key to
the distributable catalog would mix ownership and publication lifecycles. The
external immutable manifest keeps that authority explicit and local.
