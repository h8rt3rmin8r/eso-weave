# Data Model: v0.15.0 Release Preparation

## Release Candidate

| Field | Value |
| --- | --- |
| Target version | 0.15.0 |
| Source section | `CHANGELOG.md` Unreleased |
| Detailed scope | S057 through S063 and dependency PRs #89 and #90 |
| Presentation | One Highlights subsection with four bullets |
| Publication state | Prepared, not published |

## Highlight Mapping

| Highlight | Source slices | Outcome |
| --- | --- | --- |
| Documentation | S057, S058, S059, S063 | Searchable public and bundled offline guide |
| Safety | S060 | Revocable automation and protected PixelBeacon ownership |
| Linux input | S061 | Complete forwarding, explicit errors, fail-closed menu evidence |
| Live settings | S062 | Immediate safe Fishing and Pixel Bus scalar updates |

## Detailed Dependency Record

| Pull request | Direct dependency change | Highlight |
| --- | --- | --- |
| #89 | `x11rb` 0.13.2 to 0.14.0 | None |
| #90 | `thiserror` 1.0.69 to 2.0.18 | None |

## Publication Boundary

- S064 output: reviewed v0.15.0 candidate on `main` after merge.
- Release output: version rollover, release commit, and v0.15.0 tag created by cargo-release after explicit authorization.
- Workflow output: Windows and Linux packages, checksums, and concise GitHub Release body.
- Verification output: installed-package evidence on #77 and #84.
