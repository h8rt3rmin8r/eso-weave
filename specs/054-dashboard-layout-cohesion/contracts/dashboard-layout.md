# Contract: Dashboard Layout Cohesion

## C1. Effective arrangement

`effective_dashboard_layout(width, expanded)` is Stacked whenever `expanded` is
false. When true, it is Paired exactly for finite `width >= 880.0`; otherwise it
is Stacked. Live HUD is first in visual and accessibility order.

## C2. Expanded card geometry

Expanded outer rectangles have equal width and height within one rendered point.
Paired width is `(available_width - gap) / 2`, without an independent Live HUD
cap. Stacked cards each use full available width. Live HUD determines the shared
expanded body minimum; System and State may use natural height only when collapsed.

## C3. Transition safety

Collapse and expansion are responsive transitions for content-height purposes.
No transition frame may place the log over dashboard or Skills content. Pending
measurements clear after actual content extent is known and cannot permanently
increase the intrinsic minimum.

## C4. Dashboard rows

Each row allocates label, flexible value, and optional interaction regions from
its actual available width. The value allocation ends at the card trailing inset
when there is no interaction, or at the interaction gap when controls exist.
Content width measurement remains intrinsic and independent of the current
window-sized value allocation.

## C5. Complete value access

Dynamic value labels are single-line. Text truncates only when its galley is
wider than the allocated value region. Full text remains in the accessibility
name and is shown on both hover and keyboard focus when constrained.

## C6. Interaction column

Every toggle and lifecycle group starts at one shared trailing origin. Install,
Update, and Uninstall use identical rectangles. When two are shown they share a
horizontal row in primary then Uninstall order. Existing intents, enablement,
managed-marker guard, and uninstall confirmation remain unchanged.

## C7. Resource group boundary

One fixed vertical gap follows the final resource meter and precedes Game
Context. The rule is independent of resource count and is verified with three
production descriptors and four synthetic descriptors.

## C8. Label policy

The seven required replacements are exact. An explicit registry covers other
field and settings labels and enforces headline-style title case. Status values,
descriptions, tooltips, actions, and prose are outside that registry.

## C9. Stable downstream behavior

Skills source order, settings persistence, input synthesis, PixelBeacon protocol,
resource semantics, addon lifecycle safety, modal sizing, minimum-window sizing,
and log containment retain their existing contracts.
