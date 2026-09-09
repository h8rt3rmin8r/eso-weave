# Research: Release Governance and Debian Metadata

## Decision 1: Use explicit cargo-deb metadata

**Decision**: Set `maintainer = "h8rt3rmin8r
<46768484+h8rt3rmin8r@users.noreply.github.com>"` under
`[package.metadata.deb]`.

**Rationale**: cargo-deb documents that `maintainer` controls the Debian field
and otherwise falls back to the first Cargo author. ESO Weave has no Cargo
authors field, which explains the v0.15.0 omission. The explicit project-scoped
noreply address is stable and avoids a new personal-mailbox disclosure.

**Primary source**: <https://github.com/kornelski/cargo-deb#package-metadatadeb-options>

## Decision 2: Validate the built control record

**Decision**: Add `scripts/validate-debian-package.sh` and exercise it with
packages assembled by `dpkg-deb` in a shell fixture suite.

**Rationale**: Parsing Cargo metadata alone would repeat cargo-deb's assumptions.
Querying the actual package control record validates the publication object and
catches empty fields regardless of how cargo-deb produced them.

## Decision 3: Share one gate

**Decision**: Run the fixture suite in Linux pull-request CI and run the same
validator on `target/debian/*.deb` in the tagged release workflow immediately
after construction.

**Rationale**: Pull requests prove validator behavior, while the release gate
proves the real artifact. Reusing one executable contract prevents drift.

## Decision 4: Clarify rather than weaken release law

**Decision**: Amend the constitution to 2.0.1 and align tracked operating guides
around three states: pre-publication gates, artifact publication, and separate
artifact-dependent verification.

**Rationale**: Installed verification cannot precede the downloadable artifact,
but this does not excuse CI, safety, packaging, release-note, authorization, or
repository failures before publication. A PATCH accurately represents a
chronology clarification.

## Decision 5: Split #105 by lifecycle

**Decision**: Close #105 when S066 supplies the metadata and prevention gate,
and create a separate verification issue whose entry gate is the first release
containing S066.

**Rationale**: Keeping implementation open until a later release contradicts
the lifecycle rule being clarified in #104. The future issue can fail into new
implementation work without reopening completed repository changes.

## Alternatives Rejected

- Add a Cargo `authors` field: rejected because it changes general crate
  metadata merely to obtain one Debian control value and remains an implicit
  fallback.
- Validate only `Cargo.toml`: rejected because it cannot prove the generated
  control file.
- Validate only after GitHub Release publication: rejected because a malformed
  package would already be downloadable.
- Keep #105 open until the next release: rejected because implementation and
  release verification finish at different times.
- Edit the untracked project-management document: rejected because it is
  user-owned workspace content outside the repository authority chain.
