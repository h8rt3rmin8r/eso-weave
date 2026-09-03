# Runtime Truth and Input Safety Checklist: Game Runtime and Context Truth

**Purpose**: Validate the requirements that separate installation, runtime,
focus, signal freshness, and game context before they become safety inputs.

**Created**: 2026-09-03

**Feature**: [spec.md](../spec.md)

**Note**: This checklist evaluates requirements quality, not implementation.

## Observation Boundaries

- [x] CHK001 Are installation and runtime specified as independent observations? [Completeness, Spec §FR-001]
- [x] CHK002 Are the allowed installation states and their evidence standard explicit? [Clarity, Spec §FR-002 through §FR-004]
- [x] CHK003 Are the allowed runtime states exhaustive and mutually distinct? [Clarity, Spec §FR-005]
- [x] CHK004 Is game activity explicitly required to outrank launcher presence? [Consistency, Spec §FR-006]
- [x] CHK005 Is launcher presence prevented from implying launcher readiness? [Ambiguity, Spec §FR-007]
- [x] CHK006 Are stale installation paths and vanished processes covered by refresh requirements? [Coverage, Spec §FR-008]
- [x] CHK007 Are contradictory and inaccessible observations given a fail-unknown outcome? [Exception Flow, Spec §FR-009]

## Provider Coverage

- [x] CHK008 Are all supported Windows and Linux distribution providers named? [Completeness, Spec §FR-003]
- [x] CHK009 Is provider evidence required to be validated rather than inferred from an AddOns directory? [Clarity, Spec §FR-004]
- [x] CHK010 Is multiple-installation ambiguity covered rather than resolved by arbitrary priority? [Coverage, Spec §Edge Cases]
- [x] CHK011 Is reviewable evidence required for every claimed provider identity? [Dependency, Spec §SC-002]
- [x] CHK012 Are moved, uninstalled, non-default, and non-ASCII installation paths addressed? [Edge Case, Spec §Edge Cases, §SC-002]

## Context Truth

- [x] CHK013 Are runtime, focus, freshness, and surface required to remain independent? [Completeness, Spec §FR-010]
- [x] CHK014 Is a valid no-menu observation distinguishable from missing evidence? [Clarity, Spec §FR-011]
- [x] CHK015 Are all user-facing Game Context outcomes enumerated? [Completeness, Spec §FR-013]
- [x] CHK016 Is Gameplay limited to the full conjunction of Active, focused, fresh, and valid no-menu evidence? [Clarity, Spec §FR-014]
- [x] CHK017 Is an unidentified surface required to fail closed as Other menu? [Exception Flow, Spec §FR-015]
- [x] CHK018 Are hover and keyboard-focus help requirements equivalent? [Accessibility, Spec §FR-012, §SC-008]

## Dormancy and Recovery

- [x] CHK019 Is the dormant meaning required across every game-derived metric? [Completeness, Spec §FR-016]
- [x] CHK020 Is game inactivity distinguished from PixelBeacon unavailability? [Consistency, Spec §FR-017]
- [x] CHK021 Are requested settings preserved while effective action is blocked? [Clarity, Spec §FR-018]
- [x] CHK022 Are exit, restart, focus recovery, and signal restoration covered? [Recovery Flow, Spec §FR-022, §SC-007]
- [x] CHK023 Is the convergence deadline measurable for both lifecycle and context transitions? [Measurability, Spec §FR-021]

## Safety and Observability

- [x] CHK024 Are transition-only logging and unchanged-sample bounds specified? [Non-Functional, Spec §FR-019, §SC-006]
- [x] CHK025 Is normal-log path privacy explicit? [Privacy, Spec §FR-020]
- [x] CHK026 Are all constitution-protected safety surfaces preserved by requirement? [Consistency, Spec §FR-023]
- [x] CHK027 Are prohibited game-memory, injection, network, launch, and launcher-pixel behaviors explicit? [Boundary, Spec §FR-024]
- [x] CHK028 Is the zero-input dormant success criterion objective and time bounded? [Measurability, Spec §SC-004]

## Scope and Traceability

- [x] CHK029 Are issues #22 and #23 named as the slice outcomes? [Traceability, Spec §Assumptions]
- [x] CHK030 Are quickslot, auto-potion repair, geometry, and dashboard redesign explicitly excluded? [Boundary, Spec §Assumptions]
- [x] CHK031 Is terminology required to remain consistent across the interface and documentation? [Consistency, Spec §FR-025]

## Findings and Disposition

All 31 items pass. The requirements make each raw observation independent,
define the only conjunction that may display Gameplay, and preserve the
project's fail-closed input guarantees. Provider-specific identifiers remain a
planning evidence task rather than an unresolved behavior requirement.
