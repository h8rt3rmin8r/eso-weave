# Requirements Quality Checklist: GUI Ergonomics, Information Design, and Auto-Save

**Purpose**: Validate that the requirements for slice 013 are complete, clear, consistent, and measurable across UX/information design, persistence and safety, and the constitution guardrails, before planning.
**Created**: 2026-07-11
**Feature**: [spec.md](../spec.md)

## Requirement Completeness (UX and information design)

- [ ] CHK001 - Are the controls that must become toggle switches enumerated exhaustively (suspend/resume, fishing, per-skill enabled, per-skill override, every boolean setting)? [Completeness, Spec FR-001]
- [ ] CHK002 - Is it specified which sections must carry headings distinct from body text? [Completeness, Spec FR-002]
- [ ] CHK003 - Are the exact Skills column labels and their order defined? [Completeness, Spec FR-003]
- [ ] CHK004 - Is the behavior of the Delay column when override is off fully defined, including that it shows the inherited default for the row's weave type? [Completeness, Spec FR-004]
- [ ] CHK005 - Are the three status-line titles and their required order (title first) specified? [Completeness, Spec FR-006]
- [ ] CHK006 - Are the settings clusters and their membership enumerated? [Completeness, Spec FR-019]
- [ ] CHK007 - Are previously hidden settings that must now surface (beacon location override, environment) identified? [Completeness, Spec FR-022]

## Requirement Clarity (UX and information design)

- [ ] CHK008 - Are the color roles for each status value named as value semantics rather than left as vague adjectives? [Clarity, Spec FR-007]
- [ ] CHK009 - Is "spans the same horizontal extent as the Skills section" measurable against a concrete reference? [Clarity, Spec FR-008]
- [ ] CHK010 - Is "darker than the surrounding app" defined by reference to an existing palette role rather than an unquantified adjective? [Clarity, Spec FR-023]
- [ ] CHK011 - Is the log resize range clearly bounded (minimum near a tenth of window height; maximum at the bottom of the interactive area)? [Clarity, Spec FR-024]
- [ ] CHK012 - Is the no-underscore rule stated as applying to user-facing labels only, leaving persisted keys unchanged? [Clarity, Spec FR-020]

## Requirement Consistency

- [ ] CHK013 - Do the toggle-switch requirement (FR-001) and the tooltip requirement (FR-025) agree on which controls exist to be styled and explained? [Consistency, Spec FR-001, FR-025]
- [ ] CHK014 - Is the renamed terminology (Status, Pixel Beacon (Addon)) used consistently across user stories, requirements, and success criteria? [Consistency, Spec FR-006]
- [ ] CHK015 - Is the single source of tooltip and help strings required so the same concept reads identically everywhere? [Consistency, Spec FR-026]

## Acceptance Criteria Quality

- [ ] CHK016 - Can "no control remains a button or bare checkbox" be objectively verified? [Measurability, Spec SC-001]
- [ ] CHK017 - Can "a first-time user can name each Skills column from its header alone" be assessed against defined labels? [Measurability, Spec SC-002]
- [ ] CHK018 - Is "exactly one persisted write and one save confirmation per continuous drag" measurable? [Measurability, Spec SC-005]
- [ ] CHK019 - Are the modal dismissal paths (outside click, Escape, close control) each independently verifiable? [Measurability, Spec SC-006]

## Requirement Completeness (persistence and safety)

- [ ] CHK020 - Is it specified that no Save or Apply control exists anywhere after this slice? [Completeness, Spec FR-009]
- [ ] CHK021 - Are the specific previously-unpersisted main-window values that must now persist listed? [Completeness, Spec FR-010]
- [ ] CHK022 - Is the persisted representation of the fishing state defined as an on/off intent, excluding transient sub-states? [Clarity, Spec FR-011]
- [ ] CHK023 - Is write coalescing during continuous edits required and its outcome (single settle-write) specified? [Completeness, Spec FR-013]
- [ ] CHK024 - Is re-clamping of the restored log height on load specified for the case where the window is smaller than when saved? [Edge Case, Spec FR-014]
- [ ] CHK025 - Is config back-compat addressed so new sections (session, log layout) default safely when absent? [Completeness, Spec Assumptions]

## Safety and Constitution Guardrails

- [ ] CHK026 - Is the focus-scoped input invariant explicitly preserved for restored non-suspended or fishing-active states? [Safety, Spec FR-012]
- [ ] CHK027 - Is it stated that no pinned contract surface is modified by this slice? [Constraint, Spec FR-028]
- [ ] CHK028 - Is the requirement that the egui layer stays thin, with correctness logic in the tested view-model, captured? [Constraint, Spec FR-027]
- [ ] CHK029 - Is a fallback to a safe default required when a persisted session state cannot be resumed? [Edge Case, Spec Edge Cases]
- [ ] CHK030 - Are the text-hygiene constraints (UTF-8 no BOM, LF, no em/en dashes) in force for all new artifacts? [Constraint, Constitution]

## Edge Case and Scenario Coverage

- [ ] CHK031 - Are requirements defined for resizing the window while the settings modal is open? [Coverage, Spec Edge Cases]
- [ ] CHK032 - Are requirements defined so the log can neither grow past the interactive area nor shrink to nothing? [Coverage, Spec FR-024]
- [ ] CHK033 - Is the interaction between the persisted logging level and the per-panel log filter addressed so they do not conflict on restore? [Coverage, Ambiguity]
- [ ] CHK034 - Are toast-coalescing requirements defined so a burst of saves yields a single confirmation? [Coverage, Spec FR-015]

## Dependencies and Assumptions

- [ ] CHK035 - Is the assumption that the GUI framework provides a resizable panel, a backdrop modal, a monospace family, and hover tooltips stated and validated? [Assumption, Spec Assumptions]
- [ ] CHK036 - Is the assumption that the bundled typeface already includes the heading weights documented? [Assumption, Spec Assumptions]

## Notes

- Items are requirements-quality checks (unit tests for the spec), not implementation tests.
- Resolve any unchecked item by updating spec.md before `/speckit-plan`, or record a deliberate deferral.
