# Data Model: v0.14.0 Release Preparation

## Release Candidate

| Field | Value |
| --- | --- |
| Target version | 0.14.0 |
| Source section | `CHANGELOG.md` Unreleased |
| Detailed scope | S054 and S055, issues #71 through #75 |
| Presentation | One Highlights subsection with two bullets |
| Publication state | Prepared, not published |

## Highlight

- A top-level Markdown bullet.
- Describes one user-visible outcome.
- Contributes to the shared 120-word maximum.
- Contains no release-verification claim.

## Release Boundary

- S056 output: reviewed release candidate on `main` after merge.
- Post-merge output: release commit and v0.14.0 tag created by cargo-release.
- Publication output: packaged assets and concise GitHub release body created by
  the release workflow.
- Field-verification output: an evidence receipt on #77.
- Subsequent development output: documentation-site slices beginning with #79.
