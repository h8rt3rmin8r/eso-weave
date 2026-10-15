# Live Interface

The Settings modal also owns the optional **Local API and MCP Server** toggle,
truthful lifecycle status, endpoint display, and credential-copy action. The
complete client contract is documented under [Local API and
MCP](../reference/local-api-and-mcp.md).

ESO Weave uses one resizable window with a menu bar, responsive dashboard, Skills
region, and optional application log panel.

| Region | Contents |
| --- | --- |
| Menu bar | Settings, Exit, Application Log, and the offline Documentation action |
| Live HUD | Health, Stamina, Magicka, Ultimate, Game Context, Combat, Movement, Roll Dodge, Life State, Weapon Bar, and one composed Quickslot classification, availability, and cooldown row |
| System and State | Game/provider state, World State, ESO Weave state, one PixelBeacon lifecycle and Signal, Fishing, and Auto Potion |
| Skills | Slot label, enablement, weave type, timing override, effective delay, and cooldown |
| Application Log | Recent structured events with a local level filter |

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
size, and is capped at the display work area. With Application Log open, the minimum adds
the required width bonus and open-log reserve. Width-driven and collapse-driven
transitions reserve pending content height before the log renders, preventing
overlap.

Dashboard rows reserve stable label and interaction regions. Dynamic values use
the remaining width, and full constrained text is available through pointer hover
and keyboard focus, with identical text for both access paths. A fixed trailing
region is reserved only for System and State interactions. PixelBeacon actions
use equal compact sizes and share a right-aligned column with toggles. The single PixelBeacon lifecycle manages the combined package with exact ownership, inventory and transactional recovery checks. Separate Data installation/remediation rows are retired. At most two lifecycle actions share a primary-then-secondary horizontal row. Uninstall retains its managed-ownership guard and confirmation.

Catalog maintenance runs automatically, independently from native encounter log
processing. File > Settings contains two direct controls: **Disable Catalog
maintenance** and **Disable native encounter log processing**. They retain
independent choices and are gray/inactive without a compatible combined addon.
Neither changes gameplay automation or ESO's native logger. Desktop disable stops
scheduling directly; loaded Lua observes its startup preference at the next
natural addon load. Data processing and supported calculations belong to
background services and continue without opening a window. The optional
authenticated read-only [Local API and MCP](../reference/local-api-and-mcp.md)
retains access to stored data. See [Automatic Native Processing](encounter-capture.md).

System and State defaults expanded. Its full header is accessible by pointer,
keyboard, and assistive technology. Activating the header hides or restores the
complete body, persists the preference, and recomputes intrinsic height without
leaving blank space. Skills is never part of that disclosure.

Use Tab and Shift+Tab to move among interactive controls. Activate buttons,
toggles, menu items, and the System and State disclosure with the keyboard action
offered by the platform and UI framework. Combo boxes expose their current text;
text fields retain visible labels and help. When a constrained dashboard value is
truncated, pointer hover and keyboard focus expose the same full text. This
keyboard focus indicator is a UI selection cue and is distinct from ESO window
focus, which is a safety prerequisite for generated input.

Hovering an interactive control changes its color but never its size, so hover
cannot cause layout reflow.

## Offline documentation

Choose **Help > Documentation** to open the complete bundled guide in the
operating system browser. ESO Weave starts one local address on `127.0.0.1`,
reuses it for later opens, and stops it when the application exits. Navigation,
search, themes, fonts, and images are included in the executable, so the guide
does not require internet access. If the browser cannot be opened, the interface
shows an error and continues running.

The local service is read-only. It serves only immutable documentation compiled
into the executable and exposes no settings, logs, game state, filesystem paths,
directory listings, or mutation routes.

When ESO is inactive, Live HUD values say **Game not active**. When ESO is active
but current telemetry is unavailable, they say **Signal unavailable**. Neither
condition is presented as numeric zero.

By default, the last coherent Live HUD and player-state values remain visible
for 120 seconds after ESO becomes inactive, runtime or focus evidence becomes
unavailable, focus is lost, or the live signal disappears. The application logs
the freshness transition without inserting a temporary row above the meters.
Fresh coherent observations
replace the snapshot immediately. At expiry, the ordinary **Game not active**
or **Signal unavailable** presentation returns. **Stale Retention (seconds)** in
Settings accepts 0 through 999; 0 clears immediately. Retention is display-only:
weaving, Fishing, Auto Potion, and every synthesized-input gate react to the
underlying loss immediately.

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

The meter name, numeric or non-numeric text, proportional fill, programmatic
progress value, quarter landmarks, Ultimate cost threshold, and stable Ready text
are complementary cues. A reader never needs to infer state from red, green,
blue, purple, or dot color alone.

## Live log and settings

The application log uses an always-available in-memory ring buffer, colorizes events by
level, and autoscrolls while at the bottom. Its level selector currently updates
and persists the global captured level used by both the ring and optional file
sink. Its panel is resizable between a six-line readable minimum and the space
above it, and never covers interactive controls. On every rendered frame,
including simultaneous splitter drag and window resize, the pane's top edge
remains at or below the central content's bottom edge. Dragged and restored
heights are clamped before display or persistence. When a window cannot fit both
controls and six log lines, controls take priority and the pane gives up its
readable floor.

The Settings modal grows sub-linearly with the window on both axes, so its
absolute size increases while it occupies a progressively smaller fraction of a
larger window. It never exceeds 1040 by 1120 logical points or 92 percent of the
window. Settings are persisted through a coalesced save without a separate Save
action. Fishing controls and scalar Pixel Bus reader controls apply to their
running subsystems. PixelBeacon Block Size remains staged: managed redeploy writes
the package, Lua uses it at the next natural addon load and the reader at its next start; see the
[Settings Reference](../reference/settings.md#application-timing).

Its rendered rectangle equals its computed extent. The room above the body is
measured from the laid-out heading, separator, and close row rather than assumed.
At maximum size, at least half of the body remains visible without scrolling.

Settings cover keybindings; global and per-slot delays; weapon-aware timing and
weapon presets; latency adaptation and `k`; Fishing timing and detected Interact;
Auto Potion watches, detected Quickslot, and retry interval; pixel-bus block size,
tolerance, and sampling intervals; AddOns override and environment; logging;
theme; and always-on-top behavior.
Appearance also includes the bounded Stale Retention interval for the Live HUD.

Open **File > Settings** to edit configuration, **View > Application Log** to diagnose
events, and **File > Exit** to close after pending geometry is flushed. The
[Settings Reference](../reference/settings.md) lists every control. The
[Status Reference](../reference/status-reference.md) explains every field.
