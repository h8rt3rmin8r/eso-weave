# Research: PixelBeacon Resource Blocks

**Feature**: 033-resource-blocks | **Date**: 2026-07-27

Phase 0 output. No unknown is left unresolved.

## R1: Reading the pools from the addon

**Decision**: `GetUnitPower("player", COMBAT_MECHANIC_FLAGS_HEALTH)` and its
stamina and magicka counterparts, which return current and maximum, driven by
`EVENT_POWER_UPDATE` with a re-read on the existing fast tick as a backstop.

**Rationale**: Verified against the live `esoui/esoui` source this session:
`GetUnitPower` returns 11 hits and `EVENT_POWER_UPDATE` returns 8. The game's own
code uses exactly this shape, for example in its player-attribute and boss-health
handling:

```lua
local current, max = GetUnitPower("player", COMBAT_MECHANIC_FLAGS_HEALTH)
```

**Worth not re-learning**: the power-type constants are named
`COMBAT_MECHANIC_FLAGS_*`, not the older `POWERTYPE_*`. Issue #2 does not name the
call at all, so this is the first time it has been pinned down.

**Alternatives considered**: polling only, on the existing tick. Rejected as the
sole mechanism because resources move continuously and an event-driven update keeps
the block honest between ticks; the poll is retained as a backstop for the same
reason the weapon block keeps one.

## R2: Why a numeric channel beats a colour lookup table

**Decision**: the payload channel carries the percentage directly.

**Rationale**: This reverses issue #2, so the argument is set out in full.

The issue's position is that the strip "deliberately uses discrete,
tolerance-separated colours that survive capture/scaling", that "a raw numeric
channel is more fragile at 1-step resolution", and that "a mapping table lets us
space the colours for reliable distinction". Each part deserves an answer.

**On fragility.** The latency block has encoded a number in a channel since the
first slice, and the field logs confirm it decodes correctly today. The claim that
a numeric channel is too fragile is not supported by the one piece of evidence the
project actually has.

**On what the table would buy.** Consider a capture that shifts the payload channel
by one step, which is the exact scenario the issue worries about.

| Encoding | Effect of a one-step channel error |
| --- | --- |
| Colour lookup table | Decodes to whichever entry is nearest in colour space. Nearness in colour has no relationship to nearness in percentage, so the result can be any value at all. |
| Numeric channel | Decodes to one percent off. |

The table's failure mode is **unbounded**; the numeric channel's is **bounded**.
For a discrete state (in combat, on the map) unbounded is acceptable, because every
wrong answer is equally wrong and the marker plus checksum make a wrong answer
vanishingly unlikely. For an ordered quantity it is not: the difference between
reading 61 and 62 is nothing, and the difference between reading 61 and 4 is a
future consumer deciding the player is about to die.

**On the deliverable.** The table also has to be built. A hundred and one colours,
each pairwise separated, each transcribed identically into two languages, each
checked. That is the feature's stated gating deliverable and it exists only to
serve an encoding that is worse on the property that matters.

**Alternatives considered**:

- **The table at 1 percent**, as specified. Rejected above.
- **The table at 5 percent**, the issue's fallback. Rejected for the same reason,
  and the fallback's premise (that 1 percent might not be achievable) does not
  arise once the payload is numeric.
- **A numeric channel with no checksum**, using the freed channel for a second
  resource. Rejected: it halves the block count but removes the validation that
  stops an absent block reading as data, which is the whole compatibility story.

## R3: What correctness means for this signal

**Decision**: bounded error plus monotonicity, proven by enumeration.

**Rationale**: The publishable range is 101 values and the tolerance window is
small, so the full cross product of published value against in-tolerance
perturbation is enumerable. That converts the correctness claim from an argument
into a test.

The precise property, corrected during the encoding checklist, is that a perturbed
sample decodes either to a value within tolerance **or to unavailable**, never to a
different percentage. The "or unavailable" is not a weakening: the payload and its
checksum are validated together, so channels drifting the same way sum to twice the
tolerance and the sample is rejected. Rejecting is safe. The existing latency block
has always behaved this way; this feature is the first to write it down.

## R4: Marker selection under a crowded channel

**Decision**: `0x16`, `0x6D`, `0xBB`.

**Rationale**: With seven greens already in use and three to add, the channel is
getting crowded and the mnemonic scheme that produced `0xA5`/`0x5A` and
`0x2D`/`0xD2` no longer helps: the natural swap partners land badly, `0xD6` being
four away from the menu marker. The three chosen values sit in the widest remaining
gaps, giving a minimum separation of 19 against a default tolerance of 2.

This is the point at which marker selection stops being free. A tenth block will
have roughly 19 of headroom to work with, and the registry test in
`tests/pixelbus.rs` is what will say so rather than a reviewer noticing.

## R5: Log volume

**Decision**: trace level for resource changes; debug is reserved for signals that
change on a human timescale.

**Rationale**: The live log is the tool that diagnosed every field defect in this
project, including the fishing bait-burning loop, which was found by reading a
single debug line's cadence. Three resources changing many times a second at debug
would make that tool unusable. Nothing else about the signal changes.

## Open items carried forward

None. Note for slice 034 (issue #3, display detection): it touches no addon code
and adds no block, so it is the one remaining slice in this plan that does not need
a marker.
