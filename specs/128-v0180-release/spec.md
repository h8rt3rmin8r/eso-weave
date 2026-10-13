# Feature Specification: v0.18.0 Release Preparation

**Feature Branch**: `codex/s128-v0180-release`
**Created**: 2026-10-13
**Status**: Implemented; owner-merged and published in v0.18.0
**Input**: Owner-authorized S128 for [issue #279](https://github.com/h8rt3rmin8r/eso-weave/issues/279), preparing the merged S125-S127 work as a reviewed v0.18.0 candidate. Automatic push, official PR and at most two external review rounds are authorized.

## User Scenarios & Testing

### User Story 1 - Identify the release candidate accurately (Priority: P1)

A maintainer prepares v0.18.0 containing the recording workflow repairs, retained sessions and completed native import. Readers find one version and date across package identity, documentation and release history, with clear user outcomes.

**Why this priority**: Published v0.17.4 predates all three delivered slices.
**Independent Test**: Existing automated release identity and notes gates reject inconsistent metadata or incomplete notes without installing the application.
**Acceptance Scenarios**:

1. **Given** merged S125-S127, **when** preparation completes, **then** package identity, badge, bundled documentation and version-sensitive fixture agree on v0.18.0 and the preparation date.
2. **Given** release notes, **when** readers view the summary, **then** one through six bullets totaling at most 120 words cover workflow repairs, retained sessions and native import, while the full dated history remains available.
3. **Given** conflicting identity, duplicate version/tag or malformed notes, **when** preparation gates run, **then** the candidate cannot advance until the inconsistency is corrected.

### User Story 2 - Find completed and current work (Priority: P2)

A maintainer finds Plan053 archived with the actual S127 merge and a chronological current Plan054 directing S128 release preparation.

**Why this priority**: Planning still describes the already merged S127 as awaiting owner merge.
**Independent Test**: Plan indexes and the machine/prose migration ledger consistently name completed S127 and active S128 with working references.
**Acceptance Scenarios**:

1. **Given** PR #276 merged at 05c97041b9d12f5e5c10fae14ba1b80511c5974d, **when** planning is refreshed, **then** Plan053 is Complete/Archived and records the owner merge.
2. **Given** the active planning index and ledger, **when** a reader follows them, **then** they identify Plan054, S128 and #279 as candidate preparation with separate later publication.
3. **Given** historical S125-S127 records, **when** the archive is updated, **then** chronology and delivery evidence remain intact.

### User Story 3 - Review before publication (Priority: P3)

The owner receives an official candidate PR with successful automated evidence and resolved reviews, retaining final review/merge and later tag/publication decisions.

**Why this priority**: A candidate, a merged identity and a published downloadable artifact are different states.
**Independent Test**: The official PR has successful checks on its final head and no unresolved review threads; no tag or release is created by this slice.
**Acceptance Scenarios**:

1. **Given** explicit publication authorization, **when** local gates pass, **then** the branch and official PR are published without another approval pause.
2. **Given** returned review findings, **when** review completes, **then** each finding is answered and completed threads resolved, within two rounds.
3. **Given** green final-head checks and satisfied reviews, **when** the owner is notified, **then** the candidate remains unmerged and untagged.

### Edge Cases

- An existing v0.18.0 local/remote tag or release, dirty tree, stale fixture or conflicting metadata must be detected before rollover.
- Notes may overclaim native format coverage, live tailing, installed behavior or platform parity. They must describe only delivered capability and preserve explicit source limitations.
- Archiving changes relative paths; current links must resolve without rewriting frozen historical evidence.
- Tool, network, CI or review failures require actual corrected evidence, not an assumed pass. A failed rollover must leave a reviewable state before retry.
- New issues arriving during preparation are assessed separately rather than silently expanding release scope.

## Clarifications

### Session 2026-10-13

- Q: Version and scope? A: The owner selected v0.18.0 candidate preparation for S125-S127; the native import and retained-session expansion justify the proposed minor version.
- Q: Candidate or artifact publication? A: The existing local rollover is authorized as preparation; final owner merge and later annotated tag/package publication remain separate.
- Q: Issue completion? A: #279 closes with repository evidence on owner merge of this candidate. Parent release tracker #277 stays open until separately authorized publication completes. No installed/game evidence or field-verification issue is added.
- Q: Validation strategy? A: Reuse existing automated release, documentation/render, Rust and production-build gates; no runtime implementation or new tests that mirror metadata are needed.
- Coverage: scope, entities/lifecycle, interactions, quality, dependencies, failures, constraints, terminology and completion are Clear; no unresolved clarification remains.

## Requirements

### Functional Requirements

- **FR-001**: The candidate MUST identify v0.18.0 and one preparation date consistently across package/lockfile, badge, bundled documentation metadata and the version-sensitive capture request.
- **FR-002**: Notes MUST contain one through six Highlights bullets totaling at most 120 words, cover all S125-S127 user outcomes and retain their complete Added/Fixed/Decisions history plus all earlier releases and a fresh Unreleased section.
- **FR-003**: Plan053 MUST be archived Complete with PR #276 merge evidence, repaired relative references and preserved chronology.
- **FR-004**: Plan054, current indexes and migration ledger MUST agree on S128/#279 as candidate preparation, owner final merge and separate tag/publication.
- **FR-005**: Runtime, addon, store, service, dependency and tool contracts MUST remain unchanged; native normal/separate version15 support and manual provenance MUST NOT become claims of other variants, tailing or platform parity.
- **FR-006**: Preparation MUST follow the existing reviewed release procedure, detect existing target identity, use automated gates, record actual outcomes and preserve UTF-8/no-BOM/LF text.
- **FR-007**: The official PR MUST link #279, pass checks on the final head, answer every finding, resolve completed threads and request no more than two review rounds.
- **FR-008**: The handoff MUST leave the PR unmerged with no new tag/package publication, no owner installed/game check request and no assertion of unobserved runtime behavior.

### Key Entities

- **Release candidate**: One version/date, complete delivered change history and concise outcome notes; a preparation state, not a published artifact.
- **Plan lifecycle**: Completed Plan053 and active Plan054 linked to exact slice, issue and owner-merged PR evidence.
- **Integration receipt**: Exact candidate head, gate results and review dispositions, followed by the owner's final decision.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Every named identity surface agrees on v0.18.0 and one date, with zero target-identity conflicts before rollover.
- **SC-002**: The six-bullet/120-word budget is met and all three slice outcomes and earlier history remain discoverable.
- **SC-003**: Archived S127 has its exact merge receipt; all current S128 references resolve with zero contradictory lifecycle states.
- **SC-004**: All required checks pass on the final candidate head, every returned review finding has a disposition and no thread remains unresolved, within two rounds.
- **SC-005**: The delivered PR is official and unmerged, and no new tag, published package or field-evidence claim occurs in S128.

## Assumptions

- Protected main 05c97041b9d12f5e5c10fae14ba1b80511c5974d contains S125-S127. Published v0.17.4 remains the artifact baseline.
- Existing release tooling and package inventory remain authoritative; pipeline changes and new dependencies are unnecessary.
- The owner kickoff authorizes the local candidate rollover, push and official PR, while final merge and later tag/publication stay separate.
