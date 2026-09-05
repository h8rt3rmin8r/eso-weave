# Implementation Plan: Concise Release Notes

**Branch**: `codex/047-concise-release-notes` | **Date**: 2026-09-04 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/047-concise-release-notes/spec.md`

## Summary

Close issue #51 by adding an explicit, bounded Highlights subsection to the changelog contract and a dependency-free Bash generator that emits only those highlights plus a tag-specific full-changelog link. Test the generator in pull-request CI, validate it before release builds, use it to assemble GitHub release notes, document authoring, and prepare the Unreleased highlights for v0.12.0. The release itself remains a post-merge action.

## Technical Context

**Language/Version**: POSIX-oriented Bash and awk on Ubuntu GitHub-hosted runners; Markdown; GitHub Actions YAML

**Primary Dependencies**: Bash, awk, standard Unix text utilities, GitHub CLI already used by the release job

**Storage**: Version-controlled `CHANGELOG.md`; generated ephemeral `RELEASE_NOTES.md`

**Testing**: Dependency-free Bash contract suite plus existing repository CI

**Target Platform**: GitHub pull-request CI and tag-triggered GitHub Releases

**Project Type**: Desktop application repository with release automation

**Performance Goals**: Deterministic generation in under one second for the current changelog

**Constraints**: One through six top-level bullets; at most 120 words; UTF-8 without BOM; LF repository files; pinned scripts and workflows require a dated decision; no tag creation in this pull request

**Scale/Scope**: One generator, one test script, two workflow call sites, one maintainer document, one changelog subsection, and spec-kit artifacts

## Constitution Check

*GATE: Passed before Phase 0 research. Re-checked after Phase 1 design.*

- **Spec-first traceability**: PASS. Issue #51 maps to S047 specification, checklist, research, contract, tasks, and analysis before implementation.
- **Safety invariants**: PASS. No product, input, PixelBeacon, automation, or deletion behavior changes.
- **Test-first delivery**: PASS. Generator contract tests are written and observed failing before the implementation exists.
- **CI parity**: PASS. The new Bash suite runs in hosted pull-request CI; unchanged Rust remains covered by the full existing matrix.
- **Pinned artifact discipline**: PASS. Workflow, script, and release-guide changes are in scope and receive a dated changelog decision.
- **Release governance**: PASS. S047 prepares and validates notes but creates no release commit or tag.
- **Text hygiene**: PASS. All authored files use UTF-8 without BOM, LF, and no forbidden dash characters.

Post-design re-check: PASS. The design adds no dependency, credential, new permission, release trigger, or asset-format change.

## Project Structure

### Documentation (this feature)

```text
specs/047-concise-release-notes/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── checklists/
│   ├── release-note-safety.md
│   └── requirements.md
├── contracts/
│   └── release-notes.md
└── tasks.md
```

### Repository changes

```text
.github/workflows/
├── ci.yml
└── release.yml

scripts/
├── changelog-section.sh
├── release-notes.sh
└── release-notes.test.sh

docs/
├── plans/
│   ├── README.md
│   └── plan-017.md
└── releasing.md

CHANGELOG.md
```

**Structure Decision**: Keep full-section extraction unchanged for the release integrity gate. Add the concise presentation generator beside it, exercise the contract in normal CI, and make the release workflow consume the generated presentation only after full changelog validation.

## Complexity Tracking

No constitution violations require justification.
