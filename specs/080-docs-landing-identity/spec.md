# Feature Specification: Documentation Landing Identity

**Feature Branch**: `codex/s080-docs-landing-identity`
**Created**: 2026-09-10
**Status**: Implemented
**Input**: Issue #121 and the documentation presentation epic #119

## User Scenarios and Testing

### User Story 1 - Recognize the project immediately (Priority: P1)

A reader opens the documentation home page and sees the official full-color ESO
Weave wordmark as the primary visual identity without a second visible product
name competing with it.

**Independent Test**: Build the documentation and inspect the landing page at
narrow and wide viewports in light and dark themes. Confirm that the banner is
local, crisp, responsive, and paired with one accessible level-one heading.

**Acceptance Scenarios**:

1. **Given** the documentation landing page, **when** it loads, **then** the
   official full-color banner wordmark is the first visual identity.
2. **Given** the wordmark already spells ESO Weave, **when** the visible heading
   is read, **then** it says Documentation without visibly repeating ESO Weave.
3. **Given** a screen reader or images-disabled browser, **when** the page outline
   is inspected, **then** the level-one heading still names ESO Weave Documentation.

---

### User Story 2 - Judge documentation applicability (Priority: P1)

A reader can tell which project and release the static manual describes before
following setup or development guidance.

**Independent Test**: Compare the rendered metadata block with `Cargo.toml` and
the matching release heading in `CHANGELOG.md`, then follow the repository link.

**Acceptance Scenarios**:

1. **Given** the landing page, **when** the metadata block is read, **then** it
   names the package handle, applicability version, release date, and repository.
2. **Given** the repository value, **when** its link is followed, **then** it
   reaches the canonical repository declared by package metadata.
3. **Given** the documentation is static, **when** applicability is explained,
   **then** the page identifies the values as a build-time snapshot rather than
   live remote data.

---

### User Story 3 - Prevent silent metadata drift (Priority: P2)

A maintainer changing release metadata receives a deterministic repository check
if the landing-page snapshot is no longer aligned with its authoritative sources.

**Independent Test**: Run focused policy tests against valid metadata and against
mutations of the package name, version, repository, release date, image, heading,
and snapshot disclosure.

**Acceptance Scenarios**:

1. **Given** aligned package, changelog, and landing metadata, **when** policy
   validation runs, **then** it passes without network access.
2. **Given** any required metadata value drifts, **when** policy validation runs,
   **then** it reports the missing or mismatched field.
3. **Given** public Pages and bundled offline builds, **when** either is produced,
   **then** both use the same landing source and local banner asset.

### Edge Cases

- The package version exists in `Cargo.toml` but has no matching dated changelog
  release heading.
- The repository URL is shown as text but is not a clickable link to the exact
  canonical URL.
- The banner is replaced with the square mark, a monochrome logo, or a remote URL.
- A narrow viewport would otherwise make the 2000 by 650 pixel source overflow.
- Hiding the repeated product name accidentally removes it from the accessible
  heading or leaves the decorative wordmark announced twice.
- A later release changes several authoritative values in the same commit.

## Requirements

### Functional Requirements

- **FR-001**: The landing page MUST use a local copy of
  `assets/eso-weave-banner.png` as its primary visual identity.
- **FR-002**: The landing page MUST NOT use the square application mark or either
  monochrome wordmark in the primary identity.
- **FR-003**: The visible level-one heading MUST say Documentation without visibly
  repeating the ESO Weave wording already present in the banner.
- **FR-004**: The level-one heading MUST retain the accessible name ESO Weave
  Documentation when visually hidden text is included.
- **FR-005**: The banner image MUST be decorative to assistive technology because
  the adjacent accessible heading supplies its textual equivalent.
- **FR-006**: A semantic metadata block MUST appear after the identity and before
  the introductory prose.
- **FR-007**: The metadata block MUST show the official handle, applicability
  version, release date, and canonical repository URL.
- **FR-008**: `Cargo.toml` package name MUST be the handle authority.
- **FR-009**: `Cargo.toml` package version MUST be the applicability-version
  authority.
- **FR-010**: `Cargo.toml` package repository MUST be the repository authority.
- **FR-011**: The dated `CHANGELOG.md` heading matching the package version MUST be
  the current-release date authority.
- **FR-012**: The page MUST explain that metadata is a build-time snapshot and
  identify the source and update rule without implying a live query.
- **FR-013**: Repository policy MUST reject a missing, malformed, remote, or wrong
  banner and any metadata value that differs from its authority.
- **FR-014**: Focused tests MUST prove the validator accepts the complete contract
  and rejects each required drift case.
- **FR-015**: The banner and metadata layout MUST avoid page-level horizontal
  overflow at 320 CSS pixels and remain legible in supported light and dark themes.
- **FR-016**: Public and bundled offline documentation MUST use the same Markdown,
  CSS, local image asset, and build-time values.
- **FR-017**: The slice MUST add no runtime application behavior, network fetch,
  telemetry, game interaction, JavaScript dependency, or release action.

### Key Entities

- **Landing identity**: The official full-color banner plus the single accessible
  page heading.
- **Documentation snapshot**: The handle, applicability version, release date,
  and repository values rendered at build time.
- **Metadata authority**: The package fields and dated changelog release heading
  against which policy validates the snapshot.
- **Local banner asset**: The repository-owned PNG copied into the mdBook source
  tree for both public and offline builds.

## Success Criteria

### Measurable Outcomes

- **SC-001**: The built landing page contains exactly one local full-color banner
  identity and no primary square or monochrome mark.
- **SC-002**: The rendered H1 visibly exposes Documentation and has the accessible
  name ESO Weave Documentation.
- **SC-003**: All four metadata values exactly match their repository authorities,
  and the canonical repository is clickable.
- **SC-004**: Focused policy tests reject independent mutations of every identity,
  metadata, and snapshot-disclosure obligation.
- **SC-005**: The landing page has no page-level horizontal overflow at 320 CSS
  pixels and the banner never exceeds its content container.
- **SC-006**: Documentation tests, build, links, generated-site policy, spelling,
  UTF-8, forbidden-dash, and mojibake checks pass.

## Clarifications

### Session 2026-09-10

- Q: Which project facts are authoritative? A: Use the package name, version, and
  repository in `Cargo.toml`, plus the matching dated release heading in
  `CHANGELOG.md`.
- Q: Should the page query GitHub for current release data? A: No. The public and
  bundled manuals share an offline-capable build-time snapshot enforced against
  repository sources.
- Q: How should the duplicate visible name be removed without weakening the page
  outline? A: Keep ESO Weave inside a visually hidden span in the Markdown H1,
  expose Documentation visibly, and treat the adjacent wordmark image as decorative.
- Q: Should a derived banner be generated? A: No. Copy the existing approved
  2000 by 650 full-color PNG byte-for-byte into the mdBook source tree.
- Q: Is release publication included? A: No. Version metadata is descriptive only;
  S080 does not tag, publish, or alter release configuration.

## Dependencies

- Issue #121 owns this slice and has no native blockers.
- Issue #119 is the parent documentation presentation epic.
- S079 provides the current documentation policy and responsive-theme foundation.
- Issue #122 follows with broader Brand Standard asset and swatch work.

## Out of Scope

Brand Standard swatches, new logo generation, runtime application branding,
packaging assets, screenshots, diagrams, table redesign, release publication,
dynamic GitHub API data, analytics, and remote content.
