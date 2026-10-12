# Implementation Plan: Data Addon Workflow Recovery

**Branch**: `codex/s125-data-addon-workflow-recovery` | **Date**: 2026-10-11 | **Spec**: [spec.md](spec.md)

## Summary

Deliver issues #265-#271 through one slice. Repair text roles and adaptive instructions, centralize addon user chat in the existing bootstrap, add explicit start with validation before state changes, inspect saved data through the encounter worker, preserve controlled failure categories, and compose one task-oriented recording/catalog UI.

## Technical Context

**Language/Version**: Rust 2021, repository Rust 1.96 toolchain; ESO Lua.
**Primary Dependencies**: existing egui/eframe 0.36, mlua test harness, rusqlite and bounded SavedVariables parser. No new dependencies.
**Storage**: existing local saved addon file and encounter SQLite history. No storage migration.
**Testing**: production Lua command/callback tests, egui_kittest interaction and paint fixtures, service/worker import fixtures, required cargo and documentation gates.
**Target Platform**: Windows 10/11 x64 and Linux x64.
**Performance Goals**: no source file work on UI/input-hook threads; one bounded worker operation at a time.
**Constraints**: four-file managed addon inventory; no raw payloads in guidance/logs; source parsing never executes Lua; current hard limits and import atomicity retained.
**Scale/Scope**: seven issues across two addon modules and three existing desktop surfaces.

## Constitution Check

Pre-research and post-design: PASS. Full Spec Kit precedes implementation; test-first regression evidence; all input, lifecycle, source confinement and module-isolation tests preserved; no added addon surface, game-control transport, migration or capture mode. Foreground fmt/clippy/test and required build/docs gates precede commit. Owner authorized automatic PR publication and at most two Codex rounds, retaining final merge and release authority.

## Project Structure

- `addon/EsoWeaveData/EsoWeaveData.lua`: shared message delivery and readiness retry within existing package.
- `addon/EsoWeaveData/{Catalog,Encounter}.lua`: user messages, concise help/status, explicit start.
- `src/app/{mod,ui,encounter_history}.rs`: adaptive guidance, workflow, source requests/results.
- `src/encounter/{mod,validate,history}.rs`: controlled validation categories and bounded read-only source inspection.
- `tests/{app_ui_sizing,encounter_addon,collector_addon,encounter_history,app_encounter_history}.rs`: meaningful negative/positive regressions.
- `docs/src`, build-plan indexes/ledger, `CHANGELOG.md`: matching user workflow and chronological delivery record.

## Decisions

1. Keep shared chat delivery in the bootstrap, already loaded before both modules. A fifth file would expand managed inventory unnecessarily. Retry when the actual chat system is ready and keep initialization buffering bounded; report overflow rather than accumulating unbounded memory.
2. Reuse the semantic strong-text helper. A global active-control color change would corrupt filled-button foreground. Replace all scoped raw surface emphasis and gate new bypasses in the required Rust suite.
3. Essential status instructions use a distinct adaptive row layout with content-derived height; telemetry retains the compact row and its existing accessible reveal behavior. Details get intentional vertical scrolling. Wrapped content changes the shared dashboard height, so sizing checks use measured wide/narrow content bounds instead of the former two fixed heights.
4. Extend the existing encounter worker with InspectSource, tagged by selected path/environment and generation. Inspect on workflow open/environment changes and via Refresh saved source; discard stale results. Refresh imported history remains independent.
5. Reuse bounded stable-read and hostile-data parsing/validation. Introduce controlled version/channel categories at validation sites, not text matching. Import separates source read/parse from store append so store I/O remains distinguishable. Inspection returns only controlled state/counts and filesystem metadata, not raw observations.
6. Start validates its entire syntax first, reports enabled activity idempotently, and checks retained/failed blockers before updating selected fields. Existing stop/toggle/arm/disarm remain compatible.
7. Default walkthrough selects the current Live/PTS environment and single mode. Continuous, retained-session clearing, partial and failed recovery are secondary disclosures. Commands are copyable text/actions; desktop never sends them into ESO.
8. Acknowledgment remains content-bound explicit intent and is shown only when a compatible Live candidate can be installed. Busy state exposes its progress/reason; origin explanation is direct text.
9. No release rollover or owner field verification. Archive completed Plan050 and introduce Plan051 with actual chronology; one official PR closes seven outcomes on owner merge.

## Implementation and Validation

US1 readability foundation -> US2 visible/idempotent commands -> US3 inspection/diagnostics -> US4 integrated workflow and catalog intent -> matching docs/planning -> full automated gates -> local commit -> authorized push/official PR -> external review and exact-head CI -> owner merge handoff.

Write failing tests before each implementation. Inspect actual galley glyph colors, painted background/opacity and clip rectangles; accessible string presence alone is insufficient. Exercise both modules with blocked debug and separate system chat, delayed readiness and repeated flush. Source fixtures cover initialized, active retained, stopped, partial, interrupted, failed, unsupported, wrong channel, malformed, absent/denied and store failure; assert no source writes or implicit history creation. UI fixtures cover empty/eligible/incompatible/busy candidates, candidate identity changes and task reachability.

## Complexity Tracking

No constitution exception or architecture expansion is required. Typed source categories and an additional worker command are proportionate extensions of existing seams.
