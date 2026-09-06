# Implementation Plan: v0.13.0 Release Preparation

## Summary

Prepare the v0.13.0 release candidate by adding a bounded Highlights excerpt to the complete Unreleased changelog, validating the exact generated release body, and recording the chronological S053 plan. Leave the version rollover, tag, assets, and live field verification to their explicitly gated post-merge stages.

## Technical Context

- Markdown release history in `CHANGELOG.md`
- Existing dependency-free Bash and awk release-note generator
- Existing one-to-six bullet and 120-word release-note contract
- Cargo release rollover governed by `release.toml` and `docs/releasing.md`
- Documentation-only pull request with no Rust or pipeline behavior changes

## Constitution Check

- Specification precedes implementation: PASS
- Issue #68 is the atomic release-preparation outcome: PASS
- No safety-critical product behavior changes: PASS
- Existing release-note contract is reused without modification: PASS
- Pinned release scripts, workflows, configuration, and guide remain unchanged: PASS
- Version rollover, tag creation, and publication remain separately authorized: PASS
- UTF-8 without BOM, LF, punctuation, and mojibake gates are explicit: PASS

## Implementation Phases

1. Complete specification, clarification, research, model, contract, checklists, and analysis.
2. Draft four user-facing v0.13.0 Highlights bullets from the detailed S048 through S052 changelog.
3. Record S053 in the detailed changelog and chronological build-plan index.
4. Run release-note contract tests, preview the exact candidate notes, and verify scope and text hygiene.
5. Commit, push, open the closing pull request, and complete no more than two Codex review rounds.

## Decisions

- Use four outcome-oriented bullets, one each for lifecycle truth, transient-action safety, sprint-aware potion behavior, and UI organization.
- Do not edit release machinery because S047 already enforces the required compact output and tagged changelog link.
- Keep 0.12.0 as the repository version until the post-merge release command performs the single authoritative rollover.

## Complexity Tracking

No constitution violations require justification.
