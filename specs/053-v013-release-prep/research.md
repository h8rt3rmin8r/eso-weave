# Research: v0.13.0 Release Preparation

## Decision 1: Reuse the S047 release-note contract

**Decision**: Add a compliant Highlights subsection and make no release-script or workflow changes.

**Rationale**: `scripts/release-notes.sh` already extracts only Highlights, enforces one through six bullets and 120 words, and appends the immutable tagged changelog link. Editing proven machinery would add risk without improving this release.

## Decision 2: Organize highlights by user outcome

**Decision**: Use four bullets covering lifecycle truth, transient-action safety, sprint-aware auto-potion, and interface organization.

**Rationale**: This covers every S048 through S052 outcome while avoiding a chronological list of implementation slices.

## Decision 3: Keep the pull request pre-release

**Decision**: Do not bump version references or create a tag in S053.

**Rationale**: The documented `cargo release 0.13.0 --execute` command atomically rolls the version, changelog, badge, commit, and tag after the preparation pull request merges. Performing part of that rollover in a pull request would create two authorities.

## Alternatives Rejected

- Copy all Added and Decisions entries into Highlights. Rejected because users would again need to scroll past engineering history to reach assets.
- Modify the generator for this release. Rejected because the current contract already produces the required shape.
- Include field-verification claims. Rejected because #67 and #69 require the published build and remain open.
- Run the release command on the feature branch. Rejected because release governance permits it only on `main` after explicit publication authorization.
