# Feature Specification: v0.13.0 Release Preparation

**Slice**: S053

**Issue**: #68

**Status**: Implemented

## User Scenarios

### User Story 1: Understand the release quickly

As a user viewing v0.13.0, I want a compact summary of the safety improvements so I can understand the release and reach its assets without scrolling through engineering history.

### User Story 2: Retain the complete release record

As a maintainer, I want the full S048 through S052 changelog preserved so release highlights remain a concise presentation of one authoritative, detailed record.

### User Story 3: Publish only after review

As a release operator, I want the preparation pull request to prove that v0.13.0 is ready without changing versions, creating a tag, or publishing assets before the pull request merges.

## Requirements

- **FR-001**: The Unreleased section MUST begin with exactly one `### Highlights` subsection.
- **FR-002**: Highlights MUST contain one through six top-level bullets totaling no more than 120 words.
- **FR-003**: Highlights MUST summarize user-visible outcomes from S048 through S052: life and world truth, roll-dodge and travel safety, sprint-aware auto-potion, and the System and State disclosure.
- **FR-004**: Highlights MUST omit implementation detail already retained under Added, Changed, and Decisions.
- **FR-005**: The existing release-note generator MUST produce only the Highlights excerpt plus the immutable v0.13.0 changelog link.
- **FR-006**: The complete Unreleased Added, Changed, and Decisions history MUST remain intact and accurately reference its delivered issues.
- **FR-007**: S053 MUST add a detailed changelog entry that records release-candidate preparation without claiming that v0.13.0 was published.
- **FR-008**: `Cargo.toml`, `Cargo.lock`, and the README version badge MUST remain at 0.12.0 in this pull request.
- **FR-009**: S053 MUST NOT run `cargo release`, create or push a tag, publish a GitHub release, or produce release assets.
- **FR-010**: Release-build field verification issues #67 and #69 MUST remain open until a published v0.13.0 build is tested.
- **FR-011**: Spec-kit analysis, release-note contract tests, preview generation, text hygiene, and repository policy checks MUST pass before delivery.

## Assumptions

- v0.13.0 is the next semantic version after v0.12.0.
- The existing concise release-note pipeline from S047 is authoritative and requires no implementation change.
- Release publication is a separate, explicitly authorized post-merge operation governed by `docs/releasing.md`.

## Success Criteria

- The candidate release body contains at most four concise bullets and one full-changelog link.
- Every major user-facing milestone outcome appears once without duplicating detailed engineering prose.
- Versioned files, tags, releases, and assets are unchanged by the pull request.
- A maintainer can merge S053 and immediately run the documented release command without another editorial pass.
