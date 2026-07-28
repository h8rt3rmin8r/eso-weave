# Feature Specification: Master Specification Refresh

**Feature Branch**: `040-spec-refresh`

**Created**: 2026-07-28

**Status**: Draft

**Input**: GitHub issue #15 (refresh master specification to match current
implemented behavior), plus operator direction on presentation: use diagrams
sparingly and deliberately, make the document look sharp, and write declaratively
about what the system is rather than where it stands or where it came from.

## Overview

The master specification is the architecture of record. Every feature traces to
it, and every spec-kit slice is scoped against it. A document that lags the
system it governs stops being a reference and starts being a hazard: a slice
scoped against a stale baseline inherits the staleness.

Nine slices have landed since the document was last revised. The application
gained a menu gate that suppresses interception, six observable block families on
the pixel bus, a grid that wraps onto a second row, out-of-band display
detection, and auto-potion, which is the first feature to synthesize input from a
beacon reading. None of that is described correctly by the current text, and the
auto-potion section that was appended sits at a heading level and a section number
that collide with an existing subsection.

This feature brings the document level with the system and fixes how it reads.

## Clarifications

### Session 2026-07-28

Answered under the build-phase autopilot decision policy, plus explicit operator
direction on presentation.

- Q: Does the filename keep its embedded version? -> A: No. The document becomes
  `docs/ESO-Weave-Specification.md` and carries its version in the header block.
  Embedding a version in a filename guarantees that every revision either churns
  every reference in the repository or lets the filename lie. There are 47
  references today; a rename costs one mechanical pass now and nothing ever
  again.
- Q: What version does the document take? -> A: 1.0.0, and the header states
  that the document version is independent of the application version. Tying the
  two was the source of the confusion in the issue, where a 0.2.0 document beside
  a 0.8.1 application read as seven versions of drift when the document had
  simply never been renumbered.
- Q: How many diagrams, and where? -> A: Four, each earning its place, none
  adjacent to another. The current document has ten, including three in close
  succession across the architecture, concurrency, and interception sections,
  which is what made them unreadable as a group. A diagram is kept only where the
  subject is genuinely a graph: the component architecture, the interception
  decision, the fishing state machine, and the pixel-bus data flow. Everything
  else that was drawn is a list or a table pretending to be a picture, and reads
  better as the list or table it always was.
- Q: What happens to the Open Items section and the owed-validation appendix? ->
  A: Both are removed. They are ledgers of what is absent and what is owed, which
  is the opposite of a declarative statement of what the system is. Work that is
  genuinely outstanding lives on the issue tracker, which is built for it.
- Q: How is the writing changed? -> A: Every construction that hedges, dates
  itself, or narrates history is rewritten. No "currently", "for now", "at this
  version", "originally", "post-1.0", "flagged for", or "was corrected by a dated
  decision". The document states what the system is and why it is that way. The
  reasoning behind a design stays, because reasoning is architecture; the account
  of which slice changed it does not, because that is what version control and
  the changelog are for.
- Q: Does the specification document its own defects? -> A: No. A section table
  that renders as three broken tables is fixed, not annotated.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - The document describes the system that exists (Priority: P1)

An engineer or agent scoping a new feature reads the specification and finds the
behavior the application actually has: the menu gate, the full block contract,
the grid geometry, display detection, auto-potion, and the current interface.

**Why this priority**: This is the feature. Everything else is presentation.

**Independent Test**: For each subsystem, compare the document against the
shipping source and confirm no statement is false and no shipped capability is
absent.

**Acceptance Scenarios**:

1. **Given** the document, **When** a reader looks up any pixel-bus block,
   **Then** its index, position, encoding, and semantics match the addon and the
   reader.
2. **Given** the document, **When** a reader looks up the keybinding defaults,
   **Then** all three application toggles appear with their keys.
3. **Given** the document, **When** a reader looks for the features that act on
   beacon signals, **Then** the menu gate and auto-potion are both described, and
   the signals that act on nothing are identified as such.
4. **Given** the document, **When** a reader consults the interface or
   configuration sections, **Then** the described regions, sections, and limits
   match what ships.

---

### User Story 2 - The document reads as a specification, not a history (Priority: P1)

A reader learns what the system is and why, without being told what it used to
be, what it might become, or which slice changed it.

**Why this priority**: Equal with US1 because a document that is factually
correct but hedged still fails at its job. A specification that says a value is
"flagged for measurement" or a path is "post-1.0" is not stating a design; it is
recording a mood.

**Independent Test**: Search the document for hedging and historical
constructions and find none.

**Acceptance Scenarios**:

1. **Given** the document, **When** searched for "currently", "for now",
   "originally", "at this version", "post-1.0", or "TBD", **Then** there are no
   matches.
2. **Given** any design decision in the document, **When** a reader asks why,
   **Then** the reasoning is present without reference to the slice that made it.
3. **Given** the document, **When** a reader looks for a list of unbuilt work,
   **Then** there is none; that lives on the tracker.

---

### User Story 3 - The diagrams help (Priority: P2)

Each diagram is legible on its own, is about something that is genuinely a graph,
and is separated from the next by enough text to be read as its own idea.

**Why this priority**: Below correctness, but it is the operator's stated
complaint and the reason presentation is in scope at all.

**Independent Test**: Confirm no two diagrams are adjacent, and that each
remaining one describes a branch, a state machine, or a topology rather than a
sequence of steps that a list would state more plainly.

**Acceptance Scenarios**:

1. **Given** the document, **When** the diagrams are located, **Then** no two are
   separated by fewer than several paragraphs of body text.
2. **Given** any diagram, **When** a reader asks what it shows, **Then** it shows
   a topology, a decision, a state machine, or a data flow.
3. **Given** content that was previously drawn but is a linear sequence, **When**
   a reader looks for it, **Then** it appears as a table or an ordered list.

---

### Edge Cases

- **A reference to the old filename survives the rename.** Every reference is
  updated in the same change, including historical spec-kit artifacts, because a
  dangling path is a dangling path regardless of which directory it lives in.
- **A table renders broken.** The block table is one table, not three separated
  by blank lines.
- **A section number collides.** Auto-potion becomes its own top-level section
  rather than a second 10.6.
- **An anchor in the table of contents no longer resolves.** The table of
  contents is rebuilt from the final headings.

## Requirements *(mandatory)*

### Functional Requirements

**Accuracy**

- **FR-001**: The document MUST describe every capability the application ships:
  weave automation, fishing automation, auto-potion, the menu gate, the full
  pixel-bus block contract, the grid geometry, out-of-band display detection, and
  the current interface and configuration surfaces.
- **FR-002**: No statement in the document may be false against the shipping
  source. Every constant, default, key, limit, and block encoding MUST match.
- **FR-003**: The document MUST identify which decoded signals act on application
  behavior and which are observable only, because that distinction is the
  project's central safety property.
- **FR-004**: The pixel-bus block table MUST render as a single table.
- **FR-005**: Section numbering MUST be unique, and auto-potion MUST occupy its
  own top-level section.

**Presentation**

- **FR-006**: The document MUST contain no more than four diagrams, no two of
  them adjacent, each depicting a topology, a decision, a state machine, or a
  data flow.
- **FR-007**: Content that is a linear sequence MUST be a table or an ordered
  list rather than a diagram.
- **FR-008**: The document MUST be written declaratively in the present tense. It
  MUST NOT contain hedging or self-dating constructions, and MUST NOT narrate its
  own history or the history of the system.
- **FR-009**: Design reasoning MUST be retained where it explains why the system
  is shaped as it is, without attributing it to a slice or a date.
- **FR-010**: The document MUST NOT carry a list of unbuilt, deferred, or owed
  work. That belongs to the issue tracker.

**Identity and references**

- **FR-011**: The document MUST be renamed to a filename carrying no version, and
  MUST state its version in its header block along with the fact that the
  document version is independent of the application version.
- **FR-012**: Every reference to the old filename in the repository MUST be
  updated, including in historical spec-kit artifacts.
- **FR-013**: The table of contents MUST be rebuilt so every entry resolves to a
  heading in the final document.

**Boundaries**

- **FR-014**: This feature MUST NOT change application behavior. No Rust source,
  no addon source, and no test may change except where a reference to the
  document's path appears.
- **FR-015**: The changelog MUST record the refresh and the dated decisions for
  the rename and the versioning scheme.

### Key Entities

- **Master specification**: The architecture of record. One document, versioned
  independently of the application, describing what the system is.
- **Diagram budget**: Four. A cap that forces each diagram to justify itself.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Every capability shipped in the application is described, verified
  subsystem by subsystem against the source.
- **SC-002**: Zero false statements about constants, defaults, keys, limits, or
  block encodings.
- **SC-003**: Zero occurrences of the hedging and self-dating vocabulary listed
  in the clarifications.
- **SC-004**: At most four diagrams, with no two adjacent.
- **SC-005**: Zero references to the old filename anywhere in the repository.
- **SC-006**: Every table-of-contents entry resolves to a heading.
- **SC-007**: The full merge gate passes, and no Rust or addon source changes
  except path references.

## Assumptions

- **The shipping source is the truth.** Where the document and the code disagree,
  the code is right and the document is corrected. This feature changes no
  behavior to match the document.
- **The tracker owns outstanding work.** Removing the open-items ledger loses
  nothing, because anything real in it is either already an issue or was never
  going to be done.

## Dependencies

- The shipping source: the addon, the pixel-bus reader, the input engine, the
  weave engine, the fishing and auto-potion controllers, the interface, and the
  configuration model.
- The constitution, `CLAUDE.md`, `docs/build-autopilot.md`, `docs/plans/`, and the
  historical `specs/NNN-*/` artifacts, all of which reference the document by
  path.
