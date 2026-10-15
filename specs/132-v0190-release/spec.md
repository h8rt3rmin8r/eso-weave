# Feature Specification: v0.19.0 Release Candidate

**Feature Branch**: `codex/s132-v0190-release`
**Created**: 2026-10-15
**Status**: Implemented and Codex-reviewed; owner final review/merge pending in PR #303
**Input**: Owner-authorized S132 for [#302](https://github.com/h8rt3rmin8r/eso-weave/issues/302), under release coordinator [#301](https://github.com/h8rt3rmin8r/eso-weave/issues/301), including [dependency PR #284](https://github.com/h8rt3rmin8r/eso-weave/pull/284). Automatic push, official PR and at most two Codex review rounds are authorized. Owner merge and later tag/publication remain separate.

## User Scenarios & Testing

### User Story 1 - Review a complete and accurately identified candidate (Priority: P1)

The maintainer receives v0.19.0 containing all merged changes since v0.18.0 plus the three dependency updates from #284, with consistent release identity and readable outcome notes.

**Why this priority**: Published v0.18.0 predates combined PixelBeacon, automatic Catalog/native processing, full v15 profile coverage and documentation reliability fixes.
**Independent Test**: Existing automated identity, release-note and locked-dependency checks establish the candidate's content without installing the application.
**Acceptance Scenarios**:

1. **Given** protected main contains S129-S131, **when** S132 is prepared, **then** all those changes and exactly #284's three dependency updates are included.
2. **Given** the candidate identity, **when** its public metadata is read, **then** application version, badge, bundled documentation version/date and version-sensitive fixture agree on v0.19.0 and one preparation date.
3. **Given** release notes, **when** a reader views Highlights, **then** one through six bullets totaling at most 120 words summarize delivered outcomes, with complete release history retained.

### User Story 2 - Find current release guidance and delivery state (Priority: P2)

The maintainer finds release guidance describing the combined PixelBeacon package and complete v15 coverage, with chronological Plan058 and consistent current issue/slice records.

**Why this priority**: Two release pages still describe two embedded addon packages; current planning predates the release kickoff.
**Independent Test**: Existing documentation policy and generated-site checks reject inconsistent links, metadata or lifecycle records.
**Acceptance Scenarios**:

1. **Given** combined PixelBeacon is implemented, **when** release guidance is read, **then** it describes one embedded managed package while retaining the Windows MSI and Linux Debian/AppImage/tarball artifact matrix.
2. **Given** completed Plan057, **when** current planning is read, **then** Plan058 identifies S132/#302 and separate later publication under #301, preserving historical chronology.

### User Story 3 - Retain final merge and publication control (Priority: P3)

The owner receives an official PR with successful final-head checks and resolved reviews, then performs final review and merge. Tag and package publication occur only after later authorization.

**Why this priority**: Preparing and merging release identity does not publish a downloadable artifact.
**Independent Test**: The PR receipt records the final head, all required checks and review dispositions, with the PR open and target tag/release absent.
**Acceptance Scenarios**:

1. **Given** successful local gates, **when** delivery proceeds, **then** the authorized branch and PR are published automatically and all findings are answered and resolved within two Codex rounds.
2. **Given** green final-head checks, **when** handoff occurs, **then** #302 awaits owner merge, #301 remains open and v0.19.0 is untagged and unpublished.
3. **Given** #284's changes are incorporated, **when** handoff occurs, **then** its content and supersession relationship are documented; the agent has not merged either PR on the owner's behalf.

### Edge Cases

- Dirty tree, pre-existing target tag/release, unavailable dependencies or conflicting release identity block advancement until corrected.
- Dependency integration must preserve current-main changes and avoid unrelated lockfile churn.
- Missing or over-budget Highlights must fail the existing notes gate before rollover.
- Failed rollover, CI or reviews require corrected evidence before advancement; no third review round is authorized.
- Candidate evidence cannot establish installation or real-game behavior; no field-testing request or new runtime scope is introduced.

## Clarifications

### Session 2026-10-15

- Q: Merge #284 separately or incorporate its updates? A: Incorporate its exact three dependency changes into the release candidate, preserving one owner merge boundary. Keep #284 open until equivalent content lands on main.
- Q: Does candidate authorization permit local rollover? A: Yes, the named v0.19.0 preparation authorizes the configured local cargo-release command, which creates no tag and publishes nothing.
- Q: Which tracking issue closes? A: Candidate #302 closes on owner merge; release coordinator #301 remains open until later authorized publication and artifact-integrity audit.
- Q: Which evidence is required? A: Existing automated candidate gates and at most two Codex reviews. No new runtime behavior, field-testing task or claim of installed/game behavior is included.
- Coverage: scope, domain/lifecycle, interactions, quality, dependencies, failures, constraints, terminology and completion are Clear. No critical ambiguity remains; the 12-item requirements checklist remains fully passing.

## Requirements

### Functional Requirements

- **FR-001**: The candidate MUST include current-main changes since v0.18.0 and #284's serde_json 1.0.152, rmcp 3.5.1 and tokio-util 0.7.20 updates without unrelated dependency churn.
- **FR-002**: All version-sensitive identity surfaces MUST agree on v0.19.0 and one preparation date through the existing release procedure.
- **FR-003**: Highlights MUST contain 1-6 top-level bullets totaling at most 120 words and preserve complete Added/Changed/Fixed/Decisions history, earlier releases and a fresh Unreleased section after rollover.
- **FR-004**: Release guidance MUST describe one combined embedded PixelBeacon package and all four v15 profiles accurately, preserving independent addon contract versions and the existing asset matrix.
- **FR-005**: Plan058, current indexes, machine/prose ledger and managed agent context MUST identify S132/#302 consistently and retain completed Plan057 history.
- **FR-006**: Existing release-note, packaging-contract, policy, documentation/render, Rust and release-build gates MUST pass; unavailable or failed evidence MUST remain explicit.
- **FR-007**: The official PR MUST close #302 on owner merge, relate to open coordinator #301 and incorporated #284, pass final-head checks and resolve every finding within at most two Codex review rounds.
- **FR-008**: Handoff MUST leave owner merge and separately authorized tag/publication outstanding, with no installed/game evidence claim or field-testing request.

### Key Entities

- **Candidate identity**: Version/date, locked dependency inventory, concise notes and complete release history.
- **Delivery record**: S132, Plan058, candidate #302, coordinator #301 and the incorporated #284 content.
- **Review receipt**: Candidate head, local/hosted checks, review rounds and resolved findings.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Every named identity surface agrees, exactly the three intended dependencies change and all merged post-v0.18.0 outcomes remain represented.
- **SC-002**: Release summaries meet the six-bullet/120-word limits and both release pages describe the implemented package model.
- **SC-003**: Current release links and lifecycle records have zero inconsistencies while historical records remain intact.
- **SC-004**: Required final-head checks succeed and zero review findings or threads remain unresolved within two rounds.
- **SC-005**: One official candidate PR awaits owner merge, with no target tag or release published by this slice.

## Assumptions

- Baseline main is 8cc94d7c169aa85057ec36daf2e8b5f3cc0e1672; published v0.18.0 remains the artifact baseline.
- Existing release tooling, package matrix and safety-critical tests remain authoritative. Runtime source changes and new validation frameworks are unnecessary.
- Owner kickoff authorizes the configured local candidate rollover, push and PR, but not merge, tagging or release publication.
