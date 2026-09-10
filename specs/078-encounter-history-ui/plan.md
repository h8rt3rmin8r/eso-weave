# Implementation Plan: Quality-Aware Encounter History UI

**Branch**: `codex/s078-encounter-history-ui` | **Date**: 2026-09-10 | **Spec**: `spec.md`
**Input**: Feature specification for issue #135

## Summary

Add an explicit Encounter History window backed by one app-owned raw store. A
synchronous encounter history service composes the existing S076 lifecycle and
S077 projection contracts. A single background worker serializes import, list,
detail, and deletion work outside the render thread. The UI retains raw summaries
when derived detail fails, labels all metrics as observed, and exposes versions,
catalog coverage, unknown IDs, quality, and exact declared loss.

## Technical Context

**Language/Version**: Rust 1.96, edition 2021
**Primary Dependencies**: existing eframe/egui, serde, rusqlite, sha2
**Storage**: app-owned `encounters/encounters.sqlite`; in-memory selected projection
**Testing**: service integration tests, worker tests, headless egui interaction tests
**Target Platform**: Windows 10 and 11 x64 and Linux x64
**Performance Goals**: no encounter I/O or metric calculation on the render thread
**Constraints**: explicit local import, no scanning, no network, no telemetry, no
derived database, no raw/catalog mutation during calculation, confirmed deletion
**Scale/Scope**: one store, deterministic summary list, one selected detail, five
metric families, one serialized worker queue

## Constitution Check

- Full spec-kit artifacts and blocking analysis precede source implementation.
- The existing restricted parser, immutable store, and versioned projection remain
  the sole data authorities.
- Encounter history adds no gameplay, input, Pixel Bus, or network authority.
- Explicit import and confirmed deletion preserve user ownership.
- Test-first tasks precede the implementation tasks they constrain.
- Full Cargo and repository CI parity remain mandatory before publication.
- No dependency, toolchain, workflow, release, license, or packaging change is planned.

**Gate result**: PASS.

## Project Structure

```text
specs/078-encounter-history-ui/
  spec.md
  plan.md
  research.md
  data-model.md
  contracts/encounter-history-contract.md
  checklists/requirements.md
  checklists/quality-and-privacy.md
  quickstart.md
  tasks.md
  analysis.md

src/encounter/history.rs
src/encounter/mod.rs
src/app/encounter_history.rs
src/app/mod.rs
src/app/strings.rs
src/app/ui.rs
src/main.rs
tests/encounter_history.rs
tests/app_ui_sizing.rs
docs/src/reference/encounter-data-and-metrics.md
docs/src/features/encounter-capture.md
docs/project/build-plans/{README.md,plan-038.md}
CHANGELOG.md
```

**Structure Decision**: Keep synchronous data composition under `encounter` and
place worker plus presentation helpers under `app`. The eframe layer owns only
window state and dispatch. Reuse the current catalog path so a catalog replacement
is observed on the next detail request without sharing a SQLite connection across
threads.

## Implementation Phases

1. Freeze specification, clarification, checklists, design, tasks, and analysis.
2. Add failing service tests for empty history, import, projection, typed failures,
   stable ordering, and exact deletion.
3. Implement the synchronous history service over S076 and S077.
4. Add failing worker and pure presentation tests, then implement the serialized
   worker event boundary and quality-aware formatting.
5. Add failing headless UI interactions, then implement menu access, history list,
   selected detail, busy state, and confirmations.
6. Wire app-owned store, active catalog, selected environment, and source capture
   paths without adding arbitrary file discovery.
7. Update canonical docs, active plan chronology, and changelog.
8. Run post-implementation analysis and full local CI parity, then publish and review.

## Design Decisions

1. Use an app-owned raw store beneath the existing per-user application root. This
   supplies a stable desktop destination while retaining a separate SQLite authority.
2. Calculate only the selected encounter in memory. A second database would add
   invalidation and migration states before any measured query need exists.
3. Derive the fixed capture path from the existing explicit AddOns environment.
   This inherits the override and Proton path logic while avoiding arbitrary scans.
4. Reopen the catalog by path for each detail request. Catalog update replacement
   remains atomic, and the worker never shares a connection created on another thread.
5. Preserve the loaded raw snapshot when projection fails. Catalog quality cannot
   erase valid user-owned observations.
6. Use typed diagnostic categories and safe fixed messages. Raw payloads and local
   paths never enter receipts, logs, or history error text.
7. Serialize commands through a capacity-one worker queue and disable controls while
   busy. This avoids conflicting import/delete/detail operations and GUI stalls.
8. Require separate confirmations for one-record and all-record deletion. Closing or
   canceling a confirmation performs no mutation.

## Complexity Tracking

| Choice | Why needed | Simpler option rejected |
| --- | --- | --- |
| Dedicated synchronous service plus UI worker | Keeps disk and bounded calculations off the render thread and independently testable | Calling S076 and S077 directly from egui would freeze rendering on large inputs |
| Typed diagnostic mapping | Acceptance requires distinct store, catalog, and compatibility states | Displaying raw error strings can leak paths and makes behavior unstable |
| On-demand projection | Preserves rebuildability and reacts to active catalog changes | Persisted derived state adds invalidation, schema, and deletion coupling |

## Post-Design Constitution Check

The design keeps data local, import explicit, deletion confirmed, raw and catalog
authorities separate, and all observed quality visible. No exception is required.
