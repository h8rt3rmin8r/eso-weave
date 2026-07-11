# Data Model: PixelBeacon Addon

Language-neutral entities. Concrete Lua is fixed in the tasks phase.

## Beacon Block

| Field | Type | Notes |
| --- | --- | --- |
| id | B0, B1, B2 | Status, fishing, latency. |
| position | (x, y) px | (0,0), (16,0), (32,0). |
| size | 16 by 16 px | Physical pixels; UI-scale compensated. |
| color | RGB | Selected by state; see the pixel-bus contract. |
| visible | bool | B1 is hidden when idle; B2 only while B0 renders; all hidden during loading. |

## Fishing State

`Idle`, `Waiting` (cast active), or `Bite`. Selects the B1 color (absent, waiting
color, bite color).

## Bite Signal

Derived state: becomes a bite when the equipped bait stack decreases by one during
an active fishing interaction with no menu open; cleared on new item, interaction
end, or safety timeout.

## Latency Sample

From `GetLatency()`, encoded into B2 as red = clamp(latency, 0, 1020) / 4, green =
165, blue = 255 minus red.

## Manifest

| Field | Notes |
| --- | --- |
| APIVersion | Declared game API version (subject to R4 upkeep). |
| Version | Addon version (initial value set by this slice). |
| Managed marker | `## X-ESO-Weave-Managed: true`, verbatim. |
| Files | `PixelBeacon.lua`. |
