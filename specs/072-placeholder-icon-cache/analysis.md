# S072 Spec-kit Analysis

## Pre-implementation gate

Result: PASS.

The specification, plan, research, data model, manifest schema, quickstart,
requirements checklist, asset-safety checklist, and tasks are mutually
consistent and implementable without unresolved clarification.

## Coverage matrix

| Concern | Requirement authority | Planned evidence |
| --- | --- | --- |
| Placeholder-first behavior | FR-010, FR-011, FR-020 | Missing, corrupt, unsupported, and permission fixtures |
| Local-only acquisition | FR-003 through FR-005 | Path, link, containment, and no-network scans |
| Bounded decoding | FR-006 through FR-009 | PNG/DDS byte, dimension, layer, depth, and pixel tests |
| Explicit association | FR-014, FR-022 | Canonical manifest mapping tests |
| Determinism and deduplication | FR-013, FR-017 | Repeated generation and duplicate-pixel tests |
| Immutable publication | FR-012, FR-018, FR-019 | Injected failure and existing-generation tests |
| Provenance and privacy | FR-015, FR-016, FR-023 | Manifest assertions and package scans |
| Catalog independence | FR-021 | Existing compiler/runtime/package suites |

## Architecture findings

1. S070 modeled icon references and icon assets independently. That supports a
   rights-compatible empty baseline but cannot resolve one reference to one
   transformed object. S072 makes the local manifest the explicit association
   authority. It does not infer by filename and does not migrate user-local
   availability into `catalog.sqlite`.
2. General recursive directory indexing would exceed the selected-reference
   boundary and inspect unrelated user files. Component-wise resolution is
   bounded by requested references and rejects ambiguous case matches.
3. Mutable current-generation replacement is not uniformly atomic on Windows
   and Linux. Immutable content-hash generation publication is the simpler
   cross-platform contract. Later update orchestration may select one verified
   generation in process without mutating it.
4. The image decoder dependency must exclude encoding features. This removes
   the native ISPC toolchain and keeps only safe decode functionality.

## Constitution gate

- Full spec-kit sequence exists before production implementation.
- No input, addon, process-memory, packet, network, or gameplay surface changes.
- Test-first tasks cover all new hostile file boundaries.
- Full CI parity remains mandatory before each Rust commit.
- No pinned artifact change or constitution exception is required.

No CRITICAL, HIGH, duplicated requirement, unresolved scope ambiguity,
placeholder token, or constitution conflict remains.

## Post-implementation gate

Result: PASS.

The implementation matches every functional requirement and keeps the local
manifest, rather than the distributable catalog, as the association authority.
Review added explicit rejection for link-like cache directories and objects in
addition to the planned source-tree checks. First-round automated review then
identified two further gaps: a check/read race and APNG default-frame
acceptance. S072 now verifies one stable no-follow source handle before a hard-
capped read and rejects APNG before decoding. These proportional changes close
all material findings from implementation and external analysis.

Focused cache, catalog, packaging, and documentation-policy tests pass. Full
format, strict all-feature Clippy, locked tests, optimized binary builds, mdBook
test/build, generated-site policy, JSON parsing, diff hygiene, encoding, dash,
mojibake, decoder-feature, and spec-kit checks also pass. No requirement,
contract, task, or documentation conflict remains before delivery.
