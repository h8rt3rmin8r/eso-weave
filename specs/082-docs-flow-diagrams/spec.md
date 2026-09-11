# Feature Specification: Documentation Flow Diagrams

**Feature Branch**: `codex/s082-docs-flow-diagrams`

**Created**: 2026-09-11

**Status**: Implemented

**Input**: Work slice S082 implements issue #123 with a small set of purposeful top-down diagrams for high-value architecture, authorization, state, and Pixel Bus flows.

## Clarifications

### Session 2026-09-11

- Four relationships warrant diagrams: input and observation ownership, generated-action authorization, coherent safety recovery, and Pixel Bus frame validation.
- Checked-in static SVG is the delivery format. Each SVG is also its editable source, so no Mermaid-derived asset exists and no Mermaid source or renderer is required.
- Every diagram uses an opaque ink surface, restrained brand tokens, meaningful visible labels, an internal title and description, Markdown alternative text, and a complete adjacent text equivalent.
- Diagrams supplement rather than replace canonical prose and tables. Release, configuration, troubleshooting, and per-feature controller flows remain text because their existing ordered tables or checklists are more precise.
- Narrow layouts scale a deliberately compact top-down canvas without creating page-level horizontal overflow. The adjacent text equivalent remains fully readable when image text is magnified or unavailable.

## User Scenarios & Testing

### User Story 1 - Understand ownership and authorization (Priority: P1)

A reviewer can follow where physical input and observed game evidence travel, where they converge, and why unavailable evidence cannot authorize generated input.

**Why this priority**: Ownership and authorization are the highest-risk architectural relationships and are difficult to reconstruct from separate prose sequences.

**Independent Test**: Open Architecture and Action Authorization in either documentation delivery and verify that each page has one top-down diagram plus a complete adjacent ordered explanation.

**Acceptance Scenarios**:

1. **Given** the Architecture page, **When** a reviewer follows the ownership diagram, **Then** physical input and observed evidence remain distinct until named engine or controller boundaries consume them.
2. **Given** the Action Authorization page, **When** a required gate is unavailable or unsafe, **Then** the diagram leads to pass-through, cancellation, or no generated input rather than an authorized action.

---

### User Story 2 - Follow safety recovery and frame validation (Priority: P1)

A maintainer can understand how unsafe observations close gates before recovery and how a Pixel Bus frame establishes a valid layout before its payload blocks are independently decoded.

**Why this priority**: The ordering is correctness-bearing and a top-down visual makes closure, synchronization, and coherent reopening explicit.

**Independent Test**: Open State Machines and Pixel Bus Protocol and verify that the diagrams expose every fail-closed boundary named in the adjacent text.

**Acceptance Scenarios**:

1. **Given** safety evidence becomes invalid, **When** the recovery flow is followed, **Then** gates close before synchronization and reopen only after a complete positive baseline.
2. **Given** a Pixel Bus header is missing or corrupt, **When** the validation flow is followed, **Then** payload routing is suppressed and cached observations are invalidated or lost as appropriate.

---

### User Story 3 - Read diagrams in every delivery mode (Priority: P2)

A reader can use the diagrams on narrow or wide screens, in light or dark documentation themes, and in the bundled offline manual without network access or color perception.

**Why this priority**: A diagram that depends on a remote renderer, one theme, or color alone would make the documentation less reliable than the prose it supplements.

**Independent Test**: Build the documentation, disconnect network access, and inspect all four pages at 320 and 1280 CSS pixels in both themes while checking source and generated accessibility semantics.

**Acceptance Scenarios**:

1. **Given** either public or bundled output, **When** a diagram loads, **Then** it resolves from a local generated asset with no script or remote request.
2. **Given** an image is unavailable or a reader cannot perceive its colors, **When** they use the page, **Then** the alternative and adjacent text communicate the same relationships.

### Edge Cases

- A missing, renamed, malformed, scripted, or externally linked SVG fails documentation policy.
- A diagram using left-to-right progression or color-only branch meaning fails policy.
- Long labels wrap inside bounded nodes rather than crossing edges or leaving the canvas.
- The opaque diagram surface remains legible in both documentation themes.
- Generated pages retain all four local SVG outputs and their meaningful alternatives.
- Prose remains authoritative if an image is blocked, omitted by a text-only reader, or heavily magnified.

## Requirements

### Functional Requirements

- **FR-001**: The published documentation MUST contain exactly four new purposeful diagrams covering architecture ownership, action authorization, safety recovery, and Pixel Bus frame validation.
- **FR-002**: Every diagram MUST use top-down logical progression and MUST NOT use a left-to-right flow direction.
- **FR-003**: Every diagram MUST clarify a convergence, transition, ownership boundary, or fail-closed decision that is harder to scan in prose alone.
- **FR-004**: Every diagram MUST be a checked-in local SVG whose file is its editable source.
- **FR-005**: SVGs MUST contain no script, remote URL, external resource, animation, or interactive behavior.
- **FR-006**: Every SVG MUST expose a unique internal title and description through `aria-labelledby` on a role-image root.
- **FR-007**: Every Markdown image MUST have meaningful alternative text naming the flow and its primary outcome.
- **FR-008**: Every diagram MUST have a complete adjacent text equivalent that names nodes, ordering, branches, and fail-closed outcomes without relying on color or position.
- **FR-009**: Diagram labels MUST use plain or expanded terminology, avoid unexplained abbreviations, and remain within their nodes without crossing edges.
- **FR-010**: Diagram styling MUST use an opaque, bounded surface and visible text, strokes, arrowheads, and status labels that work in light and dark documentation themes.
- **FR-011**: Diagrams MUST fit the content column at 320 and 1280 CSS pixels without page-level horizontal overflow.
- **FR-012**: Public GitHub Pages and bundled offline documentation MUST render the same local assets with no network dependency.
- **FR-013**: Documentation policy MUST reject missing diagram pages, assets, references, alternatives, text equivalents, top-down semantics, accessibility metadata, unsafe SVG content, and missing generated outputs.
- **FR-014**: The visual-content audit MUST record why release, configuration, troubleshooting, controller-cycle, and other candidates remain prose or tables.
- **FR-015**: Documentation build, link, policy, spelling, UTF-8, LF, forbidden-dash, and mojibake checks MUST pass.
- **FR-016**: S082 MUST change no application, addon, input, capture, packaging, or runtime behavior.

### Key Entities

- **Diagram record**: Stable identifier, destination page, local asset path, purpose, relationship type, alternative text, text-equivalent heading, and output path.
- **Static SVG diagram**: One editable local vector asset with a top-down canvas, accessible root metadata, bounded nodes, labeled connectors, and no active or remote content.
- **Text equivalent**: Adjacent prose that communicates the diagram's complete ordered relationships and failure outcomes without depending on the image.
- **Visual-content audit**: Selection record comparing candidate relationships and documenting why each is visualized or retained as text.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Exactly 4 selected pages each contain one local diagram, one meaningful alternative, and one complete adjacent text equivalent.
- **SC-002**: All 4 SVGs pass structural checks for top-down orientation, unique title and description identifiers, safe local-only content, minimum label size, and bounded view boxes.
- **SC-003**: Mutation tests detect every governed failure class, including missing references, weak alternatives, missing text equivalents, horizontal direction, unsafe SVG content, and missing generated assets.
- **SC-004**: All 4 generated pages and assets work at 320 and 1280 CSS pixels in light and dark themes without page-level horizontal overflow.
- **SC-005**: The built documentation makes zero page-time requests for diagram scripts, renderers, fonts, or remote assets.
- **SC-006**: The complete documentation and repository text-hygiene gates pass.

## Assumptions

- Existing mdBook image copying is the only build support required.
- SVG files are maintainable source documents and do not require a second generator format.
- Adjacent canonical prose may be reorganized for a complete text equivalent but retains its technical meaning.
- Verification-only issues remain non-blocking under current project governance.

## Out of Scope

- Mermaid, Graphviz, a JavaScript renderer, or a new documentation dependency.
- Decorative illustrations, screenshots, animation, or application UI assets.
- Diagrams for every state machine, feature, table, or sequential procedure.
- The screenshot sandbox and screenshot content assigned to issues #124 and #125.
- Global responsive-table work assigned to issue #127.
- Any runtime or gameplay behavior change.
