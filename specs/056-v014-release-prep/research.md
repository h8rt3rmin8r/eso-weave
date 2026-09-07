# Research: v0.14.0 Release Preparation

## Decision 1: Release before documentation-site development

**Decision**: Prepare and publish v0.14.0 before implementing documentation-site
issues #79 through #84.

**Rationale**: The release gives #77 an installable S055 build to verify while
documentation development proceeds independently. Mixing site work into the
candidate would delay that field evidence and widen the release scope.

## Decision 2: Use a minor version

**Decision**: Target v0.14.0.

**Rationale**: S054 is compatible UI refinement, while S055 adds the new exact
Ultimate meter and advances the bundled PixelBeacon protocol compatibly. A minor
version communicates the added capability without implying a breaking change.

## Decision 3: Reuse the S047 release-note contract

**Decision**: Add a compliant Highlights subsection and make no release-script or
workflow changes.

**Rationale**: `scripts/release-notes.sh` already extracts only Highlights,
enforces one through six bullets and 120 words, and appends the immutable tagged
changelog link. Editing proven machinery would add risk without improving this
release.

## Decision 4: Organize highlights by user outcome

**Decision**: Use two bullets covering responsive dashboard cohesion and exact
bar-aware Ultimate visibility.

**Rationale**: The two outcomes map directly to S054 and S055, remain easy to
scan, and avoid repeating the detailed implementation record.

## Decision 5: Keep the pull request pre-release

**Decision**: Do not bump version references or create a tag in S056.

**Rationale**: The documented `cargo release 0.14.0 --execute` command atomically
rolls the version, changelog, badge, commit, and tag after the preparation pull
request merges. Performing part of that rollover in a pull request would create
two authorities.

## Alternatives Rejected

- Include #79 or other documentation-site work. Rejected because the operator
  explicitly requires the release first and the new work has a different review
  and verification boundary.
- Use v0.13.1. Rejected because a patch number understates the new user-visible
  Ultimate capability.
- Copy all Added and Decisions entries into Highlights. Rejected because release
  readers would again need to scroll through engineering history to reach assets.
- Run the release command on the feature branch. Rejected because release
  governance permits it only on `main` after explicit publication authorization.
