# Implementation Plan: Release Governance and Debian Metadata

**Branch**: `codex/s066-release-governance-debian-metadata` | **Date**: 2026-09-08 | **Spec**: `specs/066-release-governance-debian-metadata/spec.md`

## Summary

Clarify the release lifecycle in constitution 2.0.1 and aligned tracked guides,
then add explicit cargo-deb maintainer metadata and one generated-package
validator shared by pull-request fixtures and the tagged release gate. Close
implementation issues #104 and #105 while a new issue waits for next-release
installation evidence.

## Technical Context

**Language/Version**: Markdown, TOML, GitHub Actions YAML, POSIX shell, Rust 1.96 package inputs

**Primary Dependencies**: cargo-deb, `dpkg-deb`, GitHub Actions Ubuntu runners

**Storage**: Repository files and temporary fixture directories only

**Testing**: Shell fixture tests, generated `.deb` inspection, documentation policy, cargo fmt, clippy, and full locked tests

**Target Platform**: Linux x86_64 release packaging; cross-platform governance

**Project Type**: Desktop application release infrastructure

**Performance Goals**: Package validation completes before upload with negligible release-time overhead

**Constraints**: No runtime source or payload-layout change; no release tag; pinned edits require a dated decision; UTF-8 without BOM and LF; preserve user-owned untracked file

**Scale/Scope**: Two implementation issues, one future verification issue, five required Debian fields, five aligned tracked authority documents

## Constitution Check

- Full spec-kit sequence precedes implementation: PASS
- Constitution amendment has semantic version, Sync Impact Report, and aligned guidance: PASS
- No safety-critical runtime surface changes: PASS
- Test-first validator implementation is planned: PASS
- Full CI parity remains mandatory before commit: PASS
- Pinned workflow, scripts, and release guidance receive a dated decision: PASS
- Release/tag operations are out of scope: PASS
- Implementation and future release verification are separated: PASS
- User-owned untracked content is excluded: PASS

## Project Structure

```text
.specify/memory/constitution.md
CLAUDE.md
Cargo.toml
CHANGELOG.md
.github/workflows/
├── ci.yml
└── release.yml
scripts/
├── validate-debian-package.sh
└── validate-debian-package.test.sh
docs/project/
├── build-autopilot.md
├── governance.md
├── releasing.md
└── build-plans/
specs/066-release-governance-debian-metadata/
```

**Structure Decision**: Keep Debian validation in a reusable pinned script,
exercise it through a separate fixture suite, and call it from both relevant
workflows. Governance changes remain in their existing tracked authorities.

## Execution Sequence

1. Complete specification, clarification, checklists, research, model,
   contracts, quickstart, tasks, and pre-implementation analysis.
2. File and stage the separate next-release Debian verification issue.
3. Write validator fixture tests and confirm their red result.
4. Implement the validator and explicit maintainer metadata.
5. Wire pull-request CI and tagged release validation.
6. Amend constitution 2.0.1 and align tracked operating guidance.
7. Archive plan 035, establish plan 036, update indexes and migration ledger,
   and record the pinned-artifact decision in Unreleased.
8. Validate an actual generated package plus documentation and full CI parity.
9. Commit, push, open the official pull request, complete at most two authorized
   Codex review rounds, and wait for green CI.

## Complexity Tracking

No constitution violation or additional architecture is introduced.
