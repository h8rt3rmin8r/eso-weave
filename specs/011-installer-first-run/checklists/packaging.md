# Checklist: Packaging and First-Run Requirements Quality

**Purpose**: Validate that the installer and first-run requirements are complete, clear, consistent, measurable, and cover the relevant edge cases before planning and implementation.

**Created**: 2026-07-11

**Feature**: [spec.md](../spec.md)

**Depth**: Release gate (reviewed at the pre-push halt)

## Requirement Completeness

- [ ] CHK001 - Are the minimum required installer wizard steps explicitly enumerated? [Completeness, Spec FR-001]
- [ ] CHK002 - Is a distinct completion confirmation required and separated from the mere absence of an error? [Completeness, Spec FR-002]
- [ ] CHK003 - Are both a desktop shortcut and a Start Menu entry each required as separate obligations? [Completeness, Spec FR-006, FR-007]
- [ ] CHK004 - Are requirements defined for the silent/unattended install path, not only the interactive path? [Completeness, Spec FR-005]
- [ ] CHK005 - Is the startup-failure surface specified as both a visible message and a log entry, not one or the other? [Completeness, Spec FR-011, FR-012]
- [ ] CHK006 - Are documentation obligations (shortcut locations and log directory) captured as requirements rather than left implicit? [Completeness, Spec FR-013]
- [ ] CHK007 - Are upgrade-in-place and uninstall obligations restated so this slice cannot regress them? [Completeness, Spec FR-008]

## Requirement Clarity

- [ ] CHK008 - Is "user-controlled launch option, defaulting to selected" unambiguous about when the application does and does not start? [Clarity, Spec FR-004]
- [ ] CHK009 - Is "no console or command window shown" specified across the whole launch lifecycle (before, during, after)? [Clarity, Spec FR-010, US3]
- [ ] CHK010 - Is "visible message indicating the failure" defined precisely enough to be testable for a windowless program? [Clarity, Spec FR-011, Assumptions]
- [ ] CHK011 - Is "install location step" clear about whether the user may change the location and what the default is? [Clarity, Spec FR-001]

## Requirement Consistency

- [ ] CHK012 - Do the launch-option requirements agree between the interactive path (defaults selected) and the silent path (never launches)? [Consistency, Spec FR-004, FR-005]
- [ ] CHK013 - Is the desktop-shortcut requirement consistent with the master-spec constraint against writing to game or Documents directories? [Consistency, Spec FR-006, FR-008]
- [ ] CHK014 - Are shortcut obligations consistent between the functional requirements and the documented locations in FR-013? [Consistency, Spec FR-006, FR-007, FR-013]

## Acceptance Criteria Quality

- [ ] CHK015 - Can "install visibly completes" be objectively verified from the success criteria without inspecting implementation? [Measurability, Spec SC-001]
- [ ] CHK016 - Is the "start within 10 seconds without searching" criterion measurable and tied to a concrete user action? [Measurability, Spec SC-002]
- [ ] CHK017 - Are the console-window and startup-failure criteria expressed as pass/fail percentages that a reviewer can check? [Measurability, Spec SC-003, SC-004]
- [ ] CHK018 - Is the silent-install non-launch criterion measurable (0% of runs)? [Measurability, Spec SC-005]

## Scenario Coverage

- [ ] CHK019 - Are requirements defined for the primary interactive install-and-launch flow? [Coverage, Spec US1, US2]
- [ ] CHK020 - Are requirements defined for the upgrade-over-prior-version flow, including that completion is still shown and shortcuts stay valid? [Coverage, Spec Edge Cases]
- [ ] CHK021 - Are requirements defined for the recovery/cancel flow (user cancels partway) so no partial install remains? [Coverage, Exception Flow, Spec US1]
- [ ] CHK022 - Are requirements defined for the launch-option-selected-but-startup-fails path linking US2 to US4? [Coverage, Spec US2, US4]

## Edge Case Coverage

- [ ] CHK023 - Is behavior specified when the license is not accepted? [Edge Case, Spec FR-003, Edge Cases]
- [ ] CHK024 - Is behavior specified when a shortcut location is redirected or unavailable, and does it avoid failing the whole install? [Edge Case, Spec FR-009, Edge Cases]
- [ ] CHK025 - Is behavior specified for a silent/unattended install requiring no interaction and no auto-launch? [Edge Case, Spec FR-005, Edge Cases]

## Dependencies & Assumptions

- [ ] CHK026 - Is the assumption that an existing logging facility with a known log directory will receive startup failures stated and valid? [Assumption, Spec Assumptions]
- [ ] CHK027 - Is the scope boundary excluding Linux packaging explicitly documented? [Assumption, Spec Assumptions]
- [ ] CHK028 - Is the per-machine elevated install assumption documented, since it drives the deferred launch-mechanism decision? [Assumption, Spec Assumptions, Notes]

## Ambiguities & Conflicts

- [ ] CHK029 - Is the deferred launch-under-elevation decision clearly flagged as a mechanism choice that does not alter any functional requirement? [Ambiguity, Spec Clarifications]
- [ ] CHK030 - Is there any conflict between "defaulting to selected" launch and the requirement that startup failures never appear as nothing happening? [Conflict, Spec FR-004, FR-011]
