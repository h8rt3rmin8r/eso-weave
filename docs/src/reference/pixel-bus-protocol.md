# Pixel Bus Protocol

Pixel Bus is also called pixelbus, screen telemetry, or the color protocol.

**User guarantee:** Missing, invalid, stale, corrupt, or version-incompatible
evidence never becomes a newer valid payload or a positive authorization value.

**Implementation detail:** The exact geometry and bytes below are the current
wire authority for contributors. Feature pages link here instead of copying the
encoding.

PixelBeacon renders a three-cell layout header followed by twenty-nine payload
blocks at the top-left of the ESO client area. Blocks are 16 by 16 physical
pixels by default. Interface scaling is compensated so geometry remains physical.
Loading screens hide the blocks.

## Geometry and negotiation

PixelBeacon is the sole authority for the current column count. It publishes the
16-bit count in the header, and ESO Weave validates it against the measured client
surface. Header cells H0 through H2 occupy logical cells zero through two. Payload
block `i` occupies cell `3 + i`, with column `cell mod columns` and row
`cell div columns`.

PixelBeacon computes the number of complete physical blocks that fit the current
`GuiRoot` width. ESO Weave does not derive a competing count. This authority
supersedes the fixed 16-column contract while retaining that geometry only for a
positively identified pre-version-14 addon. At all supported client widths and
block sizes, the current 32 cells fit on one row. The occupied extent is:

```text
width = BLOCK_PX * min(3 + NUM_BLOCKS, columns)
height = BLOCK_PX * ceil((3 + NUM_BLOCKS) / columns)
```

At the default block size, that is 512 by 16 physical pixels. Cells after the
last payload block are neither drawn nor read.

H0 is `(0x45, 0x53, 0xA0)`, where `0xA0` represents protocol version 5. H1 is
`(columns_high, 0x64, 255 - columns_high)` and H2 is
`(columns_low, 0x9C, 255 - columns_low)`. Invalid magic, markers, complements,
versions, counts, or surface fit make the complete layout unavailable.

Magic, markers, and complements honor the configured capture tolerance. Geometry
metadata caps the effective tolerance at 15, below half the version-code spacing,
even when payload tolerance is broader.

A non-magic H0 selects the legacy 16-column layout only when it is a valid legacy
magenta heartbeat. Version 1 retains 22 payload cells, version 2 retains 23,
version 3 retains 24, and version 4 retains 25. B22 is sampled only from version
2, B23 from version 3, B24 from version 4, and B25 through B28 from version 5.
The corresponding PixelBeacon addon versions were 16 for world state, 17 for
roll dodge, 18 for travel, 19 for explicit on-foot sprint, and 20 for Ultimate.

The addon and application each state the header constants once. Contract tests
parse the embedded addon source to prevent byte-level drift. Capture extent and
all payload points derive from one validated layout. The reader captures one
occupied frame per steady batch, with one additional capture permitted during
initial negotiation or growth beyond the prepared frame.

## Payload blocks

Positions below use the default block size and current one-row layout.

| Block | Sample | Encoding and meaning |
| --- | --- | --- |
| B0 Status | (56, 8) | `#FF00FF` while PixelBeacon is loaded and rendering |
| B1 Fishing | (72, 8) | `#0080FF` while waiting, `#00FF00` on a bite, hidden otherwise |
| B2 Latency | (88, 8) | `R = clamp(GetLatency(), 0, 1020) / 4`, `G = 0xA5`, `B = 255 - R`; updated at 1 Hz |
| B3 Weapon Bar | (104, 8) | `G = 0x5A`; R packs the primary and backup weapon-class nibbles as `primary * 16 + backup`; B is 0 unknown, 1 primary, or 2 backup |
| B4 Combat | (120, 8) | `G = 0x2D`; R is `0xE0` in combat or `0x20` out; B complements R; player combat events and activation rebaseline it |
| B5 Menu | (136, 8) | `G = 0xD2`; R is surface code times 24: 0 gameplay, 1 system, 2 map, 3 inventory, 4 mail, 5 character, 6 guild store, 7 crown store, 8 journal, 9 chat entry, 10 other |
| B6 Health | (152, 8) | `G = 0x16`; R is 0 to 100 or `0xFF` unavailable; B complements R |
| B7 Stamina | (168, 8) | Resource encoding with `G = 0x6D` |
| B8 Magicka | (184, 8) | Resource encoding with `G = 0xBB` |
| B9 Movement | (200, 8) | `G = 0x43`; R is `0x20` on foot, `0x60` mounted, or `0xA0` bounded keyboard-mode on-foot sprint; B complements R and `0xE0` remains reserved and decodes Unknown |
| B10 through B15 Cooldowns | (216, 8) through (296, 8) | Skill 1 through Ultimate with markers `0x0B`, `0x21`, `0x4E`, `0x92`, `0xC6`, `0xE8`; R uses 50 ms steps, zero ready, 1 through 254 duration saturating at 12700 ms, `0xFF` unavailable |
| B16 Quickslot Cooldown | (312, 8) | `G = 0x38`; R uses the cooldown encoding; this attached fact never classifies the selected entry |
| B17 through B19 Quickslot Item | (328, 8) through (360, 8) | Markers `0xB0`, `0xDD`, `0xF3`; R carries an optional 24-bit item ID most significant byte first; retained only with explicit B20 potion classification and used for diagnosis only |
| B20 Quickslot Classification | (376, 8) | `G = 0x76`; spaced R codes distinguish unsupported API, invalid or inconsistent facts, empty, item, collectible, quest item, emote, quick chat, other, depleted potion, blocked potion, and usable potion |
| B21 Life State | (392, 8) | `G = 0x89`; R is `0x20` Alive, `0x80` Dead, or `0xE0` Reincarnating; reincarnation takes precedence over death and missing evidence is Unknown |
| B22 World State | (408, 8) | Version 2; `G = 0xCC`; R is `0x20` Unknown, `0x80` Transitioning, or `0xE0` Active; deactivation transitions immediately and activation refreshes all player payloads before Active |
| B23 Roll Dodge | (424, 8) | Version 3; `G = 0xF9`; R is `0x20` Unknown, `0x80` Inactive, or `0xE0` Active; player ability 28549 gain/fade drives it and a 1500 ms watchdog clears rejected gains |
| B24 Travel | (440, 8) | Version 4; `G = 0x13`; R is `0x20` Unknown, `0x80` Inactive, or `0xE0` Pending; recall-cooldown growth or jump preparation enters Pending and movement, jump failure, or a 15 second watchdog clears it |
| B25 Ultimate Current | (456, 8) | Version 5 exact 9-bit value; R carries bits 0 through 7, G is `0x05` when bit 8 is clear or `0x7B` when it is set, and B is `255 - R`; 511 is unavailable |
| B26 Ultimate Maximum | (472, 8) | The same exact codec with G `0x1B` or `0x5F`; zero and 511 are unavailable |
| B27 Primary Ultimate Cost | (488, 8) | The same exact codec with G `0x27` or `0x48`; 511 is unavailable |
| B28 Backup Ultimate Cost | (504, 8) | The same exact codec with G `0x32` or `0x3D`; 511 is unavailable |

B9 mounted state comes from `IsMounted()`. Sprint requires moving and trying to
move while every slot on `GetActiveHotbarCategory()` reports
`ActionSlotHasNonCostStateFailure`. Gamepad, mounted, swimming, falling, death,
roll dodge, and inactive-world states are excluded. Entry and ambiguous exit
debounce for 200 ms, and a stale positive expires after 1500 ms.
`EVENT_ACTION_SLOT_STATE_UPDATED` and the 100 ms tick converge through one
detector.

B10 through B15 are polled on the 1 Hz tick with change detection and rebaselined
on `EVENT_PLAYER_ACTIVATED`, because ESO exposes no per-slot cooldown event.
Synergy has no cooldown block because it is a contextual prompt rather than an
action slot.

B20 classification uses `GetCurrentQuickslot`, `GetSlotType`, `GetSlotBoundId`,
`GetSlotItemLink`, `GetSlotItemCount`, `IsSlotUsable`, and
`GetSlotCooldownInfo`. Selection, slot-content, slot-state, cooldown, inventory,
and player-activation events converge through one change-detected path with a
1 Hz recovery backstop. Missing B20 with a valid legacy B16 means the addon must
be updated. Invalid or tolerance-ambiguous B20 is a corrupt signal.

B23 combat events are filtered to the player and ability 28549. Death,
deactivation, invalid data, and signal loss publish Unknown and disable combat
event handling until activation or an in-place resurrection establishes an
Inactive baseline. While interception is active, the companion sets its sample interval at 375 ms so multiple reads fit within the bounded Active window.

B25 and B26 come from the same
`GetUnitPower("player", COMBAT_MECHANIC_FLAGS_ULTIMATE)` call. Values from 0 through 510 publish exactly, except that a zero maximum is unavailable. A nil,
negative, or out-of-range maximum makes both current and maximum unavailable. A
nil, negative, or out-of-range current makes current unavailable without
inventing a value.

B27 and B28 call
`GetSlotAbilityCost(ACTION_BAR_ULTIMATE_SLOT_INDEX + 1,
COMBAT_MECHANIC_FLAGS_ULTIMATE, hotbarCategory)` with
`HOTBAR_CATEGORY_PRIMARY` and `HOTBAR_CATEGORY_BACKUP`, respectively.
An unused slot, nil API result, negative value, zero cost, or value above 510 publishes 511 for that cost. The primary and backup costs are sampled and published together,
so a normal weapon-bar swap selects already cached data atomically. Slot and
hotbar events plus the 1 Hz recovery backstop refresh the pair.
While any special or temporary hotbar is active, both costs publish unavailable without changing
the established weapon-bar signal.

No payload block is hidden to express a state. Absence means only that the addon
is too old to draw that block, preventing an old layout from being interpreted as
a newer state.

Weapon class codes are 0 none, 1 dual wield, 2 two handed, 3 sword and shield,
4 bow, 5 destruction staff, and 6 restoration staff.

Every marker is separated from every other marker by more than the configured
tolerance. Samples must match the marker and checksum within per-channel tolerance,
which defaults to plus or minus 2. There is no nearest-match fallback. Invalid
samples produce unavailable, and any signal that can authorize behavior clears on
a failed decode rather than retaining stale state.

A new marker is selected at the midpoint of the widest remaining gap. This keeps
the minimum separation high as the registry grows.

Numeric payloads carry quantities rather than palette indexes. A sample perturbed
within tolerance therefore decodes to a nearby value or unavailable, never an
unrelated table entry. Resource values slightly over 100 clamp to 100 so ordinary
full-pool compositor drift remains stable.

## Behavioral consumers

| Signal | Consumer |
| --- | --- |
| Latency and Weapon Bar | Weave timing |
| Fishing | Fishing controller |
| Menu | Input decision, fishing, and Auto Potion |
| Life, World, Travel, and Roll Dodge | Generated-input safety boundaries |
| Resources and Quickslot | Auto Potion |
| Movement | Display and explicit sprint blocking for Auto Potion |
| Combat, skill cooldowns, and Ultimate | Display only |

Observable-only signals are covered by tests ensuring they cannot change engine
behavior.

## Sampling and display validation

Sampling defaults to 100 ms while fishing or interception can be active, and
1000 ms otherwise. The interception condition keeps the menu gate responsive
while the operator may type.
A suspended application does not start new interception and samples slowly only
when Fishing is inactive. Suspension invalidates queued or running weave work,
cancels Fishing deadlines, retains the Fishing request, and emits nothing on
resume. The reader validates the full header before reading payload from the
same captured frame. Missing B0 for more than 2000 ms
raises SignalLost; a corrupt recognized header suppresses payload immediately.

Windows captures the displayed composited desktop at the client-area origin.
Capturing only the window device context would read a GDI front buffer that lacks
hardware-accelerated content. Linux uses X11 or XWayland capture.

A separate display descriptor records physical surface size and origin, display
geometry and scale, and whether the reading was measured or configured. Live
operating-system measurements are change-detected. Stored video settings provide
a cross-check and pre-launch fallback, but never override a live measurement. A
configured descriptor is produced only when both stored resolution pairs are
identical because no verified mapping exists for the stored window-mode value.
On X11, the reported display is the X screen, which can be the union of multiple heads and carries no scale factor because the core X protocol exposes neither.

The announced row width and complete occupied extent must fit inside the measured
client area or the layout is rejected before payload decoding. Reading only the
cells that happen to fit could associate a valid marker with the wrong logical
signal, so partial layouts are never accepted.

## Diagnostic sequence

```text
capture current client extent
  -> validate H0, H1, H2 and complete occupied geometry
  -> require current or positively identified legacy layout
  -> require B0 heartbeat
  -> decode each supported payload with marker and complement checks
  -> publish changes to consumers
invalid header -> suppress payload immediately
heartbeat absent past timeout -> Signal lost and clear authorizing observations
fresh valid frame -> republish the complete current baseline
```

This sequence is also the text alternative for the protocol flow. A reader
diagnosing **Signal unavailable** should first use the
[PixelBeacon troubleshooting path](../getting-started/troubleshooting.md#pixelbeacon-signal-is-missing-or-lost),
then inspect byte-level details here only when the shared lifecycle is healthy.
