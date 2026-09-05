# Feature Specification: Concise Release Notes

**Feature Branch**: `codex/047-concise-release-notes`

**Created**: 2026-09-04

**Status**: Implemented

**Input**: Issue #51 and the maintainer requirement that GitHub release pages show a short highlights excerpt before the downloadable assets, with the complete release history retained in and linked from `CHANGELOG.md`.

## User Scenarios & Testing

### User Story 1 - Reach release assets without excessive scrolling (Priority: P1)

As a user viewing a GitHub release, I want a short account of the most important changes so I can understand the release and reach its downloadable assets without traversing the complete engineering changelog.

**Why this priority**: Release pages are download surfaces. Copying the full changelog into the page obstructs their primary action.

**Independent Test**: Generate notes from a representative versioned changelog and confirm the output contains only its Highlights bullets and one full-changelog link within the documented size budget.

**Acceptance Scenarios**:

1. **Given** a versioned changelog section with Highlights, Added, Changed, and Decisions subsections, **When** release notes are generated, **Then** only Highlights and the full-changelog link appear.
2. **Given** six concise Highlights bullets, **When** notes are generated, **Then** all six appear in their original order and the output remains within budget.
3. **Given** detailed Added, Changed, or Decisions prose, **When** notes are generated, **Then** none of that prose is copied to the release body.

---

### User Story 2 - Author release notes from one durable source (Priority: P2)

As a maintainer, I want release highlights to live beside the full changelog so the GitHub release body is reproducible and cannot drift from version-controlled release history.

**Why this priority**: A compact page must not introduce a second manually maintained release-notes document.

**Independent Test**: Change the Highlights subsection in a fixture, regenerate the notes, and confirm that the output changes without editing the workflow or another source file.

**Acceptance Scenarios**:

1. **Given** a valid Highlights subsection, **When** the release workflow runs for that version, **Then** it derives the release body from that subsection.
2. **Given** a missing version, missing Highlights heading, or empty Highlights subsection, **When** generation runs, **Then** it fails with a specific diagnostic before publication.
3. **Given** highlights exceeding the item or word budget, **When** generation runs, **Then** it fails and directs the maintainer to shorten the excerpt.

### Edge Cases

- Changelog files may use LF or CRLF line endings; subsection matching remains stable.
- Markdown links, inline code, and continuation lines inside a bullet remain intact.
- Nested bullets count as content of their top-level highlight, not additional highlights.
- A heading-like string inside a bullet does not terminate extraction unless it begins a Markdown level-three heading.
- Version and repository inputs are validated before they are included in a URL or used for matching.
- An existing full changelog can remain arbitrarily detailed because only Highlights is size-limited.

## Requirements

### Functional Requirements

- **FR-001**: `CHANGELOG.md` MUST remain the complete, authoritative release history.
- **FR-002**: Every releasable version MUST contain an explicit `### Highlights` subsection inside its level-two version section.
- **FR-003**: A repository script MUST extract only the requested version's Highlights content and preserve its Markdown order and formatting.
- **FR-004**: Highlights MUST contain one through six top-level bullets and no more than 120 words in total.
- **FR-005**: Generated notes MUST append one link to the complete `CHANGELOG.md` at the same `vX.Y.Z` tag being published.
- **FR-006**: Generation MUST fail nonzero with an actionable message when the version section, Highlights subsection, highlight content, item budget, word budget, version syntax, or repository syntax is invalid.
- **FR-007**: The release workflow MUST continue validating that the full versioned changelog section exists and is non-empty before building assets.
- **FR-008**: The release workflow MUST use the concise generator for `RELEASE_NOTES.md` instead of copying the complete version section.
- **FR-009**: Automated tests MUST cover valid extraction, exclusion of detailed subsections, stable tagged links, missing and empty content, both budget failures, invalid inputs, and CRLF input.
- **FR-010**: `docs/releasing.md` MUST document the Highlights format, budget, local verification command, and full-changelog behavior.
- **FR-011**: The current Unreleased section MUST include a compliant Highlights excerpt ready to become the v0.12.0 release notes.
- **FR-012**: Pinned workflow and script changes MUST have a dated decision in `CHANGELOG.md`.
- **FR-013**: S047 MUST NOT create a release commit or tag; v0.12.0 publication occurs only after this pull request merges.

### Key Entities

- **Version section**: The complete level-two changelog section for one semantic version, including all detailed subsections.
- **Highlights excerpt**: A bounded list of the release's most important user-facing changes, stored in the version section and suitable for the GitHub release body.
- **Generated release notes**: The Highlights excerpt followed by the immutable tag-specific full-changelog link.
- **Release asset surface**: The GitHub release page area following the release body where packaged downloads are presented.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Generated release notes contain between one and six top-level bullets and at most 120 words before the changelog link.
- **SC-002**: Zero Added, Changed, or Decisions entries appear unless deliberately summarized as a Highlights bullet.
- **SC-003**: Every invalid fixture exits nonzero with a diagnostic naming the violated contract.
- **SC-004**: The v0.12.0 candidate notes are reproducible from `CHANGELOG.md` with one command and contain a tag-specific full-changelog link.
- **SC-005**: Existing Rust behavior, packaged asset formats, and release trigger semantics remain unchanged.

## Assumptions

- Six bullets and 120 words provide a practical compactness ceiling without claiming control over every browser viewport.
- The repository slug is available as `GITHUB_REPOSITORY` in Actions and may be supplied explicitly for local testing.
- The next release version is v0.12.0.
- The detailed changelog may remain long because it is linked rather than embedded in the release page.
