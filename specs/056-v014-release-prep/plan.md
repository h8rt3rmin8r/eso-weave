# Implementation Plan: v0.14.0 Release Preparation

## Summary

Prepare the v0.14.0 release candidate by adding a bounded Highlights excerpt to
the complete Unreleased changelog, validating the exact generated release body,
and recording chronological S056 planning. Leave the version rollover, tag,
assets, and live Ultimate verification to their explicitly gated post-merge
stages.

## Technical Context

- Markdown release history in `CHANGELOG.md`
- Existing dependency-free Bash and awk release-note generator
- Existing one-to-six bullet and 120-word release-note contract
- Cargo release rollover governed by `release.toml` and `docs/releasing.md`
- Documentation-only pull request with no Rust or pipeline behavior changes

## Constitution Check

- Specification precedes implementation: PASS
- Issue #85 is the atomic release-preparation outcome: PASS
- No safety-critical product behavior changes: PASS
- Existing release-note contract is reused without modification: PASS
- Pinned release scripts, workflows, configuration, and guide remain unchanged: PASS
- Version rollover, tag creation, and publication remain separately authorized: PASS
- Documentation-site work begins only after the release is published: PASS
- UTF-8 without BOM, LF, punctuation, and mojibake gates are explicit: PASS

## Implementation Phases

1. Complete specification, clarification, research, model, contract, checklists,
   tasks, and blocking analysis.
2. Draft two user-facing v0.14.0 Highlights bullets from the detailed S054 and
   S055 changelog.
3. Record S056 in the detailed changelog and chronological build-plan index.
4. Run release-note contract tests, preview the exact candidate notes, and verify
   scope, repository state, and text hygiene.
5. Complete parallel code, governance, and release-safety review.
6. Commit, push, open the closing pull request, and complete the initial review
   plus at most one explicitly authorized second Codex review round.

## Decisions

- Use two outcome-oriented bullets, one for dashboard cohesion and one for exact
  Ultimate visibility.
- Select v0.14.0 rather than a patch because exact Ultimate visibility is a new
  backward-compatible user capability.
- Do not edit release machinery because S047 already enforces the compact output
  and immutable tagged changelog link.
- Keep 0.13.0 as the repository version until the post-merge release command
  performs the single authoritative rollover.
- Do not add a blog post. S056 is release preparation rather than a feature, and
  its Highlights are the canonical announcement input. Adding content to the
  intentionally deferred documentation-site corpus would violate scope.

## Complexity Tracking

No constitution violations require justification.
