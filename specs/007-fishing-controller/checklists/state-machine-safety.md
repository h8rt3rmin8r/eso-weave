# State Machine and Timing Safety Requirements Checklist: Fishing Controller

**Purpose**: Validate that the requirements governing the fishing controller
state machine and its safety-critical timing are complete, unambiguous,
consistent, and measurable before planning and implementation.
**Created**: 2026-07-11
**Feature**: [spec.md](../spec.md)

## State and Transition Completeness

- [x] CHK001 Are all five states (Disabled, Armed, Waiting, Reeling, Recast) enumerated with an entry condition each? [Completeness, Spec FR-003]
- [x] CHK002 Is the transition defined for every event (toggle on, toggle off, FishingStarted, BiteDetected, FishingStopped, SignalLost, Heartbeat) from each state, or is it stated that the event is ignored there? [Coverage, Spec FR-003 to FR-011]
- [x] CHK003 Is the Recast auto-continue path versus the Armed retry path unambiguous (Recast on FishingStarted returns to Waiting; Recast timeout returns to Armed)? [Clarity, Spec FR-008a, Clarifications]
- [x] CHK004 Is the Heartbeat event's effect specified (observed, no state change)? [Clarity, Spec Edge Cases]
- [x] CHK005 Is the defensive handling of BiteDetected while Armed (before FishingStarted) specified? [Edge Case, Spec Edge Cases]

## Timing and Deadline Clarity

- [x] CHK006 Are the three parameters (arm_timeout_ms, reel_delay_ms, recast_delay_ms) each given a name, meaning, and default? [Completeness, Spec FR-014]
- [x] CHK007 Is each delay defined as a deadline evaluated against an injected clock rather than a blocking sleep? [Clarity, Spec FR-003, Clarifications]
- [x] CHK008 Is the arm timeout applied consistently to both the initial Armed cast and the Recast-awaiting-FishingStarted case? [Consistency, Spec FR-005, FR-008a]
- [x] CHK009 Is the firing rule at the exact deadline boundary specified (a tick at or past the deadline fires it)? [Measurability, Spec Edge Cases]

## Safety: Signal Loss and Deadline Cancellation

- [x] CHK010 Is the requirement that SignalLost from any active state transitions to Disabled and emits no interact input stated unambiguously? [Completeness, Spec FR-009, Constitution II]
- [x] CHK011 Is it required that leaving the state that scheduled a delayed interact cancels that pending interact so no queued input fires afterward? [Consistency, Spec FR-010, Constitution II]
- [x] CHK012 Is the no-input-while-Disabled invariant stated as a requirement? [Completeness, Spec FR-011]
- [x] CHK013 Are the cancellation triggers enumerated (SignalLost, FishingStopped, toggle off, any other transition out of the scheduling state)? [Coverage, Spec FR-010]
- [x] CHK014 Is it specified that after SignalLost the controller emits nothing until the player toggles on again? [Clarity, Spec US2 Acceptance]

## Toggle and Idempotency

- [x] CHK015 Is the toggle defined to arm from Disabled (with a single cast) and disarm from any state? [Completeness, Spec FR-004]
- [x] CHK016 Is toggle idempotency specified (redundant on does not re-cast; redundant off emits nothing)? [Clarity, Spec FR-004, Edge Cases]

## Input Synthesis Seam

- [x] CHK017 Is the interact emission defined as a key press followed by a key release through a sink seam, using the same input-operation representation as the weave engine? [Completeness, Spec FR-012]
- [x] CHK018 Is it stated that the controller never drives a real input device directly? [Consistency, Spec FR-012, Constitution V]

## Testability

- [x] CHK019 Is testability required via an injected clock, a mock sink recording operations, and a stub detector, with no real device/game/wall-clock dependency? [Completeness, Spec FR-013, Constitution III]
- [x] CHK020 Are the success criteria for the cast-reel-recast cycle and the signal-loss safety expressed as measurable outcomes? [Measurability, Spec SC-001, SC-002]

## Configuration

- [x] CHK021 Is the fishing configuration required to persist as an additive settings section with documented defaults on absence? [Completeness, Spec FR-015]
- [x] CHK022 Is the fallback-to-default-with-notice behavior for an invalid timing value specified? [Edge Case, Spec FR-015, US4 Acceptance]

## Detector Abstraction

- [x] CHK023 Is the BiteDetector event set defined exactly (Heartbeat, FishingStarted, BiteDetected, FishingStopped, SignalLost), and is the PixelBusDetector's dropping of Latency stated? [Completeness, Spec FR-001, FR-002]
- [x] CHK024 Is the controller's dependency on the abstraction (not a concrete detector) stated so future detectors need no controller change? [Consistency, Spec FR-001]

## Consistency and Traceability

- [x] CHK025 Do the safety requirements align with the constitution's fishing-degrades-to-disabled-on-SignalLost surface without weakening it? [Consistency, Constitution II]
- [x] CHK026 Does every functional requirement have at least one acceptance scenario or measurable success criterion? [Traceability, Spec FR/US/SC]

## Notes

- This checklist tests the quality of the requirements, not the implementation.
- All items were evaluated against the spec as written and pass; it is retained as
  the definition-of-done reference for the pre-push review of the safety-critical
  state machine.
- The SignalLost-disables-fishing and pending-deadline-cancellation requirements
  trace to Constitution Principle II and MUST remain covered by non-weakened tests.
