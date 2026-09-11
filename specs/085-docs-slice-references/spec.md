# Feature Specification: Compact Work-Slice References

**Feature Branch**: `codex/s085-docs-slice-references`

**Created**: 2026-09-11

**Status**: Implemented

**Input**: Work slice S085 implements issue #126 by making `S###` the only reader-facing work-slice form and moving exact implementation identifiers behind compact, navigable evidence.

## Clarifications

### Session 2026-09-11

- A work-slice reference is an uppercase `S` followed by exactly three digits, from `S000` through `S999`.
- Published prose, tables, captions, alternative text, and inline code are reader-facing and must follow the compact form.
- Fenced code samples may contain otherwise invalid tokens because their contents are literal examples rather than provenance prose.
- Markdown link destinations and HTML attributes may contain concrete spec paths or source fragments because the visible link label remains compact.
- The persisted algorithm identifier `s069-v1` is not a work-slice reference and remains unchanged through one exact, page-scoped exception.
- Exact test symbols remain authoritative in source files and the unpublished content-coverage manifest. Published pages use behavioral summaries and links instead.
- S085 proves that slice references themselves do not create narrow-width overflow. The broader responsive-table redesign remains assigned to issue #127.

## User Scenarios & Testing

### User Story 1 - Read compact provenance (Priority: P1)

A reader can scan developer prose and tables without long slice-prefixed identifiers obscuring the content or crushing narrow layouts.

**Independent Test**: Search every published Markdown page and generated HTML page, then confirm that work-slice provenance is shown only as `S###` and that no visible long `s###_...` identifier remains.

**Acceptance Scenarios**:

1. **Given** a paragraph, table cell, caption, alternative text, or inline code span, **When** it cites a work slice, **Then** the visible citation uses uppercase `S` plus exactly three digits.
2. **Given** a test relationship previously named with a long slice-prefixed symbol, **When** the page is normalized, **Then** it uses a behavioral description, compact work-slice reference, and relevant source or spec link.

---

### User Story 2 - Reach exact implementation evidence (Priority: P1)

A contributor can move from a compact published reference to the relevant work-slice specification or source file without treating a long implementation symbol as reader-facing prose.

**Independent Test**: Follow every new compact evidence link from the built manual and verify that it reaches the intended repository source or specification while the exact symbol remains available in the canonical manifest or source.

**Acceptance Scenarios**:

1. **Given** a compact `S###` link, **When** a contributor follows it, **Then** the matching specification is reachable.
2. **Given** behavioral test evidence, **When** a contributor follows its link, **Then** the relevant source or test file is reachable and the unpublished manifest or source retains the exact anchor.

---

### User Story 3 - Prevent reference regressions (Priority: P2)

A maintainer receives a deterministic policy failure when malformed or layout-risky work-slice references enter published documentation.

**Independent Test**: Run focused policy fixtures for valid references, malformed forms, long symbols, fenced examples, link destinations, the algorithm exception, generated HTML, and narrow-width token bounds.

**Acceptance Scenarios**:

1. **Given** malformed visible provenance such as `S60`, `s060`, `work slice 060`, a concrete spec-directory name, or a long `s060_...` symbol, **When** policy runs, **Then** it reports an actionable error.
2. **Given** `S060`, a compact link label, a fenced literal example, a link destination, or `s069-v1` in its documented page, **When** policy runs, **Then** the valid content passes.
3. **Given** a narrow documentation viewport, **When** the normalized pages render, **Then** no work-slice reference exceeds four visible characters or causes horizontal overflow.

### Edge Cases

- A correct `S###` label may link to a concrete `specs/NNN-name/spec.md` destination without exposing that path as visible prose.
- Inline code is not a blanket exception because every current long test-symbol offender is formatted as inline code.
- Fenced examples are excluded narrowly; content after the closing fence is checked normally.
- `s069-v1` is allowed only in the two pages that document the runtime algorithm value.
- Non-published specifications, project records, changelog history, source, and tests are outside the reader-facing scan.
- Existing nonnumeric evidence labels such as `T-WEAVE-EPOCH` are not confused with work-slice references.

## Requirements

### Functional Requirements

- **FR-001**: Published documentation MUST define a work-slice reference as uppercase `S` followed by exactly three digits.
- **FR-002**: Published visible content MUST NOT use lowercase slice references, incorrect digit widths, numeric `build slice` or `work slice` phrases, concrete numbered spec-directory paths, or long `s###_...` implementation symbols as provenance.
- **FR-003**: Existing long slice-prefixed test references MUST be replaced with behavioral summaries, compact `S###` references, and relevant links without losing the evidence relationship.
- **FR-004**: Exact test identifiers MUST remain discoverable in canonical source, while unpublished evidence records retain stable representative relationships without being duplicated into reader-facing tables.
- **FR-005**: The `s069-v1` algorithm identifier MUST remain unchanged and MUST be documented as a narrow non-slice exception.
- **FR-006**: Markdown policy MUST inspect visible prose, tables, captions, alternative text, link labels, and inline code while excluding fenced code, comments, link destinations, and HTML attributes.
- **FR-007**: Generated-site policy MUST inspect visible HTML text, including inline code, while excluding comments, scripts, styles, tags, and preformatted code samples.
- **FR-008**: Policy tests MUST cover canonical references, malformed forms, long identifiers, exception boundaries, source-tree integration, generated HTML, and narrow token bounds.
- **FR-009**: Published pages MUST expose working links to relevant specifications or source evidence where exact lookup is useful.
- **FR-010**: Public and bundled documentation MUST expose the same normalized content and repository links.
- **FR-011**: S085 MUST NOT modify application runtime behavior, test identifiers, evidence anchors, or the frozen content-coverage contract.
- **FR-012**: Documentation build, policy, link, spelling, UTF-8, LF, forbidden-dash, mojibake, and diff-integrity gates MUST pass.

### Key Entities

- **Work-slice reference**: Visible provenance token matching uppercase `S` plus exactly three digits.
- **Evidence description**: Stable behavioral language that identifies why a source or test matters without exposing a long implementation symbol.
- **Evidence destination**: Repository specification, source file, or test file reached through a compact visible link.
- **Literal exception**: Content excluded only because it is a fenced code sample, hidden destination, HTML attribute, or the exact page-scoped `s069-v1` algorithm identifier.

## Success Criteria

### Measurable Outcomes

- **SC-001**: All 31 existing long slice-prefixed test-symbol occurrences are removed from published documentation.
- **SC-002**: All published work-slice provenance uses `S000` through `S999`, with no visible malformed numeric form or concrete numbered spec-directory path.
- **SC-003**: Every normalized implementation relationship retains a behavioral description and a working link to the relevant specification or source file.
- **SC-004**: Focused policy fixtures prove every allowed and rejected boundary, including the exact page-scoped `s069-v1` exception.
- **SC-005**: Generated-site validation proves no visible slice reference exceeds four characters, so slice provenance cannot create narrow-layout overflow.
- **SC-006**: Public and bundled documentation builds plus all repository merge gates pass without runtime changes.

## Assumptions

- The existing unpublished `docs/project/content-coverage.json` manifest remains the canonical mapping from stable evidence labels to exact source anchors.
- GitHub repository links are appropriate for contributor-level source and specification navigation from the published manual.
- Exact implementation names are more useful at the source destination than when duplicated in reader-facing tables.
- Issue #127 will provide the general responsive-table redesign after S085 removes the immediate long-token pressure.

## Out of Scope

- Renaming Rust tests, specification directories, or persisted algorithm identifiers.
- Changing the frozen content-coverage contract or its exact evidence anchors.
- General table restructuring, card conversion, or viewport work assigned to issue #127.
- Runtime, packaging, capture, telemetry, or game-asset changes.
