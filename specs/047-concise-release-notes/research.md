# Research: Concise Release Notes

## Decision 1: Add an explicit Highlights subsection

**Decision**: Store one bounded `### Highlights` subsection inside each releasable version section and derive the GitHub release body only from it.

**Rationale**: Highlight selection is editorial judgment. An explicit subsection makes that judgment reviewable in version control and avoids brittle heuristics that guess which Added or Changed entries matter most.

**Alternatives considered**:

- Copy the full version section. Rejected because it caused the excessive release pages this slice corrects.
- Take the first N changelog bullets. Rejected because changelog ordering and user importance are not equivalent.
- Maintain a separate release-notes file. Rejected because it creates a second source that can drift from the changelog.

## Decision 2: Enforce a six-item, 120-word budget

**Decision**: Require one through six top-level highlight bullets totaling no more than 120 words.

**Rationale**: The user's outcome needs an objective review and CI gate. The budget keeps the excerpt compact while leaving enough room to summarize the runtime, automation, pixel geometry, dashboard, and delivery improvements accumulated since v0.11.0.

**Alternatives considered**:

- Enforce browser pixels or viewport height. Rejected because GitHub layout, browser scale, width, and accessibility settings are outside repository control.
- Use only a character limit. Rejected because it is less legible to authors and handles Markdown links poorly.
- Rely on review alone. Rejected because the old standard already drifted into full-changelog output.

## Decision 3: Keep full changelog verification separate

**Decision**: Continue using `scripts/changelog-section.sh` to prove the complete version section exists and is non-empty. Run the new concise generator as an additional verification gate and later as the release-body producer.

**Rationale**: Release integrity and release-page presentation are different contracts. Replacing full-section validation with highlight-only validation could permit an empty detailed changelog.

## Decision 4: Generate an immutable tag-specific link

**Decision**: Append `https://github.com/<owner>/<repo>/blob/v<version>/CHANGELOG.md` after the excerpt.

**Rationale**: Linking to the release tag makes the full history match the artifact being downloaded even after `main` changes.

**Alternatives considered**:

- Link to `main`. Rejected because its content changes after publication.
- Generate a heading anchor with the release date. Rejected because the tag already supplies immutability and the extra slug construction adds no necessary value.

## Decision 5: Use dependency-free Bash and awk

**Decision**: Implement the generator and its contract suite using tools already required by the Linux release runner.

**Rationale**: The existing changelog extractor and release assembly are Bash based. Reusing that environment adds no package, runtime setup, or supply-chain surface.

## Decision 6: Test during pull requests and release verification

**Decision**: Run the generator suite in the Linux CI matrix and again in the release verify job. Validate the tagged version's actual notes before building packages.

**Rationale**: Pull-request execution catches regressions before merge. Tag-time execution and generation protect against a malformed release rollover or incorrect version input.

**Sources**:

- GitHub CLI manual, [gh release create](https://cli.github.com/manual/gh_release_create), documents `--notes-file` as the release body input used by the existing workflow.
- GitHub Docs, [Managing releases in a repository](https://docs.github.com/en/repositories/releasing-projects-on-github/managing-releases-in-a-repository), describes release notes and attached binary assets as parts of a GitHub release.
