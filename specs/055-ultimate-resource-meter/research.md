# Research: Ultimate Resource Meter

## R1: Authoritative ESO observations

**Decision**: Read charge with `GetUnitPower("player",
COMBAT_MECHANIC_FLAGS_ULTIMATE)`. Read both costs with
`GetSlotAbilityCost(ACTION_BAR_ULTIMATE_SLOT_INDEX + 1,
COMBAT_MECHANIC_FLAGS_ULTIMATE, hotbarCategory)` for
`HOTBAR_CATEGORY_PRIMARY` and `HOTBAR_CATEGORY_BACKUP`.

**Rationale**: The game provides current/max charge and the effective slotted
cost directly. This handles different abilities and cost modifiers without an
exhaustive skill-ID table.

**Evidence**: The current ESO UI source reads player Ultimate with `GetUnitPower`,
reads the slotted Ultimate cost with the same slot and mechanic arguments, and
checks `IsSlotUsed` before presenting its meter:
https://github.com/esoui/esoui/blob/live/esoui/ingame/actionbar/actionbutton.lua.
Its action bar registers player plus Ultimate power filters and hotbar-specific
slot callbacks:
https://github.com/esoui/esoui/blob/live/esoui/ingame/actionbar/actionbar.lua.
The live API signatures and event arguments are catalogued in:
https://github.com/esoui/esoui/blob/live/ESOUIDocumentation.txt.

**Rejected**: Hard-code a 500-point cap or map ability IDs to costs. Both become
stale and contradict the live API authority.

## R2: Wire precision

**Decision**: Publish four exact 9-bit values, one per block. Red carries the low
eight bits, one of two field-specific green markers carries the high bit, and
blue remains the `255 - red` complement checksum. Value 511 is unavailable.

**Rationale**: Three rounded percentages cannot reproduce exact values. They can
also produce a false Ready result near a boundary. The wire supports 0 through
510, safely above ESO's 500-point Ultimate bound.

**Rejected**: Three normalized blocks. This loses exact current/max and exact
comparison semantics. Two bytes per value preserve arbitrary `u16` range but add
eight blocks and break the existing one-row invariant at 1024 pixels with 32-point
blocks. Packing a 16-bit value into red and blue loses the checksum.

## R3: Availability and numeric bounds

**Decision**: Reserve 511 for unavailable. Accept publishable values from 0
through 510 and publish unavailable above that bound rather than clamping. Zero
current is valid. Zero maximum and zero cost are unavailable.
If maximum is invalid, the entire presented Ultimate state is unavailable, while
front and back cost decoding remains independently testable at the transport seam.

**Rationale**: The sentinel distinguishes observed zero from missing evidence and
supports values beyond today's common cap without baking in that cap.

## R4: Protocol evolution

**Decision**: Advance to protocol version 5 with code `0xA0` and 29 payload blocks.
Freeze protocol version 4 at 25 blocks. Versions 1 through 3 retain their existing
counts. Only v5 layouts sample B25 through B28.

**Rationale**: Application-first updates must never sample beyond an older addon's
published payload. The 0x20 version-code spacing preserves current tolerance
rules. Header plus payload remains exactly 32 cells, preserving one row at the
supported 1024-pixel minimum width and 32-pixel maximum block size.

## R5: Refresh lifecycle

**Decision**: Extend the existing player-filtered `EVENT_POWER_UPDATE` handler
without adding a power-type filter to that shared registration. Refresh costs on
`EVENT_HOTBAR_SLOT_UPDATED`, `EVENT_HOTBAR_SLOT_STATE_UPDATED`,
`EVENT_ACTION_SLOTS_ACTIVE_HOTBAR_UPDATED`,
`EVENT_ACTION_SLOTS_ALL_HOTBARS_UPDATED`,
`EVENT_ULTIMATE_ABILITY_COST_CHANGED`, weapon-pair changes, full player
activation rebaseline, and the existing one-second periodic backstop. The 100 ms
tick and player power event refresh charge only. Always recompute both costs
together and use `IsSlotUsed` before accepting a positive cost.

**Rationale**: Power gain/spend needs prompt updates, slot and bar changes alter
cost selection, modifiers may lack a dedicated event, and the poll repairs missed
events. Publishing both costs makes bar swaps immediate on the app side.

## R6: Special hotbars

**Decision**: Preserve B3's existing `GetActiveWeaponPairInfo()` and last-good-pair
semantics. When `GetActiveHotbarCategory()` is neither primary nor backup, publish
both Ultimate costs as unavailable. Republish both costs when a primary or backup
bar becomes active. The Ultimate view also hides stale values whenever World is
not Active.

**Rationale**: Werewolf, Overload, and temporary hotbars must not inherit a stale
primary or backup threshold. Changing B3 would affect action-driving weave timing
and violate the display-only boundary. Invalidating only the two display-only
costs prevents a stale threshold without introducing a second wire-level bar
authority.

## R7: Model boundary

**Decision**: Add `UltimateTelemetry` separately from `ResourceSet`, cache it in
the existing engine store, and snapshot it with `ActiveBar` for presentation.

**Rationale**: `ResourceSet` is consumed by auto-potion. A distinct type makes the
display-only guarantee structural and avoids accidental automation expansion.

## R8: Meter composition

**Decision**: Adapt ordinary and Ultimate values into one meter presentation
descriptor. A pure geometry helper reserves a 70-point label region, flexible
track, fixed numeric region, fixed Ready region, and three points below the track.
Quarter marks paint on every track. The optional cost mark paints last and extends
from inside the track to the reserved row bottom.

**Rationale**: One primitive keeps all four rows aligned. Unconditional geometry
allocation prevents visual jumps when readiness changes.

## R9: Color

**Decision**: Use `#A78BFA` in dark mode and `#6D28D9` in light mode for Ultimate
fill. Use semantic success green for Ready, darkening the light-theme Ready token
to `#047857` for normal-text contrast, and reinforce readiness with accessibility
semantics.

**Rationale**: The selected purples are clearly distinct from Health, Stamina,
and Magicka while matching the existing palette's saturation and luminance roles.

## R10: Release verification

**Decision**: Repository implementation closes #71 after merge. A separate
release-verification issue covers live ESO API, modifier, swap, lifecycle, and
theme checks after a published build is available.

**Rationale**: Project governance separates implementation evidence from
environment-dependent release verification.
