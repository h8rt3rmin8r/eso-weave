# Feature Specification: Ultimate Resource Meter

**Feature Branch**: `codex/s055-ultimate-resource-meter`

**Created**: 2026-09-07

**Status**: Implemented

**Input**: GitHub issue #71, implemented as S055

## User Scenarios & Testing

### User Story 1 - Read exact Ultimate charge (Priority: P1)

As an operator, I want a fourth Live HUD meter for Ultimate so I can see the
player's exact stored charge and its proportion of the current in-game maximum.

**Independent Test**: Feed known, zero, full, unavailable, corrupt, and lost
Ultimate samples through the addon contract and companion, then verify the
ordered fourth meter and its accessible exact value.

**Acceptance Scenarios**:

1. **Given** current and maximum Ultimate are valid, **When** the HUD renders,
   **Then** the fourth meter shows `current/maximum` and fills by their ratio.
2. **Given** current is zero and maximum is valid, **When** the HUD renders,
   **Then** zero is shown as an observed value rather than unavailable.
3. **Given** maximum is zero, evidence is missing or corrupt, or signal is lost,
   **When** the HUD renders, **Then** Ultimate is explicitly unavailable.
4. **Given** either application theme, **When** Ultimate is available, **Then**
   its fill uses the specified theme-specific purple and remains distinguishable.

---

### User Story 2 - See the active bar's cast threshold (Priority: P1)

As an operator, I want the Ultimate meter to show the cost of the Ultimate
slotted on the active weapon bar so a bar swap immediately shows the relevant
threshold without maintaining a skill-ID catalogue.

**Independent Test**: Publish unequal front and back costs, swap the active bar,
and verify that the marker and Ready state select the matching cached cost.

**Acceptance Scenarios**:

1. **Given** unequal valid costs on both bars, **When** the active bar changes,
   **Then** the threshold moves to the matching cost without waiting for a cost
   resample.
2. **Given** the selected cost is valid, **When** current is below, equal to, or
   above it, **Then** Ready is absent below and present at or above the cost.
3. **Given** the active bar is unknown or its cost is unavailable, **When** the
   HUD renders, **Then** both the dynamic threshold and Ready are hidden.
4. **Given** only the inactive bar's cost changes, **When** it later becomes
   active, **Then** its latest cached threshold is used immediately.

---

### User Story 3 - Compare meter landmarks without clutter (Priority: P2)

As an operator, I want subtle shared quarter landmarks and an understated cast
threshold so I can judge resource levels while the dashboard geometry stays
stable.

**Independent Test**: Inspect pure meter geometry and rendered accessibility
across all four rows with overlapping quarter and cost thresholds.

**Acceptance Scenarios**:

1. **Given** any resource meter, **When** it renders, **Then** unlabeled subtle
   marks appear at 25, 50, and 75 percent inside the track.
2. **Given** a valid selected Ultimate cost, **When** it renders, **Then** a
   stronger thin mark begins inside the track and protrudes about three points
   below the bottom border.
3. **Given** a cost coincides with a quarter mark, **When** it renders, **Then**
   the cost mark remains visually distinguishable.
4. **Given** readiness changes, **When** Ready appears or disappears, **Then**
   it uses green in a permanently reserved slot to the right of the numbers and
   no track, number, card, or row geometry moves.

### Edge Cases

- Current, maximum, or either cost is zero, unavailable, corrupt, or partially sampled.
- Current exceeds maximum, or cost exceeds maximum.
- Front and back costs differ, match, or change while inactive.
- The active bar is unknown during loading, death, or a transient swap.
- A dynamic threshold exactly overlaps 25, 50, or 75 percent.
- Protocol versions 1 through 4 remain present during an application-first update.
- The fourth row appears below, at, and above the dashboard reflow point.
- Signal loss and recovery occur without any numeric value changing.

## Requirements

### Functional Requirements

- **FR-001**: PixelBeacon MUST read current and maximum Ultimate from
  `GetUnitPower("player", COMBAT_MECHANIC_FLAGS_ULTIMATE)` without hard-coding a cap.
- **FR-002**: PixelBeacon MUST read front and back Ultimate costs with
  `GetSlotAbilityCost(ACTION_BAR_ULTIMATE_SLOT_INDEX + 1,
  COMBAT_MECHANIC_FLAGS_ULTIMATE, hotbarCategory)` using the primary and backup
  hotbar categories rather than a skill-ID catalogue.
- **FR-003**: Current, maximum, front cost, and back cost MUST each be published
  as one exact 9-bit value. Red MUST carry the low byte, one of two unique green
  markers MUST carry the high bit, and blue MUST retain the complement checksum.
- **FR-004**: The 9-bit value `511` MUST represent unavailable. Values above 510
  MUST fail closed rather than clamp. Zero current
  MUST remain valid; zero maximum and zero cost MUST project as unavailable.
- **FR-005**: The protocol MUST advance to version 5 and 29 payload blocks while
  freezing version 4 at 25 blocks and leaving versions 1 through 3 unchanged.
- **FR-006**: PixelBeacon MUST refresh Ultimate on player power updates, relevant
  slot and bar changes, activation rebaseline, and a periodic backstop.
- **FR-006A**: An active hotbar other than primary or backup MUST project both
  Ultimate costs as unavailable so no stale cost threshold is selected, without
  changing the existing weapon-bar signal or any weaving behavior.
- **FR-007**: Player activation MUST publish a complete Ultimate baseline before
  the world state becomes Active.
- **FR-008**: The reader MUST emit current, maximum, and both costs as one typed
  atomic event while preserving independently unavailable fields.
- **FR-009**: A malformed or missing byte MUST invalidate only its complete
  two-byte value, except invalid or zero maximum MUST make the whole presented
  Ultimate observation unavailable.
- **FR-010**: Signal loss and unsupported protocol versions MUST clear Ultimate
  exactly once and recovery MUST republish it.
- **FR-011**: Ultimate MUST remain outside `ResourceSet` and every auto-potion,
  input, timing, and automation decision path.
- **FR-012**: Live HUD MUST render Health, Stamina, Magicka, then Ultimate.
- **FR-013**: Ultimate fill MUST use `#A78BFA` in dark mode and `#6D28D9` in
  light mode; Ready MUST use an accessible semantic success green.
- **FR-014**: All four meter tracks MUST show unlabeled 25, 50, and 75 percent marks.
- **FR-015**: A valid active-bar cost MUST render as a stronger thin threshold
  clamped to the track, starting inside it and protruding three points below it.
- **FR-016**: A cost above maximum MUST remain truthfully available in text while
  its visual threshold clamps at the track end and Ready remains value-based.
- **FR-017**: Ready MUST appear exactly when valid current is greater than or
  equal to the selected valid cost.
- **FR-018**: The numeric readout and a fixed Ready slot to its right MUST remain
  permanently reserved so readiness never causes reflow.
- **FR-019**: Accessible meter information MUST expose exact current, maximum,
  selected cost, active bar, and readiness when known, without relying on color.
- **FR-020**: The added row MUST preserve S054 equal expanded-card geometry,
  collapsed stacking, Skills containment, log containment, and resize stability.
- **FR-021**: Tests MUST cover codec integrity, protocol generations, lifecycle
  refresh, bar swaps, partial availability, signal loss, inertness, geometry,
  themes, accessibility, and unchanged automation behavior.
- **FR-022**: The master specification, README, changelog, build plan, addon
  manifest, and feature announcement MUST describe the version 5 contract.
- **FR-023**: Live-game release verification MUST be tracked separately from
  repository implementation according to project governance.

### Key Entities

- **UltimateTelemetry**: Atomic raw current, maximum, front cost, and back cost.
- **UltimateValue**: Available unsigned point value or explicitly unavailable.
- **UltimateView**: Display-only projection with exact readout, fill, selected
  threshold, readiness, and accessible description.
- **MeterGeometry**: Stable row, track, quarter marks, threshold, number slot,
  and Ready slot geometry.
- **ProtocolV5Layout**: Negotiated 29-block payload with B25 through B28 assigned
  to exact 9-bit current, maximum, front-cost, and back-cost values.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Every tested value from 0 through 510 round-trips exactly; any
  marker or checksum failure produces unavailable rather than a plausible value.
- **SC-002**: The displayed fill differs from `current / maximum` by no more than
  one rendered pixel across tested track widths.
- **SC-003**: Front and back swaps select the correct cached cost and Ready state
  on the first rendered view.
- **SC-004**: Quarter marks land at exact track fractions and the cost marker
  protrudes exactly three points without changing row height.
- **SC-005**: Ready transitions change no measured row, track, numeric, card, or
  neighboring control geometry.
- **SC-006**: Protocol versions 1 through 4 retain their frozen sample extents and
  report Ultimate unavailable.
- **SC-007**: Auto-potion and weave outputs are byte-for-byte unchanged for the
  same non-Ultimate inputs across all Ultimate fixtures.
- **SC-008**: Full local and hosted CI, accessibility, protocol, lifecycle, UI,
  and text-hygiene gates pass.

## Assumptions

- ESO Ultimate point values fit in 0 through 500; the wire supports 0 through
  510 and reserves 511 as unavailable.
- `HOTBAR_CATEGORY_PRIMARY` and `HOTBAR_CATEGORY_BACKUP` are available with the
  target ESO API and are the stable categories for front and back costs.
- A zero cost means no usable Ultimate cost is exposed for that bar.
- The existing success green remains the semantic Ready color; readiness is also
  exposed in text and accessibility data.

## Explicit Design Deviation

Issue #71 proposed three normalized percentage values. S055 instead publishes
four exact 9-bit values across four blocks. Percentage rounding cannot reconstruct an
exact `current/maximum` readout and can report false readiness at a cost boundary.
The larger contract preserves exact values, independent validation, and the
issue's requirement not to assume a universal Ultimate cap.
