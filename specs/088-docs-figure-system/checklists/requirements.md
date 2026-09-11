# Requirements Checklist: Documentation Figure System

**Purpose**: Validate that the S088 specification is complete, testable, bounded, and traceable to issues #155 and #156.
**Created**: 2026-09-11
**Feature**: [spec.md](../spec.md)

## Scope and traceability

- [x] CHK001 Every completion criterion from issues #155 and #156 maps to an explicit functional requirement.
- [x] CHK002 The exact meaningful and decorative image inventories are quantified.
- [x] CHK003 The #127 responsive-table boundary and non-documentation exclusions are explicit.
- [x] CHK004 Public, bundled offline, no-JavaScript, and print surfaces are covered.

## Accessibility and interaction

- [x] CHK005 Pointer, Enter, Space, Escape, close-button, and backdrop behaviors are testable.
- [x] CHK006 Modal inertness, focus containment, initial focus, and exact focus return are testable.
- [x] CHK007 Accessible naming, caption description, decorative exclusion, and duplicate-image prevention are testable.
- [x] CHK008 Persistent affordance, focus visibility, reduced motion, and non-color signaling are explicit.

## Visual quality and evidence

- [x] CHK009 Intrinsic sizing, non-upscaling, portrait, landscape, narrow, and wide conditions are measurable.
- [x] CHK010 Caption minimum size, contrast, wrapping, hierarchy, and strong lead-ins are measurable.
- [x] CHK011 Source policy, generated policy, hidden-browser evidence, and mutation failures are required.
- [x] CHK012 All requirements are free of unresolved clarification markers and implementation ambiguity.

## Notes

- The combined S088 boundary is intentional because figure markup, theme behavior, caption association, and browser evidence overlap directly between #155 and #156.
- The local native-dialog decision is evaluated and recorded in `research.md` and `plan.md`.
