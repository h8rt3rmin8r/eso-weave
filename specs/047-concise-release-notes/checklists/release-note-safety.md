# Release Note Safety Checklist: Concise Release Notes

**Purpose**: Keep release pages compact without weakening release integrity
**Created**: 2026-09-04
**Feature**: [spec.md](../spec.md)

## Source integrity

- [x] The full changelog remains authoritative and untruncated
- [x] Highlights live inside the selected version section
- [x] Highlight selection is explicit rather than heuristic
- [x] The full-changelog link resolves against the release tag

## Failure behavior

- [x] Missing version, heading, and content fail closed
- [x] Item and word budgets fail closed
- [x] Invalid version and repository inputs fail before URL construction
- [x] No partial release-note output is emitted on failure

## Pipeline boundaries

- [x] Full-section verification remains in the release gate
- [x] Pull-request CI exercises the generator contract
- [x] Release triggers, permissions, and asset formats remain unchanged
- [x] No release tag is created in S047

## Notes

Pinned workflow and script changes require the dated decision recorded in the S047 changelog entry.
