# Feature Specification: v0.16.0 Release Preparation

**Feature Branch**: `codex/s091-v016-release-prep`

**Created**: 2026-09-11

**Status**: Implemented

**Input**: Issue #163: prepare and publish the v0.16.0 game-data and encounter-analysis release.

## User Scenarios & Testing

### User Story 1 - Understand the Release Quickly (Priority: P1)

As a release reader, I can understand the major catalog, encounter-analysis, and
documentation outcomes without reading the complete engineering record.

**Independent Test**: Generate candidate notes and verify that three concise
Highlights cover the complete post-v0.15.1 user outcome set within 120 words.

**Acceptance Scenarios**:

1. **Given** the complete Unreleased history, **When** candidate notes are
   generated, **Then** they contain three user-oriented Highlights and an
   immutable v0.16.0 changelog link.
2. **Given** all 23 merged pull requests since v0.15.1, **When** the detailed
   changelog is audited, **Then** every outcome is represented.

### User Story 2 - Produce a Coherent Release Commit (Priority: P1)

As a release operator, I can use the documented cargo-release command knowing
that every governed version and date surface advances together.

**Independent Test**: Validate exact cargo-release replacement rules and inspect
a v0.16.0 dry run before execution.

**Acceptance Scenarios**:

1. **Given** the current v0.15.1 authorities, **When** v0.16.0 is rolled, **Then**
   Cargo, lockfile, root README, changelog, and bundled documentation agree.
2. **Given** a missing, broad, duplicated, or hard-coded documentation rule,
   **When** documentation policy runs, **Then** it fails before publication.

### User Story 3 - Publish the Authorized Artifacts (Priority: P1)

As a user, I can download the official v0.16.0 Windows and Linux artifacts plus
checksums from the public GitHub Release.

**Independent Test**: Confirm the tag workflow is green and the public release
contains the exact governed artifact set.

**Acceptance Scenarios**:

1. **Given** merged preparation and explicit publication authorization, **When**
   cargo-release executes on `main`, **Then** it pushes one release commit and
   tag without an intermediate public state.
2. **Given** a green tag workflow, **When** the release is inspected, **Then**
   MSI, tarball, Debian package, AppImage, and combined checksums are public.
3. **Given** outstanding installed or live observations, **When** v0.16.0 is
   published, **Then** issues #110, #129, and #131 remain nonblocking.

## Functional Requirements

- **FR-001**: Unreleased MUST begin with exactly one Highlights subsection.
- **FR-002**: Highlights MUST contain three top-level bullets totaling at most
  120 words.
- **FR-003**: Detailed changelog history MUST cover all 23 merged pull requests
  since v0.15.1.
- **FR-004**: The target MUST be v0.16.0 because the release adds substantial
  backward-compatible capabilities without a breaking contract.
- **FR-005**: S091 MUST add exact, cardinality-checked cargo-release rules for
  the documentation snapshot version and date.
- **FR-006**: Documentation policy MUST reject any missing or malformed rollover
  rule, and release-configuration changes MUST trigger that policy in CI.
- **FR-007**: Pinned release-governance edits MUST have a dated changelog
  decision.
- **FR-008**: The preparation pull request MUST leave all current version
  authorities at v0.15.1 and MUST NOT create the release tag.
- **FR-009**: Plan 040 MUST be archived with PR #162 evidence and Plan 041 MUST
  become the sole active chronological plan.
- **FR-010**: After the preparation pull request merges, the authorized release
  command MUST run on clean `main` and the tag workflow MUST complete green.
- **FR-011**: The public release MUST contain the Windows MSI, Linux tarball,
  Debian package, AppImage, and `SHA256SUMS`.
- **FR-012**: Issues #110, #129, and #131 MUST remain independent nonblocking
  verification authorities.
- **FR-013**: The unrelated untracked project-management draft MUST remain
  untouched.

## Success Criteria

- **SC-001**: Candidate output contains exactly three Highlights and no more
  than 120 words.
- **SC-002**: Release rollover policy tests reject every malformed governed
  documentation replacement exercised by the fixture suite.
- **SC-003**: The release commit contains mutually consistent v0.16.0 identity
  and actual cargo-release execution-date surfaces.
- **SC-004**: All required tag-workflow jobs pass and five expected downloadable
  assets are visible on the public release.
- **SC-005**: No verification issue is treated as a repository or publication
  blocker.

## Clarification Decisions

- Use v0.16.0 rather than v0.15.2 because the catalog and encounter workflows
  are backward-compatible feature additions.
- Use three Highlights to map the catalog, encounter, and documentation outcome
  groups without implementation-heavy release prose.
- Repair release rollover during preparation because the current one-command
  path would otherwise create metadata drift that CI is designed to reject.
- Do not claim filesystem transactionality. The public authority is one reviewed
  release commit and tag, preceded by exact replacement checks and a dry run.
- Keep all installed and live-game observations after publication and
  nonblocking.
- Treat the operator's direct instruction to see the release done as a scoped
  exception to the default operator-only merge ritual. Completing the requested
  public release necessarily includes merging its reviewed preparation pull
  request; the exception does not become standing merge authority.
- Close preparation issue #163 when the reviewed pull request merges. Plan 041
  remains active through the separately sequenced release command and public
  artifact inspection.
