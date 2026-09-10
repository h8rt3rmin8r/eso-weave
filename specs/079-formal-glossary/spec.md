# Feature Specification: Formal Alphabetical Glossary

**Feature Branch**: `codex/s079-formal-glossary`
**Created**: 2026-09-10
**Status**: Implemented in PR #147
**Input**: Issue #120 and the documentation presentation epic #119

## User Scenarios and Testing

### User Story 1 - Scan one authoritative glossary (Priority: P1)

A reader opens the Glossary and moves through a single alphabetical reference
whose entries distinguish canonical terms, aliases, definitions, and related
documentation.

**Independent Test**: Inspect the source and built page, confirm that every entry
is under the correct alphabetical heading, and confirm that the former duplicate
Search vocabulary section is absent.

**Acceptance Scenarios**:

1. **Given** the Glossary page, **when** a reader scans its headings, **then**
   canonical terms appear exactly once in case-insensitive alphabetical order.
2. **Given** a canonical term with several player phrases, **when** its entry is
   read, **then** aliases, definition, and related link are visibly distinct.
3. **Given** a term that previously appeared in both lists, **when** the new page
   is inspected, **then** its concepts and aliases are merged into one entry.

---

### User Story 2 - Find vocabulary by canonical or player wording (Priority: P1)

A player or developer searches for familiar wording and reaches the glossary
entry and canonical page without knowing the product's exact term in advance.

**Independent Test**: Build the book, inspect its generated search index, and
verify representative canonical terms and aliases from every term family.

**Acceptance Scenarios**:

1. **Given** a required search alias, **when** the book is built, **then** the
   visible glossary prose contains that alias beside its canonical term.
2. **Given** a canonical entry, **when** its related link is followed, **then**
   the link reaches the established published documentation target.
3. **Given** the complete frozen terminology map, **when** policy validation
   runs, **then** every canonical term and every required alias is represented.

---

### User Story 3 - Navigate accessibly at any supported width (Priority: P2)

A keyboard, screen-reader, narrow-screen, or wide-screen reader uses the alphabet
navigation and heading outline without losing focus visibility or page context.

**Independent Test**: Validate the semantic source contract and built HTML, then
inspect the page at representative narrow and wide widths in both book themes.

**Acceptance Scenarios**:

1. **Given** alphabet navigation, **when** a reader follows a letter, **then**
   focus moves to an existing letter heading through a descriptive link.
2. **Given** a screen reader heading list, **when** the page is traversed, **then**
   letter groups and canonical entries form a consistent hierarchy.
3. **Given** a narrow viewport, **when** the navigation wraps, **then** links stay
   readable, individually focusable, and free of page-level overflow.

### Edge Cases

- Several entries share the same first letter or differ only by a qualifier.
- An alias contains punctuation, an abbreviation, a file name, or mixed case.
- One alias applies to more than one platform-specific canonical term.
- A letter has no canonical terms and therefore must not create a dead link.
- Long alias groups wrap at narrow widths without becoming an undifferentiated list.

## Requirements

### Functional Requirements

- **FR-001**: The Glossary MUST contain one formal primary reference and MUST NOT
  retain a separate Search vocabulary list.
- **FR-002**: Every canonical term and alias in the current two-list glossary MUST
  remain represented, with duplicates merged.
- **FR-003**: The glossary MUST also represent every canonical term and alias in
  the established S059 terminology search map.
- **FR-004**: Canonical entries MUST be ordered by Unicode case-insensitive term
  order within ascending single-letter groups.
- **FR-005**: Every entry MUST expose a canonical heading, an explicit Aliases
  field, a substantive definition, and at least one descriptive related link.
- **FR-006**: A term MUST appear as a canonical heading exactly once.
- **FR-007**: Alphabet navigation MUST link only to populated letter groups and
  MUST have an accessible navigation label.
- **FR-008**: Letter groups MUST use second-level headings and canonical entries
  MUST use third-level headings so heading navigation is stable.
- **FR-009**: Player slang, platform phrases, abbreviations, file names, and
  protocol identifiers MUST remain visible prose rather than hidden metadata.
- **FR-010**: Definitions MUST explain the ESO Weave meaning and MUST keep safety
  state terms such as Unknown, Unavailable, Ready, and Active distinct.
- **FR-011**: Related links MUST retain the canonical published targets from the
  S059 terminology contract.
- **FR-012**: The glossary structure MUST be protected by focused policy tests for
  ordering, uniqueness, aliases, links, navigation, and rejection of the old list.
- **FR-013**: Public and bundled offline documentation MUST use the same glossary
  source, theme, search index, and local links.
- **FR-014**: The change MUST introduce no network-only asset, client-side
  dependency, runtime application behavior, telemetry, or game interaction.

### Key Entities

- **Glossary entry**: One canonical term, its aliases, one definition, and one or
  more related documentation links.
- **Letter group**: A populated alphabetical section containing ordered entries.
- **Alphabet navigation**: A labeled group of links to every populated letter group.
- **Terminology map**: The existing frozen canonical, alias, and target contract.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Policy validation finds exactly one heading for every contracted
  canonical term and no duplicate canonical headings.
- **SC-002**: Every contracted alias and every term or alias from the former page
  appears in visible prose inside its merged entry.
- **SC-003**: Every alphabet link resolves to a populated second-level heading,
  and no populated heading is omitted from navigation.
- **SC-004**: The generated search index contains representative canonical and
  alias phrases from all glossary families, including gameplay, safety, addon,
  protocol, configuration, logging, platform, release, and development terms.
- **SC-005**: The page produces no page-level horizontal overflow at 320 CSS
  pixels and remains readable in light and dark themes.
- **SC-006**: Documentation tests, build, link, policy, spelling, UTF-8,
  forbidden-dash, and mojibake gates pass.

## Clarifications

### Session 2026-09-10

- Q: Which term inventory is authoritative? A: Preserve the current two-list page
  and include the complete established S059 terminology map, using the map's
  canonical targets when the inventories overlap.
- Q: Which semantic structure should be used? A: Use native Markdown letter and
  term headings plus explicitly labeled prose fields. This gives mdBook stable
  anchors, search text, keyboard navigation, and screen-reader headings without a
  custom renderer.
- Q: Should empty alphabet letters be shown? A: No. Navigation includes only
  populated groups so every link has a useful target.
- Q: Is runtime application work included? A: No. S079 is a documentation and
  documentation-policy slice only.

## Dependencies

- Issue #120 owns this slice.
- Issue #119 is the parent documentation presentation epic.
- S059 supplies the frozen terminology and search contract.

## Out of Scope

New runtime terminology, application UI changes, JavaScript search replacement,
external search services, translation, screenshots, diagrams, landing-page brand
work, and responsive table redesign.
