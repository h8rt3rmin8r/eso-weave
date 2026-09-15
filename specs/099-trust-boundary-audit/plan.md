# Implementation Plan: Repository Trust-Boundary Audit

**Branch**: `codex/s099-trust-boundary-audit` | **Date**: 2026-09-15 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/099-trust-boundary-audit/spec.md`

## Summary

Establish protected `main` as the repository-policy trust anchor, define how authority-bearing agents handle untrusted collaboration and branch content, remove unrelated high-risk local skills, and enforce the boundary through deterministic policy tests. Pin every GitHub Action to an immutable commit, disable checkout credential persistence, verify downloaded release tooling by digest, pin release-only Cargo tools, and configure GitHub branch and Actions settings so the written integration contract is technically enforced.

## Technical Context

**Language/Version**: JavaScript on the GitHub-hosted Node.js 22 runtime; YAML, Markdown, and JSON policy artifacts

**Primary Dependencies**: Node.js standard library, GitHub Actions, GitHub repository REST API

**Storage**: Checked-in workflow and guidance files plus hosted GitHub repository settings

**Testing**: Node.js policy unit tests, repository policy scan, existing documentation gates, full Rust CI parity, hosted Actions and live settings verification

**Target Platform**: GitHub-hosted Ubuntu and Windows runners; local Windows and Linux contributor environments

**Project Type**: Single-crate desktop repository with repository automation

**Performance Goals**: Complete the static trust scan in under one second locally with no network access

**Constraints**: Public evidence remains high-level; no secret values; no untrusted checkout as authority; exact Action SHAs; read-only default tokens; solo-maintainer merge remains possible

**Scale/Scope**: Seven workflows, one release download, repository-local guidance and skills, one policy checker and test suite, hosted branch and Actions settings, canonical contributor documentation

## Constitution Check

*GATE: Passed before research and re-checked after design.*

- **Principle I, Spec-Driven Development**: PASS. Issue #181, Plan 043, this complete S099 package, and canonical guidance form the authority chain. The initial audit was read-only, and implementation begins only after analysis.
- **Principle II, Safety-Critical Surfaces**: PASS. No application or addon surface changes. Full locked tests remain mandatory to prove the governance work did not weaken safety behavior.
- **Principle III, Test-First**: PASS. Policy fixtures for mutable actions, checkout credentials, privileged triggers, unexpected writes, secrets, and local skill metadata precede the policy implementation.
- **Principle IV, CI Parity**: PASS. Although S099 changes no Rust source, full fmt, strict Clippy, locked tests, release build, and documentation gates run before commit.
- **Principle V, Bounded Scope**: PASS. The slice changes repository governance and automation only. It does not alter desktop, addon, protocol, data, automation, or release contents.
- **Pinned artifacts and text hygiene**: PASS. Workflow and script changes are required by issue #181 and receive a dated changelog decision. All text remains UTF-8 without BOM, LF, and free of forbidden dash characters.

## Design

### Authority boundary

Direct operator instructions authorize scope and mutations. Policy from protected `main` constrains that authority but cannot expand it. Issues, PR bodies, comments, reviews, reactions, logs, artifacts, external content, generated text, and proposed branch files remain untrusted data regardless of author. An authority-bearing agent reviews proposed guidance against the protected base without adopting the proposed copy as instructions.

The root agent guidance and autopilot procedure will state this boundary before operational instructions. Project-local skills dedicated to unrelated offensive bypass or evasion will be removed, and the policy scan will reject comparable high-risk trigger descriptions from the local skill surface.

### Workflow integrity policy

Add a zero-dependency Node.js checker with pure validation functions and a repository entry point. It inspects every workflow and local skill metadata. The workflow rules require exact Action SHAs, readable version comments, explicit `persist-credentials: false`, read-only top-level contents permission, a small write-permission allowlist, safe triggers, and no unexpected secret references. A dedicated base-owned Trust boundary workflow is the sole restricted `pull_request_target` exception; it scans the proposed immutable checkout with protected-base policy and never executes proposed files.

The checker also verifies the release download contract and exact packaging-tool versions. Unit tests construct representative invalid inputs for every rule, including instruction-shaped text that must remain inert test data.

### Immutable dependencies

Replace mutable Action major tags with the current resolved commits and version comments. GitHub-owned actions remain allowed. `Swatinem/rust-cache` remains the single third-party Action publisher because it is already used by CI and has a bounded cache-only role.

The AppImage builder remains sourced from the upstream `continuous` release because no versioned release channel exists. Its accepted SHA-256 digest is fixed in the workflow and checked before the executable bit is set or the tool runs. `cargo-wix` and `cargo-deb` receive exact top-level versions in addition to `--locked` transitive resolution.

### Hosted enforcement

Configure classic branch protection for `main` with strict required checks, pull-request-only updates, administrator enforcement, linear history, resolved review conversations, and deletion and force-push disabled. Bootstrap checks are limited to jobs emitted by every pull request: Linux CI, Windows CI, dependency review, CodeQL, and both issue-link jobs. Once S099 is present on protected `main`, the base-owned `Enforce protected trust policy` job becomes the seventh required check so proposed workflow edits cannot skip enforcement.

Keep default workflow tokens read-only and unable to approve reviews. Restrict allowed actions to GitHub-owned actions plus `Swatinem/rust-cache`, and enable immutable SHA enforcement. Because the repository has one collaborator, no independent approval count is required; the documented operator final-review ritual remains mandatory without making self-authored work impossible to merge.

### Disclosure boundary

The specification, tests, documentation, changelog, issue updates, and PR describe control classes and results only. Detailed evidence and reproduction reasoning remain in the operator-approved session. No secrets exist in the repository or Dependabot inventories, and no values are copied into public artifacts.

## Project Structure

### Specification package

```text
specs/099-trust-boundary-audit/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── issue-181.md
├── contracts/
│   ├── authority-boundary.md
│   └── workflow-integrity.md
├── checklists/
│   ├── requirements.md
│   └── trust-and-authorization.md
├── analysis.md
└── tasks.md
```

### Repository integration

```text
.github/CODEOWNERS
.github/scripts/trust-policy.mjs
.github/scripts/trust-policy.test.mjs
.github/workflows/catalog-candidate.yml
.github/workflows/ci.yml
.github/workflows/codeql.yml
.github/workflows/docs.yml
.github/workflows/issue-link.yml
.github/workflows/release.yml
.claude/skills/sandbox-escape-techniques/        (removed)
.claude/skills/windows-av-evasion/               (removed)
CLAUDE.md
CONTRIBUTING.md
docs/src/SUMMARY.md
docs/src/development/repository-trust.md
docs/project/build-autopilot.md
docs/project/build-plans/plan-043.md
docs/project/build-plans/README.md
CHANGELOG.md
.specify/feature.json
```

**Structure Decision**: Keep trust-policy checks separate from the documentation-policy module. Repository authorization is a distinct concern, and coupling it to the large documentation validator would obscure ownership and make enforcement conditional on documentation paths.

## Verification Strategy

1. Add failing pure-function tests for mutable Action references, missing version comments, persisted checkout credentials, privileged event triggers, unexpected write permissions, secret references, release-tool drift, and high-risk local skill metadata.
2. Implement the checker and make all focused tests pass.
3. Pin all workflow dependencies, disable checkout credential persistence, and harden release tools until the repository scan passes.
4. Update agent, contributor, canonical, autopilot, build-plan, and changelog guidance, then run documentation policy, rendering, spelling, link, encoding, and disclosure scans.
5. Run full fmt, strict Clippy, locked tests, and release build.
6. Configure GitHub branch protection and Actions policies, then verify their effective live values through read-only API calls.
7. Publish the PR, wait for all required checks and external reviews, address every thread, and trigger only the one authorized second automated review round.

## Complexity Tracking

No constitution violations or complexity exceptions are required.
