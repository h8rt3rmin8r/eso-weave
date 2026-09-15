# Spec-Kit Analysis: Persistent Auto Potion Request

**Date**: 2026-09-15

**Result**: PASS

## Authority consistency

- GitHub issue #172 requires persistence, legacy default-off behavior, one UI and
  F3 authority, startup safety, tests, and direct canonical prose.
- `spec.md` expresses every issue acceptance item and explicitly supersedes the
  historical memory-only decision in S039 R7 and S043 FR-002.
- Constitution Principle II is preserved because storage contains request only;
  controller gates, evidence, retry history, and input submission remain
  transient and unchanged.
- `plan.md`, `research.md`, `data-model.md`, and both contracts select one
  architecture: version 4 `state.json` plus the existing controller and session
  scheduler.

## Requirement-to-task coverage

| Requirements | Implementation and evidence tasks |
| --- | --- |
| FR-001 through FR-006 | T017, T020, T029 through T032 |
| FR-007 through FR-010 | T024 through T028, T039 through T043 |
| FR-011 through FR-013 | T018, T019, T021 through T023 |
| FR-014 and FR-017 | T027, T042, T043 |
| FR-015 | T033 through T038, T041 |
| FR-016 | T017 through T032, T039, T040 |

Every functional requirement maps to at least one implementation or validation
task. Every user story has an independent deterministic test path.

## Finding audit

### Resolved findings

1. **Historical authority conflict**: S039 and S043 required non-persistence.
   The spec, research, plan, and changelog task now require an explicit,
   justified deviation rather than silently copying the old logic.
2. **Schema ambiguity**: The design now advances the version to 4 while retaining
   additive default-off compatibility. It does not mislabel a changed shape as
   version 3.
3. **Startup action risk**: The restore contract prohibits ticks, gate changes,
   telemetry restoration, and direct input. The controller's fail-closed
   defaults remain the only effective-state authority.
4. **Toggle divergence risk**: Both UI and F3 already converge on one intent;
   tasks require persistence to be added at that shared model seam.
5. **Partial corruption risk**: A malformed request continues to invalidate the
   complete document and use the existing safe default plus notice.

### Remaining findings

None. There are no unresolved placeholders, unclear requirements, conflicting
contracts, missing test obligations, or constitution exceptions.

## Scope audit

The planned diff is confined to session state, application model integration,
tests, documentation, changelog, build-plan rollover, and spec artifacts. It
does not require Pixel Bus, addon, input backend, controller trigger, retry,
resource, quickslot, configuration schema, or package changes.

## Gate evidence

- Requirements and safety checklists: complete
- Clarification markers: none (only checklist/task statements naming the check)
- Spec-kit prerequisite command with required tasks: pass
- Initial `git diff --check`: pass
- Critical conflicts: none
