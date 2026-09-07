# Implementation Plan: Documentation Corpus Reorganization

**Branch**: `codex/s058-documentation-corpus`

**Date**: 2026-09-07

**Spec**: `specs/058-documentation-corpus/spec.md`

## Summary

Freeze and machine-check the pre-S058 corpus, split the existing README and
technical specification into canonical audience pages, move current maintainer
records and completed plans into explicit lifecycles, retire competing sources,
and synchronize governance with the new paths.

## Technical Context

- **Languages**: Markdown, JSON, dependency-free JavaScript, small Rust comment update
- **Tools**: Existing mdBook 0.5.4 and mdbook-linkcheck2 0.13.0
- **Testing**: Node policy fixtures, mdBook test/build/linkcheck, generated-site
  validation, full Cargo parity
- **Target**: GitHub-rendered repository records plus the `/eso-weave/` Pages site
- **Constraints**: Preserve 50 baseline artifacts and 20 spec units, no generated
  HTML in Git, no runtime behavior changes, no #81/#82/#84 work
- **Scale**: 27 legacy plans, one active plan, one structured ledger, roughly 20
  canonical reader pages

## Constitution Check

- Full spec-kit sequence precedes implementation: PASS
- S058 traces to issue #80 and plan 028: PASS
- The monolith remains until replacement contract and 2.0.0 amendment are designed: PASS
- Safety-critical runtime logic and tests remain unchanged: PASS
- Test-first negative corpus fixtures are required: PASS
- Pinned release guidance move receives dated changelog decisions: PASS
- UTF-8, LF, forbidden-dash, mojibake, and secret gates are explicit: PASS
- No scope crosses the outside-the-game boundary: PASS

Post-design re-check: PASS. The MAJOR amendment changes documentation authority,
not the safety, analyze, CI, or pull-request gates.

## Project Structure

```text
docs/
  README.md
  book.toml
  theme/
  src/
    getting-started/
    features/
    concepts/
    reference/
    development/
  project/
    governance.md
    releasing.md
    build-autopilot.md
    migration-ledger.json
    migration-ledger.md
    architecture-decisions/
    build-plans/{README.md,plan-028.md}
  archive/
    build-plans/{README.md,plan-001.md..plan-027.md}
    website/ultimate-resource-meter.md
specs/058-documentation-corpus/
```

## Implementation Phases

1. Complete specification, clarification decisions, checklists, design artifacts,
   tasks, and blocking cross-artifact analysis.
2. Freeze the baseline ledger and write failing corpus-policy fixtures.
3. Establish lifecycle directories and evidence-backed plan indexes.
4. Amend governance and move current maintainer records atomically.
5. Split existing user and technical prose into canonical pages, then remove the
   competing monolith and long root manual.
6. Repair current and historical navigational references with exact exceptions.
7. Bring corpus policy, mdBook, text hygiene, and full Cargo parity green.
8. Complete independent review, publish the PR, and resolve both authorized
   hosted review rounds before the user merge gate.

## Architecture Decisions

- Use actual baseline counts (50 artifacts, 20 H2 units, 27 plans).
- Use a checked JSON ledger plus human-readable lifecycle indexes.
- Preserve all old plans and the Ultimate article as history; delete no unique history.
- Publish the brand standard in the developer track.
- Use Constitution 2.0.0 for the authority-model change.
- Keep one active plan 028 during S058 and archive it only after merge housekeeping.
- Preserve exact historical changelog literals through narrow ledger exceptions.
- Extend the existing dependency-free policy suite without changing docs CI checkout depth.
- Omit a new blog post because S058 reorganizes existing documentation and
  explicitly archives an orphaned announcement; it adds no user-facing product feature.

## Complexity Tracking

No constitution violation requires justification. The constitution amendment is
an intentional governance migration required by the issue, not a waived gate.
