# Research: Negotiated Width-Aware Pixel Geometry

**Feature**: `specs/045-negotiated-width-geometry/` | **Date**: 2026-09-04

## R1: Which side chooses the columns?

**Finding**: PixelBeacon must choose. It owns the drawn control positions and
can measure the root it draws into. If the companion derives a separate count,
rounding or lifecycle disagreement can shift payload identities while colors
still pass their individual markers.

**Decision**: The addon publishes one decision and the companion validates and
consumes it. The companion's measured display is a plausibility bound only.

**Deviation**: This supersedes slice 035's fixed-column decision. That decision
was correct under its dual-derivation premise. An invariant header removes the
premise rather than ignoring the risk.

## R2: How is the header located before the layout is known?

**Finding**: Header positions cannot themselves depend on the announced column
count. Placing them after a wrapping payload or at a measured right edge creates
the same bootstrap problem.

**Decision**: Reserve cells 0, 1, and 2 at the top-left. Negotiated payload index
`i` begins at cell `3 + i`. Three is also the minimum valid column count, keeping
the entire header on row zero.

**Alternative rejected**: Reusing payload channels would make independent
header validation depend on the semantics of changing game signals. A separate
header keeps geometry and payload authority distinct.

## R3: What bytes make corruption obvious?

**Decision**:

| Cell | Red | Green | Blue |
| --- | --- | --- | --- |
| H0 | `0x45` | `0x53` | version-1 wire code `0x20` |
| H1 | column high byte | `0x64` | `255 - high` |
| H2 | column low byte | `0x9C` | `255 - low` |

The magic bytes identify a new header. H1 and H2 each have a marker and an
independent complement checksum. The markers occupy well-separated gaps in the
existing green-channel registry and are asserted against it by tests.

All H0 channels use normal capture tolerance. Logical version 1 is represented
by spaced wire code `0x20`, rather than the literal byte 1, so compositor drift
cannot turn an adjacent future version into version 1. Future codes must remain
farther apart than twice the maximum supported tolerance. Geometry decoding caps
its effective tolerance at `0x0F`; a broader user payload tolerance cannot weaken
version discrimination.

**Decision**: Recognized magic plus any later failure is invalid, never legacy.
Only a non-magic cell zero that independently decodes as the legacy heartbeat
may select legacy mode.

## R4: Why a 16-bit column count?

**Finding**: One byte caps the grid at 255 columns. At a two-pixel block size,
that is only 510 pixels and would recreate premature wrapping on common displays.

**Decision**: Use two bytes. The maximum representable row width is far above
current displays, while the three-cell header still fits comfortably at the
minimum supported width and maximum block size.

## R5: How is physical capacity calculated in the addon?

**Evidence**: ESO UI source and staff guidance establish `GuiRoot` as the live
root dimension and `GetUIGlobalScale()` as the UI-unit-to-physical relationship.
The addon already uses the inverse operation to render `BLOCK_PX` physical
pixels at any UI scale. ESO also exposes `EVENT_SCREEN_RESIZED`.

**Sources**:

- [ESO UI source mirror](https://github.com/esoui/esoui)
- [ESOUI GuiRoot and global-scale discussion](https://www.esoui.com/forums/showthread.php?t=2429)

**Decision**: `floor(GuiRoot:GetWidth() * GetUIGlobalScale() / BLOCK_PX)`, with
zero edge margin, clamped to `[3, 65535]`. A width below three cells cannot carry
a valid header and is rejected by the companion's surface plausibility check.

**Lifecycle**: Recompute immediately on `EVENT_SCREEN_RESIZED` and once per
existing one-second tick. The tick covers UI-scale changes and missed events.
Reflow compares the applied scale as well as the derived columns, because the
same column count at a new scale still requires new UI-unit dimensions.

## R6: How many captures are allowed?

**Finding**: Capturing the header and payload separately on every tick doubles
the GDI work unnecessarily. Capturing an entire client surface to avoid two
small startup captures costs far more.

**Decision**: Cache the last validated layout. Prepare its full extent once,
decode the invariant header from that frame, and reuse it when the layout is
unchanged or shrinks. On first acquisition or growth beyond the prepared frame,
prepare the new extent once more. X11 remains direct point sampling, but honors
the same seam.

## R7: What is the legacy contract?

**Decision**: A valid legacy status cell at `(BLOCK_PX / 2, BLOCK_PX / 2)` with
no new magic selects 16 columns, payload offset zero, and 21 payload cells.
Missing or arbitrary cell-zero data is not legacy. New addon with old companion
is intentionally incompatible because H0 replaces the old status position;
managed deployment and manifest version 14 make that transition explicit.

## R8: How is live geometry shown?

**Decision**: Add a typed `LayoutState` event to the reader's existing shared
observation path. The weave model stores it for display only. Settings renders
negotiated dimensions, legacy dimensions, or a truthful unavailable reason and
does not persist the state. The extent uses the reader's process-lifetime block
size, never a draft setting paired with live columns from another geometry.
