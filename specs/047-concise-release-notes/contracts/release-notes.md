# Release Notes Contract

## Changelog input

Within a releasable section:

```markdown
## [0.12.0] - 2026-09-04

### Highlights

- First user-facing result.
- Second user-facing result.

### Added

...complete detail...
```

Highlights contain one through six top-level `- ` bullets and at most 120 words. Wrapped or nested lines are permitted. The next level-three or level-two heading ends the excerpt.

## Command

```bash
scripts/release-notes.sh <version> [owner/repository] [changelog-file]
```

- `version` is supplied without the leading `v`.
- `owner/repository` defaults to `GITHUB_REPOSITORY`, then to `h8rt3rmin8r/eso-weave` for local use.
- `changelog-file` defaults to `CHANGELOG.md`.
- `CHANGELOG_HEADING=Unreleased` may select the candidate subsection while the version argument continues to define the future tag link.
- Success writes only the generated Markdown to stdout.
- Failure writes one actionable diagnostic to stderr and exits nonzero.

## Output

```markdown
- First user-facing result.
- Second user-facing result.

[Read the full changelog for v0.12.0](https://github.com/owner/repository/blob/v0.12.0/CHANGELOG.md)
```

No Added, Changed, Fixed, Removed, Security, or Decisions subsection is copied unless its substance was deliberately summarized in Highlights.

## Workflow integration

The release verify job must:

1. run the generator contract suite;
2. validate the complete version section with `changelog-section.sh`;
3. validate the concise notes with `release-notes.sh` before package builds.

The release job must call the same generator to create `RELEASE_NOTES.md` for `gh release create --notes-file`.
