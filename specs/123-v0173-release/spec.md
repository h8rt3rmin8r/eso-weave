# Feature Specification: v0.17.3 Release Preparation

**Feature Branch**: `codex/s123-v0173-release`
**Created**: 2026-10-06
**Status**: Specification validated
**Input**: Owner-authorized S123 for [issue #255](https://github.com/h8rt3rmin8r/eso-weave/issues/255), following the proposed v0.17.3 candidate and Plan048 archival approach. Automatic push/official PR and at most two Codex review rounds are authorized.

## User Scenarios & Testing

### User Story 1 - Identify the next release accurately (Priority: P1)

A maintainer prepares a v0.17.3 candidate carrying the merged S120-S122 work. Readers see one version/date across the package, documentation and release notes without mistaking a reviewed candidate for a published release.

**Why this priority**: The published v0.17.2 artifact does not contain the recovery and clarity work.
**Independent Test**: Existing automated release-identity and documentation gates reject inconsistent version/date authorities and validate a complete candidate without installing the application.
**Acceptance Scenarios**:

1. **Given** merged S120-S122 changes, **when** the candidate is prepared, **then** all release identity surfaces agree on v0.17.3 and the preparation date, and all prior release history is preserved.
2. **Given** the candidate changelog, **when** compact release notes are generated, **then** one through six highlights totaling at most 120 words summarize the user outcomes and link the full versioned history.
3. **Given** one stale identity surface, **when** automated candidate gates run, **then** inconsistency prevents publication of the candidate PR.

### User Story 2 - Understand completed and current work (Priority: P2)

A maintainer finds Plan048 in the archive with the four delivered figure outcomes and an accurate chronological record of the intervening recovery work. Current planning directs S123 rather than awaiting an already completed S122 merge.

**Why this priority**: Stale lifecycle messages misdirect future work.
**Independent Test**: Repository references and delivery evidence identify each completed figure and the current release-preparation plan without broken paths.
**Acceptance Scenarios**:

1. **Given** PR #254 merged and #221/#223 closed, **when** plans are updated, **then** Plan048 is Complete/Archived, with S112/#220, S121/#222 and S122/#221/#223 delivery evidence.
2. **Given** unused S113-S115 reservations, **when** history is archived, **then** they remain explicitly historical, not completed slices.
3. **Given** the active index, **when** a reader follows the current plan, **then** it names S123, its candidate issue and the owner merge/publication boundaries.

### User Story 3 - Review a candidate before publication (Priority: P3)

The owner receives an official candidate PR with automated evidence and resolved external findings, retaining the final merge decision and later tag authorization.

**Why this priority**: Candidate, merge and published artifact are distinct delivery states.
**Independent Test**: GitHub head-specific checks and review disposition accompany the candidate; no tag or release is created by preparation.
**Acceptance Scenarios**:

1. **Given** explicit push authorization, **when** local gates pass, **then** an official PR is published without another permission pause.
2. **Given** external findings, **when** reviews complete, **then** every finding is answered and completed threads resolved, with no more than two requested Codex rounds.
3. **Given** all final-head checks green and reviews satisfied, **when** the owner is notified, **then** the candidate remains unmerged and untagged for their final review.

### Edge Cases

- A duplicate v0.17.3 tag/release, stale lockfile/fixture/badge, or conflicting dates must be detected before rollover.
- A release highlight omits an important recovery outcome, exceeds its budget or claims unobserved installed behavior.
- Historical plan references use different relative paths after archival; preserve chronology and repair current references.
- Network/CI or external review failures do not count as successful checks; routine authorized fixes/retries remain within scope.
- A release-preparation issue completes on owner merge; later publication does not silently become part of this issue.

## Clarifications

### Session 2026-10-06

- Q: Candidate or package publication? A: Candidate PR only, followed by owner final merge; no tag or release publication in S123.
- Q: Version choice? A: v0.17.3, the proposed patch release for existing behavior repairs and explanatory documentation, with no breaking contract changes.
- Q: Release command authority? A: The accepted release-preparation scope authorizes the existing version rollover operation, which commits locally and neither tags nor pushes; later tag authorization remains separate.
- Q: Issue completion? A: #255 owns candidate preparation and closes on merge. Publication and installed evidence are separate lifecycles; no field issues are reopened.
- Coverage: scope, entities, interactions, quality, integrations, failures, constraints, terminology, completion and placeholders are Clear. No unresolved clarification remains.

## Requirements

### Functional Requirements

- **FR-001**: The candidate MUST identify v0.17.3 consistently across package/lockfile, badge, bundled documentation version/date and the version-sensitive capture request.
- **FR-002**: Release notes MUST include one through six highlights of at most 120 words, preserve the full S120-S122 Added/Fixed/Decisions history and earlier versions, and retain a fresh Unreleased section.
- **FR-003**: Plan048 MUST move to the completed archive with chronological delivery evidence and explicit unused reservation history; current references MUST remain valid.
- **FR-004**: Current planning MUST identify S123 and #255 as candidate preparation with explicit owner merge and separate publication boundaries.
- **FR-005**: The slice MUST retain runtime/addon/service contracts, dependency/tool/workflow pins and existing safety checks; release identity changes MUST NOT be presented as new runtime implementation.
- **FR-006**: Preparation MUST run existing automated release-note, identity, documentation, formatting, lint and test gates, record their actual results and preserve UTF-8/no-BOM/LF text hygiene.
- **FR-007**: The official PR MUST reference #255, receive head-specific green checks, answer all returned findings and resolve completed threads, with at most two requested Codex rounds.
- **FR-008**: No merge, tag, package publication, installed application check or game-session check MAY occur in S123; the handoff MUST distinguish repository evidence from unobserved installed behavior.

### Key Entities

- **Release candidate**: Version/date identity and full reviewed change history, not a published artifact.
- **Build-plan lifecycle**: Historical completed Plan048 and current S123 plan, related through immutable slice/issue/PR evidence.
- **Integration receipt**: Exact candidate head, automated outcomes, review findings/disposition and owner decision boundary.

## Success Criteria

### Measurable Outcomes

- **SC-001**: All named identity surfaces agree on v0.17.3 and one date; no earlier changelog entry is lost.
- **SC-002**: Release highlights stay within six bullets/120 words and cover all three recovery outcomes, encounter/addon clarity and documentation figures.
- **SC-003**: All four Plan048 figure outcomes have merged evidence, zero current references point to its removed location, and unused reservations remain distinguishable.
- **SC-004**: All required automated checks pass on the final candidate head, every returned review finding has a disposition, no thread remains unresolved, and no more than two rounds are requested.
- **SC-005**: The candidate PR is official and unmerged at handoff; no new release/tag or field evidence is claimed.

## Assumptions

- Protected main `56d97ae` contains the S120-S122 owner merges; published v0.17.2 remains the artifact baseline.
- Existing release tooling and package inventory remain authoritative. New dependencies and pipeline changes are unnecessary.
- Owner authorization includes local candidate rollover, push and official PR. It excludes later tag/publication.
- No new issue supersedes #255 at kickoff; independently arriving requests are assessed without silently expanding the release scope.
