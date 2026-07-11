# Contract: Pixel Bus and Manifest

The on-screen contract a future Pixel Bus Reader samples, and the manifest the
Beacon Manager verifies. Values are fixed by master specification section 9.

## Blocks

Anchored to the top-left of the client area. Each block is 16 by 16 physical
pixels (compensated for UI scale).

| Block | Position (px) | Sample point (px) | Meaning and color |
| --- | --- | --- | --- |
| B0 Status | (0, 0) | (8, 8) | Solid `#FF00FF` whenever the addon is loaded and rendering. |
| B1 Fishing | (16, 0) | (24, 8) | `#0080FF` while a cast is active and waiting; `#00FF00` on a bite; absent otherwise. |
| B2 Latency | (32, 0) | (40, 8) | red = clamp(latency, 0, 1020) / 4; green = `0xA5`; blue = 255 minus red. Updated at 1 Hz; rendered only while B0 renders. |

- All blocks are hidden during loading screens by the game UI lifecycle.
- Colors are set in the game's 0 to 1 channel range (each 8-bit value divided by
  255).

## Latency encoding examples

- latency 0: red 0, green 165, blue 255.
- latency 400: red 100, green 165, blue 155.
- latency 1020 or higher: red 255, green 165, blue 0 (clamped).

The green marker `0xA5` (165) identifies a latency sample; blue is the checksum
complement of red, so a reader can validate `red + blue == 255`.

## Bite detection contract

- Trigger: equipped bait stack decreases by one during an active fishing
  interaction.
- Gate: interaction-active derived from `EVENT_CLIENT_INTERACT_RESULT` and the
  camera-interaction state.
- Clear: on a new item gained, on `EVENT_CHATTER_END`, or after a safety timeout.
- Suppress: while any menu is open.

## Manifest

`PixelBeacon.txt` MUST contain:

- A declared `## APIVersion`.
- An addon version (for example `## Version: 1`).
- The managed marker line, verbatim: `## X-ESO-Weave-Managed: true`.
- The single file entry `PixelBeacon.lua`.
- No saved variables, no dependencies, no libraries.
