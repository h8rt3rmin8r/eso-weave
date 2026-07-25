# Requirements Quality Checklist: Pixel-Bus Block Size Single Source of Truth

**Purpose**: Validate that the geometry-sync contract, safe re-deploy, config
robustness, and no-behavior-change guarantees are completely, clearly, and
consistently specified before planning and implementation.
**Created**: 2026-07-24
**Feature**: [spec.md](../spec.md)

**Note**: These are "unit tests for the requirements" - they check the spec's
quality, not the implementation.

## Geometry Sync Contract

- [x] CHK001 Is the single-source-of-truth value (block size) explicitly named as the sole knob from which all other geometry derives? [Clarity, Spec §FR-001, Key Entities]
- [x] CHK002 Is the block-center derivation formula stated unambiguously and identically for both the reader and the addon? [Consistency, Spec §FR-002, §FR-004]
- [x] CHK003 Are the companion capture-region dimensions specified as derived from block size and block count rather than independent constants? [Completeness, Spec §FR-003]
- [x] CHK004 Is the number of blocks stated as fixed for this feature so the derivation is well-defined? [Clarity, Spec Assumptions]
- [x] CHK005 Is byte-for-byte agreement between the drawn square size and the read points a stated requirement, not just an implication? [Measurability, Spec §FR-005, §SC-002]
- [x] CHK006 Are the block sizes that must be validated (2, 4, 8, 16) enumerated as measurable success criteria? [Acceptance Criteria, Spec §SC-002]

## Safe Re-Deploy and Managed-Marker

- [x] CHK007 Is it required that a re-deploy preserve all addon content except the block-size value, including the managed marker? [Completeness, Spec §FR-005, §SC-003]
- [x] CHK008 Is the managed-only precondition for re-deploy stated (an unmanaged folder is never overwritten)? [Coverage, Spec §FR-008, §SC-004]
- [x] CHK009 Is the not-installed case specified (companion still updates its own geometry, no re-deploy attempted)? [Edge Case, Spec §FR-008, Edge Cases]
- [x] CHK010 Is the trigger for re-deploy defined precisely (automatic on apply, only when the applied size differs from the deployed size)? [Clarity, Spec §FR-007, Clarifications]
- [x] CHK011 Is user feedback required when a re-deploy is skipped because the addon is unmanaged or absent? [Completeness, Spec §FR-008, Acceptance Scenarios US2]
- [x] CHK012 Does the spec avoid any requirement that would weaken the existing uninstall managed-marker verification? [Consistency, Spec Out of Scope, Constitution II]

## Config Robustness

- [x] CHK013 Is the supported block-size set (even integers 2 to 32) explicitly bounded and defined? [Clarity, Spec §FR-010, Clarifications]
- [x] CHK014 Is the correction behavior for invalid values (odd, out of range, wrong type) fully specified, including direction of rounding/clamping? [Completeness, Spec §FR-010, Clarifications]
- [x] CHK015 Is it required that an invalid value produces a non-fatal notice and never a crash? [Measurability, Spec §FR-010, §SC-005]
- [x] CHK016 Is the setting specified as additive with no configuration schema version bump and backward compatible with existing config files? [Consistency, Spec §FR-009]
- [x] CHK017 Is the config text/format discipline (user settings only, JSON, UTF-8 no BOM, LF) upheld by this change per the constitution? [Consistency, Constitution Config Constraints]

## No Behavior Change at Default

- [x] CHK018 Is the default block size fixed at the current value and stated to produce zero observable change for existing and fresh installs? [Completeness, Spec §FR-011, §SC-001]
- [x] CHK019 Is the no-change guarantee tied to a verifiable criterion (existing decoding behavior/tests unchanged)? [Measurability, Spec §SC-001]
- [x] CHK020 Is lowering the default below the current value excluded until the owed in-game validation is complete? [Scope, Spec §FR-012, Out of Scope]

## UX and Discoverability

- [x] CHK021 Is the setting's placement (advanced, existing Pixel Beacon cluster) specified so it is discoverable without a new UI framework? [Clarity, Spec §FR-006, Assumptions]
- [x] CHK022 Is help text that warns a re-deploy is required a stated requirement? [Completeness, Spec §FR-006, Acceptance Scenarios US2]

## Traceability and Scope Boundaries

- [x] CHK023 Are the out-of-scope items (resource blocks, resolution detection/grid wrap, default lowering) explicitly excluded to prevent scope creep? [Scope, Spec Out of Scope]
- [x] CHK024 Is the owed in-game minimum-reliable-size validation recorded as a tracked follow-up rather than a silent gap? [Assumption, Spec §FR-012, Assumptions]

## Notes

- All items are satisfied by the current spec after the clarify pass; this
  checklist is retained as the requirements-quality gate for this slice.
- CHK012 and CHK017 map directly to constitution NON-NEGOTIABLEs (Safety-Critical
  Surfaces; Configuration constraints) and must remain green through analyze.
