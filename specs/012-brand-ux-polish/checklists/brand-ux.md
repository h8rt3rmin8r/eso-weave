# Brand and UX Requirements Quality Checklist: Brand and UX Polish

**Purpose**: Validate that the brand and UX requirements are complete, clear,
consistent, and measurable before implementation (unit tests for the requirements,
not the implementation).
**Created**: 2026-07-11
**Feature**: [spec.md](../spec.md)

## Visual and Brand Consistency

- [ ] CHK001 Are the palette color tokens specified concretely enough to apply without re-derivation (named roles for surfaces, accent, text, muted), for both dark and light themes? [Clarity, Spec §FR-001, §FR-002]
- [ ] CHK002 Is the primary UI typeface unambiguously identified with its license so it can be bundled and redistributed? [Completeness, Spec §Clarifications, §FR-009]
- [ ] CHK003 Are spacing and corner-radius requirements defined as a scale rather than left to the implementer's discretion? [Clarity, Spec §FR-001]
- [ ] CHK004 Are accent-usage rules stated (where the teal accent is and is not used) so accent application is consistent across surfaces? [Consistency, Spec §FR-001]
- [ ] CHK005 Is the light theme required to express the same identity as dark, rather than being an afterthought, with an explicit contrast expectation? [Completeness, Spec §FR-009, §SC-001]

## UI Interaction Correctness

- [ ] CHK006 Is column alignment specified for all seven skill rows, including the requirement that the value cell is reserved so override and non-override rows match width? [Completeness, Spec §FR-011, §SC-002]
- [ ] CHK007 Is the set of "clickable controls" that must show a pointer cursor enumerated or clearly bounded (buttons, toggles, selectors)? [Clarity, Spec §FR-012, §SC-003]
- [ ] CHK008 Are status-indicator and log-level color requirements defined for BOTH themes, not only dark, with a legibility criterion? [Coverage, Spec §FR-010]
- [ ] CHK009 Is it explicit that the window layout structure and the view-model behavior remain unchanged (presentation-only change)? [Consistency, Spec §FR-018, §SC-008]

## Icon and Asset Quality

- [ ] CHK010 Is icon legibility specified at concrete sizes (16, 32, 48, 256px) and on both light and dark backgrounds, as a measurable acceptance criterion? [Measurability, Spec §FR-004, §SC-004]
- [ ] CHK011 Is the theme-safe requirement defined (a self-contained badged form that does not rely on the surface behind it)? [Clarity, Spec §FR-004, §Edge Cases]
- [ ] CHK012 Are all icon renderings that must be regenerated from the new mark enumerated (Windows multi-resolution, Linux, AppImage, embedded exe, window icon)? [Completeness, Spec §FR-005, §FR-007, §FR-008]
- [ ] CHK013 Is the requirement that the exe-icon embedding must not break the non-Windows build stated? [Edge Case, Spec §Edge Cases]
- [ ] CHK014 Is it explicit that the new mark must NOT reintroduce the prior antique two-fish gold identity as the primary mark? [Consistency, Spec §FR-019]

## Installer Polish

- [ ] CHK015 Is the license-page requirement specific about presenting the full, unmodified license text in a clean proportional layout (vs summarizing or altering it)? [Clarity, Spec §FR-013, §SC-006]
- [ ] CHK016 Is the desktop-shortcut default state explicitly "off/opt-in", and is the Start Menu shortcut explicitly retained? [Completeness, Spec §FR-014]
- [ ] CHK017 Are the persistence/upgrade requirements for the shortcut choice defined (not resurrected or removed on upgrade)? [Coverage, Spec §FR-015, §Clarifications, §Edge Cases]
- [ ] CHK018 Is the branded wizard artwork requirement defined distinctly from the icon (banner and dialog art)? [Completeness, Spec §FR-016]
- [ ] CHK019 Is the platform scope of the desktop-shortcut opt-out unambiguous (Windows MSI only; Linux unaffected)? [Clarity, Spec §FR-014, §Edge Cases]

## Process and Text Hygiene Compliance

- [ ] CHK020 Is the requirement to record pinned-artifact changes with a dated changelog decision stated and traceable to which artifacts are pinned? [Traceability, Spec §FR-017]
- [ ] CHK021 Do the requirements avoid conflicting with the non-negotiable safety surfaces (this slice touches none of them)? [Conflict, Constitution §II]
- [ ] CHK022 Are text-hygiene constraints (UTF-8 no BOM, LF, no em/en dashes) applicable to every new/edited text asset acknowledged as in-scope for this slice? [Assumption, Constitution §Text Hygiene]

## Acceptance Criteria Quality

- [ ] CHK023 Can each success criterion (SC-001 through SC-008) be objectively verified without inspecting implementation internals? [Measurability, Spec §Success Criteria]
- [ ] CHK024 Is every functional requirement covered by at least one acceptance scenario or success criterion? [Coverage, Spec §Requirements, §Success Criteria]
- [ ] CHK025 Are the design-sign-off dependency and the "final tokens fixed at mockup" assumption documented so downstream steps are not blocked by undefined exact values? [Assumption, Spec §Assumptions]

## Notes

- These items validate requirement quality, not the running build; the build is
  validated separately by the implement/verify steps and the CI parity gate.
- Check items off as they are confirmed against the spec during the analyze step.
