# Feature Specification: Documentation Syntax Highlighting

**Feature Branch**: `codex/s087-docs-syntax-highlighting`

**Created**: 2026-09-11

**Status**: Implemented

**Input**: Issue #157 and the approved S087 work-slice proposal.

## User Scenarios & Testing

### User Story 1 - Read commands by syntax role (Priority: P1)

A reader can distinguish an executable, its options, values, variables, and punctuation in command examples without prompts or other characters that would make a copied command invalid.

**Why this priority**: The current Catalog Compiler examples render as one color, so the language metadata promises highlighting that the shipped grammar does not deliver.

**Independent Test**: Build the manual and inspect the real Catalog Compiler and Installation pages in every supported theme. Command and PowerShell blocks retain their exact text while exposing multiple meaningful token classes with AA contrast.

**Acceptance Scenarios**:

1. **Given** a prompt-free command block, **When** the generated page loads, **Then** its executable, options, values, and punctuation are visibly distinguishable where present.
2. **Given** a PowerShell block, **When** the generated page loads, **Then** cmdlets, switches, variables, strings, numbers, comments, and punctuation are highlighted where present.
3. **Given** a reader who copies a block, **When** the selected text is read, **Then** it exactly matches the source command without an injected prompt or altered whitespace.

---

### User Story 2 - Keep deliberate plain examples honest (Priority: P1)

A reader sees formulas, directory maps, decision flows, and ESO slash-command lists as readable plain text rather than falsely highlighted executable syntax.

**Why this priority**: Applying a language grammar to non-code content can misrepresent meaning and weakens documentation trust.

**Independent Test**: Validate the complete source inventory against an explicit path, content digest, and rationale allowlist. Each approved plain block remains un-tokenized and any new or changed plain block fails for review.

**Acceptance Scenarios**:

1. **Given** one of the eight approved plain blocks, **When** the manual is built, **Then** it remains semantic, readable, selectable plain text with no syntax token spans.
2. **Given** an unlabeled, unknown, or unapproved plain fence, **When** policy runs, **Then** the build fails with its page and source line.

---

### User Story 3 - Preserve offline, accessible rendering (Priority: P2)

A Pages or bundled-documentation reader receives the same local highlighting behavior, AA token contrast, code semantics, and contained horizontal overflow with no network dependency.

**Why this priority**: Highlighting is useful only if it remains legible and does not compromise offline packaging, narrow layouts, or assistive access.

**Independent Test**: Exercise shell command, PowerShell, JSON probe, and plain-text cases at 320 and 1280 CSS pixels in navy, light, coal, ayu, and rust, then validate identical checked-in and generated runtime assets through the release manifest.

**Acceptance Scenarios**:

1. **Given** any supported theme, **When** meaningful tokens render, **Then** each observed token has at least 4.5:1 contrast against its code background.
2. **Given** a long block at narrow width, **When** it renders, **Then** the block scrolls horizontally without creating page-level overflow and its text remains selectable.
3. **Given** Pages or bundled output, **When** highlighting loads, **Then** the same local mdBook runtime and project extension are used with no remote resource.

### Edge Cases

- mdBook 0.5.4 recognizes `console`, but its Shell Session grammar requires prompts and produces no spans for the current prompt-free commands.
- The bundled Bash grammar also produces no spans for simple argv-only commands, so metadata correction alone cannot deliver the requested visual distinction.
- Additional JavaScript loads after `book.js` has attempted highlighting, so the local extension must safely and idempotently re-highlight only its owned language classes.
- Nested fences inside Markdown lists count as real blocks and must appear in the inventory.
- Plain-block content drift invalidates its digest even when its page and line remain unchanged.
- Generated HTML contains language classes before runtime highlighting; token-span assertions must run in a browser after scripts execute.
- A structured JSON probe validates the bundled grammar without adding a contrived example to reader documentation.

## Requirements

### Functional Requirements

- **FR-001**: The system MUST inventory all 23 current documentation fences, including nested list fences, with an explicit language classification.
- **FR-002**: The 12 prompt-free executable blocks MUST use the accurate `bash` source identifier and MUST be enhanced by a bounded local command grammar because mdBook's bundled Bash grammar does not tokenize their argv-only form.
- **FR-003**: The three PowerShell blocks MUST retain the accurate `powershell` identifier and MUST receive a bounded local grammar registered into the existing Highlight.js runtime.
- **FR-004**: The eight deliberate `text` blocks MUST be allowlisted by page, exact content digest, and substantive rationale.
- **FR-005**: Source policy MUST reject missing identifiers, unknown identifiers, obsolete `console` and `sh` aliases, and any unallowlisted or changed plain block.
- **FR-006**: The command grammar MUST distinguish executable, option, value, number, variable, string, and punctuation roles where those forms are present.
- **FR-007**: The PowerShell grammar MUST distinguish cmdlet, switch, variable, keyword, string, number, comment, and punctuation roles where those forms are present.
- **FR-008**: The local extension MUST reuse mdBook's one bundled Highlight.js runtime, MUST add no second highlighter or package dependency, and MUST run idempotently after `book.js`.
- **FR-009**: Runtime highlighting MUST preserve exact block text, `pre > code` semantics, selection, whitespace, and code-local horizontal scrolling.
- **FR-010**: Project-owned theme overrides MUST provide at least 4.5:1 contrast for every meaningful observed token class in navy, light, coal, ayu, and rust.
- **FR-011**: A browser smoke MUST cover real Bash, PowerShell, and plain blocks plus a bundled JSON grammar probe at 320 and 1280 CSS pixels across all five supported themes.
- **FR-012**: Generated and browser policy MUST reject missing or incorrect language classes, missing local highlight assets, absent runtime registration, missing or unexpected spans, text mutation, low contrast, lost selection, code overflow failure, or page-level overflow.
- **FR-013**: GitHub Pages and release-bundled documentation MUST use the same generated bytes and MUST require no network resource.
- **FR-014**: The work MUST change no application, addon, gameplay, input, telemetry, catalog, or encounter behavior.
- **FR-015**: Issue #127 MUST remain the owner of general responsive table and page-overflow work; S087 MUST only prevent code-block highlighting from creating page-level overflow.

### Key Entities

- **Fence Record**: Source page, opening line, identifier, exact content, normalized digest, classification, and rationale.
- **Plain Exception**: One exact page and content digest authorized to remain un-tokenized, with a human-readable reason.
- **Highlight Extension**: The project-local command and PowerShell grammars plus the idempotent re-highlighting boundary over mdBook's bundled runtime.
- **Syntax Observation**: Language case, theme, viewport, semantics, token roles, text identity, selection, overflow, contrast, and result.
- **Rendering Receipt**: The complete browser evidence matrix, failures, and pass sentinel.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Source policy accounts for exactly 23 fences: 12 Bash, 3 PowerShell, and 8 allowlisted plain-text blocks.
- **SC-002**: Every meaningful browser case exposes at least the required language-specific token roles, while every plain case exposes zero syntax-token spans.
- **SC-003**: All 40 syntax matrix cells across 4 cases, 5 themes, and 2 viewport widths preserve exact text, semantics, selection, code-local overflow, and no page-level overflow.
- **SC-004**: Every observed meaningful token color has a computed contrast ratio of at least 4.5:1 against its code background.
- **SC-005**: Shell, PowerShell, JSON, and plain-text positive cases pass, and all named source, runtime, accessibility, contrast, and delivery mutations fail.
- **SC-006**: Documentation build, policy, browser smoke, accessibility, UTF-8, mojibake, text-hygiene, and repository gates pass with zero remote highlighting dependencies.

## Assumptions

- mdBook remains pinned to 0.5.4 for this slice, with Highlight.js 10.1.1 as its bundled runtime.
- The eight current plain examples are intentionally non-executable and should remain visually neutral.
- A runtime JSON probe is stronger and less misleading than adding a reader-facing configuration fragment solely for test coverage.
- Host-provided Chrome or Edge remains the representative browser seam already used by documentation CI.

## Out of Scope

- Replacing or upgrading mdBook or Highlight.js.
- Adding Prism, Shiki, a package graph, a browser download, or another highlighter runtime.
- Adding shell prompts to copy-ready commands.
- General table responsiveness, general page overflow remediation, code-copy UI redesign, line numbers, or runnable examples.
