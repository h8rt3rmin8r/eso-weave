# Contract: Quickslot Observation B16 to B20

**Feature**: [../spec.md](../spec.md) | **Date**: 2026-09-03

## Grid

`NUM_BLOCKS = 21`, `COLUMNS = 16`. B20 is column 4, row 1. Existing B0 to B19
positions and encodings remain unchanged.

## Existing attached facts

- B16: cooldown steps with `QUICKSLOT_MARKER = 0x38`
- B17: identity bits 23..16 with marker `0xB0`
- B18: identity bits 15..8 with marker `0xDD`
- B19: identity bits 7..0 with marker `0xF3`

Every block uses red payload, green marker, and blue `255 - red` checksum.

## B20 classification

`QUICKSLOT_STATE_MARKER = 0x76`.

| Red code | Classification |
| --- | --- |
| `0x10` | Unavailable(UnsupportedApi) |
| `0x20` | Unavailable(InvalidSelection) |
| `0x30` | Unavailable(InconsistentFacts) |
| `0x40` | Empty |
| `0x50` | NonPotion(Item) |
| `0x60` | NonPotion(Collectible) |
| `0x70` | NonPotion(QuestItem) |
| `0x80` | NonPotion(Emote) |
| `0x90` | NonPotion(QuickChat) |
| `0xA0` | NonPotion(Other) |
| `0xB0` | Potion(Depleted) |
| `0xC0` | Potion(Blocked) |
| `0xD0` | Potion(Usable) |

An absent B20 becomes Unavailable(LegacyProtocol) when legacy B16 is readable,
otherwise Unavailable(NoSignal). A present B20 with an invalid marker, checksum,
or code becomes Unavailable(CorruptProtocol).

## Composition rules

- Cooldown decodes independently from B16.
- Identity is assembled only when B17 to B19 all decode and B20 classifies a potion.
- Empty, non-potion, and unavailable states discard identity bytes.
- A valid Potion classification may have Unknown cooldown or absent identity;
  neither changes its classification or availability.
- The application-level automation consumer gate remains false in S042 regardless of the decoded state.

## Cross-language agreement

Protocol tests parse the Lua source and compare `NUM_BLOCKS`, `COLUMNS`, every B20
code, and `QUICKSLOT_STATE_MARKER` with Rust constants. Marker separation and
complement checks remain exhaustive.
