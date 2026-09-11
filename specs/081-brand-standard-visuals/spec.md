# Feature Specification: Brand Standard Visuals

**Feature Branch**: `codex/s081-brand-standard-visuals`

**Created**: 2026-09-10

**Status**: Implemented

**Input**: Work slice S081 implements issue #122 by publishing the approved brand assets, accessible palette swatches, responsive surface examples, and offline reproduction guidance in the Brand Standard.

## Clarifications

### Session 2026-09-10

- The approved visual set is the full-color banner PNG plus the badged mark and badge-less glyph SVG masters. Older clear and white raster logo variants are compatibility outputs, not masters.
- Documentation copies of approved assets must be byte-identical to their repository authorities so the public and bundled sites never publish an approximation.
- The banner and badged mark may appear on dark and light surfaces. The badge-less glyph is demonstrated only on ink because its gold strand loses contrast on light surfaces.
- Each palette chip exposes an accessible name containing the palette role and uppercase hexadecimal value. Visible role and hexadecimal text remain present, so color is never the only signal.
- Asset links point to local published copies. No network request, JavaScript, image regeneration, or new dependency is required.

## User Scenarios & Testing

### User Story 1 - Identify and use the approved assets (Priority: P1)

A reader can see the banner wordmark, badged mark, and badge-less glyph in context and understand which asset fits a particular surface.

**Why this priority**: A written brand description is incomplete when the authoritative visuals and their placement constraints are invisible.

**Independent Test**: Open the Brand Standard from either the public or bundled documentation and verify that all three named assets render from local files with clear intended-use guidance.

**Acceptance Scenarios**:

1. **Given** the Brand Standard is open on a wide viewport, **When** the reader reviews the approved assets, **Then** the banner, badged mark, and glyph appear with names, purposes, and dark or light surface context.
2. **Given** the reader is choosing an asset for a light surface, **When** they inspect the placement guidance, **Then** they are directed to the badged mark rather than the badge-less glyph.
3. **Given** older raster variants remain in the repository, **When** the reader reviews asset authority, **Then** those files are identified as generated compatibility outputs rather than approved masters.

---

### User Story 2 - Compare the palettes accessibly (Priority: P1)

A reader can visually compare every dark and light theme token while retaining the same information without color perception or image access.

**Why this priority**: Color chips make the palette useful as a visual standard, while role and hexadecimal labels preserve equivalent non-visual meaning.

**Independent Test**: Inspect both palette tables and confirm that every row has a bordered chip whose accessible name and visible text identify the same role and hexadecimal value.

**Acceptance Scenarios**:

1. **Given** either palette table, **When** a reader scans its rows, **Then** every hexadecimal token has an adjacent color chip with a visible boundary.
2. **Given** assistive technology reads a chip, **When** the chip receives an accessible name, **Then** that name includes the visible role and uppercase hexadecimal value.
3. **Given** a reader cannot perceive color, **When** they use the table, **Then** the visible role, hexadecimal value, and intended use still communicate the complete token definition.

---

### User Story 3 - Reproduce the identity offline (Priority: P2)

A contributor can select and download an authoritative asset and apply practical sizing and clear-space rules without leaving the documentation.

**Why this priority**: Reproduction guidance prevents accidental distortion and keeps offline documentation self-sufficient.

**Independent Test**: Follow every source link from a locally built book and verify that the asset opens locally, while the accompanying guidance covers clear space and minimum size.

**Acceptance Scenarios**:

1. **Given** a bundled offline documentation build, **When** the reader follows an approved-asset link, **Then** the corresponding local file opens without a network connection.
2. **Given** the Brand Standard is viewed at a narrow width, **When** asset examples reflow, **Then** images stay within their cards and no horizontal page overflow is introduced.

### Edge Cases

- Near-white, near-black, and panel swatches retain a visible outline in every documentation theme.
- Asset examples remain legible when the documentation theme differs from the surface being demonstrated.
- The transparent glyph does not appear on a light demonstration surface.
- A missing, altered, or renamed published asset fails documentation policy rather than silently degrading the page.
- Palette drift, a missing chip, or an accessible-name mismatch fails documentation policy.

## Requirements

### Functional Requirements

- **FR-001**: The Brand Standard MUST visibly present the approved full-color banner, badged mark, and badge-less glyph using local assets.
- **FR-002**: Each approved asset example MUST state its name, intended use, and permitted surface context.
- **FR-003**: The page MUST explicitly identify older clear and white raster logo variants as generated compatibility outputs rather than masters.
- **FR-004**: Published copies of the banner, mark, and glyph MUST be byte-identical to their authoritative repository files.
- **FR-005**: Every row in both current palette tables MUST display one chip adjacent to its hexadecimal token.
- **FR-006**: Every chip MUST have an accessible name containing its row's role and uppercase hexadecimal value.
- **FR-007**: Role, hexadecimal value, and use MUST remain visible text independent of each chip.
- **FR-008**: Every chip MUST have a theme-independent visible boundary, including near-white, near-black, and panel colors.
- **FR-009**: Asset examples MUST demonstrate valid dark and light placement, and MUST NOT demonstrate the badge-less glyph on a light surface.
- **FR-010**: The page MUST state clear-space, minimum-size, aspect-ratio, and no-recolor guidance.
- **FR-011**: Source or download links MUST resolve to the three local published assets in public and bundled documentation.
- **FR-012**: The presentation MUST reflow at narrow widths without horizontal page overflow or clipped content.
- **FR-013**: The feature MUST add no runtime network dependency, JavaScript behavior, build-time image generation, telemetry, or application-code change.
- **FR-014**: Automated policy MUST reject missing approved assets, altered asset bytes, incomplete palette rows, incorrect chip labels, unsafe surface examples, broken local output, and missing responsive styling.
- **FR-015**: Source and generated documentation MUST pass link, accessibility, UTF-8, LF, mojibake, narrow-layout, and wide-layout checks.

### Key Entities

- **Approved brand asset**: A named banner, mark, or glyph with one authoritative repository file, one byte-identical published copy, an intended use, and allowed surface contexts.
- **Surface example**: A visually bounded dark or light context containing one or more approved assets and an explicit surface label.
- **Palette token**: A theme, role, uppercase hexadecimal value, intended use, visible chip, and matching accessible chip name.
- **Compatibility output**: A generated raster retained for packaging or legacy use that is not an editable master and is not promoted as a primary reproduction source.

## Success Criteria

### Measurable Outcomes

- **SC-001**: All 3 approved assets render from local files and all 3 published copies match their authorities byte for byte.
- **SC-002**: All 25 palette rows (13 dark and 12 light) include exactly one labeled, bordered color chip beside the hexadecimal value.
- **SC-003**: Automated negative cases detect each governed failure class: altered assets, missing assets, palette drift, missing or incorrect labels, invalid light-surface glyph use, and absent narrow-layout behavior.
- **SC-004**: The generated Brand Standard has no broken local links and remains usable at 320 CSS pixels and a conventional desktop width.
- **SC-005**: Documentation policy, mdBook tests and build, text hygiene, and repository CI parity checks all pass.

## Assumptions

- Repository files under `assets/` remain the authorities; S081 does not redesign or regenerate them.
- The existing mdBook theme and raw HTML support can express the gallery and chips without JavaScript.
- S081 is documentation-only and changes no ESO integration, runtime behavior, packaging asset, or user data.
- Verification-only issues remain non-blocking under current project governance.

## Out of Scope

- Creating, redesigning, recoloring, or regenerating brand assets.
- Promoting legacy raster variants to master status.
- Global table responsiveness, which remains assigned to issue #127.
- Documentation diagrams or screenshot infrastructure assigned to issues #123 through #125.
- Application UI or packaging changes.
