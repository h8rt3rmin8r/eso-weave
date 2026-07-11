# Research: PixelBeacon Addon

Phase 0 decisions. No open NEEDS CLARIFICATION items remain.

## Block rendering

**Decision**: Create three backdrop controls under a top-level window anchored to
the top-left of `GuiRoot`. Each block sets its center color from its state. Blocks
use the game's standard control lifecycle so they are hidden automatically during
loading screens (FR-005).

**Rationale**: Standard controls give the loading-screen hide behavior for free
and keep the shim minimal (section 9.1). A backdrop with a solid center color is
the simplest solid block.

**Alternatives considered**: A custom texture per block (rejected: heavier than a
solid backdrop for a solid color). Drawing to a scene fragment (rejected:
unnecessary complexity).

## Physical-pixel geometry

**Decision**: Compute UI-space dimensions as 16 divided by `GetUIGlobalScale()`
and re-derive them when the scale changes, so each block stays 16 by 16 physical
pixels regardless of UI scale (FR-001).

**Rationale**: The reader samples physical pixels; constant physical geometry is
required by section 9.3.

## Colors and latency encoding

**Decision**: Status is magenta `#FF00FF`; fishing waiting is `#0080FF` and bite
is `#00FF00`; the latency block encodes red = clamp(latency, 0, 1020) / 4, green =
`0xA5`, blue = 255 minus red, from `GetLatency()`, updated at 1 Hz via a
registered update and only while the status block renders (FR-002 to FR-004).
Colors are converted to the 0 to 1 range the API expects.

**Rationale**: These are the exact section 9.3 contract values the future reader
depends on.

## Bite detection

**Decision**: Register `EVENT_INVENTORY_SINGLE_SLOT_UPDATE` and treat a minus-one
stack change on the equipped bait, while a fishing interaction is active, as a
bite; gate "interaction active" via `EVENT_CLIENT_INTERACT_RESULT` and the
camera-interaction state; clear on a new item gained, `EVENT_CHATTER_END`, or a
safety timeout; suppress while any menu is open (FR-006 to FR-009).

**Rationale**: This is the bait-consumption mechanism from established ESO addon
art that the specification adopts, including the menu-open suppression for the
known false-positive class.

**Alternatives considered**: Screen or animation heuristics (rejected: the
specification fixes the bait-consumption approach for v1).

## Manifest

**Decision**: A minimal `PixelBeacon.txt` with the title, author, a declared
`## APIVersion`, an addon version, the managed marker line
`## X-ESO-Weave-Managed: true`, and the single `PixelBeacon.lua` file entry. No
saved variables, no dependencies, no libraries (FR-010 to FR-012).

**Rationale**: The smallest valid descriptor plus the marker the later Beacon
Manager verifies before any uninstall. The declared API version is subject to the
later upkeep item R4.
