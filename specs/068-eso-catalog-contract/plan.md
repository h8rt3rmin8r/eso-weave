# Implementation Plan: ESO Catalog Source Contract

**Branch**: `codex/s068-eso-catalog-contract` | **Date**: 2026-09-09 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/068-eso-catalog-contract/spec.md`

## Summary

Complete the repository-verifiable portion of issue #112 with a validated
machine-readable category matrix, pinned live and PTS evidence, a canonical
source and redistribution guide, and explicit compiler, collector, and icon
handoffs. Keep game-dependent experiments in a separate verification issue and
leave every unproved completeness claim fail-closed.

## Technical Context

**Language/Version**: Markdown, JSON schema-shaped data, JavaScript on Node 24

**Primary Dependencies**: Existing documentation policy and Node test runner; no
new production or development dependency

**Storage**: Version-controlled JSON research contract and Markdown

**Testing**: Node fixture tests, production documentation policy, mdBook and
linkcheck, spelling, JSON parsing, text hygiene

**Target Platform**: Repository and bundled documentation on Windows and Linux

**Project Type**: Documentation and governance contract for a desktop companion

**Performance Goals**: Constant-time per-row validation over 15 category groups;
no runtime application impact

**Constraints**: No game process or packet access; no bulk archive extraction;
no icon-byte redistribution; no live game experiment claims; UTF-8 without BOM,
LF, and no forbidden dash characters

**Scale/Scope**: 15 catalog category groups, two game channels, six source
families, one collector envelope, and three downstream implementation handoffs

## Constitution Check

*GATE: Passed before research and design. Re-check after implementation.*

- **Spec-first traceability**: PASS. Issue #112 maps to S068 and Plan 038 before implementation.
- **Safety invariants**: PASS. No input, PixelBeacon lifecycle, or action-driving runtime code changes.
- **Test-first delivery**: PASS planned. Contract validation fails before the machine-readable matrix is added.
- **CI parity**: PASS by scope. No Rust source changes, so documentation and Node policy gates replace the Cargo gate under Constitution IV.
- **Bounded scope**: PASS. Research uses public addon APIs, published source, and documentation only. It does not read game memory or packets.
- **Configuration discipline**: PASS. No application or session configuration changes.
- **Text hygiene**: PASS by design with automated final audit.

## Project Structure

### Documentation

```text
docs/project/build-plans/plan-038.md
docs/project/catalog-sources.json
docs/src/reference/catalog-sources-and-rights.md
specs/068-eso-catalog-contract/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── checklists/
│   ├── requirements.md
│   └── source-governance.md
├── contracts/
│   └── catalog-source-contract.md
└── tasks.md
```

### Repository changes

```text
.github/scripts/docs-policy.{mjs,test.mjs}
docs/project/{catalog-sources.json,migration-ledger.json,migration-ledger.md}
docs/project/build-plans/{README.md,plan-038.md}
docs/src/{SUMMARY.md,reference/README.md,reference/external-sources.md,reference/catalog-sources-and-rights.md}
CHANGELOG.md
```

**Structure Decision**: Keep the consumable source matrix in the maintainer
lifecycle, publish a human-readable canonical reference, and extend the existing
documentation policy instead of creating a second validation framework.

The frozen S059 `content-coverage.json` semantic projection remains unchanged.
S068 adds its reference through mdBook navigation and the generated search index
because expanding the frozen baseline manifest would invalidate its preservation
contract rather than improve S068 coverage.

## Complexity Tracking

No constitution violations require justification.
