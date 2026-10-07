# Release Governance Requirements Checklist

Purpose: Assess release identity, history and publication authority requirements.
Created: 2026-10-06
Audience: Candidate author and PR reviewer, before implementation.

## Identity and History

- [x] CHK001 Are all version/date authorities explicitly enumerated? [Spec FR-001]
- [x] CHK002 Is the highlight budget quantified and the complete history preserved? [Spec FR-002, SC-002]
- [x] CHK003 Are duplicate version and stale metadata failure cases defined? [Spec Edge Cases]
- [x] CHK004 Is archival chronology distinct from unused slice reservations? [Spec FR-003, SC-003]
- [x] CHK005 Are active planning and candidate completion boundaries consistent? [Spec FR-004, US2/US3]

## Authority and Evidence

- [x] CHK006 Does the scope distinguish local rollover from tag/publication authority? [Spec Clarifications, FR-008]
- [x] CHK007 Are all existing safety and automated gates retained? [Spec FR-005/006]
- [x] CHK008 Is the exact-head review/CI completion condition specified? [Spec FR-007, SC-004]
- [x] CHK009 Is the two-round cap explicit for corrections and clean reviews? [Spec FR-007]
- [x] CHK010 Are installed behavior claims and field checks excluded? [Spec FR-008, SC-005]
- [x] CHK011 Is the candidate issue lifecycle separately closeable from publication? [Spec Clarifications, Edge Cases]
- [x] CHK012 Are scope failures and retry outcomes distinguished from successful evidence? [Spec Edge Cases, FR-006/007]

Result: 12/12 requirements-quality items pass. No mandatory hooks exist. The
checklist prerequisite helper expects plan.md prematurely; its PathsOnly output
provided the valid S123 spec path before the ordered plan step.
