# Build Plan 017: Concise Release Notes

## Context

The tag workflow copied each complete changelog section into its GitHub release
body. Since v0.11.0, the Unreleased section accumulated several implementation
slices, dependency upgrades, governance decisions, and documentation work. That
detail belongs in the changelog, but embedding all of it makes the download
surface unnecessarily long.

## Slice 047: Bounded release highlights

Implement issue #51 as one prerequisite for v0.12.0:

1. add an explicit Highlights subsection to the changelog contract;
2. generate release bodies from only one through six Highlights bullets with a
   120-word ceiling;
3. append an immutable link to the complete changelog at the release tag;
4. fail before package builds when the excerpt is missing or invalid;
5. exercise the generator in pull-request CI and the release verify job;
6. document and preview the v0.12.0 candidate excerpt.

The full changelog validation remains intact. The slice does not alter release
triggers, permissions, package formats, application behavior, or version
numbers, and it creates no tag. After S047 merges, the established release
command publishes v0.12.0 using the new standard.

## Exit gate

- Issue #51 closes through the pull request.
- Generator contract tests pass locally and in hosted CI.
- Candidate v0.12.0 notes contain only compliant Highlights plus the full link.
- No release commit or tag exists before the prerequisite reaches `main`.
