# Implementation Plan: Fishing and Weaving README Documentation

**Branch**: `017-fishing-weaving-docs` | **Date**: 2026-07-12 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/017-fishing-weaving-docs/spec.md`

## Summary

Add two detailed usage sections to the project README, one for fishing and one
for weaving, and reorder the README so the Disclaimer becomes the next-to-last
section immediately before the License. The Fishing section documents the
interaction model and prerequisites so a player can operate it correctly and
self-diagnose the early-stop symptom; the Weaving section documents a single-bar
overview and the default timings. Documentation only; no source or test changes.

## Technical Context

**Language/Version**: Markdown (GitHub-flavored)

**Primary Dependencies**: none

**Storage**: N/A

**Testing**: manual review of rendered README plus text-hygiene checks (UTF-8
without BOM, LF, no em- or en-dashes, links resolve)

**Target Platform**: the project README rendered on the repository host

**Project Type**: documentation

**Performance Goals**: N/A

**Constraints**: text hygiene rules; documented values must match the shipped code

**Scale/Scope**: one file, `README.md`

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- I. Spec-Driven Development: PASS. Runs the spec-kit sequence under build plan
  004 slice 017.
- II. Safety-Critical Surfaces Are Sacrosanct: PASS. No source or test change; no
  safety surface is touched.
- III. Test-First With Explicit Seams: N/A. Documentation only; there is no code
  to test-drive. Validation is manual review plus text-hygiene checks.
- IV. CI Parity Before Every Commit: PASS by exception. A documentation-only
  commit that touches no Rust source has no cargo gate to run, but still obeys the
  text hygiene rules.
- V. Bounded Scope: Outside The Game: PASS. Documentation only.

Text hygiene: UTF-8 without BOM, LF, no em- or en-dashes. No violations.

## Project Structure

### Documentation (this feature)

```text
specs/017-fishing-weaving-docs/
├── plan.md
├── quickstart.md        # validation steps (render and hygiene checks)
└── tasks.md
```

Data model and contracts are not applicable: this feature introduces no data
entities and no external interfaces. The only artifact is prose in `README.md`.

### Source Code (repository root)

```text
README.md   # add Fishing and Weaving sections; move Disclaimer before License
```

**Structure Decision**: Single documentation file. New section order: banner and
badges, Installation, Fishing, Weaving, Disclaimer, License.

## Complexity Tracking

No constitution violations; no entries.
