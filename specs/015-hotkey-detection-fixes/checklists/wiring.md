# Checklist: Hotkey and Detection Wiring Requirements Quality

**Purpose**: Validate that the requirements for feature 015 are complete, clear,
consistent, and measurable before planning and implementation.
**Created**: 2026-07-11
**Feature**: [spec.md](../spec.md)

## Requirement Completeness

- [x] CHK001 - Is the shared-state requirement (hotkey and GUI reach one suspend state) explicitly stated rather than implied? [Completeness, Spec §FR-001]
- [x] CHK002 - Is the shared-state requirement for fishing (hotkey and GUI reach one enabled state) explicitly stated? [Completeness, Spec §FR-002]
- [x] CHK003 - Are requirements defined for what the GUI must reflect after a hotkey-driven change, and on what timing (next frame)? [Completeness, Spec §FR-005]
- [x] CHK004 - Are requirements defined for both the detected and the not-detected weapon-bar display states, with no third state? [Completeness, Spec §FR-007, §FR-008, §SC-004]
- [x] CHK005 - Is a diagnostic-visibility requirement documented so an absent readout can be distinguished from a decode mismatch? [Completeness, Spec §FR-009]
- [x] CHK006 - Are the safety-critical regression surfaces enumerated as explicit requirements rather than assumed unchanged? [Completeness, Spec §FR-011]
- [x] CHK007 - Is the persistence requirement for hotkey-driven toggles (parity with GUI live session state) documented? [Completeness, Spec §FR-012]

## Requirement Clarity

- [x] CHK008 - Is "once per physical key press" defined precisely enough to exclude auto-repeat, matching the existing newly-pressed guard? [Clarity, Spec §FR-003, Edge Cases]
- [x] CHK009 - Is "while the game window is focused" defined as the sole condition under which hotkey toggles take effect? [Clarity, Spec §FR-004]
- [x] CHK010 - Is the diagnostic's default-level non-spam constraint quantified (no log line per idle sample) versus what is logged (decode + state transitions)? [Clarity, Spec §FR-009, §SC-005]
- [x] CHK011 - Is "no weave-timing/synthesis change" stated as a measurable outcome tied to unchanged existing tests? [Clarity, Spec §FR-010, §SC-006]

## Requirement Consistency

- [x] CHK012 - Do the hotkey toggle requirements follow the bound action (not the literal F1/F2 key), consistently across FR and Edge Cases? [Consistency, Spec §FR-006, Edge Cases]
- [x] CHK013 - Are the suspend and fishing toggle requirements symmetric (same focus, same once-per-press, same GUI reflection, same persistence)? [Consistency, Spec §FR-001..006, §FR-012]
- [x] CHK014 - Is the weapon-bar-on-signal-loss behavior consistent with the fishing-degrades-on-signal-loss safety requirement (both return to a safe/absent state)? [Consistency, Spec §FR-011, Edge Cases]

## Acceptance Criteria Quality

- [x] CHK015 - Are the F1 and F2 success criteria objectively verifiable against the GUI button as the parity reference? [Measurability, Spec §SC-001, §SC-002]
- [x] CHK016 - Is the held-key success criterion (exactly one toggle per press) objectively testable? [Measurability, Spec §SC-003]
- [x] CHK017 - Is the "no other states" weapon-bar criterion phrased so a third or partial state would fail it? [Measurability, Spec §SC-004]
- [x] CHK018 - Can "operator can determine from the log alone" be objectively demonstrated without a debugger? [Measurability, Spec §SC-005]

## Scenario and Edge Case Coverage

- [x] CHK019 - Are requirements defined for the rebound-key scenario (rebound key toggles, old key does not)? [Coverage, Spec §FR-006, Edge Cases]
- [x] CHK020 - Are requirements defined for a B3 block whose marker byte is outside tolerance (treated as no signal, not a misdecode)? [Coverage, Edge Cases]
- [x] CHK021 - Are requirements defined for the unfocused-window case for both F1 and F2 (no state change)? [Coverage, Spec §US1, §US2 scenario 3]
- [x] CHK022 - Are requirements defined for signal loss after a bar was detected (readout returns to not-detected)? [Coverage, Edge Cases]

## Dependencies and Assumptions

- [x] CHK023 - Is the assumption that the slice 014 decode path is structurally correct, with live-signal validation deferred to the operator, documented? [Assumption, Spec Assumptions]
- [x] CHK024 - Is the dependency on the existing derived-view-per-frame GUI (no new layout) documented? [Assumption, Spec Assumptions]
- [x] CHK025 - Is the release-authorization boundary (v0.4.2 tag requires separate explicit authorization) documented as out of automated scope? [Assumption, Spec Assumptions]

## Ambiguities and Conflicts

- [x] CHK026 - Is there any residual ambiguity about which component owns the shared suspend/fishing state once the weave worker no longer swallows the toggle actions? [Ambiguity, Spec §FR-001, §FR-002]
- [x] CHK027 - Is it unambiguous that diagnostics obey the constitution's rule (input contents never logged above DEBUG, never while suspended)? [Ambiguity, Spec Assumptions]

## Notes

- These items test whether the requirements are well written, not whether the
  code works. Implementation verification lives in tasks and tests.
