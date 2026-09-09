# Implementation Plan: External Encounter Model

**Branch**: `codex/s069-encounter-model` | **Date**: 2026-09-09 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/069-encounter-model/spec.md`

## Summary

Complete the repository-verifiable portion of issue #113 with a validated,
machine-readable encounter contract, a deterministic synthetic encounter and
metric projection, canonical privacy and storage guidance, a Combat Metrics
parity roadmap, and ordered downstream implementation issues. Keep live
same-parse comparison in a separate release-verification issue.

## Technical Context

**Language/Version**: Markdown, JSON, JavaScript on Node 24

**Primary Dependencies**: Existing documentation policy and Node test runner;
Node standard library only

**Storage**: Version-controlled JSON design contract and synthetic fixtures;
future runtime storage remains a user-owned database separate from
`catalog.sqlite`

**Testing**: Node fixture tests, production documentation policy, mdBook and
linkcheck, spelling, JSON parsing, text hygiene

**Target Platform**: Repository and bundled documentation on Windows and Linux

**Project Type**: Data-model and governance contract for a desktop companion

**Performance Goals**: Deterministic linear projection over an ordered encounter;
no runtime application impact

**Constraints**: No game process or packet access; no production capture or
import implementation; no bulk encounter traffic over Pixel Bus; local-only by
default; UTF-8 without BOM, LF, and no forbidden dash characters

**Scale/Scope**: One synthetic encounter, fourteen required event kinds, five
minimum metric projections, three storage planes, and five ordered handoffs

## Constitution Check

*GATE: Passed before research and design. Re-check after implementation.*

- **Spec-first traceability**: PASS. Issue #113 maps to S069 and active Plan 038.
- **Safety invariants**: PASS. Pixel Bus and action-driving automation remain unchanged.
- **Test-first delivery**: PASS planned. Contract and projection tests fail before implementation.
- **CI parity**: PASS by scope. No Rust source changes, so documentation and Node policy gates replace Cargo gates.
- **Bounded scope**: PASS. The slice models local observations and uses public addon sources plus synthetic data.
- **Configuration discipline**: PASS. No application or session configuration changes.
- **Text hygiene**: PASS by design with automated final audit.

## Project Structure

```text
specs/069-encounter-model/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── checklists/
│   ├── requirements.md
│   └── encounter-governance.md
├── contracts/
│   └── encounter-model-contract.md
├── fixtures/
│   ├── dummy-encounter.json
│   └── dummy-projection.json
└── tasks.md
```

```text
.github/scripts/docs-policy.{mjs,test.mjs}
docs/project/{encounter-model.json,migration-ledger.json,migration-ledger.md}
docs/project/build-plans/{README.md,plan-038.md}
docs/src/{SUMMARY.md,reference/README.md,reference/external-sources.md,reference/encounter-data-and-metrics.md}
CHANGELOG.md
```

**Structure Decision**: Extend the existing documentation policy with exported
pure validation and projection functions. Keep the consumable authority under
`docs/project`, publish a canonical reader-facing guide, and retain fixtures in
the S069 evidence package.

## Delivery Sequence

1. Pin source evidence and settle transport, order, loss, privacy, and raw versus derived decisions.
2. Complete the spec-kit artifact chain and pre-implementation analysis.
3. Move issue #113 through Specced to In progress with Slice S069.
4. Add failing validator and projector tests.
5. Add the authority, synthetic encounter, projection, and pure implementation.
6. Publish canonical documentation and project ledgers.
7. Create one live parity verification issue and ordered capture, import, calculation, UI, and recommendation issues.
8. Run post-implementation analysis and all proportional gates.
9. Publish the authorized PR, resolve reviews, and stop for the merge ritual.

## Design Decisions

1. Live Combat Metrics parity is separate because synthetic evidence cannot prove live equivalence.
2. Pixel Bus remains a small safety and action-observation surface. A future addon and bounded SavedVariables import own bulk encounter capture.
3. Raw observations are immutable; derived metrics are versioned and rebuildable.
4. `(session_id, sequence)` is authoritative for identity and order; monotonic milliseconds drive durations.
5. Actor identity is encounter-local. Account names, character names, chat, and location are omitted by default.
6. Synthetic fixture sizes are exact. Broader storage estimates remain provisional until live verification.

## Complexity Tracking

No constitution violations require justification.
