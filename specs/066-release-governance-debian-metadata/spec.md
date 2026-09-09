# Feature Specification: Release Governance and Debian Metadata

**Feature Branch**: `codex/s066-release-governance-debian-metadata`

**Created**: 2026-09-08

**Status**: Ready for Review

**Input**: Issues #104 and #105: clarify when artifact-dependent verification
occurs, add valid Debian maintainer metadata, and prevent future publication of
a package with incomplete required control fields.

## User Scenarios & Testing

### User Story 1 - Sequence Release Verification Truthfully (Priority: P1)

As a release operator, I can publish packages after all pre-publication gates
pass and then perform installed UI, package, field, or platform verification
against the downloadable artifacts that actually exist.

**Independent Test**: Review the amended constitution and aligned operating
guidance and confirm they describe one chronological prepared, published,
release-verification flow without weakening any pre-publication gate.

**Acceptance Scenarios**:

1. **Given** a verification criterion that requires a downloadable artifact,
   **When** the release candidate passes required automated gates, **Then**
   publication may create the artifact before that criterion is evaluated.
2. **Given** a published artifact awaiting field evidence, **When** its separate
   verification issue is inspected, **Then** it remains in Release verification
   until evidence passes or a defect creates linked implementation work.

### User Story 2 - Publish Complete Debian Control Metadata (Priority: P1)

As a Linux package consumer, I receive a `.deb` whose required identity and
maintainer fields are populated without package-manager warnings.

**Independent Test**: Build the package from the configured metadata and query
its control record with `dpkg-deb -f`, proving Package, Version, Architecture,
Maintainer, and Description are all non-empty.

**Acceptance Scenarios**:

1. **Given** the authoritative cargo-deb configuration, **When** a package is
   built, **Then** its Maintainer is `h8rt3rmin8r
   <46768484+h8rt3rmin8r@users.noreply.github.com>`.
2. **Given** a package missing any required control field, **When** the package
   validator runs, **Then** it fails before artifact upload or publication.

### User Story 3 - Preserve the Implementation and Verification Boundary (Priority: P2)

As a maintainer, I can close the metadata implementation issue from repository
evidence while a separate issue waits for the next released package.

**Independent Test**: Inspect #105, the new verification issue, their Project
stages, and the pull request closing references and confirm each owns one
independently closeable lifecycle.

**Acceptance Scenarios**:

1. **Given** S066 has fixed configuration and the pre-publication gate, **When**
   its pull request merges, **Then** #104 and #105 close.
2. **Given** no post-v0.15.0 package exists yet, **When** S066 completes, **Then**
   the new verification issue remains open in Release verification.

## Edge Cases

- `dpkg-deb` is unavailable on a non-Linux developer machine.
- The validator receives no package, more than one argument, or a missing file.
- A required field exists but contains only whitespace.
- cargo-deb changes fallback behavior while explicit metadata remains stable.
- Publication succeeds but later installation or query evidence fails.
- An untracked user-owned document overlaps the topic but is not repository
  content and must remain untouched.

## Functional Requirements

- **FR-001**: The constitution MUST be amended from 2.0.0 to 2.0.1 with a
  complete Sync Impact Report and 2026-09-08 amendment date.
- **FR-002**: The constitution MUST distinguish mandatory pre-publication gates
  from artifact-dependent post-publication verification.
- **FR-003**: Publication MUST NOT be described as proof of installed behavior.
- **FR-004**: Separate release-verification issues MUST remain open until their
  required evidence passes or a defect creates linked implementation work.
- **FR-005**: Existing CI, release-note, safety, packaging, repository, and
  authorization gates MUST remain mandatory.
- **FR-006**: `CLAUDE.md`, build-autopilot, governance, and release guidance
  MUST use the same chronological lifecycle language.
- **FR-007**: `Cargo.toml` MUST set an explicit valid cargo-deb Maintainer value.
- **FR-008**: One reusable Linux validator MUST reject an absent, unreadable, or
  malformed `.deb` and any empty Package, Version, Architecture, Maintainer, or
  Description field.
- **FR-009**: The release workflow MUST run the validator after cargo-deb builds
  the package and before packaging or upload.
- **FR-010**: Pull-request CI MUST exercise the validator's success and failure
  behavior without requiring a release tag.
- **FR-011**: Package name, architecture, description, dependencies, assets,
  desktop entry, icon, udev rule, and binary layout MUST remain unchanged.
- **FR-012**: A new release-verification issue MUST own the next released-deb
  installation and query cycle, and #105 MUST be narrowed to repository evidence.
- **FR-013**: Pinned workflow, script, release-guide, and packaging decisions
  MUST receive a dated Unreleased changelog decision.
- **FR-014**: S066 MUST archive completed plan 035, establish plan 036, and
  update both plan indexes and the migration ledger consistently.
- **FR-015**: The user-owned untracked
  `docs/project/github-native-project-management.md` MUST NOT be edited, staged,
  committed, deleted, or treated as authoritative repository content.
- **FR-016**: Spec-kit analysis, validator tests, documentation policy, text
  hygiene, and full CI parity MUST pass before publication.

## Key Entities

- **Release candidate**: Reviewed source and generated package inputs that have
  passed every mandatory pre-publication gate.
- **Published artifact**: Immutable downloadable package created by the release
  workflow and eligible for artifact-dependent verification.
- **Release-verification issue**: Separate lifecycle record for evidence that
  cannot exist until a published artifact or target environment is available.
- **Debian control contract**: Required non-empty Package, Version,
  Architecture, Maintainer, and Description fields in one `.deb`.

## Success Criteria

- **SC-001**: All aligned governance records describe the same chronological
  pre-publication, publication, and post-publication verification lifecycle.
- **SC-002**: A generated `.deb` contains all five required control fields and
  the exact explicit Maintainer value.
- **SC-003**: Validator fixtures prove one valid package passes and every missing
  required field fails before publication.
- **SC-004**: #104 and #105 can close independently of the new next-release
  verification issue.
- **SC-005**: No runtime Rust source or distributed payload layout changes.
- **SC-006**: Hosted and local quality gates pass with no unresolved findings.

## Assumptions

- `dpkg-deb` is available on Ubuntu GitHub runners and the Linux release runner.
- GitHub's numeric noreply address is a stable public contact for the repository
  owner and avoids publishing a personal mailbox in new package metadata.
- The next release version is intentionally not predicted in the verification
  issue; its entry gate is the first release containing the S066 merge commit.

## Clarification Decisions

- Constitution 2.0.1 is a PATCH because it clarifies chronology without adding,
  removing, or weakening a core principle.
- An explicit `[package.metadata.deb].maintainer` is authoritative because
  cargo-deb otherwise falls back to the first Cargo author and this manifest has
  no authors field.
- One reusable shell validator is shared by pull-request fixtures and the tagged
  release workflow so policy and publication cannot drift.
- #105 closes from configuration, generated-package, and gate evidence. A new
  release-verification issue owns installation and query evidence after the next
  package is downloadable.
- The untracked project-management document is user-owned and excluded. Tracked
  constitution, CLAUDE, autopilot, governance, and release guidance are the
  aligned authority set for S066.
