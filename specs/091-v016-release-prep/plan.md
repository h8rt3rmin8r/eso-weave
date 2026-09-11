# Implementation Plan: v0.16.0 Release Preparation

**Branch**: `codex/s091-v016-release-prep` | **Date**: 2026-09-11 | **Spec**: `specs/091-v016-release-prep/spec.md`

## Summary

Prepare a complete v0.16.0 candidate, repair the documentation metadata rollover
introduced after v0.15.1, pass hosted review, merge, and execute the authorized
public release from `main`.

## Technical Context

**Primary dependencies**: Markdown, Node documentation policy, cargo-release,
GitHub Actions, and GitHub Releases

**Testing**: Release-note fixtures and preview, documentation policy, complete
Rust checks, cargo-release dry run, hosted CI, review, and release asset audit

**Constraints**: No premature version bump or tag in the preparation pull
request; no closure of independent verification issues; preserve the unrelated
untracked draft

## Constitution Check

- Issue, specification, checklists, plan, tasks, and analysis precede delivery:
  PASS
- The identified release-path defect is corrected proportionally: PASS
- Pinned artifact changes have a dated decision: PASS
- Existing release workflow and packaging remain unchanged: PASS
- UTF-8 without BOM, LF, punctuation, and mojibake gates are explicit: PASS
- Verification work never blocks repository progress or publication: PASS

## Implementation Phases

1. Audit every post-v0.15.1 merge and select v0.16.0.
2. Add three bounded Highlights and the detailed S091 record.
3. Add field-scoped documentation version and date replacements to release.toml.
4. Enforce the replacement contract in documentation policy and fixtures.
5. Correct release-governance ownership and artifact prose.
6. Archive Plan 040 and establish active Plan 041 across all lifecycle records.
7. Validate locally, publish the preparation pull request, and resolve hosted
   checks and review findings.
8. Merge, dry-run and execute cargo-release on clean `main`, monitor the tag
   workflow, and inspect the public artifacts.

## Complexity Tracking

No constitution violations require justification.
