# Feature Specification: Documentation Diagram Rendering Compatibility

**Feature Branch**: `codex/s086-diagram-rendering-compatibility`

**Created**: 2026-09-11

**Status**: Implemented

**Input**: Issue #154 and the approved S086 work-slice proposal.

## User Scenarios & Testing

### User Story 1 - See every diagram reliably (Priority: P1)

A reader can open each of the four documentation flow diagrams on GitHub Pages or in bundled offline documentation and see a nonblank, correctly proportioned diagram.

**Why this priority**: A blank diagram removes the visual explanation that S082 intentionally added to four high-value pages.

**Independent Test**: Build the documentation and exercise all four diagram pages through a representative browser at narrow and wide viewports in navy and light themes. Each normal image decodes, paints meaningful content, remains contained, and reports the expected intrinsic ratio.

**Acceptance Scenarios**:

1. **Given** any supported diagram page, **When** it loads in either theme and viewport class, **Then** the primary diagram is nonblank, legible, proportionate, and contained.
2. **Given** either Pages or bundled documentation, **When** a diagram asset is requested, **Then** the exact local SVG is returned successfully as `image/svg+xml` without a remote dependency.

---

### User Story 2 - Expand without distortion (Priority: P1)

A reader can use mdBook's existing expanded-image state for a diagram without stretching, clipping, or losing the diagram.

**Why this priority**: The current broad diagram rule stretches the generated expanded clone to the viewport and breaks the image's aspect ratio.

**Independent Test**: Toggle each generated diagram's current zoom control at both viewports and verify that the expanded clone is visible, nonblank, proportionate, and inside the viewport.

**Acceptance Scenarios**:

1. **Given** a rendered diagram, **When** its existing expansion control is activated, **Then** the clone preserves the source aspect ratio and fits inside the viewport.
2. **Given** the generated zoom wrapper, **When** accessibility is inspected, **Then** the primary image remains meaningfully named and the duplicate clone does not add a second announcement.

---

### User Story 3 - Catch rendering regressions before publication (Priority: P2)

A maintainer receives a deterministic failure when diagram geometry, generated delivery, media type, zoom markup, or visible paint regresses.

**Why this priority**: Source-string checks alone cannot prove that a browser decoded and painted the generated asset.

**Independent Test**: Run focused mutation tests and the generated-site browser smoke. Known blank, solid, malformed, missing, distorted, remote, and wrong-media-type cases must fail.

**Acceptance Scenarios**:

1. **Given** a malformed or visually empty diagram fixture, **When** policy runs, **Then** it fails with a specific rendering-contract error.
2. **Given** the generated manual, **When** the browser smoke runs, **Then** all matrix observations pass and one machine-readable success receipt is emitted.

### Edge Cases

- A valid `viewBox` without explicit intrinsic dimensions must fail because some renderers infer an unsuitable fallback size.
- A generated SVG that exists but differs from its checked-in source must fail.
- A decoded image that is fully transparent or effectively one solid color must fail the paint observation.
- A primary image may fill its responsive wrapper, but the hidden expanded clone must not inherit that width rule.
- A missing compatible Chrome executable fails the required smoke with an actionable override instruction.
- Failure to reproduce the original blank report does not block the work. The finite supported matrix and hardened contract are the resolution evidence.

## Requirements

### Functional Requirements

- **FR-001**: The system MUST retain exactly the four S082 diagram records and their existing page placements, meaningful alternatives, and adjacent text equivalents.
- **FR-002**: Every diagram root MUST declare positive intrinsic width and height values equal to its `viewBox` dimensions.
- **FR-003**: Every diagram MUST retain an opaque canvas plus visible labels, connectors, fills, and non-background paint.
- **FR-004**: Generated diagram assets MUST be byte-identical to their checked-in source assets.
- **FR-005**: Pages and bundled delivery MUST return every diagram successfully as `image/svg+xml` and MUST require no CDN, font download, script renderer, or other network dependency.
- **FR-006**: Generated diagram markup MUST retain mdBook's local normal-image and expanded-image structure with the expected asset path.
- **FR-007**: Responsive CSS MUST allow only the primary diagram image to fill its wrapper and MUST preserve the expanded clone's intrinsic aspect ratio and viewport containment.
- **FR-008**: The primary image MUST retain a meaningful accessible name and the generated duplicate clone MUST be decorative to avoid duplicate announcement, while the adjacent text equivalent remains complete.
- **FR-009**: A dependency-free browser smoke MUST exercise all four diagrams in navy and light themes at 320 and 1280 CSS pixels, in normal and expanded states.
- **FR-010**: Browser observations MUST verify successful decode, positive geometry, expected aspect ratio, containment, high opaque coverage, multiple colors, and meaningful non-background paint.
- **FR-011**: Policy mutations MUST reject missing or mismatched intrinsic dimensions, blank or solid paint, generated byte drift, missing zoom DOM, unsafe remote dependencies, and wrong media types.
- **FR-012**: The work MUST preserve identical public and bundled documentation sources and MUST change no application, addon, gameplay, input, or telemetry behavior.
- **FR-013**: The work MUST add no npm dependency graph, browser download, Mermaid runtime, alternate raster format, or page-time renderer.
- **FR-014**: The maintained compatibility record MUST identify the tested surfaces, engines, themes, viewports, states, response evidence, and any explicitly untested boundary.

### Key Entities

- **Diagram Geometry Contract**: The asset path, intrinsic width and height, `viewBox`, opaque canvas, and required visible anchors for one diagram.
- **Generated Figure Contract**: The primary image, expansion control, decorative clone, local source identity, and scoped sizing rules emitted by mdBook.
- **Compatibility Observation**: One diagram, surface, theme, viewport, state, response, geometry, and paint result.
- **Rendering Receipt**: The aggregate pass or actionable failure emitted by the browser smoke.

## Success Criteria

### Measurable Outcomes

- **SC-001**: All 4 generated figures pass 32 browser matrix cells across 2 themes, 2 viewport widths, and normal plus expanded states. Separate delivery evidence covers Pages and release embedding.
- **SC-002**: All 8 surface-and-asset delivery pairs return success with `image/svg+xml`, and all generated and embedded asset bytes match source.
- **SC-003**: Every browser observation reports positive dimensions, the correct aspect ratio, viewport containment, at least 95 percent opaque pixels, at least four opaque colors, and meaningful deviation from the dominant background.
- **SC-004**: Every named malformed mutation fails focused policy or browser-observation validation.
- **SC-005**: Documentation, generated-site, accessibility, text-hygiene, and repository merge gates pass with zero remote rendering dependencies.

## Assumptions

- GitHub's hosted Ubuntu runner continues to provide Chrome; local runs may select Chrome or Edge through an explicit environment variable.
- Semantic geometry and canvas observations are more stable than pixel-golden screenshots across browser updates.
- The four existing static SVG designs and text equivalents are correct; this slice hardens delivery and rendering rather than redrawing content.
- Issue #155 owns general click-to-expand behavior and cross-corpus interaction design. S086 only fixes the four current diagram figures.

## Out of Scope

- General screenshot, brand-asset, or raw-HTML figure expansion behavior from issue #155.
- New expansion affordances, focus management redesign, captions, syntax highlighting, responsive tables, or diagram content redesign.
- Mermaid, PNG fallbacks, remote assets, runtime rendering, or application behavior changes.
