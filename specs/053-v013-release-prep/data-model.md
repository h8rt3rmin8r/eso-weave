# Data Model: v0.13.0 Release Preparation

## Release Candidate

| Field | Value |
| --- | --- |
| Target version | 0.13.0 |
| Source section | `CHANGELOG.md` Unreleased |
| Detailed scope | S048 through S052 |
| Presentation | One Highlights subsection with four bullets |
| Publication state | Prepared, not published |

## Highlight

- A top-level Markdown bullet.
- Describes one user-visible outcome.
- Contributes to the shared 120-word maximum.
- Contains no release-verification claim.

## Release Boundary

- S053 output: reviewed release candidate on `main` after merge.
- Post-merge output: release commit and v0.13.0 tag created by cargo-release.
- Post-publication output: packaged assets and concise GitHub release body created by the release workflow.
- Field-verification output: receipts on #67 and #69.
