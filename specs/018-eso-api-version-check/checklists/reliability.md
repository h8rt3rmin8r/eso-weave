# Reliability and Safety Requirements Checklist: ESO API Version Check Automation

**Purpose**: Validate that the safety-critical and reliability requirements for a
networked automation that edits an on-disk addon manifest are complete, clear,
consistent, and measurable before planning and implementation.
**Created**: 2026-07-12
**Feature**: [spec.md](../spec.md)

## Managed-Marker Write Gate

- [x] CHK001 - Is the requirement that every manifest write is gated by the managed marker stated for all write paths, not only uninstall? [Coverage, Spec §FR-007]
- [x] CHK002 - Is the behavior when the managed marker is absent explicitly specified as "do not write"? [Clarity, Spec §FR-007, §US1-3]
- [x] CHK003 - Is the behavior when the manifest file exists but is unreadable specified? [Edge Case, Spec §Edge Cases]
- [x] CHK004 - Is it specified that a marker-preserving update never removes or alters the managed marker line? [Consistency, Spec §FR-008]

## Version Resolution Order

- [x] CHK005 - Is the numeric resolution order (stored last known, then compiled default) stated as an explicit, ordered rule, distinct from the network bump-detection fetch? [Clarity, Spec §FR-005]
- [x] CHK006 - Is the compiled default requirement stated so a version is always available with no network and no stored value? [Completeness, Spec §FR-004]
- [x] CHK007 - Is the persistence location of the last known numeric version and last seen game version specified (session state, not user settings)? [Clarity, Spec §FR-003, §Assumptions]
- [x] CHK021 - Is the behavior on detecting a game version newer than this build knows about specified (record, surface a notice, never guess a number)? [Completeness, Spec §FR-005a]
- [x] CHK022 - Is the fetched network value defined as a game-version detection signal rather than the numeric value written to the manifest? [Clarity, Spec §FR-001, §Clarifications]

## Never-Downgrade Rule

- [x] CHK008 - Is the never-downgrade rule stated for the on-disk manifest? [Completeness, Spec §FR-011]
- [x] CHK009 - Is "differs" defined so that an equal value produces no write and no disk churn? [Measurability, Spec §FR-006, §Edge Cases]
- [x] CHK010 - Is the behavior specified when the discovered version is older than the on-disk value? [Edge Case, Spec §Edge Cases]

## Multi-Value APIVersion Token Handling

- [x] CHK011 - Is the rule for the primary token on rewrite specified? [Clarity, Spec §FR-008a]
- [x] CHK012 - Is the treatment of existing newer tokens (preserve) specified? [Completeness, Spec §FR-008a]
- [x] CHK013 - Is the treatment of existing older tokens (drop) specified? [Completeness, Spec §FR-008a]

## Non-Blocking and Non-Panicking Startup

- [x] CHK014 - Is the requirement that the check runs off the interface thread stated? [Clarity, Spec §FR-001]
- [x] CHK015 - Is the requirement that startup is never blocked by the check, regardless of latency, measurable? [Measurability, Spec §FR-012, §SC-005]
- [x] CHK016 - Is the requirement that network or parse failure never crashes the app stated? [Completeness, Spec §FR-012]
- [x] CHK017 - Is a malformed or unexpected source response required to be treated as no result? [Edge Case, Spec §FR-002, §Edge Cases]

## Write Boundary and Scope

- [x] CHK018 - Is the manifest-write boundary confined to the addon subfolder consistent with the existing install/uninstall guarantee? [Consistency, Spec §US1, plan constitution check]
- [x] CHK019 - Is single-check-per-startup (no continuous polling) stated as an explicit scope boundary? [Coverage, Spec §Assumptions]
- [x] CHK020 - Is the not-installed path required to still discover and store a version for later install? [Completeness, Spec §FR-009, §US2]

## Notes

- All items reference existing spec sections; this checklist is a requirements
  audit, not an implementation test. Items map to the safety-critical surface the
  constitution requires to stay tested (PixelBeacon manifest writes gated by the
  managed marker) and to the reliability requirements FR-001 through FR-012.
