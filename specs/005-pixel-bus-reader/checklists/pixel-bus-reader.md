# Requirements Quality Checklist: Pixel Bus Reader

**Purpose**: Validate the clarity, completeness, and consistency of the reader
requirements before planning. Unit tests for the requirements.
**Created**: 2026-07-11
**Feature**: [spec.md](../spec.md)

## Heartbeat and Signal Loss

- [x] CHK001 Is a heartbeat defined precisely (status matches magenta within tolerance)? [Clarity, Spec FR-003]
- [x] CHK002 Is the signal-loss condition and its single-emission behavior defined against a timeout and last heartbeat? [Clarity, Spec FR-004]
- [x] CHK003 Is the no-sample (window unsampleable) case treated as absent status? [Coverage, Spec FR-004, Edge Cases]
- [x] CHK004 Is the recovery behavior (heartbeat resumes, lost state clears) defined? [Completeness, Spec FR-004]
- [x] CHK005 Is it stated that fishing and latency decode only while the heartbeat is present? [Consistency, Spec FR-005]

## Fishing Transitions

- [x] CHK006 Are the three fishing transitions (started, bite, stopped) each defined by a precise color change? [Clarity, Spec FR-006 to FR-008]
- [x] CHK007 Is the transition into waiting from both absent and bite covered? [Coverage, Spec FR-006]

## Latency and Tolerance

- [x] CHK008 Is the latency validation (marker within tolerance and checksum sum) fully specified? [Completeness, Spec FR-009]
- [x] CHK009 Is a failing latency block defined to produce no value? [Coverage, Spec FR-010]
- [x] CHK010 Is the per-channel tolerance defined and applied to all comparisons? [Clarity, Spec FR-002]
- [x] CHK011 Is behavior at the tolerance boundary verifiable? [Measurability, Spec SC-004]

## Events, Testability, and Platform

- [x] CHK012 Is the event set consistent with the fishing detector event set plus latency? [Consistency, Spec FR-011]
- [x] CHK013 Is the decoding and state machine required to be pure and clock-injected for testing? [Measurability, Spec FR-012, SC-005]
- [x] CHK014 Is the platform sampling seam defined with the Wayland exclusion? [Completeness, Spec FR-013]

## Notes

- All items pass. Colors, encoding, tolerance default, and timeout default are the
  fixed contract values from master specification section 9.3.
