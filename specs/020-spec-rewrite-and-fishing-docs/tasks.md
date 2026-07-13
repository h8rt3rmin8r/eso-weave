---

description: "Task list for Specification Rewrite and Fishing Documentation Fix"
---

# Tasks: Specification Rewrite and Fishing Documentation Fix

**Input**: Design documents from `specs/020-spec-rewrite-and-fishing-docs/`

**Tests**: None (documentation only). Verification is mermaid validation, a
stale-reference grep, and a text-hygiene check.

## Phase 1: User Story 1 - Accurate architecture of record (Priority: P1)

- [ ] T001 [US1] Author `docs/ESO-Weave-Specification-v0.2.0.md`: declarative RFC
  style, built subsystems with current defaults, including the API-version
  automation, and the expanded mermaid diagram set.
- [ ] T002 [US1] Validate every mermaid diagram in the new spec.
- [ ] T003 [US1] Repoint every `v0.1.0` spec-path reference to `v0.2.0` across
  `CLAUDE.md`, `.specify/memory/constitution.md`, `docs/build-autopilot.md`,
  `docs/plans/*`, and the citing `specs/*`; remove the superseded v0.1.0 file.
- [ ] T004 [US1] Record the supersession and autopilot re-affirmation as a dated
  `CHANGELOG.md` decision plus a Documentation entry.

## Phase 2: User Story 2 - Fishing docs require bait (Priority: P1)

- [ ] T005 [US2] In `README.md`, add bait as a fishing prerequisite, insert a bait
  step in "Using it" before the F2 press (renumbering), and add a bait check to
  troubleshooting.

## Phase 3: Verification

- [ ] T006 Grep the repository for the old spec path; confirm none survive.
- [ ] T007 Text-hygiene check on changed and added files (no em-dashes or
  en-dashes; UTF-8 no BOM; LF).

## Dependencies

T001 to T004 form the spec rewrite; T005 is independent. T006/T007 follow all.
