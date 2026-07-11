# Lifecycle Safety Requirements Checklist: Beacon Manager

**Purpose**: Validate that the requirements governing the safety-critical
filesystem lifecycle surface (discovery, install, verify, uninstall) are
complete, unambiguous, consistent, and measurable before planning and
implementation.
**Created**: 2026-07-11
**Feature**: [spec.md](../spec.md)

## Discovery Requirements

- [x] CHK001 Are the platform resolution paths for the AddOns directory specified for both Windows and Linux (Proton) without assuming a literal path? [Completeness, Spec FR-001/FR-002]
- [x] CHK002 Is the requirement to resolve a relocated Documents folder via the known-folder API stated unambiguously? [Clarity, Spec FR-002]
- [x] CHK003 Are requirements defined for choosing among multiple Steam libraries (locate the library actually containing app id 306130)? [Coverage, Spec Edge Cases]
- [x] CHK004 Is the game-environment selection (`live` default, `pts` selectable) and its effect on the resolved path specified? [Completeness, Spec FR-003]
- [x] CHK005 Is the precedence of a manual override path over auto-discovery stated? [Clarity, Spec FR-004]
- [x] CHK006 Is the not-found outcome defined as a typed result rather than a guessed or created path? [Completeness, Spec FR-005]
- [x] CHK007 Are the operations that become unavailable when the directory is unresolved enumerated? [Coverage, Spec Clarifications]

## Install Requirements

- [x] CHK008 Are the exact files written (manifest plus Lua) and their destination subfolder specified? [Completeness, Spec FR-006]
- [x] CHK009 Is install-over-existing defined as a safe update with a well-defined resulting state regardless of prior contents? [Clarity, Spec FR-007]
- [x] CHK010 Is the requirement that the installed manifest carry the managed-marker line and a `## Version:` equal to the embedded version stated? [Completeness, Spec FR-008]
- [x] CHK011 Is the confinement of all writes to the `PixelBeacon` subtree stated as a hard requirement? [Consistency, Spec FR-009, Constitution II]
- [x] CHK012 Is the precondition that the resolved AddOns directory must exist before any write specified? [Completeness, Spec FR-010]
- [x] CHK013 Are failure and partial-write outcomes defined (reported as failure, nothing partial claimed as success)? [Edge Case, Spec FR-010]

## Verify Requirements

- [x] CHK014 Are all four installed states (NotInstalled, Managed-UpToDate, Managed-VersionMismatch, Unmanaged) enumerated with mutually exclusive predicates? [Completeness, Spec FR-011]
- [x] CHK015 Is the exact predicate for Managed-UpToDate (folder present, marker present, `## Version:` equals embedded) specified? [Clarity, Spec FR-012]
- [x] CHK016 Is the predicate distinguishing Unmanaged (marker line absent) from the managed states unambiguous? [Clarity, Spec FR-013]
- [x] CHK017 Are the NotInstalled conditions (no folder, or folder with no readable manifest) specified? [Completeness, Spec FR-014]
- [x] CHK018 Is the classification of a marker-bearing but malformed/unreadable-version manifest defined (Managed-VersionMismatch, never Unmanaged)? [Edge Case, Spec Edge Cases]

## Uninstall Requirements (Safety-Critical)

- [x] CHK019 Is deletion gated strictly on the managed-marker line being verified present in the on-disk manifest? [Completeness, Spec FR-015, Constitution II]
- [x] CHK020 Is the refusal to delete a marker-less folder or a manifest-less folder stated as a requirement, with the folder left untouched? [Consistency, Spec FR-016]
- [x] CHK021 Is the requirement that nothing outside the resolved `PixelBeacon` folder is ever deleted stated? [Coverage, Spec FR-017, Constitution II]
- [x] CHK022 Is the post-uninstall state (verify reports NotInstalled) specified for the managed case? [Measurability, Spec US3 Acceptance]
- [x] CHK023 Is the marker-verification source unambiguous (the manifest actually on disk, not an assumed or cached value)? [Ambiguity, Spec FR-015]

## Reload Reminder Requirements

- [x] CHK024 Is the condition for surfacing the reload-required reminder (game running or indeterminate) specified? [Clarity, Spec FR-018]
- [x] CHK025 Is it stated that the running-game check never blocks or fails the lifecycle operation? [Consistency, Spec FR-019]
- [x] CHK026 Is the fail-safe direction (surface the reminder when the running state is indeterminate) specified? [Edge Case, Spec Clarifications]

## Testability and Observability Requirements

- [x] CHK027 Are injectable seams (an AddOns root and a running-game check) required so the surface is testable without the real known-folder API or ESO process? [Completeness, Spec FR-020, Constitution III]
- [x] CHK028 Is structured logging of each lifecycle outcome (including the uninstall refusal) required, without logging file contents? [Coverage, Spec FR-021, Constitution logging]

## Consistency and Traceability

- [x] CHK029 Do the safety requirements in the spec align with the constitution's non-negotiable surfaces (marker-gated uninstall; no writes outside AddOns) without weakening them? [Consistency, Constitution II]
- [x] CHK030 Does every functional requirement have at least one corresponding acceptance scenario or measurable success criterion? [Traceability, Spec FR/US/SC]

## Notes

- This checklist tests the quality of the requirements, not the implementation.
- All items were evaluated against the spec as written and pass; the checklist is
  retained as the definition-of-done reference for the pre-push review of the
  safety-critical lifecycle surface.
- The three safety-critical predicates (marker-gated uninstall, writes confined to
  the resolved subtree, deletes confined to the resolved folder) trace directly to
  Constitution Principle II and MUST remain covered by non-weakened tests.
