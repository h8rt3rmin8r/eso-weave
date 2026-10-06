# Implementation Plan: S122 Authority Maps

**Branch**: `codex/s122-catalog-and-extension-authority-maps`
**Date**: 2026-10-06 | **Spec**: [spec.md](spec.md)

## Summary

Deliver #221 and #223 as two owned SVGs with full adjacent equivalents, exact
source-contract records and shared source/generated/browser coverage. Regroup
Plan048's unused S113/S114 reservations at this owner-authorized kickoff.

## Technical Context

- Language: Markdown, static annotated SVG and dependency-free Node ESM.
- Dependencies: existing mdBook 0.5.4, linkcheck2 0.13.0, typos 1.50.1 and host
  Chrome. No dependency, workflow, toolchain or runtime change.
- Storage: owned documentation assets; existing finite content-coverage manifest.
- Testing: policy mutation tests, renderer receipt mutations, mdBook examples,
  build/link/source/generated checks, headless browser paint/topology/zoom.
- Platform: public and bundled offline documentation, desktop and narrow reading.
- Constraints: UTF-8 no BOM, LF, no long dashes; hidden noninteractive processes.
- Scope: two figures; no application/addon/data/transport/release changes.

## Constitution Check

Pre-research and post-design: PASS. Issue/corpus traceability and complete
spec-kit artifacts satisfy I; existing safety tests are retained (II). Negative
policy tests precede implementation (III). Automated fmt/clippy/full Rust tests
run for autopilot parity despite no Rust source change (IV). Current managed
addon and local-only capture boundaries are depicted without changes (V).
Publication is authorized; merge/release/field checks are excluded. Existing
documentation scripts receive bounded inventory/zoom coverage changes.

## Phase 0: Research

Two read-only research agents traced current catalog and local-extension code,
tests and prose. Consolidated decisions are in [research.md](research.md).
No unknown remains. Historical specs do not override current production behavior.

## Phase 1: Design

- Catalog: two alternative input nodes converge on normalization; compile and
  review; Live explicit trust/install path proceeds to atomic selection and
  rollback. PTS branches to a terminal preview node, with no selection edge.
- Local interface: non-secret discovery and separately copied credential are
  qualifications at the client boundary, one loopback admission/generation,
  HTTP/MCP branches, one shared authority boundary, then parallel canonical
  state and bounded query service/fixed-store nodes.
- Vertical narrow SVGs, orthogonal explicitly annotated edges, stage gaps,
  high-contrast text and no external SVG resources. Text equivalent preserves
  relationships, shared semantics and operational detail beyond concise labels.
- Narrow 200 percent probes exposed modal label shrinkage in the initial
  15-unit design. Both new figures now use minimum 24-unit labels and full-width
  vertically offset alternative/parallel nodes, linked directly to their shared
  destination with independent side routes. No viewer behavior changes.
- Extend existing finite policy inventory from six to eight SVGs; no viewer or
  theme change. Generalize the existing zoom probe for the two new maps while
  retaining the encounter lineage probe and all prior checks.
- Record exact source authorities and update triggers in contracts and maintained
  docs records; [data-model.md](data-model.md) defines figure metadata.
- [quickstart.md](quickstart.md) defines automated end-to-end validation only.

## Project Structure

`specs/122-catalog-and-extension-authority-maps/` holds spec, requirements/domain
checklists, research, model, contracts, quickstart, tasks and verification.
Assets live in `docs/src/assets/diagrams/`; authority pages are
`docs/src/development/catalog-candidate-pipeline.md` and
`docs/src/reference/local-api-and-mcp.md`. Existing `.github/scripts/docs-*`
checks and `docs/project/` inventories are updated in the same slice.

## Decisions and Alternatives

One combined figure would blur separate reader questions; use two placements.
Horizontal figures would shrink narrow-screen labels; use bounded vertical
topology plus existing expansion and full prose. Duplicate per-page catalog
figures would drift; link the single lifecycle from supporting pages.
New SVG runtime or renderer dependencies are unnecessary; reuse owned static
assets and existing host browser. Do not change historical S104 discovery prose;
use the current authenticated-operation/non-secret-discovery contract.
No complexity exception is needed.
