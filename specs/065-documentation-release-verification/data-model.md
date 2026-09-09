# Data Model: Documentation Release Verification

## Release Identity

| Field | Value |
| --- | --- |
| Version | 0.15.0 |
| Tag | `v0.15.0` |
| Commit | `2ec90787e5b817897736d85a6c75c25baf365cba` |
| Workflow run | `34302551069` |
| Published | 2026-09-09 UTC |

## Artifact Record

| Field | Meaning |
| --- | --- |
| Name | Exact GitHub Release asset name |
| Size | Published and downloaded byte count |
| Expected digest | Value from published `SHA256SUMS` |
| Observed digest | Independently calculated SHA-256 |
| Package role | Windows installed, Linux installed, Linux portable, or checksum authority |
| Result | Pass or fail with evidence |

## Platform Observation

| Field | Meaning |
| --- | --- |
| Platform | Exact OS, version, desktop, and architecture |
| Package | Artifact and installation or extraction path |
| Browser | Browser and version used by the shipped action |
| Interaction | Pointer, keyboard, accessibility tree, HTTP, process, or socket inspection |
| Network condition | Scoped isolation or request trace used |
| Expected | One #84 criterion or service contract |
| Observed | Concrete result without inference |
| Result | Pass, fail, or blocked |

## Verification State

- `Pending`: evidence not yet collected.
- `Pass`: direct or bounded supporting evidence satisfies the criterion.
- `Fail`: observed behavior contradicts the criterion and requires a defect issue.
- `Blocked`: the criterion cannot be observed in the available environment and prevents closure.

Only an all-Pass completion matrix permits #84 and #83 to close.
