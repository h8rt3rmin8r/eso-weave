# Ultimate Resource

Ultimate Cost means the cast cost for the active front-bar or back-bar skill,
including the readiness tick shown on the meter.

Ultimate differs from Health, Stamina, and Magicka. It has a stored point total,
a game-reported maximum, and a cast cost that depends on the ability slotted on
each weapon bar. A percentage alone cannot say whether the active ability is
castable.

Players may search for this as ult, Ultimate points, enough Ultimate, cast-ready,
or the readiness tick. The interface uses the exact labels **Ultimate** and
**Ready**.

PixelBeacon publishes exact current and maximum Ultimate plus the effective cost
for both the primary and backup hotbars. ESO Weave selects the cost associated
with the active bar. It does not maintain a catalogue of ability IDs or assume
one cost for every Ultimate.

The Live HUD presents:

- A purple meter with exact current and maximum points.
- Subtle 25, 50, and 75 percent landmarks shared with every resource meter.
- A stronger tick at the active bar's exact cost threshold.
- Green **Ready** text when current points meet or exceed that cost.
- Reserved number and Ready regions, so state changes do not shift nearby text.

If no Ultimate is slotted, the active bar is unknown, a special hotbar is active,
or cost telemetry is unavailable, the threshold and Ready state remain hidden.
The meter does not guess.

Ultimate is display-only. It does not affect weaving, Auto Potion, or any input
gate. Protocol version 5 transports exact values; older protocol layouts remain
bounded to their original payload lengths and report Ultimate unavailable.

After installing or updating PixelBeacon, use `/reloadui` in ESO and restart ESO
Weave so both sides negotiate the current protocol. Swapping weapon bars changes
the selected threshold to the cost already reported for that bar.

No color is required to interpret the meter: exact current and maximum points,
the protruding cost tick, and the fixed Ready text provide separate cues. See
[Live Interface](interface.md#resource-meters) for meter accessibility and the
[Status Reference](../reference/status-reference.md#resources-and-ultimate) for
unavailable states.
