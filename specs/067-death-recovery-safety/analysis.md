# Spec-kit Analysis: Death Recovery Safety

## Pre-implementation Gate

**Result**: PASS

**Date**: 2026-09-09

## Authority and Scope

- Issue #109 is actionable, P0, and independently closeable from repository evidence.
- Issue #110 owns later installed-release evidence and remains outside S067.
- The specification, plan, research, model, contract, tasks, and checklists agree
  on one death-recovery authorization boundary.

## Constitution Alignment

- The complete spec-kit chain exists before runtime implementation.
- The design keeps hook-thread work atomic and non-blocking.
- Existing recursion, focus, held-release, signal-loss, addon ownership, and path
  containment invariants remain mandatory.
- No memory, packet, persistence, dependency, workspace, or new payload-block
  expansion is proposed.

## Consistency Review

- Addon lifecycle evidence owns the decision it alone can observe.
- B21 compatibility remains fail closed for old and new participants.
- Unsafe transitions close before locks; recovered Alive opens after current
  controller observations are routed.
- Existing S060 epochs invalidate weave and Fishing work; the new death epoch is
  diagnostic rather than a competing authorization source.
- Auto-potion retry reset uses a complete interval after recovery rather than
  clearing the timestamp and accidentally authorizing immediately.
- Path-specific completion and polling fallback are bounded and never rely on an
  unconditional sleep.

## Coverage Review

| Risk | Requirements | Planned evidence |
| --- | --- | --- |
| Premature Alive | FR-001 through FR-007 | Lua source contract and B21 transition tests |
| Stale queued/running weave | FR-008 through FR-010 | epoch admission and sink cancellation tests |
| Fishing deadline replay | FR-011 | armed, reel, recast, and timeout tests |
| Immediate recovered potion | FR-012, FR-013 | exact field-report reproduction and boundary timing tests |
| Stale recovery inputs | FR-009, FR-013 | forced event ordering and routing tests |
| Wire drift | FR-014, FR-015 | cross-language constants and manifest tests |
| Weak diagnostics | FR-016 | decoder, view-model, and transition-log tests |
| Regression | FR-017, SC-006 | focused and full locked suites |

## Findings

No CRITICAL, HIGH, unresolved ambiguity, placeholder, or constitution conflict
remains. Implementation may begin under TDD.

## Post-implementation Gate

**Result**: PASS

**Date**: 2026-09-09

- PixelBeacon now treats player-alive as evidence inside an idempotent death
  episode and requires path-specific completion plus a later baseline.
- B21 retains its position, marker, checksum, Alive, Dead, and legacy 0xE0
  fail-closed semantics while adding distinct world and no-load recovery values.
  Its decoder now requires exactly one matching payload at every tolerance. This
  deliberately improves the former first-match logic because the added recovery
  codes make ambiguous high-tolerance matches a realistic safety concern.
- Unsafe Life transitions route first. Recovered Alive routes last after a
  forced current baseline, including roll-dodge even though the initial task
  list named only the auto-potion observations. This proportional addition is
  required because roll-dodge is also action-driving weave evidence.
- Existing authorization epochs cancel queued and running work, the death epoch
  provides diagnostics, Fishing cancels all active phases, and Auto Potion starts
  a new retry episode only after an observed death.
- Ordinary signal and tolerance recovery deliberately do not reset the potion
  retry interval because they are not death episodes and existing recovery
  behavior remains valid.
- Focused suites, the complete locked suite, and documentation policy pass. The
  final formatting and strict Clippy results are recorded by tasks T024 to T026.

No CRITICAL, HIGH, unresolved ambiguity, placeholder, privacy issue, or
constitution conflict remains.
