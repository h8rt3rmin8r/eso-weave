# Implementation Plan: Versioned Encounter Metrics

**Branch**: `codex/s077-encounter-metrics` | **Date**: 2026-09-10 | **Spec**: `spec.md`  
**Input**: Feature specification for issue #134

## Summary

Add a deterministic derived-analysis module over the validated S076 raw store.
It loads one immutable capture, verifies an exact channel/API catalog, calculates
the five S069 baseline metrics with explicit loss quality, creates a catalog join
receipt, and atomically publishes canonical JSON. A maintainer CLI command exposes
the reusable library contract.

## Technical Context

**Language/Version**: Rust 1.96, edition 2021  
**Primary Dependencies**: existing serde, serde_json, sha2, rusqlite, tempfile  
**Storage**: read-only `encounters.sqlite`, read-only `catalog.sqlite`, explicit
canonical projection JSON  
**Testing**: unit and integration tests plus complete locked Cargo and docs gates  
**Target Platform**: Windows and Linux desktop and CI  
**Performance Goals**: bounded linear work over at most 100,000 imported events  
**Constraints**: no network, no raw/catalog writes, deterministic bytes, explicit
loss, exact channel/API match, no sensitive payload output  
**Scale/Scope**: one encounter per invocation, five metric families, one receipt

## Constitution Check

- Full spec-kit sequence precedes implementation.
- The feature is local, non-interactive, and separated from gameplay authority.
- Raw observations and catalog data remain immutable authorities.
- Derived output is versioned, explicit, atomic, and rebuildable.
- Test-first tasks precede implementation tasks.
- Full Cargo and documentation parity are mandatory before publication.
- No dependency, workflow, release, packaging, license, or toolchain change is planned.

**Gate result**: PASS.

## Project Structure

```text
specs/077-encounter-metrics/
  spec.md
  plan.md
  research.md
  data-model.md
  contracts/encounter-metrics-contract.md
  checklists/requirements.md
  checklists/metric-integrity.md
  quickstart.md
  tasks.md
  analysis.md
  fixtures/encounter-metrics-capture.json

src/encounter/metrics.rs
src/encounter/mod.rs
src/bin/catalog-compiler.rs
tests/encounter_metrics.rs
docs/src/reference/encounter-data-and-metrics.md
docs/project/build-plans/plan-038.md
CHANGELOG.md
```

**Structure Decision**: Keep calculation under the encounter domain but expose a
separate projection API and file artifact. Reuse the raw-store loader, catalog
reader, bounded path checks, and atomic publisher.

## Implementation Phases

1. Freeze specification, research, model, contract, checklists, tasks, and the
   blocking consistency analysis.
2. Add failing tests for values, ordering, loss, zero denominators, exact catalog
   compatibility, later resolution, determinism, and no-clobber.
3. Implement typed projection models and pure sequence-ordered calculations.
4. Add catalog receipt construction and atomic canonical JSON publication.
5. Expose `encounter-project` through the maintainer CLI.
6. Update canonical docs, plan chronology, changelog, and issue evidence.
7. Run full CI parity and repository hygiene checks, then publish and review.

## Complexity Tracking

| Choice | Why needed | Simpler option rejected |
| --- | --- | --- |
| Explicit JSON projection artifact | Proves reproducibility without guessing UI storage needs | Tables in the raw store violate plane ownership; a second database pre-empts #135 |
| Source type 1 for player attribution | Actor IDs depend on observation order | Actor 1 can silently misattribute incoming-first encounters |
| Duration-anchored effect interval | Raw capture omits absolute clock origin and effect slot | Treating API begin/end as elapsed encounter time creates false clipping |

## Post-Design Constitution Check

The contract keeps inputs read-only, output explicit and atomic, algorithms
versioned, unknown IDs visible, and synthetic evidence distinct from live parity.
No constitutional exception is required.
