# Requirements Quality Checklist: Fishing Capture Hardening and Diagnostics

**Purpose**: Validate that the requirements for the screen capture, the
diagnostics, and the safety constraints are complete, clear, and consistent before
planning.
**Created**: 2026-07-13
**Feature**: [spec.md](../spec.md)

## Requirement Completeness

- [ ] CHK001 - Is the capture-correctness requirement stated in terms of reading what is displayed (including accelerated content)? [Completeness, Spec FR-001]
- [ ] CHK002 - Is the end-to-end fishing advance (Casting to Fishing to Reeling) tied to reading the heartbeat and waiting signal? [Completeness, Spec FR-002]
- [ ] CHK003 - Are both diagnostic outputs specified (per-sample verbose detail and heartbeat acquire/lose messages)? [Completeness, Spec FR-004, FR-005]
- [ ] CHK004 - Is the decode-preservation requirement explicit (positions, colors, tolerance, events, tests unchanged)? [Completeness, Spec FR-003]
- [ ] CHK005 - Is the spec-reconciliation requirement captured (update section 10.3, dated decision)? [Completeness, Spec FR-009]

## Requirement Clarity

- [ ] CHK006 - Is "reads the heartbeat as present" objectively checkable (the status block reads magenta, not black)? [Clarity, Spec FR-002, SC-002]
- [ ] CHK007 - Is the culprit-disambiguation outcome unambiguous (never-present heartbeat vs never-blue fishing block)? [Clarity, Spec FR-006, SC-003]
- [ ] CHK008 - Is "outside the game" defined (reads on-screen pixels only, no memory or packets)? [Clarity, Spec FR-008]

## Requirement Consistency

- [ ] CHK009 - Is the capture change consistent with the unchanged degrade-on-signal-loss behavior? [Consistency, Spec FR-002, FR-007]
- [ ] CHK010 - Is the deliberate non-change of the addon interaction detection and keypress synthesis consistent with keeping the capture fix isolable? [Consistency, Spec FR-010]
- [ ] CHK011 - Is the safety-surface non-change consistent across the input engine and the beacon uninstall guarantee? [Consistency, Spec FR-010, Constitution II]

## Acceptance Criteria Quality

- [ ] CHK012 - Are the success criteria measurable, including the in-game advance-past-Casting outcome and the log-evidence disambiguation? [Measurability, Spec SC-001..SC-005]
- [ ] CHK013 - Is the automated-test expectation stated (existing decoder/reader tests pass, new pixel-extraction helper tested)? [Measurability, Spec FR-011, SC-004]

## Scenario & Edge Case Coverage

- [ ] CHK014 - Is the occluded/minimized/unfocused case covered (no heartbeat, degrade to disabled, log shows it)? [Coverage, Edge Case, Spec FR-007]
- [ ] CHK015 - Is the no-bait precondition covered as a documented cause of the identical symptom? [Coverage, Edge Case]
- [ ] CHK016 - Is the transient capture failure covered (treated as an absent block, recovers next sample)? [Coverage, Edge Case, Spec FR-003]
- [ ] CHK017 - Is the addon-out-of-date/not-loaded case covered in the validation protocol? [Coverage, Edge Case]

## Constitution & Scope

- [ ] CHK018 - Is the outside-the-game boundary and the log-only (no new interface) scope stated? [Assumption, Spec FR-006, FR-008, Constitution V]
- [ ] CHK019 - Is the tested surface (decoders, reader, pixel-extraction helper) identified while the end-to-end fix is validated in-game? [Assumption, Spec FR-011, Constitution III]

## Notes

- Items validate requirement quality, not the eventual code. All reference a spec
  section or an edge case/assumption for traceability.
