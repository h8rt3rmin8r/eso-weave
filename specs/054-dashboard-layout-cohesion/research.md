# Research: Dashboard Layout Cohesion

## R1. Make collapse part of layout selection

**Decision**: Select the effective layout from available point width and the
System and State expansion flag. Collapsed always means stacked; expanded uses
the existing 880-point boundary.

**Rationale**: This directly models the interaction instead of relying on a
wide layout to notice a much shorter child after placement.

**Rejected**: Keeping a collapsed card beside Live HUD at wide widths.

## R2. Use equal width and one expanded height authority

**Decision**: Divide usable wide width equally and use Live HUD's rendered body
height as the minimum body height for expanded System and State. The same rule
applies while stacked.

**Rationale**: Live HUD is currently the taller stable card and future Ultimate
work naturally grows it. A shared minimum aligns frame exteriors without
stretching row spacing or performing a duplicate interactive render pass.

**Rejected**: The 46 percent split with a 380 to 520-point cap, independent
natural heights, vertical justification, or an invisible sizing pass.

## R3. Treat collapse as a responsive-height transition

**Decision**: Extend the existing transition protection so a collapse-driven
wide-to-stacked change defers the log until one actual content measurement is
available, then clears the pending state.

**Rationale**: Reusing the prior expanded-card height as a projected collapsed
delta can overestimate and permanently ratchet the window. Ignoring the state
transition can allow one-frame log overlap.

**Rejected**: Blindly applying the width-transition estimate to collapse.

## R4. Allocate rows explicitly

**Decision**: Replace dashboard Grids and the fixed 230-point value cell with a
shared row allocator: stable label width, flexible value width, and an optional
fixed trailing interaction width.

**Rationale**: A row owns the actual card width and can reserve only what its
controls need. The value then consumes all remaining space without advertising
an expanding intrinsic minimum.

**Rejected**: Increasing the fixed value width or right-justifying each control
independently. Both preserve drift and premature truncation.

## R5. Size the interaction column for its maximum supported state

**Decision**: Reserve two equal compact button widths plus one gap. Every toggle
or button group starts at that column's leading edge; a two-button group is
horizontal in primary then secondary order.

**Rationale**: At most two lifecycle actions are visible. One stable maximum
prevents state changes from shifting values or controls.

**Rejected**: Per-state column width, vertical button stacks, or moving toggles
to the far edge beyond button origins.

## R6. Preserve complete text through progressive detail

**Decision**: Let dynamic values use the flexible allocation. When the text does
not fit, retain single-line truncation plus full text on pointer hover, keyboard
focus, and the accessibility node.

**Rationale**: This keeps rows compact without discarding information or making
pointer use mandatory.

**Rejected**: A fixed state width, multi-line wrapping, or hover-only detail.

## R7. Put spacing on the resource group boundary

**Decision**: Render the meters through a count-agnostic group helper and add one
gap after the group scope.

**Rationale**: Spacing belongs to the relationship between groups, not to the
current final item. A synthetic fourth descriptor proves future compatibility
without inventing Ultimate telemetry.

**Rejected**: Adding margin after Magicka or adding an Ultimate placeholder.

## R8. Audit labels with an explicit registry

**Decision**: Maintain an explicit field-label registry and apply headline-style
title case to field and settings labels only. Preserve ESO, HUD, PTS,
PixelBeacon, AddOns, and units; leave short articles, conjunctions, and
prepositions lowercase.

**Rationale**: The existing all-copy registry includes state sentences,
tooltips, toasts, and actions that must remain ordinary prose.

**Rejected**: Runtime or mechanical conversion of every visible string.

## Sources

- GitHub issues #72 through #75 and their attached current-version screenshot
- S046 responsive dashboard specification and rendered-frame contracts
- egui 0.36.1 local `Ui`, `Frame`, `Grid`, and `Label` implementation
- Existing `egui_kittest` dashboard, sizing, focus, and log regressions
