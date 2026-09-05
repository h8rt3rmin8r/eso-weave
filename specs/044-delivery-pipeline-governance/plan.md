# Implementation Plan: Delivery Pipeline Governance

**Branch**: `codex/044-delivery-pipeline-governance` | **Date**: 2026-09-03 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/044-delivery-pipeline-governance/spec.md`

## Summary

Deliver issues #45, #46, and #47 as one governance slice. Add a dependency-free pull request closing-reference policy and CI check, document atomic issue and verification lifecycles, audit all issue metadata against repository evidence, and create a linked GitHub Project with a minimal Stage and Slice schema. No Rust product behavior or release artifacts change.

## Technical Context

**Language/Version**: ECMAScript modules on the Node.js version provided by GitHub-hosted runners; Markdown; GitHub Actions YAML; GitHub CLI and GraphQL

**Primary Dependencies**: Node.js standard library only, GitHub Actions, GitHub Issues, GitHub Projects v2

**Storage**: Version-controlled repository files plus GitHub-hosted issue and Project metadata

**Testing**: Node.js built-in test runner, spec-kit checks, repository documentation and encoding checks, existing Rust quality gates

**Target Platform**: GitHub pull requests targeting `main` and the public `h8rt3rmin8r/eso-weave` repository

**Project Type**: Desktop application repository with governance automation

**Performance Goals**: The policy check completes in one lightweight CI job without installing dependencies or calling external APIs

**Constraints**: Read-only workflow permissions; untrusted event metadata never enters script source; no `pull_request_target`; no product-code changes; no release; UTF-8 without BOM and LF line endings

**Scale/Scope**: All repository issues through #47, three implementation issues, one public Project, one policy workflow, and repository governance documentation

## Constitution Check

*GATE: Passed before Phase 0 research. Re-checked after Phase 1 design.*

- **Spec-first traceability**: PASS. S044 has a specification, research record, contracts, task plan, and analysis before implementation.
- **Safety invariants**: PASS. No runtime input, timing, foreground-process, or automation behavior changes.
- **Test-first delivery**: PASS. Policy behavior is defined in tests before the checker is implemented.
- **Documented public contracts**: PASS. Closing syntax, exemptions, lifecycle states, and project fields are documented.
- **Workflow artifact discipline**: PASS. The new workflow is treated as pinned governance behavior and receives a dated changelog entry.
- **Validation proportionality**: PASS. Targeted policy tests and external-state audits are mandatory; the complete repository gate runs before handoff.
- **Repository hygiene**: PASS. The implementation is narrowly contained in `.github`, governance docs, changelog, and spec artifacts.

Post-design re-check: PASS. The design introduces no new dependency, credential, write-capable workflow, or product behavior.

## Project Structure

### Documentation (this feature)

```text
specs/044-delivery-pipeline-governance/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── checklists/
│   ├── governance-safety.md
│   └── requirements.md
├── contracts/
│   ├── issue-lifecycle.md
│   ├── pr-closing-policy.md
│   └── project-schema.md
└── tasks.md
```

### Repository changes

```text
.github/
├── ISSUE_TEMPLATE/
├── scripts/
│   ├── issue-link-policy.mjs
│   └── issue-link-policy.test.mjs
├── workflows/
│   └── issue-link.yml
└── pull_request_template.md

docs/
├── plans/
│   ├── README.md
│   └── plan-014.md
└── project-governance.md

CHANGELOG.md
CONTRIBUTING.md
```

GitHub-hosted state includes the `skip: issue-link` label, issue metadata corrections, and the linked `ESO Weave Delivery` Project.

**Structure Decision**: Keep governance code under `.github`, durable maintainer guidance under `docs`, and all slice evidence under the existing spec-kit directory. Avoid introducing an application-level scripts package for a CI-only policy.

## Complexity Tracking

No constitution violations require justification.
