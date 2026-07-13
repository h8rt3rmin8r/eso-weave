# Implementation Plan: Specification Rewrite and Fishing Documentation Fix

**Branch**: `020-spec-rewrite-and-fishing-docs` | **Date**: 2026-07-12 | **Spec**: [spec.md](spec.md)

## Summary

Supersede the master specification with a v0.2.0 rewrite that documents the system
as built, in a declarative voice with expanded, validated mermaid diagrams; repoint
every reference and remove the old file; re-affirm autopilot against v0.2.0; and
correct the fishing README to require bait selection. Documentation only.

## Technical Context

**Language/Version**: Not applicable (documentation).

**Testing**: No cargo gate (docs only). Verification is a mermaid validation pass,
a repository grep for the old path, and a text-hygiene check.

**Constraints**: UTF-8 without a byte order mark, LF, no em-dashes or en-dashes.

## Constitution Check

- **I. Spec-Driven Development**: PASS. Full spec-kit artifacts precede the change.
- **II. Safety-Critical Surfaces**: PASS. No source or safety surface is touched.
- **IV. CI Parity**: N/A for docs; text hygiene still holds.
- **Governance**: The master spec supersession re-affirms the standing autopilot
  authorization against v0.2.0, recorded as a dated `CHANGELOG.md` decision. The
  constitution references the spec by path, which is repointed; no constitution
  semver amendment is required.

## Approach

1. Author `docs/ESO-Weave-Specification-v0.2.0.md`: RFC-style structure and section
   numbering, declarative voice, current defaults, and the built subsystems
   including the slice-018 API-version automation. Add and validate the mermaid
   diagram set.
2. Repoint every `v0.1.0` spec-path reference to `v0.2.0` across `CLAUDE.md`,
   `.specify/memory/constitution.md`, `docs/build-autopilot.md`, `docs/plans/*`,
   and the `specs/*` that cite it. Remove the superseded v0.1.0 file.
3. Record the supersession and autopilot re-affirmation as a dated `CHANGELOG.md`
   decision, and add a Documentation entry.
4. README: add bait as a fishing prerequisite, as a step in "Using it" before F2,
   and as a troubleshooting check.

## Project Structure

```text
docs/ESO-Weave-Specification-v0.2.0.md   # new; v0.1.0 removed
README.md                                 # bait prerequisite, step, troubleshooting
CHANGELOG.md                              # Documentation + Decisions entries
CLAUDE.md, .specify/memory/constitution.md, docs/build-autopilot.md,
docs/plans/*, specs/*                     # repointed references
```

## Complexity Tracking

No violations.
