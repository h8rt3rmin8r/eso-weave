# Feature Specification: Specification Rewrite and Fishing Documentation Fix

**Feature Branch**: `020-spec-rewrite-and-fishing-docs`

**Created**: 2026-07-12

**Status**: Draft

**Input**: User description: "Full rewrite of the master ESO Weave technical specification, documenting everything built so far with expanded mermaid diagrams, in an authoritative declarative voice. Also fix the README fishing 'Using it' steps, which omit bait selection: without bait the F2 automation fails."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Accurate architecture of record (Priority: P1)

A contributor or agent opens the master specification to understand the system.
The document describes the system as built, in a declarative voice, with diagrams
that make the architecture, threading, pixel-bus protocol, beacon lifecycle, and
the API-version automation clear at a glance. Every repository pointer resolves to
the current document.

**Why this priority**: The specification is the architecture of record that every
feature traces to. A stale spec misleads every downstream reader and agent.

**Independent Test**: Open the new spec; confirm it covers the built subsystems
(input engine, weave engine, fishing, pixel bus, beacon manager plus API-version
automation, GUI, config and state), that each mermaid diagram renders, and that no
repository reference points at the old file.

**Acceptance Scenarios**:

1. **Given** the rewritten spec, **When** a reader looks for any built subsystem,
   **Then** it is documented declaratively with current defaults and behavior.
2. **Given** the repository, **When** any file references the master spec, **Then**
   it points at `docs/ESO-Weave-Specification-v0.2.0.md` and the old file is gone.
3. **Given** each mermaid diagram, **When** it is rendered, **Then** it is valid.

### User Story 2 - Fishing docs do not set users up to fail (Priority: P1)

A user reads the README fishing section and follows the steps. Because the steps
now include selecting bait, the F2 automation actually starts a cast instead of
silently failing.

**Why this priority**: Missing the bait step is a correctness gap that guarantees
failure for a first-time user, which is as severe as an accurate-spec need.

**Independent Test**: Read the fishing "Using it" steps and confirm bait selection
appears before the F2 press, and that the prerequisites and troubleshooting mention
bait.

**Acceptance Scenarios**:

1. **Given** the fishing "Using it" steps, **When** a user follows them in order,
   **Then** bait selection precedes pressing F2.
2. **Given** the fishing prerequisites and troubleshooting, **When** a user checks
   them, **Then** the bait requirement is stated and tied to the no-cast symptom.

### Edge Cases

- A reader on a plain markdown viewer without mermaid rendering: diagrams degrade
  to readable fenced code, and the prose still stands on its own.
- Historical changelog and feature references: repointing keeps links valid, and
  the referenced appendix still exists in v0.2.0.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The master specification MUST be rewritten as
  `docs/ESO-Weave-Specification-v0.2.0.md`, documenting the built subsystems
  declaratively with current defaults.
- **FR-002**: The spec MUST include expanded mermaid diagrams, each of which MUST
  be valid and render.
- **FR-003**: Every repository reference to the master spec MUST point at the v0.2.0
  file, and the superseded v0.1.0 file MUST be removed.
- **FR-004**: The standing autopilot authorization MUST be re-affirmed against
  v0.2.0, recorded as a dated decision.
- **FR-005**: The README fishing "Using it" steps MUST include selecting bait before
  pressing F2, and the prerequisites and troubleshooting MUST state the bait
  requirement.
- **FR-006**: All changed and added text files MUST be UTF-8 without a byte order
  mark, LF, and free of em-dashes and en-dashes.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A repository grep for the old spec path returns nothing.
- **SC-002**: Every mermaid diagram in the new spec validates.
- **SC-003**: The fishing "Using it" steps name bait selection before F2.
- **SC-004**: No cargo gate applies (docs only); text hygiene holds.

## Assumptions

- The spec matches the existing RFC-style structure and section numbering rather
  than importing a different house style.
- Repointing historical references (changelog, past features) is preferred over
  leaving broken links to the removed file, and the referenced content still exists
  in v0.2.0.
- Autopilot re-affirmation is recorded in `CHANGELOG.md` rather than as a
  constitution amendment, since the constitution already points at the spec path.
