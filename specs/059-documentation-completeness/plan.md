# Implementation Plan: Documentation Completeness

**Branch**: `codex/s059-documentation-completeness`
**Date**: 2026-09-07
**Spec**: `specs/059-documentation-completeness/spec.md`

## Summary

Freeze a source-backed completeness inventory, make missing user and developer journeys canonical and searchable, enforce page quality and evidence through the existing documentation policy, and preserve the S058 authority boundaries.

## Technical Context

- **Languages**: Markdown, JSON, dependency-free JavaScript
- **Tools**: mdBook 0.5.4, mdbook-linkcheck2 0.13.0, typos-cli 1.50.1
- **Testing**: Node fixtures, mdBook test/build/linkcheck, generated-site policy, text hygiene, full Cargo parity
- **Target**: GitHub source and the `/eso-weave/` Pages site
- **Constraints**: Documentation-only, no runtime fixes, no generated HTML in Git, S058 preservation remains authoritative

## Constitution Check

- Full spec-kit sequence precedes implementation: PASS
- S059 traces to issue #81 and active plan 029: PASS
- Source and test evidence precede claims: PASS
- Safety contradictions become limitations and separate issues: PASS
- Test-first negative fixtures precede policy: PASS
- Workflow pin changes receive a changelog decision: PASS
- Text integrity and secret gates are explicit: PASS
- Runtime behavior remains unchanged: PASS

Post-design re-check: PASS. No exception is required.

## Project Structure

```text
docs/src/getting-started/{first-launch,troubleshooting}.md
docs/src/reference/{settings,status-reference}.md
docs/src/development/{state-machines,test-strategy,release-and-packaging,coverage-matrix}.md
docs/project/content-coverage.json
docs/project/build-plans/plan-029.md
specs/059-documentation-completeness/
```

## Implementation Phases

1. Complete spec-kit and blocking analysis.
2. File source contradictions and freeze the manifest.
3. Write failing completeness, terminology, accessibility, and workflow fixtures.
4. Implement manifest and page validation.
5. Add missing user journeys and expand incomplete pages.
6. Add missing developer logic and evidence pages.
7. Add spelling CI, lifecycle updates, and validation.
8. Complete reviews, publish, resolve two hosted rounds, and stop before merge.

## Architecture Decisions

- Use stable topic IDs and source/test symbols in JSON.
- Publish a concise evidence matrix, while canonical pages own detailed claims.
- Treat aliases as visible content, not hidden keyword stuffing.
- Prefer Markdown tables and text flows to a new diagram runtime.
- Mark product defects accurately and link issues without modifying code.
- Archive plan 028 and establish plan 029 as the sole current plan.
- Omit a blog post because no user-facing runtime capability is added.

## Complexity Tracking

The manifest is necessary because page counts and grep cannot prove complete, evidence-backed coverage.
