# Implementation Plan: Documentation Landing Identity

**Branch**: `codex/s080-docs-landing-identity` | **Date**: 2026-09-10 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/080-docs-landing-identity/spec.md`

## Summary

Replace the landing page's square mark with the approved full-color banner,
retain one accessible H1 without repeating the visible product name, and add a
semantic project metadata snapshot. Extend the documentation policy so the page
cannot drift from package name, package version, package repository, or the
matching changelog release date.

## Technical Context

**Language/Version**: Markdown, CSS, ECMAScript modules on the repository Node.js baseline
**Primary Dependencies**: mdBook, mdbook-linkcheck2, Node.js built-ins
**Storage**: Static repository files only
**Testing**: `node --test`, mdBook test/build/linkcheck, documentation policy, typos
**Target Platform**: GitHub Pages and bundled loopback-served offline documentation
**Project Type**: Documentation site inside a Rust desktop application repository
**Performance Goals**: One 73,763-byte local banner, no remote requests, negligible layout shift
**Constraints**: UTF-8 without BOM, LF, no forbidden dashes, 320 CSS pixel support, no Rust changes
**Scale/Scope**: One landing page, one copied image, one theme stylesheet, one policy module and focused tests

## Constitution Check

*GATE: Passed before Phase 0 research and re-checked after Phase 1 design.*

- **Spec-driven sequence**: PASS. Issue #121, Plan 039, the spec, clarification
  record, checklists, plan, design artifacts, tasks, and analysis form the authority chain.
- **Safety-critical surfaces**: PASS. No input, addon, capture, automation, or
  application runtime behavior changes.
- **Test first**: PASS. Focused landing-contract failures precede validator and
  documentation implementation.
- **CI parity**: PASS. Documentation and text gates are mandatory. Rust source is
  unchanged, so cargo commands are optional confirmation rather than a commit gate.
- **Bounded scope**: PASS. All content is static, local, shared by public and
  bundled documentation, and free of networking or telemetry.
- **Text hygiene**: PASS. New text uses UTF-8 without BOM, LF, and standard hyphens.

No complexity exception is required.

## Project Structure

### Documentation for this feature

```text
specs/080-docs-landing-identity/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── contracts/
│   └── landing-identity.md
└── checklists/
    ├── requirements.md
    └── identity-and-metadata.md
```

### Repository surfaces

```text
Cargo.toml                                  # package metadata authority
CHANGELOG.md                                # release-date authority and S080 record
assets/eso-weave-banner.png                 # approved banner source
docs/src/README.md                          # landing identity and metadata snapshot
docs/src/assets/brand/eso-weave-banner.png  # local mdBook asset copy
docs/theme/eso-weave.css                    # responsive and accessible layout
.github/scripts/docs-policy.mjs             # drift and structure validation
.github/scripts/docs-policy.test.mjs        # focused contract regressions
docs/project/build-plans/plan-039.md         # chronological slice status
docs/project/migration-ledger.json           # active spec evidence
```

**Structure Decision**: Keep all presentation in the existing mdBook source and
theme. Validate static values at repository policy time instead of adding a
preprocessor, JavaScript fetch, or second metadata file.

## Phase 0: Research

1. Confirm the approved banner dimensions, bytes, and current uses.
2. Identify existing package and changelog authorities for every requested field.
3. Evaluate H1 and image alternatives for visible and assistive naming.
4. Confirm mdBook copies local source assets into both delivery forms.
5. Identify existing policy seams for source and generated-site assertions.

Output: [research.md](research.md)

## Phase 1: Design

1. Define the landing identity and documentation snapshot entities.
2. Specify exact source structure, authority mappings, and validator failures.
3. Define responsive CSS behavior and assistive naming.
4. Define local and CI verification commands.

Outputs: [data-model.md](data-model.md), [landing identity contract](contracts/landing-identity.md), and [quickstart.md](quickstart.md)

## Phase 2: Tasks and Analysis

Generate story-ordered tasks with tests before implementation. Analyze traceability,
authority consistency, accessibility, offline equivalence, and scope. Resolve every
finding before implementation.

## Implementation Strategy

1. Add failing policy tests for the complete landing contract and each drift case.
2. Implement a pure landing-contract validator using supplied source strings.
3. Copy the approved banner byte-for-byte into the mdBook source tree.
4. Rebuild the landing identity, metadata block, and source disclosure.
5. Add scoped responsive CSS and generated-output assertions.
6. Update Plan 039, migration evidence, and the changelog.
7. Run the quickstart and full documentation gates before publication.

## Complexity Tracking

No constitutional violation or additional subsystem is introduced.
