# Quickstart: Concise Release Notes

## Author highlights

Add `### Highlights` immediately after `## [Unreleased]`. Write one through six top-level bullets totaling at most 120 words. Summarize user-visible outcomes; keep implementation detail and architectural decisions in the normal detailed subsections.

## Run the contract suite

```bash
bash scripts/release-notes.test.sh
```

## Preview the next release

Before changelog rollover, preview the Unreleased excerpt:

```bash
CHANGELOG_HEADING=Unreleased scripts/release-notes.sh 0.12.0 h8rt3rmin8r/eso-weave
```

After `cargo release` rolls the section to v0.12.0, the release workflow runs:

```bash
scripts/release-notes.sh 0.12.0 "$GITHUB_REPOSITORY" > RELEASE_NOTES.md
```

Confirm the preview contains only the Highlights bullets and one tag-specific full-changelog link.

## Full handoff gate

Run the Bash contract suite, YAML sanity checks, text hygiene checks, spec-kit analysis, and the complete repository CI-parity commands. The pull request must close #51 and must not create a release tag.
