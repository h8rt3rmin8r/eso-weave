# Feature Specification: Documentation Diagram Legibility

**Feature Branch**: `codex/s103-documentation-diagram-legibility`

**Created**: 2026-09-15

**Status**: Implemented

**Input**: Work slice S103 implements GitHub issue #168.

## User Scenarios & Testing

### User Story 1 - Follow every flow without guessing (Priority: P1)

A reader can trace each branch, merge, and successive stage in all four documentation flow diagrams without mistaking one connector, label, source, or destination for another.

**Why this priority**: The diagrams currently render, but cramped routes and labels impair the comprehension they were added to provide.

**Independent Test**: Inspect each reconstructed SVG and run element-level layout observations that identify nodes, edges, and edge labels, then prove stage spacing, route clearance, and unambiguous topology.

**Acceptance Scenarios**:

1. **Given** any diagram edge, **when** its route is followed, **then** its declared source and destination are visually and structurally unambiguous.
2. **Given** any branch label, **when** it is read, **then** it is associated with exactly one edge and clears nodes, other labels, bends, and arrowheads.
3. **Given** successive logical stages, **when** the diagram is viewed, **then** deliberate vertical space separates the stages and their routing lanes.

### User Story 2 - Read diagrams on every maintained surface (Priority: P1)

A reader can use each diagram in normal and expanded states, in navy and light themes, and at narrow or wide viewport widths without clipping, distortion, unreadably small labels, or page overflow.

**Why this priority**: A clearer source layout is only valuable if the established Pages and bundled documentation surfaces preserve it.

**Independent Test**: Build the generated manual and run the existing 32-cell browser matrix over all four updated intrinsic geometries.

**Acceptance Scenarios**:

1. **Given** a 320 or 1280 CSS-pixel viewport, **when** a diagram is shown normally or expanded, **then** it remains proportionate, contained, nonblank, and readable.
2. **Given** navy or light documentation themes, **when** the same diagram is shown, **then** text, state distinctions, and direction remain understandable without relying on color alone.

### User Story 3 - Catch renewed crowding before publication (Priority: P2)

A maintainer receives a deterministic failure when a future diagram change compresses stages, routes an edge through an unrelated node, creates an undeclared crossing, or detaches a label from its edge.

**Why this priority**: Paint and aspect-ratio checks cannot distinguish a technically visible diagram from a confusing one.

**Independent Test**: Mutate one valid layout observation at a time and confirm the layout receipt rejects lost metadata, insufficient gaps, node incursions, crossings, label collisions, and arrowhead crowding.

**Acceptance Scenarios**:

1. **Given** an edge that enters an unrelated node clearance zone, **when** layout validation runs, **then** it fails with the edge and node identities.
2. **Given** a label that collides with a node, another label, a bend, or an arrowhead, **when** layout validation runs, **then** it fails with the label and edge identities.
3. **Given** all four valid layouts, **when** the browser smoke runs, **then** one complete four-diagram layout receipt passes alongside the 32-cell rendering receipt.

### Edge Cases

- Two edges may terminate at the same node, but they must use distinct lanes and terminal points rather than an implicit shared segment.
- An edge may touch only its declared source and destination nodes; clearance checks exclude those two nodes and no others.
- A label may sit beside its edge, but it must not overlap the edge, a bend, or its terminal arrowhead.
- Narrow responsive rendering may scale a diagram down, but intrinsic label sizes remain at least 14 SVG units and the expanded view provides full-size inspection.
- Complete adjacent prose equivalents remain authoritative when a reader cannot use the visual.
- Browser font metrics can vary slightly, so geometry thresholds include a small deterministic tolerance without accepting overlaps.

## Requirements

### Functional Requirements

- **FR-001**: The work MUST reconstruct `architecture-ownership.svg`, `action-authorization.svg`, `safety-recovery.svg`, and `pixel-bus-validation.svg` rather than uniformly stretching their current layouts.
- **FR-002**: Every diagram MUST retain a top-down progression, explicit intrinsic geometry, an opaque local canvas, meaningful title and description, and its existing adjacent prose equivalent.
- **FR-003**: Each logical node, edge, and edge label MUST expose stable local metadata sufficient to identify topology and measure layout without inferring meaning from paint color.
- **FR-004**: Successive stage node bounds MUST have at least 36 SVG units of vertical separation, excluding the title region and final standalone note.
- **FR-005**: Every edge MUST declare one existing source node and one existing destination node, start and end at those node boundaries, and remain at least 10 SVG units from every unrelated node.
- **FR-006**: Distinct edges MUST NOT cross or share a segment unless a deliberate junction element declares that topology. S103 diagrams MUST require no shared junction.
- **FR-007**: Every visible branch outcome MUST have one edge label associated with exactly one edge.
- **FR-008**: Each edge label MUST clear every node and other edge label, remain 4 to 24 SVG units from its edge, and remain at least 12 SVG units from the edge's bends and terminal arrowhead.
- **FR-009**: Node contents MUST retain comfortable padding, visible text MUST remain at least 14 SVG units, and no label may be clipped by the canvas.
- **FR-010**: State and branch meaning MUST use visible words and shapes in addition to color.
- **FR-011**: The generated documentation MUST retain source-byte identity, local-only SVG delivery, accessible alternatives, intrinsic aspect ratio, and the shared accessible expansion interaction.
- **FR-012**: The dependency-free browser smoke MUST collect one element-level layout observation for each of the four direct generated SVG assets and reject incomplete or ambiguous layout receipts.
- **FR-013**: The existing 32-cell rendering matrix MUST continue to pass across four diagrams, two themes, two viewport widths, and normal plus expanded states.
- **FR-014**: Focused mutation tests MUST prove that the pre-S103 compressed characteristics and each defined clearance failure are rejected.
- **FR-015**: The maintained compatibility record MUST distinguish S103 comprehension evidence from the S086 rendering and S088 interaction contracts.
- **FR-016**: S103 MUST add no remote renderer, runtime diagram library, package dependency, application behavior, addon behavior, or second figure viewer.
- **FR-017**: Build-plan chronology MUST archive completed Plan 043 and establish S103 in a new active plan.

### Key Entities

- **Diagram node**: A named logical state or owner with intrinsic rectangular bounds and a stage number.
- **Diagram edge**: A named orthogonal connector with declared source and destination node identities.
- **Edge label**: Visible branch text linked to one edge identity.
- **Layout observation**: Browser-measured topology, bounds, clearances, and collision results for one direct generated SVG.
- **Layout receipt**: The complete four-diagram collection with its pass sentinel and actionable failures.

## Success Criteria

### Measurable Outcomes

- **SC-001**: All four diagrams pass the element-level layout receipt with at least 36 units between stages and at least 10 units between every edge and unrelated node.
- **SC-002**: Every declared edge has one valid source and destination, zero undeclared crossings or shared segments, and every branch edge has exactly one unambiguous label.
- **SC-003**: Every edge label clears nodes and peer labels, stays within the defined association distance, and clears bends and arrowheads.
- **SC-004**: All 32 established normal and expanded rendering observations pass at 320 and 1280 CSS pixels in navy and light themes.
- **SC-005**: Focused tests reject missing topology metadata, compressed stages, node incursions, edge crossings, and label collisions.
- **SC-006**: Documentation policy, browser smoke, mdBook test/build/link checks, spelling, UTF-8, mojibake, and hosted CI pass.

## Assumptions

- The four diagrams' semantic content and adjacent prose equivalents remain correct; S103 changes composition and test evidence, not documented runtime behavior.
- Orthogonal connector paths are sufficient for all four current flows and make clearance evidence deterministic.
- Direct generated SVG inspection in the existing host Chrome process is the most faithful dependency-free source of element bounds.
- S103 keeps issue #168 atomic because the four assets share one layout language and one regression contract.

## Out of Scope

- New diagrams, new documentation topics, or the corpus-wide visualization audit in issue #170.
- Changes to the figure dialog, responsive table system, syntax highlighting, or screenshot inventory.
- Pixel-golden screenshots tied to one browser rasterizer.
- Application, addon, input, automation, telemetry, packaging, or release behavior.
