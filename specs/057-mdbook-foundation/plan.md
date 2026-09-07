# Implementation Plan: mdBook Documentation Foundation

**Branch**: `codex/s057-mdbook-foundation`

**Date**: 2026-09-07

**Spec**: `specs/057-mdbook-foundation/spec.md`

## Summary

Establish a minimal branded mdBook skeleton under the future canonical
`docs/src` tree, enforce local content and generated-artifact contracts with
exactly pinned tools, and add a least-privilege GitHub Pages workflow that checks
pull requests but deploys only the exact validated main artifact.

## Technical Context

- **Languages**: Markdown, TOML, CSS, dependency-free JavaScript, YAML
- **Tools**: mdBook 0.5.4, mdbook-linkcheck2 0.13.0, Node standard library
- **Testing**: Node test fixtures, mdBook test/build, linkcheck renderer, output
  policy scan, hosted Actions
- **Target**: Static GitHub Pages site under `/eso-weave/`, offline-capable output
- **Constraints**: No Node package graph, no remote runtime assets, no generated
  HTML in Git, no legacy-content migration, no desktop embedding

## Constitution Check

- Full spec-kit sequence precedes implementation: PASS
- S057 traces to issue #79 and plan 027: PASS
- Safety-critical runtime surfaces remain untouched: PASS
- Test-first negative fixtures are required: PASS
- Pinned workflow change receives a dated changelog decision: PASS
- Pull requests remain read-only and never deploy: PASS
- UTF-8, LF, forbidden-dash, mojibake, and secret gates are explicit: PASS
- No scope crosses the outside-the-game boundary: PASS

## Project Structure

```text
.github/
  scripts/
    docs-policy.mjs
    docs-policy.test.mjs
  workflows/
    docs.yml
docs/
  book.toml
  architecture-decisions/0001-mdbook-documentation-site.md
  src/
    SUMMARY.md
    README.md
    404.md
    getting-started/README.md
    features/README.md
    concepts/README.md
    reference/README.md
    development/README.md
    assets/brand/eso-weave-mark.svg
    assets/brand/fonts/{Inter-Regular.ttf,Inter-SemiBold.ttf,OFL.txt}
  theme/{eso-weave.css,eso-weave.js}
docs/plans/plan-027.md
specs/057-mdbook-foundation/
```

## Implementation Phases

1. Complete specification, research, model, contract, checklists, tasks, and
   blocking analysis.
2. Write failing policy fixtures for navigation, case, local resources, and
   workflow permissions.
3. Add the minimal source tree, book configuration, brand layer, and ADR.
4. Implement the repository policy checker and bring negative fixtures green.
5. Add the immutable, least-privilege build and Pages deployment workflow.
6. Configure repository Pages and the protected main deployment boundary.
7. Validate local build, generated output, accessibility evidence, text hygiene,
   and full Cargo CI parity.
8. Complete independent reviews, publish the PR, and handle both authorized
   hosted review rounds.

## Architecture Decisions

- Establish `docs/src` immediately instead of creating a temporary website book.
- Use selective CSS and local assets rather than copying mdBook templates.
- Use mdbook-linkcheck2 plus a dependency-free repository checker.
- Build under the already ignored `target` tree.
- Give write and OIDC permissions only to the guarded deploy job.
- Use immutable action SHAs and exact Cargo tool versions.
- Leave corpus migration, full content, binary embedding, and release verification
  to #80, #81, #82, and #84.
- Omit a blog post because the feature is delivery plumbing and public content is
  owned by later documentation slices.

## Complexity Tracking

No constitution violation requires justification.
