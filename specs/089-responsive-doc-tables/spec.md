# Feature Specification: Responsive Documentation Tables

**Feature Branch**: `codex/s089-responsive-doc-tables`

**Created**: 2026-09-11

**Status**: Implemented

**Input**: User description: "Complete issue #127 as S089 by auditing every published documentation table, making dense tables readable at narrow and wide widths, preserving semantics and keyboard access, adding regression evidence, and then assessing documentation epic #119 for closure."

## User Scenarios & Testing

### User Story 1 - Read every table without losing content (Priority: P1)

A reader can inspect documentation tables at narrow and wide widths without columns collapsing into unreadable stacks, content being clipped, or the complete page moving sideways.

**Why this priority**: Tables carry setup, safety, protocol, state, and verification information. A reader must be able to compare their rows without changing devices or guessing at hidden content.

**Independent Test**: Open representative compact and dense tables at 320 and 1280 CSS pixels in both supported documentation themes. Every cell remains readable, all content remains reachable, and overflow stays inside the table boundary.

**Acceptance Scenarios**:

1. **Given** a compact table whose content fits the available width, **When** the page is read at a supported width, **Then** the table remains an ordinary table without unnecessary scrolling or an overflow notice.
2. **Given** a dense table that cannot fit without damaging readability, **When** the page is read at a narrow width, **Then** the table receives sufficient column width and its own contained horizontal scrolling boundary.
3. **Given** long code symbols, commands, links, or prose in a cell, **When** the table narrows, **Then** the content remains readable without tiny type, indiscriminate character breaking, clipping, or page-level horizontal overflow.

---

### User Story 2 - Operate overflow tables with keyboard and assistive technology (Priority: P1)

A keyboard or assistive-technology reader can discover, identify, focus, and horizontally scroll a dense table while retaining native row and header relationships.

**Why this priority**: Containing visual overflow is insufficient if the contained content cannot be reached or understood without a pointer.

**Independent Test**: Focus a genuinely overflowing table region by keyboard, verify its visible and announced context, scroll it with native keyboard input, and inspect the table accessibility structure before and after scrolling.

**Acceptance Scenarios**:

1. **Given** a table that overflows, **When** a keyboard reader reaches it, **Then** a visible cue identifies the horizontal interaction and the focused boundary has a visible focus indicator.
2. **Given** an overflowing table boundary has focus, **When** the reader uses native horizontal navigation keys, **Then** previously hidden columns become reachable without moving the entire page.
3. **Given** a table does not overflow, **When** a keyboard reader traverses the page, **Then** no redundant focus stop, region announcement, or scrolling cue is added.
4. **Given** any enhanced table, **When** assistive technology inspects it, **Then** the original table, header cells, rows, and cell associations remain intact.

---

### User Story 3 - Preserve responsive behavior through resize, zoom, print, and script failure (Priority: P2)

A reader receives truthful table behavior when the viewport changes, browser zoom reaches 200 percent, the page is printed, or local enhancement cannot run.

**Why this priority**: Responsive state is not fixed at initial load, and public plus bundled documentation must retain usable static content without a network or script dependency.

**Independent Test**: Resize a table across its overflow threshold, apply 200 percent page zoom, emulate print, and block page scripts. The interactive state follows the current geometry, print exposes complete static tables, and the script-free page contains overflow locally.

**Acceptance Scenarios**:

1. **Given** a table changes between fitting and overflowing after resize or zoom, **When** layout settles, **Then** its cue, focusability, and overflow state match the current geometry.
2. **Given** documentation is viewed at 200 percent browser zoom, **When** a dense table overflows, **Then** the page remains contained and every column remains reachable within the table boundary.
3. **Given** print output, **When** a table would normally scroll, **Then** interactive cues and focus decoration are absent and the table is not intentionally clipped by the screen treatment.
4. **Given** local page scripts are blocked, **When** a wide table is opened, **Then** native table semantics and a local overflow boundary remain available without remote resources.

---

### User Story 4 - Maintain a complete table contract (Priority: P2)

A maintainer can account for every published table and detect a regression in inventory, semantics, responsive containment, discoverability, or the five named high-density pages before merge.

**Why this priority**: The corpus contains many tables and will continue to evolve. The responsive contract must fail clearly when a new or changed table is not considered.

**Independent Test**: Run the source policy and generated-browser evidence against the complete table inventory and mutate representative source, theme, and receipt inputs. Missing coverage or accessibility behavior is rejected.

**Acceptance Scenarios**:

1. **Given** the current documentation corpus, **When** the inventory runs, **Then** it accounts for exactly 54 tables across 26 source pages.
2. **Given** Coverage Matrix, Test Strategy, State Machines, Status Reference, and Pixel Bus Protocol, **When** responsive evidence runs, **Then** each named page contributes explicit narrow and wide observations in both themes.
3. **Given** a future table is added or removed, **When** policy runs without an intentional inventory update, **Then** it fails with an actionable inventory mismatch.
4. **Given** a responsive or accessibility contract is removed, **When** mutation coverage runs, **Then** the missing behavior is rejected before publication.

### Edge Cases

- A compact table may fit at a wide width and overflow after a resize, orientation change, sidebar change, or 200 percent zoom.
- A dense table may become wide because of prose, a code symbol, a command, a URL label, column count, or a combination rather than one fixed content length.
- Two tables may share the same nearest heading and still require distinguishable accessible names.
- A page may contain several compact and dense tables with different overflow states at the same viewport width.
- A reader may scroll to the middle or end of a table and then resize it so scrolling is no longer necessary.
- Theme changes must not erase the overflow cue, scrollbar boundary, focus indicator, header contrast, or alternating-row distinction.
- Browser scrollbars may be always visible, overlaid, or hidden until interaction, so discoverability cannot depend on scrollbar appearance alone.
- Script-free output must contain wide tables without causing page-level horizontal overflow.
- Print layout may be narrower than desktop layout and must not retain screen-only focus or interaction instructions.

## Requirements

### Functional Requirements

- **FR-001**: The system MUST inventory exactly 54 Markdown tables across 26 published source pages.
- **FR-002**: The inventory MUST explicitly identify every table in Coverage Matrix, Test Strategy, State Machines, Status Reference, and Pixel Bus Protocol.
- **FR-003**: Every table MUST retain native table, header, row, and cell semantics in published and bundled documentation.
- **FR-004**: A table that fits its available width MUST remain free of redundant focus stops, region announcements, and overflow instructions.
- **FR-005**: A table that cannot fit without harming readability MUST receive a contained horizontal overflow boundary rather than shrinking type, collapsing columns to a few characters, clipping content, or widening the page.
- **FR-006**: Responsive treatment MUST NOT use indiscriminate `break-all` character splitting or reduce table text below the surrounding documentation's readable body scale.
- **FR-007**: Every genuinely overflowing boundary MUST be keyboard focusable, expose a visible focus indicator, and support the browser's native horizontal keyboard scrolling.
- **FR-008**: Every genuinely overflowing boundary MUST expose a concise visible instruction that does not rely on color or scrollbar visibility alone.
- **FR-009**: Every genuinely overflowing boundary MUST have a programmatic region name derived from visible page context and a programmatic relationship to its visible instruction.
- **FR-010**: Multiple table boundaries on one page MUST receive unique, stable programmatic names and relationships.
- **FR-011**: Overflow, cue, and focusability state MUST update after viewport, sidebar, font, or zoom-driven geometry changes.
- **FR-012**: Responsive state changes MUST preserve the reader's position when possible and MUST reset an obsolete horizontal offset when a table no longer overflows.
- **FR-013**: Source tables containing long code, command, link, or prose content MUST preserve readable tokens and natural wrapping opportunities without changing their meaning.
- **FR-014**: Screen presentation MUST prevent every table from producing page-level horizontal overflow or content hidden beneath documentation navigation.
- **FR-015**: Script-free output MUST retain native table semantics and local horizontal containment for wide content.
- **FR-016**: Print output MUST omit screen-only scrolling instructions and focus decoration and MUST avoid an intentional screen-width minimum that clips table content.
- **FR-017**: The responsive behavior MUST use the same checked-in local resources for GitHub Pages and bundled offline documentation without a remote runtime, package, font, or stylesheet.
- **FR-018**: Automated source and generated policy MUST reject inventory drift, lost table wrappers, missing semantics, missing responsive contracts, remote dependencies, or screen rules leaking into print.
- **FR-019**: Generated-browser evidence MUST cover the five named dense pages at 320 and 1280 CSS pixels in navy and light themes, for a complete 20-observation matrix.
- **FR-020**: Browser evidence MUST cover fitting and overflowing tables, page containment, cell readability, region naming, instruction association, focus visibility, native keyboard scrolling, geometry-state changes, 200 percent page zoom, print, and blocked-script output.
- **FR-021**: The work MUST preserve all existing diagram, syntax, figure, documentation policy, link, and offline delivery behavior.
- **FR-022**: The work MUST change no application, addon, gameplay, input, catalog, encounter, telemetry, packaging, or release behavior.
- **FR-023**: Issue #127 MUST close only with repository-verifiable evidence, and epic #119 MUST remain open until every child and epic completion gate is demonstrably satisfied.

### Key Entities

- **Table Placement**: Source page, source line, header signature, row count, column count, and responsive classification.
- **Table Boundary**: The local containment area around one table, with its current overflow state, focusability, accessible name, instruction relationship, and horizontal position.
- **Table Classification**: Compact or dense treatment selected from the table's semantic and content shape without changing table meaning.
- **Table Observation**: Page, table identity, theme, viewport, geometry, overflow state, focus state, cue state, semantic structure, and containment result.
- **Table Receipt**: Complete source inventory, named-page browser matrix, interaction journeys, special-state observations, failures, and pass sentinel.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Policy accounts for all 54 current tables across all 26 table-bearing source pages with zero unclassified placements.
- **SC-002**: All 20 named-page observations across five pages, two themes, and two viewport widths pass table semantics, readable geometry, local overflow, and page-containment checks.
- **SC-003**: Every observed overflowing table has one visible instruction, one unique programmatic region name, one instruction relationship, and a visible keyboard focus treatment; every observed fitting table has none of those redundant interactions.
- **SC-004**: Trusted keyboard input moves an overflowing table horizontally while page horizontal position remains unchanged.
- **SC-005**: Resizing and true 200 percent page zoom produce state matching current geometry with every table column reachable and zero page-level horizontal overflow.
- **SC-006**: Print and blocked-script observations preserve complete semantic tables, omit inappropriate interactive chrome, and keep overflow contained without remote resources.
- **SC-007**: Documentation policy, generated browser evidence, mdBook build and test, accessibility checks, link validation, UTF-8, LF, mojibake, text hygiene, and repository parity gates complete with zero failures.

## Assumptions

- mdBook's generated local table boundary remains the authoritative wrapper and may be progressively enhanced without changing source table semantics.
- The current English documentation headings and table headers provide sufficient visible context for unique programmatic names when combined with table order.
- Dense tables should retain comparison-friendly rows and columns inside a discoverable scroll boundary rather than be converted into visually separate cards that risk weakening native table relationships.
- Compact tables should continue to wrap naturally and should not acquire forced minimum widths merely for visual uniformity.
- The existing local documentation browser seam can provide deterministic geometry, keyboard, zoom, print, and blocked-script evidence without downloading a browser or accessing a live game.

## Out of Scope

- Rewriting table content for editorial brevity except where an exact, meaning-preserving adjustment is required to create natural wrapping opportunities.
- Replacing semantic tables with cards, definition lists, grids, or duplicated mobile-only content.
- Adding a framework, package dependency, remote asset, downloaded browser, or visual-regression service.
- Changing application UI, runtime behavior, release packages, game data, or gameplay automation.
