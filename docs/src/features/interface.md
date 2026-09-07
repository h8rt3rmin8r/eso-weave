# Live Interface

ESO Weave uses one resizable window with a menu bar, responsive dashboard, Skills
region, and optional live log panel.

| Region | Contents |
| --- | --- |
| Menu bar | Settings, Exit, and Live Log |
| Live HUD | Health, Stamina, Magicka, Ultimate, Game Context, Combat, Movement, Roll Dodge, Life State, Weapon Bar, Quickslot, Potion Availability, and Potion Cooldown |
| System and State | Game/provider state, World State, ESO Weave state, PixelBeacon Status and Signal, Fishing, Auto Potion, and lifecycle actions |
| Skills | Slot label, enablement, weave type, timing override, effective delay, and cooldown |
| Live Log | Recent structured events with a local level filter |

## Responsive dashboard

Expanded Live HUD and System and State cards always have equal outer width and
height. They split available width evenly at 880 or more logical points and stack
at narrower widths. Collapsing System and State forces top-down stacking at every
width and is the only card-height exception. Expanding it restores width-driven
layout immediately. Skills always remains below the dashboard.

The window defaults to 600 by 720 logical points. Its minimum is not permanently
fixed. A 480 by 420 boot floor applies only until content has been laid out and
measured over two consecutive stable frames. The enforced minimum then uses the
intrinsic content width and the active responsive layout height, plus panel
padding.

The intrinsic minimum width never follows an expanding dashboard container, so a
continuous drag can cross the responsive breakpoint without ratcheting the
minimum width upward. The minimum follows content down when a row disappears,
grows the window when content no longer fits without shrinking an operator-chosen
size, and is capped at the display work area. With Live Log open, the minimum adds
the required width bonus and open-log reserve. Width-driven and collapse-driven
transitions reserve pending content height before the log renders, preventing
overlap.

Dashboard rows reserve stable label and interaction regions. Dynamic values use
the remaining width, and full constrained text is available through pointer hover
and keyboard focus, with identical text for both access paths. A fixed trailing
region is reserved only for System and State interactions. Install, Update, and
Uninstall actions use equal compact sizes and share a right-aligned column with
toggles. At most two lifecycle actions are available together, and they render in
one primary-then-secondary horizontal row. The managed-marker uninstall guard and
confirmation remain unchanged.

System and State defaults expanded. Its full header is accessible by pointer,
keyboard, and assistive technology. Activating the header hides or restores the
complete body, persists the preference, and recomputes intrinsic height without
leaving blank space. Skills is never part of that disclosure.

Hovering an interactive control changes its color but never its size, so hover
cannot cause layout reflow.

When ESO is inactive, Live HUD values say **Game not active**. When ESO is active
but current telemetry is unavailable, they say **Signal unavailable**. Neither
condition is presented as numeric zero.

## Resource meters

Health, Stamina, and Magicka use red, green, and blue fill. They show exact
percentages and include unlabeled 25, 50, and 75 percent landmarks. A watched
resource says **Low** only at or below its configured Auto Potion threshold.

Every resource meter is unanimated and reuses one component. Each meter exposes
a visible name, its exact numeric reading, proportional fill, and a programmatic
progress value. Observed zero remains a numeric empty bar. Dormant and unavailable
states have no numeric value.

Ultimate uses purple, exact current and maximum points, and a threshold for the
ability on the active front or back bar. Green **Ready** text occupies a fixed
trailing region when current charge meets the exact active cost. Special bars and
unavailable costs hide the threshold and Ready state without changing layout.
Ultimate is display-only.

Text and meaningful graphical boundaries meet WCAG 2.2 AA contrast, and color is
never the only state cue. One compact group boundary follows Ultimate before Game
Context.

## Live log and settings

The live log uses an always-available in-memory ring buffer, colorizes events by
level, autoscrolls while at the bottom, and can be filtered independently of file
logging. Its panel is resizable between a six-line readable minimum and the space
above it, and never covers interactive controls. On every rendered frame,
including simultaneous splitter drag and window resize, the pane's top edge
remains at or below the central content's bottom edge. Dragged and restored
heights are clamped before display or persistence. When a window cannot fit both
controls and six log lines, controls take priority and the pane gives up its
readable floor.

The Settings modal grows sub-linearly with the window on both axes, so its
absolute size increases while it occupies a progressively smaller fraction of a
larger window. It never exceeds 1040 by 1120 logical points or 92 percent of the
window. Settings apply immediately and are persisted through a coalesced save
without a separate Save action.

Its rendered rectangle equals its computed extent. The room above the body is
measured from the laid-out heading, separator, and close row rather than assumed.
At maximum size, at least half of the body remains visible without scrolling.

Settings cover keybindings; global and per-slot delays; weapon-aware timing and
weapon presets; latency adaptation and `k`; fishing timing and interact binding;
Auto Potion watches, quickslot binding, and retry interval; pixel-bus block size,
tolerance, and sampling intervals; AddOns override and environment; logging;
theme; and always-on-top behavior.
