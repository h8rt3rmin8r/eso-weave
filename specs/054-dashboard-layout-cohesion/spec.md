# Feature Specification: Dashboard Layout Cohesion

**Feature Branch**: `codex/s054-dashboard-layout-cohesion`

**Created**: 2026-09-07

**Status**: Implemented

**Input**: GitHub issues #72, #73, #74, and #75, combined as S054

## User Scenarios & Testing

### User Story 1 - Keep the dashboard visually paired (Priority: P1)

As an operator, I want the expanded Live HUD and System and State cards to move
and size as a pair so the dashboard remains balanced at every supported width.

**Why this priority**: The current unequal growth and height make one connected
dashboard look like unrelated panels and waste available space.

**Independent Test**: Render expanded cards below, at, and above the breakpoint,
resize repeatedly, and verify equal outer width and height in every settled frame.

**Acceptance Scenarios**:

1. **Given** both cards are expanded, **When** the window is narrow or wide,
   **Then** their outer widths and heights are equal.
2. **Given** a wide window grows, **When** the frame settles, **Then** both cards
   gain the same width.
3. **Given** System and State is collapsed at any width, **When** the frame
   renders, **Then** the dashboard stacks with Live HUD first and only the
   collapsed card may be shorter.
4. **Given** System and State is expanded again, **When** the frame renders,
   **Then** the ordinary 880-point breakpoint immediately governs layout again.

---

### User Story 2 - Find every system action in one stable column (Priority: P1)

As an operator, I want System and State controls aligned in one trailing column
so switches and lifecycle actions are predictable and compact.

**Why this priority**: Misaligned switches and stacked unequal buttons add visual
noise and make closely related controls look structurally unrelated.

**Independent Test**: Render absent, outdated, and current addon states and
compare control origins, button sizes, grouping, and interaction results.

**Acceptance Scenarios**:

1. **Given** any System and State row, **When** an interaction is present,
   **Then** its group starts at the same trailing-column origin.
2. **Given** two lifecycle buttons are present, **When** the row renders,
   **Then** the equal-sized buttons appear in one horizontal row without overlap.
3. **Given** only one lifecycle button is present, **When** the row renders,
   **Then** it retains the same size and origin used in the two-button state.
4. **Given** a control is activated, **When** its intent is handled, **Then** the
   existing toggle, install, update, uninstall, and confirmation behavior remains.

---

### User Story 3 - Read complete dashboard values when space exists (Priority: P2)

As an operator, I want dashboard state text to use the available card width and
resource meters to remain visually grouped so useful information is not hidden.

**Why this priority**: Fixed-width value cells currently truncate content even
when a card has unused room, and the facts start too close to the meter group.

**Independent Test**: Render long values across narrow and wide cards, with
three and a synthetic fourth meter, and inspect allocation, detail access, and
the single group boundary gap.

**Acceptance Scenarios**:

1. **Given** unused horizontal room, **When** a long value renders, **Then** its
   allocation extends to the same trailing inset used by the card's leading inset.
2. **Given** a value truly cannot fit, **When** it truncates, **Then** the full
   value remains available by pointer hover and keyboard focus.
3. **Given** three or four resource meters, **When** Game Context follows them,
   **Then** exactly one small gap appears after the final meter.

---

### User Story 4 - Scan concise, consistent field labels (Priority: P2)

As an operator, I want field and settings labels in consistent title case so the
interface is easier to scan and avoids needlessly long names.

**Why this priority**: Mixed casing and several verbose labels weaken hierarchy
and consume width needed by live values.

**Independent Test**: Inspect the explicit field-label registry and rendered
dashboard, Skills, and Settings surfaces for required replacements and absence
of superseded labels.

**Acceptance Scenarios**:

1. **Given** the main dashboard, **When** labels render, **Then** `Weapon Bar`,
   `Life State`, `Roll Dodge`, `World State`, `Auto Potion`,
   `PixelBeacon Status`, and `PixelBeacon Signal` use those exact forms.
2. **Given** any audited application field or settings label, **When** it
   renders, **Then** major words are capitalized while short articles,
   conjunctions, and prepositions remain lowercase and product names are kept.
3. **Given** status sentences, tooltips, or prose, **When** copy renders, **Then**
   it is not mechanically converted to title case.

### Edge Cases

- Width is just below, exactly at, or well above 880 points.
- System and State starts collapsed from persisted settings.
- Collapse or expansion happens before or after a resize and while the log is open.
- Addon state exposes Install, Update plus Uninstall, Uninstall only, or no action.
- Dynamic state text is short, exactly fits, or exceeds the remaining value width.
- UI scale changes while the dashboard crosses its point-based breakpoint.
- A future resource group contains four meters without changing group spacing logic.

## Requirements

### Functional Requirements

- **FR-001**: Expanded Live HUD and System and State cards MUST have equal outer
  width and equal outer height in both stacked and side-by-side arrangements.
- **FR-002**: Side-by-side expanded cards MUST split usable width equally and
  grow by equal amounts as the window grows.
- **FR-003**: A collapsed System and State card MUST force the dashboard into
  top-down order at every width and MAY be shorter only while collapsed.
- **FR-004**: Expanding System and State MUST immediately restore the layout
  selected by the existing 880-point breakpoint.
- **FR-005**: Collapse and resize transitions MUST preserve Skills ordering,
  log non-overlap, intrinsic minimum sizing, and anti-ratchet behavior.
- **FR-006**: System and State MUST reserve one fixed trailing interaction
  column whose leading edge is shared by toggles and button groups.
- **FR-007**: Install, Update, and Uninstall buttons MUST use one size; any two
  visible lifecycle buttons MUST share one horizontal row.
- **FR-008**: The interaction column MUST fit two lifecycle buttons and their
  gap without clipping while preserving existing action availability and safety.
- **FR-009**: Dashboard rows MUST use a stable label region, flexible value
  region, and optional fixed interaction region rather than a fixed value width.
- **FR-010**: A flexible Live HUD value MUST extend to the card's trailing inset
  and truncate only when its full text would exceed that allocation.
- **FR-011**: Truncated dynamic values MUST expose complete text on pointer hover
  and keyboard focus and retain complete accessible names.
- **FR-012**: Exactly one small vertical gap MUST separate the complete resource
  meter group from Game Context for three meters and future four-meter rendering.
- **FR-013**: S054 MUST NOT add Ultimate telemetry, a placeholder Ultimate row,
  protocol fields, or skill-cost logic.
- **FR-014**: Required dashboard labels MUST use the exact concise replacements
  listed in User Story 4.
- **FR-015**: Audited field and settings labels MUST follow the documented title
  case rule without changing status sentences, tooltips, actions, or prose.
- **FR-016**: Toggle controls MUST have meaningful accessible names while their
  visible state text remains available independently of color.
- **FR-017**: Rendered-frame tests MUST cover settled geometry, symmetric growth,
  collapse timing, lifecycle controls, long values, resource grouping, scaling,
  Skills containment, and log containment.
- **FR-018**: Master specification, README, changelog, and build-plan records
  MUST describe the revised dashboard behavior and label vocabulary.

### Key Entities

- **EffectiveDashboardLayout**: Collapse-aware stacked or width-driven paired
  arrangement selected from available point width and expansion state.
- **ExpandedCardGeometry**: Shared expanded width and Live-HUD-authoritative
  height used by both dashboard cards.
- **DashboardRowGeometry**: Stable label, flexible value, and optional fixed
  interaction allocations inside a card.
- **InteractionGroup**: One or two equal-sized controls sharing a stable origin.
- **FieldLabelRegistry**: Explicit inventory of field and settings labels subject
  to the title-case rule.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Expanded card width and height differ by no more than one rendered
  point at every tested width and scale.
- **SC-002**: Increasing a wide dashboard by 400 points changes each card width
  by the same amount within one rendered point.
- **SC-003**: Every collapsed fixture is stacked and every re-expanded fixture
  returns to the expected side-by-side or stacked mode on its first settled frame.
- **SC-004**: All toggle and lifecycle groups share one trailing origin within
  one rendered point, all lifecycle buttons have equal dimensions, and two-button
  states are horizontal and non-overlapping.
- **SC-005**: Long values receive all flexible row width up to symmetric card
  padding, with full detail available whenever visible truncation occurs.
- **SC-006**: Three- and four-meter fixtures produce one equal resource-to-context
  gap after the final meter.
- **SC-007**: The explicit label registry contains every audited field label and
  none of the seven superseded forms.
- **SC-008**: Full CI parity and existing input, addon-removal, Skills, sizing,
  persistence, accessibility, and log regression suites pass.

## Assumptions

- Egui point width remains the correct responsive input at all display scales.
- Live HUD is the expanded height authority because it is the stable telemetry
  stack and will naturally grow when Ultimate is implemented in a later slice.
- A fixed trailing control allocation is appropriate because no supported addon
  state exposes more than two lifecycle buttons at once.
- Issue #71 remains a separate slice because it requires new game telemetry and
  bar-aware Ultimate-cost data that S054 does not own.

## Explicit Design Deviation

S046 allowed the two expanded dashboard cards to size independently and used a
46 percent Live HUD column capped at 520 points. Issue #72 supersedes both
choices. S054 uses equal column growth and one shared expanded-card height, while
retaining S046's 880-point breakpoint whenever System and State is expanded.
