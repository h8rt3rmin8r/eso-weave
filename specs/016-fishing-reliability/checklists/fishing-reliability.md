# Requirements Quality Checklist: Fishing Reliability and Status Collaboration

**Purpose**: Validate that the requirements for feature 016 are complete, clear,
consistent, and measurable before planning and implementation.
**Created**: 2026-07-12
**Feature**: [spec.md](../spec.md)

## Requirement Completeness

- [x] CHK001 - Are requirements defined for every fishing routine phase the player can observe (idle, casting, waiting for a bite, reeling in, recasting)? [Completeness, Spec §FR-007]
- [x] CHK002 - Are requirements defined for all three stop reasons (player stopped, no cast detected, signal lost)? [Completeness, Spec §FR-008]
- [x] CHK003 - Does the spec state the requirement that a previously installed addon copy is refreshed, not just newly installed copies? [Completeness, Spec §FR-002]
- [x] CHK004 - Are the safety behaviors that must be preserved enumerated (no blocking on the hook thread, focused-window-only suppression, SignalLost cancels pending interact, managed-marker-gated uninstall)? [Completeness, Spec §FR-009]
- [x] CHK005 - Does the spec state where the correctness logic must live so it is testable without the graphical layer? [Completeness, Spec §FR-010]
- [x] CHK006 - Are the poll cadence requirements defined for both the active fishing state and the idle state? [Completeness, Spec §FR-004]

## Requirement Clarity

- [x] CHK007 - Is the condition that selects the fast fishing cadence stated unambiguously (derived from the fishing state being other than idle)? [Clarity, Spec §FR-004]
- [x] CHK008 - Are the provisional timeout values (arm, reel, recast) stated with specific numbers rather than an unquantified adjective like conservative? [Clarity, Spec §FR-006]
- [x] CHK009 - Is the API version requirement specific about declaring both a current live value and a future value using the supported multi-value form? [Clarity, Spec §FR-001]
- [x] CHK010 - Is the persistence duration of the stop reason clearly bounded (until fishing is next started)? [Clarity, Spec §FR-008]
- [x] CHK011 - Is the meaning of the managed-marker preservation requirement clear enough to test (the exact marker line is unchanged)? [Clarity, Spec §FR-003]

## Requirement Consistency

- [x] CHK012 - Do the status labels named in the User Story 3 acceptance scenarios match the plain-language requirement in FR-007 without introducing conflicting names? [Consistency, Spec §FR-007]
- [x] CHK013 - Are the safety behaviors in FR-009 consistent with the constitution's safety-critical surfaces (no contradictions or omissions)? [Consistency, Spec §FR-009]
- [x] CHK014 - Is the interaction model (the hotkey casts; the player does not cast first) stated consistently between the assumptions and the user scenarios? [Consistency, Assumptions]

## Acceptance Criteria Quality

- [x] CHK015 - Are the reliability success criteria measurable with explicit counts or thresholds rather than vague reliability language? [Measurability, Spec §SC-001, §SC-002]
- [x] CHK016 - Can the addon-current success criterion be objectively verified against the live client? [Measurability, Spec §SC-003]
- [x] CHK017 - Is the stop-reason success criterion measurable (every early stop shows a reason)? [Measurability, Spec §SC-004]
- [x] CHK018 - Does each functional requirement map to at least one acceptance scenario or success criterion? [Traceability, Spec §FR-001..§FR-010]

## Scenario Coverage

- [x] CHK019 - Are requirements defined for the primary flow (cast, wait, reel, recast loop)? [Coverage, Spec §US1]
- [x] CHK020 - Are requirements defined for the addon-load prerequisite flow (addon recognized as current so the game loads it)? [Coverage, Spec §US2]
- [x] CHK021 - Are requirements defined for the status and stop-reason reporting flow across all stop paths? [Coverage, Spec §US3]

## Edge Case Coverage

- [x] CHK022 - Are requirements defined for a hotkey press when the player is not aimed at a hole (no cast confirmation, then a no-cast-detected stop)? [Edge Case, Spec Edge Cases]
- [x] CHK023 - Are requirements defined for signal loss mid-session, including that the interact key is not fired blindly? [Edge Case, Spec §FR-009]
- [x] CHK024 - Are requirements defined for rapid toggling on and off without leaving a pending action stranded? [Edge Case, Spec Edge Cases]
- [x] CHK025 - Are requirements defined for the future-update case where the live API version passes the first declared value? [Edge Case, Spec §FR-001]

## Non-Functional and Safety Requirements

- [x] CHK026 - Is the timing responsiveness requirement (advance and reel promptly while active) expressed as a user-facing outcome rather than an internal interval only? [Non-Functional, Spec §SC-002]
- [x] CHK027 - Does the spec require that the clock skew between setting and evaluating a deadline is eliminated? [Non-Functional, Spec §FR-005]
- [x] CHK028 - Are the safety-critical requirements marked as must-preserve and tied to tests that are never weakened? [Safety, Spec §FR-009, §FR-010]

## Dependencies and Assumptions

- [x] CHK029 - Is the assumption that the current live API version is 101050 recorded and marked for confirmation at implementation time? [Assumption, Spec Assumptions]
- [x] CHK030 - Is the dependency on the player reloading the UI or relogging to pick up refreshed addon files documented? [Dependency, Spec Assumptions]
- [x] CHK031 - Is the owed in-game validation of signal behavior and tuned timings recorded as an assumption rather than an unstated gap? [Assumption, Spec Assumptions]

## Ambiguities and Conflicts

- [x] CHK032 - Is the scope boundary clear that dual-bar or weaving concerns are out of scope for this feature? [Ambiguity, Spec scope]
- [x] CHK033 - Are there any remaining unquantified adjectives (for example prompt, adequate, conservative) that a reader could interpret differently? [Ambiguity, Spec §FR-006]
