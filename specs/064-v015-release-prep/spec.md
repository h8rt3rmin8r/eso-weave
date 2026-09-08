# Feature Specification: v0.15.0 Release Preparation

**Feature Branch**: `codex/s064-v015-release-prep`

**Created**: 2026-09-08

**Status**: Implemented

**Input**: Issue #102: prepare the v0.15.0 documentation and reliability release candidate without publishing it.

## User Scenarios & Testing

### User Story 1 - Understand the Release Quickly (Priority: P1)

As a release reader, I can understand the major documentation, safety, Linux input, and live-settings improvements without reading the complete engineering record.

**Independent Test**: Generate the v0.15.0 candidate notes and verify that four concise bullets cover all post-v0.14.0 user outcomes within the established item and word budgets.

**Acceptance Scenarios**:

1. **Given** the complete Unreleased history, **When** candidate notes are generated, **Then** they contain only four user-oriented Highlights bullets and the immutable tagged changelog link.
2. **Given** S057 through S063, **When** the bullets are compared with the detailed record, **Then** documentation, safety, Linux input, and live-settings outcomes are each represented without repeating implementation detail.

### User Story 2 - Preserve the Complete Release Record (Priority: P1)

As a maintainer, I can rely on the detailed Unreleased sections as the authoritative record while Highlights remain a compact presentation layer.

**Independent Test**: Compare the pre-S064 Added, Changed, and Decisions content byte-for-byte except for the new S064 preparation entry and chronological plan updates.

**Acceptance Scenarios**:

1. **Given** the existing post-v0.14.0 changelog, **When** S064 is implemented, **Then** every detailed S057 through S063 entry and dated decision remains intact.
2. **Given** the release-note generator, **When** it reads Unreleased, **Then** engineering detail stays out of the generated GitHub release body.

### User Story 3 - Keep Publication Separately Authorized (Priority: P1)

As the release operator, I can merge a reviewed candidate without accidentally changing versions, creating a tag, publishing assets, or closing field-verification work.

**Independent Test**: Verify repository versions, tags, releases, workflows, scripts, and issue states before and after the S064 diff.

**Acceptance Scenarios**:

1. **Given** the S064 pull request, **When** it is inspected, **Then** Cargo, lockfile, and README remain at v0.14.0 and no release artifact exists.
2. **Given** candidate preparation is complete, **When** the pull request merges, **Then** `cargo release 0.15.0 --execute` still remains the separately authorized publication action.
3. **Given** no installed v0.15.0 packages exist yet, **When** S064 completes, **Then** verification issues #77 and #84 remain open.

## Edge Cases

- Detailed changes span documentation, safety, platform behavior, configuration, and dependency-only merges.
- A Highlight becomes a nested list or exceeds the shared 120-word budget.
- A release link points to `main` or Unreleased instead of the immutable v0.15.0 tag.
- Preparation accidentally rolls the version, tag, release, or README badge early.
- Release machinery is edited despite no demonstrated blocker.
- A Highlight claims installed or offline package behavior that still belongs to #84.

## Functional Requirements

- **FR-001**: Unreleased MUST begin with exactly one `### Highlights` subsection.
- **FR-002**: Highlights MUST contain four top-level bullets totaling no more than 120 words.
- **FR-003**: The four bullets MUST cover the public and bundled documentation system, automation and PixelBeacon ownership safety, Linux input parity and fail-closed menu evidence, and live Fishing and Pixel Bus settings behavior.
- **FR-004**: Highlights MUST describe user outcomes and MUST NOT duplicate detailed implementation prose or claim installed-package verification.
- **FR-005**: Existing Added, Changed, and Decisions content for S057 through S063 MUST remain intact.
- **FR-006**: S064 MUST add one detailed Changed entry describing candidate preparation without claiming publication.
- **FR-007**: The existing release-note generator MUST output only Highlights plus the immutable v0.15.0 changelog link.
- **FR-008**: `Cargo.toml`, `Cargo.lock`, and the README badge MUST remain at v0.14.0 in this pull request.
- **FR-009**: S064 MUST NOT run cargo-release, create or push a tag, publish a GitHub Release, or upload assets.
- **FR-010**: Pinned release scripts, workflows, configuration, packaging, and release guidance MUST remain unchanged because no release blocker has been demonstrated.
- **FR-011**: Issues #77 and #84 MUST remain open for installed-release verification.
- **FR-012**: S064 MUST archive completed plan 033 with PR #101 evidence, establish chronological active plan 034 for release preparation, and update the lifecycle ledger and indexes consistently.
- **FR-013**: Release-note contract tests, exact candidate preview, repository scope checks, text hygiene, and applicable CI parity checks MUST pass.
- **FR-014**: The pull request MUST close only issue #102.

## Key Entities

- **Release candidate**: The reviewed Unreleased history and its bounded Highlights presentation for v0.15.0.
- **Highlight**: One top-level, user-oriented release-note bullet contributing to the shared word budget.
- **Publication boundary**: The separation between candidate preparation and the later authorized version, tag, release, asset, and verification lifecycle.

## Success Criteria

- **SC-001**: Candidate output contains exactly four top-level bullets and no more than 120 words.
- **SC-002**: Every post-v0.14.0 user-facing outcome maps to one Highlight category with no unexplained gap.
- **SC-003**: The generated candidate ends with the v0.15.0 immutable changelog link and contains no detailed subsections.
- **SC-004**: All v0.14.0 version references, tags, releases, pinned release machinery, and open verification issues remain unchanged.
- **SC-005**: The detailed changelog retains every pre-S064 S057 through S063 entry and decision.

## Assumptions

- v0.15.0 is the correct next version because the bundled offline documentation action is a new backward-compatible user capability.
- Four Highlights are the smallest structure that represents the distinct post-v0.14.0 user outcomes without collapsing safety or platform behavior into vague prose.
- Dependency-only merges require no user-facing Highlight because the complete repository history retains them and no direct user capability changed.
- The S047 release-note contract and current release machinery remain fit for purpose.

## Clarification Decisions

- Use v0.15.0 rather than v0.14.1 because Help > Documentation adds a backward-compatible feature.
- Use four outcome-oriented bullets rather than one documentation-only bullet because S060 through S062 also shipped after v0.14.0.
- Do not modify release machinery, version references, or verification issues in this preparation slice.
