# Feature Specification: v0.19.1 Patch Release Candidate

**Feature Branch**: `codex/s134-v0191-release`

**Created**: 2026-10-17

**Status**: Complete, owner-merged PR #316 on 2026-10-17

**Input**: Implement S134 for v0.19.1, observe established conventions, open a PR for review, and stop before merging, tagging or publishing. Candidate issue #315 is under release coordinator #314 and Plan060.

## User Scenarios & Testing

### User Story 1 - Understand the patch (Priority: P1)

A user assessing v0.19.1 can quickly identify the five fixes delivered since v0.19.0 without reading internal implementation notes.

**Why this priority**: The candidate exists to deliver the owner's observed regressions as a coherent patch.

**Independent Test**: Preview the candidate release notes and compare them with merged S133 issue outcomes and the complete changelog.

**Acceptance Scenarios**:

1. Given the merged S133 fixes, when viewing candidate Highlights, then all five outcomes are represented within one through six bullets and 120 words.
2. Given retained historical release notes, when preparing the patch, then the complete prior release record remains available and the new section preserves all unreleased entries.

### User Story 2 - Review one consistent candidate (Priority: P1)

The maintainer can review a single v0.19.1 identity with its corresponding bundled guidance, fixtures and validation evidence.

**Why this priority**: Conflicting identities can mislabel a build or invalidate the packaged documentation.

**Independent Test**: Compare the candidate's version/date surfaces and run the existing release, repository and build gates.

**Acceptance Scenarios**:

1. Given clean merged main, when preparing the candidate through the governed rollover, then every governed application-version surface agrees on v0.19.1 and the same preparation date.
2. Given independently versioned addon contracts, when the application identity changes, then their declared versions and compatibility meaning remain unchanged.
3. Given the candidate, when validating it, then existing safety and S133 regressions remain enabled and pass alongside the release and documentation checks.

### User Story 3 - Preserve owner release control (Priority: P1)

The owner receives a reviewable PR with evidence and retains final control of merge, tag and publication.

**Why this priority**: The requested deliverable is a candidate PR and the owner explicitly reserved later actions.

**Independent Test**: Inspect the PR, issue/Project state, local/remote tag refs and published-release identity.

**Acceptance Scenarios**:

1. Given completed local checks, when the candidate PR opens, then it closes only #315 upon owner merge and retains #314 for later publication.
2. Given no authorization to merge, tag or publish, when candidate preparation finishes, then the PR is open and unmerged, no v0.19.1 tag exists, and the latest published release remains v0.19.0.
3. Given a failed check or review finding, when the candidate is assessed, then its receipt reports the observed failure and does not claim readiness until the finding is resolved.

### Edge Cases

- A dirty tree prevents the governed rollover rather than silently mixing unrelated changes.
- Missing, empty, malformed or over-budget Highlights fail the existing notes gate.
- A version/date or version-sensitive fixture mismatch blocks handoff.
- An already-existing v0.19.1 tag or release requires investigation before any conflicting operation.
- Repository, synthetic UI and build evidence does not establish installed/game behavior; no field-testing task is created.
- Package format, platform matrix, dependency updates and addon contract changes are outside the patch preparation unless a demonstrated blocker requires a separately recorded decision.

## Requirements

### Functional Requirements

- **FR-001**: Cover all five S133 fixes in concise user-facing Highlights while retaining the complete release history.
- **FR-002**: Prepare the application identity as v0.19.1 through the authoritative local rollover, including its preparation date and all governed downstream references.
- **FR-003**: Preserve addon contract identities, the package/platform matrix, runtime behavior and all required safety/regression coverage.
- **FR-004**: Complete the Spec-Kit sequence and record the exact scope, decisions, checks, failures and limitations without claiming unperformed behavior.
- **FR-005**: Pass existing Rust, release-note, packaging-policy, documentation/render, trust and production-build gates before candidate handoff, with hosted checks and review threads evaluated on the final PR head.
- **FR-006**: Publish an official short-lived-branch PR targeting main with a separate closing reference for candidate #315; retain coordinator #314 for later publication evidence.
- **FR-007**: Stop with the PR open and unmerged, without creating a v0.19.1 tag, publishing packages, deploying the addon or requesting installed/game checks.
- **FR-008**: Preserve #313 housekeeping truth, link Plan060 to S134 and maintain exactly one delivery Project item per issue with evidence-backed Stage and Slice values.

### Key Entities

- Candidate identity: application version, preparation date, source revision and governed version references.
- Release record: concise Highlights plus retained complete release history.
- Delivery record: candidate issue #315, publication coordinator #314, Plan060, S134 packet and review PR.
- Validation receipt: observed check/review results, exact revision and explicit evidence limits.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Five of five S133 issue outcomes appear in at most six Highlights bullets totaling at most 120 words.
- **SC-002**: Zero disagreements remain across governed candidate version/date references.
- **SC-003**: All required pre-handoff checks pass and all review findings are resolved on the final candidate head.
- **SC-004**: Exactly one reviewable candidate PR exists, with zero merge, tag or package-publication actions performed.
- **SC-005**: Both delivery issues appear once in the Project, with candidate and later-publication lifecycles clearly distinguished.

## Assumptions

- The owner's S134 selection authorizes the configured local candidate rollover, commit, push and review PR. The existing configuration disables tag, push and publication inside the rollover.
- S133 on merged main is the complete patch payload; there are no open feature/bug issues or dependency PRs to add as of intake.
- The existing Windows x64 MSI and Linux x86_64 Debian/AppImage/tarball plus SHA256SUMS matrix remains authoritative.
- Published v0.19.0 and historical evidence remain immutable. Candidate preparation is not proof of installed or game behavior.

## Clarifications

### Session 2026-10-17

- Q: Does the owner authorize a local candidate rollover? A: Yes, S134 preparation uses the existing rollover with tag/push/publication disabled; only the explicit PR push is authorized separately by the same request.
- Q: Which issues close on candidate merge? A: Only #315; #314 remains open until separately authorized publication and asset audit.
- Q: Are field checks part of this candidate? A: No. Preserve repository evidence limits and the owner's established no-field-testing direction.

Coverage scan: functional scope, identity/lifecycle, review flow, quality, external
release dependencies, failure/recovery, constraints, terminology and completion
signals are clear. No user question or deferred ambiguity remains.

## Delivery

The owner merged PR #316 as `6a3c95220120978a6decc87bc8e1ce7a63de5742`,
closing candidate #315. Plan060 is [archived](../../docs/archive/build-plans/plan-060.md).
The owner then separately authorized tag/publication and asset verification under
#314. [v0.19.1](https://github.com/h8rt3rmin8r/eso-weave/releases/tag/v0.19.1)
published on 2026-10-17 at 06:25:02 UTC, and its five-asset integrity audit passed.
#314 is Closed/Done and milestone v0.19.1 is closed. See the
[verification receipt](verification.md) for candidate, source and artifact evidence.
No installed/game behavior is established.
