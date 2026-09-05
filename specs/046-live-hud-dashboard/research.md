# Research: Responsive Live HUD Dashboard

## R1. Organize by operator question

**Decision**: Split the region into Live HUD (`What is happening in game?`) and
System and automation (`Can ESO Weave act, and why not?`).

**Rationale**: Visibility of system status requires timely, intelligible state,
while progressive disclosure favors primary task information before diagnostic
detail. Game observations have a different cadence and purpose from setup and
automation controls, so one flat grid obscures both.

**Rejected**: One card per row. It adds decoration without hierarchy and makes
related high-cadence signals slower to scan.

## R2. One explicit responsive breakpoint

**Decision**: Select layout from egui point width at a single 880 point
breakpoint. At or above it, render two equal columns. Below it, stack Live HUD
before System and automation.

**Rationale**: Points already account for platform scaling. One boundary is
predictable and easy to test. Two approximately 420 point columns fit the
longest stable field and resource meter without truncation.

**Rejected**: Automatic wrapping from arbitrary child widths. It makes reading
order and minimum-size behavior difficult to reason about and test.

## R3. Replace the obsolete height invariant

**Decision**: Preserve a window-independent intrinsic minimum width, but allow
dashboard height to change across the responsive breakpoint. Test narrow and
wide heights plus the transition directly.

**Rationale**: Stacking necessarily increases height. Pretending height is
window-independent would either block responsiveness or hide content. Minimum
width must still ignore expanding column containers so resize ratcheting cannot
return.

**Rejected**: Raising minimum width to the wide layout. That would prevent the
narrow layout from ever being reached through normal resizing.

## R4. Use a custom semantic meter over the stock progress bar

**Decision**: Implement one small custom `ResourceMeter` renderer backed by a
typed `ResourcePresentation`.

**Rationale**: The stock widget clamps numeric progress and assumes every state
is a number. S046 needs dormant and unavailable non-numeric states, explicit
state borders, stable text positioning, and full accessibility metadata. A
bounded custom widget can provide those without changing telemetry types.

**Rejected**: Three copied `ProgressBar` calls. Copies would drift and could not
express the non-numeric contract consistently.

## R5. Derive Low only from user configuration

**Decision**: Mark a resource Low only when its corresponding auto-potion watch
is enabled and the percentage is at or below that watch's threshold.

**Rationale**: A universal low value would invent meaning. The controller already
owns validated thresholds, so the view can project them without duplicating
trigger behavior.

**Rejected**: A hard-coded 20 or 25 percent warning threshold.

## R6. Separate addon installation from live signal

**Decision**: Keep the existing disk-derived beacon installation line and add a
runtime/freshness-derived PixelBeacon signal line.

**Rationale**: Installed, Fresh, Never observed, Lost, and Game not active are
different facts. Combining them can present a healthy green addon while no
telemetry is available.

**Rejected**: Infer signal health from installation status or Game Context.
Both collapse independent evidence.

## R7. Make state redundant to color

**Decision**: Retain semantic hues, and also render exact text, fill geometry,
and state-specific labels. Status lines include a glyph plus readable text.

**Rationale**: WCAG 2.2 Use of Color requires another visual means, such as text
or shape. Non-text Contrast requires meaningful graphics and component states
to reach at least 3:1 against adjacent colors. Meter labels and values also map
to `WidgetInfo::ProgressIndicator` with a numeric value only when observed.

**Rejected**: Hue-only Health, Stamina, and Magicka bars or hue-only warning
states.

## R8. Make only the next addon action primary

**Decision**: Show gold Install when absent or unresolved, gold Update when
outdated, and no gold lifecycle action when current. Show Uninstall as a neutral
secondary action only when managed removal is available, preserving confirmation.

**Rationale**: The primary action should answer the current setup need. Showing
Install, Update, and Uninstall with equal prominence creates conflicting calls
to action.

**Rejected**: Keep all three buttons visible in every state.

## R9. Preserve Skills as a sealed boundary

**Decision**: Stop the replacement immediately before the existing Skills
heading and grid.

**Rationale**: #28 explicitly excludes Skills redesign, and its behavior already
has mature persistence, cooldown, and sizing coverage.

**Rejected**: Move skill cooldowns into the Live HUD or restyle the grid in the
same slice.

## Sources

- W3C, WCAG 2.2 Understanding SC 1.4.1, Use of Color
- W3C, WCAG 2.2 Understanding SC 1.4.11, Non-text Contrast
- Nielsen Norman Group, Visibility of System Status
- Nielsen Norman Group, Progressive Disclosure
- egui 0.36.1 local source, `ProgressBar` and `WidgetInfo`
