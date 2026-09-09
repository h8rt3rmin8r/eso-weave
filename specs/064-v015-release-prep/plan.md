# Implementation Plan: v0.15.0 Release Preparation

**Branch**: `codex/s064-v015-release-prep` | **Date**: 2026-09-08 | **Spec**: `specs/064-v015-release-prep/spec.md`

## Summary

Prepare a reviewable v0.15.0 candidate by adding four bounded user-facing Highlights to the complete Unreleased changelog, recording S064 without claiming publication, and advancing the chronological build-plan lifecycle. Reuse the established release-note machinery and leave version rollover, tagging, assets, and installed-package verification to separately authorized post-merge work.

## Technical Context

**Language/Version**: Markdown and existing dependency-free Bash/awk release-note tooling

**Primary Dependencies**: `CHANGELOG.md`, `scripts/release-notes.sh`, cargo-release configuration, GitHub release history

**Storage**: Repository Markdown only

**Testing**: Release-note contract suite, exact candidate preview, documentation policy, text hygiene, Git scope checks, and CI parity

**Target Platform**: Repository release process for Windows x64 and Linux x86_64 packages

**Project Type**: Documentation-only release preparation for a Rust desktop application

**Performance Goals**: Four bullets, at most 120 words, immediately scannable before download links

**Constraints**: No version bump, tag, release, asset, pinned machinery edit, or verification closure

**Scale/Scope**: S057 through S063 plus dependency-only merges since v0.14.0

## Constitution Check

- Specification and actionable issue precede implementation: PASS
- Full specify, clarify, checklist, plan, tasks, analyze sequence is required: PASS
- No safety-critical product behavior changes: PASS
- Existing release-note contract is reused unchanged: PASS
- Pinned scripts, workflows, configuration, packaging, and release guide remain unchanged: PASS
- Version rollover, tag creation, publication, and verification remain separately authorized: PASS
- Current plan chronology advances from completed plan 033 to plan 034: PASS
- UTF-8 without BOM, LF, punctuation, and mojibake gates are explicit: PASS

## Project Structure

```text
CHANGELOG.md
docs/
├── archive/build-plans/
│   ├── README.md
│   └── plan-033.md
└── project/
    ├── build-plans/
    │   ├── README.md
    │   └── plan-034.md
    ├── migration-ledger.json
    └── migration-ledger.md
specs/064-v015-release-prep/
├── analysis.md
├── checklists/
├── contracts/
├── data-model.md
├── plan.md
├── quickstart.md
├── research.md
├── spec.md
└── tasks.md
```

**Structure Decision**: Limit implementation to the changelog and chronological planning records. Keep all release executables, configuration, workflows, scripts, packaging, and versioned files unchanged.

## Implementation Phases

1. Complete specification, clarification decisions, checklists, research, model, contract, quickstart, tasks, and pre-implementation analysis.
2. Add four outcome-oriented Highlights covering documentation, safety, Linux input, and live settings.
3. Record S064 in Changed without claiming publication.
4. Archive plan 033 with PR #101 evidence, establish active plan 034, and update the lifecycle ledger and indexes.
5. Run the release-note contract, exact preview, scope, lifecycle, documentation, and text-hygiene gates.
6. Complete post-implementation analysis, commit, publish PR, and resolve the initial plus exactly one authorized second Codex review round.

## Decisions

- Target v0.15.0 because bundled offline documentation is a new backward-compatible user capability.
- Use four Highlights because they are the smallest complete mapping of distinct post-v0.14.0 user outcomes.
- Keep dependency-only upgrades out of Highlights because they add no direct user workflow.
- Reuse S047 release tooling unchanged because it already enforces the item budget, word budget, and immutable tag link.
- Leave #77 and #84 open because candidate preparation is not installed-package evidence.
- Do not add a blog post or marketing page because Highlights are the governed release presentation input.

## Complexity Tracking

No constitution violations require justification.
