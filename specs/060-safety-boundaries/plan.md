# Implementation Plan: Safety Boundaries

**Branch**: `codex/s060-safety-boundaries` | **Date**: 2026-09-07 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/060-safety-boundaries/spec.md`

## Summary

Close issues #92 and #94 through two independent fail-safe tracks. Extend the existing atomic input authorities with a monotonic invalidation epoch and feature-specific projections so stale weave work and autonomous Fishing output stop when authorization closes. Separately, make PixelBeacon target ownership a domain-level prerequisite for every writer, expose a distinct unmanaged UI state, and update managed content in place.

## Technical Context

**Language/Version**: Rust 2021 on the pinned repository toolchain

**Primary Dependencies**: Rust standard library atomics/channels, eframe/egui, serde, tracing

**Storage**: Existing filesystem-based PixelBeacon files; no schema migration

**Testing**: Rust unit and integration tests, docs policy, mdBook, typos

**Target Platform**: Windows 10/11 x64 and Linux x64

**Project Type**: Single-crate desktop application

**Performance Goals**: Constant-time non-blocking input classification; gate closure visible to workers without controller locks; no new polling thread

**Constraints**: No generated Down after gate closure; held generated input must be released; unmanaged targets remain byte-for-byte untouched; UTF-8 without BOM and LF

**Scale/Scope**: Input, weave, Fishing, PixelBeacon lifecycle, UI projection, tests, canonical docs, and plan lifecycle only

## Constitution Check

*GATE: PASS before research; rechecked PASS after design.*

- **Spec-driven**: PASS. Issues #92 and #94 are actionable, S060 has the complete spec-kit package, and implementation waits on analysis.
- **Safety-critical surfaces**: PASS. The slice strengthens non-blocking input gating, generated-input cancellation, marker-gated ownership, and subtree confinement with mandatory tests.
- **Test-first seams**: PASS. Existing mock input and Fishing sinks plus temporary filesystem fixtures support deterministic red-green-refactor work.
- **CI parity**: PASS. All Rust merge gates plus documentation validation are required before commit.
- **Outside-game scope**: PASS. No process memory, packets, or in-game automation surface is added.
- **Platform/config/text constraints**: PASS. No settings schema or platform expansion; linked-target tests are platform-gated where necessary.

## Architecture and Decisions

### D1: Extend typed gates with an invalidation epoch

The current shared weave projection omits focus, suspension, and menu state. Adding booleans alone would still allow a close-reopen transition to make stale queued work appear valid. The input-owned shared state therefore adds a monotonic epoch advanced on safe-to-unsafe transitions. Queued weave actions capture an epoch, running sinks retain the admitted epoch, and feature-specific checks combine current gate bits with epoch equality.

This is intentionally not a universal policy engine. Weave and Fishing use typed projections because roll dodge applies to weave but not Fishing, and their recovery rules differ.

### D2: Close before locks, reopen after synchronization

Focus, suspension, and menu closure publish to shared atomic state before any weave or Fishing mutex. Reopening occurs only after controller-local state is synchronized. This preserves the existing close-before-lock ordering used for life, world, and travel gates.

### D3: Cancel Fishing on suspend without replay

Suspension preserves the requested toggle, clears the active state and deadline, records `Suspended`, and emits nothing on resume. A fresh manual FishingStarted observation may restore Waiting, while an explicit off-then-on request may start a new cast. Menu gating keeps its established bounded deferral policy.

### D4: Ownership is proven at mutation time

`NotInstalled` means the target entry is absent. Any existing directory, file, link, or unreadable target that cannot prove the exact marker is unmanaged. Every write path rechecks this at mutation time. The UI is explanatory but never the security boundary.

### D5: Managed update is non-destructive

Update refreshes a positively managed target in place rather than deleting it first. This deviates from the prior clean-reinstall sequence because that sequence could remove valid managed content before replacement succeeds and continued to install after an ownership refusal.

### D6: Canonical docs, not a blog announcement

This bug-fix slice updates the shipped safety and feature documentation. It does not add a marketing blog post because the repository classifies the orphaned website/blog lifecycle separately, and these corrections are not a new product feature.

## Project Structure

### Documentation (this feature)

```text
specs/060-safety-boundaries/
├── analysis.md
├── checklists/
│   ├── addon-ownership.md
│   ├── input-authorization.md
│   └── requirements.md
├── contracts/
│   ├── action-authorization.md
│   └── pixelbeacon-lifecycle.md
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
├── app/
│   ├── beacon_light.rs
│   ├── mod.rs
│   └── routing.rs
├── beacon/
│   ├── api_check.rs
│   └── mod.rs
├── fishing/mod.rs
├── input/mod.rs
├── main.rs
└── weave/mod.rs

tests/
├── app_view_model.rs
├── beacon.rs
├── fishing.rs
├── input_engine.rs
└── weave_engine.rs

docs/src/
├── concepts/action-authorization.md
├── concepts/input-safety.md
├── features/fishing.md
├── features/pixelbeacon.md
└── features/weaving.md
```

**Structure Decision**: Extend the single crate and its existing trait seams. Add no crate, worker, dependency, or settings schema.

## Complexity Tracking

No constitution violation requires justification.

## Delivery Lifecycle

- Archive plan 029 with issue #81 and PR #97 evidence.
- Create plan 030 as the sole active plan for S060.
- Move #92 and #94 through In progress and PR review, then close both from the pull request.
- Leave #93, #95, #96, and release-verification issues untouched.
