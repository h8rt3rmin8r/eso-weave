# Encounter Capture

S075 adds the ESO Weave Encounter addon as an explicit local observation tool.
It records one privacy-minimized encounter into ESO SavedVariables so later work
can import and analyze the same ordered facts outside the game.

This capture is not automatic. Loading or installing the addon does not authorize
recording. It remains dormant until you arm one Live or PTS encounter.

## Install and arm

S075 supplies the addon source under `addon/EsoWeaveEncounter`. Desktop install
controls are not available yet. For development use, copy that directory into
the selected ESO environment's `AddOns` directory and reload the game interface.

Outside combat, enter one of:

```text
/ewencounter arm live
/ewencounter arm pts
```

The channel is always explicit. Arming during combat waits until combat ends,
then starts at the next clean combat boundary. One arm authorizes at most one
encounter, and normal combat end disarms automatically.

Use these additional commands:

| Command | Result |
| --- | --- |
| `/ewencounter status` | Show state plus stored and omitted event counts |
| `/ewencounter disarm` | Cancel an arm before capture starts |
| `/ewencounter stop` | End an active capture as partial |
| `/ewencounter clear confirm` | Clear the retained encounter envelope |
| `/ewencounter help` | Show the command summary |

A complete or partial retained capture blocks another arm until you clear it.
This makes replacement a deliberate local decision.

## Captured facts

The addon records encounter boundaries, damage, healing, effects, resources,
casts, weapon-bar changes, death, resurrection, boss health, performance,
quickslots, and explicit discontinuities. Every event carries an authoritative
sequence and nondecreasing elapsed milliseconds.

Actors receive opaque positive integers that exist only for one encounter.
Ability and effect references remain numeric so a later catalog can resolve an
unknown ID without rewriting the raw observation.

The capture intentionally omits account names, character names, unit names,
ability and effect names, chat, guild, and location. Unit IDs and tags are used
only as in-memory keys for assigning encounter-local actor numbers. The mapping
is discarded after finalization.

## Bounds and incomplete capture

One capture is limited to 100,000 stored events, 4,096 local actor mappings, and
a conservative 32 MiB estimated SavedVariables budget. Exact serialized size
depends on ESO and is not claimed by the in-game estimate.

Capacity is reserved for a discontinuity and encounter-end event. If regular
capacity is exhausted, source sequencing continues in constant memory and the
terminal discontinuity declares the exact omitted range. Reload, deactivation,
backward clock movement, explicit stop, or callback failure also produces a
partial result instead of a false complete result.

## Save and ownership boundary

After capture, use `/reloadui`, log out, or exit ESO so the client writes
`SavedVariables/EsoWeaveEncounter.lua`. The file stays on your system. S075 adds
no upload, telemetry, network transfer, automatic sharing, or desktop import.

Issue #133 owns the future non-executing importer and user-owned raw store. It
must treat the SavedVariables file as hostile text, validate every bound and
sequence, compute the canonical hash, and preserve the prior store on failure.

Encounter capture does not use Pixel Bus and has no relationship to input
authorization, Weaving, Fishing, or Auto Potion. It does not perform protected
actions, use items, change equipment, move the player, or generate input.

Continue to [Encounter Data and Metrics](../reference/encounter-data-and-metrics.md)
for the complete observation and analysis model.
