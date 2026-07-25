# UX Layout Requirements Checklist: UI Window-Sizing and Layout Hardening

**Purpose**: Validate that the window-sizing, log-viewer, save-confirmation, and
control-height requirements are complete, clear, consistent, and measurable
before planning and implementation. These items test the REQUIREMENTS, not the
implementation.
**Created**: 2026-07-24
**Feature**: [spec.md](../spec.md)

## Requirement Completeness

- [ ] CHK001 Are minimum-size requirements defined for both width and height independently? [Completeness, Spec §FR-001]
- [ ] CHK002 Is the requirement that the floor be derived from laid-out content (not a fixed constant) explicitly stated? [Completeness, Spec §FR-002]
- [ ] CHK003 Are requirements defined for restoring a saved geometry smaller than the current content minimum? [Completeness, Spec §FR-003]
- [ ] CHK004 Are requirements defined for all four log-viewer behaviors (grow-on-open, min height, min width, top-clamp)? [Completeness, Spec §FR-004..FR-007]
- [ ] CHK005 Is the disable-log-viewer behavior specified, including the resulting window height? [Completeness, Spec §FR-008]
- [ ] CHK006 Are requirements defined distinguishing which changes persist silently versus which raise the save confirmation? [Completeness, Spec §FR-009, §FR-010]
- [ ] CHK007 Is it stated that persisted content (window geometry, log height) is unchanged by this feature? [Completeness, Spec §FR-013]
- [ ] CHK008 Are the control types subject to height reduction (buttons, toggles, dropdowns) enumerated? [Completeness, Spec §FR-011]

## Requirement Clarity

- [ ] CHK009 Is "at least six lines of log text" quantified against a defined text size? [Clarity, Spec §FR-005, Clarifications]
- [ ] CHK010 Is "wider minimum width while the log viewer is open" expressed relative to a defined base minimum? [Clarity, Spec §FR-006]
- [ ] CHK011 Is the control-height reduction ("up to about 20 percent") bounded and its recording of the final figure required? [Clarity, Spec §FR-011, §FR-012]
- [ ] CHK012 Is "meaningful settings change" defined by example (toggle, form-field edit) so it is unambiguous? [Clarity, Spec §FR-009]
- [ ] CHK013 Is "the Skills area" used consistently as the boundary the log pane may not cross? [Clarity, Spec §FR-007]
- [ ] CHK014 Is the shrink-back-on-disable amount specified precisely (same amount grown) rather than vaguely? [Clarity, Spec §FR-008]

## Requirement Consistency

- [ ] CHK015 Do the content-minimum requirement (§FR-001) and the log-pane top-clamp requirement (§FR-007) reference the same measured content extent? [Consistency]
- [ ] CHK016 Are the shorter controls (§FR-011) and the dynamic content-derived floor (§FR-002) consistent, so the floor shrinks with the controls? [Consistency, Spec Assumptions]
- [ ] CHK017 Do the persist-silently requirement (§FR-010) and the persistence-unchanged requirement (§FR-013) agree without contradiction? [Consistency]
- [ ] CHK018 Is terminology for the save confirmation ("Settings saved" confirmation/toast) consistent across requirements and scenarios? [Consistency]

## Acceptance Criteria Quality

- [ ] CHK019 Is each success criterion (SC-001..SC-007) objectively measurable without implementation knowledge? [Measurability, Spec §Success Criteria]
- [ ] CHK020 Can "zero clipped rows/controls" be verified by observation in both themes? [Measurability, Spec §SC-001, §SC-006]
- [ ] CHK021 Is the toast criterion measurable as an exact count (zero for layout, exactly one for a real change)? [Measurability, Spec §SC-005]
- [ ] CHK022 Does every functional requirement map to at least one acceptance scenario or success criterion? [Traceability, Spec §FR-001..FR-013]

## Scenario Coverage

- [ ] CHK023 Are primary flows covered for all four user stories (min-size, log viewer, toast, control height)? [Coverage, Spec §User Stories 1-4]
- [ ] CHK024 Are requirements defined for the light and dark theme variants of the visual checks? [Coverage, Spec §SC-001, §SC-006]
- [ ] CHK025 Is the relaunch/persistence-survival scenario covered for geometry and log height? [Coverage, Spec §FR-013, §SC-005]

## Edge Case Coverage

- [ ] CHK026 Are requirements defined for a display smaller than the content minimum? [Edge Case, Spec §Edge Cases]
- [ ] CHK027 Are requirements defined for enabling the log viewer when the window is already at maximum height and cannot grow? [Edge Case, Spec §Edge Cases]
- [ ] CHK028 Are requirements defined for a saved log height outside the valid range (from an old version or edited file)? [Edge Case, Spec §Edge Cases]
- [ ] CHK029 Are requirements defined for a simultaneous settings-change-and-window-move within one save batch? [Edge Case, Spec §Edge Cases, §FR-009]
- [ ] CHK030 Is the interaction between shorter controls and the minimum-size floor addressed as an edge case? [Edge Case, Spec §Edge Cases]

## Non-Functional & Bounded Scope

- [ ] CHK031 Is it stated that all behavior is verifiable at the desk without the running game? [Scope, Spec Assumptions]
- [ ] CHK032 Are out-of-scope areas (PixelBeacon addon, pixel-bus reader, fishing, weave, input) explicitly excluded? [Scope, Spec §Assumptions]
- [ ] CHK033 Is legibility (no clipped/overflowed text) treated as a required, verifiable quality rather than an aesthetic preference? [Non-Functional, Spec §FR-012, §SC-006]

## Dependencies & Assumptions

- [ ] CHK034 Is the assumption that persistence semantics are unchanged documented and validated against the requirements? [Assumption, Spec §FR-013, Assumptions]
- [ ] CHK035 Is the decision to bundle all four issues as one interacting slice documented with rationale? [Assumption, Spec §Assumptions]

## Notes

- These items validate requirement quality (are the requirements complete, clear,
  consistent, measurable?), not implementation behavior.
- Check items off during the `/speckit-analyze` gate as each is confirmed against
  the spec and plan.
