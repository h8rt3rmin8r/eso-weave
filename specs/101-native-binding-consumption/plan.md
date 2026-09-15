# Implementation Plan: Native Binding Consumption

**Branch**: `codex/s101-native-binding-consumption` | **Date**: 2026-09-15 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/101-native-binding-consumption/spec.md`

## Summary

Implement issue #207 by moving combat trigger ownership and weave synthesis from duplicated desktop keys and hardcoded mouse buttons to the coherent S100 native binding snapshot. The input engine will match exact physical keyboard or mouse chords, compile one immutable chord plan per admitted trigger, and invalidate queued or running work whenever binding authority changes. The weave sink will execute each chord through a platform-neutral ownership ledger that presses missing modifiers around primary-down, never releases physical modifiers, and cleans up application-owned held primaries on every cancellation path. Desktop persistence and the settings editor will retain only F1, F2, and F3 application toggles.

## Technical Context

**Language/Version**: Rust 2021 on pinned stable 1.96; Markdown

**Primary Dependencies**: Existing Rust standard library, `windows-sys`, `evdev`, `libc`, pixel-bus reader, and input/weave engines

**Storage**: Existing JSON settings, with obsolete combat binding entries discarded on load/save

**Testing**: Rust unit and integration tests, platform mapping tests compiled on their target, existing documentation and trust gates

**Target Platform**: Windows 10/11 and Linux X11/XWayland desktop companion

**Project Type**: Single-crate cross-platform desktop application plus unchanged embedded ESO addon

**Performance Goals**: Keep hook classification synchronous, allocation-free on the steady path, and bounded to eleven binding facts; preserve the existing 10 ms cancellation observation bound

**Constraints**: Read-only ESO authority; exact chord matching; no guessed controls; no release of physically held modifiers; existing gates remain monotonic authorities; Linux recursion separation remains structural

**Scale/Scope**: Seven combat triggers, Attack and Block targets, 111 keyboard primaries, seven mouse primaries, four modifiers, four weave types, three desktop toggles

## Constitution Check

*GATE: Passed before research and re-checked after design.*

- **Principle I, Spec-Driven Development**: PASS. S101 is child issue #207 and follows specify, checklist, plan, tasks, analysis, then implementation.
- **Principle II, Safety-Critical Surfaces**: PASS. Native evidence starts unavailable, exact matching is fail closed, binding replacement increments the existing authorization epoch, and physical modifier ownership is never widened.
- **Principle III, Test-First**: PASS. Planning requires failing contract tests before input, execution, platform, persistence, and presentation changes.
- **Principle IV, CI Parity**: PASS. Formatting, strict linting, all-target locked tests, release build, docs, trust, spelling, links, and encoding remain required.
- **Principle V, Bounded Scope**: PASS. Combat consumes only Skill 1 through 5, Ultimate, Synergy, Attack, and Block. Fishing Interact and Auto Potion Quickslot remain S102.
- **Addon constraints**: PASS. PixelBeacon is unchanged and remains read-only with no SavedVariables or mutation surface.
- **Pinned artifacts and text hygiene**: PASS. No dependency is added. All text remains UTF-8 without BOM, LF, and free of forbidden dash characters.

## Design

### Coherent combat authority

Store the latest `NativeBindingSet` behind the input engine's short-lived binding mutex. Every set replacement, including unavailable evidence, increments the shared weave authorization epoch before publishing the replacement. Build a collision index for the seven combat triggers and reject every duplicated exact chord.

The input engine also stores the active weave requirement for each trigger. Light and heavy require skill plus Attack, bash requires skill plus Attack and Block, and block casting requires skill plus Block. An exact physical primary-down can be suppressed only after all required facts are valid, the plan has no duplicate trigger, existing gates admit work, and the current physical modifiers are compatible with every target chord.

### Physical event model

Add platform-neutral physical events for supported native primaries and normalized modifier transitions. Modifier events update a dedicated physical state and pass through. Primary events first preserve the existing pass-through lifecycle rule, then try native combat classification, then independently try the legacy three-toggle table for keyboard primaries.

Windows installs keyboard and mouse low-level hooks on the same message-loop thread and rejects injected events through both hook flags. Linux grabs one keyboard and one pointer device, waits for either descriptor with `poll`, forwards unrelated events through one union-capability uinput device, and never reads its own virtual output.

### Immutable chord plans

Extend `QueuedAction` with an optional copied combat plan containing skill, Attack, and optional Block chords from one binding generation. Toggle events carry no plan. The worker rechecks the captured authorization before and after its weave lock, then passes the immutable plan to the sequence builder. No later binding lookup can change an admitted sequence.

### Chord execution and ownership

Replace key and fixed mouse operations with native chord-primary transitions. On a chord down, the real sink verifies that currently physical modifiers remain a subset of the target, presses missing modifiers in Control, Alt, Shift, Command order, emits the target primary, then releases only those temporary modifiers in reverse order. The primary remains owned until its matching up when the sequence type requires a hold. Wheel activation emits one relative tick and has no held release.

The sink tracks generated held primaries. Gate invalidation or a synthesis failure prevents all later down events and performs best-effort reverse-order cleanup. Cleanup never emits an up event for a physical modifier.

### Configuration and presentation migration

Narrow `BindingTable` to the three application toggles while retaining its public role for settings and tests. Loading ignores legacy combat keys without a fallback notice because their authority has moved to live ESO evidence; saving writes only the three toggle entries. Remove `SkillSlot.key`, `sync_keys`, combat rows from the settings editor, and hardcoded key suffixes from weave labels. Fishing and Auto Potion configuration remain unchanged.

## Project Structure

### Specification package

```text
specs/101-native-binding-consumption/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── combat-chord-execution.md
├── checklists/
│   └── requirements.md
├── analysis.md
└── tasks.md
```

### Repository integration

```text
src/input/native.rs
src/input/mod.rs
src/input/bindings.rs
src/input/windows.rs
src/input/linux.rs
src/input/mock.rs
src/weave/types.rs
src/weave/sequence.rs
src/weave/mod.rs
src/app/routing.rs
src/app/mod.rs
src/app/ui.rs
src/main.rs
tests/input_engine.rs
tests/weave_sequence.rs
tests/weave_engine.rs
tests/config.rs
tests/app_settings.rs
tests/app_view_model.rs
docs/src/concepts/input-safety.md
docs/src/features/weaving.md
docs/src/reference/settings.md
docs/src/getting-started/troubleshooting.md
docs/project/build-plans/plan-043.md
docs/project/build-plans/README.md
CHANGELOG.md
.specify/feature.json
specs/101-native-binding-consumption/
```

**Structure Decision**: Extend the existing input and weave boundaries. Native facts remain in `input::native`, synchronous admission remains in `InputEngine`, pure ordering remains in `weave::sequence`, and OS mapping remains in the existing platform backends.

## Verification Strategy

1. Add failing exact-match, plan, invalidation, collision, ownership, sequence, migration, and presentation tests.
2. Implement coherent native authority and immutable queued plans in the input engine.
3. Replace hardcoded weave operations with chord operations and implement cancellation-safe real-sink ownership.
4. Expand Windows and Linux interception and synthesis mappings exhaustively.
5. Remove duplicate combat settings and hardcoded labels while preserving all three toggle regressions.
6. Route binding evidence through the pre-lock safety boundary and update canonical documentation.
7. Run focused tests, spec-kit analysis, full local gates, release build, docs, trust, spelling, links, encoding, and diff review.
8. Publish the PR, address every hosted result, request exactly one second `@Codex review` round, and ask for final merge only after all checks and threads are satisfied.

## Complexity Tracking

No constitution violations or complexity exceptions are required.
