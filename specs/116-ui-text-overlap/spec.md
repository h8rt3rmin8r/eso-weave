# Feature Specification: Collision-Safe Status Rows

**Feature Branch**: `codex/s116-ui-text-overlap`

**Created**: 2026-09-17

**Status**: Implemented

**Input**: Work slice S116 implements GitHub issue #231 after installed v0.17.0 evidence showed status-row labels overlapping their status dots and values.

## Clarifications

### Session 2026-09-17

- Q: Does S116 replace the reserved S115 documentation slice? -> A: No. S115 remains reserved for issue #222. S116 is an urgent P1 interruption because the defect is present in the published v0.17.0 MSI.
- Q: Which surfaces are in scope? -> A: The shared dashboard metric row, every System and State row, and every ESO Weave Data Details row. Other widgets are covered only if they use the same primitive.
- Q: How should narrow layouts behave? -> A: The title column uses measured font metrics and a bounded shared width. When space is insufficient, the visible title truncates inside a clipped cell while its complete accessible name and tooltip remain available.
- Q: What proves that overlap cannot silently return? -> A: Component geometry and paint-bound tests enumerate the shipping row titles, and full-surface tests cover both affected surfaces across themes, widths, addon states, and enlarged logical text.
- Q: Does S116 add a screenshot dependency or CI workflow? -> A: No. The existing Rust test suite receives the reusable collision oracle, so required CI runs it through `cargo test --all --locked` without changing pinned workflow files.

## User Scenarios & Testing

### User Story 1 - Read every status without collision (Priority: P1)

A user reads System and State or opens ESO Weave Data Details and sees each row title, status marker, value, and action as separate content with no text painted over neighboring content.

**Why this priority**: The published UI is visually corrupted and several important lifecycle states are harder to read.

**Independent Test**: Render the longest shipping titles in the shared row at supported widths and assert that label, value, and interaction regions are ordered, disjoint, clipped, and contained.

**Acceptance Scenarios**:

1. **Given** System and State is expanded, **when** `PixelBeacon Status`, `ESO Weave Data`, or `Data Addon Next Step` renders, **then** the title does not intersect the status marker, value, or action controls.
2. **Given** ESO Weave Data Details is open, **when** ownership, compatibility, encounter, or remediation rows render, **then** each title remains separate from its marker and value.
3. **Given** the available row width is constrained, **when** the complete title cannot fit, **then** its visible paint truncates inside the title cell and the complete title remains exposed to assistive technology and tooltip users.

### User Story 2 - Retain stable aligned status columns (Priority: P1)

A user scans a group of related statuses and sees their markers and values start at a shared horizontal origin rather than shifting independently with every title.

**Why this priority**: Per-row expansion could remove overlap while replacing it with a ragged and harder-to-scan dashboard.

**Independent Test**: Render each surface group and verify that all row label cells use one measured group width at a given font style and available width.

**Acceptance Scenarios**:

1. **Given** a group contains short and long titles, **when** it renders at ordinary width, **then** every row uses the measured width of the longest title in that group.
2. **Given** a row reserves trailing actions, **when** the group width is bounded, **then** a minimum readable value region remains and the actions stay contained.
3. **Given** logical body text is enlarged, **when** the group is measured again, **then** the column adapts from current font metrics without a hand-tuned replacement constant.

### User Story 3 - Block future text collisions in CI (Priority: P1)

A maintainer who adds or changes a status title receives an actionable test failure if visible text can escape its cell or overlap another semantic region.

**Why this priority**: Existing presence and viewport tests passed while the published text was visibly overlapping.

**Independent Test**: Prove the new oracle fails against the pre-fix row contract, then run the complete app UI sizing suite with the corrected implementation.

**Acceptance Scenarios**:

1. **Given** a title galley exceeds its allocation, **when** the component oracle runs, **then** it reports the title and intersecting region.
2. **Given** every shipping dashboard title, **when** default and enlarged logical text tests run, **then** all visible text paint is contained and pairwise non-overlapping within the tested row.
3. **Given** either theme and each maintained width/state fixture, **when** the full surfaces render, **then** their semantic title, value, and control rectangles remain disjoint.

### Edge Cases

- At the wide-dashboard breakpoint, System and State shares the window with Live HUD while data-addon actions reserve 212 points. The title column must yield before the value and action regions collapse.
- The modal may be drawn above an already painted dashboard. A global pairwise scan of flattened paint output would misclassify obscured background text, so the oracle must scope itself to a row or explicit semantic container.
- Enlarging display pixels per point is not the same as enlarging logical text. The regression test must enlarge the body text style itself.
- Truncated titles must not lose their complete AccessKit label or hover tooltip,
  and must not become new keyboard focus stops.
- A zero-width value region is not an acceptable way to avoid overlap. The layout contract reserves a documented minimum value width.

## Requirements

### Functional Requirements

- **FR-001**: S116 MUST replace the fixed 118-point dashboard label contract with label widths derived from current font metrics for the active surface group.
- **FR-002**: Rows in the same surface group MUST share one label-column width so their status markers and values remain aligned.
- **FR-003**: The measured label width MUST be bounded by the row width, horizontal gaps, trailing interaction reserve, and a non-zero minimum value width.
- **FR-004**: The title, value, and interaction child UIs MUST each clip paint to their exact allocation.
- **FR-005**: A title that does not fit MUST truncate within its clipped allocation, retain its complete accessible name plus hover tooltip text, and preserve the existing keyboard focus sequence.
- **FR-006**: The status value MUST retain its complete accessible value and tooltip behavior.
- **FR-007**: `DashboardRowGeometry` MUST expose the row, label, value, and optional interaction allocations for reusable contract tests.
- **FR-008**: A reusable test helper MUST inspect semantic node and painted-text bounds within one explicit row/container and reject intersections greater than a documented floating-point tolerance.
- **FR-009**: The component suite MUST enumerate every shipping dashboard status title and fail if the title inventory or layout contract drifts.
- **FR-010**: Full-surface tests MUST cover expanded System and State plus ESO Weave Data Details at the enforced minimum, narrow, wide-breakpoint, 900-point, and 1200-point widths where applicable.
- **FR-011**: Tests MUST cover installed, missing, outdated, incompatible, and remediation-required addon states, both maintained themes, and default plus enlarged logical body text.
- **FR-012**: Tests MUST keep exact full accessible labels while verifying visible truncation and paint containment.
- **FR-013**: The new collision tests MUST run through the existing required Rust test command without a CI workflow or dependency change.
- **FR-014**: S116 MUST preserve row order, status roles/colors, action availability, keyboard focus behavior, tooltips, dashboard breakpoints, and modal close behavior.
- **FR-015**: S116 MUST update the `[Unreleased]` changelog and MUST NOT create a duplicate project tracker or modify release identity.
- **FR-016**: S116 MUST publish through an official pull request that closes #231. Remote publication remains subject to the required pre-push authorization halt.

### Key Entities

- **Status-row group**: A set of rows rendered on one surface that share a measured label-column width.
- **Row allocation**: The exact outer, label, value, and optional interaction rectangles assigned to one row.
- **Visible text bounds**: The painted galley rectangle after applying the cell clip rectangle.
- **Collision**: An intersection wider and taller than the documented 0.5-point tolerance between semantic text/control regions that are not intentional overlays.
- **Shipping title inventory**: The maintained list of titles rendered by the shared dashboard row primitive and exercised by the collision suite.

## Success Criteria

### Measurable Outcomes

- **SC-001**: The reported System and State and ESO Weave Data Details layouts contain zero title-to-marker, title-to-value, title-to-action, or neighboring-row text intersections.
- **SC-002**: Every tested row allocation is ordered, disjoint, and contained at all maintained widths and text styles.
- **SC-003**: All shipping status titles pass the reusable paint-containment oracle in both themes at default and enlarged logical text.
- **SC-004**: Full accessible labels remain exact for 100 percent of truncated titles.
- **SC-005**: The focused UI suite, fmt, clippy, complete locked test suite, release build, diff check, encoding check, and forbidden-dash check pass.
- **SC-006**: The fixing pull request closes #231 and supplies before/after Windows evidence without claiming installed verification before a fixed release exists.

## Assumptions

- The shared row primitive is the correct single enforcement point for both reported surfaces.
- The existing egui and egui_kittest versions expose font layout, semantic rectangles, and painted text shapes needed for deterministic tests.
- Existing required CI already runs all Rust integration tests on Windows and Linux.

## Out of Scope

- Redesigning the dashboard, changing status meaning, changing lifecycle actions, or altering addon/runtime behavior.
- Global OCR or screenshot-only comparison as the correctness gate.
- Changing CI workflow files, adding a rendering dependency, cutting a release, or closing installed verification issue #230.
