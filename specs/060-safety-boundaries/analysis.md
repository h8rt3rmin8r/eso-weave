# Spec-kit Analysis: Safety Boundaries

## Pre-implementation Gate

**Result**: PASS

**Date**: 2026-09-07

## Authority and Scope

- Issues #92 and #94 are actionable and independently closeable.
- The combined slice is coherent around two fail-safe authorization boundaries, while contracts and task phases remain independent.
- Issues #93, #95, #96, embedded documentation, release work, and field verification remain outside S060.

## Constitution Alignment

- The full spec-kit artifact chain exists before implementation.
- Input callback work remains constant-time and non-blocking.
- Generated-input recursion, focused-window pass-through, held-input release, marker-gated ownership, and AddOns subtree confinement have explicit requirements and tests.
- Existing mock backends and temporary filesystem fixtures preserve test-first seams.
- No workspace, dependency, settings schema, process-memory, packet, or addon-protocol expansion is proposed.

## Consistency Review

- All 22 functional requirements map to tasks and at least one acceptance or verification path.
- The monotonic epoch resolves transient close-reopen races that gate booleans alone cannot detect.
- Typed feature projections resolve the difference between weave gates and Fishing gates without inventing one universal controller policy.
- Fishing suspension cancellation and menu deferral are deliberately different and consistently stated across spec, research, model, and contract.
- PixelBeacon `NotInstalled` means absence everywhere; every existing unproven target is consistently unmanaged.
- UI action hiding and final writer enforcement are both required, so stale status cannot bypass ownership.
- Managed in-place update is consistent with the no-delete-before-replacement decision.

## Coverage Review

| Risk | Requirement | Planned evidence |
| --- | --- | --- |
| Stale queued weave | FR-002 through FR-004 | epoch queue tests and worker admission tests |
| Mid-sequence gate closure | FR-004, FR-005 | light/heavy/bash operation matrix |
| Stranded generated input | FR-005 | held key and mouse release assertions |
| Suspended Fishing output | FR-007 through FR-011 | initial cast/reel/recast/timeout matrix |
| Unsafe Fishing replay | FR-008, FR-009 | retained-request and no-output recovery tests |
| Unmanaged overwrite | FR-012 through FR-018 | target-shape snapshots and stale-intent tests |
| Misleading UI ownership | FR-015, FR-016 | view-model action matrix and visible guidance tests |
| Documentation drift | FR-021 | coverage manifest and documentation policy tests |
| Lifecycle traceability | FR-022 | plan index and migration-ledger policy tests |

## Resolved Clarifications

1. Gate close-reopen transitions invalidate old work by epoch.
2. Unsuspending Fishing emits nothing automatically.
3. A fresh manual cast observation or explicit off-then-on request is the recovery boundary.
4. Existing links and unreadable targets are unmanaged.
5. Managed Update refreshes in place rather than deleting first.

## Findings

No CRITICAL, HIGH, or unresolved ambiguity remains. Implementation may begin under TDD.

## Post-implementation Gate

**Result**: PASS

**Date**: 2026-09-07

- Queue admission and running sinks now enforce the same monotonic authorization
  epoch across game, focus, suspension, menu, life, roll-dodge, world, and travel
  boundaries.
- Fishing suspension cancels active state and deadlines, retains the request,
  emits nothing on resume, and rechecks the applicable shared epoch at the real
  interact boundary.
- PixelBeacon classifies only an absent target as Not Installed. Every existing
  unproven target is Unmanaged, every writer refuses it, and managed Update writes
  in place without a preceding delete.
- The focused S060 matrix, full locked Rust suite, content coverage, plan
  lifecycle, and generated documentation policy all pass.
- Issues #93, #95, and #96 remain explicitly deferred and unchanged.

No CRITICAL, HIGH, or unresolved cross-artifact inconsistency remains.
