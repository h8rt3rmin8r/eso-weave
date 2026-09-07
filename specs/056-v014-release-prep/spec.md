# Feature Specification: v0.14.0 Release Preparation

**Slice**: S056

**Issue**: #85

**Status**: Implemented

## User Scenarios

### User Story 1: Understand the release quickly

As a user viewing v0.14.0, I want a compact summary of the dashboard and Ultimate
improvements so I can understand the release and reach its assets without
scrolling through implementation history.

### User Story 2: Retain the complete release record

As a maintainer, I want the full S054 and S055 changelog preserved so the release
highlights remain a concise presentation of one authoritative detailed record.

### User Story 3: Publish only after review

As a release operator, I want the preparation pull request to prove that v0.14.0
is ready without changing versions, creating a tag, or publishing assets before
the pull request merges.

## Requirements

- **FR-001**: The Unreleased section MUST begin with exactly one
  `### Highlights` subsection.
- **FR-002**: Highlights MUST contain one through six top-level bullets totaling
  no more than 120 words.
- **FR-003**: Highlights MUST summarize the user-visible outcomes from S054 and
  S055: cohesive responsive dashboard behavior and exact bar-aware Ultimate
  visibility.
- **FR-004**: Highlights MUST omit implementation detail already retained under
  Added, Changed, and Decisions.
- **FR-005**: The existing release-note generator MUST produce only the
  Highlights excerpt plus the immutable v0.14.0 changelog link.
- **FR-006**: The complete Unreleased Added, Changed, and Decisions history MUST
  remain intact and accurately reference issues #71 through #75.
- **FR-007**: S056 MUST add a detailed changelog entry that records release
  preparation without claiming that v0.14.0 was published.
- **FR-008**: `Cargo.toml`, `Cargo.lock`, and the README version badge MUST remain
  at 0.13.0 in this pull request.
- **FR-009**: S056 MUST NOT run `cargo release`, create or push a tag, publish a
  GitHub Release, or produce release assets.
- **FR-010**: Release verification issue #77 MUST remain open until a published
  v0.14.0 build is tested in ESO.
- **FR-011**: Documentation-site issues #79 through #84 MUST remain outside the
  slice so the current release can publish before that development begins.
- **FR-012**: Spec-kit analysis, release-note contract tests, preview generation,
  text hygiene, and repository policy checks MUST pass before delivery.

## Assumptions

- v0.14.0 is the next semantic version after v0.13.0 because S055 adds a
  backward-compatible user-visible capability.
- The concise release-note pipeline delivered by S047 remains authoritative and
  requires no implementation change.
- Release publication is a separate, explicitly authorized post-merge operation
  governed by `docs/releasing.md`.

## Success Criteria

- The candidate release body contains two concise bullets and one full-changelog
  link.
- Every S054 and S055 user-facing outcome appears once without duplicating
  detailed engineering prose.
- Versioned files, tags, releases, and assets are unchanged by the pull request.
- A maintainer can merge S056 and immediately run the documented release command
  without another editorial pass.
