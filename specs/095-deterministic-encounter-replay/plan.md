# Implementation Plan: Deterministic Encounter Replay

**Branch**: `codex/s095-deterministic-encounter-replay` | **Date**: 2026-09-15 | **Spec**: [spec.md](spec.md)

## Summary

Complete issue #186 by pinning the full subscription decision surface and adding
a pure Rust replay verifier for current schema-v2 captures. Addon version 3
embeds a bounded runtime normalization profile; complete captures must replay
exactly before import, partial captures remain explicitly indeterminate, and
legacy captures remain compatible without invented guarantees.

## Technical Context

**Language/Version**: Rust 2021 on pinned stable toolchain; Lua 5.1 addon runtime
**Primary Dependencies**: serde, serde_json, rusqlite, mlua test harness
**Storage**: ESO SavedVariables input and immutable SQLite encounter store
**Testing**: cargo test, production Lua differential harness, docs-policy Node tests
**Target Platform**: Windows 10/11 x64 and Linux x64; ESO Live 101050 and PTS 101051
**Project Type**: Single Rust desktop crate plus managed Lua addon
**Performance Goals**: Linear replay at the existing 100,000-observation ceiling
**Constraints**: Local-only, no gameplay authority, checked numeric conversion,
bounded value-free diagnostics, exact legacy byte/hash preservation
**Scale/Scope**: 11 callbacks, six API sources, two lifecycle sources, seven
excluded source families, v1 and pre-profile-v2 compatibility

## Constitution Check

- Principle I: PASS. The full spec-kit sequence and child issue #200 precede code.
- Principle II: PASS. Replay is local, deterministic, bounded, and read-only.
- Principle III: PASS. Differential and mutation tests are observed red first.
- Principle IV: PASS. Full code, docs, policy, text, encoding, and diff gates run.
- Principle V: PASS. No capture mode, transport, source family, or action is added.
- Text hygiene: PASS. New text is UTF-8 without BOM or mojibake.

Post-design re-check: PASS. The runtime profile avoids undocumented hard-coded
enum values without changing capture schema or granting new authority.

## Project Structure

```text
specs/095-deterministic-encounter-replay/
├── analysis.md
├── checklists/
├── contracts/
├── data-model.md
├── plan.md
├── quickstart.md
├── research.md
├── spec.md
└── tasks.md

addon/EsoWeaveData/Encounter.lua
src/encounter/{mod.rs,model.rs,replay.rs,validate.rs}
tests/{encounter_addon.rs,encounter_import.rs,encounter_metrics.rs}
docs/src/{features/encounter-capture.md,reference/architecture.md,
          reference/testing-strategy.md}
docs/project/{encounter-model.json,build-plans/plan-042.md}
```

**Structure Decision**: Add one pure replay module to the established encounter
boundary and extend the production Lua envelope. Do not create a second parser,
store, capture stack, or documentation subsystem.

## Complexity Tracking

No constitution violations. The normalization profile is a proportional contract
addition required to reproduce runtime enum classification without undocumented
numeric assumptions.

## Delivery Sequence

1. Complete specification, clarification, security checklist, research, design,
   contracts, tasks, and analysis gates.
2. Add failing differential, mutation, legacy, loss, nil-edge, and ceiling tests.
3. Add the addon-v3 normalization profile and pure bounded Rust replay engine.
4. Enforce complete-current verification before import and preserve legacy paths.
5. Complete the Live/PTS include and exclude matrix and canonical documentation.
6. Run independent code, security, and domain reviews and resolve every finding.
7. Run the full merge gate, push, open the PR, process hosted reviews, request no
   more than one second Codex review, and stop for operator merge approval.
