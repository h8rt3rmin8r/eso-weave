# Data Model: Release Governance and Debian Metadata

## Release State

| State | Entry | Exit |
| --- | --- | --- |
| Candidate | Reviewed source is ready | All pre-publication gates pass |
| Published | Authorized tag workflow creates immutable assets | Assets exist for download |
| Release verification | Artifact-dependent issue names a published asset | Evidence passes, or a defect creates implementation work |

Publication is the entry gate for artifact-dependent verification, not its
success condition.

## Debian Control Contract

| Field | Requirement |
| --- | --- |
| Package | Non-empty; remains `eso-weave` |
| Version | Non-empty; supplied by cargo-deb from the crate version |
| Architecture | Non-empty; remains the release runner target architecture |
| Maintainer | Exact explicit public project contact |
| Description | Non-empty; remains derived from the existing package description |

## Verification Issue

| Field | Meaning |
| --- | --- |
| Entry gate | First downloadable release containing S066 |
| Artifact | That release's x86_64 `.deb` |
| Evidence | Checksum, control query, install, package query, and warning output |
| Success | Required fields match and no control warning occurs |
| Failure | Linked implementation issue plus rerun against a fixed release |
| Stage before entry | Release verification |
