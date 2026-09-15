# Spec-Kit Analysis: Stale HUD Retention

**Date**: 2026-09-15

**Result**: PASS

## Authority consistency

- GitHub issue #171 requires bounded persisted configuration, retention across runtime, focus, and signal loss, accessible cause and age, fresh recovery, exact expiry, zero behavior, safety separation, tests, and documentation.
- `spec.md` expresses every issue acceptance item and adds truthful handling for unknown runtime or focus evidence.
- Constitution Principle II is preserved because retention contains only rendered output and no controller or input type.
- `plan.md`, `research.md`, `data-model.md`, and both contracts select one architecture: an additive UI preference and one monotonic process-local presentation cache.

## Requirement-to-task coverage

| Requirements | Implementation and evidence tasks |
| --- | --- |
| FR-001 through FR-005 | T015 through T019, T038 through T041 |
| FR-006 through FR-011 | T020 through T025, T038 through T042 |
| FR-012 through FR-016 | T021, T026 through T028, T038 through T042 |
| FR-017, FR-018, FR-021 | T029 through T032, T039, T041, T042 |
| FR-019 | T015, T016, T020, T021, T026, T029, T030, T038, T039 |
| FR-020 | T033 through T037, T040, T042 |

Every functional requirement maps to implementation or validation work. Every user story has a deterministic independent test path.

## Finding audit

### Resolved findings

1. **Authority leakage risk**: The design caches rendered views in `AppModel`, not decoded signals in an engine or controller. No reverse path exists.
2. **Clock duplication risk**: Observation transitions and `view_at` use the same injected monotonic origin as the model. No widget timer or background worker is introduced.
3. **Indefinite extension risk**: Cause changes retain the original `lost_at_ms`; settings edits are capped at `original_deadline_ms`, so the current preference can shorten but never restart or extend the interval.
4. **Mixed snapshot risk**: Coherence is evaluated once for the complete rendered player-state presentation instead of per field.
5. **Misleading cause risk**: Unknown runtime and focus have distinct unavailable causes rather than being asserted inactive or unfocused.
6. **Accessibility risk**: A visible labeled text row states stale status, cause, and age before the values. Color is supplementary.
7. **Persistence risk**: Only the interval is configuration. Values, cause, and timestamps are memory-only and empty on construction.

### Remaining findings

None. There are no unresolved placeholders, conflicting contracts, missing test obligations, or constitution exceptions.

## Scope audit

The planned production diff is confined to UI configuration, application presentation, strings, and rendering. Routing, game evidence, controllers, input, addons, Pixel Bus protocol and decoding, encounter data, and durable player-state storage remain unchanged.

## Gate evidence

- Requirements and presentation-safety checklists: complete
- Clarification markers: none, apart from checklist/task statements naming the marker check
- Critical conflicts: none
