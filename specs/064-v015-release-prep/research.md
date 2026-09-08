# Research: v0.15.0 Release Preparation

## Decision 1: Use a minor version

**Decision**: Target v0.15.0.

**Rationale**: S063 adds Help > Documentation and a complete bundled searchable guide. This is a backward-compatible user capability, so a minor release communicates the change accurately.

## Decision 2: Represent all post-v0.14.0 outcomes

**Decision**: Use four Highlights for documentation, safety, Linux input, and live settings.

**Rationale**: S057 through S059 and S063 form one documentation-system outcome. S060, S061, and S062 each change a separate user-visible reliability contract. Combining any of those three into a generic maintenance bullet would hide material safety or platform behavior.

Dependency PRs #89 and #90 belong in a detailed Dependencies subsection so every merge since v0.14.0 is recorded, but they do not warrant a user-facing Highlight.

## Decision 3: Reuse the S047 release-note contract

**Decision**: Add compliant Highlights and make no script, workflow, release configuration, packaging, or guide changes.

**Rationale**: The existing generator already limits Highlights to one through six bullets and 120 words, excludes detailed subsections, and adds the immutable tagged changelog link. No blocker has been demonstrated.

## Decision 4: Keep candidate preparation separate from publication

**Decision**: Retain v0.14.0 in all version sources and do not create a tag or release.

**Rationale**: The documented post-merge `cargo release 0.15.0 --execute` action atomically rolls Cargo, lockfile, changelog, README badge, commit, and tag. Splitting that authority across the feature branch would increase drift risk.

## Decision 5: Preserve field verification

**Decision**: Leave #77 and #84 open.

**Rationale**: #77 owns live-game Ultimate evidence, and #84 cannot begin until a tagged package containing S063 exists. A candidate PR proves neither.

## Alternatives Rejected

- v0.14.1: rejected because it understates a new Help surface and bundled documentation capability.
- Documentation-only Highlights: rejected because S060 through S062 also shipped after v0.14.0.
- Modify release automation: rejected because current machinery passed v0.14.0 and already supports bundled documentation prerequisites.
- Publish from the feature branch: rejected because release governance reserves publication for explicitly authorized work on `main`.
- Include dependency upgrade details in Highlights: rejected because those merges do not change a user workflow; retain them in the detailed Dependencies record instead.
