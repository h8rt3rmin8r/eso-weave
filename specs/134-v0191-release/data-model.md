# Candidate Data Model

## Candidate identity

Version 0.19.1; common rollover date; source preparation commit and resulting
identity commit. Cargo.toml is authoritative. Cargo.lock's root package entry,
README badge, bundled documentation version/date, changelog heading/date and
collector fixture tool_version must agree. Addon manifest versions are separate.

## Release notes

One through six Highlights bullets, at most 120 words, followed by a tag-specific
complete-changelog link in generated release notes. Preserve all prior release
sections and complete patch entries. A fresh Unreleased heading remains after rollover.

## Delivery lifecycle

Plan060 -> S134 -> candidate #315 -> official PR -> owner merge. Candidate #315
can close on merge; coordinator #314 and milestone v0.19.1 stay open until later
separately authorized publication and asset audit. Every issue appears exactly
once in the Project. Candidate Slice is S134; the coordinator has no invented
feature-slice assignment. Current Stage follows actual evidence.

## Evidence

A validation result names the command, observed outcome, revision and limits.
Local tests/builds and hosted checks do not establish installed/game behavior.
No private runtime payload is collected for this release preparation.
